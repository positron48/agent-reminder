use std::{fs::File, io::BufReader, path::PathBuf};

use rodio::{Decoder, OutputStream, Sink};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::models::Timer;

pub fn play_ding(app: &AppHandle) {
    let sound_path = resolve_ding_path(app);
    if !sound_path.exists() {
        return;
    }

    std::thread::spawn(move || {
        let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
            return;
        };
        let Ok(file) = File::open(sound_path) else {
            return;
        };
        let Ok(source) = Decoder::new(BufReader::new(file)) else {
            return;
        };
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            sink.append(source);
            sink.sleep_until_end();
        }
    });
}

pub fn notify_timer_complete(app: &AppHandle, timer: &Timer) {
    let title = format!("{} is available", timer.agent_label);
    let body = timer
        .comment
        .clone()
        .unwrap_or_else(|| "Agent limit has reset".to_string());

    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
}

fn resolve_ding_path(app: &AppHandle) -> PathBuf {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("ding.wav"))
        .unwrap_or_else(|| PathBuf::from("assets/ding.wav"))
}
