#!/usr/bin/env bash
# Wadhu Template production APK for Android phones and tablets — an Android device
# : dist/template-app-android/wadhu-template-app.apk with the
# production app id. No device needed. For live install on a connected
# device use android-dev.sh (development app id).
#   templates/app/scripts/package-android.sh
# Signed with the release key when WADHU_ANDROID_KEYSTORE (+ _PASS) is set
# (see scripts/android-env.sh), which also builds the Play bundle
# dist/template-app-android/wadhu-template-app.aab.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
export WADHU_APP_DIR="$ROOT/templates/app"
export WADHU_ENV_DIR="$WADHU_APP_DIR/env"
# shellcheck source=../../../scripts/load-wadhu-env.sh
source "$ROOT/scripts/load-wadhu-env.sh"
# shellcheck source=../../../scripts/android-env.sh
source "$ROOT/scripts/android-env.sh"
export WADHU_ROOT="$ROOT"
load_wadhu_env production
resolve_makepad_root
export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Android/Sdk}}"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
if [[ ! -d "$ANDROID_HOME" ]]; then
  echo "Android SDK not found. Set ANDROID_HOME (tried: $ANDROID_HOME)" >&2
  exit 1
fi
ensure_android_toolchain "$ANDROID_HOME"
"$ROOT/scripts/apply-android-activity-bridge.sh"
fix_makepad_android_deprecations
android_pkg_opts "$ANDROID_HOME"
android_release_opts

OUT="$ROOT/dist/template-app-android"
# Stage into a fresh folder, then swap over `$OUT` only on success — same
# idea as MSI overwriting dist/Wadhu Template-*.msi (no stale leftovers; a failed
# build leaves the previous good package).
STAGE="$ROOT/dist/.template-app-android-staging.$$"
rm -rf "$STAGE"
mkdir -p "$STAGE"
LOG="$STAGE/build.log"
cleanup_stage() { rm -rf "$STAGE"; }
trap cleanup_stage EXIT
echo "Makepad: $MAKEPAD_ROOT"
echo "ANDROID_HOME: $ANDROID_HOME"
echo "Building ${WADHU_APP_ID} (${WADHU_APP_LABEL}) release APK…"
set +e
cargo run --manifest-path "$MAKEPAD_ROOT/tools/cargo_makepad/Cargo.toml" --release -- \
  android "${ANDROID_PKG_OPTS[@]}" build -p wadhu-template-app --release 2>&1 | tee "$LOG"
STATUS=${PIPESTATUS[0]}
set -e

# shellcheck source=../../../scripts/cargo-target-dir.sh
source "$ROOT/scripts/cargo-target-dir.sh"
TARGET_DIR="$(cargo_target_dir)"
# This build's APK: named after the production label (not the dev one,
# not another ABI's leftover).
APK_DIR="$TARGET_DIR/android/makepad-android-apk/wadhu_template_app/apk"
APK_LABEL="$(echo "$WADHU_APP_LABEL" | tr -d ' ' | tr '[:upper:]' '[:lower:]').apk"
APK="$APK_DIR/$APK_LABEL"
if [[ ! -f "$APK" ]]; then
  APK="$(find "$APK_DIR" -maxdepth 1 -name '*.apk' -printf '%T@ %p\n' 2>/dev/null | sort -nr | head -1 | cut -d' ' -f2- || true)"
fi
if [[ $STATUS -ne 0 || -z "${APK:-}" || ! -f "$APK" ]]; then
  echo "Android build did not complete (exit $STATUS). See $LOG" >&2
  # Keep previous $OUT; drop staging via trap.
  exit 1
fi
cp -f "$APK" "$STAGE/wadhu-template-app.apk"
android_release_sign "$STAGE/wadhu-template-app.apk" "$ANDROID_HOME"
SIGNED="Makepad's debug key — for trying out only; a shop's devices need the release key"
if android_release_keystore_set; then
  SIGNED="the Wadhu Template release key"
  android_aab_sign_opts
  cargo run --manifest-path "$MAKEPAD_ROOT/tools/cargo_makepad/Cargo.toml" --release -- \
    android "${ANDROID_PKG_OPTS[@]}" "${ANDROID_AAB_SIGN_OPTS[@]}" build-aab -p wadhu-template-app --release 2>&1 | tee -a "$LOG"
  AAB="$(find "$TARGET_DIR/android/makepad-android-aab/wadhu_template_app" -maxdepth 1 -name '*.aab' 2>/dev/null | head -1 || true)"
  if [[ -z "$AAB" ]]; then
    echo "Play bundle build did not complete. See $LOG" >&2
    exit 1
  fi
  cp -f "$AAB" "$STAGE/wadhu-template-app.aab"
  echo "Play bundle → $OUT/wadhu-template-app.aab"
fi
cat > "$STAGE/README.txt" <<EOF
Wadhu Template for Android phones and tablets
======================================
App id: ${WADHU_APP_ID}
Label: ${WADHU_APP_LABEL}
Signed with: ${SIGNED}

Install: copy wadhu-template-app.apk to the device and open it (allow installs
from this source), or: adb install -r wadhu-template-app.apk
An update installs over the last one and keeps the books — never uninstall
to update (that deletes the books on the device).

At launch pick what this device is:
  Server Mode — it holds the shop's books (keep the app open, plugged in).
  Client Mode — Find the shop's server → type the pairing code shown on the
  Server (Settings → Extra tills → Allow extra tills) → sign in with your PIN.
The Server can be any device running Wadhu Template: Windows, Linux or Android.
EOF
# Atomic replace of the published folder (like MSI overwriting the .msi).
rm -rf "$OUT"
mv "$STAGE" "$OUT"
trap - EXIT
echo "APK → $OUT/wadhu-template-app.apk"
