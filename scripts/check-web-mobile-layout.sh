#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
fail=0
echo "== unit: mobile_layout_size.js =="
node "$ROOT/scripts/web/mobile_layout_size.js"
WEBJS="$ROOT/vendor/makepad/platform/src/os/web/web.js"
[[ -f "$WEBJS" ]] || { echo "error: missing vendor web.js" >&2; exit 1; }
for needle in css_layout_size_for_canvas "visualViewport.addEventListener" "Desktop site" \
  "fire_resize_debounced" "canvas.width !== sw"; do
  grep -q "$needle" "$WEBJS" || { echo "error: web.js missing $needle" >&2; fail=1; }
done
if grep -q "visualViewport.addEventListener('scroll'" "$WEBJS"; then
  echo "error: web.js must not listen to visualViewport scroll for resize" >&2
  fail=1
fi
grep -q web_mobile_layout_width "$ROOT/scripts/setup-makepad.sh" || fail=1
[[ -f "$ROOT/patches/makepad/web_mobile_layout_width.rs.patch" ]] || fail=1
GUARD="$ROOT/sites/wadhu/shell/mobile-viewport-guard.js"
[[ -f "$GUARD" ]] || { echo "error: missing $GUARD" >&2; fail=1; }
grep -q 'maximum-scale=1' "$GUARD" || fail=1
grep -q 'mobile-viewport-guard' "$ROOT/sites/wadhu/scripts/patch-web-shell.sh" || {
  echo "error: patch-web-shell must inject mobile-viewport-guard" >&2; fail=1; }
[[ "$fail" -eq 0 ]] || { echo "check-web-mobile-layout: FAILED" >&2; exit 1; }
echo "check-web-mobile-layout: OK"
