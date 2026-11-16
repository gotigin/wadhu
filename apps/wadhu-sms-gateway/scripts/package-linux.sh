#!/usr/bin/env bash
# Build production Linux package for SMS Gateway + install/uninstall helpers.
# Output: dist/sms-gateway-linux/
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
load_wadhu_env production
makepad_enable_package_dir

# Window/taskbar icons: app resources/ if present, else workspace default.
export_wadhu_app_icons "$WADHU_APP_DIR"
cargo build -p wadhu-sms-gateway --release
# shellcheck source=../../../scripts/cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
TARGET_DIR="$(cargo_target_dir)"
BIN="$TARGET_DIR/release/wadhu-sms-gateway"
if [[ ! -f "$BIN" ]]; then
  echo "Missing binary: $BIN" >&2
  exit 1
fi

OUT="$ROOT/dist/sms-gateway-linux"
rm -rf "$OUT"
mkdir -p "$OUT/bin"

cp -f "$BIN" "$OUT/bin/wadhu-sms-gateway"
chmod +x "$OUT/bin/wadhu-sms-gateway"
# Makepad: resources at <package>/wadhu_sms_gateway/resources/ (underscores; cwd = package root).
# Binary lives under bin/ (hyphenated name) so it never collides with the resources dir.
makepad_copy_desktop_resources "$OUT"
copy_app_menu_assets_if_present "$OUT"

stage_desktop_package "$OUT" linux

cat >"$OUT/README.txt" <<EOF
SMS Gateway Linux (production)
=================================
App id: ${WADHU_APP_ID}
Label: ${WADHU_APP_LABEL}

Install (recommended — adds app menu + Uninstall entry):
  ./install.sh

Portable run (no install):
  ./run.sh

Uninstall (after install, or re-run to delete leftover data):
  Uninstall SMS Gateway  (app menu)
  or: ~/.local/lib/wadhu-sms-gateway/uninstall.sh
  or: ./uninstall.sh --prod

Uninstall can delete local data under:
  ${HOME}/.local/share/${WADHU_APP_ID}/

EOF

echo "Linux package: $OUT/"
echo "  Install with: $OUT/install.sh"
