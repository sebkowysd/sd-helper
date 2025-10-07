#!/bin/bash

# deploy.sh - Skrypt do budowania i wdrażania aplikacji na Windows laptop

echo "🔨 Budowanie aplikacji SD Helper dla Windows..."

# Build aplikacji dla Windows
npm run tauri:build:windows

if [ $? -eq 0 ]; then
    echo "✅ Build zakończony pomyślnie!"
    
    # Ścieżki do plików instalacyjnych
    MSI_PATH="src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi"
    NSIS_PATH="src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis"
    
    echo ""
    echo "📦 Pliki instalacyjne:"
    
    # Pokaż pliki MSI
    if [ -d "$MSI_PATH" ]; then
        echo "MSI installers:"
        ls -lh "$MSI_PATH"/*.msi 2>/dev/null
    fi
    
    # Pokaż pliki NSIS
    if [ -d "$NSIS_PATH" ]; then
        echo "NSIS installers:"
        ls -lh "$NSIS_PATH"/*.exe 2>/dev/null
    fi
    
    echo ""
    echo "🚀 Transfer na laptop Windows (192.168.0.183)..."
    echo ""
    echo "Wybierz metodę transferu:"
    echo "1) SCP (wymaga SSH na Windows)"
    echo "2) HTTP server (udostępnij lokalnie)"
    echo "3) Pomiń transfer"
    read -p "Wybór (1-3): " choice
    
    case $choice in
        1)
            read -p "Podaj użytkownika Windows: " windows_user
            read -p "Podaj ścieżkę docelową (np. C:\\Users\\$windows_user\\Downloads): " dest_path
            
            # Transfer przez SCP
            if [ -d "$MSI_PATH" ]; then
                for file in "$MSI_PATH"/*.msi; do
                    if [ -f "$file" ]; then
                        echo "Przesyłanie $file..."
                        scp "$file" "$windows_user@192.168.0.183:$dest_path"
                    fi
                done
            fi
            
            if [ -d "$NSIS_PATH" ]; then
                for file in "$NSIS_PATH"/*.exe; do
                    if [ -f "$file" ]; then
                        echo "Przesyłanie $file..."
                        scp "$file" "$windows_user@192.168.0.183:$dest_path"
                    fi
                done
            fi
            
            echo "✅ Transfer zakończony!"
            ;;
        2)
            echo "Uruchamianie HTTP servera na porcie 8000..."
            echo "Otwórz na Windows: http://$(ipconfig getifaddr en0):8000"
            cd src-tauri/target/x86_64-pc-windows-msvc/release/bundle
            python3 -m http.server 8000
            ;;
        3)
            echo "Transfer pominięty."
            ;;
        *)
            echo "Nieprawidłowy wybór."
            ;;
    esac
else
    echo "❌ Build nie powiódł się!"
    exit 1
fi

