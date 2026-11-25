#!/usr/bin/env bash
# Keep wadhu.com language packs in step with locale.rs EN.
#
#   ./scripts/scaffold-site-translations.sh wadhu
#
# For every table language in sites/_shared/i18n/languages.rs (minus English
# and the languages that read another table: pt-BR es-MX yue gom) it rewrites
# sites/wadhu/translations/<code>.tsv so the rows follow EN order.
# A row that already has text keeps it. A new key gets the English source
# (not an @@EN@@ stand-in). Hindi and Urdu must be real translations — the
# site tests reject English and @@EN@@ in those two packs.
#
# Bash + gawk only.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SITE_ARG="${1:?usage: scaffold-site-translations.sh wadhu}"
case "$SITE_ARG" in
  wadhu|wadhu-site) DIR="$ROOT/sites/wadhu" ;;
  *) echo "error: unknown site '$SITE_ARG' (expected wadhu)" >&2; exit 1 ;;
esac
LOC="$DIR/src/locale.rs"
LANGS="$ROOT/sites/_shared/i18n/languages.rs"
[[ -f "$LOC" ]] || { echo "error: no $LOC" >&2; exit 1; }
[[ -f "$LANGS" ]] || { echo "error: no $LANGS" >&2; exit 1; }
mkdir -p "$DIR/translations"

# KEY<TAB>ENGLISH for every EN row (Rust \" → "; other escapes stay two characters).
en_pairs() {
  gawk '
    /^pub const EN: &\[\(&str, &str\)\] = &\[/ { on = 1; next }
    on && /^\];/ { exit }
    on && match($0, /^[[:space:]]*\("([a-z0-9_]+)", "((\\.|[^"\\])*)"\),[[:space:]]*$/, m) {
      v = m[2]; gsub(/\\"/, "\"", v); print m[1] "\t" v
    }
  ' "$LOC"
}

catalogue() {
  gawk 'match($0, /code: "([^"]+)", name_en: "([^"]*)", native_name: "([^"]*)"/, m) { print m[1] "\t" m[2] "\t" m[3] }' "$LANGS"
}

PAIRS="$(mktemp)"
trap 'rm -f "$PAIRS"' EXIT
en_pairs > "$PAIRS"
n_keys="$(wc -l < "$PAIRS" | tr -d ' ')"
if [[ "$n_keys" -lt 5 ]]; then
  echo "error: read only $n_keys EN rows from $LOC — parse failed? (each pair must be one line)" >&2
  exit 1
fi

made=0
while IFS=$'\t' read -r code name native; do
  case "$code" in en|pt-BR|es-MX|yue|gom) continue ;; esac
  file="$DIR/translations/$code.tsv"
  tmp="$(mktemp)"
  {
    printf '# %s — %s (%s). key<TAB>text in locale.rs EN order. Untranslated rows are the English source until rewritten. Keep {placeholders}. \\n = line break.\n' "$name" "$native" "$code"
    gawk -F'\t' -v old="$file" '
      BEGIN {
        if ((getline line < old) > 0) {
          do {
            if (line == "" || substr(line, 1, 1) == "#") continue
            i = index(line, "\t"); if (i == 0) continue
            have[substr(line, 1, i - 1)] = substr(line, i + 1)
          } while ((getline line < old) > 0)
        }
      }
      {
        key = $1; english = substr($0, length($1) + 2)
        if (key in have && have[key] != "") print key "\t" have[key]
        else print key "\t" english
      }
    ' "$PAIRS"
  } > "$tmp"
  [[ -f "$file" ]] || made=$((made + 1))
  mv "$tmp" "$file"
done < <(catalogue)

echo "wadhu: $n_keys keys; packs created $made; dir $DIR/translations"
