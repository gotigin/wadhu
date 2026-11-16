# How clients reach SMS Gateway — LAN, Bluetooth, relay

Audience: product + brand apps. **End users are not developers.** They must
not type IP addresses, ports, or curl. The phone owner (SMS Gateway) stays in
control so the setup feels safe.

Doctrine overview: [`LOCAL_SMS.md`](LOCAL_SMS.md).

------------------------------------------------------------------------

## Same job, three doors

Every path feeds the **same outbox → SIM** on the phone.

| Approach | When to use | Needs internet? | Who pays |
|----------|-------------|-----------------|----------|
| **LAN** | Same Wi‑Fi / hotspot as the phone | No | Free |
| **Bluetooth** | Nearby device; Wi‑Fi awkward | No | Free |
| **Relay** | Client far away / different network | Yes | Hosted = subscription; self-host = your server |

Killing relay or letting a cloud subscription expire **must not** break LAN or Bluetooth.

------------------------------------------------------------------------

## LAN (primary for shops)

### What the **client user** should feel

1. Open the brand app → **Connect to SMS phone** (or similar).
2. App finds the gateway on this Wi‑Fi (**no IP typing**).
3. Enter a short **code shown on the phone** (like a pairing PIN), or complete
   an equally simple “Allow” flow on the phone.
4. Done. Send SMS. If the phone’s IP changes later, the app **re-finds** the
   gateway — the user is not asked to “update the Listen URL.”

### What the **phone owner** (SMS Gateway) should feel

1. Tap **Allow a new app** → big code appears for a few minutes.
2. See **Connected apps**: name, how many SMS sent, Allowed / Blocked.
3. **Block** an app that should not send anymore.
4. **Change access key** (rotate) when something feels compromised.
5. Advanced / developer details (raw URL, operator key) stay secondary — not
   the default path for shop staff.

### How it works (under the hood)

| Piece | Behavior |
|-------|----------|
| Discovery | UDP beacon on the LAN (`:18767`) so apps find `host:18765` without mDNS/JNI |
| Pairing | Timed one-time code on the gateway → brand app receives a **per-app token** |
| Auth | Per-app token (or operator key for advanced tools) |
| IP change | App discovers again; token stays valid until blocked or rotated away |
| Abuse controls | Block app, rotate operator key, per-app send counts |

### Today vs target

| | Status |
|--|--------|
| Easy find + code pair (no IP) | **Implemented** (gateway + `wadhu-sms-client` discover/pair) |
| Connected apps / block / counts / rotate | **Implemented** in SMS Gateway |
| Brand-app polished UI | Brand apps should call discover + pair — demo shows the flow |

------------------------------------------------------------------------

## Bluetooth

### What users should feel

1. Phone Bluetooth on; SMS Gateway shows Bluetooth **On**.
2. Pair once in system Bluetooth settings (OS remembers the bond).
3. Brand app → connect to **Wadhu SMS Gateway** (SPP) — no Wi‑Fi IP involved.
4. Same “allowed app” idea as LAN once pairing tokens apply to BT frames
   (token in the JSON `send`); OS bond ≠ permission to spend SMS.

### Under the hood

- Classic RFCOMM SPP UUID `00001101-0000-1000-8000-00805F9B34FB`
- Length-prefixed JSON — see [`apps/wadhu-sms-gateway/docs/BLUETOOTH.md`](../apps/wadhu-sms-gateway/docs/BLUETOOTH.md)
- Desktop lab: TCP `:18766` with the same framing (integrators / CI only)

------------------------------------------------------------------------

## Relay (optional, internet)

### What users should feel

1. Only when the client is **not** on the shop LAN/BT.
2. Phone owner pairs the gateway to a relay once (or uses Wadhu hosted later).
3. Brand app uses cloud/relay credentials from a **Connect** flow — not a
   permanent secret baked into the APK.
4. Local LAN/BT keep working if relay is down.

### Under the hood

- Phone opens outbound WebSocket to relay; clients `POST /v1/messages` to relay
- Same message JSON as LAN
- Monetization (when productized): hosted relay/orgs only — see
  [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md)

------------------------------------------------------------------------

## Security model (gateway feels safe)

| Control | LAN | Bluetooth | Relay |
|---------|-----|-----------|-------|
| Short pairing code / Connect allow | Yes | Token in frame after allow | Relay keys / future OAuth |
| Per-app identity + send counts | Yes | Same tokens when presented | Relay-side keys |
| Block app | Yes (gateway UI) | Blocked token rejected | Revoke relay client key |
| Rotate operator key | Yes | Advanced clients using operator key break until re-paired | Separate from local |

**Non-expert rule:** never require typing `http://192.168.…` as the normal path.
