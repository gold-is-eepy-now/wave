#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -ne 0 ]]; then
  echo "Please run as root: sudo bash scripts/install_debian.sh"
  exit 1
fi

apt-get update
apt-get install -y --no-install-recommends \
  build-essential curl pkg-config libwayland-dev libxkbcommon-dev libdbus-1-dev xorg

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "$ROOT_DIR"
cargo build --release --workspace
install -Dm755 target/release/wm /usr/local/bin/snowdesk-wm
install -Dm755 target/release/panel /usr/local/bin/snowdesk-panel
install -Dm755 target/release/dock /usr/local/bin/snowdesk-dock
install -Dm755 session/snowdesk-session /usr/local/bin/snowdesk-session
install -Dm644 session/snowdesk.desktop /usr/share/wayland-sessions/snowdesk.desktop
install -Dm644 session/snowdesk-x11.desktop /usr/share/xsessions/snowdesk.desktop

echo "SnowDesk prototype installed. In LightDM, choose 'SnowDesk (X11)'."
