#!/usr/bin/env bash
# Run wadhu.com locally — one tree like deploy (Khaynjo run-web):
#   1) build debug wasm
#   2) overlay /privacy /terms /about … beside Makepad home
#   3) python http.server
#
# Serves at http://127.0.0.1:8020
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
PKG="$ROOT/target/makepad-wasm-app/debug/wadhu-site"
PORT="${PORT:-8020}"

cd "$ROOT"
if [[ ! -d vendor/makepad-full/tools/cargo_makepad ]]; then
  echo "error: vendor/makepad-full missing — run ./scripts/setup-makepad.sh" >&2
  exit 1
fi

cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
  wasm --no-threads build -p wadhu-site "$@"

if [[ ! -f "$PKG/index.html" ]]; then
  echo "error: missing $PKG/index.html" >&2
  exit 1
fi

"$SITE/scripts/overlay-legal-onto-pkg.sh" "$PKG"
# Language packs (site i18n): the wasm and site-i18n.js fetch /i18n/<code>.tsv.
# patch-web-shell.sh (via the overlay) injects lang-nav.js for the language chip.
mkdir -p "$PKG/i18n" "$PKG/js"
cp "$SITE"/translations/*.tsv "$PKG/i18n/"
cp "$SITE/static/js/lang-nav.js" "$PKG/js/lang-nav.js"
cp "$SITE/static/js/site-i18n.js" "$PKG/js/site-i18n.js"

# Free :$PORT if a previous run still holds it.
if command -v fuser >/dev/null 2>&1; then
  if fuser "${PORT}/tcp" >/dev/null 2>&1; then
    echo "Port ${PORT} in use — stopping previous listener."
    fuser -k "${PORT}/tcp" >/dev/null 2>&1 || true
    sleep 0.5
  fi
elif command -v ss >/dev/null 2>&1; then
  if ss -ltn "( sport = :${PORT} )" 2>/dev/null | rg -q ":${PORT}"; then
    echo "error: port ${PORT} already in use. Stop it, then re-run:" >&2
    echo "  fuser -k ${PORT}/tcp" >&2
    exit 1
  fi
fi

echo "Serving $PKG at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    One tree: / = Makepad SPA; /privacy /terms /about = same site"
echo "    /i18n/hi.tsv and /i18n/ur.tsv are the Hindi and Urdu packs"
echo "    Hard-refresh if an old tab still looks cached."
cd "$PKG"
exec python3 -m http.server "$PORT" --bind 127.0.0.1
