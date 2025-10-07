# Notatki techniczne SD Helper

## Architektura aplikacji

### Stack technologiczny

**Frontend:**
- Vue 3.5.13 (Composition API)
- Pinia 2.x (state management)
- Vite 6.x (build tool)

**Backend:**
- Rust 2021 Edition
- Tauri 2.8.5
- Serde (serializacja JSON)

**Protokoły:**
- IPC między Vue a Rust (Tauri API)
- JSON dla storage lokalnego

### Struktura IPC

Komunikacja między frontendem a backendem odbywa się przez Tauri commands:

```javascript
// Frontend (Vue)
import { invoke } from '@tauri-apps/api/core'

const hosts = await invoke('load_hosts')
```

```rust
// Backend (Rust)
#[tauri::command]
pub fn load_hosts(app: tauri::AppHandle) -> Result<Vec<Host>, String> {
    // ...
}
```

### State management

**Frontend (Pinia store):**
- Przechowuje aktualną listę hostów w pamięci
- Synchronizuje z backendem przy każdej zmianie
- Cache dla lepszej responsywności UI

**Backend (Rust):**
- AppState z Mutex<HostsData>
- Persystencja do JSON przy każdym zapisie
- Thread-safe dzięki Mutex

## Storage

### Format JSON

```json
{
  "hosts": [
    {
      "id": "unique_id",
      "name": "Host name",
      "hostname": "192.168.1.1",
      "port": 3389,
      "protocol": "rdp",
      "username": "admin",
      "category": "Category",
      "notes": "Some notes"
    }
  ]
}
```

### Lokalizacja pliku

**Windows:**
```
C:\Users\[Username]\AppData\Roaming\sd-helper\hosts.json
```

**macOS (dev):**
```
~/Library/Application Support/sd-helper/hosts.json
```

### Migracja do SQLite (planowana w etapie 3)

Schemat tabeli:
```sql
CREATE TABLE hosts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    username TEXT,
    category TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_protocol ON hosts(protocol);
CREATE INDEX idx_category ON hosts(category);
```

## Protokoły połączeń

### RDP (Windows tylko)

**Command:**
```bash
mstsc.exe /v:hostname:port /u:username
```

**Implementacja:**
```rust
Command::new("mstsc.exe")
    .args(&[format!("/v:{}:{}", hostname, port)])
    .spawn()
```

**Ograniczenia:**
- Brak programatycznego przekazania hasła (bezpieczeństwo Windows)
- Nie działa na macOS/Linux

### VNC

**Command:**
```bash
vncviewer.exe hostname:port
```

**Implementacja:**
```rust
Command::new("vncviewer.exe")
    .arg(format!("{}:{}", hostname, port))
    .spawn()
```

**Wymagania:**
- UltraVNC Viewer zainstalowany
- vncviewer.exe w PATH lub standardowej lokalizacji

**Format portów VNC:**
- Port 5900 = display :0
- Port 5901 = display :1
- itd.

### SSH

**Windows:**
```bash
powershell.exe -NoExit -Command "ssh user@host -p port"
```

**macOS/Linux:**
```bash
open -a Terminal.app "ssh user@host -p port"
```

**Implementacja:**
```rust
#[cfg(target_os = "windows")]
Command::new("powershell.exe")
    .args(&["-NoExit", "-Command", &ssh_command])
    .spawn()
```

**Wymagania:**
- OpenSSH Client (wbudowany w Windows 10+)
- Klucze SSH w `~/.ssh/` (opcjonalnie)

## Cross-compilation (macOS → Windows)

### Setup

```bash
# Dodanie Windows target
rustup target add x86_64-pc-windows-msvc
```

### Build

```bash
# Standard build (dla obecnej platformy)
npm run tauri:build

# Build dla Windows (z macOS)
npm run tauri:build:windows
```

### Ograniczenia

- Nie można testować Windows-specific features na macOS
- RDP/VNC commands są mockowane na macOS (zwracają błąd)
- Może wymagać dodatkowych narzędzi (wine dla testów)

### Bundle targets

W `tauri.conf.json`:
```json
"bundle": {
  "targets": ["msi", "nsis"]
}
```

**MSI:**
- Windows Installer format
- Standardowy dla enterprise
- Wymaga WiX Toolset (instalowany automatycznie)

**NSIS:**
- Nullsoft Scriptable Install System
- Bardziej customizable
- Lżejszy installer

## Bezpieczeństwo

### Aktualne

1. **Lokalne przechowywanie** - dane tylko na komputerze użytkownika
2. **Brak przechowywania haseł** - użytkownik wprowadza przy każdym połączeniu
3. **IPC security** - Tauri sandboxing

### Planowane (etap 5)

1. **Windows Credential Manager integration**
   ```rust
   use windows::Win32::Security::Credentials::*;
   ```

2. **Szyfrowanie bazy lokalnej**
   ```rust
   use aes_gcm::Aes256Gcm;
   ```

3. **Hasło główne aplikacji**
   - Derive key z hasła użytkownika
   - Szyfruj/deszyfruj przy każdym dostępie

4. **Active Directory integration**
   - LDAP authentication
   - Single Sign-On

## Performance

### Obecne

- ✅ Lazy loading komponentów Vue
- ✅ Wirtualizacja listy (dla >100 hostów w przyszłości)
- ✅ Debounced search (300ms)
- ✅ In-memory cache (Pinia store)

### Optimizacje planowane

1. **SQLite z indexami** (etap 3)
   - Szybsze wyszukiwanie
   - Paginacja dla dużych list

2. **Web Workers** (jeśli potrzebne)
   - Import/export w tle
   - Szyfrowanie/deszyfrowanie

3. **Compression**
   - Gzip dla dużych JSON exports

## Testing

### Unit tests (planowane)

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_hosts() {
        // ...
    }
}
```

**Vue:**
```javascript
import { describe, it, expect } from 'vitest'

describe('HostsStore', () => {
  it('loads hosts correctly', async () => {
    // ...
  })
})
```

### E2E tests (planowane)

- Playwright lub Tauri WebDriver
- Test całego flow: dodaj host → połącz → usuń

## Deployment

### Development workflow

```bash
# Development
npm run tauri:dev

# Build
npm run tauri:build:windows

# Deploy
./deploy.sh
```

### CI/CD (planowane)

GitHub Actions:
```yaml
name: Build
on: [push]
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
      - run: npm install
      - run: npm run tauri:build
      - uses: actions/upload-artifact@v2
```

### Release process

1. Bump version w `package.json` i `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit i tag: `git tag v0.1.0`
4. Build: `npm run tauri:build:windows`
5. Upload artifacts do GitHub Releases
6. Notify users (email, Slack)

## Debugging

### Frontend (Vue DevTools)

```bash
npm run tauri:dev
# Otwórz Chrome DevTools (Cmd+Opt+I na macOS)
# Zainstaluj Vue DevTools extension
```

### Backend (Rust)

```rust
// Dodaj println! lub dbg!
println!("Hosts: {:?}", hosts);
dbg!(&hosts);

// Lub użyj env_logger
env_logger::init();
log::debug!("Loading hosts from {}", path.display());
```

Uruchom z logami:
```bash
RUST_LOG=debug npm run tauri:dev
```

### Common issues

1. **"Nie można otworzyć pliku"**
   - Sprawdź uprawnienia
   - Sprawdź czy katalog istnieje

2. **"Command not found"**
   - Sprawdź PATH
   - Sprawdź czy .exe istnieje

3. **"IPC error"**
   - Sprawdź czy command jest zarejestrowany w `invoke_handler!`
   - Sprawdź typy parametrów

## Wersjonowanie

### Semantic Versioning

`MAJOR.MINOR.PATCH`

- **MAJOR** - breaking changes (API compatibility)
- **MINOR** - nowe features (backward compatible)
- **PATCH** - bugfixy

Przykłady:
- `0.1.0` - MVP, initial release
- `0.2.0` - dodanie tunelowania SSH
- `0.2.1` - bugfix w tunelowaniu
- `1.0.0` - pierwsza stabilna wersja produkcyjna

### Update policy

- **Patch releases** - co tydzień (bugfixy)
- **Minor releases** - co 2-4 tygodnie (features)
- **Major releases** - co kilka miesięcy (breaking changes)

## Linki

- Tauri Docs: https://tauri.app/
- Vue 3 Docs: https://vuejs.org/
- Pinia Docs: https://pinia.vuejs.org/
- Rust Book: https://doc.rust-lang.org/book/

---

**Ostatnia aktualizacja:** Październik 2025  
**Autor:** Sebastian Konofal

