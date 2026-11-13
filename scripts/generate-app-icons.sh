#!/usr/bin/env bash
# Regenerate window / launcher icons from a square master image.
# Names no app: the caller says which master and where the icons go.
#
# Usage:
#   ./scripts/generate-app-icons.sh <master.png> <out-dir> [--android]
#
# Examples:
#   ./scripts/generate-app-icons.sh assets/images/wadhu-brand-logo.png assets/icons
#   ./apps/<app>/scripts/generate-icons.sh   (wraps this with the app's paths)
#
# Writes into <out-dir>:
#   icon_{32,64,128,256,512,1024}.png  — Makepad window / taskbar (MAKEPAD_APP_ICON_*)
#   icon.ico                           — Windows (multi-size)
#   app-mark.webp                      — lossless 256px mark for in-app embedding
#   android/res/mipmap-*/ic_launcher.png  (only with --android)
set -euo pipefail
SRC="${1:?usage: generate-app-icons.sh <master.png> <out-dir> [--android]}"
OUT="${2:?usage: generate-app-icons.sh <master.png> <out-dir> [--android]}"
ANDROID="${3:-}"

if [[ ! -f "$SRC" ]]; then
  echo "Missing master image: $SRC" >&2
  exit 1
fi
if ! command -v magick >/dev/null 2>&1; then
  echo "ImageMagick (magick) is required" >&2
  exit 1
fi
mkdir -p "$OUT"

resize_png() {
  local size="$1" dest="$2"
  magick "$SRC" -resize "${size}x${size}" -background none -gravity center \
    -extent "${size}x${size}" "$dest"
  echo "wrote $dest (${size}x${size} png)"
}

for size in 32 64 128 256 512 1024; do
  resize_png "$size" "$OUT/icon_${size}.png"
done

# In-app embedding: lossless WebP keeps alpha (lossy VP8 drops it and Makepad
# then shows a solid black square).
magick "$SRC" -resize 256x256 -background none -gravity center \
  -extent 256x256 -define webp:lossless=true "$OUT/app-mark.webp"
echo "wrote $OUT/app-mark.webp (256x256 webp lossless)"

magick "$SRC" -define icon:auto-resize=256,128,64,48,32,16 "$OUT/icon.ico"
echo "wrote $OUT/icon.ico"

if [[ "$ANDROID" == "--android" ]]; then
  declare -A MIPMAPS=(
    [mipmap-mdpi]=48
    [mipmap-hdpi]=72
    [mipmap-xhdpi]=96
    [mipmap-xxhdpi]=144
    [mipmap-xxxhdpi]=192
  )
  for dir in "${!MIPMAPS[@]}"; do
    mkdir -p "$OUT/android/res/$dir"
    resize_png "${MIPMAPS[$dir]}" "$OUT/android/res/$dir/ic_launcher.png"
  done
fi
