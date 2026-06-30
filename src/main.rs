mod compositor;
mod shell;
mod config;
mod input;

use tracing::info;
use tracing_subscriber;
use compositor::KydeCompositor;
use config::KydeConfig;

fn main() {
    tracing_subscriber::fmt::init();
    info!("KydeShell v0.1.0 — Aurora");

    let config = KydeConfig::load();
    info!("Config: dark_mode={}, accent={}", config.dark_mode, config.accent_color);

    let mut comp = KydeCompositor::new();
    comp.run();
}
