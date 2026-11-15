# wadhu-sms-client

Typed HTTP helpers for **Wadhu SMS Gateway** (LAN) and **SMS Relay**
(internet). Same `/v1/messages` schema either way. No Makepad dependency.

```toml
wadhu-sms-client = { path = "../path/to/wadhu-workspace/crates/sms-client" }
```

## Platforms

| Target | How to call |
|--------|-------------|
| Linux, Windows, macOS, Android, iOS (native) | Sync: `health`, `capabilities`, `accept`, `get_message`, `status`, `list_messages` over `http://` |
| Browsers / Edge (`wasm32-unknown-unknown`) | Sync methods return `ClientError::Unsupported`. Use `health_async`, `accept_async`, … (browser `fetch`). `https://` works via fetch. |

Browser pages talking to a phone on LAN need the gateway to send CORS
headers (Wadhu sms-gateway does). Mixed content still blocks
`https://` pages calling `http://192.168.…`.

## Example (native) — easy LAN connect (no IP typing)

On the phone: SMS Gateway → **Allow a new app** → note the 6-digit code.  
In the brand app:

```rust
use wadhu_sms_client::{AcceptRequest, Client};

// Discovers the phone on Wi‑Fi, then pairs with the Allow code.
let client = Client::connect_on_lan("123456", "Moklanjo")?;
let msg = client.accept(&AcceptRequest {
    to: "+15551212".into(),
    body: "hello".into(),
    idempotency_key: Some("order-42".into()),
})?;
println!("{} → {}", msg.id, msg.status);
```

Advanced / tools may still use `Client::new(base, operator_or_app_token)`.  
Relay: pass the relay base URL + client key. See [`docs/TRANSPORTS.md`](../../docs/TRANSPORTS.md).

### Framed TCP (Bluetooth lab)

When the SMS Gateway runs on the **same desktop**, it listens on
`127.0.0.1:18766` with the same length-prefixed JSON as Classic RFCOMM
([`BLUETOOTH.md`](../../apps/sms-gateway/docs/BLUETOOTH.md)):

```rust
use wadhu_sms_client::framed::FramedClient;

let mut bt = FramedClient::connect_lab("your-api-key")?;
bt.ping()?;
let reply = bt.send("+15551212", "hello via framed TCP", Some("lab-1"))?;
```

Production phone↔nearby-device path uses **Classic RFCOMM SPP** on the
gateway; this helper proves framing without cloud. Native RFCOMM clients
belong in brand apps (not this HTTP/wasm crate path).

Desktop integrator UI: [`apps/sms-client-demo`](../../apps/sms-client-demo).
Doctrine: [`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md).
