use chrono::Utc;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use crate::{
    models::{
        AddTimerPayload, AgentType, AppSettings, RestartTimerPayload, Timer, TimerStatus,
        TraySummary, compute_tray_summary,
    },
    store::{save_settings, save_timers, AppState},
    timer_engine::refresh_tray,
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn resolve_timer_schedule(
    days: u32,
    hours: u32,
    minutes: u32,
    ends_at: Option<i64>,
) -> Result<(u64, i64, i64), String> {
    let started = now_ms();
    if let Some(end) = ends_at {
        if end <= started {
            return Err("Reset time must be in the future".into());
        }
        let duration_ms = (end - started) as u64;
        if duration_ms == 0 {
            return Err("Duration must be greater than 0".into());
        }
        Ok((duration_ms, started, end))
    } else if days == 0 && hours == 0 && minutes == 0 {
        Err("Duration must be greater than 0".into())
    } else {
        let duration_ms = ((days as u64 * 24 * 60 * 60)
            + (hours as u64 * 60 * 60)
            + (minutes as u64 * 60))
            * 1000;
        Ok((duration_ms, started, started + duration_ms as i64))
    }
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
    let agent_type = AgentType::from_str(&payload.agent_type);
    let agent_label = payload
        .agent_label
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| agent_type.default_label().to_string());

    let (duration_ms, started_at, ends_at) = resolve_timer_schedule(
        payload.days,
        payload.hours,
        payload.minutes,
        payload.ends_at,
    )?;
    let timer = Timer {
        id: Uuid::new_v4().to_string(),
        agent_type,
        agent_label,
        duration_ms,
        started_at,
        ends_at,
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
pub fn restart_timer(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: RestartTimerPayload,
) -> Result<Timer, String> {
    let (duration_ms, started_at, ends_at) = resolve_timer_schedule(
        payload.days,
        payload.hours,
        payload.minutes,
        payload.ends_at,
    )?;

    let updated = {
        let mut timers = state.timers.lock().unwrap();
        let timer = timers
            .iter_mut()
            .find(|t| t.id == payload.id)
            .ok_or_else(|| "Timer not found".to_string())?;

        timer.duration_ms = duration_ms;
        timer.started_at = started_at;
        timer.ends_at = ends_at;
        timer.status = TimerStatus::Active;
        timer.notified = false;
        if let Some(comment) = payload.comment.filter(|s| !s.trim().is_empty()) {
            timer.comment = Some(comment);
        }
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
