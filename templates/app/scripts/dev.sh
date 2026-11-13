#!/usr/bin/env bash
# Desktop live-dev for THIS app: loads env/development.env, then runs with
# --hot (Splash/script_mod! hot reload on save) and --remote (F12 tweaker).
# Usage: ./templates/app/scripts/dev.sh   or: cargo template-app-dev
set -euo pipefail
APP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$APP_DIR/../.." && pwd)"
cd "$ROOT"
# shellcheck source=../../../scripts/load-wadhu-env.sh
source "$ROOT/scripts/load-wadhu-env.sh"
export WADHU_ROOT="$ROOT"
export WADHU_APP_DIR="$APP_DIR"
export WADHU_ENV_DIR="$APP_DIR/env"
load_wadhu_env development
# This app's own window/taskbar icons when it has them (resources/icon_*.png);
# otherwise the workspace default Wadhu mark from .cargo/config.toml.
export_wadhu_app_icons "$APP_DIR"
exec cargo run -p wadhu-template-app -- "$@" --hot --remote
