#!/usr/bin/env bash
# Install SMS Gateway Dev (debug app id) on Linux.
# Rebuilds dist/sms-gateway-linux-dev/ from the current tree, then installs it.
# Thin wrapper — same as: ./apps/wadhu-sms-gateway/scripts/install-linux.sh --dev
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
exec "$ROOT/install-linux.sh" --dev "$@"
