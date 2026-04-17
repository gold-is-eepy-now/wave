mod desktop;
mod dock;
mod menubar;

use gdk4 as gdk;
use gtk::prelude::*;
use gtk4 as gtk;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, PropMode};

#[derive(Debug, Clone, Copy)]
pub struct ScreenGeometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct StrutConfig {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct PartialStrut {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
    pub left_start_y: u32,
    pub left_end_y: u32,
    pub right_start_y: u32,
    pub right_end_y: u32,
    pub top_start_x: u32,
    pub top_end_x: u32,
    pub bottom_start_x: u32,
    pub bottom_end_x: u32,
}

pub fn monitor_geometry() -> ScreenGeometry {
    let display = gdk::Display::default().expect("No GDK display found");
    let monitors = display.monitors();
    if let Some(item) = monitors.item(0) {
        if let Ok(monitor) = item.downcast::<gdk::Monitor>() {
            let geometry = monitor.geometry();
            return ScreenGeometry {
                x: geometry.x(),
                y: geometry.y(),
                width: geometry.width(),
                height: geometry.height(),
            };
        }
    }

    // Safe fallback when no monitor could be inferred.
    ScreenGeometry {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
    }
}

fn atom_id<C: Connection>(conn: &C, name: &str) -> Option<u32> {
    conn.intern_atom(false, name.as_bytes())
        .ok()?
        .reply()
        .ok()
        .map(|r| r.atom)
}

/// Attempts to apply X11-only hints for dock/panel style windows:
/// - _NET_WM_WINDOW_TYPE_DOCK
/// - _NET_WM_STATE_ABOVE / STICKY / SKIP_TASKBAR / SKIP_PAGER
/// - _NET_WM_STRUT + _NET_WM_STRUT_PARTIAL edge reservation
///
/// No-op on non-X11 sessions.
pub fn apply_x11_dock_hints(
    surface: &gdk::Surface,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    strut: StrutConfig,
    partial: PartialStrut,
) {
    let Some(x11_surface) = surface.downcast_ref::<gdk4_x11::X11Surface>() else {
        eprintln!("AquaDE: surface is not X11; skipping EWMH struts and dock hints");
        return;
    };

    let xid = x11_surface.xid();
    let Ok((conn, _screen_num)) = x11rb::connect(None) else {
        eprintln!("AquaDE: unable to connect to X11 server for hints");
        return;
    };

    let wm_type = atom_id(&conn, "_NET_WM_WINDOW_TYPE");
    let wm_type_dock = atom_id(&conn, "_NET_WM_WINDOW_TYPE_DOCK");
    if let (Some(wm_type), Some(wm_type_dock)) = (wm_type, wm_type_dock) {
        let _ = conn.change_property32(
            PropMode::REPLACE,
            xid,
            wm_type,
            AtomEnum::ATOM,
            &[wm_type_dock],
        );
    }

    let wm_state = atom_id(&conn, "_NET_WM_STATE");
    let state_above = atom_id(&conn, "_NET_WM_STATE_ABOVE");
    let state_sticky = atom_id(&conn, "_NET_WM_STATE_STICKY");
    let state_skip_taskbar = atom_id(&conn, "_NET_WM_STATE_SKIP_TASKBAR");
    let state_skip_pager = atom_id(&conn, "_NET_WM_STATE_SKIP_PAGER");

    if let (Some(wm_state), Some(a), Some(s), Some(t), Some(p)) = (
        wm_state,
        state_above,
        state_sticky,
        state_skip_taskbar,
        state_skip_pager,
    ) {
        let _ = conn.change_property32(
            PropMode::REPLACE,
            xid,
            wm_state,
            AtomEnum::ATOM,
            &[a, s, t, p],
        );
    }

    if let Some(strut_atom) = atom_id(&conn, "_NET_WM_STRUT") {
        let _ = conn.change_property32(
            PropMode::REPLACE,
            xid,
            strut_atom,
            AtomEnum::CARDINAL,
            &[strut.left, strut.right, strut.top, strut.bottom],
        );
    }

    if let Some(strut_partial_atom) = atom_id(&conn, "_NET_WM_STRUT_PARTIAL") {
        let _ = conn.change_property32(
            PropMode::REPLACE,
            xid,
            strut_partial_atom,
            AtomEnum::CARDINAL,
            &[
                partial.left,
                partial.right,
                partial.top,
                partial.bottom,
                partial.left_start_y,
                partial.left_end_y,
                partial.right_start_y,
                partial.right_end_y,
                partial.top_start_x,
                partial.top_end_x,
                partial.bottom_start_x,
                partial.bottom_end_x,
            ],
        );
    }

    let aux = x11rb::protocol::xproto::ConfigureWindowAux::new()
        .x(x)
        .y(y)
        .width(width)
        .height(height);
    let _ = conn.configure_window(xid, &aux);
    let _ = conn.flush();
}

fn install_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_path("style/aqua.css");

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("org.aquade.shell")
        .build();

    app.connect_activate(|app| {
        install_css();

        let screen = monitor_geometry();
        desktop::build_desktop(app, screen);
        menubar::build_menubar(app, screen);
        dock::build_dock(app, screen);
    });

    app.run();
}
