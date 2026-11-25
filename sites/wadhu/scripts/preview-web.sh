#!/usr/bin/env bash
# Stage the wadhu.com tree the way deploy does, then serve it locally.
# No Firebase upload.
#
#   ./sites/wadhu/scripts/preview-web.sh
#   ./sites/wadhu/scripts/preview-web.sh --no-build
#
# --no-build skips the wasm compile. If a release package is already in
# target/, it is staged; otherwise public/ still receives static/ and
# /i18n so pack URLs can be checked.
# Serves http://127.0.0.1:8020
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
OUT="$SITE/public"
WASM_PKG="$ROOT/target/makepad-wasm-app/release/wadhu-site"
PORT="${PORT:-8020}"
DO_BUILD=1

for arg in "$@"; do
  case "$arg" in
    --no-build) DO_BUILD=0 ;;
    -h|--help) sed -n '2,14p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

if [[ "$DO_BUILD" -eq 1 ]]; then
  echo "1/3 Build Makepad wasm (wadhu-site, release)"
  (
    cd "$ROOT"
    cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
      wasm --no-threads --strip build -p wadhu-site --release
  )
fi

mkdir -p "$OUT"
if [[ -f "$WASM_PKG/index.html" && -f "$WASM_PKG/wadhu-site.wasm" ]]; then
  echo "Stage $OUT from $WASM_PKG"
  rsync -a --exclude='*.br' "$WASM_PKG/" "$OUT/"
  "$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
else
  echo "note: no release wasm at $WASM_PKG — staging static and /i18n only" >&2
  rsync -a "$SITE/static/" "$OUT/"
fi

# Language packs (site i18n): the wasm and site-i18n.js fetch /i18n/<code>.tsv.
mkdir -p "$OUT/i18n" "$OUT/js"
cp "$SITE"/translations/*.tsv "$OUT/i18n/"
cp "$SITE/static/js/lang-nav.js" "$OUT/js/lang-nav.js"
cp "$SITE/static/js/site-i18n.js" "$OUT/js/site-i18n.js"

if command -v fuser >/dev/null 2>&1; then
  if fuser "${PORT}/tcp" >/dev/null 2>&1; then
    echo "Port ${PORT} in use — stopping previous listener."
    fuser -k "${PORT}/tcp" >/dev/null 2>&1 || true
    sleep 0.5
  fi
fi

echo "Serving $OUT at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    /i18n/hi.tsv and /i18n/ur.tsv are the Hindi and Urdu packs"
cd "$OUT"
exec python3 -m http.server "$PORT" --bind 127.0.0.1
