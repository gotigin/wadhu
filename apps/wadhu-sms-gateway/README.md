# SMS Gateway

**Android-first** SMS execution app: install on a phone, tablet, or other
Android device with a SIM. Client apps (Gotigin or third parties) connect
over LAN or Bluetooth; optional [SMS Relay](../wadhu-sms-relay/README.md) covers
internet clients. Local send never depends on the relay.

```bash
cargo wadhu-sms-gateway-android-dev      # primary path — device / emulator with telephony
cargo wadhu-sms-gateway-dev              # desktop UI/API dry-run only (no SIM)
cargo test -p wadhu-sms-gateway
```

Ids: `com.wadhu.smsgateway` / `.debug`.

Docs: [`docs/SMS_GATEWAY.md`](docs/SMS_GATEWAY.md) ·
[`docs/openapi-v1.yaml`](docs/openapi-v1.yaml) ·
[`docs/BLUETOOTH.md`](docs/BLUETOOTH.md) ·
[`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md) ·
[`docs/ANDROID_PLAY_LESSONS.md`](docs/ANDROID_PLAY_LESSONS.md) (Android / Play do-not-regress)

**Store / legal (required URLs):** privacy
[wadhu.com/privacy](https://wadhu.com/privacy) · terms
[wadhu.com/terms](https://wadhu.com/terms) · support
[wadhu.com/support](https://wadhu.com/support) · data deletion
[wadhu.com/data-deletion](https://wadhu.com/data-deletion) —
sources in [`docs/legal/`](../../docs/legal/README.md). Play paste:
[`docs/PLAY_STORE_LISTING.md`](docs/PLAY_STORE_LISTING.md) ·
[`docs/DATA_SAFETY.md`](docs/DATA_SAFETY.md) ·
[`docs/PUBLISH_PLAY_STORE.md`](docs/PUBLISH_PLAY_STORE.md) ·
Windows: [`docs/PUBLISH_WINDOWS_STORE.md`](docs/PUBLISH_WINDOWS_STORE.md).

Integrator SDK: [`crates/wadhu-sms-client`](../../crates/wadhu-sms-client) · desktop demo:
[`apps/wadhu-sms-client-demo`](../wadhu-sms-client-demo) (`cargo wadhu-sms-client-demo-dev`).

### Client example (LAN)

```bash
TOKEN=…   # from the SMS Gateway screen on the Android device
curl -s -H "X-Api-Key: $TOKEN" http://<device-ip>:18765/v1/health
curl -s -H "X-Api-Key: $TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+15551212","body":"hello","idempotency_key":"demo-1"}' \
  http://<device-ip>:18765/v1/messages
```
