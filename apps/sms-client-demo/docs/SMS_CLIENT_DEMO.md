# SMS Client Demo

Living example of [`wadhu-sms-client`](../../../crates/sms-client): a Makepad
desktop shell that only calls the HTTP `/v1` helpers.

| Piece | Role |
|-------|------|
| This app | Operator/demo UI (desktop) |
| `wadhu-sms-client` | Portable SDK for third parties |
| `sms-gateway` | SIM execution device |
| `sms-relay` | Optional internet plane |

## Run

1. Start SMS Gateway (desktop dry-run or Android device on the same Wi‑Fi).
2. `cargo sms-client-demo-dev`
3. Paste base URL (`http://127.0.0.1:18765` or `http://<phone-ip>:18765`) and API key.
4. Check health → send a test SMS → watch status update.

## Not in this demo

- Android / iOS / browser packaging of *this* app (library still targets them)
- Bluetooth transport
