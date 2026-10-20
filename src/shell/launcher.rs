#[derive(Clone, Debug)]
pub struct AppItem {
    pub name: String,
    pub exec: String,
    pub color: [f32; 4],
}

pub fn get_default_apps() -> Vec<AppItem> {
    let mut apps = vec![
        AppItem {
            name: "Terminal".into(),
            exec: "foot".into(),
            color: [0.18, 0.72, 0.44, 1.0],
        },
        AppItem {
            name: "Files".into(),
            exec: "foot -e lf".into(),
            color: [0.92, 0.65, 0.18, 1.0],
        },
        AppItem {
            name: "Settings".into(),
            exec: "foot -e htop".into(),
            color: [0.45, 0.55, 0.68, 1.0],
        },
        AppItem {
            name: "Editor".into(),
            exec: "foot -e vim".into(),
            color: [0.24, 0.52, 0.88, 1.0],
        },
        AppItem {
            name: "Browser".into(),
            exec: "foot".into(),
            color: [0.90, 0.38, 0.35, 1.0],
        },
        AppItem {
            name: "Store".into(),
            exec: "foot".into(),
            color: [0.62, 0.38, 0.88, 1.0],
        },
    ];

    if let Ok(entries) = std::fs::read_dir("/usr/share/applications") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let mut name = None;
                    let mut exec = None;
                    let mut is_app = false;
                    let mut no_display = false;

                    for line in content.lines() {
                        let line = line.trim();
                        if line == "Type=Application" {
                            is_app = true;
                        } else if line == "NoDisplay=true" {
                            no_display = true;
                        } else if line.starts_with("Name=") && name.is_none() {
                            name = Some(line[5..].to_string());
                        } else if line.starts_with("Exec=") && exec.is_none() {
                            let mut e = line[5..].to_string();
                            if let Some(idx) = e.find(" %") {
                                e.truncate(idx);
                            }
                            exec = Some(e);
                        }
                    }

                    if is_app && !no_display {
                        if let (Some(n), Some(e)) = (name, exec) {
                            if !apps.iter().any(|a| a.exec == e || a.name == n) {
                                apps.push(AppItem {
                                    name: n,
                                    exec: e,
                                    color: [0.35, 0.55, 0.75, 1.0],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    apps
}
