#!/usr/bin/env bash
# Uninstall SMS Client Demo Dev from a connected Android device/emulator.
# Package id: com.wadhu.smsclientdemo.debug
set -euo pipefail
APP_ID=com.wadhu.smsclientdemo.debug
echo "adb uninstall $APP_ID"
adb uninstall "$APP_ID"
