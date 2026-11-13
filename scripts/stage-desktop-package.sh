#!/usr/bin/env bash
# Stage the app's install/uninstall helpers + manifest into a desktop
# package folder. The helpers are each app's own (apps/<app>/scripts/:
# install-linux.sh, uninstall-linux.sh, install-windows.ps1,
# uninstall-windows.ps1, Setup.bat, Uninstall.bat, desktop-ids.sh) — they
# name that app's exe, folders and labels.
# Usage (after binary + resources are in $OUT):
#   source "$WADHU_APP_DIR/scripts/desktop-ids.sh"
#   # WADHU_* already exported (WADHU_APP_DIR, WADHU_BIN_NAME, env ids)
#   stage_desktop_package "$OUT" linux|windows
stage_desktop_package() {
  local out="${1:?}"
  local os="${2:?}" # linux | windows
  local scripts="${WADHU_APP_DIR:?stage_desktop_package: set WADHU_APP_DIR}/scripts"
  local bin="${WADHU_BIN_NAME:?stage_desktop_package: set WADHU_BIN_NAME}"

  mkdir -p "$out"
  cp -f "$scripts/desktop-ids.sh" "$out/desktop-ids.sh"

  # Manifest baked into the package (install copies it into the install dir).
  {
    echo "# Packaged with ${WADHU_APP_LABEL:?} — identifies prod vs debug install."
    echo "WADHU_ENV=${WADHU_ENV:?}"
    echo "WADHU_APP_ID=${WADHU_APP_ID:?}"
    echo "WADHU_APP_LABEL=${WADHU_APP_LABEL:?}"
    echo "INSTALL_SLUG=$(desktop_install_slug)"
    echo "BIN_NAME=$(desktop_bin_name)"
    echo "ENTRY_ID=$(desktop_entry_id)"
  } >"$out/install-manifest.env"

  if [[ "$os" == "linux" ]]; then
    cp -f "$scripts/install-linux.sh" "$out/install.sh"
    cp -f "$scripts/uninstall-linux.sh" "$out/uninstall.sh"
    chmod +x "$out/install.sh" "$out/uninstall.sh" "$out/desktop-ids.sh"
    # Portable launcher without full install (same layout as after install).
    cat >"$out/run.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
ROOT="\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)"
export MAKEPAD_PACKAGE_DIR=.
cd "\$ROOT"
exec "\$ROOT/bin/$bin" "\$@"
EOF
    chmod +x "$out/run.sh"
  else
    cp -f "$scripts/install-windows.ps1" "$out/install.ps1"
    cp -f "$scripts/uninstall-windows.ps1" "$out/uninstall.ps1"
    # Double-click entry points (opening .ps1 in Explorer often launches Notepad).
    cp -f "$scripts/Setup.bat" "$out/Setup.bat"
    cp -f "$scripts/Uninstall.bat" "$out/Uninstall.bat"
  fi
}
