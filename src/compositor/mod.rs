use tracing::info;

pub struct KydeCompositor {
    pub running: bool,
}

impl KydeCompositor {
    pub fn new() -> Self {
        info!("Creating KydeShell compositor...");
        KydeCompositor {
            running: true,
        }
    }

    pub fn run(&mut self) {
        info!("KydeShell compositor running...");
        // TODO: Smithay event loop
        // TODO: DRM/KMS backend init
        // TODO: Input handling
    }

    pub fn stop(&mut self) {
        info!("Stopping KydeShell compositor...");
        self.running = false;
    }
}
