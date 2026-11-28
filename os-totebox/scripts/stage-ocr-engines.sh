#!/usr/bin/env bash
# SPDX-License-Identifier: FSL-1.1-ALv2
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# stage-ocr-engines.sh — install one OCR reader (a llama.cpp `llama-server`
# serving a vision model + mmproj) as a loopback systemd unit on the target host,
# for the Doorman's `ocr-openai-vision` engine kind.
#
# What it does NOT do: it never touches local-slm.service, local-totebox.service
# or the Doorman. It does not edit the engine registry; it prints the registry
# entry to add. DRY-RUN BY DEFAULT: prints every step and changes nothing.
# `--apply` is required to act, and also requires OCR_LICENCE_CLEARED=<licence
# id the operator verified> — an unknown licence blocks install.
#
# CPU pinning is enforced HERE (CPUAffinity=/Nice= in the unit); the Doorman
# only declares and reports `cpu_set`/`nice` from its registry. Keep the two
# in agreement. OLMo (local-slm) is left unpinned and keeps priority.
# Default port is 8082: a hand-started llama-server may already be running
# on 127.0.0.1:8081; do not collide with it. To adopt that server
# instead of installing a unit, point the registry entry at 127.0.0.1:8081.
#
# Usage:
#   OCR_LICENCE_CLEARED=<id> bash scripts/stage-ocr-engines.sh <instance> \
#       --engine-id lighton-ocr-2-1b-q8 --model /path/model.gguf \
#       --mmproj /path/mmproj.gguf --model-sha256 <hex> --mmproj-sha256 <hex> \
#       [--zone us-west1-a] [--port 8082] [--cpu-set 0-4] [--nice 19] \
#       [--ctx 8192] [--apply]

set -euo pipefail

usage() { sed -n '2,/^set -euo/p' "$0" | sed 's/^# \{0,1\}//' | head -n -1; exit 2; }

TARGET="${1:-}"; [ -n "${TARGET}" ] || usage
shift
ZONE="us-west1-a"; PORT="8082"; CPU_SET="0-4"; NICE="19"; CTX="8192"; APPLY=0
ENGINE_ID=""; MODEL=""; MMPROJ=""; MODEL_SHA=""; MMPROJ_SHA=""
while [ $# -gt 0 ]; do
    case "$1" in
        --engine-id) ENGINE_ID="$2"; shift 2 ;;
        --model) MODEL="$2"; shift 2 ;;
        --mmproj) MMPROJ="$2"; shift 2 ;;
        --model-sha256) MODEL_SHA="$2"; shift 2 ;;
        --mmproj-sha256) MMPROJ_SHA="$2"; shift 2 ;;
        --zone) ZONE="$2"; shift 2 ;;
        --port) PORT="$2"; shift 2 ;;
        --cpu-set) CPU_SET="$2"; shift 2 ;;
        --nice) NICE="$2"; shift 2 ;;
        --ctx) CTX="$2"; shift 2 ;;
        --apply) APPLY=1; shift ;;
        *) echo "unknown argument: $1"; usage ;;
    esac
done

for v in ENGINE_ID MODEL MMPROJ MODEL_SHA MMPROJ_SHA; do
    [ -n "${!v}" ] || { echo "error: --$(echo "${v}" | tr 'A-Z_' 'a-z-') is required"; exit 2; }
done
[[ "${ENGINE_ID}" =~ ^[a-z0-9][a-z0-9.-]{0,62}$ ]] || { echo "error: engine-id must be lowercase [a-z0-9.-]"; exit 2; }
[[ "${PORT}" =~ ^[0-9]+$ ]] || { echo "error: port must be numeric"; exit 2; }
[[ "${CPU_SET}" =~ ^[0-9,-]+$ ]] || { echo "error: cpu-set must look like 0-4 or 0,2"; exit 2; }
[[ "${NICE}" =~ ^-?[0-9]+$ ]] || { echo "error: nice must be an integer"; exit 2; }
[ -f "${MODEL}" ] && [ -f "${MMPROJ}" ] || { echo "error: model/mmproj file not found"; exit 1; }

echo "== verifying local weight checksums =="
[ "$(sha256sum "${MODEL}" | cut -d' ' -f1)" = "${MODEL_SHA}" ] || { echo "error: model sha256 mismatch"; exit 1; }
[ "$(sha256sum "${MMPROJ}" | cut -d' ' -f1)" = "${MMPROJ_SHA}" ] || { echo "error: mmproj sha256 mismatch"; exit 1; }

UNIT="local-ocr-${ENGINE_ID}.service"
DEST="/var/lib/os-totebox-engines/${ENGINE_ID}"
gssh() { gcloud compute ssh "${TARGET}" --zone="${ZONE}" --internal-ip --command "$1"; }
gscp() { gcloud compute scp "$1" "${TARGET}:$2" --zone="${ZONE}" --internal-ip; }

UNIT_TEXT="[Unit]
Description=OCR reader ${ENGINE_ID} (llama-server, loopback only) for the Doorman ocr-openai-vision engine
After=network.target

[Service]
Type=simple
User=@USER@
ExecStart=@LLAMA@ -m ${DEST}/model.gguf --mmproj ${DEST}/mmproj.gguf --host 127.0.0.1 --port ${PORT} -c ${CTX} --parallel 1 --alias ${ENGINE_ID}
CPUAffinity=${CPU_SET}
Nice=${NICE}
Restart=on-failure
RestartSec=5
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadOnlyPaths=${DEST}

[Install]
WantedBy=multi-user.target"

REGISTRY_ENTRY="{\"engine_id\":\"${ENGINE_ID}\",\"kind\":\"ocr-openai-vision\",\"endpoint\":\"http://127.0.0.1:${PORT}\",\"model\":\"${ENGINE_ID}\",\"weights_sha256\":\"${MODEL_SHA}\",\"licence\":\"${OCR_LICENCE_CLEARED:-UNVERIFIED}\",\"permits\":1,\"cpu_set\":\"${CPU_SET}\",\"nice\":${NICE},\"max_job_secs\":900,\"max_input_bytes\":20971520}"

if [ "${APPLY}" -ne 1 ]; then
    echo "== DRY RUN (nothing changed). Would, on ${TARGET} (${ZONE}):"
    echo "   1. copy model.gguf + mmproj.gguf to ${DEST}/ (root-owned, world-readable: the unit runs as the local-slm user)"
    echo "   2. read llama-server path and User= from 'systemctl cat local-slm'"
    echo "   3. install /etc/systemd/system/${UNIT}:"
    echo "${UNIT_TEXT}" | sed 's/^/        /'
    echo "   4. systemctl daemon-reload; enable --now ${UNIT}; curl 127.0.0.1:${PORT}/health"
    echo "   5. NOT restart the Doorman; add this entry to the engine registry yourself:"
    echo "        ${REGISTRY_ENTRY}"
    echo "Re-run with --apply and OCR_LICENCE_CLEARED=<licence id> to act."
    exit 0
fi

[ -n "${OCR_LICENCE_CLEARED:-}" ] || { echo "error: OCR_LICENCE_CLEARED not set — an unknown licence blocks install"; exit 1; }

echo "== transferring weights =="
gscp "${MODEL}" "/tmp/${ENGINE_ID}-model.gguf"
gscp "${MMPROJ}" "/tmp/${ENGINE_ID}-mmproj.gguf"
echo "== installing weights =="
gssh "sudo install -d -m 0755 -o root -g root ${DEST} && sudo install -m 0644 /tmp/${ENGINE_ID}-model.gguf ${DEST}/model.gguf && sudo install -m 0644 /tmp/${ENGINE_ID}-mmproj.gguf ${DEST}/mmproj.gguf && rm -f /tmp/${ENGINE_ID}-model.gguf /tmp/${ENGINE_ID}-mmproj.gguf"
echo "== verifying remote checksums =="
[ "$(gssh "sudo sha256sum ${DEST}/model.gguf" | cut -d' ' -f1)" = "${MODEL_SHA}" ] || { echo "error: remote model sha256 mismatch"; exit 1; }
[ "$(gssh "sudo sha256sum ${DEST}/mmproj.gguf" | cut -d' ' -f1)" = "${MMPROJ_SHA}" ] || { echo "error: remote mmproj sha256 mismatch"; exit 1; }

echo "== deriving llama-server path and service user from local-slm.service =="
LLAMA="$(gssh "systemctl cat local-slm.service | sed -n 's/^ExecStart=\\([^ ]*\\).*/\\1/p' | head -1" | tr -d '\r')"
SVC_USER="$(gssh "systemctl cat local-slm.service | sed -n 's/^User=//p' | head -1" | tr -d '\r')"
[ -n "${LLAMA}" ] || { echo "error: could not read llama-server path from local-slm.service"; exit 1; }
[ -n "${SVC_USER}" ] || SVC_USER="nobody"

echo "== installing ${UNIT} =="
echo "${UNIT_TEXT}" | sed "s|@USER@|${SVC_USER}|; s|@LLAMA@|${LLAMA}|" | gssh "sudo tee /etc/systemd/system/${UNIT} >/dev/null"
gssh "sudo systemctl daemon-reload && sudo systemctl enable --now ${UNIT}"

echo "== waiting for the reader to answer /health =="
for i in $(seq 1 60); do
    if gssh "curl -sf -m 3 http://127.0.0.1:${PORT}/health >/dev/null 2>&1"; then echo "   healthy after ${i} tries"; break; fi
    sleep 2
    [ "${i}" -eq 60 ] && { echo "error: ${UNIT} did not become healthy — check: journalctl -u ${UNIT}"; exit 1; }
done

echo "== done. Add this entry to the engine registry (the Doorman is NOT restarted by this script):"
echo "${REGISTRY_ENTRY}"
