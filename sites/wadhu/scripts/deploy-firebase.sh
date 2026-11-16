#!/usr/bin/env bash
# Stage wadhu.com public/ and deploy to Firebase Hosting.
# One-time: npm i -g firebase-tools && firebase login
# Firebase Console → create project, set .firebaserc, add custom domain wadhu.com
#
#   ./sites/wadhu/scripts/deploy-firebase.sh
#   ./sites/wadhu/scripts/deploy-firebase.sh --dry-run
#   ./sites/wadhu/scripts/deploy-firebase.sh --with-wasm   # if wadhu-site wasm built
#
# Always regenerates legal pages from docs/legal/*.md (do not hand-edit HTML).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/wadhu"
OUT="$SITE/public"
LEGAL="$ROOT/docs/legal"
DRY=0
WITH_WASM=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
    --with-wasm) WITH_WASM=1 ;;
    -h|--help) sed -n '2,12p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v firebase >/dev/null 2>&1; then
  echo "error: firebase CLI not found (npm i -g firebase-tools)" >&2
  exit 1
fi

echo "1/3 Present docs/legal → formatted HTML in static/"
"$SITE/scripts/present-legal.sh"

echo "2/3 Stage $OUT"
rm -rf "$OUT"
mkdir -p "$OUT"

if [[ "$WITH_WASM" -eq 1 ]]; then
  WASM_PKG="$ROOT/target/makepad-wasm-app/release/wadhu-site"
  if [[ ! -f "$WASM_PKG/index.html" ]]; then
    echo "Building wadhu-site wasm…"
    (
      cd "$ROOT"
      cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
        wasm --no-threads --strip --brotli build -p wadhu-site --release
    )
  fi
  if [[ -f "$WASM_PKG/index.html" ]]; then
    rsync -a "$WASM_PKG/" "$OUT/"
    # Legal beside Makepad home — never replace index.html with static shell.
    "$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
    echo "Staged Makepad wasm + legal routes."
  else
    echo "warn: wasm package missing — deploying static shell only" >&2
    rsync -a "$SITE/static/" "$OUT/"
  fi
else
  # Static-only deploy (no canvas): wide site.css home + legal pages.
  rsync -a "$SITE/static/" "$OUT/"
fi

for need in privacy terms data-deletion support about; do
  if [[ ! -f "$OUT/$need/index.html" ]]; then
    echo "error: missing /$need after present-legal" >&2
    exit 1
  fi
done
if ! grep -q "Wadhu privacy policy" "$OUT/privacy/index.html"; then
  echo "error: /privacy not generated from docs/legal/PRIVACY.md" >&2
  exit 1
fi
if ! grep -q "data deletion" "$OUT/data-deletion/index.html"; then
  echo "error: /data-deletion not generated from docs/legal/DATA_DELETION.md" >&2
  exit 1
fi
# Ensure we did not stage the stub "Moved" pointers as privacy.md
if grep -q '^# Moved' "$OUT/privacy.md" 2>/dev/null; then
  echo "error: privacy.md is a stub — present-legal must copy docs/legal/PRIVACY.md" >&2
  exit 1
fi
test -f "$LEGAL/PRIVACY.md"

echo "3/3 Deploy Firebase Hosting → wadhu.com (project from .firebaserc)"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --dry-run
else
  firebase deploy --only hosting
fi
