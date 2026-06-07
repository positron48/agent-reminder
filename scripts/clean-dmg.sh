#!/usr/bin/env bash
# Post-process macOS DMGs: apply Finder layout (background, icon positions) and
# strip service files. Re-applies layout when bundle_dmg AppleScript failed or an
# older clean-dmg run left a broken .DS_Store without background metadata.
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
TEMPLATE="$ROOT/scripts/create-dmg-template.applescript"
BACKGROUND="$ROOT/src-tauri/dmg-assets/background.png"

# Must match src-tauri/tauri.conf.json bundle.macOS.dmg
WINW=660
WINH=400
WINX=10
WINY=60
APP_X=170
APP_Y=100
APPS_X=470
APPS_Y=100

find_dmgs() {
  find "$ROOT/src-tauri/target" -path '*/release/bundle/dmg/*.dmg' ! -name 'rw.*' 2>/dev/null
}

collect_dmgs() {
  if [ "$#" -gt 0 ]; then
    printf '%s\n' "$@"
    return
  fi

  find_dmgs
}

ds_store_has_background() {
  local mount="$1"
  [ -f "$mount/.DS_Store" ] && strings "$mount/.DS_Store" 2>/dev/null | grep -q 'icvpblob'
}

apply_finder_layout() {
  local vol="$1"
  local app="$2"

  if [ ! -f "$TEMPLATE" ]; then
    echo "Missing AppleScript template: $TEMPLATE" >&2
    return 1
  fi

  local background_clause='set background picture of opts to file ".background:background.png"'
  local reposition_clause='try
				set position of item ".background" to {-400, -400}
			end try'
  local position_clause="set position of item \"$app\" to {$APP_X, $APP_Y}
			"
  local application_clause="set position of item \"Applications\" to {$APPS_X, $APPS_Y}
			"

  local applescript_file
  applescript_file=$(mktemp -t createdmg.tmp.XXXXXXXXXX)
  cat "$TEMPLATE" \
    | sed -e "s/WINX/$WINX/g" -e "s/WINY/$WINY/g" -e "s/WINW/$WINW/g" \
          -e "s/WINH/$WINH/g" -e "s/ICON_SIZE/128/g" -e "s/TEXT_SIZE/16/g" \
          -e "s/BACKGROUND_CLAUSE/$background_clause/g" \
    | perl -pe "s/REPOSITION_HIDDEN_FILES_CLAUSE/$reposition_clause/g" \
    | perl -pe "s/POSITION_CLAUSE/$position_clause/g" \
    | perl -pe "s/APPLICATION_CLAUSE/$application_clause/g" \
    | perl -pe "s/HIDING_CLAUSE//g" \
    | perl -pe "s/QL_CLAUSE//g" \
    > "$applescript_file"

  sleep 2
  if /usr/bin/osascript "$applescript_file" "$vol"; then
    rm -f "$applescript_file"
    return 0
  fi

  rm -f "$applescript_file"
  return 1
}

DMGS=()
while IFS= read -r line; do
  [ -n "$line" ] && DMGS+=("$line")
done < <(collect_dmgs "$@")

if [ ${#DMGS[@]} -eq 0 ]; then
  echo "No DMG files found." >&2
  echo "" >&2
  echo "Build first:" >&2
  echo "  npm run tauri build -- --target aarch64-apple-darwin" >&2
  echo "" >&2
  echo "Then run without arguments:" >&2
  echo "  bash scripts/clean-dmg.sh" >&2
  exit 1
fi

if [ ! -f "$BACKGROUND" ]; then
  echo "Missing DMG background: $BACKGROUND" >&2
  echo "Run: bash scripts/render-dmg-background.sh" >&2
  exit 1
fi

for DMG in "${DMGS[@]}"; do
  if [ ! -f "$DMG" ]; then
    echo "Skipping missing file: $DMG" >&2
    continue
  fi

  echo "Cleaning $(basename "$DMG")..."
  WORK=$(mktemp -d)
  RW="$WORK/rw.dmg"

  hdiutil convert "$DMG" -format UDRW -o "$RW" -quiet
  MOUNT=$(hdiutil attach -readwrite -noverify -nobrowse "$RW" | grep -Eo '/Volumes/.*' | head -1)
  if [ -z "$MOUNT" ] || [ ! -d "$MOUNT" ]; then
    echo "Failed to mount $DMG for post-processing." >&2
    rm -rf "$WORK"
    continue
  fi

  VOL=$(basename "$MOUNT")
  APP=$(find "$MOUNT" -maxdepth 1 -name '*.app' -print | head -1 | xargs -I{} basename "{}")

  if [ -z "$APP" ]; then
    echo "No .app bundle found in $DMG, skipping." >&2
    hdiutil detach "$MOUNT" -quiet || true
    rm -rf "$WORK"
    continue
  fi

  mkdir -p "$MOUNT/.background"
  cp "$BACKGROUND" "$MOUNT/.background/background.png"

  if [ -f "$MOUNT/.VolumeIcon.icns" ]; then
    rm "$MOUNT/.VolumeIcon.icns"
    SetFile -a c "$MOUNT" 2>/dev/null || true
    echo "Removed .VolumeIcon.icns"
  fi

  if [ -d "$MOUNT/.background" ]; then
    chflags hidden "$MOUNT/.background" 2>/dev/null || true
  fi

  if ds_store_has_background "$MOUNT"; then
    echo "Finder layout already present, skipping AppleScript."
  else
    echo "Applying Finder layout (background + icon positions)..."
    rm -f "$MOUNT/.DS_Store"
    if ! apply_finder_layout "$VOL" "$APP"; then
      echo "Warning: Finder layout AppleScript failed; DMG may lack background image." >&2
    else
      sync
      sleep 1
      if ds_store_has_background "$MOUNT"; then
        echo "Finder layout applied."
      else
        echo "Warning: .DS_Store still missing background metadata." >&2
      fi
    fi
  fi

  hdiutil detach "$MOUNT" -quiet

  hdiutil convert "$RW" -format UDZO -imagekey zlib-level=9 -o "$WORK/out.dmg" -quiet
  mv "$WORK/out.dmg" "$DMG"
  rm -rf "$WORK"
  echo "Done."
done
