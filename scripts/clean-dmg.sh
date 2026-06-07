#!/usr/bin/env bash
# Post-process macOS DMGs: remove visible service files from the final image.
# Finder layout is applied during `bundle_dmg.sh` at build time; on CI we only
# strip .VolumeIcon.icns because AppleScript/Finder automation is unavailable.
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

should_skip_finder_layout() {
  [ -n "${SKIP_FINDER_LAYOUT:-}" ] || [ -n "${GITHUB_ACTIONS:-}" ] || [ -n "${CI:-}" ]
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
  echo '  bash scripts/clean-dmg.sh "src-tauri/target/release/bundle/dmg/Agent Reminder_0.1.4_aarch64.dmg"' >&2
  exit 1
fi

apply_finder_layout() {
  local mount=$1
  local vol=$2
  local app=$3

  open "$mount" >/dev/null 2>&1 || true
  sleep 2

  osascript <<EOF
tell application "Finder"
  tell disk "$vol"
    open
    tell container window
      set current view to icon view
      set toolbar visible to false
      set statusbar visible to false
      set bounds to {200, 120, 860, 520}
      set position of item "$app" to {170, 100}
      set position of item "Applications" to {470, 100}
      repeat with entry in items
        if name of entry starts with "." then
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
}

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
  APP_PATH=$(find "$MOUNT" -maxdepth 1 -name '*.app' -print -quit)
  APP=
  if [ -n "$APP_PATH" ]; then
    APP=$(basename "$APP_PATH")
  fi

  if [ -f "$MOUNT/.VolumeIcon.icns" ]; then
    rm "$MOUNT/.VolumeIcon.icns"
    SetFile -a c "$MOUNT" 2>/dev/null || true
    echo "Removed .VolumeIcon.icns"
  fi

  if [ -d "$MOUNT/.background" ]; then
    chflags hidden "$MOUNT/.background" 2>/dev/null || true
  fi

  if should_skip_finder_layout; then
    echo "Skipping Finder layout (headless/CI environment)."
  elif [ -n "$APP" ]; then
    rm -f "$MOUNT/.DS_Store"
    if apply_finder_layout "$MOUNT" "$VOL" "$APP"; then
      echo "Applied Finder layout."
    else
      echo "Warning: Finder layout failed; keeping existing .DS_Store from bundler." >&2
    fi
  else
    echo "No .app bundle found; skipping Finder layout." >&2
  fi

  hdiutil detach "$MOUNT" -quiet
  hdiutil convert "$RW" -format UDZO -imagekey zlib-level=9 -o "$WORK/out.dmg" -quiet
  mv "$WORK/out.dmg" "$DMG"
  rm -rf "$WORK"
  echo "Done."
done
