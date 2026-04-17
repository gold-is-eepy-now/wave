use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

use crate::{apply_x11_dock_hints, PartialStrut, ScreenGeometry, StrutConfig};

const PANEL_HEIGHT: i32 = 34;

pub fn build_menubar(app: &gtk::Application, screen: ScreenGeometry) {
    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("AquaDE Panel")
        .decorated(false)
        .resizable(false)
        .default_width(screen.width)
        .default_height(PANEL_HEIGHT)
        .build();

    bar.add_css_class("panel-window");

    let row = gtk::CenterBox::new();
    row.add_css_class("panel-row");

    let left = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let logo = gtk::Label::new(Some(" AquaDE"));
    logo.add_css_class("panel-brand");
    left.append(&logo);

    let center = gtk::Label::new(Some("Desktop Shell Prototype"));
    center.add_css_class("panel-title");

    let right = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let clock = gtk::Label::new(None);
    clock.add_css_class("panel-clock");
    right.append(&clock);

    row.set_start_widget(Some(&left));
    row.set_center_widget(Some(&center));
    row.set_end_widget(Some(&right));
    bar.set_child(Some(&row));

    // Live clock update.
    let update_clock = move || {
        clock.set_text(&Local::now().format("%a %Y-%m-%d %H:%M:%S").to_string());
        glib::ControlFlow::Continue
    };
    update_clock();
    glib::timeout_add_seconds_local(1, update_clock);

    bar.connect_realize(move |win| {
        if let Some(surface) = win.surface() {
            let x = screen.x;
            let y = screen.y;

            apply_x11_dock_hints(
                &surface,
                x,
                y,
                screen.width as u32,
                PANEL_HEIGHT as u32,
                StrutConfig {
                    left: 0,
                    right: 0,
                    top: PANEL_HEIGHT as u32,
                    bottom: 0,
                },
                PartialStrut {
                    left: 0,
                    right: 0,
                    top: PANEL_HEIGHT as u32,
                    bottom: 0,
                    left_start_y: 0,
                    left_end_y: 0,
                    right_start_y: 0,
                    right_end_y: 0,
                    top_start_x: 0,
                    top_end_x: screen.width as u32,
                    bottom_start_x: 0,
                    bottom_end_x: 0,
                },
            );
        }
    });

    bar.present();
}
