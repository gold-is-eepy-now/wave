use std::thread;
use std::time::Duration;
use ui::aqua;

#[derive(Debug, Clone)]
struct DockItem {
    name: String,
    running: bool,
    base_size: f32,
}

#[derive(Debug)]
struct Dock {
    items: Vec<DockItem>,
    magnification: f32,
}

impl Dock {
    fn new() -> Self {
        Self {
            items: vec![
                DockItem {
                    name: "Finder".into(),
                    running: true,
                    base_size: 48.0,
                },
                DockItem {
                    name: "Terminal".into(),
                    running: true,
                    base_size: 48.0,
                },
                DockItem {
                    name: "Browser".into(),
                    running: false,
                    base_size: 48.0,
                },
            ],
            magnification: 1.65,
        }
    }

    fn hover_scale(&self, center_index: usize, item_index: usize) -> f32 {
        let distance = center_index.abs_diff(item_index) as f32;
        let falloff = (1.0 - (distance / 3.0)).max(0.0);
        1.0 + (self.magnification - 1.0) * falloff
    }

    fn reorder(&mut self, from: usize, to: usize) {
        if from >= self.items.len() || to >= self.items.len() || from == to {
            return;
        }
        let item = self.items.remove(from);
        self.items.insert(to, item);
    }

    fn render_preview(&self, hover: usize) {
        println!("[dock] traffic-light highlight {:?}", aqua::BLUE_HIGHLIGHT);
        for (i, item) in self.items.iter().enumerate() {
            let scale = self.hover_scale(hover, i);
            let size = item.base_size * scale;
            println!(
                "[dock] {:<10} size={:.1}px running={}",
                item.name, size, item.running
            );
        }
    }
}

fn main() {
    let oneshot = std::env::args().any(|a| a == "--oneshot");
    println!("SnowDesk Dock prototype");
    let mut dock = Dock::new();
    dock.reorder(2, 1);
    dock.render_preview(1);

    if oneshot {
        return;
    }

    loop {
        thread::sleep(Duration::from_secs(12));
        dock.render_preview(1);
    }
}
