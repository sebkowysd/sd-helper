// lib.rs - Główny plik biblioteki Tauri
mod commands;
mod protocols;
mod storage;

use commands::AppState;
use std::sync::Mutex;
use storage::HostsData;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            hosts: Mutex::new(HostsData::default()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_hosts,
            commands::save_hosts,
            commands::add_host,
            commands::update_host,
            commands::delete_host,
            commands::connect_rdp,
            commands::connect_vnc,
            commands::connect_ssh,
            commands::connect_to_host,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
