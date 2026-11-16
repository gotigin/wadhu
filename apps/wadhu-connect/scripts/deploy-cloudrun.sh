#!/usr/bin/env bash
# Build + deploy Wadhu Connect to Cloud Run (project wadhu-ac8dc, us-central1).
#
#   ./apps/wadhu-connect/scripts/deploy-cloudrun.sh
#
# Requires: gcloud, docker, apps/wadhu-connect/.env with Meta credentials.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
APP="$ROOT/apps/wadhu-connect"
ENV_FILE="$APP/.env"
PROJECT="${GCP_PROJECT:-wadhu-ac8dc}"
REGION="${GCP_REGION:-us-central1}"
SERVICE="${CLOUD_RUN_SERVICE:-wadhu-connect}"
REPO="${ARTIFACT_REPO:-wadhu}"
IMAGE="${REGION}-docker.pkg.dev/${PROJECT}/${REPO}/${SERVICE}"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "error: missing $ENV_FILE" >&2
  exit 1
fi

# shellcheck disable=SC1090
set -a
# Load only KEY=VAL lines (no export of unrelated shell state).
while IFS= read -r line || [[ -n "$line" ]]; do
  [[ -z "$line" || "$line" =~ ^[[:space:]]*# ]] && continue
  [[ "$line" == *=* ]] || continue
  key="${line%%=*}"
  val="${line#*=}"
  val="${val%\"}"
  val="${val#\"}"
  val="${val%\'}"
  val="${val#\'}"
  export "$key=$val"
done < "$ENV_FILE"
set +a

: "${META_APP_ID:?META_APP_ID required in .env}"
: "${META_APP_SECRET:?META_APP_SECRET required in .env}"
: "${META_CONFIG_ID:?META_CONFIG_ID required in .env}"
: "${WEBHOOK_VERIFY_TOKEN:?WEBHOOK_VERIFY_TOKEN required in .env}"

PUBLIC_ORIGIN="${PUBLIC_ORIGIN:-https://connect.wadhu.com}"
META_GRAPH_VERSION="${META_GRAPH_VERSION:-v22.0}"

echo "1/5 Ensure Artifact Registry + auth"
gcloud config set project "$PROJECT" >/dev/null
gcloud config set run/region "$REGION" >/dev/null
gcloud artifacts repositories describe "$REPO" --location="$REGION" >/dev/null 2>&1 \
  || gcloud artifacts repositories create "$REPO" \
       --repository-format=docker --location="$REGION" \
       --description='Wadhu container images'
gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

echo "2/5 Upsert Secret Manager secrets (from .env)"
upsert_secret() {
  local name="$1" value="$2"
  if gcloud secrets describe "$name" --project="$PROJECT" >/dev/null 2>&1; then
    printf '%s' "$value" | gcloud secrets versions add "$name" --data-file=- --project="$PROJECT" >/dev/null
  else
    printf '%s' "$value" | gcloud secrets create "$name" --data-file=- --project="$PROJECT" --replication-policy=automatic >/dev/null
  fi
}
upsert_secret META_APP_ID "$META_APP_ID"
upsert_secret META_APP_SECRET "$META_APP_SECRET"
upsert_secret META_CONFIG_ID "$META_CONFIG_ID"
upsert_secret WEBHOOK_VERIFY_TOKEN "$WEBHOOK_VERIFY_TOKEN"
if [[ -n "${META_SYSTEM_USER_TOKEN:-}" ]]; then
  upsert_secret META_SYSTEM_USER_TOKEN "$META_SYSTEM_USER_TOKEN"
fi

echo "3/5 Docker build → $IMAGE:latest"
cd "$ROOT"
docker build -f apps/wadhu-connect/Dockerfile -t "${IMAGE}:latest" .
docker push "${IMAGE}:latest"

PROJECT_NUMBER="$(gcloud projects describe "$PROJECT" --format='value(projectNumber)')"
RUNTIME_SA="${PROJECT_NUMBER}-compute@developer.gserviceaccount.com"
SECRET_NAMES=(META_APP_ID META_APP_SECRET META_CONFIG_ID WEBHOOK_VERIFY_TOKEN)
SECRET_MOUNTS="META_APP_ID=META_APP_ID:latest,META_APP_SECRET=META_APP_SECRET:latest,META_CONFIG_ID=META_CONFIG_ID:latest,WEBHOOK_VERIFY_TOKEN=WEBHOOK_VERIFY_TOKEN:latest"
if [[ -n "${META_SYSTEM_USER_TOKEN:-}" ]]; then
  SECRET_NAMES+=(META_SYSTEM_USER_TOKEN)
  SECRET_MOUNTS+=",META_SYSTEM_USER_TOKEN=META_SYSTEM_USER_TOKEN:latest"
fi
for s in "${SECRET_NAMES[@]}"; do
  gcloud secrets add-iam-policy-binding "$s" \
    --member="serviceAccount:${RUNTIME_SA}" \
    --role="roles/secretmanager.secretAccessor" \
    --project="$PROJECT" >/dev/null
done

echo "4/5 Deploy Cloud Run service $SERVICE"
ENV_VARS="HOST=0.0.0.0,PUBLIC_ORIGIN=${PUBLIC_ORIGIN},META_GRAPH_VERSION=${META_GRAPH_VERSION},WADHU_CONNECT_ROOT=/app,PUBLIC_DIR=/app/public,DATA_DIR=/tmp/wadhu-connect-data"
if [[ -n "${META_PHONE_NUMBER_ID:-}" ]]; then
  ENV_VARS+=",META_PHONE_NUMBER_ID=${META_PHONE_NUMBER_ID}"
fi
if [[ -n "${META_WABA_ID:-}" ]]; then
  ENV_VARS+=",META_WABA_ID=${META_WABA_ID}"
fi
gcloud run deploy "$SERVICE" \
  --project="$PROJECT" \
  --region="$REGION" \
  --image="${IMAGE}:latest" \
  --allow-unauthenticated \
  --port=8080 \
  --memory=512Mi \
  --cpu=1 \
  --min-instances=0 \
  --max-instances=3 \
  --timeout=300 \
  --set-env-vars="$ENV_VARS" \
  --set-secrets="$SECRET_MOUNTS"

URL="$(gcloud run services describe "$SERVICE" --project="$PROJECT" --region="$REGION" --format='value(status.url)')"
echo "5/5 Smoke"
curl -sS -o /dev/null -w "onboard %{http_code}\n" "$URL/onboard"
curl -sS -o /dev/null -w "privacy %{http_code}\n" "$URL/privacy"
curl -sS "$URL/api/config" | head -c 400; echo
curl -sS "$URL/v1/health"; echo

echo
echo "Cloud Run URL: $URL"
echo "Next: map custom domain connect.wadhu.com → this service, then Meta webhook Verify."
echo "  gcloud beta run domain-mappings create --service=$SERVICE --domain=connect.wadhu.com --region=$REGION"
