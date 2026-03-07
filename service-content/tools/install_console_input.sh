#!/usr/bin/env bash
# Purpose: Setup Desktop Hot-Zone and symlink the pointsav-input command.
echo "🏛️  POINTSAV CONSOLE: INITIATING INSTALLER..."

# Find the absolute path of the app script
SCRIPT_SRC="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
APP_PATH="${SCRIPT_SRC}/app-console-input.sh"

# Create Desktop Airlock
echo "📂 Creating Desktop Hot-Zone..."
mkdir -p ~/Desktop/service-content/input/processed
mkdir -p ~/Desktop/service-content/output
mkdir -p ~/Desktop/service-content/logs

# Create Global Symlink
echo "🔗 Linking 'pointsav-input' command..."
sudo ln -sf "$APP_PATH" /usr/local/bin/pointsav-input

echo "✅ SUCCESS: pointsav-input installed."
echo "👉 Drop files in ~/Desktop/service-content/input and run 'pointsav-input'."
