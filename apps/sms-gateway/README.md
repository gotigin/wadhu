# SMS Gateway

**SIM execution device:** other apps connect here (LAN / Bluetooth) to send
SMS. Optional outbound session to a self-hosted
[SMS Relay](../sms-relay/README.md) for internet clients. Local send never
depends on the relay.

```bash
cargo sms-gateway-dev              # desktop UI + API on 127.0.0.1:18765
cargo sms-gateway-android-dev      # Android gateway (SIM) on LAN :18765
cargo test -p wadhu-sms-gateway
```

Ids: `com.wadhu.smsgateway` / `.debug`.

Docs: [`docs/SMS_GATEWAY.md`](docs/SMS_GATEWAY.md) ·
[`docs/openapi-v1.yaml`](docs/openapi-v1.yaml) ·
[`docs/BLUETOOTH.md`](docs/BLUETOOTH.md) ·
[`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md)

### Client example (LAN)

```bash
TOKEN=…   # from the SMS Gateway window
curl -s -H "X-Api-Key: $TOKEN" http://127.0.0.1:18765/v1/health
curl -s -H "X-Api-Key: $TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+15551212","body":"hello","idempotency_key":"demo-1"}' \
  http://127.0.0.1:18765/v1/messages
```
