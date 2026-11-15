#!/usr/bin/env bash
# Stage wadhu.com public/ and deploy to Firebase Hosting.
# One-time: npm i -g firebase-tools && firebase login
# Firebase Console → create project, set .firebaserc, add custom domain wadhu.com
#
#   ./sites/wadhu/scripts/deploy-firebase.sh
#   ./sites/wadhu/scripts/deploy-firebase.sh --dry-run
#   ./sites/wadhu/scripts/deploy-firebase.sh --with-wasm   # if wadhu-site wasm built
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
OUT="$SITE/public"
DRY=0
WITH_WASM=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
    --with-wasm) WITH_WASM=1 ;;
    -h|--help) sed -n '2,12p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v firebase >/dev/null 2>&1; then
  echo "error: firebase CLI not found (npm i -g firebase-tools)" >&2
  exit 1
fi

echo "1/2 Stage $OUT from static/ (+ optional Makepad wasm)"
rm -rf "$OUT"
mkdir -p "$OUT"
rsync -a "$SITE/static/" "$OUT/"

if [[ "$WITH_WASM" -eq 1 ]]; then
  WASM_PKG="$ROOT/target/makepad-wasm-app/release/wadhu-site"
  if [[ ! -f "$WASM_PKG/index.html" ]]; then
    echo "Building wadhu-site wasm…"
    (
      cd "$ROOT"
      cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
        wasm --no-threads --strip --brotli build -p wadhu-site --release
    )
  fi
  if [[ -f "$WASM_PKG/index.html" ]]; then
    # Keep legal routes from static/; wasm becomes the interactive home when present.
    rsync -a "$WASM_PKG/" "$OUT/"
    rsync -a "$SITE/static/privacy/" "$OUT/privacy/"
    rsync -a "$SITE/static/terms/" "$OUT/terms/"
    rsync -a "$SITE/static/about/" "$OUT/about/"
    rsync -a "$SITE/static/css/" "$OUT/css/"
    # Prefer static shell index if wasm index is only a canvas stub without legal links —
    # when both exist, keep wasm index and ensure /privacy still works.
    echo "Staged Makepad wasm package."
  else
    echo "warn: wasm package missing — deploying static shell only" >&2
  fi
fi

echo "2/2 Deploy Firebase Hosting → wadhu.com (project from .firebaserc)"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --dry-run
else
  firebase deploy --only hosting
fi
