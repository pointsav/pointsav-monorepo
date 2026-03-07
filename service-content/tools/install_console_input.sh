#!/usr/bin/env bash
# ==============================================================================
# Script: install_console_input.sh
# Purpose: Setup the Desktop Hot-Zone and symlink the pointsav-input command.
# ==============================================================================

echo "🏛️  POINTSAV CONSOLE: INITIATING INSTALLER..."

# 1. Determine the absolute path of the script source
SCRIPT_SRC="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
APP_PATH="${SCRIPT_SRC}/app-console-input.sh"

# 2. Create Desktop Hot-Zone (The Operator Workspace)
echo "📂 Creating Desktop Hot-Zone at ~/Desktop/service-content/..."
mkdir -p ~/Desktop/service-content/input
mkdir -p ~/Desktop/service-content/output
mkdir -p ~/Desktop/service-content/logs
mkdir -p ~/Desktop/service-content/input/processed

# 3. Set Permissions
chmod 755 ~/Desktop/service-content/input
chmod 755 ~/Desktop/service-content/output

# 4. Create Symlink to Command Authority
echo "🔗 Linking 'pointsav-input' command..."
sudo ln -sf "$APP_PATH" /usr/local/bin/pointsav-input

echo "------------------------------------------------"
echo "✅ INSTALLATION COMPLETE"
echo "👉 Place legacy files in '~/Desktop/service-content/input'"
echo "👉 Type 'pointsav-input' in any terminal to process them."
