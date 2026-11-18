#!/usr/bin/env bash
# Regenerates every app icon from vega/brand/vega-icon.svg.
# Requires: rsvg-convert (brew install librsvg), iconutil (macOS), python3.
set -euo pipefail

cd "$(dirname "$0")/../.."
src=vega/brand/vega-icon.svg
res=crates/zed/resources
tmp=$(mktemp -d)
trap 'rm -r "$tmp"' EXIT

# channel -> gradient end color (stable keeps the brand Electric Blue)
channels=("stable:#00D4FF" "preview:#F472B6" "nightly:#10B981" "dev:#F59E0B")

for entry in "${channels[@]}"; do
  channel=${entry%%:*}
  color=${entry#*:}
  svg="$tmp/$channel.svg"
  sed "s/#00D4FF/$color/g" "$src" > "$svg"

  suffix=""
  [[ $channel != stable ]] && suffix="-$channel"

  rsvg-convert -w 512 "$svg" -o "$res/app-icon$suffix.png"
  rsvg-convert -w 1024 "$svg" -o "$res/app-icon$suffix@2x.png"

  pngs=()
  for size in 16 24 32 48 64 128 256; do
    rsvg-convert -w "$size" "$svg" -o "$tmp/$channel-$size.png"
    pngs+=("$tmp/$channel-$size.png")
  done
  python3 -I vega/brand/png_to_ico.py "$res/windows/app-icon$suffix.ico" "${pngs[@]}"
done

# macOS document icon (file associations)
iconset="$tmp/Document.iconset"
mkdir "$iconset"
for size in 16 32 128 256 512; do
  rsvg-convert -w "$size" "$src" -o "$iconset/icon_${size}x${size}.png"
  rsvg-convert -w $((size * 2)) "$src" -o "$iconset/icon_${size}x${size}@2x.png"
done
iconutil -c icns "$iconset" -o "$res/Document.icns"

echo "icons written to $res"
