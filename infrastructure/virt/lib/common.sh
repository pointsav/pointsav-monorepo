#!/usr/bin/env bash
# infrastructure/virt/lib/common.sh
# Shared shell functions sourced by provision-vm-*.sh scripts.
#
# VM-* / os-* correspondence:
#   VM-MediaKit     ← os-mediakit   (Phase 1: Ubuntu 24.04 QEMU)
#   VM-Totebox      ← os-totebox    (Phase 1: Ubuntu 24.04 QEMU)
#   VM-Orchestration← os-orchestration (Phase 1: Ubuntu 24.04 QEMU)
#   VM-PrivateGit   ← os-privategit (Phase 1: Ubuntu 24.04 QEMU)
#   VM-Infrastructure← os-infrastructure (host fleet; see provision-vm-infrastructure-*.sh)

set -euo pipefail

# Wait until SSH is responsive on a given port (default 10022).
# Usage: wait_for_ssh [port] [key] [timeout_s]
wait_for_ssh() {
    local port="${1:-10022}"
    local key="${2:-infrastructure/virt/work/foundry-vm-key}"
    local timeout="${3:-300}"
    local elapsed=0
    echo "Waiting for SSH on localhost:${port} (timeout ${timeout}s)..."
    while ! ssh -q -o StrictHostKeyChecking=no -o ConnectTimeout=5 \
              -i "${key}" -p "${port}" foundry@localhost true 2>/dev/null; do
        sleep 5
        elapsed=$((elapsed + 5))
        if [[ ${elapsed} -ge ${timeout} ]]; then
            echo "ERROR: SSH not available after ${timeout}s" >&2
            return 1
        fi
        echo "  still waiting (${elapsed}s)..."
    done
    echo "SSH ready after ${elapsed}s."
}

# Smoke-test a service via host-side port-forward.
# Usage: smoke_test <host_port> [timeout_s]
smoke_test() {
    local host_port="$1"
    local timeout="${2:-60}"
    echo "Smoke test: curl http://127.0.0.1:${host_port}/ (timeout ${timeout}s)..."
    local http_code
    http_code=$(curl --silent --output /dev/null --write-out '%{http_code}' \
        --max-time "${timeout}" "http://127.0.0.1:${host_port}/" 2>/dev/null) || true
    if [[ "${http_code}" =~ ^(200|301|302|404)$ ]]; then
        echo "  HTTP ${http_code} — service responding."
        return 0
    else
        echo "  WARNING: HTTP ${http_code} — service may not be ready (TCG latency expected)."
        return 0  # non-fatal on TCG
    fi
}

# Install and enable a guest systemd unit inside the VM.
# Usage: install_guest_unit <unit_file_path> <ssh_port> <key>
install_guest_unit() {
    local unit_file="$1"
    local port="${2:-10022}"
    local key="${3:-infrastructure/virt/work/foundry-vm-key}"
    local unit_name
    unit_name=$(basename "${unit_file}")
    echo "Installing guest unit: ${unit_name}"
    scp -P "${port}" -i "${key}" -o StrictHostKeyChecking=no \
        "${unit_file}" "foundry@localhost:/tmp/${unit_name}"
    ssh -p "${port}" -i "${key}" -o StrictHostKeyChecking=no foundry@localhost \
        "sudo mv /tmp/${unit_name} /etc/systemd/system/${unit_name} && \
         sudo systemctl daemon-reload && \
         sudo systemctl enable ${unit_name} && \
         sudo systemctl restart ${unit_name}"
}
