use std::time::{SystemTime, UNIX_EPOCH};
use ui::aqua;

#[derive(Debug)]
struct StatusItem {
    name: &'static str,
    enabled: bool,
}

#[derive(Debug)]
struct TopPanel {
    active_app: String,
    menu_items: Vec<String>,
    status_items: Vec<StatusItem>,
}

impl TopPanel {
    fn new() -> Self {
        Self {
            active_app: "Finder".into(),
            menu_items: vec!["File".into(), "Edit".into(), "View".into(), "Go".into()],
            status_items: vec![
                StatusItem {
                    name: "WiFi",
                    enabled: true,
                },
                StatusItem {
                    name: "Volume",
                    enabled: true,
                },
                StatusItem {
                    name: "Battery",
                    enabled: true,
                },
            ],
        }
    }

    fn render(&self) {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default();
        println!("[panel] gradient {:?} -> {:?}", aqua::GLASS_TOP, aqua::GLASS_BOTTOM);
        println!("[panel] app: {}", self.active_app);
        println!("[panel] menu: {}", self.menu_items.join(" | "));
        println!(
            "[panel] status: {}",
            self.status_items
                .iter()
                .filter(|i| i.enabled)
                .map(|i| i.name)
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!("[panel] clock(epoch): {}", seconds);
    }
}

fn main() {
    println!("SnowDesk Panel prototype (global menubar model)");
    TopPanel::new().render();
}
