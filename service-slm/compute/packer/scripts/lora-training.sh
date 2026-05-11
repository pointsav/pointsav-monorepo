#!/usr/bin/env bash
# Yo-Yo LoRA training trigger.
#
# Watches /srv/foundry/data/training-pending/ for *.json marker files dropped
# by the workspace's corpus-threshold.py. When a marker lands, runs QLoRA via
# accelerate against the corpus referenced in the marker, writes adapter
# artifacts to /data/weights/adapters/<tenant>/<role>/v<n>/, and triggers
# adapter-publish.service to upload the result to GCS.
#
# This unit is **disabled by default**. Master ratifies the first task-type
# promotion (Doctrine claim around apprenticeship + LoRA training threshold —
# currently a Doctrine gap surfaced separately to operator/Master). When that
# happens, `systemctl enable --now lora-training.service` activates the loop.
#
# Until activated, this script does not exist on the runtime path. The unit
# file is present so promotion is a single systemctl command, not an image
# rebuild.

set -euo pipefail

PENDING_DIR="${PENDING_DIR:-/srv/foundry/data/training-pending}"
ADAPTER_BASE="${ADAPTER_BASE:-/data/weights/adapters}"
LOG_FILE="/var/log/yoyo-lora-training.log"
BASE_MODEL="${BASE_MODEL:-/data/weights/olmo-3-32b-think-q4.gguf}"

mkdir -p "${ADAPTER_BASE}"
exec >> >(tee -a "${LOG_FILE}") 2>&1

log() { echo "[lora-training $(date -u +'%Y-%m-%dT%H:%M:%SZ')] $*"; }

log "Lora training watcher started. Pending dir: ${PENDING_DIR}"

while true; do
    # Skip if pending dir doesn't exist yet (workspace mount issue, etc.)
    if [[ ! -d "${PENDING_DIR}" ]]; then
        log "WARN: ${PENDING_DIR} absent. Sleeping 60s before retry."
        sleep 60
        continue
    fi

    # Find oldest unclaimed marker
    marker=$(find "${PENDING_DIR}" -maxdepth 1 -name '*.json' -not -name '*.claimed' \
        -printf '%T@ %p\n' 2>/dev/null | sort -n | head -1 | cut -d' ' -f2)

    if [[ -z "${marker}" ]]; then
        sleep 30
        continue
    fi

    # Claim it (atomic rename)
    claimed="${marker}.claimed"
    mv "${marker}" "${claimed}"
    log "Claimed marker: $(basename "${claimed}")"

    # Parse marker: { tenant, role, corpus_path, method (sft|dpo), tuple_count, version }
    tenant=$(jq -r '.tenant // empty' "${claimed}")
    role=$(jq -r '.role // empty' "${claimed}")
    corpus_path=$(jq -r '.corpus_path // empty' "${claimed}")
    method=$(jq -r '.method // "sft"' "${claimed}")
    version=$(jq -r '.version // 1' "${claimed}")

    if [[ -z "${tenant}" || -z "${role}" || -z "${corpus_path}" ]]; then
        log "ERROR: marker malformed. Skipping. Contents:"
        cat "${claimed}"
        mv "${claimed}" "${claimed}.invalid"
        continue
    fi

    out_dir="${ADAPTER_BASE}/${tenant}/${role}/v${version}"
    mkdir -p "${out_dir}"

    log "Training: tenant=${tenant} role=${role} method=${method} corpus=${corpus_path} → ${out_dir}"

    # Activate the venv that has torch + peft + accelerate
    source /opt/vllm/bin/activate

    # Run the training (PLACEHOLDER — actual accelerate invocation lands when
    # apprenticeship/corpus paths are finalized; for now this is a stub that
    # writes a placeholder adapter and exits with informative log).
    case "${method}" in
        sft|dpo)
            log "STUB: would run accelerate train ${method} on ${corpus_path}"
            log "STUB: writing placeholder adapter at ${out_dir}/adapter_config.json"
            cat > "${out_dir}/adapter_config.json" <<EOF
{
    "stub": true,
    "tenant": "${tenant}",
    "role": "${role}",
    "method": "${method}",
    "corpus_path": "${corpus_path}",
    "trained_at": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
    "note": "Real training pipeline lands when Master ratifies Yo-Yo-runs-LoRA + corpus consumer is finalized."
}
EOF
            ;;
        *)
            log "ERROR: unknown method '${method}' (expected sft|dpo)"
            mv "${claimed}" "${claimed}.invalid"
            continue
            ;;
    esac

    # Trigger adapter-publish to upload to GCS
    log "Triggering adapter-publish.service for ${out_dir}..."
    ADAPTER_OUT_DIR="${out_dir}" systemctl start adapter-publish.service || \
        log "WARN: adapter-publish trigger failed (unit may need to be configured)"

    # Mark marker as completed
    mv "${claimed}" "${claimed}.completed"
    log "Adapter v${version} for ${tenant}/${role} ready at ${out_dir}"
done
