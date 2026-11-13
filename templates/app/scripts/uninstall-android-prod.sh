#!/usr/bin/env bash
# Uninstall Wadhu Template from a connected Android device/emulator.
# Package id: com.wadhu.template
set -euo pipefail
APP_ID=com.wadhu.template
echo "adb uninstall $APP_ID"
adb uninstall "$APP_ID"
