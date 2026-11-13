#!/usr/bin/env bash
# Uninstall SMS Gateway from a connected Android device/emulator.
# Package id: com.wadhu.smsgateway
set -euo pipefail
APP_ID=com.wadhu.smsgateway
echo "adb uninstall $APP_ID"
adb uninstall "$APP_ID"
