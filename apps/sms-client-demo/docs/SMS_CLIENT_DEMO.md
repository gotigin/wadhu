# SMS Client Demo

Living example of [`wadhu-sms-client`](../../../crates/sms-client): a Makepad
desktop shell that only calls the HTTP `/v1` helpers.

| Piece | Role |
|-------|------|
| This app | Operator/demo UI (desktop) |
| `wadhu-sms-client` | Portable SDK for third parties |
| `sms-gateway` | SIM execution device (phone) |
| `sms-relay` | Optional internet plane |

## Connect

| Mode | Base URL to paste |
|------|-------------------|
| **LAN (primary)** | Listen URL from the gateway app on the phone — `http://<phone-lan-ip>:18765` (same Wi‑Fi) |
| **Relay** | Your sms-relay bind — e.g. `http://<relay-host>:9876` |
| **Desktop dry-run only** | `http://127.0.0.1:18765` if gateway runs on this PC (no SIM) |
| **Bluetooth** | Not HTTP — see gateway `docs/BLUETOOTH.md` |

1. Start SMS Gateway on Android; grant SMS; copy Listen URL + API key.
2. `cargo sms-client-demo-dev` on a desktop on the same LAN.
3. Paste URL + key → Check health → send a test SMS → watch status.

## Not in this demo

- Android / iOS / browser packaging of *this* app (library still targets them)
- Bluetooth client UI (library/docs point at framing; HTTP demo stays HTTP)
