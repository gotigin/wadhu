# tools/

Thin binaries behind cargo aliases (`.cargo/config.toml`). Each one only
runs the matching script under `apps/<app>/scripts/` (or
`templates/app/scripts/` for the starters). Never share a shim between
apps; fork it. Shims depend on no workspace member.
`./scripts/new-app.sh` creates the three shims and aliases for a new app.

| Shim | Alias | Runs |
|---|---|---|
| `template-app-dev` | `cargo template-app-dev` | `templates/app/scripts/dev.sh` |
| `template-app-android-dev` | `cargo template-app-android-dev` | `templates/app/scripts/android-dev.sh` |
| `template-app-windows-dev` | `cargo template-app-windows-dev` | `templates/app/scripts/windows-dev.sh` |
| `wadhu-sms-gateway-dev` | `cargo wadhu-sms-gateway-dev` | `apps/wadhu-sms-gateway/scripts/dev.sh` |
| `wadhu-sms-gateway-android-dev` | `cargo wadhu-sms-gateway-android-dev` | `apps/wadhu-sms-gateway/scripts/android-dev.sh` |
| `wadhu-sms-gateway-windows-dev` | `cargo wadhu-sms-gateway-windows-dev` | `apps/wadhu-sms-gateway/scripts/windows-dev.sh` |
| `wadhu-sms-relay-dev` | `cargo wadhu-sms-relay-dev` | `apps/wadhu-sms-relay/scripts/dev.sh` |
| `wadhu-sms-client-demo-dev` | `cargo wadhu-sms-client-demo-dev` | `apps/wadhu-sms-client-demo/scripts/dev.sh` |
| `wadhu-connect-dev` | `cargo wadhu-connect-dev` | `apps/wadhu-connect/scripts/dev.sh` |
