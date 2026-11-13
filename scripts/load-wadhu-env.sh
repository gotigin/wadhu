# Source from an app's scripts:  source "$ROOT/scripts/load-wadhu-env.sh"
# then: load_wadhu_env development|production
# Exports keys from the app's env file and sets WADHU_ENV.
#
# Env files live with each app (apps/<app>/env/, not the workspace root), so
# every Wadhu app owns its own ids and labels.
# Which app: WADHU_ENV_DIR, else WADHU_APP_DIR — one of them is required
# (no default app: a missing one must fail, not package another app).
load_wadhu_env() {
  local name="${1:?usage: load_wadhu_env development|production}"
  local dir="${WADHU_ENV_DIR:-${WADHU_APP_DIR:+$WADHU_APP_DIR/env}}"
  if [[ -z "$dir" ]]; then
    echo "load_wadhu_env: set WADHU_APP_DIR (apps/<app>) first" >&2
    return 1
  fi
  local file="$dir/${name}.env"
  if [[ ! -f "$file" ]]; then
    echo "Missing Wadhu env file: $file" >&2
    return 1
  fi
  # Parse KEY=VALUE (don't `source` — unquoted spaces break bash, e.g. "App Dev").
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line#"${line%%[![:space:]]*}"}"
    [[ -z "$line" || "$line" == \#* ]] && continue
    [[ "$line" != *=* ]] && continue
    local key="${line%%=*}"
    local value="${line#*=}"
    key="${key%"${key##*[![:space:]]}"}"
    value="${value#"${value%%[![:space:]]*}"}"
    value="${value%"${value##*[![:space:]]}"}"
    value="${value#\"}"
    value="${value%\"}"
    value="${value#\'}"
    value="${value%\'}"
    export "$key=$value"
  done <"$file"
  export WADHU_ENV="$name"
  echo "Wadhu env: $name (APP_ID=${WADHU_APP_ID:-?} LABEL=${WADHU_APP_LABEL:-?})"
}

# Point Makepad's window/taskbar icons at an app's own resources/ when it has
# them; otherwise leave the workspace default (.cargo/config.toml → Wadhu mark).
export_wadhu_app_icons() {
  local res="${1:?usage: export_wadhu_app_icons <app-dir>}/resources"
  [[ -f "$res/icon_32.png" ]] || return 0
  local size
  for size in 32 64 128 256 512 1024; do
    export "MAKEPAD_APP_ICON_${size}=$res/icon_${size}.png"
  done
  export MAKEPAD_APP_ICON_ICO="$res/icon.ico"
}
