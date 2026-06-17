#!/bin/bash
# install.sh — one-shot setup for os-console on Mac Pro (macOS 10.13) or iMac (Linux Mint)
#
# What this script does:
#   1. Checks for Rust toolchain (installs silently if missing)
#   2. Builds os-console from the current source tree
#   3. Installs binary to ~/bin/os-console
#   4. Writes ~/.config/os-console/config.toml
#   5. Auto-detects the right SSH key and adds "foundry-services" tunnel entry
#   6. Installs desktop launcher (double-click to open)
#
# Run from the monorepo root:
#   bash os-console/scripts/install.sh
#
# After install, double-click "OS Console" on your desktop to launch.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MONOREPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
GCE_IP="34.53.65.203"
GCE_USER="mathew"

echo "=== os-console setup ==="
echo ""

# ── Step 1: Rust toolchain ──────────────────────────────────────────────────

if ! command -v cargo > /dev/null 2>&1; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet
    source "$HOME/.cargo/env"
fi

echo "Rust: $(cargo --version)"

# ── Step 2: Build ───────────────────────────────────────────────────────────

echo ""
echo "Building os-console (first build: ~5–10 min)..."
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
    mkdir -p "$CONFIG_DIR"
    cat > "$CONFIG_FILE" << 'EOF'
[profile]
username = "jennifer"
tenant   = "woodfine"

totebox_host     = "127.0.0.1"
totebox_ssh_port = 2222

# T0: service-input runs on :9100 until Stage 6 redeploys on :9106
ingest_endpoint = "http://127.0.0.1:9100"
EOF
    echo "Config: $CONFIG_FILE"
fi

# ── Step 5: SSH key auto-detect + tunnel entry ─────────────────────────────

SSH_CONFIG="$HOME/.ssh/config"
mkdir -p "$HOME/.ssh"
chmod 700 "$HOME/.ssh"

# Remove any broken foundry-services entry from a previous failed install
if grep -q "Host foundry-services" "$SSH_CONFIG" 2>/dev/null; then
    # Check if it actually works
    if ssh -N -o BatchMode=yes -o ConnectTimeout=5 foundry-services true 2>/dev/null; then
        echo "SSH tunnel entry already working — skipping."
        SKIP_SSH=1
    else
        echo "Removing broken foundry-services entry..."
        # Remove the block from the config
        python3 - "$SSH_CONFIG" << 'PYEOF'
import sys
path = sys.argv[1]
with open(path) as f:
    lines = f.readlines()
out, skip = [], False
for line in lines:
    if line.strip() == "Host foundry-services":
        skip = True
    elif skip and (line.startswith("Host ") or (line.strip() and not line.startswith(" ") and not line.startswith("\t"))):
        skip = False
    if not skip:
        out.append(line)
with open(path, "w") as f:
    f.writelines(out)
PYEOF
    fi
fi

if [ -z "$SKIP_SSH" ]; then
    # Auto-detect the SSH key that can reach the GCE VM
    SSH_KEY=""
    for CANDIDATE in \
        "$HOME/.ssh/google_compute_engine" \
        "$HOME/.ssh/id_ed25519" \
        "$HOME/.ssh/id_rsa" \
        "$HOME/.ssh/id_rsa_foundry"
    do
        if [ -f "$CANDIDATE" ]; then
            if ssh -i "$CANDIDATE" \
                   -o BatchMode=yes \
                   -o ConnectTimeout=5 \
                   -o StrictHostKeyChecking=no \
                   "$GCE_USER@$GCE_IP" true 2>/dev/null; then
                SSH_KEY="$CANDIDATE"
                echo "SSH key: $SSH_KEY"
                break
            fi
        fi
    done

    if [ -z "$SSH_KEY" ]; then
        echo ""
        echo "ERROR: Could not find an SSH key that connects to $GCE_IP."
        echo "Run:  ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -N ''"
        echo "Then ask the system administrator to add your public key:"
        echo "  cat ~/.ssh/id_ed25519.pub"
        exit 1
    fi

    # Accept host key silently on first connect
    ssh-keyscan -H "$GCE_IP" >> "$HOME/.ssh/known_hosts" 2>/dev/null

    cat >> "$SSH_CONFIG" << EOF

Host foundry-services
  HostName $GCE_IP
  User $GCE_USER
  IdentityFile $SSH_KEY
  StrictHostKeyChecking no
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
    echo "SSH tunnel entry added."
fi

# ── Step 6: Desktop launcher ────────────────────────────────────────────────

cp "$SCRIPT_DIR/launch-console.sh"      "$HOME/bin/launch-console.sh"      2>/dev/null || true
cp "$SCRIPT_DIR/launch-console.command" "$HOME/bin/launch-console.command" 2>/dev/null || true
chmod +x "$HOME/bin/launch-console.sh"      2>/dev/null || true
chmod +x "$HOME/bin/launch-console.command" 2>/dev/null || true

case "$(uname -s)" in
  Linux)
    DESKTOP_FILE="$HOME/Desktop/OS Console.desktop"
    LAUNCH_CMD="$HOME/bin/launch-console.sh"
    cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=OS Console
Comment=Totebox Console
Exec=bash -c '$LAUNCH_CMD; exec bash'
Terminal=true
Type=Application
Icon=utilities-terminal
StartupNotify=false
EOF
    chmod +x "$DESKTOP_FILE"
    echo "Desktop launcher: $DESKTOP_FILE"
    echo "Right-click → 'Allow Launching' once, then double-click to open."
    ;;

  Darwin)
    LAUNCHER="$HOME/Desktop/OS Console.command"
    cp "$SCRIPT_DIR/launch-console.command" "$LAUNCHER"
    chmod +x "$LAUNCHER"
    echo "Desktop launcher: $LAUNCHER"
    echo "First open: right-click → Open → Open anyway (one-time Gatekeeper prompt)."
    ;;
esac

# ── Done ────────────────────────────────────────────────────────────────────

echo ""
echo "=== Setup complete ==="
echo ""
echo "Double-click 'OS Console' on your desktop to launch."
echo ""
echo "To rebuild after a source update:"
echo "  cd $MONOREPO_ROOT && cargo build --release --bin os-console && cp target/release/os-console ~/bin/os-console"
