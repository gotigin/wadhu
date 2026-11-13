#!/usr/bin/env bash
# Create a new, independent Wadhu app from templates/app.
#
# Usage:
#   ./scripts/new-app.sh <name> [--id <reverse.dns.id>] [--label "<Label>"]
#
#   <name>   lower-case, digits and dashes (e.g. print-bridge)
#   --id     production app id (default com.wadhu.<name without dashes>,
#            from wadhu.com — not com.gotigin…); development is always <id>.debug
#   --label  human name (default: <name> in Title Case)
#
# Creates apps/<name>/ (package wadhu-<name>), tools/<name>-{dev,android-dev,
# windows-dev}/ (the shims behind the cargo aliases), registers them as
# workspace members, adds the aliases, then runs the independence check and
# `cargo check`.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

usage() { sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'; exit 1; }
NAME="${1:-}"
[[ -z "$NAME" || "$NAME" == -* ]] && usage
shift
ID=""
LABEL=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --id) ID="${2:?--id needs a value}"; shift 2 ;;
    --label) LABEL="${2:?--label needs a value}"; shift 2 ;;
    *) usage ;;
  esac
done

if [[ ! "$NAME" =~ ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ ]]; then
  echo "Invalid name '$NAME': use lower-case letters, digits and single dashes" >&2
  exit 1
fi
SNAKE="${NAME//-/_}"
UPPER="$(tr 'a-z-' 'A-Z_' <<<"$NAME")"
PASCAL="$(sed -E 's/(^|-)([a-z])/\U\2/g' <<<"$NAME")"
[[ -z "$ID" ]] && ID="com.wadhu.${NAME//-/}"
if [[ -z "$LABEL" ]]; then
  LABEL="$(sed -E 's/(^|-)([a-z])/\1\u\2/g; s/-/ /g' <<<"$NAME")"
fi
if [[ ! "$ID" =~ ^[a-z][a-z0-9]*(\.[a-z][a-z0-9_]*)+$ ]]; then
  echo "Invalid id '$ID': use reverse-DNS like com.example.app" >&2
  exit 1
fi

APP="apps/$NAME"
TOOL_DEV="tools/$NAME-dev"
TOOL_ANDROID="tools/$NAME-android-dev"
TOOL_WINDOWS="tools/$NAME-windows-dev"
for dir in "$APP" "$TOOL_DEV" "$TOOL_ANDROID" "$TOOL_WINDOWS"; do
  [[ -e "$dir" ]] && { echo "$dir already exists" >&2; exit 1; }
done
if cargo metadata --format-version=1 --no-deps --offline 2>/dev/null \
    | jq -e --arg n "wadhu-$NAME" '.packages[] | select(.name == $n)' >/dev/null; then
  echo "A package named wadhu-$NAME already exists" >&2
  exit 1
fi

# Copy-then-diverge: the new app owns its copy; nothing links back to the template.
rsync -a --exclude target/ templates/app/ "$APP/"
rsync -a --exclude target/ tools/template-app-dev/ "$TOOL_DEV/"
rsync -a --exclude target/ tools/template-app-android-dev/ "$TOOL_ANDROID/"
rsync -a --exclude target/ tools/template-app-windows-dev/ "$TOOL_WINDOWS/"
mv "$APP/docs/TEMPLATE_APP.md" "$APP/docs/$UPPER.md"

# Fresh MSI upgrade codes per app (never reuse the template's).
PROD_UUID="$(python3 -c 'import uuid; print("{"+str(uuid.uuid4()).upper()+"}")')"
DEV_UUID="$(python3 -c 'import uuid; print("{"+str(uuid.uuid4()).upper()+"}")')"
if [[ -f "$APP/scripts/msi.sh" ]]; then
  PROD_UUID="$PROD_UUID" DEV_UUID="$DEV_UUID" PASCAL="$PASCAL" perl -0pi -e '
    s/export WADHU_MSI_NAME=.*/export WADHU_MSI_NAME=$ENV{PASCAL}/;
    s/export WADHU_MSI_UPGRADE_CODE=.*/export WADHU_MSI_UPGRADE_CODE="$ENV{PROD_UUID}"/;
    s/export WADHU_MSI_DEV_UPGRADE_CODE=.*/export WADHU_MSI_DEV_UPGRADE_CODE="$ENV{DEV_UUID}"/;
  ' "$APP/scripts/msi.sh"
fi

# Rewrite template names (longest first so a shorter one never splits a longer one).
replace_all() {
  local file
  while IFS= read -r -d '' file; do
    OLD_NEW=(
      "wadhu-template-app" "wadhu-$NAME"
      "wadhu_template_app" "wadhu_$SNAKE"
      "template-app-android-dev" "$NAME-android-dev"
      "template-app-windows-dev" "$NAME-windows-dev"
      "template-app-linux-dev" "${NAME}-linux-dev"
      "template-app-linux" "${NAME}-linux"
      "template-app-windows-dev" "${NAME}-windows-dev"
      "template-app-windows" "${NAME}-windows"
      "template-app-android" "${NAME}-android"
      "template-app-dev" "$NAME-dev"
      "WadhuTemplate" "$PASCAL"
      "templates/app" "apps/$NAME"
      "com.wadhu.template" "$ID"
      "Wadhu Template Dev" "$LABEL Dev"
      "Wadhu Template" "$LABEL"
      "TEMPLATE_APP" "$UPPER"
      "Template app" "$LABEL"
    )
    local tmp="$file.tmp.$$"
    cp "$file" "$tmp"
    local i
    for ((i = 0; i < ${#OLD_NEW[@]}; i += 2)); do
      OLD="${OLD_NEW[i]}" NEW="${OLD_NEW[i+1]}" perl -0pi -e 's/\Q$ENV{OLD}\E/$ENV{NEW}/g' "$tmp"
    done
    mv "$tmp" "$file"
  done < <(find "$1" -type f \( \
    -name '*.rs' -o -name '*.toml' -o -name '*.md' -o -name '*.sh' -o -name '*.env' \
    -o -name '*.ps1' -o -name '*.bat' -o -name '*.rtf' -o -name '*.js' \
  \) -print0)
}
replace_all "$APP"
replace_all "$TOOL_DEV"
replace_all "$TOOL_ANDROID"
replace_all "$TOOL_WINDOWS"

# The copy is a real app now: drop template-only wording, take its own
# description, and let its place (apps/) give its kind.
LABEL="$LABEL" perl -0pi -e 's/^description = .*$/description = "$ENV{LABEL} — TODO: one line on what this app does"/m' "$APP/Cargo.toml"
LABEL="$LABEL" perl -0pi -e 's/Wadhu template app/$ENV{LABEL}/g; s{//! Copy this app with `scripts/new-app.sh <name>`; do not depend on it.\n}{}' \
  "$APP/src/main.rs" "$APP/src/lib.rs"
perl -0pi -e 's/\n# Kind for scripts\/check-independence.sh[^\n]*\n\[package.metadata.wadhu\]\nkind = "app"\n//' "$APP/Cargo.toml"
for tool in "$TOOL_DEV" "$TOOL_ANDROID" "$TOOL_WINDOWS"; do
  perl -0pi -e 's/\n\[package.metadata.wadhu\]\nkind = "tool"\n//' "$tool/Cargo.toml"
done

# Register members (after the last apps/ entry, else at the top of the list).
add_member() {
  local member="$1"
  MEMBER="$member" perl -0pi -e '
    my $m = $ENV{MEMBER};
    s/(members = \[\n)/$1    "$m",\n/ unless /"\Q$m\E"/;
  ' Cargo.toml
}
add_member "$TOOL_WINDOWS"
add_member "$TOOL_ANDROID"
add_member "$TOOL_DEV"
add_member "$APP"

# Cargo aliases for live / package-dev shims.
add_alias() {
  local alias_name="$1" pkg="$2" comment="$3"
  local line="$alias_name = [\"run\", \"-p\", \"$pkg\", \"--\"]"
  if ! grep -qF "$line" .cargo/config.toml; then
    ALIAS="# $comment
# Usage: cargo $alias_name
$line
" perl -0pi -e 's/(\n\# Cross-link the Windows GNU target)/\n$ENV{ALIAS}$1/' .cargo/config.toml
  fi
}
add_alias "$NAME-dev" "$NAME-dev" "$LABEL desktop live-dev (apps/$NAME, $ID.debug)."
add_alias "$NAME-android-dev" "$NAME-android-dev" "$LABEL Android live-dev (apps/$NAME, $ID.debug)."
add_alias "$NAME-windows-dev" "$NAME-windows-dev" "$LABEL Windows package (apps/$NAME, $ID.debug)."

cat > "$APP/README.md" <<EOF
# $LABEL

Package \`wadhu-$NAME\` — TODO: one line on what this app does and who needs it.
Brief: [\`docs/$UPPER.md\`](docs/$UPPER.md). Wadhu rules: [\`docs/\`](../../docs/README.md).

\`\`\`bash
cargo $NAME-dev                   # live dev: --hot reload, --remote (F12)
cargo $NAME-android-dev           # Android: build, install, launch
cargo $NAME-windows-dev           # Windows cross package (dev app id)
cargo run -p wadhu-$NAME          # plain run
cargo test -p wadhu-$NAME
\`\`\`

Ids: production \`$ID\` · development \`$ID.debug\` (\`env/\`, baked by \`build.rs\`).

| Path | What |
|---|---|
| \`src/main.rs\` | Makepad 2 shell (\`script_mod!\` UI + \`AppMain\`) |
| \`src/lib.rs\` | Non-UI half — what tests and the shell both call |
| \`src/identity.rs\` | App id / label / env baked by \`build.rs\` |
| \`env/\` | Development and production ids and labels |
| \`scripts/\` | \`dev.sh\`, packaging (\`package-*.sh\`, \`msi.sh\`), installers, \`generate-icons.sh\` |
| \`docs/\` | This app's own docs |
EOF

chmod +x "$APP"/scripts/*.sh
echo "Created $APP (wadhu-$NAME, $ID / $ID.debug) and tools/$NAME-{dev,android-dev,windows-dev}."
./scripts/check-independence.sh
cargo check -p "wadhu-$NAME" -p "$NAME-dev" -p "$NAME-android-dev" -p "$NAME-windows-dev"
cat <<EOF

Next:
  cargo $NAME-dev  — run it (--hot reload, --remote F12)
  $APP/docs/$UPPER.md  — write the app brief
  Cargo.toml  — add "$APP" to default-members once it ships
  $APP/scripts/generate-icons.sh  — after adding $APP/assets/app-icon.png
  $APP/scripts/msi.sh  — MSI upgrade codes were generated; never change them after first release
EOF
