# SnowDesk

SnowDesk is a from-scratch desktop environment prototype for Debian Linux that recreates the look and workflow of Mac OS X 10.6 Snow Leopard.

## High-level architecture

### 1. `wm` — Wayland compositor/window manager skeleton
- Current backend support: **X11-first runtime (LightDM compatible)** with explicit Wayland mode flag for future wlroots integration.
- Current prototype provides:
  - scene/window model abstraction
  - animation scheduler (150–300 ms easing)
  - Exposé-style overview layout algorithm (grid)
  - IPC event stub for panel/dock integration
- Production expansion points:
  - real Wayland surfaces and outputs
  - blur and drop-shadow rendering pass
  - focus rings, open/close/minimize transitions

### 2. `panel` — global menubar prototype
- Top anchored menubar model with:
  - dynamic app menu title source
  - status area model (Wi-Fi, volume, battery, clock)
  - clock update loop
- IPC-ready API boundary (`PanelEvent`) for updates from the compositor.

### 3. `dock` — bottom dock prototype
- Aqua-like dock data model with:
  - app icon list + running indicators
  - magnification curve on hover
  - reordering support with drag target index
- Animation hooks prepared for smooth scale transitions.

### 4. `finder` — Finder clone scaffold
- View modes: Icon / List / Column.
- Sidebar locations and directory model.
- Hooks for animated transitions and icon-size slider.

### 5. `spotlight` — launcher/search scaffold
- Query parsing and instant result model.
- Supports app/file/calculation result kinds.
- Designed for keyboard-first invocation.

### 6. `ui` — shared Aqua design system
- Reusable style tokens for:
  - gradients, blue selection glow, rounded radii
  - traffic-light controls
  - motion curve presets (`EaseInOutCubic`)
- Shared constants consumed by all other components.

## Step-by-step build plan

1. **Foundation**
   - Create Rust workspace and component crates.
   - Implement shared `ui` style primitives and motion tokens.

2. **Compositor core**
   - Integrate wlroots/Smithay backend for real Wayland session.
   - Wire surface lifecycle to internal scene graph.
   - Implement GPU effects (shadow, blur, translucency).

3. **Desktop shell services**
   - Add DBus service for app menu export + status notifier bridge.
   - Connect `panel` and `dock` to compositor IPC.

4. **Shell UX features**
   - Exposé overview with live window thumbnails.
   - Spaces (virtual desktops) with animated transitions.
   - Dock spring-loading and bounce animations.

5. **Applications**
   - Expand `finder` with real file IO and icon/list/column views.
   - Expand `spotlight` with file index + app launch + calculator.

6. **Packaging/session**
   - Install binaries, assets, configs.
   - Register `snowdesk.desktop` session entry.
   - Add Debian helper script for dependency setup.

## Build and run on Debian

### Dependencies
- Rust toolchain (`cargo`, `rustc`)
- Runtime targets for final implementation (future):
  - `libwayland-dev`, `wlroots`, `libxkbcommon-dev`, `libdbus-1-dev`, `mesa`

### Compile prototype
```bash
cd snowdesk
cargo build --workspace
```

### Run components individually
```bash
cargo run -p wm -- --oneshot
cargo run -p panel -- --oneshot
cargo run -p dock -- --oneshot
```

### Run an X11 shell session manually (for LightDM/X11 systems)
```bash
bash scripts/run_x11_session.sh
```
### Install session prototype (LightDM friendly)
```bash
bash scripts/install_debian.sh
```

This installs binaries under `/usr/local/bin`, registers `/usr/share/xsessions/snowdesk.desktop` for LightDM/X11, and also installs a Wayland session file.
