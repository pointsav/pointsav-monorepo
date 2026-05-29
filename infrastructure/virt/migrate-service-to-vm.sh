#!/usr/bin/env bash
# migrate-service-to-vm.sh — copy a service from the GCP host into vm-mediakit
#
# Usage:
#   ./migrate-service-to-vm.sh <service-name> <port>
#
# Examples:
#   ./migrate-service-to-vm.sh proofreader 9092
#   ./migrate-service-to-vm.sh knowledge-documentation 9090
#   ./migrate-service-to-vm.sh service-fs 9100
#
# The script copies:
#   1. The service binary   /usr/local/bin/<binary> → /opt/mediakit/bin/
#   2. The deployment data  ~/Foundry/deployments/<name>-*/ → /opt/mediakit/data/
#   3. The systemd unit     infrastructure/local-<name>/<unit>.service → /etc/systemd/system/
#   4. Enables + starts the unit, then smoke-tests via host port-forward
#
# Prerequisites:
#   vm-mediakit must be running (provision-vm-mediakit.sh)
#   SSH key: infrastructure/virt/work/foundry-vm-key
#
# Originals remain running on host throughout. No DNS changes.
# Only migrate one service at a time; verify before moving to the next.

set -euo pipefail

SERVICE="${1:-}"
PORT="${2:-}"

if [[ -z "$SERVICE" || -z "$PORT" ]]; then
    echo "usage: $0 <service-name> <port>"
    echo ""
    echo "Services and ports:"
    echo "  service-fs             9100   (WORM ledger — install first)"
    echo "  proofreader            9092   (internal only)"
    echo "  knowledge-documentation 9090"
    echo "  knowledge-projects     9093"
    echo "  knowledge-corporate    9095"
    echo "  marketing-pointsav     9101"
    echo "  marketing              9102"
    echo "  bim-orchestration      9096   (last — GIS data dep)"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
KEY="${SCRIPT_DIR}/work/foundry-vm-key"
SSH_OPTS="-p 10022 -i ${KEY} -o StrictHostKeyChecking=no -o ConnectTimeout=10"
SCP_OPTS="-P 10022 -i ${KEY} -o StrictHostKeyChecking=no -o ConnectTimeout=10"
VM="foundry@localhost"
HOST_PORT="1${PORT}"

# Service name → binary name mapping
declare -A BINARY_MAP
BINARY_MAP["proofreader"]="service-proofreader"
BINARY_MAP["knowledge-documentation"]="app-mediakit-knowledge"
BINARY_MAP["knowledge-corporate"]="app-mediakit-knowledge"
BINARY_MAP["knowledge-projects"]="app-mediakit-knowledge"
BINARY_MAP["marketing-pointsav"]="app-mediakit-marketing"
BINARY_MAP["marketing"]="app-mediakit-marketing"
BINARY_MAP["bim-orchestration"]="app-orchestration-bim"
BINARY_MAP["service-fs"]="service-fs"

BINARY="${BINARY_MAP[$SERVICE]:-$SERVICE}"
UNIT_NAME="local-${SERVICE}.service"
UNIT_DIR="${REPO_ROOT}/infrastructure/systemd"

echo "migrate-service-to-vm: migrating ${SERVICE} (binary: ${BINARY}, port: ${PORT})"
echo "migrate-service-to-vm: target host port: localhost:${HOST_PORT}"
echo ""

# --- Verify VM is reachable --------------------------------------------------

echo "[1/5] Verifying SSH to vm-mediakit..."
ssh $SSH_OPTS "$VM" "hostname" || {
    echo "ERROR: Cannot reach vm-mediakit. Is it running? Check provision-vm-mediakit.sh"
    exit 1
}

# --- Copy binary -------------------------------------------------------------

echo "[2/5] Copying binary ${BINARY}..."
BINARY_PATH="/usr/local/bin/${BINARY}"
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "WARNING: ${BINARY_PATH} not found on host — skipping binary copy"
    echo "         Install the binary first, then re-run this script"
else
    scp $SCP_OPTS "$BINARY_PATH" "${VM}:/tmp/${BINARY}"
    ssh $SSH_OPTS "$VM" "sudo mv /tmp/${BINARY} /opt/mediakit/bin/ && sudo chmod +x /opt/mediakit/bin/${BINARY}"
    echo "  → /opt/mediakit/bin/${BINARY}"
fi

# --- Copy deployment data + content directories ------------------------------

echo "[3/5] Copying deployment data..."

# Service-specific content directory transfers
case "$SERVICE" in
    knowledge-documentation)
        echo "  copy: content-wiki-documentation (~20M, tar pipe)..."
        ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/content-wiki-documentation /opt/mediakit/data/knowledge"
        tar -czf - --exclude='.git' --exclude='target' \
            -C /srv/foundry/clones/project-knowledge/content-wiki-documentation . \
            | ssh $SSH_OPTS "$VM" "tar -xzf - -C /opt/mediakit/data/content-wiki-documentation/"
        scp $SCP_OPTS /srv/foundry/citations.yaml "${VM}:/opt/mediakit/data/content-wiki-documentation/citations.yaml"
        echo "  → /opt/mediakit/data/content-wiki-documentation/"
        FOUND_DEPLOY=1
        ;;
    knowledge-corporate)
        echo "  copy: content-wiki-corporate (~4M, tar pipe)..."
        ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/content-wiki-corporate /opt/mediakit/data/knowledge"
        tar -czf - --exclude='.git' \
            -C /srv/foundry/customer/content-wiki-corporate . \
            | ssh $SSH_OPTS "$VM" "tar -xzf - -C /opt/mediakit/data/content-wiki-corporate/"
        echo "  → /opt/mediakit/data/content-wiki-corporate/"
        FOUND_DEPLOY=1
        ;;
    knowledge-projects)
        echo "  copy: content-wiki-projects (~4M, tar pipe)..."
        ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/content-wiki-projects /opt/mediakit/data/knowledge"
        tar -czf - --exclude='.git' \
            -C /srv/foundry/customer/content-wiki-projects . \
            | ssh $SSH_OPTS "$VM" "tar -xzf - -C /opt/mediakit/data/content-wiki-projects/"
        echo "  → /opt/mediakit/data/content-wiki-projects/"
        FOUND_DEPLOY=1
        ;;
    marketing-pointsav)
        echo "  copy: marketing-landing-2 (pointsav, ~2.5M, tar pipe)..."
        ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/marketing-pointsav"
        tar -czf - --exclude='.git' \
            -C ~/Foundry/deployments/media-marketing-landing-2 . \
            | ssh $SSH_OPTS "$VM" "tar -xzf - -C /opt/mediakit/data/marketing-pointsav/"
        echo "  → /opt/mediakit/data/marketing-pointsav/"
        FOUND_DEPLOY=1
        ;;
    marketing)
        echo "  copy: marketing-landing-1 (woodfine, ~2.5M, tar pipe)..."
        ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/marketing-woodfine"
        tar -czf - --exclude='.git' \
            -C ~/Foundry/deployments/media-marketing-landing-1 . \
            | ssh $SSH_OPTS "$VM" "tar -xzf - -C /opt/mediakit/data/marketing-woodfine/"
        echo "  → /opt/mediakit/data/marketing-woodfine/"
        FOUND_DEPLOY=1
        ;;
    *)
        DEPLOY_DIRS=( ~/Foundry/deployments/media-${SERVICE}-* ~/Foundry/deployments/${SERVICE}-* )
        FOUND_DEPLOY=0
        for DEPLOY_DIR in "${DEPLOY_DIRS[@]}"; do
            if [[ -d "$DEPLOY_DIR" ]]; then
                DEPLOY_NAME="$(basename "$DEPLOY_DIR")"
                ssh $SSH_OPTS "$VM" "mkdir -p /opt/mediakit/data/${DEPLOY_NAME}"
                scp -r $SSH_OPTS "$DEPLOY_DIR/" "${VM}:/opt/mediakit/data/${DEPLOY_NAME}/"
                echo "  → /opt/mediakit/data/${DEPLOY_NAME}/"
                FOUND_DEPLOY=1
                break
            fi
        done
        if [[ "$FOUND_DEPLOY" -eq 0 ]]; then
            echo "  (no deployment data directory found for ${SERVICE} — continuing)"
        fi
        ;;
esac

# --- Copy systemd unit -------------------------------------------------------

echo "[4/5] Installing systemd unit..."
UNIT_PATH="${UNIT_DIR}/${UNIT_NAME}"
if [[ ! -f "$UNIT_PATH" ]]; then
    echo "WARNING: ${UNIT_PATH} not found — skipping unit install"
    echo "         Create the unit file, then re-run this script"
else
    # Adapt unit: replace /usr/local/bin with /opt/mediakit/bin
    # Extract WorkingDirectory from unit file and create it in the VM
    WORK_DIR=$(grep '^WorkingDirectory=' "$UNIT_PATH" | cut -d= -f2-)
    if [[ -n "$WORK_DIR" ]]; then
        ssh $SSH_OPTS "$VM" "mkdir -p ${WORK_DIR} && chown foundry:foundry ${WORK_DIR}"
    fi
    # Adapt unit: replace /usr/local/bin with /opt/mediakit/bin
    ADAPTED_UNIT=$(sed "s|/usr/local/bin/${BINARY}|/opt/mediakit/bin/${BINARY}|g" "$UNIT_PATH")
    echo "$ADAPTED_UNIT" | ssh $SSH_OPTS "$VM" "sudo tee /etc/systemd/system/${UNIT_NAME} > /dev/null"
    ssh $SSH_OPTS "$VM" "sudo systemctl daemon-reload && sudo systemctl enable --now ${UNIT_NAME}"
    echo "  → /etc/systemd/system/${UNIT_NAME} (enabled + started)"
fi

# --- Smoke test --------------------------------------------------------------

echo "[5/5] Smoke test (localhost:${HOST_PORT})..."
sleep 2
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "http://localhost:${HOST_PORT}/" 2>/dev/null)
if [[ "$HTTP_STATUS" == "200" ]]; then
    echo "  ✓ localhost:${HOST_PORT}/ → HTTP 200"
elif [[ "$HTTP_STATUS" != "000" ]]; then
    echo "  ✓ localhost:${HOST_PORT}/ → HTTP ${HTTP_STATUS} (service is responding)"
else
    echo "  ! localhost:${HOST_PORT}/ — no response (service may need configuration)"
    echo "  Check: ssh -p 10022 -i ${KEY} foundry@localhost 'systemctl status ${UNIT_NAME}'"
    echo "  Check: ssh -p 10022 -i ${KEY} foundry@localhost 'journalctl -u ${UNIT_NAME} -n 20'"
fi

echo ""
echo "migrate-service-to-vm: ${SERVICE} migration complete"
echo "Original on host still running — verify VM version before any DNS change"
