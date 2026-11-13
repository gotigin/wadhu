#!/usr/bin/env bash
# Development Windows x86_64 package (debug *app id*, not Cargo debug).
# Uses --release so the .exe stays sane (~tens of MB). Cargo debug PE with
# Makepad was ~400MB — unusable to copy. Identity still comes from
# development.env → com.wadhu.template.debug (separate install/data from prod).
# Makepad layout: wadhu-template-app.exe + wadhu_template_app/resources/ (underscores).
#
#   windows-dev.sh        development package  -> dist/template-app-windows-dev/
#   windows-dev.sh prod   production package   -> dist/template-app-windows/
#                         (the MSI: templates/app/scripts/msi.sh)
set -euo pipefail
FLAVOR="${1:-dev}"
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
export WADHU_APP_DIR="$ROOT/templates/app"
export WADHU_ENV_DIR="$WADHU_APP_DIR/env"
# Desktop install identity for THIS app (exe, install folder slug).
export WADHU_BIN_NAME=wadhu-template-app
export WADHU_INSTALL_SLUG=wadhu-template-app
# shellcheck source=../../../scripts/load-wadhu-env.sh
source "$ROOT/scripts/load-wadhu-env.sh"
# shellcheck source=../../../scripts/windows-env.sh
source "$ROOT/scripts/windows-env.sh"
# Wadhu Template's own install helpers (Setup.bat / install.ps1 / uninstall name
# Wadhu Template's exe and folders.
# shellcheck source=desktop-ids.sh
source "$WADHU_APP_DIR/scripts/desktop-ids.sh"
# shellcheck source=../../../scripts/stage-desktop-package.sh
source "$ROOT/scripts/stage-desktop-package.sh"

export WADHU_ROOT="$ROOT"
case "$FLAVOR" in
  prod) load_wadhu_env production; OUT_NAME=template-app-windows ;;
  dev) load_wadhu_env development; OUT_NAME=template-app-windows-dev ;;
  *) echo "usage: $0 [dev|prod]" >&2; exit 1 ;;
esac
makepad_enable_package_dir

OUT="$ROOT/dist/$OUT_NAME"
rm -rf "$OUT"
mkdir -p "$OUT"
TARGET="$(windows_default_target)"

if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* || "$(uname -s)" == CYGWIN* ]]; then
  if rustup target list --installed 2>/dev/null | grep -qx 'x86_64-pc-windows-msvc'; then
    TARGET=x86_64-pc-windows-msvc
  fi
else
  windows_ensure_toolchain
fi

# Same as Android: release binary + development.env (console stays — WADHU_ENV).
# Window/taskbar icons: app resources/ if present, else workspace default.
export_wadhu_app_icons "$WADHU_APP_DIR"
cargo build -p wadhu-template-app --release --target "$TARGET"

# shellcheck source=../../../scripts/cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
TARGET_DIR="$(cargo_target_dir)"
EXE=""
for cand in \
  "$TARGET_DIR/$TARGET/release/wadhu-template-app.exe" \
  "$TARGET_DIR/$TARGET/release/wadhu-template-app"
do
  if [[ -f "$cand" ]]; then
    EXE="$cand"
    break
  fi
done
if [[ -z "$EXE" ]]; then
  echo "Built binary not found under $TARGET_DIR/$TARGET/release/" >&2
  exit 1
fi

cp -f "$EXE" "$OUT/wadhu-template-app.exe"
makepad_copy_desktop_resources "$OUT"
copy_app_menu_assets_if_present "$OUT"
stage_desktop_package "$OUT" windows

SIZE="$(du -h "$OUT/wadhu-template-app.exe" | awk '{print $1}')"

if [ "$FLAVOR" = prod ]; then CONSOLE_NOTE="off (production.env)"; else CONSOLE_NOTE="on (development.env) — prod hides it."; fi
cat >"$OUT/README.txt" <<EOF
Wadhu Template Windows ($FLAVOR)
====================================
App id: ${WADHU_APP_ID}
Label: ${WADHU_APP_LABEL}
Cargo: --release (optimized). "Dev" means debug app id / data dir, not a 400MB debug PE.
Exe size: ${SIZE}
Target: $TARGET
Console: ${CONSOLE_NOTE}

Install:
  Double-click Setup.bat
→ %LOCALAPPDATA%\\Programs\\${WADHU_APP_LABEL}\\
→ data: %APPDATA%\\${WADHU_APP_ID}\\

Uninstall:
  Start Menu → Uninstall Wadhu Template Dev
  or double-click Uninstall.bat

Prod: ./templates/app/scripts/windows-dev.sh prod → dist/template-app-windows/
MSI:  ./templates/app/scripts/msi.sh [prod|dev] → dist/Wadhu Template-<ver>.msi
EOF

echo "Windows $FLAVOR package: $OUT/  (exe ${SIZE})"
echo "Copy the whole folder to Windows (exe + wadhu_template_app/resources + assets/ + Setup.bat)."
