use gtk::prelude::*;
use gtk4 as gtk;

use crate::{apply_x11_dock_hints, PartialStrut, ScreenGeometry, StrutConfig};

const DOCK_HEIGHT: i32 = 78;
const DOCK_WIDTH: i32 = 560;

pub fn build_dock(app: &gtk::Application, screen: ScreenGeometry) {
    let dock = gtk::ApplicationWindow::builder()
        .application(app)
        .title("AquaDE Dock")
        .decorated(false)
        .resizable(false)
        .default_width(DOCK_WIDTH)
        .default_height(DOCK_HEIGHT)
        .build();

    dock.add_css_class("dock-window");

    let frame = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    frame.add_css_class("dock-frame");
    frame.set_margin_start(10);
    frame.set_margin_end(10);
    frame.set_margin_top(10);
    frame.set_margin_bottom(10);

    for (icon, cmd) in [
        ("🖥", "x-terminal-emulator"),
        ("📁", "nautilus"),
        ("🌐", "xdg-open https://duckduckgo.com"),
        ("⚙", "gnome-control-center"),
    ] {
        let button = gtk::Button::with_label(icon);
        button.add_css_class("dock-icon");
        button.set_tooltip_text(Some(cmd));
        button.connect_clicked(move |_| {
            let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
        });
        frame.append(&button);
    }

    dock.set_child(Some(&frame));

    dock.connect_realize(move |win| {
        if let Some(surface) = win.surface() {
            let x = screen.x + (screen.width - DOCK_WIDTH) / 2;
            let y = screen.y + screen.height - DOCK_HEIGHT - 16;
            let bottom = (DOCK_HEIGHT + 16) as u32;

            apply_x11_dock_hints(
                &surface,
                x,
                y,
                DOCK_WIDTH as u32,
                DOCK_HEIGHT as u32,
                StrutConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom,
                },
                PartialStrut {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom,
                    left_start_y: 0,
                    left_end_y: 0,
                    right_start_y: 0,
                    right_end_y: 0,
                    top_start_x: 0,
                    top_end_x: 0,
                    bottom_start_x: x.max(0) as u32,
                    bottom_end_x: (x + DOCK_WIDTH).max(0) as u32,
                },
            );
        }
    });

    dock.present();
}
