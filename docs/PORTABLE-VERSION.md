# 📦 Wersja Portable SD Helper

## Pobieranie z GitHub Actions

### Krok 1: Przejdź do Actions
1. Otwórz repozytorium na GitHub
2. Kliknij zakładkę **"Actions"**
3. Wybierz workflow **"Build SD Helper"**

### Krok 2: Pobierz artifacts
1. Kliknij na najnowszy build (zielony znaczek ✅)
2. Przewiń w dół do sekcji **"Artifacts"**
3. Pobierz **"sd-helper-portable"** (plik ZIP)

### Krok 3: Rozpakuj i uruchom
1. Rozpakuj `sd-helper-portable.zip` do dowolnego folderu
2. Uruchom `start.bat` lub `sd-helper.exe`
3. Pierwsze uruchomienie może zająć chwilę

## Zawartość paczki portable

```
sd-helper-portable/
├── sd-helper.exe          # Główna aplikacja
├── start.bat              # Skrypt uruchamiający
├── README.md              # Dokumentacja
├── hosts.example.json     # Przykładowe hosty
└── PORTABLE-README.txt    # Instrukcja portable
```

## Wymagania systemowe

- **Windows 10/11** (64-bit)
- **mstsc.exe** - wbudowane w Windows (RDP)
- **UltraVNC Viewer** - do pobrania (VNC)
- **OpenSSH Client** - wbudowane w Windows 10+ (SSH)

## Lokalizacja danych

Aplikacja zapisuje dane w:
```
C:\Users\[TwojaNazwa]\AppData\Roaming\sd-helper\hosts.json
```

## Rozwiązywanie problemów

### "Nie można uruchomić aplikacji"
- Sprawdź czy masz Windows 10/11
- Uruchom jako administrator (prawy klik → "Uruchom jako administrator")

### "Brak mstsc.exe"
- RDP jest wbudowane w Windows
- Sprawdź: `C:\Windows\System32\mstsc.exe`

### "Brak vncviewer.exe"
- Pobierz UltraVNC Viewer z: https://www.uvnc.com/downloads/ultravnc.html
- Zainstaluj lub skopiuj `vncviewer.exe` do folderu z aplikacją

### "Błąd SSH"
- Włącz OpenSSH Client w Windows:
  - Ustawienia → Aplikacje → Funkcje opcjonalne → Klient OpenSSH

## Automatyczne aktualizacje

Workflow uruchamia się automatycznie przy:
- Push do branch `main`
- Pull request do `main`
- Ręcznym uruchomieniu (workflow_dispatch)

## Ręczne uruchomienie build

1. Przejdź do **Actions** → **Build SD Helper**
2. Kliknij **"Run workflow"**
3. Wybierz branch `main`
4. Kliknij **"Run workflow"**
5. Poczekaj na zakończenie (5-10 minut)
6. Pobierz artifacts

---

**Ostatnia aktualizacja:** Październik 2025  
**Wersja:** 0.1.0 Portable
