#!/usr/bin/env bash
# Deploy the one wadhu.com Hosting tree (Khaynjo pattern):
#   Makepad SPA at /  +  /privacy /terms /about …  +  Analytics
#
# Firebase project: wadhu-ac8dc (.firebaserc). Custom domain: wadhu.com.
#
#   ./sites/wadhu/scripts/deploy-firebase.sh
#   ./sites/wadhu/scripts/deploy-firebase.sh --dry-run
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
OUT="$SITE/public"
LEGAL="$ROOT/docs/legal"
WASM_PKG="$ROOT/target/makepad-wasm-app/release/wadhu-site"
DRY=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
    -h|--help) sed -n '2,10p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v firebase >/dev/null 2>&1; then
  echo "error: firebase CLI not found (npm i -g firebase-tools)" >&2
  exit 1
fi

echo "1/4 Bundle Firebase Analytics + Auth → static/js/"
(
  cd "$SITE"
  if [[ ! -d node_modules/firebase ]]; then
    npm install
  fi
  npm run build
)
if [[ ! -f "$SITE/static/js/firebase-init.js" || ! -f "$SITE/static/js/account.js" ]]; then
  echo "error: missing firebase-init.js or account.js after npm run build" >&2
  exit 1
fi
if [[ ! -f "$SITE/static/account/index.html" ]]; then
  echo "error: missing static/account/index.html" >&2
  exit 1
fi

echo "2/4 Build Makepad wasm (wadhu-site, release)"
(
  cd "$ROOT"
  cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
    wasm --no-threads --strip --brotli build -p wadhu-site --release
)
if [[ ! -f "$WASM_PKG/index.html" || ! -f "$WASM_PKG/wadhu-site.wasm" ]]; then
  echo "error: wasm package incomplete at $WASM_PKG" >&2
  exit 1
fi

echo "3/4 Stage public/ = one tree (wasm home + legal + Auth)"
"$SITE/scripts/present-legal.sh"
rm -rf "$OUT"
mkdir -p "$OUT"
# cargo-makepad writes a .br twin beside every file. Firebase Hosting
# compresses on its own and never serves those, so they stay out of the tree.
rsync -a --exclude='*.br' "$WASM_PKG/" "$OUT/"
"$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
# Language packs (site i18n): the wasm and site-i18n.js fetch /i18n/<code>.tsv.
mkdir -p "$OUT/i18n" "$OUT/js"
cp "$SITE"/translations/*.tsv "$OUT/i18n/"
cp "$SITE/static/js/lang-nav.js" "$OUT/js/lang-nav.js"
cp "$SITE/static/js/site-i18n.js" "$OUT/js/site-i18n.js"
# Fonts: only the characters this site draws (the originals stay whole).
"$SITE/scripts/subset-fonts.sh" "$OUT"
mkdir -p "$OUT/js"
cp "$SITE/static/js/firebase-init.js" "$OUT/js/firebase-init.js"
cp "$SITE/static/js/account.js" "$OUT/js/account.js"
mkdir -p "$OUT/css"

BUILD_ID="$(date -u +%Y%m%d%H%M%S)-$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo local)"
INDEX="$OUT/index.html"
if ! grep -qE 'wadhu-site\.wasm|web_gl|full_canvas' "$INDEX"; then
  echo "error: staged index.html is not the Makepad home" >&2
  exit 1
fi
# A new address per build for the wasm and its loader, so a browser never
# pairs old wasm with new glue (the wasm itself is served immutable).
awk -v bid="$BUILD_ID" '
  {
    gsub(/\.\/makepad_platform\/web_gl\.js/, "./makepad_platform/web_gl.js?v=" bid)
    gsub(/\.\/wadhu-site\.wasm/, "./wadhu-site.wasm?v=" bid)
  }
  /<meta name="theme-color"/ && !stamped {
    print "        <meta name=\"wadhu-build\" content=\"" bid "\" />"
    stamped = 1
  }
  /<\/body>/ && !done {
    print "        <noscript>"
    print "          <h1>Wadhu — Tools · Apps · More</h1>"
    print "          <p>Wadhu is Memon for <em>more</em>. Reusable tools, utils, and apps under Gotigin — a trademark that GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED is authorized to use, operate under, and represent.</p>"
    print "          <p>Products: <a href=\"/about\">About</a> · SMS Gateway · SMS Relay · <a href=\"https://connect.wadhu.com/\">Wadhu Connect</a>.</p>"
    print "          <p>Contact: <a href=\"mailto:support@gotigin.com\">support@gotigin.com</a> · Parent: <a href=\"https://gotigin.com/\">gotigin.com</a>.</p>"
    print "          <p><a href=\"/privacy\">Privacy</a> · <a href=\"/terms\">Terms</a> · <a href=\"/cancellation\">Cancellation</a> · <a href=\"/data-deletion\">Data deletion</a> · <a href=\"/support\">Support</a> · <a href=\"/account\">Account</a>.</p>"
    print "        </noscript>"
    done = 1
  }
  { print }
  END { if (!done) exit 2 }
' "$INDEX" > "$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
printf '%s\n' "$BUILD_ID" > "$OUT/build-id.txt"
if ! grep -q 'id="boot"' "$INDEX" || ! grep -q 'og:image' "$INDEX"; then
  echo "error: staged index.html lacks the start-up screen or the link-preview tags" >&2
  exit 1
fi

for need in privacy terms cancellation data-deletion support about account; do
  if [[ ! -f "$OUT/$need/index.html" ]]; then
    echo "error: missing /$need after overlay" >&2
    exit 1
  fi
done
for need in robots.txt sitemap.xml; do
  if [[ ! -f "$OUT/$need" ]]; then
    echo "error: missing /$need after overlay (static/)" >&2
    exit 1
  fi
done
# Account is hand-written static (not present-legal); stamp the same sha10 as legal/home.
PREVIEW="$SITE/static/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing $PREVIEW — run sites/wadhu/scripts/make-link-preview.sh" >&2
  exit 1
fi
PREVIEW_V="$(sha256sum "$PREVIEW" | cut -c1-10)"
sed -i -E "s#(https://wadhu\\.com/media/link-preview\\.jpg)(\\?v=[0-9a-f]+)?\"#\\1?v=${PREVIEW_V}\"#g" "$OUT/account/index.html"
if ! grep -q "link-preview.jpg?v=${PREVIEW_V}" "$OUT/account/index.html"; then
  echo "error: account page missing cache-busted link-preview tags" >&2
  exit 1
fi
if [[ ! -f "$OUT/js/firebase-init.js" || ! -f "$OUT/js/account.js" ]]; then
  echo "error: missing $OUT/js/firebase-init.js or account.js" >&2
  exit 1
fi
if ! grep -q 'lang-nav.js' "$OUT/index.html"; then
  echo "error: wasm shell missing lang-nav.js" >&2
  exit 1
fi
if [[ ! -f "$OUT/i18n/hi.tsv" || ! -f "$OUT/i18n/ur.tsv" ]]; then
  echo "error: staged /i18n packs missing hi.tsv or ur.tsv" >&2
  exit 1
fi
if ! grep -q "Wadhu privacy policy" "$OUT/privacy/index.html"; then
  echo "error: /privacy not generated from docs/legal/PRIVACY.md" >&2
  exit 1
fi
if ! grep -q "data deletion" "$OUT/data-deletion/index.html"; then
  echo "error: /data-deletion not generated from docs/legal/DATA_DELETION.md" >&2
  exit 1
fi
if grep -q '^# Moved' "$OUT/privacy.md" 2>/dev/null; then
  echo "error: privacy.md is a stub — present-legal must copy docs/legal/PRIVACY.md" >&2
  exit 1
fi
test -f "$LEGAL/PRIVACY.md"

PROJECT="$(python3 -c "import json;print(json.load(open('$SITE/.firebaserc'))['projects']['default'])")"
echo "4/4 Deploy Firebase Hosting → ${PROJECT} (one site: / + /privacy …)"
cd "$SITE"
# Always pass --project so a leftover `firebase use` elsewhere cannot hijack this deploy.
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --project "$PROJECT" --dry-run
else
  firebase deploy --only hosting --project "$PROJECT"
fi
