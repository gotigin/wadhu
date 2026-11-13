#!/usr/bin/env bash
# Build development Linux package (debug app id) + install/uninstall helpers.
# Output: dist/template-app-linux-dev/
# Uses debug Cargo profile + the app's env/development.env (→ com.wadhu.template.debug)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
export WADHU_APP_DIR="$ROOT/templates/app"
export WADHU_ENV_DIR="$WADHU_APP_DIR/env"
export WADHU_BIN_NAME=wadhu-template-app
export WADHU_INSTALL_SLUG=wadhu-template-app
# shellcheck source=../../../scripts/load-wadhu-env.sh
source "$ROOT/scripts/load-wadhu-env.sh"
# shellcheck source=../../../scripts/windows-env.sh
source "$ROOT/scripts/windows-env.sh"
# shellcheck source=desktop-ids.sh
source "$WADHU_APP_DIR/scripts/desktop-ids.sh"
# shellcheck source=../../../scripts/stage-desktop-package.sh
source "$ROOT/scripts/stage-desktop-package.sh"

export WADHU_ROOT="$ROOT"
load_wadhu_env development
makepad_enable_package_dir

# Window/taskbar icons: app resources/ if present, else workspace default.
export_wadhu_app_icons "$WADHU_APP_DIR"
cargo build -p wadhu-template-app
# shellcheck source=../../../scripts/cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
TARGET_DIR="$(cargo_target_dir)"
BIN="$TARGET_DIR/debug/wadhu-template-app"
if [[ ! -f "$BIN" ]]; then
  echo "Missing binary: $BIN" >&2
  exit 1
fi

OUT="$ROOT/dist/template-app-linux-dev"
rm -rf "$OUT"
mkdir -p "$OUT/bin"

cp -f "$BIN" "$OUT/bin/wadhu-template-app"
chmod +x "$OUT/bin/wadhu-template-app"
makepad_copy_desktop_resources "$OUT"
copy_app_menu_assets_if_present "$OUT"

stage_desktop_package "$OUT" linux

cat >"$OUT/README.txt" <<EOF
Wadhu Template Linux (development / debug)
==========================================
App id: ${WADHU_APP_ID}
Label: ${WADHU_APP_LABEL}

Install:
  ./install.sh
→ ~/.local/lib/wadhu-template-app-debug/
→ command: wadhu-template-app-dev
→ data: ~/.local/share/${WADHU_APP_ID}/

Uninstall:
  Uninstall Wadhu Template Dev (app menu)
  or: ./uninstall.sh --dev

Prod package: ./templates/app/scripts/package-linux.sh → dist/template-app-linux/
Live from repo: cargo template-app-dev
EOF

echo "Linux Dev package: $OUT/"
