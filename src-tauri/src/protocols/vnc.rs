// protocols/vnc.rs - Obsługa połączeń VNC (Virtual Network Computing)
use std::process::Command;

// Łączy się z hostem przez VNC używając UltraVNC viewer (Windows)
pub fn connect(hostname: &str, port: u16) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // UltraVNC używa formatu host:port lub host::port dla VNC display
        let connection_string = if port == 5900 {
            hostname.to_string()
        } else {
            format!("{}:{}", hostname, port)
        };

        // Uruchom vncviewer.exe
        Command::new("vncviewer.exe")
            .arg(connection_string)
            .spawn()
            .map_err(|e| format!("Nie można uruchomić vncviewer.exe: {}", e))?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Na macOS/Linux - tylko zwróć komunikat (mock dla developmentu)
        Err(format!(
            "VNC nie jest dostępny na tym systemie. Docelowo: vncviewer.exe {}:{}",
            hostname, port
        ))
    }
}

