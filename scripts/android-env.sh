#!/usr/bin/env bash
# Shared Android env for each app's package-android.sh / android-dev.sh.
# shellcheck disable=SC2034

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Must match scripts/setup-makepad.sh — cargo-makepad and vendor/makepad are
# the same upstream commit. Picking a newer cargo git checkout breaks Android
# packaging (e.g. requiring `.makepad.font-assets.v1` the app tree does not emit).
MAKEPAD_REV="${MAKEPAD_REV:-915ce7c4e271593101b46dcff0aa96e4466864af}"
MAKEPAD_REV_SHORT="${MAKEPAD_REV:0:7}"

# Materialize vendor/makepad-full at MAKEPAD_REV (includes tools/cargo_makepad).
ensure_makepad_full() {
  local dest="$ROOT/vendor/makepad-full"
  local stamp="$dest/.wadhu-makepad-rev"
  local cargo_git="${CARGO_HOME:-$HOME/.cargo}/git"
  local checkout="$cargo_git/checkouts/makepad-ec2f134f34cd9f98/${MAKEPAD_REV_SHORT}"
  local cargo_db="$cargo_git/db/makepad-ec2f134f34cd9f98"

  if [[ -d "$dest/tools/cargo_makepad" && -f "$stamp" ]] \
    && [[ "$(cat "$stamp" 2>/dev/null || true)" == "$MAKEPAD_REV" ]]; then
    return 0
  fi

  echo "Preparing vendor/makepad-full at $MAKEPAD_REV_SHORT for cargo-makepad…"
  mkdir -p "$ROOT/vendor"
  local src=""
  if [[ -d "$checkout/tools/cargo_makepad" ]]; then
    src="$checkout"
  elif git -C "$cargo_db" cat-file -e "${MAKEPAD_REV}^{commit}" 2>/dev/null; then
    src="$(mktemp -d)/makepad"
    mkdir -p "$src"
    git -C "$cargo_db" archive "$MAKEPAD_REV" | tar -x -C "$src"
  else
    src="$(mktemp -d)/makepad"
    git init -q "$src"
    git -C "$src" fetch -q --depth 1 https://github.com/makepad/makepad.git "$MAKEPAD_REV"
    git -C "$src" checkout -q FETCH_HEAD
  fi

  rm -rf "$dest"
  mkdir -p "$dest"
  # Keep a complete workspace: cargo-makepad is a member of the Makepad root
  # Cargo.toml, so every workspace.members path must exist on disk (unlike
  # vendor/makepad, which is never loaded as that workspace). Drop only
  # build/VCS noise — not examples/ or apps/.
  rsync -a \
    --exclude '.git/' \
    --exclude 'target/' \
    --exclude 'node_modules/' \
    "$src/" "$dest/"
  printf '%s\n' "$MAKEPAD_REV" >"$stamp"
  if [[ "$src" == /tmp/* || "$src" == "${TMPDIR:-/tmp}"/* ]]; then
    rm -rf "$(dirname "$src")"
  fi
}

resolve_makepad_root() {
  if [[ -n "${MAKEPAD_ROOT:-}" && -d "$MAKEPAD_ROOT/tools/cargo_makepad" ]]; then
    return 0
  fi
  ensure_makepad_full
  MAKEPAD_ROOT="$ROOT/vendor/makepad-full"
  if [[ ! -d "$MAKEPAD_ROOT/tools/cargo_makepad" ]]; then
    echo "MAKEPAD_ROOT missing tools/cargo_makepad at $MAKEPAD_ROOT (rev $MAKEPAD_REV)" >&2
    return 1
  fi
}

# Real API migrations (associated MAX/MIN, Atomic::try_update) on the
# cargo-makepad tree and the app path-dep — not allow(deprecated).
fix_makepad_android_deprecations() {
  local app_makepad="$ROOT/vendor/makepad"
  if [[ -d "$app_makepad" && "$MAKEPAD_ROOT" != "$app_makepad" ]]; then
    "$ROOT/scripts/fix-makepad-rust-deprecations.sh" "$MAKEPAD_ROOT" "$app_makepad"
  else
    "$ROOT/scripts/fix-makepad-rust-deprecations.sh" "$MAKEPAD_ROOT"
  fi
}

# cargo-makepad installs INTO --sdk-path (ANDROID_HOME), not under tools/cargo_makepad.
# These match ANDROID_SDK_URLS_33 in vendor/makepad-full/tools/cargo_makepad/src/android/sdk.rs
android_toolchain_ready() {
  local sdk="$1"
  [[ -x "$sdk/platform-tools/adb" ]] \
    && [[ -d "$sdk/build-tools/33.0.1" ]] \
    && [[ -d "$sdk/platforms/android-33-ext4" || -d "$sdk/platforms/android-33" ]] \
    && [[ -d "$sdk/ndk/28.2.13676358" ]] \
    && [[ -x "$sdk/openjdk/bin/java" || -x "$sdk/openjdk/jdk-17.0.2/bin/java" ]] \
    && [[ -f "$sdk/bundletool/bundletool.jar" ]]
}

ensure_rust_android_target() {
  if ! rustup target list --installed 2>/dev/null | grep -qx 'aarch64-linux-android'; then
    echo "Adding rustup target aarch64-linux-android…"
    rustup target add aarch64-linux-android
  fi
}

# Only call cargo-makepad install-toolchain when ANDROID_HOME is missing pieces.
# That command always re-downloads zips — never run it "just in case".
ensure_android_toolchain() {
  local sdk="$1"
  ensure_rust_android_target
  if android_toolchain_ready "$sdk"; then
    echo "Android toolchain already present under $sdk — skipping download."
    return 0
  fi
  echo "Android toolchain incomplete under $sdk — running install-toolchain once…"
  echo "  (missing one of: platform-tools/adb, build-tools/33.0.1, platforms/android-33*, ndk/28.2.13676358, openjdk, bundletool)"
  cargo run --manifest-path "$MAKEPAD_ROOT/tools/cargo_makepad/Cargo.toml" --release -- \
    android --sdk-path="$sdk" install-toolchain
}

android_pkg_opts() {
  local sdk="$1"
  # The app's ids, from load_wadhu_env (no default: never another app's id).
  local pkg="${WADHU_APP_ID:?load_wadhu_env first (WADHU_APP_ID)}"
  local label="${WADHU_APP_LABEL:?load_wadhu_env first (WADHU_APP_LABEL)}"
  ANDROID_PKG_OPTS=(
    --package-name="$pkg"
    --app-label="$label"
    --abi=aarch64
    --sdk-path="$sdk"
  )
  echo "Android package: $pkg ($label)"
}

# Release builds: cargo-makepad reads version_code from the app's Cargo.toml
# `[package.metadata.makepad.android]`. Never pass --version-code=auto
# (timestamps burn Play's ceiling and trip Console warnings). Bump the
# integer by hand (+1) per Play upload; new app ids start at 1.
android_release_opts() {
  :
}

# Release signing. An Android update installs only over an app signed with
# the same key — so Play/shop packages use the app's own release keystore.
# cargo-makepad's `build` always signs with Makepad's bundled debug key;
# android_release_sign re-signs the APK in place.
#
# Defaults (no env required when these files exist):
#   $HOME/<slug>-release.jks            production / Play upload keystore
#   $HOME/<slug>-release.jks.password   store+key password (mode 600, one line)
#   $HOME/<slug>-release.jks.makepad    alias sidecar (optional; else "upload")
# where <slug> is the lowercased product label with spaces removed
# (WADHU_APP_LABEL "SMS Gateway" → smsgateway), else basename of WADHU_APP_DIR.
#
# Optional overrides:
#   WADHU_ANDROID_KEYSTORE       keystore path
#   WADHU_ANDROID_KEYSTORE_PASS  password (else read from <keystore>.password)
#   WADHU_ANDROID_KEY_ALIAS      default: sibling <path>.makepad alias, else "upload"
#
# Keep the keystore and its password file outside the repo, backed up: a lost
# key means no more Play updates for that package id.

android_release_keystore_path() {
  if [[ -n "${WADHU_ANDROID_KEYSTORE:-}" ]]; then
    printf '%s\n' "$WADHU_ANDROID_KEYSTORE"
    return 0
  fi
  local slug=""
  if [[ -n "${WADHU_APP_LABEL:-}" ]]; then
    # Strip trailing " Dev" so development packaging still finds the same
    # production keystore as release packages.
    local label="${WADHU_APP_LABEL%" Dev"}"
    slug="$(printf '%s' "$label" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')"
  elif [[ -n "${WADHU_APP_DIR:-}" ]]; then
    slug="$(basename "$WADHU_APP_DIR" | tr -d '-')"
  else
    slug="wadhu"
  fi
  printf '%s\n' "${HOME}/${slug}-release.jks"
}

# True when a usable release keystore file is available; exports
# WADHU_ANDROID_KEYSTORE to the resolved path for callers.
android_release_keystore_set() {
  local ks
  ks="$(android_release_keystore_path)"
  export WADHU_ANDROID_KEYSTORE="$ks"
  [[ -f "$ks" ]]
}

# Password from env, else sibling `<keystore>.password` (trim trailing newlines).
android_release_keystore_pass() {
  if [[ -n "${WADHU_ANDROID_KEYSTORE_PASS:-}" ]]; then
    printf '%s\n' "$WADHU_ANDROID_KEYSTORE_PASS"
    return 0
  fi
  local ks="${WADHU_ANDROID_KEYSTORE:-$(android_release_keystore_path)}"
  local pwf="${ks}.password"
  if [[ -f "$pwf" ]]; then
    # shellcheck disable=SC2002
    tr -d '\r\n' <"$pwf"
    return 0
  fi
  return 1
}

android_key_alias() {
  local alias="${WADHU_ANDROID_KEY_ALIAS:-}"
  if [[ -z "$alias" && -n "${WADHU_ANDROID_KEYSTORE:-}" && -f "$WADHU_ANDROID_KEYSTORE.makepad" ]]; then
    alias="$(sed -n 's/^[[:space:]]*alias[[:space:]]*=[[:space:]]*"\{0,1\}\([^"]*\)"\{0,1\}[[:space:]]*$/\1/p' "$WADHU_ANDROID_KEYSTORE.makepad" | head -1)"
  fi
  echo "${alias:-upload}"
}

android_release_sign() {
  local apk="$1" sdk="$2"
  if ! android_release_keystore_set; then
    echo "NOTE: $(basename "$apk") is signed with Makepad's debug key — fine to try out," >&2
    echo "      not for shops. Create \$HOME/<slug>-release.jks and \$HOME/<slug>-release.jks.password" >&2
    echo "      (e.g. smsgateway-release.jks) — or set WADHU_ANDROID_KEYSTORE / _PASS." >&2
    return 0
  fi
  local pass
  if ! pass="$(android_release_keystore_pass)"; then
    echo "Release keystore found at $WADHU_ANDROID_KEYSTORE but no password:" >&2
    echo "  set WADHU_ANDROID_KEYSTORE_PASS or create ${WADHU_ANDROID_KEYSTORE}.password" >&2
    return 1
  fi
  export WADHU_ANDROID_KEYSTORE_PASS="$pass"
  local java="$sdk/openjdk/bin/java"
  [[ -x "$java" ]] || java="$sdk/openjdk/jdk-17.0.2/bin/java"
  local signer="$sdk/build-tools/33.0.1/lib/apksigner.jar"
  # The password goes through the environment, never the command line.
  WADHU_KS_PASS="$WADHU_ANDROID_KEYSTORE_PASS" "$java" -jar "$signer" sign \
    --ks "$WADHU_ANDROID_KEYSTORE" --ks-key-alias "$(android_key_alias)" \
    --ks-pass env:WADHU_KS_PASS --key-pass env:WADHU_KS_PASS "$apk"
  "$java" -jar "$signer" verify --print-certs "$apk" | grep -m1 "certificate DN" || true
  echo "Signed with the release key: $apk"
}

# Signing options for build-aab: the release key when set (password through
# MAKEPAD_KEYSTORE_PASS), else nothing — cargo-makepad then uses its debug
# key, which Play rejects.
android_aab_sign_opts() {
  ANDROID_AAB_SIGN_OPTS=()
  if android_release_keystore_set; then
    local pass
    if ! pass="$(android_release_keystore_pass)"; then
      echo "Release keystore found at $WADHU_ANDROID_KEYSTORE but no password:" >&2
      echo "  set WADHU_ANDROID_KEYSTORE_PASS or create ${WADHU_ANDROID_KEYSTORE}.password" >&2
      return 1
    fi
    export WADHU_ANDROID_KEYSTORE_PASS="$pass"
    export MAKEPAD_KEYSTORE_PASS="$WADHU_ANDROID_KEYSTORE_PASS"
    ANDROID_AAB_SIGN_OPTS=(--keystore="$WADHU_ANDROID_KEYSTORE" --keystore-key-alias="$(android_key_alias)")
  fi
}
