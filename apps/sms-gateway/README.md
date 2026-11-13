# SMS Gateway

**Android-first** SMS execution app: install on a phone, tablet, or other
Android device with a SIM. Client apps (Gotigin or third parties) connect
over LAN or Bluetooth; optional [SMS Relay](../sms-relay/README.md) covers
internet clients. Local send never depends on the relay.

```bash
cargo sms-gateway-android-dev      # primary path — device / emulator with telephony
cargo sms-gateway-dev              # desktop UI/API dry-run only (no SIM)
cargo test -p wadhu-sms-gateway
```

Ids: `com.wadhu.smsgateway` / `.debug`.

Docs: [`docs/SMS_GATEWAY.md`](docs/SMS_GATEWAY.md) ·
[`docs/openapi-v1.yaml`](docs/openapi-v1.yaml) ·
[`docs/BLUETOOTH.md`](docs/BLUETOOTH.md) ·
[`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md)

### Client example (LAN)

```bash
TOKEN=…   # from the SMS Gateway screen on the Android device
curl -s -H "X-Api-Key: $TOKEN" http://<device-ip>:18765/v1/health
curl -s -H "X-Api-Key: $TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+15551212","body":"hello","idempotency_key":"demo-1"}' \
  http://<device-ip>:18765/v1/messages
```
