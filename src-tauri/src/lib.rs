mod commands;
mod icon_render;
mod models;
mod notifier;
mod panel_window;
mod store;
mod timer_engine;
mod tray;

use tauri::Manager;

use commands::{
    add_timer, clear_completed, complete_timer, get_settings, get_timers, get_tray_summary,
    hide_panel, remove_timer, restart_timer, set_sound_enabled,
};
use store::{load_state, AppState};
use timer_engine::{refresh_tray, start_timer_engine};
use tray::build_tray;

#[cfg(target_os = "macos")]
use tauri_nspanel::tauri_panel;

#[cfg(target_os = "macos")]
tauri_panel! {
    panel!(MainPanel {
        config: {
            can_become_key_window: true,
            becomes_key_only_if_needed: true,
            is_floating_panel: true,
            hides_on_deactivate: false,
        }
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_notification::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(AppState::new())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let state = app.state::<AppState>();
            load_state(&app.handle(), &state)?;

            panel_window::setup(app)?;

            build_tray(app)?;

            refresh_tray(&app.handle(), &state);

            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        if panel_window::should_hide_on_blur() {
                            tray::hide_panel(&app_handle);
                        }
                    }
                });
            }

            start_timer_engine(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_timers,
            get_tray_summary,
            get_settings,
            set_sound_enabled,
            add_timer,
            remove_timer,
            complete_timer,
            restart_timer,
            clear_completed,
            hide_panel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
