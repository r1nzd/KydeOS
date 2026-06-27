use tracing::info;

pub struct KydeShell {
    pub taskbar_visible: bool,
    pub launcher_open: bool,
}

impl KydeShell {
    pub fn new() -> Self {
        info!("Initializing KydeShell UI...");
        KydeShell {
            taskbar_visible: true,
            launcher_open: false,
        }
    }

    pub fn show_taskbar(&mut self) {
        info!("Showing taskbar");
        self.taskbar_visible = true;
    }

    pub fn open_launcher(&mut self) {
        info!("Opening app launcher");
        self.launcher_open = true;
    }

    pub fn close_launcher(&mut self) {
        info!("Closing app launcher");
        self.launcher_open = false;
    }
}
