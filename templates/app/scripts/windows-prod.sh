#!/usr/bin/env bash
# Production Wadhu Template Windows package -> dist/template-app-windows/
exec "$(cd "$(dirname "$0")" && pwd)/windows-dev.sh" prod
