use std::sync::Mutex;
use std::time::{Duration, Instant};

use tauri::{Rect, WebviewWindow};

static PANEL_SHOWN_AT: Mutex<Option<Instant>> = Mutex::new(None);

pub fn should_hide_on_blur() -> bool {
    PANEL_SHOWN_AT
        .lock()
        .ok()
        .and_then(|guard| *guard)
        .map(|shown_at| shown_at.elapsed() > Duration::from_millis(400))
        .unwrap_or(true)
}

fn mark_panel_shown() {
    if let Ok(mut shown_at) = PANEL_SHOWN_AT.lock() {
        *shown_at = Some(Instant::now());
    }
}

pub fn position_panel(window: &WebviewWindow, tray_rect: Option<Rect>) {
    let win_size = window.outer_size().unwrap_or(tauri::PhysicalSize {
        width: 360,
        height: 520,
    });

    if let Some(tray_rect) = tray_rect {
        if let Some((x, y, width, height)) = physical_rect(&tray_rect) {
            let panel_x = x + (width as i32 / 2) - (win_size.width as i32 / 2);
            let panel_y = y + height as i32 + 8;
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: panel_x.max(0),
                y: panel_y,
            }));
            return;
        }
    }

    if let Ok(monitor) = window.current_monitor() {
        if let Some(monitor) = monitor {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            let x = (size.width as f64 - win_size.width as f64 - 16.0 * scale) as i32;
            let y = (24.0 * scale) as i32;
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: x.max(0),
                y,
            }));
        }
    }
}

fn physical_rect(rect: &Rect) -> Option<(i32, i32, u32, u32)> {
    let (x, y) = match rect.position {
        tauri::Position::Physical(pos) => (pos.x, pos.y),
        tauri::Position::Logical(pos) => (pos.x as i32, pos.y as i32),
    };
    let (width, height) = match rect.size {
        tauri::Size::Physical(size) => (size.width, size.height),
        tauri::Size::Logical(size) => (size.width as u32, size.height as u32),
    };
    Some((x, y, width, height))
}

fn apply_window_effects(window: &WebviewWindow) -> tauri::Result<()> {
    use tauri::window::{Effect, EffectState, EffectsBuilder};

    window.set_visible_on_all_workspaces(true)?;
    window.set_effects(
        EffectsBuilder::new()
            .effects(vec![Effect::WindowBackground])
            .state(EffectState::Active)
            .radius(14.0)
            .build(),
    )?;
    Ok(())
}

#[cfg(target_os = "macos")]
mod imp {
    use super::{apply_window_effects, mark_panel_shown, position_panel, should_hide_on_blur};
    use crate::MainPanel;
    use tauri::{App, AppHandle, Manager, Rect};
    use tauri_nspanel::{
        CollectionBehavior, ManagerExt, PanelLevel, StyleMask, WebviewWindowExt,
    };

    pub fn setup(app: &App) -> tauri::Result<()> {
        let Some(window) = app.get_webview_window("main") else {
            return Ok(());
        };

        let panel = window.to_panel::<MainPanel>()?;

        panel.set_level(PanelLevel::Floating.value());
        panel.set_style_mask(StyleMask::empty().nonactivating_panel().into());
        panel.set_collection_behavior(
            CollectionBehavior::new()
                .full_screen_auxiliary()
                .can_join_all_spaces()
                .into(),
        );
        panel.set_hides_on_deactivate(false);

        if let Some(window) = panel.to_window() {
            apply_window_effects(&window)?;
        }

        Ok(())
    }

    pub fn toggle(app: &AppHandle, tray_rect: Option<Rect>) {
        let Ok(panel) = app.get_webview_panel("main") else {
            fallback_toggle(app, tray_rect);
            return;
        };

        if panel.is_visible() {
            panel.hide();
            return;
        }

        if let Some(window) = panel.to_window() {
            position_panel(&window, tray_rect);
        }

        mark_panel_shown();
        panel.order_front_regardless();
    }

    pub fn hide(app: &AppHandle) {
        if !should_hide_on_blur() {
            return;
        }

        if let Ok(panel) = app.get_webview_panel("main") {
            panel.hide();
            return;
        }

        fallback_hide(app);
    }

    fn fallback_toggle(app: &AppHandle, tray_rect: Option<Rect>) {
        let Some(window) = app.get_webview_window("main") else {
            return;
        };

        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            return;
        }

        position_panel(&window, tray_rect);
        mark_panel_shown();
        let _ = window.show();
    }

    fn fallback_hide(app: &AppHandle) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    use super::{apply_window_effects, mark_panel_shown, position_panel, should_hide_on_blur};
    use tauri::{App, AppHandle, Manager, Rect, WebviewWindow};

    pub fn setup(app: &App) -> tauri::Result<()> {
        if let Some(window) = app.get_webview_window("main") {
            apply_window_effects(&window)?;
        }
        Ok(())
    }

    pub fn toggle(app: &AppHandle, tray_rect: Option<Rect>) {
        let Some(window) = app.get_webview_window("main") else {
            return;
        };

        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            return;
        }

        position_panel(&window, tray_rect);
        mark_panel_shown();
        let _ = window.show();
        let _ = window.set_focus();
    }

    pub fn hide(app: &AppHandle) {
        if !should_hide_on_blur() {
            return;
        }

        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
}

pub use imp::{hide, setup, toggle};
