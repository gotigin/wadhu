#!/usr/bin/env bash
# SMS Client Demo MSI (per-user install, no admin): dist/SmsClientDemo-<ver>.msi,
# or dist/SmsClientDemoDev-<ver>.msi for the development app id. Builds the
# Windows package first. Its own upgrade codes: installs and upgrades never
# collide with another app's MSI.
#   apps/wadhu-sms-client-demo/scripts/msi.sh        # prod
#   apps/wadhu-sms-client-demo/scripts/msi.sh dev    # dev
# After scripts/new-app.sh, regenerate the two upgrade-code UUIDs before the
# first release of that app, then never change them.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
export WADHU_APP_DIR="$ROOT/apps/wadhu-sms-client-demo"
export WADHU_BIN_NAME=wadhu-sms-client-demo
export WADHU_MSI_NAME=SmsClientDemo
export WADHU_MSI_UPGRADE_CODE="{58DF06EE-33F8-49DF-B519-2132577B3027}"
export WADHU_MSI_DEV_UPGRADE_CODE="{2602C096-9D73-4216-84D4-0A675C2B9058}"
export WADHU_MSI_PKG_PROD="$ROOT/dist/sms-client-demo-windows"
export WADHU_MSI_PKG_DEV="$ROOT/dist/sms-client-demo-windows-dev"
export WADHU_MSI_PACKAGE_PROD="$HERE/windows-prod.sh"
exec "$ROOT/scripts/build-msi.sh" "${1:-prod}"
