#!/usr/bin/env bash
set -euo pipefail

# Local developer helper: run SnowDesk directly inside current X11 login.
export SNOWDESK_BACKEND=x11
exec ./session/snowdesk-session
