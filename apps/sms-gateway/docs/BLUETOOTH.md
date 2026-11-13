# Bluetooth transport (sms-gateway)

Third parties can enqueue SMS jobs over Bluetooth Classic using the same
JSON fields as HTTP `POST /v1/messages`.

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
| **Desktop / lab** | TCP listener on `127.0.0.1:18766` (falls back to an ephemeral port) speaking this framing — use to integrate without a phone. |
| **Android** | Same framing; Classic RFCOMM SPP UUID `00001101-0000-1000-8000-00805F9B34FB`, discoverable name **Wadhu SMS Gateway**. |

Jobs accepted over Bluetooth enter the **same outbox** as LAN and relay.
