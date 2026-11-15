# Deploy Wadhu Connect → connect.wadhu.com

## Binary

```bash
cargo build -p wadhu-connect --release
# binary: target/release/wadhu-connect
```

Env (systemd / container):

```bash
META_APP_ID=…
META_APP_SECRET=…
META_CONFIG_ID=…
WEBHOOK_VERIFY_TOKEN=…
PUBLIC_ORIGIN=https://connect.wadhu.com
HOST=0.0.0.0
PORT=3847
DATA_DIR=/var/lib/wadhu-connect
WADHU_CONNECT_ROOT=/opt/wadhu-connect   # directory containing public/
```

Copy `apps/wadhu-connect/public/` next to the configured root.

Put nginx/caddy TLS in front (or platform HTTPS) for `connect.wadhu.com`.

## DNS

| Name | Target |
| --- | --- |
| `connect.wadhu.com` | Connect host (A/AAAA or CNAME) |
| `wadhu.com` | Firebase Hosting |

## Brand site

```bash
./sites/wadhu/scripts/deploy-firebase.sh
# optional Makepad home:
./sites/wadhu/scripts/deploy-firebase.sh --with-wasm
```

Set `.firebaserc` to your real Firebase project id before first deploy.
