# wadhu-sms-client

Typed HTTP helpers for **Wadhu SMS Gateway** (LAN) and **SMS Relay**
(internet). Same `/v1/messages` schema either way.

```toml
wadhu-sms-client = { path = "../path/to/wadhu-workspace/crates/sms-client" }
```

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

Plain Rust — no Makepad dependency. Docs: [`docs/LOCAL_SMS.md`](../../docs/LOCAL_SMS.md).
