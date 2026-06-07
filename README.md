# Agent Reminder

**English** · [Русский](README.ru.md)

A lightweight tray / menu bar app for **macOS**, **Windows**, and **Linux**. It tracks countdown timers for AI agent rate limits — Claude, Codex, Cursor, or any custom service — so you know exactly when you can send the next request.

![Agent Reminder](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![Tauri 2](https://img.shields.io/badge/Tauri-2-purple)
![License MIT](https://img.shields.io/badge/license-MIT-green)

---

## Why this exists

AI coding assistants often hit usage limits with opaque reset times. Agent Reminder lives in the system tray, shows a live countdown, and notifies you the moment a limit clears — without keeping a browser tab or spreadsheet open.

**Typical workflow:**

1. You hit a rate limit on Claude / Codex / Cursor.
2. You add a timer for the expected reset window (hours, minutes, or days).
3. The tray icon reflects status at a glance.
4. When the timer ends, you get a sound + system notification.

---

## Features

| Feature | Description |
|--------|-------------|
| **Menu bar / tray icon** | Status at a glance: idle, waiting, available, or ending soon |
| **Popup panel** | Click the icon to open a compact panel — nearest agent, timer list, quick add |
| **Flexible timers** | Days, hours, minutes; optional comment per timer |
| **Persistence** | Timers survive app restarts |
| **Alerts** | Optional ding sound + native notification on completion |
| **Fullscreen-friendly (macOS)** | Panel appears over fullscreen apps via NSPanel |
| **Cross-platform builds** | GitHub Actions produces `.dmg`, Windows installer, Linux AppImage/deb/rpm |

---

## Tray icon states

| Icon | Meaning |
|------|---------|
| Dashed circle | No timers — nothing to track |
| Hourglass | All tracked agents are still waiting |
| Check / number | One or more agents are available (badge shows count) |
| Alert | Nearest timer ends in less than 5 minutes |

Tray icons are based on [Lucide](https://lucide.dev) (ISC license). See [`assets/tray-icons/LICENSE.txt`](assets/tray-icons/LICENSE.txt).

---

## Getting started

### Requirements

- [Node.js](https://nodejs.org/) LTS
- [Rust](https://www.rust-lang.org/tools/install)
- **macOS:** Xcode Command Line Tools
- **Linux:** `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
- **Windows:** WebView2, Visual Studio Build Tools

### Development

```bash
npm install
npm run tauri:dev
```

The app runs in the menu bar / system tray (no dock icon on macOS). Click the icon to open the panel.

### Checks & local build

```bash
npm run check      # TypeScript + Rust checks
npm run build      # Frontend build
npm run tauri:build # Native app bundle
```

---

## Usage

1. Launch the app — it appears in the menu bar or system tray.
2. Click the tray icon to open the panel.
3. Add a timer via quick buttons (`+ Claude`, `+ Codex`, `+ Cursor`) or **New timer**.
4. When a limit clears, you'll hear a ding (if enabled) and get a notification.
5. Completed agents appear under **Available**; clear them when you're done.

Right-click the tray icon (or use the menu) to quit.

---

## Releases

Push a version tag to trigger GitHub Actions:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Workflow: [`.github/workflows/release.yml`](.github/workflows/release.yml)

### macOS code signing (optional)

For Gatekeeper-friendly distribution, add these GitHub Secrets:

- `APPLE_CERTIFICATE`
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_SIGNING_IDENTITY`
- `APPLE_ID`
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`

Docs: [Tauri — macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/)

---

## Tech stack

- **Shell:** Tauri 2 + Rust
- **UI:** React + TypeScript + Vite
- **Audio:** rodio
- **Notifications:** tauri-plugin-notification
- **macOS panel:** [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel)

---

## License

MIT
