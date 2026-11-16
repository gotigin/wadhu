#!/usr/bin/env bash
# Overlay same-site pages (legal HTML + css/js) onto the Makepad wasm package.
# Khaynjo pattern: one tree — wasm home at `/`, `/privacy` etc. beside it.
# static/ must not contain a competing index.html (home is always the wasm shell).
#
#   ./sites/wadhu/scripts/overlay-legal-onto-pkg.sh <makepad-wasm-pkg-dir>
set -euo pipefail
SITE="$(cd "$(dirname "$0")/.." && pwd)"
PKG="${1:-}"
if [[ -z "$PKG" || ! -d "$PKG" ]]; then
  echo "usage: $0 <makepad-wasm-pkg-dir>" >&2
  exit 1
fi
if [[ ! -f "$PKG/index.html" ]]; then
  echo "error: missing $PKG/index.html (Makepad home)" >&2
  exit 1
fi
if [[ -f "$SITE/static/index.html" ]]; then
  echo "error: $SITE/static/index.html must not exist — home is the Makepad package only" >&2
  exit 1
fi

"$SITE/scripts/present-legal.sh"

rsync -a "$SITE/static/" "$PKG/"

if [[ ! -f "$PKG/privacy/index.html" ]]; then
  echo "error: privacy HTML missing after overlay" >&2
  exit 1
fi
if ! grep -qE 'wadhu-site\.wasm|web_gl|full_canvas|makepad' "$PKG/index.html"; then
  echo "error: Makepad index.html was overwritten" >&2
  exit 1
fi
echo "Overlayed same-site pages onto $PKG (Makepad index preserved)"
