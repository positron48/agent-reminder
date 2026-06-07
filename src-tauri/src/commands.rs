use chrono::Utc;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use crate::{
    models::{
        AddTimerPayload, AgentType, AppSettings, Timer, TimerStatus, TraySummary,
        compute_tray_summary,
    },
    store::{save_settings, save_timers, AppState},
    timer_engine::refresh_tray,
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

#[tauri::command]
pub fn get_timers(state: State<'_, AppState>) -> Result<Vec<Timer>, String> {
    Ok(state.timers.lock().unwrap().clone())
}

#[tauri::command]
pub fn get_tray_summary(state: State<'_, AppState>) -> Result<TraySummary, String> {
    let timers = state.timers.lock().unwrap();
    Ok(compute_tray_summary(&timers, now_ms()))
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().unwrap().clone())
}

#[tauri::command]
pub fn set_sound_enabled(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<AppSettings, String> {
    {
        let mut settings = state.settings.lock().unwrap();
        settings.sound_enabled = enabled;
        save_settings(&app, &settings)?;
    }
    Ok(state.settings.lock().unwrap().clone())
}

#[tauri::command]
pub fn add_timer(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: AddTimerPayload,
) -> Result<Timer, String> {
    if payload.days == 0 && payload.hours == 0 && payload.minutes == 0 {
        return Err("Duration must be greater than 0".into());
    }

    let agent_type = AgentType::from_str(&payload.agent_type);
    let agent_label = payload
        .agent_label
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| agent_type.default_label().to_string());

    let duration_ms = ((payload.days as u64 * 24 * 60 * 60)
        + (payload.hours as u64 * 60 * 60)
        + (payload.minutes as u64 * 60))
        * 1000;
    let started = now_ms();
    let timer = Timer {
        id: Uuid::new_v4().to_string(),
        agent_type,
        agent_label,
        duration_ms,
        started_at: started,
        ends_at: started + duration_ms as i64,
        comment: payload.comment.filter(|s| !s.trim().is_empty()),
        status: TimerStatus::Active,
        notified: false,
    };

    {
        let mut timers = state.timers.lock().unwrap();
        timers.push(timer.clone());
        save_timers(&app, &timers)?;
    }

    refresh_tray(&app, &state);
    let _ = app.emit("timers-updated", ());
    Ok(timer)
}

#[tauri::command]
pub fn remove_timer(app: AppHandle, state: State<'_, AppState>, id: String) -> Result<(), String> {
    {
        let mut timers = state.timers.lock().unwrap();
        timers.retain(|t| t.id != id);
        save_timers(&app, &timers)?;
    }
    refresh_tray(&app, &state);
    let _ = app.emit("timers-updated", ());
    Ok(())
}

#[tauri::command]
pub fn complete_timer(app: AppHandle, state: State<'_, AppState>, id: String) -> Result<(), String> {
    {
        let mut timers = state.timers.lock().unwrap();
        if let Some(timer) = timers.iter_mut().find(|t| t.id == id) {
            timer.status = TimerStatus::Completed;
            timer.notified = true;
        }
        save_timers(&app, &timers)?;
    }
    refresh_tray(&app, &state);
    let _ = app.emit("timers-updated", ());
    Ok(())
}

#[tauri::command]
pub fn restart_timer(app: AppHandle, state: State<'_, AppState>, id: String) -> Result<Timer, String> {
    let updated = {
        let mut timers = state.timers.lock().unwrap();
        let timer = timers
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| "Timer not found".to_string())?;

        let started = now_ms();
        timer.started_at = started;
        timer.ends_at = started + timer.duration_ms as i64;
        timer.status = TimerStatus::Active;
        timer.notified = false;
        timer.clone()
    };

    save_timers(&app, &state.timers.lock().unwrap())?;
    refresh_tray(&app, &state);
    let _ = app.emit("timers-updated", ());
    Ok(updated)
}

#[tauri::command]
pub fn clear_completed(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    {
        let mut timers = state.timers.lock().unwrap();
        timers.retain(|t| t.status != TimerStatus::Completed);
        save_timers(&app, &timers)?;
    }
    refresh_tray(&app, &state);
    let _ = app.emit("timers-updated", ());
    Ok(())
}

#[tauri::command]
pub fn hide_panel(app: AppHandle) -> Result<(), String> {
    crate::tray::hide_panel(&app);
    Ok(())
}
