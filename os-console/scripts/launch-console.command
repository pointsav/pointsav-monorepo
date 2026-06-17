#!/bin/bash
# launch-console.command — macOS launcher for os-console
#
# .command extension tells macOS to open this in Terminal.app on double-click.
# Starts the GCE VM service tunnel if not running, opens os-console,
# then kills the tunnel when the console exits.
#
# First run: right-click → Open → Open anyway (Gatekeeper prompt, one-time only)
# After that: double-click launches the console directly.
#
# Requires: ~/.ssh/config entry "foundry-services" with LocalForward lines
#           Run install.sh once to configure the SSH entry.

set -e

BINARY="$HOME/bin/os-console"

if [ ! -x "$BINARY" ]; then
    echo "os-console binary not found at $BINARY"
    echo "Run os-console/scripts/install.sh first."
    read -r -p "Press Enter to close..."
    exit 1
fi

TUNNEL_PID=""
if ! pgrep -f "ssh -N foundry-services" > /dev/null 2>&1; then
    ssh -N foundry-services &
    TUNNEL_PID=$!
    sleep 2
fi

"$BINARY"

if [ -n "$TUNNEL_PID" ]; then
    kill "$TUNNEL_PID" 2>/dev/null || true
fi
