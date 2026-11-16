#!/usr/bin/env bash
set -euo pipefail
APP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$APP_DIR/../.." && pwd)"
exec "$ROOT/scripts/generate-app-icons.sh" "$APP_DIR/assets/app-icon.png" "$APP_DIR/resources"
