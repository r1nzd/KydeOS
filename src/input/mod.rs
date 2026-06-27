use tracing::info;

pub struct InputHandler {
    pub keyboard_active: bool,
    pub pointer_active: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        info!("Initializing input handler...");
        InputHandler {
            keyboard_active: false,
            pointer_active: false,
        }
    }

    pub fn init_keyboard(&mut self) {
        info!("Keyboard initialized");
        self.keyboard_active = true;
    }

    pub fn init_pointer(&mut self) {
        info!("Pointer initialized");
        self.pointer_active = true;
    }
}
