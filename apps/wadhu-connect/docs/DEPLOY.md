# Deploy Wadhu Connect → connect.wadhu.com

## Production (Google Cloud Run)

Project **`wadhu-ac8dc`**, region **`us-central1`** (domain mapping + fullest
Cloud Run feature set). One service serves onboard UI + API + webhook.

```bash
# Needs apps/wadhu-connect/.env with Meta credentials (never commit).
./apps/wadhu-connect/scripts/deploy-cloudrun.sh
```

That builds the Docker image, pushes to Artifact Registry, stores secrets in
Secret Manager, and deploys service `wadhu-connect`.

Temporary URL (always works before DNS):

`https://wadhu-connect-349667335886.us-central1.run.app`

### Custom domain

```bash
gcloud beta run domain-mappings create \
  --service=wadhu-connect \
  --domain=connect.wadhu.com \
  --region=us-central1
```

Then add the DNS records Google prints (usually a CNAME / A for
`connect.wadhu.com`). After DNS propagates:

- Meta webhook: `https://connect.wadhu.com/webhook`
- Onboard: `https://connect.wadhu.com/onboard`

### Env (Cloud Run)

| Var | Notes |
| --- | --- |
| `PUBLIC_ORIGIN` | `https://connect.wadhu.com` |
| `HOST` | `0.0.0.0` |
| `PORT` | Set by Cloud Run — do not override |
| `META_*` / `WEBHOOK_VERIFY_TOKEN` | Secret Manager |
| `DATA_DIR` | `/tmp/...` (ephemeral until a volume is added) |

## Local

```bash
cargo wadhu-connect-dev
# → http://127.0.0.1:3847/onboard
```

## Brand site (separate)

```bash
./sites/wadhu/scripts/deploy-firebase.sh
```

`wadhu.com` = Firebase Hosting (Makepad SPA + legal). Not the Connect binary.
