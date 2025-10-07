# 🖥️ SD Helper

**ServiceDesk Helper** - Aplikacja do zarządzania połączeniami RDP / VNC / SSH

## 📋 Opis

SD Helper to aplikacja desktopowa stworzona w Tauri + Vue 3, przeznaczona dla zespołów ServiceDesk. Umożliwia łatwe zarządzanie i łączenie się z serwerami za pomocą różnych protokołów:

- 🔵 **RDP** (Remote Desktop Protocol) - mstsc.exe
- 🟠 **VNC** (Virtual Network Computing) - UltraVNC viewer
- 🟣 **SSH** (Secure Shell) - PowerShell/Terminal

## ✨ Funkcje

### Aktualnie zaimplementowane (MVP - Setup):

- ✅ Zarządzanie listą hostów (CRUD)
- ✅ Łączenie przez RDP/VNC/SSH
- ✅ Przechowywanie danych lokalnie (JSON)
- ✅ Wyszukiwanie i filtrowanie hostów
- ✅ Grupowanie po protokole i kategorii
- ✅ Notatki przy hostach
- ✅ Nowoczesny, prosty UI

### Planowane (zgodnie z roadmapą):

- 🔄 Tunelowanie SSH
- 🔒 Szyfrowane przechowywanie poświadczeń
- 📊 Historia połączeń
- 🎯 Ulubione hosty
- 🔗 Integracje z ticketing
- 📁 Transfer plików
- 🎨 Dark mode

## 🚀 Wymagania

### Development (macOS)

- Node.js 18+
- Rust + Cargo
- npm

### Target (Windows)

- Windows 10/11
- mstsc.exe (wbudowane w Windows)
- UltraVNC viewer (opcjonalnie)
- OpenSSH Client (wbudowane w Windows 10+)

## 📦 Instalacja

### Development

```bash
# Klonowanie repozytorium
git clone <repo-url>
cd sd-helper

# Instalacja zależności
npm install

# Uruchomienie w trybie deweloperskim
npm run tauri:dev
```

### Build dla Windows (na macOS)

```bash
# Build dla Windows
npm run tauri:build:windows

# Lub użyj skryptu deploy
./deploy.sh
```

Pliki instalacyjne znajdziesz w:
- MSI: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/`
- NSIS: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/`

### Transfer na laptop Windows

Skrypt `deploy.sh` oferuje kilka opcji transferu:

1. **SCP** - bezpośredni transfer przez SSH
2. **HTTP Server** - udostępnienie przez lokalny serwer
3. **Ręczny** - skopiuj pliki samodzielnie

Lub ręcznie:
```bash
scp src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/*.msi user@192.168.0.183:/path/
```

## 🏗️ Struktura projektu

```
sd-helper/
├── src/                    # Frontend Vue
│   ├── App.vue            # Główny komponent
│   ├── main.js            # Entry point
│   ├── components/        # Komponenty UI
│   │   ├── HostList.vue   # Lista hostów
│   │   ├── HostForm.vue   # Formularz dodawania/edycji
│   │   └── HostItem.vue   # Pojedynczy host
│   └── store/             # State management (Pinia)
│       └── hosts.js       # Store dla hostów
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── lib.rs         # Entry point
│   │   ├── commands.rs    # Tauri commands
│   │   ├── storage.rs     # JSON storage
│   │   └── protocols/     # Moduły protokołów
│   │       ├── rdp.rs     # RDP
│   │       ├── vnc.rs     # VNC
│   │       └── ssh.rs     # SSH
│   └── tauri.conf.json    # Konfiguracja Tauri
└── deploy.sh              # Skrypt wdrożeniowy
```

## 💻 Użytkowanie

### Dodawanie hosta

1. Kliknij przycisk "➕ Dodaj hosta"
2. Wypełnij formularz:
   - Nazwa (wymagana)
   - Host/IP (wymagany)
   - Protokół (RDP/VNC/SSH)
   - Port (domyślny dla protokołu)
   - Username (opcjonalnie)
   - Kategoria (opcjonalnie)
   - Notatki (opcjonalnie)
3. Kliknij "Dodaj hosta"

### Łączenie z hostem

- Kliknij przycisk "🔌 Połącz" przy wybranym hoście
- Lub wybierz host i kliknij duży przycisk połączenia w panelu głównym

### Edycja/Usuwanie hosta

- Kliknij ikonę ✏️ aby edytować
- Kliknij ikonę 🗑️ aby usunąć (z potwierdzeniem)

## 🔧 Konfiguracja

### Przechowywanie danych

Hosty są przechowywane lokalnie w pliku JSON:

- **Windows**: `%APPDATA%/sd-helper/hosts.json`
- **macOS**: `~/Library/Application Support/sd-helper/hosts.json`

### Protokoły

#### RDP (Windows tylko)
- Używa `mstsc.exe /v:hostname:port /u:username`
- Domyślny port: 3389

#### VNC (Windows tylko)
- Używa `vncviewer.exe hostname:port`
- Domyślny port: 5900
- Wymaga UltraVNC viewer

#### SSH
- **Windows**: używa `powershell.exe ssh user@host -p port`
- **macOS**: używa `Terminal.app ssh user@host -p port`
- Domyślny port: 22

## 🛠️ Development

### Dostępne komendy

```bash
# Development mode
npm run tauri:dev

# Build dla obecnej platformy
npm run tauri:build

# Build dla Windows (z macOS)
npm run tauri:build:windows

# Deploy (build + transfer)
./deploy.sh
```

### Testowanie na macOS

Protokoły RDP i VNC nie działają na macOS (wymagają Windows). Aplikacja zwróci komunikat informacyjny. SSH działa na obu platformach.

## 📝 Roadmapa

### Etap 1-2: MVP ✅
- [x] Setup projektu
- [x] Podstawowy UI
- [x] Obsługa RDP/VNC/SSH
- [x] CRUD hostów

### Etap 3-4: Baza + Tunelowanie
- [ ] Baza SQLite (obecnie JSON)
- [ ] Tunelowanie SSH
- [ ] Zaawansowane grupowanie

### Etap 5-6: Bezpieczeństwo + UX
- [ ] Szyfrowane hasła
- [ ] Historia połączeń
- [ ] Ulubione hosty
- [ ] Zakładki dla sesji

### Etap 7-8: Integracje
- [ ] UVDesk API
- [ ] Monitoring
- [ ] Transfer plików
- [ ] Automatyczne skrypty

### Etap 9-10: Finalizacja
- [ ] Dark mode
- [ ] Skróty klawiszowe
- [ ] Auto-updater
- [ ] Centralna konfiguracja

## 👤 Autor

Sebastian Konofal

## 📄 Licencja

Projekt prywatny

## 🐛 Znane problemy

- Cross-compilation z macOS na Windows może wymagać dodatkowych narzędzi
- RDP/VNC nie działają na macOS (tylko Windows target)
- SSH na macOS otwiera nowe okno Terminal (nie można uniknąć)

## 💡 Wsparcie

W razie problemów:
1. Sprawdź logi aplikacji
2. Sprawdź czy odpowiednie programy są zainstalowane (mstsc.exe, vncviewer.exe)
3. Sprawdź uprawnienia do zapisu w katalogu app data
