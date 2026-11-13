# Self-hosting Wadhu SMS Relay

## Run

```bash
export SMS_RELAY_BIND=0.0.0.0:9876
export SMS_RELAY_DATA=/var/lib/wadhu-sms-relay
export SMS_RELAY_ADMIN_TOKEN='replace-with-long-random-secret'
cargo run -p wadhu-sms-relay --release
# or: install the binary and run under systemd / docker
```

## TLS

Terminate TLS in front of the binary (Caddy, nginx, Traefik). Example Caddy:

```
relay.example.com {
  reverse_proxy 127.0.0.1:9876
}
```

Phones dial `ws://` or (via proxy) `wss://` to the same host. The Android
gateway accepts `http://host:port` in the relay URL field and opens
`ws://host:port/v1/device/ws`.

## Firewall

- Expose only the reverse-proxy HTTPS port publicly.
- Keep `SMS_RELAY_ADMIN_TOKEN` secret; use it only for pairing and key minting.
- Client apps use **client keys** from `POST /v1/keys`, not the admin token.

## Durability

State is an atomic JSON file under `SMS_RELAY_DATA/relay-state.json`
(devices, keys, messages). Back up that directory.
