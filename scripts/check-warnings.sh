#!/usr/bin/env bash
# Fail if any supported-platform build emits rustc/cargo warnings.
# See docs/WARNINGS.md — fix warnings; never suppress.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Capture full cargo output; fail if a warning: line appears (manifest or rustc).
run_clean() {
  local label="$1"
  shift
  local log
  log="$(mktemp)"
  echo "==> $label"
  if ! "$@" >"$log" 2>&1; then
    echo "error: command failed ($label)" >&2
    cat "$log" >&2
    rm -f "$log"
    exit 1
  fi
  if rg -n '^warning:' "$log" >/dev/null; then
    echo "error: warnings under $label (fix them — docs/WARNINGS.md)" >&2
    rg -n 'warning:' "$log" >&2 || true
    rm -f "$log"
    exit 1
  fi
  rm -f "$log"
}

run_clean "workspace host (all-targets)" \
  cargo build --workspace --all-targets

run_clean "wadhu-sms-gateway windows" \
  cargo check -p wadhu-sms-gateway --target x86_64-pc-windows-gnu

run_clean "wadhu-sms-gateway android aarch64 (lib)" \
  cargo check -p wadhu-sms-gateway --target aarch64-linux-android --lib

run_clean "wadhu-site wasm32" \
  cargo check -p wadhu-site --target wasm32-unknown-unknown

run_clean "wadhu-connect host" \
  cargo check -p wadhu-connect

echo "OK — no warnings on checked platforms (docs/WARNINGS.md)."
