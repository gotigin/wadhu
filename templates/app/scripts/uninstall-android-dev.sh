#!/usr/bin/env bash
# Uninstall Wadhu Template Dev from a connected Android device/emulator.
# Package id: com.wadhu.template.debug
set -euo pipefail
APP_ID=com.wadhu.template.debug
echo "adb uninstall $APP_ID"
adb uninstall "$APP_ID"
