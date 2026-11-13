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

## Example (native)

```rust
use wadhu_sms_client::{AcceptRequest, Client};

let client = Client::new("http://127.0.0.1:18765", "your-api-key");
let msg = client.accept(&AcceptRequest {
    to: "+15551212".into(),
    body: "hello".into(),
    idempotency_key: Some("order-42".into()),
})?;
println!("{} → {}", msg.id, msg.status);
```

Desktop integrator UI: [`apps/sms-client-demo`](../../apps/sms-client-demo).
Doctrine: [`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md).
