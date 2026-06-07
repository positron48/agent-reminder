# Agent Reminder

[English](README.md) · **Русский**

Компактное приложение для **menu bar / системного трея** на **macOS**, **Windows** и **Linux**. Отслеживает обратный отсчёт лимитов ИИ-агентов — Claude, Codex, Cursor или любой другой сервис — и показывает, когда можно снова отправить запрос.

![Agent Reminder](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![Tauri 2](https://img.shields.io/badge/Tauri-2-purple)
![License MIT](https://img.shields.io/badge/license-MIT-green)

---

## Зачем это нужно

У coding-ассистентов часто есть лимиты с неочевидным временем сброса. Agent Reminder живёт в трее, показывает живой countdown и уведомляет, когда лимит снят — без вкладки в браузере и таблиц.

**Типичный сценарий:**

1. Вы упёрлись в лимит Claude / Codex / Cursor.
2. Добавляете таймер на ожидаемое окно сброса (часы, минуты или дни).
3. Иконка в трее показывает статус с первого взгляда.
4. По окончании таймера — звук и системное уведомление.

---

## Возможности

| Функция | Описание |
|--------|----------|
| **Иконка в трее / menu bar** | Статус: нет таймеров, ожидание, доступен, скоро освободится |
| **Popup-панель** | Клик по иконке — ближайший агент, список таймеров, быстрое добавление |
| **Гибкие таймеры** | Дни, часы, минуты; опциональный комментарий |
| **Сохранение** | Таймеры переживают перезапуск приложения |
| **Оповещения** | Звук ding и нативное уведомление (можно выключить) |
| **Fullscreen (macOS)** | Панель показывается поверх fullscreen-приложений через NSPanel |
| **Сборка под все ОС** | GitHub Actions: `.dmg`, Windows installer, Linux AppImage/deb/rpm |

---

## Статусы иконки в трее

| Иконка | Значение |
|--------|----------|
| Пунктирный круг | Нет таймеров |
| Песочные часы | Все агенты ещё ждут сброса |
| Галочка / число | Есть свободные агенты (бейдж — количество) |
| Восклицание | Ближайший таймер заканчивается менее чем через 5 минут |

Иконки из набора [Lucide](https://lucide.dev) (лицензия ISC). См. [`assets/tray-icons/LICENSE.txt`](assets/tray-icons/LICENSE.txt).

---

## Быстрый старт

### Требования

- [Node.js](https://nodejs.org/) LTS
- [Rust](https://www.rust-lang.org/tools/install)
- **macOS:** Xcode Command Line Tools
- **Linux:** `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
- **Windows:** WebView2, Visual Studio Build Tools

### Разработка

```bash
npm install
npm run tauri:dev
```

Приложение появится в menu bar / трее (на macOS без иконки в Dock). Клик по иконке открывает панель.

### Проверки и локальная сборка

```bash
npm run check       # TypeScript + Rust
npm run build       # Сборка фронтенда
npm run tauri:build # Нативное приложение
```

---

## Использование

1. Запустите приложение — оно появится в menu bar / трее.
2. Кликните по иконке, чтобы открыть панель.
3. Добавьте таймер через быстрые кнопки (`+ Claude`, `+ Codex`, `+ Cursor`) или **New timer**.
4. Когда лимит сбросится — звук (если включён) и уведомление.
5. Свободные агенты — в секции **Available**; очистите, когда не нужны.

Правый клик по иконке (или пункт меню) — выход из приложения.

---

## Релизы

Запушьте тег версии — GitHub Actions соберёт артефакты:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Workflow: [`.github/workflows/release.yml`](.github/workflows/release.yml)

### Подпись macOS (опционально)

Для распространения без предупреждений Gatekeeper добавьте в GitHub Secrets:

- `APPLE_CERTIFICATE`
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_SIGNING_IDENTITY`
- `APPLE_ID`
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`

Документация: [Tauri — macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/)

---

## Стек

- **Оболочка:** Tauri 2 + Rust
- **UI:** React + TypeScript + Vite
- **Звук:** rodio
- **Уведомления:** tauri-plugin-notification
- **macOS panel:** [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel)

---

## Лицензия

MIT
