# SMS Client Demo

Desktop integrator UI for [`wadhu-sms-client`](../../crates/sms-client).
Shows how a third-party app talks to **SMS Gateway** or **SMS Relay**
using only the crate — no radio code in this app.

**Desktop only for now** (Linux / Windows / macOS). The library itself also
compiles for Android, iOS, and `wasm32` (browsers / Edge).

## Connect (real path)

1. Run **SMS Gateway** on an Android device with a SIM  
   (`cargo sms-gateway-android-dev`).
2. On the phone: grant SMS, note **Listen URL** (LAN IP, e.g.
   `http://192.168.x.x:18765`) and **API key**.
3. Same Wi‑Fi as this PC:
   ```bash
   cargo sms-client-demo-dev
   ```
4. Paste that **phone LAN URL** + API key → Check health → Send.

**Relay (internet):** paste your sms-relay base URL (e.g.
`http://<relay-host>:9876`) and client API key instead.

**Bluetooth:** not this HTTP demo — see gateway
[`docs/BLUETOOTH.md`](../sms-gateway/docs/BLUETOOTH.md). Framing is separate;
use the phone’s BT endpoint when integrating BT clients.

`http://127.0.0.1:18765` only applies if you deliberately run
`cargo sms-gateway-dev` on **this same machine** (desktop dry-run, no SIM).
Do not treat localhost as the normal integrator path.

Ids: `com.wadhu.smsclientdemo` / `.debug`.

Brief: [`docs/SMS_CLIENT_DEMO.md`](docs/SMS_CLIENT_DEMO.md) · doctrine:
[`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md)
