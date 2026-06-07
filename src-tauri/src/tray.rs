use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager,
};

use crate::models::TrayStatusKind;

const FALLBACK_ICON: Image<'static> = tauri::include_image!("icons/32x32.png");

pub fn build_tray(app: &App) -> tauri::Result<()> {
    let icon = load_tray_icon("available")?;
    let quit = MenuItem::with_id(app, "quit", "Quit Agent Reminder", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit])?;

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Agent Reminder")
        .on_menu_event(|app, event| {
            if event.id() == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_panel(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

pub fn toggle_panel(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
        return;
    }

    position_panel(&window);
    let _ = window.show();
    let _ = window.set_focus();
}

fn position_panel(window: &tauri::WebviewWindow) {
    if let Ok(monitor) = window.current_monitor() {
        if let Some(monitor) = monitor {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            let win_size = window.outer_size().unwrap_or(tauri::PhysicalSize {
                width: 360,
                height: 520,
            });
            let x = (size.width as f64 - win_size.width as f64 - 16.0 * scale) as i32;
            let y = (24.0 * scale) as i32;
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: x.max(0),
                y,
            }));
        }
    }
}

pub fn update_tray_icon(app: &AppHandle, summary: &crate::models::TraySummary) {
    let Some(tray) = app.tray_by_id("main-tray") else {
        return;
    };

    let icon_name = match summary.status {
        TrayStatusKind::Idle | TrayStatusKind::Available => "available",
        TrayStatusKind::Waiting => "waiting",
        TrayStatusKind::Soon => "soon",
    };

    if let Ok(icon) = load_tray_icon(icon_name) {
        let _ = tray.set_icon(Some(icon));
    }

    let tooltip = match summary.status {
        TrayStatusKind::Idle => "Agent Reminder — нет активных лимитов".to_string(),
        TrayStatusKind::Available if summary.waiting_count == 0 => {
            format!(
                "Agent Reminder — {} агент(ов) свободно",
                summary.available_count
            )
        }
        TrayStatusKind::Available => format!(
            "Agent Reminder — {} своб., {} ждут",
            summary.available_count, summary.waiting_count
        ),
        TrayStatusKind::Waiting => {
            if let (Some(ms), Some(label)) = (summary.nearest_ms, &summary.nearest_label) {
                format!(
                    "Agent Reminder — ближайший: {} через {}",
                    label,
                    format_duration(ms)
                )
            } else {
                "Agent Reminder — все агенты ждут".to_string()
            }
        }
        TrayStatusKind::Soon => {
            if let (Some(ms), Some(label)) = (summary.nearest_ms, &summary.nearest_label) {
                format!("Agent Reminder — {} через {}", label, format_duration(ms))
            } else {
                "Agent Reminder — скоро освободится".to_string()
            }
        }
    };

    let _ = tray.set_tooltip(Some(&tooltip));
}

fn load_tray_icon(name: &str) -> tauri::Result<Image<'static>> {
    let candidates = [
        std::path::PathBuf::from(format!("src-tauri/icons/tray/{name}.png")),
        std::path::PathBuf::from(format!("icons/tray/{name}.png")),
    ];

    for path in candidates {
        if path.exists() {
            if let Ok(image) = Image::from_path(path) {
                return Ok(image);
            }
        }
    }

    Ok(FALLBACK_ICON.clone())
}

fn format_duration(ms: i64) -> String {
    let total_secs = (ms / 1000).max(0);
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}
