mod compositor;
mod shell;
mod config;
mod input;
mod backend;

use tracing::info;
use tracing_subscriber;
use smithay::reexports::{
    wayland_server::Display,
    calloop::EventLoop,
};
use compositor::KydeCompositor;
use config::KydeConfig;
use backend::winit::run_winit;
use backend::drm::run_drm;
use std::env;

fn main() {
    tracing_subscriber::fmt::init();
    info!("KydeShell v0.1.0 — Aurora");

    let mut config = KydeConfig::load();

    if let Some(color) = shell::monet::extract_accent_color("/home/ace/kydeshell/assets/wallpaper.png") {
        config.accent_color = shell::monet::color_to_hex(color);
        info!("Extracted accent color from wallpaper: {}", config.accent_color);
    } else {
        info!("Failed to extract accent color, using default");
    }
    info!("Config: dark_mode={}, accent={}", config.dark_mode, config.accent_color);

    let display: Display<KydeCompositor> = Display::new().unwrap();
    let display_handle = display.handle();
    let event_loop: EventLoop<KydeCompositor> = EventLoop::try_new().unwrap();

    let comp = KydeCompositor::new(display_handle);
    info!("KydeShell compositor initialized successfully!");

    let backend_choice = env::var("KYDESHELL_BACKEND").unwrap_or_else(|_| "winit".to_string());
    match backend_choice.as_str() {
        "drm" => {
            info!("Backend selected: DRM/KMS (bare metal)");
            run_drm(comp, event_loop);
        }
        _ => {
            info!("Backend selected: winit (windowed)");
            run_winit(comp, display, event_loop, config.accent_color.clone());
        }
    }
}
