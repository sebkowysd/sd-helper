# Instrukcja obsługi SD Helper

## Wprowadzenie

SD Helper to aplikacja stworzona z myślą o zespołach ServiceDesk, która ułatwia zarządzanie i łączenie się z wieloma serwerami i urządzeniami.

## Pierwsze uruchomienie

### Instalacja

1. Pobierz plik instalacyjny (MSI lub EXE) dla Windows
2. Uruchom instalator i postępuj zgodnie z instrukcjami
3. Po instalacji uruchom aplikację z menu Start

### Interfejs użytkownika

Aplikacja składa się z trzech głównych części:

1. **Nagłówek** - tytuł aplikacji i informacje
2. **Boczny panel (sidebar)** - lista wszystkich hostów z wyszukiwarką
3. **Panel główny** - szczegóły wybranego hosta lub formularz dodawania/edycji

## Zarządzanie hostami

### Dodawanie nowego hosta

1. Kliknij zielony przycisk **"➕ Dodaj hosta"** w górnej części bocznego panelu
2. Wypełnij formularz:
   
   **Pola wymagane:**
   - **Nazwa** - przyjazna nazwa dla hosta (np. "Serwer główny klienta X")
   - **Host / IP** - adres IP lub nazwa domenowa
   - **Protokół** - wybierz RDP, VNC lub SSH
   - **Port** - port połączenia (automatycznie ustawiony dla wybranego protokołu)

   **Pola opcjonalne:**
   - **Nazwa użytkownika** - login do połączenia
   - **Kategoria** - grupowanie hostów (np. nazwa klienta, środowisko)
   - **Notatki** - dodatkowe informacje

3. Kliknij **"Dodaj hosta"** aby zapisać

### Domyślne porty

- **RDP**: 3389
- **VNC**: 5900
- **SSH**: 22

Port zmieni się automatycznie przy zmianie protokołu, ale możesz go edytować jeśli serwer używa niestandardowego portu.

### Wyszukiwanie hostów

W górnej części bocznego panelu znajduje się pole wyszukiwania. Możesz szukać po:
- Nazwie hosta
- Adresie IP/hostname
- Kategorii

Wyniki filtrują się na żywo podczas pisania.

### Łączenie z hostem

Są dwa sposoby na połączenie z hostem:

1. **Szybkie połączenie** - kliknij zielony przycisk **"🔌 Połącz"** bezpośrednio przy wybranym hoście na liście

2. **Połączenie z panelu** - wybierz host klikając na niego (podświetli się na zielono), zobaczysz szczegóły w panelu głównym, następnie kliknij duży przycisk **"🔌 Połącz z tym hostem"**

### Edycja hosta

1. Kliknij ikonę **✏️** przy wybranym hoście
2. Zmodyfikuj potrzebne pola w formularzu
3. Kliknij **"Zapisz zmiany"**

### Usuwanie hosta

1. Kliknij ikonę **🗑️** przy wybranym hoście
2. Potwierdź usunięcie w oknie dialogowym

**UWAGA:** Usunięcie jest trwałe i nieodwracalne!

## Protokoły połączeń

### RDP (Remote Desktop Protocol)

- **Wymaga:** Windows z zainstalowanym mstsc.exe (domyślnie w Windows)
- **Użycie:** Zdalne połączenie z pulpitem Windows
- **Domyślny port:** 3389

Aplikacja uruchamia `mstsc.exe /v:host:port /u:username`

### VNC (Virtual Network Computing)

- **Wymaga:** UltraVNC Viewer zainstalowany w systemie
- **Użycie:** Zdalne połączenie z pulpitem (różne systemy)
- **Domyślny port:** 5900

Aplikacja uruchamia `vncviewer.exe host:port`

### SSH (Secure Shell)

- **Wymaga:** OpenSSH Client (wbudowany w Windows 10+)
- **Użycie:** Terminal/konsola zdalnego serwera
- **Domyślny port:** 22

Aplikacja uruchamia `powershell.exe ssh user@host -p port`

## Lokalizacja danych

Wszystkie hosty są przechowywane lokalnie na Twoim komputerze w formacie JSON:

**Windows:** `C:\Users\[TwojaNazwa]\AppData\Roaming\sd-helper\hosts.json`

### Import/Export hostów

Możesz ręcznie skopiować plik `hosts.json` aby:
- Przenieść hosty na inne stanowisko
- Stworzyć kopię zapasową
- Udostępnić listę hostów współpracownikom

Przykładowy format pliku znajduje się w `hosts.example.json` w katalogu instalacyjnym.

## Kolory i oznaczenia

### Kolorowe etykiety protokołów

- 🔵 **Niebieski** - RDP
- 🟠 **Pomarańczowy** - VNC
- 🟣 **Fioletowy** - SSH

### Podświetlenie hosta

- **Biały** - niewybrany host
- **Zielona ramka** - najechanie myszą
- **Zielone tło** - aktualnie wybrany host

## Rozwiązywanie problemów

### "Nie można uruchomić mstsc.exe"

1. Sprawdź czy używasz Windows (RDP nie działa na innych systemach)
2. Sprawdź czy mstsc.exe istnieje: `C:\Windows\System32\mstsc.exe`

### "Nie można uruchomić vncviewer.exe"

1. Sprawdź czy masz zainstalowany UltraVNC Viewer
2. Upewnij się, że vncviewer.exe jest w PATH lub w `C:\Program Files\UltraVNC\`

### "Nie można połączyć przez SSH"

1. Sprawdź czy OpenSSH Client jest włączony w Windows:
   - Ustawienia → Aplikacje → Funkcje opcjonalne → Klient OpenSSH
2. Sprawdź czy port 22 nie jest zablokowany przez firewall

### Host nie zapisuje się

1. Sprawdź czy masz uprawnienia do zapisu w katalogu AppData
2. Sprawdź czy dysk nie jest pełny
3. Sprawdź logi aplikacji

### Aplikacja nie uruchamia się

1. Sprawdź wymagania systemowe (Windows 10+)
2. Zainstaluj Microsoft Visual C++ Redistributable
3. Skontaktuj się z administratorem

## Skróty klawiaturowe

*Planowane w przyszłych wersjach*

## Planowane funkcje

- 🔄 Tunelowanie SSH przez bastion host
- 🔒 Szyfrowane przechowywanie haseł
- 📊 Historia połączeń
- 🎯 Ulubione hosty
- 🌙 Dark mode
- 🔗 Integracja z systemem ticketowym (UVDesk)
- 📁 Transfer plików
- ⚡ Skróty klawiszowe

## Wsparcie techniczne

W razie problemów lub pytań:
1. Sprawdź tę instrukcję
2. Sprawdź README.md w repozytorium
3. Skontaktuj się z autorem

---

**Wersja dokumentacji:** 0.1.0  
**Data aktualizacji:** Październik 2025

