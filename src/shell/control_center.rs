#[derive(Clone, Debug)]
pub struct QuickTile {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub active: bool,
    pub disabled: bool,
}

pub fn get_default_tiles() -> [QuickTile; 6] {
    [
        QuickTile {
            id: "internet",
            title: "Internet",
            subtitle: "Connected",
            active: true,
            disabled: false,
        },
        QuickTile {
            id: "bluetooth",
            title: "Bluetooth",
            subtitle: "Off",
            active: false,
            disabled: false,
        },
        QuickTile {
            id: "dnd",
            title: "Do Not Disturb",
            subtitle: "Off",
            active: false,
            disabled: false,
        },
        QuickTile {
            id: "flashlight",
            title: "Flashlight",
            subtitle: "Off",
            active: false,
            disabled: false,
        },
        QuickTile {
            id: "airplane",
            title: "Airplane",
            subtitle: "Off",
            active: false,
            disabled: true,
        },
        QuickTile {
            id: "nightlight",
            title: "Night Light",
            subtitle: "Off",
            active: false,
            disabled: false,
        },
    ]
}
