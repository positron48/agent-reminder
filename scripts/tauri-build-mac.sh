#!/usr/bin/env bash
# Wrapper for tauri-action on macOS: build, then clean DMG layout.
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"

if [ "${1:-}" = "build" ]; then
  shift
fi

npm run tauri -- build "$@"
bash scripts/clean-dmg.sh
