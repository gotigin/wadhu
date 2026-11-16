#!/usr/bin/env bash
# Uninstall SMS Client Demo (production) on Linux.
# Thin wrapper — same as: ./apps/wadhu-sms-client-demo/scripts/uninstall-linux.sh --prod
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
exec "$ROOT/uninstall-linux.sh" --prod "$@"
