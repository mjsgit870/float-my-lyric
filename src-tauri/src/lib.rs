// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use enigo::{Enigo, Key, KeyboardControllable};

#[tauri::command]
fn play_pause_media() {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::MediaPlayPause);
}

#[tauri::command]
fn next_media() {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::MediaNextTrack);
}

#[tauri::command]
fn prev_media() {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::MediaPrevTrack);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_pause_media,
            next_media,
            prev_media
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
