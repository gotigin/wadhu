#!/usr/bin/env bash
# Present docs/legal/*.md as formatted HTML for wadhu.com.
# Same files Connect embeds via include_str! — do not hand-edit the HTML.
#
#   ./sites/wadhu/scripts/present-legal.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
LEGAL="$ROOT/docs/legal"
TMPL="$SITE/legal-template.html"

if [[ ! -f "$TMPL" ]]; then
  echo "error: missing $TMPL" >&2
  exit 1
fi
if ! command -v pandoc >/dev/null 2>&1; then
  echo "error: pandoc required to present legal Markdown" >&2
  exit 1
fi

# The link-preview picture's address carries a short hash of the file, so chat
# apps that cache previews fetch a changed picture.
PREVIEW="$SITE/static/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing $PREVIEW — run sites/wadhu/scripts/make-link-preview.sh" >&2
  exit 1
fi
PREVIEW_V="$(sha256sum "$PREVIEW" | cut -c1-10)"

present() {
  local slug="$1" title="$2" description="$3" canonical="$4" md_name="$5"
  local md="$LEGAL/$md_name"
  if [[ ! -f "$md" ]]; then
    echo "error: missing $md" >&2
    exit 1
  fi
  local out_dir="$SITE/static/$slug"
  mkdir -p "$out_dir"
  pandoc "$md" \
    -f markdown \
    -t html5 \
    --template="$TMPL" \
    --metadata title="$title" \
    -V lang=en \
    -V description="$description" \
    -V canonical="$canonical" \
    -V preview_v="$PREVIEW_V" \
    -o "$out_dir/index.html"
  cp "$md" "$SITE/static/${slug}.md"
  if [[ -d "$SITE/public" ]]; then
    mkdir -p "$SITE/public/$slug"
    cp "$out_dir/index.html" "$SITE/public/$slug/index.html"
    cp "$md" "$SITE/public/${slug}.md"
  fi
  echo "  /$slug ← $md"
}

echo "Presenting docs/legal → sites/wadhu/static/"
present privacy "Privacy — Wadhu" \
  "Privacy policy for the Wadhu brand (Tools · Apps · More), including wadhu.com, SMS Gateway, and Wadhu Connect." \
  "https://wadhu.com/privacy" "PRIVACY.md"
present terms "Terms — Wadhu" \
  "Terms of use for the Wadhu brand and its products, including SMS Gateway and Wadhu Connect." \
  "https://wadhu.com/terms" "TERMS.md"
present cancellation "Cancellation & refund — Wadhu" \
  "Cancellation and refund policy for the Wadhu brand — additional to Gotigin." \
  "https://wadhu.com/cancellation" "CANCELLATION.md"
present data-deletion "Data deletion — Wadhu" \
  "How to delete data for Wadhu products, including SMS Gateway and Wadhu Connect." \
  "https://wadhu.com/data-deletion" "DATA_DELETION.md"
present support "Support — Wadhu" \
  "Support for the Wadhu brand and its products." \
  "https://wadhu.com/support" "SUPPORT.md"
# description uses -V (raw into the template) so pre-escape &; title uses
# --metadata and pandoc escapes it — pass a literal & there.
present about "About — Wadhu" \
  "Wadhu means more in Memon. Tools, utils, and apps by Gotigin — GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED." \
  "https://wadhu.com/about" "ABOUT.md"

echo "Done. Deploy with ./sites/wadhu/scripts/deploy-firebase.sh"
