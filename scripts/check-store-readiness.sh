#!/usr/bin/env bash
# File-level store / brand readiness. Does not build an APK or talk to Play.
#
#   ./scripts/check-store-readiness.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
fail=0

note() { printf '  ok  %s\n' "$1"; }
bad() { printf '  FAIL %s\n' "$1" >&2; fail=1; }
warn() { printf '  warn %s\n' "$1"; }

need_file() {
  if [[ -f "$1" ]]; then
    note "present $1"
  else
    bad "missing $1"
  fi
}

need_grep() {
  local file="$1" pattern="$2" label="$3"
  if [[ ! -f "$file" ]]; then
    bad "missing $file ($label)"
    return
  fi
  if grep -qF "$pattern" "$file"; then
    note "$label"
  else
    bad "$label — expected [$pattern] in $file"
  fi
}

forbid_grep() {
  local file="$1" pattern="$2" label="$3"
  if [[ -f "$file" ]] && grep -qF "$pattern" "$file"; then
    bad "$label — found [$pattern] in $file"
  else
    note "$label"
  fi
}

echo "Brand"
need_grep docs/BRAND.md "Memon" "Wadhu etymology is Memon"
need_grep docs/BRAND.md "not a messaging brand" "brand table says Wadhu is not a messaging brand"
need_grep docs/legal/ABOUT.md "means *more* in Memon" "public about page states Memon more"
need_grep sites/wadhu/src/ui.rs "Memon for “more”" "homepage names band uses Memon"
forbid_grep sites/wadhu/src/ui.rs "Gujarati" "homepage does not call Wadhu Gujarati"
forbid_grep sites/wadhu/src/ui.rs "Kutchi" "homepage does not call Wadhu Kutchi"
forbid_grep apps/wadhu-sms-gateway/scripts/package-android.sh "shop's books" "gateway package note is not a till app"
forbid_grep apps/wadhu-sms-client-demo/scripts/package-android.sh "Extra tills" "client-demo package note is not a till app"
forbid_grep templates/app/scripts/package-android.sh "shop's books" "app template package note is not a till app"
if [[ -d apps/gotigin-connect ]]; then
  bad "apps/gotigin-connect must not exist"
else
  note "no apps/gotigin-connect"
fi

echo "Legal URLs"
for f in docs/legal/PRIVACY.md docs/legal/TERMS.md docs/legal/CANCELLATION.md docs/legal/DATA_DELETION.md docs/legal/SUPPORT.md docs/legal/ABOUT.md; do
  need_file "$f"
done
need_file sites/wadhu/static/robots.txt
need_file sites/wadhu/static/sitemap.xml
need_grep sites/wadhu/static/robots.txt "Sitemap: https://wadhu.com/sitemap.xml" "robots points at sitemap"
need_grep sites/wadhu/static/sitemap.xml "https://wadhu.com/cancellation" "sitemap lists cancellation"
need_grep sites/wadhu/static/cancellation/index.html "Cancellation &amp; refund — Wadhu" "cancellation title is singly escaped"
forbid_grep sites/wadhu/static/cancellation/index.html "&amp;amp;" "cancellation HTML has no double-escaped amp"
need_grep sites/wadhu/firebase.json "X-Content-Type-Options" "firebase.json sets nosniff"
need_grep apps/wadhu-sms-gateway/docs/PLAY_STORE_LISTING.md "https://wadhu.com/privacy" "Play listing privacy URL"
need_grep apps/wadhu-sms-gateway/docs/PLAY_STORE_LISTING.md "https://wadhu.com/data-deletion" "Play listing deletion URL"
need_grep apps/wadhu-sms-gateway/docs/PLAY_STORE_LISTING.md "https://wadhu.com/cancellation" "Play listing cancellation URL"
need_grep apps/wadhu-sms-gateway/docs/PLAY_STORE_LISTING.md "Tools · Apps · More" "Play listing names the Wadhu brand"
need_grep apps/wadhu-connect/README.md "https://wadhu.com/privacy" "Connect privacy URL"
need_grep apps/wadhu-connect/README.md "Product" "Connect README calls itself a product"

echo "SMS Gateway Android"
need_grep apps/wadhu-sms-gateway/Cargo.toml 'identifier = "com.wadhu.smsgateway"' "production app id"
need_grep apps/wadhu-sms-gateway/Cargo.toml "version_code = 1" "version_code is a hand-set integer"
need_grep apps/wadhu-sms-gateway/Cargo.toml "min_sdk_version = 26" "min SDK 26"
if grep -q 'version-code=auto' apps/wadhu-sms-gateway/scripts/package-android.sh; then
  bad "package-android.sh must not pass --version-code=auto"
else
  note "package script does not use version-code=auto"
fi
need_grep apps/wadhu-sms-gateway/scripts/package-android.sh "android_write_play_crash_files" "package script writes Play crash files"
need_grep scripts/android-env.sh "android_write_play_crash_files()" "android env defines crash-file helper"
if [[ -f "$HOME/smsgateway-release.jks" ]]; then
  note "release keystore ~/smsgateway-release.jks"
else
  warn "no ~/smsgateway-release.jks — Play AAB stays unsigned until the keystore exists"
fi
if [[ -d "${ANDROID_HOME:-$HOME/Android/Sdk}" ]]; then
  note "Android SDK at ${ANDROID_HOME:-$HOME/Android/Sdk}"
else
  warn "Android SDK not found — package-android.sh cannot run here"
fi

echo "Other products"
need_grep apps/wadhu-sms-relay/README.md "Product" "SMS Relay README calls itself a product"
need_grep apps/wadhu-sms-client-demo/README.md "not a store product" "client demo is labeled a demo"
need_file apps/wadhu-sms-relay/docs/SMS_RELAY.md
need_file apps/wadhu-connect/docs/STATUS.md

if [[ "$fail" -ne 0 ]]; then
  echo "Store readiness: FAIL" >&2
  exit 1
fi
echo "Store readiness: PASS (file checks; device smoke and Play SMS policy are still manual)"
