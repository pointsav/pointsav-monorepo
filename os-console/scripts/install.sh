#!/bin/bash
# install.sh — one-shot setup for os-console on Mac Pro (macOS 10.13) or iMac (Linux Mint)
#
# What this script does:
#   1. Checks for Rust toolchain (prompts to install if missing)
#   2. Builds os-console from the current source tree
#   3. Installs binary to ~/bin/os-console
#   4. Writes ~/.config/os-console/config.toml
#   5. Adds "foundry-services" SSH tunnel entry to ~/.ssh/config
#   6. Installs desktop launcher (Linux: .desktop file; macOS: .command file)
#
# Run from the monorepo root:
#   bash os-console/scripts/install.sh
#
# After install, double-click "OS Console" on your desktop to launch.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MONOREPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "=== os-console install script ==="
echo "Source: $MONOREPO_ROOT"
echo ""

# ── Step 1: Rust toolchain ──────────────────────────────────────────────────

if ! command -v cargo > /dev/null 2>&1; then
    echo "Rust toolchain not found."
    echo "Install now? This runs: curl https://sh.rustup.rs | sh"
    read -r -p "[y/N] " ANSWER
    if [ "$ANSWER" = "y" ] || [ "$ANSWER" = "Y" ]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Install Rust first, then re-run this script."
        exit 1
    fi
fi

echo "Rust: $(cargo --version)"

# ── Step 2: Build ───────────────────────────────────────────────────────────

echo ""
echo "Building os-console (first build takes ~5–10 min)..."
cd "$MONOREPO_ROOT"
cargo build --release --bin os-console
echo "Build complete."

# ── Step 3: Install binary ──────────────────────────────────────────────────

mkdir -p "$HOME/bin"
cp target/release/os-console "$HOME/bin/os-console"
chmod +x "$HOME/bin/os-console"
echo "Installed: $HOME/bin/os-console"

# ── Step 4: config.toml ────────────────────────────────────────────────────

CONFIG_DIR="$HOME/.config/os-console"
CONFIG_FILE="$CONFIG_DIR/config.toml"

if [ ! -f "$CONFIG_FILE" ]; then
    echo ""
    read -r -p "Username (e.g. jennifer): " USERNAME
    read -r -p "Tenant (e.g. woodfine): " TENANT

    mkdir -p "$CONFIG_DIR"
    cat > "$CONFIG_FILE" << EOF
[profile]
username = "$USERNAME"
tenant   = "$TENANT"

# MBA SSH connection — localhost forwarded through foundry-services tunnel
totebox_host     = "127.0.0.1"
totebox_ssh_port = 2222

# T0 override: service-input is currently deployed on :9100 on the GCE VM.
# The Phase 10 code defaults to :9106 (port was updated in commit a17cfdb0).
# This line keeps F12 working until Stage 6 redeploys service-input on :9106.
# Remove this line after Stage 6 lands.
ingest_endpoint = "http://127.0.0.1:9100"
EOF
    echo "Created: $CONFIG_FILE"
else
    echo "Config exists, skipping: $CONFIG_FILE"
fi

# ── Step 5: SSH config ──────────────────────────────────────────────────────

SSH_CONFIG="$HOME/.ssh/config"
mkdir -p "$HOME/.ssh"
chmod 700 "$HOME/.ssh"

if ! grep -q "Host foundry-services" "$SSH_CONFIG" 2>/dev/null; then
    echo ""
    read -r -p "GCE VM external IP address: " GCE_IP
    read -r -p "SSH key path [~/.ssh/id_ed25519]: " SSH_KEY
    SSH_KEY="${SSH_KEY:-$HOME/.ssh/id_ed25519}"

    cat >> "$SSH_CONFIG" << EOF

Host foundry-services
  HostName $GCE_IP
  User mathew
  IdentityFile $SSH_KEY
  LocalForward 9080 localhost:9080
  LocalForward 9081 localhost:9081
  LocalForward 9092 localhost:9092
  LocalForward 9100 localhost:9100
  LocalForward 9093 localhost:9093
  LocalForward 9201 localhost:9201
  LocalForward 2222 localhost:2222
  ServerAliveInterval 30
  ServerAliveCountMax 3
EOF
    chmod 600 "$SSH_CONFIG"
    echo "SSH config updated: $SSH_CONFIG"
else
    echo "SSH entry exists, skipping: foundry-services"
fi

# ── Step 6: Desktop launcher ────────────────────────────────────────────────

case "$(uname -s)" in
  Linux)
    DESKTOP_FILE="$HOME/Desktop/OS Console.desktop"
    cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=OS Console
Comment=Totebox Console
Exec=bash -c '$HOME/bin/launch-console.sh; exec bash'
Terminal=true
Type=Application
Icon=utilities-terminal
StartupNotify=false
EOF
    chmod +x "$DESKTOP_FILE"
    # Install launcher script
    cp "$SCRIPT_DIR/launch-console.sh" "$HOME/bin/launch-console.sh"
    chmod +x "$HOME/bin/launch-console.sh"
    echo "Desktop launcher created: $DESKTOP_FILE"
    echo "Right-click → 'Allow Launching' once, then double-click to open."
    ;;

  Darwin)
    LAUNCHER="$HOME/Desktop/OS Console.command"
    cp "$SCRIPT_DIR/launch-console.command" "$LAUNCHER"
    chmod +x "$LAUNCHER"
    cp "$SCRIPT_DIR/launch-console.command" "$HOME/bin/launch-console.command"
    chmod +x "$HOME/bin/launch-console.command"
    echo "Desktop launcher created: $LAUNCHER"
    echo "First open: right-click → Open → Open anyway (Gatekeeper prompt, one-time)."
    ;;

  *)
    echo "Unknown OS — install launcher manually from os-console/scripts/"
    ;;
esac

# ── Done ────────────────────────────────────────────────────────────────────

echo ""
echo "=== Setup complete ==="
echo ""
echo "To launch: double-click 'OS Console' on your desktop"
echo "  or run:  ~/bin/launch-console.sh   (Linux)"
echo "  or run:  ~/bin/launch-console.command   (macOS)"
echo ""
echo "Verify services on first launch:"
echo "  F4  → Proofreader (ContentCartridge)"
echo "  F12 → Input Machine"
echo ""
echo "To rebuild after source update:"
echo "  cd $MONOREPO_ROOT && git pull && cargo build --release --bin os-console && cp target/release/os-console ~/bin/os-console"
