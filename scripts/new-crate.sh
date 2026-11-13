#!/usr/bin/env bash
# Create a new, self-contained Wadhu library crate.
#
# Usage:
#   ./scripts/new-crate.sh <name> [--widgets]
#
#   <name>     lower-case, digits and dashes (e.g. escpos, lan-discovery)
#   --widgets  a Makepad 2 widget crate (from templates/widgets, with a
#              runnable `--example demo`); default is plain Rust (templates/crate)
#
# Creates crates/<name>/ (package wadhu-<name>, lib wadhu_<name>), registers
# it as a workspace member, then runs the independence check and the tests.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

usage() { sed -n '2,13p' "$0" | sed 's/^# \{0,1\}//'; exit 1; }
NAME="${1:-}"
[[ -z "$NAME" || "$NAME" == -* ]] && usage
shift
WIDGETS=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --widgets) WIDGETS=1; shift ;;
    *) usage ;;
  esac
done

if [[ ! "$NAME" =~ ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ ]]; then
  echo "Invalid name '$NAME': use lower-case letters, digits and single dashes" >&2
  exit 1
fi
SNAKE="${NAME//-/_}"
CAMEL="$(sed -E 's/(^|-)([a-z])/\u\2/g' <<<"$NAME")"
DEST="crates/$NAME"
[[ -e "$DEST" ]] && { echo "$DEST already exists" >&2; exit 1; }
if cargo metadata --format-version=1 --no-deps --offline 2>/dev/null \
    | jq -e --arg n "wadhu-$NAME" '.packages[] | select(.name == $n)' >/dev/null; then
  echo "A package named wadhu-$NAME already exists" >&2
  exit 1
fi

if [[ $WIDGETS -eq 1 ]]; then
  SRC=templates/widgets; OLD=wadhu-template-widgets
else
  SRC=templates/crate; OLD=wadhu-template-crate
fi
OLD_SNAKE="${OLD//-/_}"
OLD_DIR="crates/${OLD#wadhu-}"

rsync -a --exclude target/ "$SRC/" "$DEST/"

while IFS= read -r -d '' file; do
  OLD_NEW=(
    "$OLD" "wadhu-$NAME"
    "$OLD_SNAKE" "wadhu_$SNAKE"
    "$OLD_DIR" "crates/$NAME"
    "TemplatePill" "${CAMEL}Pill"
    "template_pill" "${SNAKE}_pill"
  )
  for ((i = 0; i < ${#OLD_NEW[@]}; i += 2)); do
    OLD="${OLD_NEW[i]}" NEW="${OLD_NEW[i+1]}" perl -0pi -e 's/\Q$ENV{OLD}\E/$ENV{NEW}/g' "$file"
  done
done < <(find "$DEST" -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.md' \) -print0)

# A real crate now: own description, kind from its place (crates/).
perl -0pi -e 's/^description = .*$/description = "TODO: one line on what this crate does"/m' "$DEST/Cargo.toml"
perl -0pi -e 's/\n# Kind for scripts\/check-independence.sh[^\n]*\n\[package.metadata.wadhu\]\nkind = "crate"\n//' "$DEST/Cargo.toml"
perl -0pi -e 's/^(\/\/! )Wadhu template (?:widget )?crate — /$1wadhu-'"$NAME"' — /m' "$DEST/src/lib.rs"

if [[ $WIDGETS -eq 1 ]]; then
  USE_NOTE="Register with \`wadhu_$SNAKE::script_mod(vm)\` after \`makepad_widgets::script_mod\`; demo: \`cargo run -p wadhu-$NAME --example demo\`."
else
  USE_NOTE="Plain Rust — no Makepad dependency."
fi
cat > "$DEST/README.md" <<EOF
# wadhu-$NAME

TODO: one line on what this crate does, and for whom.

\`\`\`toml
[dependencies]
wadhu-$NAME = { path = "../../crates/$NAME" }   # from a Wadhu app
\`\`\`

$USE_NOTE From another workspace see docs/DEV.md § Consuming Wadhu from
another workspace. Rules: docs/CRATES.md. Changes: [CHANGELOG.md](CHANGELOG.md).
EOF

MEMBER="$DEST" perl -0pi -e '
  my $m = $ENV{MEMBER};
  s/(members = \[\n)/$1    "$m",\n/ unless /"\Q$m\E"/;
' Cargo.toml

echo "Created $DEST (wadhu-$NAME)."
./scripts/check-independence.sh
cargo test -p "wadhu-$NAME"
if [[ $WIDGETS -eq 1 ]]; then
  cargo build -p "wadhu-$NAME" --example demo
  echo "Demo: cargo run -p wadhu-$NAME --example demo"
fi
cat <<EOF

Next: replace the example API in $DEST/src/, write $DEST/README.md, and keep
$DEST/CHANGELOG.md current. Rules: docs/CRATES.md.
EOF
