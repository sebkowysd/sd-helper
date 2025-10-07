// protocols/rdp.rs - Obsługa połączeń RDP (Remote Desktop Protocol)
use std::process::Command;

// Łączy się z hostem przez RDP używając mstsc.exe (Windows)
pub fn connect(hostname: &str, port: u16, username: Option<&str>) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut args = vec![format!("/v:{}:{}", hostname, port)];
        
        // Dodaj username jeśli podano
        if let Some(user) = username {
            args.push(format!("/u:{}", user));
        }

        // Uruchom mstsc.exe
        Command::new("mstsc.exe")
            .args(&args)
            .spawn()
            .map_err(|e| format!("Nie można uruchomić mstsc.exe: {}", e))?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Na macOS/Linux - tylko zwróć komunikat (mock dla developmentu)
        Err(format!(
            "RDP nie jest dostępny na tym systemie. Docelowo: mstsc.exe /v:{}:{}",
            hostname, port
        ))
    }
}

