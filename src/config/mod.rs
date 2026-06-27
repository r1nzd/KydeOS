use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct KydeConfig {
    pub taskbar_position: TaskbarPosition,
    pub dark_mode: bool,
    pub accent_color: String,
    pub font_scale: f32,
    pub animations: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskbarPosition {
    Bottom,
    Top,
    Left,
    Right,
}

impl Default for KydeConfig {
    fn default() -> Self {
        info!("Loading default KydeShell config...");
        KydeConfig {
            taskbar_position: TaskbarPosition::Bottom,
            dark_mode: true,
            accent_color: "#6750A4".to_string(), // Material 3 purple
            font_scale: 1.0,
            animations: true,
        }
    }
}

impl KydeConfig {
    pub fn load() -> Self {
        // TODO: Load from ~/.config/kydeshell/config.json
        info!("Loading KydeShell config...");
        Self::default()
    }
}
