use gtk::cairo;
use gtk::prelude::*;
use gtk4 as gtk;

use crate::ScreenGeometry;

/// Builds the full-screen desktop background with a painted Aqua-like gradient
/// and a handful of clickable shortcut buttons in the top-right corner.
pub fn build_desktop(app: &gtk::Application, screen: ScreenGeometry) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("AquaDE Desktop")
        .decorated(false)
        .default_width(screen.width)
        .default_height(screen.height)
        .resizable(false)
        .build();

    window.add_css_class("desktop-window");
    window.fullscreen();

    let overlay = gtk::Overlay::new();

    // Custom background paint using Cairo (gradient + glossy highlight).
    let background = gtk::DrawingArea::new();
    background.set_content_width(screen.width);
    background.set_content_height(screen.height);
    background.set_draw_func(move |_, cr, width, height| {
        paint_aqua_background(cr, width as f64, height as f64);
    });
    overlay.set_child(Some(&background));

    let shortcuts = gtk::Box::new(gtk::Orientation::Vertical, 8);
    shortcuts.add_css_class("shortcut-column");
    shortcuts.set_halign(gtk::Align::End);
    shortcuts.set_valign(gtk::Align::Start);
    shortcuts.set_margin_top(54);
    shortcuts.set_margin_end(20);

    for (label, command) in [
        ("Terminal", "x-terminal-emulator"),
        ("Files", "nautilus"),
        ("Browser", "xdg-open https://example.org"),
    ] {
        let button = gtk::Button::with_label(label);
        button.add_css_class("desktop-shortcut");
        button.connect_clicked(move |_| {
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .spawn();
        });
        shortcuts.append(&button);
    }

    overlay.add_overlay(&shortcuts);
    window.set_child(Some(&overlay));
    window.present();
}

fn paint_aqua_background(cr: &cairo::Context, width: f64, height: f64) {
    let gradient = cairo::LinearGradient::new(0.0, 0.0, 0.0, height);
    gradient.add_color_stop_rgba(0.00, 0.10, 0.34, 0.67, 1.0);
    gradient.add_color_stop_rgba(0.55, 0.16, 0.50, 0.86, 1.0);
    gradient.add_color_stop_rgba(1.00, 0.06, 0.23, 0.50, 1.0);

    cr.rectangle(0.0, 0.0, width, height);
    cr.set_source(&gradient);
    cr.fill().expect("fill gradient");

    // Aqua gloss pass near the top.
    let gloss = cairo::LinearGradient::new(0.0, 0.0, 0.0, height * 0.38);
    gloss.add_color_stop_rgba(0.0, 1.0, 1.0, 1.0, 0.28);
    gloss.add_color_stop_rgba(1.0, 1.0, 1.0, 1.0, 0.0);
    cr.rectangle(0.0, 0.0, width, height * 0.38);
    cr.set_source(&gloss);
    cr.fill().expect("fill gloss");
}
