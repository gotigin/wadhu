#!/usr/bin/env bash
# Build development Linux package (debug app id) + install/uninstall helpers.
# Output: dist/sms-gateway-linux-dev/
# Uses debug Cargo profile + the app's env/development.env (→ com.wadhu.smsgateway.debug)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
export WADHU_APP_DIR="$ROOT/apps/wadhu-sms-gateway"
export WADHU_ENV_DIR="$WADHU_APP_DIR/env"
export WADHU_BIN_NAME=wadhu-sms-gateway
export WADHU_INSTALL_SLUG=wadhu-sms-gateway
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
cargo build -p wadhu-sms-gateway
# shellcheck source=../../../scripts/cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
TARGET_DIR="$(cargo_target_dir)"
BIN="$TARGET_DIR/debug/wadhu-sms-gateway"
if [[ ! -f "$BIN" ]]; then
  echo "Missing binary: $BIN" >&2
  exit 1
fi

OUT="$ROOT/dist/sms-gateway-linux-dev"
rm -rf "$OUT"
mkdir -p "$OUT/bin"

cp -f "$BIN" "$OUT/bin/wadhu-sms-gateway"
chmod +x "$OUT/bin/wadhu-sms-gateway"
makepad_copy_desktop_resources "$OUT"
copy_app_menu_assets_if_present "$OUT"

stage_desktop_package "$OUT" linux

cat >"$OUT/README.txt" <<EOF
SMS Gateway Linux (development / debug)
==========================================
App id: ${WADHU_APP_ID}
Label: ${WADHU_APP_LABEL}

Install:
  ./install.sh
→ ~/.local/lib/wadhu-sms-gateway-debug/
→ command: wadhu-sms-gateway-dev
→ data: ~/.local/share/${WADHU_APP_ID}/

Uninstall:
  Uninstall SMS Gateway Dev (app menu)
  or: ./uninstall.sh --dev

Prod package: ./apps/wadhu-sms-gateway/scripts/package-linux.sh → dist/sms-gateway-linux/
Live from repo: cargo wadhu-sms-gateway-dev
EOF

echo "Linux Dev package: $OUT/"
