# AquaDE (Rust + GTK4)

A minimal, modular desktop-shell prototype inspired by **macOS Aqua**, rewritten in Rust + GTK4.

> ⚠️ This is a shell prototype for X11 sessions (best tested with lightweight WMs such as Openbox, i3, or bspwm). Running without a WM is possible but less complete.

## Project layout

```text
.
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   ├── desktop.rs
│   ├── dock.rs
│   └── menubar.rs
└── style
    └── aqua.css
```

## Features in this starter

- Top menubar/panel with live clock
- Bottom dock with launch buttons
- Fullscreen desktop window with Cairo gradient background
- Clickable desktop shortcuts (top-right)
- X11 EWMH helper for:
  - `_NET_WM_WINDOW_TYPE_DOCK`
  - `_NET_WM_STATE_*` hints (above, sticky, skip taskbar/pager)
  - `_NET_WM_STRUT` and `_NET_WM_STRUT_PARTIAL` reservations

## Debian/Ubuntu dependencies

Install Rust and GTK4 dev packages:

```bash
sudo apt update
sudo apt install -y \
  build-essential pkg-config curl git \
  libgtk-4-dev libglib2.0-dev libpango1.0-dev libcairo2-dev \
  libx11-dev libxfixes-dev libxext-dev

# Rust toolchain (if needed)
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

## Build & run

```bash
cargo run
```

## Running as a desktop shell via `.xinitrc`

Use AquaDE inside an X11 session with a WM for full behavior.

Example `~/.xinitrc`:

```bash
#!/bin/sh
xsetroot -cursor_name left_ptr

# Start a WM first (recommended):
openbox &
# or: i3 &
# or: bspwm &

cd /path/to/aquade
exec cargo run --release
```

Then:

```bash
startx
```

### Why WM recommended?

A shell without a WM may have:
- focus oddities,
- unusual stacking behavior,
- missing shortcuts/window controls in app windows.

## Common pitfalls & fixes

### 1) `pkg-config` can't find GTK4

Symptom:
- build fails mentioning `gtk4.pc`.

Fix:
- install `libgtk-4-dev`, ensure `pkg-config --modversion gtk4` works.

### 2) `glib` / `gtk4` crate version mismatch

Symptom:
- trait/type mismatch between `glib` and `gtk4` types.

Fix:
- pin compatible versions in `Cargo.toml` (as in this project) and run:

```bash
cargo update
cargo clean
cargo build
```

### 3) `clone!` macro issues

Symptom:
- macro not found or lifetime errors.

Fix:
- import `gtk::glib` (or `glib`) and call `glib::clone!` explicitly.
- this prototype intentionally avoids unnecessary macro-heavy closures.

### 4) Cursor missing in bare X sessions

Fix:
- set one in `.xinitrc` using:

```bash
xsetroot -cursor_name left_ptr
```

### 5) Dock/panel not always on top

Some WMs ignore/override hints differently.

Fix options:
- use a WM known to honor EWMH strongly,
- configure WM rules (window type `dock` above normal windows),
- consider integrating a layer-shell strategy for Wayland sessions separately.

## Extending AquaDE

- **Wallpapers:** Replace Cairo painting with `gdk::Texture` + `gtk::Picture`, then blend gradient overlays.
- **Launcher integration:** parse `.desktop` files from `/usr/share/applications` and `$HOME/.local/share/applications`.
- **WM integration:**
  - for X11: consume `_NET_CLIENT_LIST` and implement task switching,
  - for Wayland: move to compositor-side protocols and `gtk4-layer-shell` patterns.
- **Dock UX:** magnification, drag/reorder, right-click menus, running indicators.
- **Panel extras:** status menu, volume/network battery widgets, notifications.

## Notes

- This starter targets clarity and hackability over feature completeness.
- EWMH edge reservations are implemented via `x11rb` and only apply on X11.
