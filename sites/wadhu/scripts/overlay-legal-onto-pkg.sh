#!/usr/bin/env bash
# Overlay presented legal pages + css onto a Makepad wasm package **without**
# replacing the Makepad index.html (full-bleed canvas). Khaynjo pattern:
# legal routes are static HTML beside the wasm home — never overwrite index.
#
#   ./sites/wadhu/scripts/overlay-legal-onto-pkg.sh <pkg-dir>
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

"$SITE/scripts/present-legal.sh"

# Keep a copy of Makepad index; never let static/index.html replace it.
MAK_INDEX="$(mktemp)"
cp "$PKG/index.html" "$MAK_INDEX"

mkdir -p "$PKG/css"
rsync -a "$SITE/static/css/" "$PKG/css/"
for slug in privacy terms data-deletion support about; do
  mkdir -p "$PKG/$slug"
  rsync -a "$SITE/static/$slug/" "$PKG/$slug/"
  [[ -f "$SITE/static/${slug}.md" ]] && cp "$SITE/static/${slug}.md" "$PKG/${slug}.md"
done

cp "$MAK_INDEX" "$PKG/index.html"
rm -f "$MAK_INDEX"

if [[ ! -f "$PKG/privacy/index.html" ]]; then
  echo "error: privacy HTML missing after overlay" >&2
  exit 1
fi
# Sanity: home must still be Makepad, not the static shell.
if grep -q 'legal.css' "$PKG/index.html" && ! grep -q 'makepad\|\.wasm\|web_gl' "$PKG/index.html"; then
  echo "error: Makepad index.html was replaced by static shell" >&2
  exit 1
fi
echo "Overlayed legal routes onto $PKG (Makepad index preserved)"
