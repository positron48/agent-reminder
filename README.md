# Agent Reminder

Компактное tray/menu bar приложение для macOS, Windows и Linux. Помогает отслеживать обратный отсчёт лимитов ИИ-агентов (Claude, Codex, Cursor и других).

## Возможности

- Иконка в системном трее / menu bar со статусом доступности агентов
- Popup-панель по клику: ближайший свободный агент, список таймеров, быстрое добавление
- Таймеры с выбором типа агента, длительностью (часы/минуты) и опциональным комментарием
- Приятный звуковой сигнал и системное уведомление при завершении таймера
- Сохранение таймеров между перезапусками
- Сборка релизов через GitHub Actions: `.dmg`, Windows installer, Linux AppImage/deb/rpm

## Локальная разработка

### Требования

- [Node.js](https://nodejs.org/) LTS
- [Rust](https://www.rust-lang.org/tools/install)
- macOS: Xcode Command Line Tools
- Linux: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
- Windows: WebView2, Visual Studio Build Tools

### Запуск

```bash
npm install
npm run tauri:dev
```

### Проверки

```bash
npm run check
npm run build
npm run tauri:build
```

## Использование

1. Запустите приложение — оно появится в menu bar / системном трее.
2. Кликните по иконке, чтобы открыть панель.
3. Добавьте таймер через быстрые кнопки `+ Claude`, `+ Codex`, `+ Cursor` или форму «Новый таймер».
4. Когда лимит сбросится, прозвучит сигнал и появится уведомление.
5. Завершённые агенты отображаются в секции «Свободны».

## Статусы иконки

| Статус | Значение |
|--------|----------|
| Доступен | Есть свободные агенты или нет активных лимитов |
| Ожидание | Все отслеживаемые агенты ещё ждут сброса |
| Скоро | Ближайший таймер заканчивается менее чем через 5 минут |

## Релизы

Создайте git tag вида `v0.1.0` и запушьте его — GitHub Actions соберёт артефакты для macOS, Windows и Linux.

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

Подробнее: [Tauri — macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/)

## Стек

- Tauri 2 + Rust
- React + TypeScript + Vite
- rodio (звук), tauri-plugin-notification (уведомления)

## Лицензия

MIT
