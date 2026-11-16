#!/usr/bin/env bash
# Production SMS Gateway Windows package -> dist/sms-gateway-windows/
exec "$(cd "$(dirname "$0")" && pwd)/windows-dev.sh" prod
