# SMS Relay

Self-hostable **remote control plane** for [SMS Gateway](../wadhu-sms-gateway/README.md)
devices. Internet clients submit SMS here; the relay pushes jobs to paired
Android phones over an outbound WebSocket. LAN and Bluetooth on the phone
never require this service.

```bash
cargo wadhu-sms-relay-dev
# → http://127.0.0.1:9876
# admin token (dev): dev-admin-token
```

Env:

| Variable | Default (dev shim) | Purpose |
|---|---|---|
| `SMS_RELAY_BIND` | `127.0.0.1:9876` | Listen address |
| `SMS_RELAY_DATA` | `target/sms-relay-data` | Durable state directory |
| `SMS_RELAY_ADMIN_TOKEN` | `dev-admin-token` | Admin API auth |

Docs: [`docs/SMS_RELAY.md`](docs/SMS_RELAY.md) ·
[`docs/openapi-v1.yaml`](docs/openapi-v1.yaml) ·
[`docs/SELF_HOST.md`](docs/SELF_HOST.md)
