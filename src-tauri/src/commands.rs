// commands.rs - Tauri commands wywoływane z frontendu
use crate::protocols;
use crate::storage::{self, Host, HostsData};
use tauri::State;
use std::sync::Mutex;

// Struktura przechowująca stan aplikacji
pub struct AppState {
    pub hosts: Mutex<HostsData>,
}

// Wczytuje wszystkie hosty z pliku JSON
#[tauri::command]
pub fn load_hosts(app: tauri::AppHandle, state: State<AppState>) -> Result<Vec<Host>, String> {
    let hosts_data = storage::load_hosts(&app)?;
    
    // Zaktualizuj state
    let mut state_hosts = state.hosts.lock().unwrap();
    *state_hosts = hosts_data.clone();
    
    Ok(hosts_data.hosts)
}

// Zapisuje hosty do pliku JSON
#[tauri::command]
pub fn save_hosts(
    app: tauri::AppHandle,
    state: State<AppState>,
    hosts: Vec<Host>,
) -> Result<(), String> {
    let hosts_data = HostsData { hosts: hosts.clone() };
    storage::save_hosts(&app, &hosts_data)?;
    
    // Zaktualizuj state
    let mut state_hosts = state.hosts.lock().unwrap();
    *state_hosts = hosts_data;
    
    Ok(())
}

// Dodaje nowy host
#[tauri::command]
pub fn add_host(
    app: tauri::AppHandle,
    state: State<AppState>,
    host: Host,
) -> Result<Vec<Host>, String> {
    let mut state_hosts = state.hosts.lock().unwrap();
    state_hosts.hosts.push(host);
    
    storage::save_hosts(&app, &state_hosts)?;
    
    Ok(state_hosts.hosts.clone())
}

// Aktualizuje istniejący host
#[tauri::command]
pub fn update_host(
    app: tauri::AppHandle,
    state: State<AppState>,
    host: Host,
) -> Result<Vec<Host>, String> {
    let mut state_hosts = state.hosts.lock().unwrap();
    
    // Znajdź i zaktualizuj host
    if let Some(existing) = state_hosts.hosts.iter_mut().find(|h| h.id == host.id) {
        *existing = host;
    } else {
        return Err("Host nie znaleziony".to_string());
    }
    
    storage::save_hosts(&app, &state_hosts)?;
    
    Ok(state_hosts.hosts.clone())
}

// Usuwa host
#[tauri::command]
pub fn delete_host(
    app: tauri::AppHandle,
    state: State<AppState>,
    host_id: String,
) -> Result<Vec<Host>, String> {
    let mut state_hosts = state.hosts.lock().unwrap();
    state_hosts.hosts.retain(|h| h.id != host_id);
    
    storage::save_hosts(&app, &state_hosts)?;
    
    Ok(state_hosts.hosts.clone())
}

// Łączy się z hostem RDP
#[tauri::command]
pub fn connect_rdp(hostname: String, port: u16, username: Option<String>) -> Result<(), String> {
    protocols::rdp::connect(&hostname, port, username.as_deref())
}

// Łączy się z hostem VNC
#[tauri::command]
pub fn connect_vnc(hostname: String, port: u16) -> Result<(), String> {
    protocols::vnc::connect(&hostname, port)
}

// Łączy się z hostem SSH
#[tauri::command]
pub fn connect_ssh(hostname: String, port: u16, username: Option<String>) -> Result<(), String> {
    protocols::ssh::connect(&hostname, port, username.as_deref())
}

// Uniwersalna funkcja połączenia - wybiera odpowiedni protokół
#[tauri::command]
pub fn connect_to_host(state: State<AppState>, host_id: String) -> Result<(), String> {
    let state_hosts = state.hosts.lock().unwrap();
    
    let host = state_hosts
        .hosts
        .iter()
        .find(|h| h.id == host_id)
        .ok_or("Host nie znaleziony")?;
    
    match host.protocol.as_str() {
        "rdp" => protocols::rdp::connect(&host.hostname, host.port, host.username.as_deref()),
        "vnc" => protocols::vnc::connect(&host.hostname, host.port),
        "ssh" => protocols::ssh::connect(&host.hostname, host.port, host.username.as_deref()),
        _ => Err(format!("Nieznany protokół: {}", host.protocol)),
    }
}

