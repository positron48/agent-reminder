use std::{
    fs,
    path::PathBuf,
    sync::Mutex,
};

use tauri::{AppHandle, Manager};

use crate::models::{AppSettings, Timer};

pub struct AppState {
    pub timers: Mutex<Vec<Timer>>,
    pub settings: Mutex<AppSettings>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            timers: Mutex::new(Vec::new()),
            settings: Mutex::new(AppSettings::default()),
        }
    }
}

fn data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())
}

fn timers_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(data_dir(app)?.join("timers.json"))
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(data_dir(app)?.join("settings.json"))
}

pub fn load_state(app: &AppHandle, state: &AppState) -> Result<(), String> {
    if let Ok(dir) = data_dir(app) {
        let _ = fs::create_dir_all(dir);
    }

    if let Ok(path) = timers_path(app) {
        if path.exists() {
            let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let timers: Vec<Timer> = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
            *state.timers.lock().unwrap() = timers;
        }
    }

    if let Ok(path) = settings_path(app) {
        if path.exists() {
            let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let settings: AppSettings =
                serde_json::from_str(&raw).map_err(|e| e.to_string())?;
            *state.settings.lock().unwrap() = settings;
        }
    }

    Ok(())
}

pub fn save_timers(app: &AppHandle, timers: &[Timer]) -> Result<(), String> {
    let path = timers_path(app)?;
    let raw = serde_json::to_string_pretty(timers).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| e.to_string())
}

pub fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let raw = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| e.to_string())
}
