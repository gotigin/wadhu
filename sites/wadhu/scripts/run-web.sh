#!/usr/bin/env bash
# Serves wadhu.com Makepad site at http://127.0.0.1:8020
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
if [[ ! -d vendor/makepad-full/tools/cargo_makepad ]]; then
  echo "error: vendor/makepad-full missing — run ./scripts/setup-makepad.sh" >&2
  exit 1
fi
exec cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
  wasm --no-threads --port=8020 run -p wadhu-site "$@"
