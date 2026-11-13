#!/usr/bin/env bash
# Isolated UI smoke: private Xvfb display, never the user's desktop (:0 / Wayland).
#
# Builds (if needed) and launches a Wadhu app binary on DISPLAY=:$N, waits,
# takes a root PNG, then stops. Your session on :0 / wayland-0 is left alone.
#
# Usage (workspace root):
#   ./scripts/run-ui-smoke-isolated.sh templates/app
#   ./scripts/run-ui-smoke-isolated.sh apps/print-bridge
#
# Env:
#   SMOKE_DISPLAY   X display number (default: first free from 99)
#   SMOKE_OUT       output PNG path (default: /tmp/wadhu-ui-smoke-<pid>.png)
#   SMOKE_WAIT_SECS seconds before screenshot (default: 6)
#   SKIP_BUILD=1    skip cargo build
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

APP_PATH="${1:?usage: $0 apps/<app>|templates/app}"
[[ "$APP_PATH" = /* ]] || APP_PATH="$ROOT/$APP_PATH"
[[ -f "$APP_PATH/Cargo.toml" ]] || { echo "no Cargo.toml in $APP_PATH" >&2; exit 1; }
PKG="$(grep -m1 '^name = ' "$APP_PATH/Cargo.toml" | cut -d'"' -f2)"
[[ -n "$PKG" ]] || { echo "could not read package name from $APP_PATH/Cargo.toml" >&2; exit 1; }

need() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "need '$1' on PATH" >&2
    exit 1
  }
}
need Xvfb
need import

if [[ "${SKIP_BUILD:-}" != "1" ]]; then
  cargo build -p "$PKG"
fi

# shellcheck source=cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
BIN="$(cargo_target_dir)/debug/$PKG"
[[ -x "$BIN" ]] || { echo "missing binary: $BIN" >&2; exit 1; }

N="${SMOKE_DISPLAY:-}"
if [[ -z "$N" ]]; then
  N=99
  while [[ -e /tmp/.X11-unix/X$N || -e /tmp/.X$N-lock ]]; do N=$((N + 1)); done
fi
OUT="${SMOKE_OUT:-/tmp/wadhu-ui-smoke-$$.png}"
WAIT="${SMOKE_WAIT_SECS:-6}"

# Wide private display so a phone-tall window is not clipped on the right
# when the WM places it off-origin (root shots then look "cropped").
Xvfb :"$N" -screen 0 1200x1200x24 -nolisten tcp >/tmp/wadhu-xvfb-"$N".log 2>&1 &
XPID=$!
cleanup() { kill "$APID" "$XPID" 2>/dev/null || true; }
trap cleanup EXIT
sleep 0.4
env -u WAYLAND_DISPLAY DISPLAY=:"$N" "$BIN" >/tmp/wadhu-app-smoke-$$.log 2>&1 &
APID=$!
sleep "$WAIT"
# Prefer the app window (full frame). Fall back to root if xdotool is missing.
if command -v xdotool >/dev/null 2>&1; then
  WIN="$(DISPLAY=:"$N" xdotool search --name . 2>/dev/null | tail -1 || true)"
  if [[ -n "${WIN:-}" ]]; then
    DISPLAY=:"$N" xdotool windowmove "$WIN" 20 20 2>/dev/null || true
    DISPLAY=:"$N" import -window "$WIN" "$OUT"
  else
    DISPLAY=:"$N" import -window root "$OUT"
  fi
else
  DISPLAY=:"$N" import -window root "$OUT"
fi
echo "smoke ok: $PKG → $OUT"
