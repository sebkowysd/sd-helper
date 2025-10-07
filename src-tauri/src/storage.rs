// storage.rs - Obsługa JSON storage dla hostów
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

// Struktura reprezentująca pojedynczy host
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub protocol: String, // "rdp", "vnc", "ssh"
    pub username: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

// Struktura przechowująca wszystkie hosty
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostsData {
    pub hosts: Vec<Host>,
}

impl Default for HostsData {
    fn default() -> Self {
        HostsData { hosts: Vec::new() }
    }
}

// Pobiera ścieżkę do pliku hosts.json w katalogu app data
pub fn get_hosts_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Nie można uzyskać ścieżki app data: {}", e))?;

    // Upewnij się, że katalog istnieje
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Nie można utworzyć katalogu app data: {}", e))?;

    Ok(app_data_dir.join("hosts.json"))
}

// Wczytuje hosty z pliku JSON
pub fn load_hosts(app: &AppHandle) -> Result<HostsData, String> {
    let file_path = get_hosts_file_path(app)?;

    if !file_path.exists() {
        // Jeśli plik nie istnieje, zwróć pustą listę
        return Ok(HostsData::default());
    }

    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Nie można odczytać pliku hosts.json: {}", e))?;

    let hosts_data: HostsData = serde_json::from_str(&contents)
        .map_err(|e| format!("Błąd parsowania JSON: {}", e))?;

    Ok(hosts_data)
}

// Zapisuje hosty do pliku JSON
pub fn save_hosts(app: &AppHandle, hosts_data: &HostsData) -> Result<(), String> {
    let file_path = get_hosts_file_path(app)?;

    let json = serde_json::to_string_pretty(hosts_data)
        .map_err(|e| format!("Błąd serializacji JSON: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Nie można zapisać pliku hosts.json: {}", e))?;

    Ok(())
}

