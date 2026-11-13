#!/usr/bin/env bash
# Wadhu Template MSI (per-user install, no admin): dist/WadhuTemplate-<ver>.msi,
# or dist/WadhuTemplateDev-<ver>.msi for the development app id. Builds the
# Windows package first. Its own upgrade codes: installs and upgrades never
# collide with another app's MSI.
#   templates/app/scripts/msi.sh        # prod
#   templates/app/scripts/msi.sh dev    # dev
# After scripts/new-app.sh, regenerate the two upgrade-code UUIDs before the
# first release of that app, then never change them.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
export WADHU_APP_DIR="$ROOT/templates/app"
export WADHU_BIN_NAME=wadhu-template-app
export WADHU_MSI_NAME=WadhuTemplate
export WADHU_MSI_UPGRADE_CODE="{29EB5499-71A7-496A-ADD1-143B78ABC71C}"
export WADHU_MSI_DEV_UPGRADE_CODE="{52CD0DCE-2A6E-49FC-84D2-7FCC11BD397D}"
export WADHU_MSI_PKG_PROD="$ROOT/dist/template-app-windows"
export WADHU_MSI_PKG_DEV="$ROOT/dist/template-app-windows-dev"
export WADHU_MSI_PACKAGE_PROD="$HERE/windows-prod.sh"
exec "$ROOT/scripts/build-msi.sh" "${1:-prod}"
