#!/usr/bin/env bash
# Enforce Wadhu's independence rules (docs/WADHU_VISION_AND_PRINCIPLES.md §3)
# on every workspace member, using `cargo metadata` (no build needed).
#
#   app   → may depend on crates; never on another app, tool or site
#   crate → may depend on no other workspace member (self-contained), and
#           must take Makepad as `makepad-widgets = { workspace = true }`,
#           never as a raw path into vendor/ (that would break consumers in
#           other workspaces — docs/DEV.md § Consuming Wadhu)
#   tool  → standalone shim; no workspace member dependencies
#   site  → may depend on crates; never on an app, tool or another site
#
# A member's kind is `[package.metadata.wadhu] kind = "…"`, else its top
# directory (apps/ → app, crates/ → crate, tools/ → tool, sites/ → site).
#
# Usage: ./scripts/check-independence.sh   (exit 1 on any violation)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
command -v jq >/dev/null 2>&1 || { echo "jq is required" >&2; exit 1; }

META="$(cargo metadata --format-version=1 --no-deps --offline 2>/dev/null \
  || cargo metadata --format-version=1 --no-deps)"

# One line per member: name <TAB> kind <TAB> manifest dir (relative to ROOT)
MEMBERS="$(jq -r --arg root "$ROOT/" '
  .packages[]
  | (.manifest_path | rtrimstr("/Cargo.toml") | ltrimstr($root)) as $dir
  | (.metadata.wadhu.kind
     // ({"apps":"app","crates":"crate","tools":"tool","sites":"site"}[$dir | split("/")[0]])
     // "unknown") as $kind
  | [.name, $kind, $dir] | @tsv' <<<"$META")"

declare -A KIND DIR
while IFS=$'\t' read -r name kind dir; do
  KIND[$name]="$kind"
  DIR[$name]="$dir"
done <<<"$MEMBERS"

fail=0
violation() { echo "✗ $*" >&2; fail=1; }

for name in "${!KIND[@]}"; do
  kind="${KIND[$name]}"
  [[ "$kind" == unknown ]] && violation "$name (${DIR[$name]}): unknown kind — put it under apps/, crates/, tools/ or sites/, or set [package.metadata.wadhu] kind"
done

# Every path dependency of every member: from <TAB> to-name <TAB> to-path <TAB> dep-kind
DEPS="$(jq -r --arg root "$ROOT/" '
  .packages[] as $p
  | $p.dependencies[]
  | select(.path != null)
  | [$p.name, .name, (.path | ltrimstr($root)), (.kind // "normal")] | @tsv' <<<"$META")"

while IFS=$'\t' read -r from to path depkind; do
  [[ -z "${from:-}" ]] && continue
  fk="${KIND[$from]:-unknown}"
  if [[ -n "${KIND[$to]:-}" ]]; then
    tk="${KIND[$to]}"
    case "$fk:$tk" in
      app:crate|site:crate) ;;
      *) violation "$from ($fk) → $to ($tk) [$depkind]: ${fk}s may not depend on ${tk}s" ;;
    esac
  elif [[ "$fk" == crate && "$path" == vendor/* ]]; then
    violation "$from (crate) → $to via path '$path': list $to in root [workspace.dependencies] by registry name (+ a [patch.crates-io] entry to the vendor path) and use '$to = { workspace = true }'"
  fi
done <<<"$DEPS"

count="$(wc -l <<<"$MEMBERS")"
if [[ $fail -ne 0 ]]; then
  echo "Independence check FAILED (docs/WADHU_VISION_AND_PRINCIPLES.md §3)." >&2
  exit 1
fi
echo "Independence check passed: $count member(s), no sibling coupling."
