#!/usr/bin/env bash
# Post-process macOS DMGs: hide service files and reset Finder layout without horizontal scroll.
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)

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
  echo "" >&2
  echo "Or pass an explicit path (quote paths with spaces):" >&2
  echo '  bash scripts/clean-dmg.sh "src-tauri/target/release/bundle/dmg/Agent Reminder_0.1.3_aarch64.dmg"' >&2
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
  MOUNT=$(hdiutil attach -readwrite -noverify -nobrowse "$RW" | grep -o '/Volumes/.*' | head -1)
  VOL=$(basename "$MOUNT")
  APP=$(find "$MOUNT" -maxdepth 1 -name '*.app' -print | head -1 | xargs -I{} basename "{}")

  if [ -z "$APP" ]; then
    echo "No .app bundle found in $DMG, skipping layout fix."
    hdiutil detach "$MOUNT" -quiet
    rm -rf "$WORK"
    continue
  fi

  if [ -f "$MOUNT/.VolumeIcon.icns" ]; then
    rm "$MOUNT/.VolumeIcon.icns"
    SetFile -a c "$MOUNT" 2>/dev/null || true
  fi

  if [ -d "$MOUNT/.background" ]; then
    chflags hidden "$MOUNT/.background" 2>/dev/null || true
  fi

  rm -f "$MOUNT/.DS_Store"

  osascript <<EOF
tell application "Finder"
  tell disk "$VOL"
    open
    tell container window
      set current view to icon view
      set toolbar visible to false
      set statusbar visible to false
      set bounds to {200, 120, 860, 520}
      set position of item "$APP" to {170, 100}
      set position of item "Applications" to {470, 100}
      repeat with entry in items
        set entryName to name of entry
        if entryName starts with "." then
          set position of entry to {-5000, -5000}
        end if
      end repeat
    end tell
    close
    open
    delay 2
    tell container window
      set statusbar visible to false
      set bounds to {200, 120, 860, 520}
      repeat with entry in items
        if name of entry starts with "." then
          set position of entry to {-5000, -5000}
        end if
      end repeat
    end tell
    close
  end tell
end tell
EOF

  hdiutil detach "$MOUNT" -quiet
  hdiutil convert "$RW" -format UDZO -imagekey zlib-level=9 -o "$WORK/out.dmg" -quiet
  mv "$WORK/out.dmg" "$DMG"
  rm -rf "$WORK"
  echo "Done."
done
