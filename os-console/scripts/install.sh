#!/bin/bash
# install.sh — one-shot setup for os-console on Mac Pro (macOS 10.13) or iMac (Linux Mint)
#
# What this script does:
#   1. Checks for Rust toolchain (installs silently if missing)
#   2. Builds os-console from the current source tree
#   3. Installs binary to ~/bin/os-console
#   4. Detects SSH key that can reach the GCE VM
#   5. Writes ~/.config/os-console/config.toml with tunnel settings
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
    # shellcheck source=/dev/null
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

# ── Step 4: SSH key auto-detect ─────────────────────────────────────────────

mkdir -p "$HOME/.ssh"
chmod 700 "$HOME/.ssh"

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

# Accept host key silently
ssh-keyscan -H "$GCE_IP" >> "$HOME/.ssh/known_hosts" 2>/dev/null

# ── Step 5: config.toml ────────────────────────────────────────────────────
#
# The binary manages the SSH tunnel internally — no separate ssh process needed.
# gce_host enables the embedded tunnel; ssh_key_path points to the key it uses.

CONFIG_DIR="$HOME/.config/os-console"
CONFIG_FILE="$CONFIG_DIR/config.toml"
mkdir -p "$CONFIG_DIR"

cat > "$CONFIG_FILE" << EOF
[profile]
username = "jennifer"
tenant   = "woodfine"

totebox_host     = "127.0.0.1"
totebox_ssh_port = 2222

# T0: service-input runs on :9100 until Stage 6 redeploys on :9106
ingest_endpoint = "http://127.0.0.1:9100"

# Embedded SSH tunnel — binary connects to gce_host at startup
gce_host     = "$GCE_IP"
gce_user     = "$GCE_USER"
ssh_key_path = "$SSH_KEY"
EOF

echo "Config: $CONFIG_FILE"

# ── Step 6: Desktop launcher ────────────────────────────────────────────────
#
# The binary now handles its own tunnel — the launcher just runs the binary.

case "$(uname -s)" in
  Linux)
    DESKTOP_FILE="$HOME/Desktop/OS Console.desktop"
    cat > "$DESKTOP_FILE" << 'DEOF'
[Desktop Entry]
Name=OS Console
Comment=Totebox Console
Exec=bash -c 'exec ~/bin/os-console'
Terminal=true
Type=Application
Icon=utilities-terminal
StartupNotify=false
DEOF
    chmod +x "$DESKTOP_FILE"
    echo "Desktop launcher: $DESKTOP_FILE"
    echo "Right-click → 'Allow Launching' once, then double-click to open."
    ;;

  Darwin)
    LAUNCHER="$HOME/Desktop/OS Console.command"
    cat > "$LAUNCHER" << 'DEOF'
#!/bin/bash
exec ~/bin/os-console
DEOF
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
