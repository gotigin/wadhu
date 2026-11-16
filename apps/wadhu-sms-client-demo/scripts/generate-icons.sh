#!/usr/bin/env bash
# Regenerate THIS app's icons from its master (assets/app-icon.png) into
# resources/ (window/taskbar PNGs, .ico, Android mipmaps). Until the app has
# a master, it runs with the workspace default Wadhu mark.
set -euo pipefail
APP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$APP_DIR/../.." && pwd)"
exec "$ROOT/scripts/generate-app-icons.sh" "$APP_DIR/assets/app-icon.png" "$APP_DIR/resources" --android
