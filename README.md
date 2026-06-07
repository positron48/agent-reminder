# Agent Reminder

**Never miss when your AI agent is ready again.**

A tiny menu bar / tray app that tracks rate-limit countdowns for **Claude**, **Codex**, **Cursor**, and any other agent you use. Set a timer, glance at the icon, get notified when the limit clears.

**English** · [Русский](README.ru.md)

<img src="docs/screen.png" alt="Agent Reminder panel" width="50%" />

[**Download latest release →**](https://github.com/positron48/agent-reminder/releases)

macOS · Windows · Linux · Free & open source (MIT)

> **macOS:** the app is ad-hoc signed (no Apple Developer account). If macOS says the app is *damaged*, run `xattr -cr "/Applications/Agent Reminder.app"` after copying from the DMG, or right-click the app → **Open** the first time.

---

## Why you'll want it

Hit a rate limit, start a timer, go back to work. Agent Reminder sits quietly in the tray and tells you the exact moment you can prompt again — no mental math, no browser tabs, no spreadsheet.

- **Live countdown** — see the next available agent at a glance
- **Smart tray icon** — waiting, almost ready, or available right now
- **Sound + notification** when a limit resets (optional)
- **Stays out of the way** — no dock icon on macOS, opens from the menu bar
- **Works over fullscreen** on macOS — panel pops up even on top of full-screen apps
- **Remembers your timers** across restarts

---

## How it works

1. **Hit a limit** on Claude, Codex, Cursor, or another tool.
2. **Add a timer** — days, hours, minutes, optional note.
3. **Get pinged** when it's time to go again.

Quick-add buttons for popular agents. One click to mark an agent as ready or clear the list.

---

## For developers

```bash
npm install
npm run tauri:dev
```

Releases are built and published automatically on version tags via GitHub Actions. See [`.github/workflows/release.yml`](.github/workflows/release.yml).

---

## License

MIT
