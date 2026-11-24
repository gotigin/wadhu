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

# cargo-makepad writes bare <html>; crawlers want a language.
if grep -q '<html>' "$PKG/index.html" && ! grep -q '<html lang=' "$PKG/index.html"; then
  sed -i 's/<html>/<html lang="en">/' "$PKG/index.html"
fi

# Reviewers, search engines and chat apps read the wasm shell and never run
# the wasm, so the name, the description and the link-preview picture are
# written into it (Makepad titles it "wadhu-site").
PREVIEW="$SITE/static/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing $PREVIEW — run sites/wadhu/scripts/make-link-preview.sh" >&2
  exit 1
fi
# The picture's address changes when the picture does: WhatsApp, Telegram and
# Facebook keep a preview for days and only fetch again for a new address.
PREVIEW_URL="https://wadhu.com/media/link-preview.jpg?v=$(sha256sum "$PREVIEW" | cut -c1-10)"
if ! grep -q 'og:site_name' "$PKG/index.html"; then
  HEAD_INJECT='        <title>Wadhu — Tools · Apps · More</title>
        <meta name="description" content="Wadhu means more in Memon. Reusable tools, utils, and apps by Gotigin. SMS Gateway, SMS Relay, and Wadhu Connect are products under the brand." />
        <link rel="canonical" href="https://wadhu.com/" />
        <meta name="theme-color" content="#0d0b3a" />
        <meta name="robots" content="max-image-preview:large" />
        <link rel="icon" href="./favicon.png" type="image/png" />
        <link rel="apple-touch-icon" href="./media/apple-touch-icon.png" />
        <meta property="og:site_name" content="Wadhu" />
        <meta property="og:title" content="Wadhu — Tools · Apps · More" />
        <meta property="og:description" content="Wadhu means more in Memon. Reusable tools, utils, and apps by Gotigin." />
        <meta property="og:url" content="https://wadhu.com/" />
        <meta property="og:type" content="website" />
        <meta property="og:locale" content="en_IN" />
        <meta property="og:image" content="'"$PREVIEW_URL"'" />
        <meta property="og:image:type" content="image/jpeg" />
        <meta property="og:image:width" content="1200" />
        <meta property="og:image:height" content="630" />
        <meta property="og:image:alt" content="The wadhu.com home page: Wadhu means more. Tools, apps and more." />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="Wadhu — Tools · Apps · More" />
        <meta name="twitter:description" content="Wadhu means more in Memon. Reusable tools, utils, and apps by Gotigin." />
        <meta name="twitter:image" content="'"$PREVIEW_URL"'" />
        <meta name="twitter:image:alt" content="The wadhu.com home page: Wadhu means more. Tools, apps and more." />
        <script type="application/ld+json">{"@context":"https://schema.org","@type":"Organization","name":"Wadhu","alternateName":"Wadhu — Tools · Apps · More","url":"https://wadhu.com/","logo":"https://wadhu.com/media/wadhu-brand-logo.webp","image":"https://wadhu.com/media/link-preview.jpg","description":"Memon for more. Reusable tools, utils, and apps — Tools · Apps · More.","brand":{"@type":"Brand","name":"Wadhu","slogan":"Tools · Apps · More"},"parentOrganization":{"@type":"Organization","name":"Gotigin","legalName":"GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED","url":"https://gotigin.com/","logo":"https://gotigin.com/media/gotigin-brand-logo.png","contactPoint":{"@type":"ContactPoint","email":"support@gotigin.com","contactType":"customer support"}},"sameAs":["https://gotigin.com/","https://connect.wadhu.com/"],"contactPoint":{"@type":"ContactPoint","email":"support@gotigin.com","contactType":"customer support"}}</script>'
  awk -v head="$HEAD_INJECT" '
    /<title>/ && !titled {
      print head
      titled = 1
      next
    }
    { print }
    END { if (!titled) exit 2 }
  ' "$PKG/index.html" > "$PKG/index.html.tmp" && mv "$PKG/index.html.tmp" "$PKG/index.html"
fi

# Upgrade an older WebSite-only JSON-LD (pre-sprint) to Brand + parentOrganization.
if grep -q '"@type":"WebSite"' "$PKG/index.html"; then
  python3 - "$PKG/index.html" <<'PY'
import re, sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
new = (
    '{"@context":"https://schema.org","@type":"Organization","name":"Wadhu",'
    '"alternateName":"Wadhu — Tools · Apps · More","url":"https://wadhu.com/",'
    '"logo":"https://wadhu.com/media/wadhu-brand-logo.webp",'
    '"image":"https://wadhu.com/media/link-preview.jpg",'
    '"description":"Memon for more. Reusable tools, utils, and apps — Tools · Apps · More.",'
    '"brand":{"@type":"Brand","name":"Wadhu","slogan":"Tools · Apps · More"},'
    '"parentOrganization":{"@type":"Organization","name":"Gotigin",'
    '"legalName":"GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED",'
    '"url":"https://gotigin.com/","logo":"https://gotigin.com/media/gotigin-brand-logo.png",'
    '"contactPoint":{"@type":"ContactPoint","email":"support@gotigin.com","contactType":"customer support"}},'
    '"sameAs":["https://gotigin.com/","https://connect.wadhu.com/"],'
    '"contactPoint":{"@type":"ContactPoint","email":"support@gotigin.com","contactType":"customer support"}}'
)
text2, n = re.subn(
    r'<script type="application/ld\+json">\{"@context":"https://schema\.org","@type":"WebSite".*?</script>',
    f'<script type="application/ld+json">{new}</script>',
    text,
    count=1,
    flags=re.S,
)
if n:
    open(path, "w", encoding="utf-8").write(text2)
PY
fi

# The start-up screen (logo, progress ring) replaces cargo-makepad's grey bar.
"$SITE/scripts/patch-web-shell.sh" "$PKG/index.html"

# Crawlers that never run wasm still need a plain-HTML summary (deploy injects
# the same block with a cache-busted Analytics script beside it).
if ! grep -q '<noscript>' "$PKG/index.html"; then
  awk '
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
  ' "$PKG/index.html" > "$PKG/index.html.tmp" && mv "$PKG/index.html.tmp" "$PKG/index.html"
fi

echo "Overlayed same-site pages onto $PKG (Makepad index preserved)"
