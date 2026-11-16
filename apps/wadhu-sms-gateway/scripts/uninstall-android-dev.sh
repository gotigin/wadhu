#!/usr/bin/env bash
# Uninstall SMS Gateway Dev from a connected Android device/emulator.
# Package id: com.wadhu.smsgateway.debug
set -euo pipefail
APP_ID=com.wadhu.smsgateway.debug
echo "adb uninstall $APP_ID"
adb uninstall "$APP_ID"
