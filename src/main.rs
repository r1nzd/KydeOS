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

fn main() {
    tracing_subscriber::fmt::init();
    info!("KydeShell v0.1.0 — Aurora");

    let config = KydeConfig::load();
    info!("Config: dark_mode={}, accent={}", config.dark_mode, config.accent_color);

    let display: Display<KydeCompositor> = Display::new().unwrap();
    let display_handle = display.handle();
    let event_loop: EventLoop<KydeCompositor> = EventLoop::try_new().unwrap();

    let comp = KydeCompositor::new(display_handle);
    info!("KydeShell compositor initialized successfully!");

    run_winit(comp, event_loop);
}
