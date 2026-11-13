#!/usr/bin/env bash
# Install Wadhu Template Dev (debug app id) on Linux.
# Rebuilds dist/template-app-linux-dev/ from the current tree, then installs it.
# Thin wrapper — same as: ./templates/app/scripts/install-linux.sh --dev
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
exec "$ROOT/install-linux.sh" --dev "$@"
