#!/usr/bin/env bash
# Production SMS Client Demo Windows package -> dist/sms-client-demo-windows/
exec "$(cd "$(dirname "$0")" && pwd)/windows-dev.sh" prod
