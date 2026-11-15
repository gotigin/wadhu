# SMS Client Demo

Living example of [`wadhu-sms-client`](../../../crates/sms-client): desktop shell
for **easy LAN connect**, HTTP `/v1`, and framed TCP (Bluetooth lab).

| Piece | Role |
|-------|------|
| This app | Operator/demo UI (desktop) |
| `wadhu-sms-client` | Portable SDK for third parties |
| `sms-gateway` | SIM execution device (phone) |
| `sms-relay` | Optional internet plane |

## Connect (normal path — no IP typing)

1. On the phone: SMS Gateway → **Allow a new app** → note the 6-digit code.
2. Same Wi‑Fi as the phone.
3. In this demo: enter the code → **Find phone & connect**.
4. Check health / send. URL and token fields fill in automatically.

See [`docs/TRANSPORTS.md`](../../../docs/TRANSPORTS.md).

## Other modes

| Mode | When |
|------|------|
| **Advanced / relay** | Paste relay URL + key (or inspect auto-filled LAN fields) |
| **Framed TCP (BT lab)** | Gateway on this PC → `:18766` framing |
| **Phone RFCOMM** | Brand apps / native BT — [`BLUETOOTH.md`](../../sms-gateway/docs/BLUETOOTH.md) |

## Not in this demo

- Shipping Android/iOS packaging of *this* shell (library still targets them)
- Production RFCOMM UI (lab framing only here)
