// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod manager;
use manager::get_data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn refresh_data(name: &str) -> String {
    let newdata = get_data();
    return format!("{} : {}", name, newdata)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![refresh_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
