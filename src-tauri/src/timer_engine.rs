use std::time::Duration;

use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    models::{compute_tray_summary, TimerStatus},
    notifier::{notify_timer_complete, play_ding},
    store::AppState,
    tray::update_tray_icon,
};

pub fn refresh_tray(app: &AppHandle, state: &AppState) {
    let timers = state.timers.lock().unwrap();
    let summary = compute_tray_summary(&timers, Utc::now().timestamp_millis());
    drop(timers);
    update_tray_icon(app, &summary);
}

pub fn start_timer_engine(app: AppHandle) {
    std::thread::spawn(move || loop {
        if let Some(state) = app.try_state::<AppState>() {
            tick(&app, &state);
        }
        std::thread::sleep(Duration::from_secs(1));
    });
}

fn tick(app: &AppHandle, state: &AppState) {
    let now = Utc::now().timestamp_millis();
    let mut changed = false;
    let mut completed: Vec<(String, String)> = Vec::new();

    {
        let mut timers = state.timers.lock().unwrap();
        for timer in timers.iter_mut() {
            if timer.status == TimerStatus::Active && timer.ends_at <= now && !timer.notified {
                timer.status = TimerStatus::Completed;
                timer.notified = true;
                changed = true;
                completed.push((timer.id.clone(), timer.agent_label.clone()));
            }
        }

        if changed {
            let _ = crate::store::save_timers(app, &timers);
        }
    }

    for (id, label) in completed {
        let sound_enabled = state.settings.lock().unwrap().sound_enabled;
        if sound_enabled {
            play_ding(app);
        }

        if let Some(timer) = state
            .timers
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.id == id)
            .cloned()
        {
            notify_timer_complete(app, &timer);
            let _ = app.emit(
                "timer-completed",
                serde_json::json!({ "id": id, "label": label }),
            );
        }
    }

    refresh_tray(app, state);
    if changed {
        let _ = app.emit("timers-updated", ());
    }
}
