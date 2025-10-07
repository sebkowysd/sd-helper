// protocols/ssh.rs - Obsługa połączeń SSH (Secure Shell)
use std::process::Command;

// Łączy się z hostem przez SSH
pub fn connect(hostname: &str, port: u16, username: Option<&str>) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Na Windows używamy PowerShell z ssh
        let connection_string = if let Some(user) = username {
            format!("{}@{}", user, hostname)
        } else {
            hostname.to_string()
        };

        let ssh_command = format!("ssh {} -p {}", connection_string, port);

        // Uruchom PowerShell z komendą ssh
        Command::new("powershell.exe")
            .args(&["-NoExit", "-Command", &ssh_command])
            .spawn()
            .map_err(|e| format!("Nie można uruchomić SSH: {}", e))?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Na macOS/Linux używamy terminala z ssh
        let connection_string = if let Some(user) = username {
            format!("{}@{}", user, hostname)
        } else {
            hostname.to_string()
        };

        // Na macOS otwórz Terminal.app z komendą ssh
        Command::new("open")
            .args(&[
                "-a",
                "Terminal.app",
                &format!("ssh {} -p {}", connection_string, port),
            ])
            .spawn()
            .map_err(|e| format!("Nie można uruchomić Terminal: {}", e))?;

        Ok(())
    }
}

