#!/usr/bin/env bash
# SMS Gateway MSI (per-user install, no admin): dist/SmsGateway-<ver>.msi,
# or dist/SmsGatewayDev-<ver>.msi for the development app id. Builds the
# Windows package first. Its own upgrade codes: installs and upgrades never
# collide with another app's MSI.
#   apps/sms-gateway/scripts/msi.sh        # prod
#   apps/sms-gateway/scripts/msi.sh dev    # dev
# After scripts/new-app.sh, regenerate the two upgrade-code UUIDs before the
# first release of that app, then never change them.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
export WADHU_APP_DIR="$ROOT/apps/sms-gateway"
export WADHU_BIN_NAME=wadhu-sms-gateway
export WADHU_MSI_NAME=SmsGateway
export WADHU_MSI_UPGRADE_CODE="{1F238DD6-D81D-4C1A-94A5-90D3CA10047B}"
export WADHU_MSI_DEV_UPGRADE_CODE="{9D4C7A8B-7CEE-4256-AC4B-1AF0FED581EA}"
export WADHU_MSI_PKG_PROD="$ROOT/dist/sms-gateway-windows"
export WADHU_MSI_PKG_DEV="$ROOT/dist/sms-gateway-windows-dev"
export WADHU_MSI_PACKAGE_PROD="$HERE/windows-prod.sh"
exec "$ROOT/scripts/build-msi.sh" "${1:-prod}"
