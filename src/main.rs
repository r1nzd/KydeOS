mod compositor;
mod shell;
mod config;
mod input;

use tracing::info;
use tracing_subscriber;
use smithay::reexports::wayland_server::Display;
use compositor::KydeCompositor;
use config::KydeConfig;

fn main() {
    tracing_subscriber::fmt::init();
    info!("KydeShell v0.1.0 — Aurora");

    let config = KydeConfig::load();
    info!("Config: dark_mode={}, accent={}", config.dark_mode, config.accent_color);

    let display: Display<KydeCompositor> = Display::new().unwrap();
    let display_handle = display.handle();

    let mut comp = KydeCompositor::new(display_handle);
    info!("KydeShell compositor initialized successfully!");
}
