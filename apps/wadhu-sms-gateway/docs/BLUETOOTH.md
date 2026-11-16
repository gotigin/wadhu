# Bluetooth transport (sms-gateway)

Third parties can enqueue SMS jobs over **Bluetooth Classic RFCOMM** using the
same JSON fields as HTTP `POST /v1/messages`. Jobs enter the **same outbox** as
LAN and relay — no cloud required.

Offline paths that work without relay / wadhu.com:

| Path | Needs internet? | Notes |
|------|-----------------|--------|
| LAN HTTP `:18765` | No (same Wi‑Fi / hotspot) | Discover + Allow code — see [`TRANSPORTS.md`](../../../docs/TRANSPORTS.md) |
| Classic RFCOMM SPP | No (nearby paired device) | This doc (same per-app tokens as LAN when `token` is sent) |
| Framed TCP `:18766` lab | No | Desktop simulator of the same framing |

Product UX for non-experts (LAN / BT / relay): [`docs/TRANSPORTS.md`](../../../docs/TRANSPORTS.md).

## Framing

Every message is **length-prefixed JSON**:

```text
uint32_be length | utf-8 JSON body   (max 64 KiB)
```

## Operations

### Send

```json
{
  "op": "send",
  "token": "<gateway API key>",
  "to": "+919876543210",
  "body": "Your parcel has been dispatched.",
  "idempotency_key": "optional-unique-key"
}
```

Reply:

```json
{"ok":true,"message":{"id":"msg-…","status":"queued","to":"+91…","idempotency_key":"…"}}
```

or `{"ok":false,"error":"…"}`.

### Ping

```json
{"op":"ping"}
```

→ `{"ok":true,"pong":true}`

## Endpoints

| Platform | How |
|---|---|
| **Android (production nearby)** | Classic RFCOMM SPP server on the phone. Service name **Wadhu SMS Gateway**. UUID `00001101-0000-1000-8000-00805F9B34FB` (standard Serial Port Profile). Status chip **On** when listening (`rfcomm://Wadhu SMS Gateway (SPP)`). Requires runtime `BLUETOOTH_CONNECT` + `BLUETOOTH_ADVERTISE` (API 31+); grant via the app’s permission button. Pair/discover the phone, then open an RFCOMM socket to that UUID and speak this framing. |
| **Desktop / lab** | TCP listener on `127.0.0.1:18766` speaking **identical** framing — integrate without a phone. Use `wadhu-sms-client::framed::FramedClient` or the SMS Client Demo “Framed TCP (BT lab)” section. |

### Client helpers

- **Lab / desktop:** `FramedClient::connect_lab(api_key)` or `connect_tcp(host, 18766, api_key)` in [`crates/wadhu-sms-client`](../../../crates/wadhu-sms-client).
- **Production RFCOMM:** brand apps implement native Classic BT (Android `BluetoothSocket` / iOS External Accessory or platform RFCOMM). The HTTP crate does not open RFCOMM; framing above is the contract.

## Permissions (Android)

Declared in the gateway manifest:

- Legacy: `BLUETOOTH`, `BLUETOOTH_ADMIN` (`maxSdkVersion=30`)
- API 31+: `BLUETOOTH_CONNECT`, `BLUETOOTH_ADVERTISE`, `BLUETOOTH_SCAN` (`neverForLocation`)

Adapter must be enabled. If the chip shows **Need BT**, allow the runtime prompts and leave Bluetooth on.

## Explicit non-goals

- BLE GATT
- Browser Web Bluetooth (Classic RFCOMM is unavailable in browsers)
- Cloud / identity / billing (LAN + BT stay free forever)
