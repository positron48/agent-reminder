use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Rect,
};

use crate::{icon_render, models::TrayStatusKind, panel_window};

static LAST_ICON_KEY: Mutex<Option<icon_render::IconCacheKey>> = Mutex::new(None);
static LAST_TOOLTIP: Mutex<Option<String>> = Mutex::new(None);

pub fn build_tray(app: &App) -> tauri::Result<()> {
    let summary = crate::models::TraySummary {
        available_count: 0,
        waiting_count: 0,
        nearest_ms: None,
        nearest_label: None,
        status: TrayStatusKind::Idle,
    };
    let icon = icon_render::render_tray_icon(&summary);
    let quit = MenuItem::with_id(app, "quit", "Quit Agent Reminder", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("Agent Reminder");

    #[cfg(target_os = "macos")]
    {
        builder = builder.icon_as_template(true);
    }

    builder
        .on_menu_event(|app, event| {
            if event.id() == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                toggle_panel(tray.app_handle(), Some(rect));
            }
        })
        .build(app)?;

    Ok(())
}

pub fn toggle_panel(app: &AppHandle, tray_rect: Option<Rect>) {
    panel_window::toggle(app, tray_rect);
}

pub fn hide_panel(app: &AppHandle) {
    panel_window::hide(app);
}

pub fn update_tray_icon(app: &AppHandle, summary: &crate::models::TraySummary) {
    let Some(tray) = app.tray_by_id("main-tray") else {
        return;
    };

    let icon_key = icon_render::IconCacheKey::from_summary(summary);
    let icon_changed = LAST_ICON_KEY
        .lock()
        .ok()
        .and_then(|mut last| {
            let changed = last.as_ref() != Some(&icon_key);
            if changed {
                *last = Some(icon_key);
            }
            Some(changed)
        })
        .unwrap_or(true);

    if icon_changed {
        let icon = icon_render::render_tray_icon(summary);
        let _ = tray.set_icon(Some(icon));

        #[cfg(target_os = "macos")]
        let _ = tray.set_icon_as_template(true);
    }

    let tooltip = build_tooltip(summary);
    let tooltip_changed = LAST_TOOLTIP
        .lock()
        .ok()
        .and_then(|mut last| {
            let changed = last.as_ref() != Some(&tooltip);
            if changed {
                *last = Some(tooltip.clone());
            }
            Some(changed)
        })
        .unwrap_or(true);

    if tooltip_changed {
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}

fn build_tooltip(summary: &crate::models::TraySummary) -> String {
    match summary.status {
        TrayStatusKind::Idle => "Agent Reminder — no active limits".to_string(),
        TrayStatusKind::Available if summary.waiting_count == 0 => {
            if summary.available_count > 0 {
                format!(
                    "Agent Reminder — {} agent(s) available",
                    summary.available_count
                )
            } else {
                "Agent Reminder — all limits cleared".to_string()
            }
        }
        TrayStatusKind::Available => format!(
            "Agent Reminder — {} free, {} waiting",
            summary.available_count, summary.waiting_count
        ),
        TrayStatusKind::Waiting => {
            if let (Some(ms), Some(label)) = (summary.nearest_ms, &summary.nearest_label) {
                format!(
                    "Agent Reminder — next: {} in {}",
                    label,
                    format_duration(ms)
                )
            } else {
                "Agent Reminder — all agents waiting".to_string()
            }
        }
        TrayStatusKind::Soon => {
            if let (Some(ms), Some(label)) = (summary.nearest_ms, &summary.nearest_label) {
                format!("Agent Reminder — {} in {}", label, format_duration(ms))
            } else {
                "Agent Reminder — available soon".to_string()
            }
        }
    }
}

fn format_duration(ms: i64) -> String {
    let total_secs = (ms / 1000).max(0);
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    if days > 0 {
        return format!("{days}d {hours:02}:{minutes:02}:{seconds:02}");
    }
    if hours > 0 {
        return format!("{hours:02}:{minutes:02}:{seconds:02}");
    }
    format!("{minutes:02}:{seconds:02}")
}
