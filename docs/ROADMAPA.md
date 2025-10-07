# 🗺️ Roadmapa rozwoju SD Helper

## Status aktualny: ✅ Etap 1-2 (MVP - Setup) - UKOŃCZONY

---

## Etap 1-2: MVP – Absolutne podstawy ✅ UKOŃCZONY

### Cel
Stworzenie działającej aplikacji z podstawowymi funkcjami zarządzania połączeniami.

### Zrealizowane funkcje
- ✅ Utworzenie projektu Tauri + Vue
- ✅ Podstawowy interfejs użytkownika
- ✅ Lista hostów z wyszukiwaniem
- ✅ Formularz dodawania/edycji hostów
- ✅ Obsługa RDP (mstsc.exe)
- ✅ Obsługa VNC (vncviewer.exe)
- ✅ Obsługa SSH (PowerShell/Terminal)
- ✅ Lokalne przechowywanie danych (JSON)
- ✅ CRUD dla hostów (Create, Read, Update, Delete)
- ✅ Grupowanie po protokole i kategorii
- ✅ Notatki przy hostach
- ✅ Cross-compilation (macOS → Windows)
- ✅ Skrypt deploymentu

### Rezultat
Funkcjonalna aplikacja gotowa do testowania w środowisku produkcyjnym.

---

## Etap 3-4: Zarządzanie hostami + Tunelowanie 🔄 ZAPLANOWANE

### Cel
Rozszerzenie zarządzania hostami i dodanie zaawansowanej obsługi SSH z tunelowaniem.

### Planowane funkcje

#### Zarządzanie hostami
- [ ] Migracja z JSON na SQLite
  - Lepsze zarządzanie dużą ilością hostów
  - Szybsze wyszukiwanie
  - Możliwość backupu
- [ ] Import/Export hostów
  - Format CSV
  - Format JSON
  - Backup/restore całej bazy
- [ ] Zaawansowane grupowanie
  - Foldery/kategorie hierarchiczne
  - Kolory dla kategorii
  - Ikony dla hostów
- [ ] Sortowanie listy
  - Po nazwie
  - Po dacie dodania
  - Po ostatnim użyciu
  - Po protokole

#### Tunelowanie SSH
- [ ] Konfiguracja bastion hosta
  - Dodanie pola "Tunel przez" w formularzu hosta
  - Lista dostępnych serwerów bastion
- [ ] Automatyczne zestawianie tunelu
  - `ssh -L localport:targethost:targetport user@bastion`
  - Śledzenie aktywnych tuneli
  - Automatyczne zamykanie tunelu
- [ ] Mapowanie portów
  - Automatyczne wybieranie wolnych portów lokalnych
  - Historia mapowań
- [ ] Obsługa kluczy SSH
  - Wybór pliku klucza prywatnego
  - Zarządzanie kluczami
- [ ] Status tunelu w UI
  - Zielona kropka = aktywny
  - Czerwona kropka = nieaktywny
  - Możliwość ręcznego rozłączenia

### Szacowany czas
2-3 tygodnie

---

## Etap 5-6: Bezpieczeństwo + Ułatwienia dla servicedesk 🔒 ZAPLANOWANE

### Cel
Zabezpieczenie aplikacji i dodanie funkcji usprawniających codzienną pracę.

### Bezpieczeństwo

#### Szyfrowane przechowywanie poświadczeń
- [ ] Integracja z Windows Credential Manager
  - Bezpieczne przechowywanie haseł
  - Automatyczne odczytywanie przy połączeniu
- [ ] Opcjonalne hasło główne do aplikacji
  - Szyfrowanie lokalnej bazy
  - Timeout sesji
- [ ] Logowanie do aplikacji
  - Integracja z Active Directory / LDAP
  - Lokalne konta użytkowników
- [ ] Role użytkowników
  - Admin - pełny dostęp
  - Technik - tylko podgląd i połączenia
  - Manager - tylko raporty

#### Audyt i logi
- [ ] Logowanie wszystkich działań
  - Kto się połączył
  - Kiedy
  - Z jakim hostem
- [ ] Eksport logów
  - Format CSV/JSON
  - Filtrowanie po dacie/użytkowniku

### Ułatwienia dla servicedesk

#### Historia połączeń
- [ ] Panel "Ostatnie połączenia"
  - Timestamp
  - Host
  - Protokół
  - Status (sukces/błąd)
- [ ] Szybkie ponowne połączenie
  - Jeden klik z historii
- [ ] Statystyki
  - Najczęściej używane hosty
  - Czas spędzony na połączeniach

#### Ulubione hosty
- [ ] Oznaczanie hostów gwiazdką ⭐
- [ ] Panel "Ulubione" w sidebar
- [ ] Szybki dostęp klawiszem (Ctrl+1-9)

#### Zakładki dla wielu sesji
- [ ] Panel zakładek u góry okna
- [ ] Przełączanie między aktywnymi połączeniami
- [ ] Śledzenie statusu sesji
- [ ] Zamykanie zakładek

#### Dodatkowe funkcje
- [ ] Notatki globalne
  - Dziennik pracy
  - Quick notes
- [ ] Przypomnienia
  - "Sprawdź serwer X za godzinę"
- [ ] Powiadomienia systemowe
  - Połączenie nawiązane
  - Połączenie zakończone
  - Błędy

### Szacowany czas
3-4 tygodnie

---

## Etap 7-8: Integracje + Zaawansowane funkcje 🔗 ZAPLANOWANE

### Cel
Integracja z zewnętrznymi systemami i dodanie zaawansowanych funkcji.

### Integracje

#### System ticketingu (UVDesk)
- [ ] Konfiguracja połączenia API
  - URL, API key
- [ ] Lista zgłoszeń w aplikacji
  - Otwarte tickety
  - Przypisane do mnie
  - Pilne
- [ ] Powiązanie hosta z ticketem
  - Automatyczne kopiowanie numeru ticketu do notatek
- [ ] Kliknięcie w zgłoszenie → otwiera powiązanego hosta
- [ ] Aktualizacja statusu ticketu z aplikacji

#### Monitoring (Zabbix API)
- [ ] Konfiguracja połączenia API
- [ ] Wyświetlanie alertów
  - Lista aktywnych problemów
  - Filtrowanie po severity
- [ ] Kliknięcie w alert → otwiera powiązanego hosta
- [ ] Status hostów w liście (🟢🔴)
  - Zielony = OK
  - Czerwony = Problem

#### Inne integracje
- [ ] Slack/Teams notifications
  - Powiadomienie o rozpoczęciu sesji
- [ ] Microsoft 365
  - Synchronizacja kalendarza
  - Scheduled maintenance

### Zaawansowane funkcje

#### Transfer plików
- [ ] Panel "Transfer plików"
- [ ] PSCP dla Windows (PuTTY)
  - Upload/Download
  - Progress bar
- [ ] SFTP dla SSH
  - Przeglądarka plików
  - Drag & drop
- [ ] RDP shared folders
  - Mapowanie lokalnych folderów

#### Clipboard sync
- [ ] Automatyczne kopiowanie między sesją a lokalnym systemem
- [ ] Historia schowka (ostatnie 10 elementów)
- [ ] Bezpieczne czyszczenie (po zamknięciu sesji)

#### Automatyczne skrypty
- [ ] Panel "Skrypty"
- [ ] Predefiniowane skrypty
  - Restart usługi
  - Sprawdź logi
  - Status dysku
- [ ] Edytor skryptów
  - PowerShell dla Windows
  - Bash dla Linux
- [ ] Wykonywanie na wielu hostach jednocześnie
- [ ] Output w aplikacji

#### Dashboard aktywnych sesji
- [ ] Widok kafelkowy wszystkich aktywnych połączeń
- [ ] Czas trwania sesji
- [ ] Zużycie zasobów (opcjonalnie)

### Szacowany czas
4-6 tygodni

---

## Etap 9-10: UX i Wdrożenie 🎨 ZAPLANOWANE

### Cel
Dopracowanie interfejsu i przygotowanie do wdrożenia w firmie.

### UX i ergonomia

#### Motywy
- [ ] Dark mode
  - Przełącznik w ustawieniach
  - Automatyczne przełączanie (dzień/noc)
- [ ] Light mode (już jest)
- [ ] Custom themes
  - Możliwość zmiany kolorów

#### Skróty klawiszowe
- [ ] Globalne skróty
  - `Ctrl+N` - Nowy host
  - `Ctrl+F` - Wyszukiwanie
  - `Ctrl+Q` - Zamknij aplikację
- [ ] Skróty do ulubionych
  - `Ctrl+1-9` - Połącz z ulubionym #1-9
- [ ] Nawigacja klawiaturą
  - Strzałki po liście
  - Enter - połącz
  - Delete - usuń

#### Powiadomienia systemowe
- [ ] Windows toast notifications
  - Połączenie nawiązane
  - Tunel zestawiony
  - Błędy połączenia
- [ ] Konfiguracja powiadomień
  - Włącz/wyłącz
  - Dźwięk
  - Czas wyświetlania

#### Dostępność
- [ ] Skalowanie interfejsu
  - Małe/Średnie/Duże czcionki
- [ ] Skróty klawiaturowe dla screenreaderów
- [ ] Kontrast kolorów (WCAG AA)

### Wdrożenie i utrzymanie

#### Pakiety instalacyjne
- [ ] MSI installer
  - Opcje instalacji
  - Silent install dla IT
- [ ] NSIS installer (alternatywny)
- [ ] Portable version (ZIP)
  - Bez instalacji
  - Dla adminiów

#### Automatyczne aktualizacje
- [ ] Tauri Updater
  - Sprawdzanie dostępności nowych wersji
  - Pobieranie w tle
  - Instalacja przy następnym uruchomieniu
- [ ] Changelog w aplikacji
  - "Co nowego" po aktualizacji
- [ ] Beta channel
  - Testerzy mogą otrzymywać wczesne wersje

#### Centralna konfiguracja
- [ ] Serwer z centralną listą hostów
  - REST API
  - Synchronizacja między stanowiskami
- [ ] Polityki grupowe
  - Admin może wymusić ustawienia
  - Blokowanie pewnych funkcji
- [ ] Multi-tenant
  - Różne firmy/zespoły w jednej bazie

#### Monitoring użycia
- [ ] Anonimowe statystyki (opt-in)
  - Jakie funkcje są używane
  - Błędy i crashe
- [ ] Logi lokalne
  - Debugging
  - Audyt dla IT
- [ ] Remote logging (opcjonalnie)
  - Centralne zbieranie logów

#### Dokumentacja
- [ ] Video tutorial
- [ ] FAQ
- [ ] Administrator guide
  - Deployment
  - GPO integration
  - Troubleshooting

### Szacowany czas
3-4 tygodnie

---

## Przyszłe pomysły (backlog) 💡

### Funkcje do rozważenia w kolejnych wersjach

- Mobile app (iOS/Android) - podgląd hostów, triggering połączeń
- Web interface - thin client w przeglądarce
- Team collaboration - udostępnianie hostów w zespole
- Chat integration - łączenie się z hostem z poziomu czatu
- AI assistant - sugestie rozwiązań na podstawie logów
- Macros/automations - nagrywanie sekwencji akcji
- Multi-hop SSH - tunelowanie przez kilka serwerów
- VPN integration - automatyczne łączenie z VPN
- 2FA dla aplikacji - dodatkowe zabezpieczenie
- Backup do chmury - synchronizacja między komputerami
- Role-based dashboards - różne widoki dla różnych ról

---

## Harmonogram ogólny

| Etap | Czas szacowany | Status |
|------|----------------|--------|
| 1-2: MVP | 1-2 tygodnie | ✅ Ukończony |
| 3-4: Hosty + Tunele | 2-3 tygodnie | 🔄 Zaplanowane |
| 5-6: Bezpieczeństwo | 3-4 tygodnie | 📅 Przyszłość |
| 7-8: Integracje | 4-6 tygodni | 📅 Przyszłość |
| 9-10: UX + Deploy | 3-4 tygodnie | 📅 Przyszłość |

**Łącznie:** około 3-4 miesiące do pełnej wersji produkcyjnej

---

## Priorytetyzacja

### Must have (wymagane)
- ✅ MVP (Etap 1-2)
- Bezpieczeństwo (Etap 5)
- Instalator (Etap 9)

### Should have (bardzo przydatne)
- Tunelowanie (Etap 3)
- Historia i ulubione (Etap 6)
- Auto-updater (Etap 10)

### Nice to have (miłe dodatki)
- Integracje (Etap 7-8)
- Dark mode (Etap 9)
- Skrypty (Etap 8)

### Won't have now (na później)
- Mobile app
- Web interface
- AI features

---

**Ostatnia aktualizacja:** Październik 2025  
**Wersja:** 0.1.0

