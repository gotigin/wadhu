# SMS Client Demo

Desktop integrator UI for [`wadhu-sms-client`](../../crates/sms-client).
Shows how a third-party app talks to **SMS Gateway** (LAN) or **SMS Relay**
using only the crate — no radio code in this app.

**Desktop only for now** (Linux / Windows / macOS). The library itself also
compiles for Android, iOS, and `wasm32` (browsers / Edge).

```bash
# Terminal 1 — gateway (phone or desktop dry-run)
cargo sms-gateway-dev
# or on a phone: cargo sms-gateway-android-dev

# Terminal 2 — this demo
cargo sms-client-demo-dev
```

Paste the gateway Listen URL + API key, **Check health**, then **Send**.

Ids: `com.wadhu.smsclientdemo` / `.debug`.

Brief: [`docs/SMS_CLIENT_DEMO.md`](docs/SMS_CLIENT_DEMO.md) · doctrine:
[`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md)
