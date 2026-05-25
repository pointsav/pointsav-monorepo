#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# bin/promote-corpus.sh — F12 corpus promotion gate (SYS-ADR-10)
#
# Promotes accepted, verdict-signed apprenticeship corpus tuples to
# stage_at_capture: "training-eligible".  Requires an SSH signature over
# the promotion manifest (namespace: corpus-promote-v1).
#
# This is the ONLY path that sets stage_at_capture = "training-eligible".
# Running the LoRA training toolchain (bin/lora-update.sh) without a
# promote-corpus.sh pass first is a SYS-ADR-10 violation.
#
# Usage:
#   FOUNDRY_ROOT=/srv/foundry ./bin/promote-corpus.sh [--dry-run]
#
# Optional env:
#   FOUNDRY_ROOT      workspace root   (default: /srv/foundry)
#   CORPUS_ROOT       data root        (default: $FOUNDRY_ROOT/data)
#   SENIOR_IDENTITY   GitHub identity  (default: jwoodfine)

set -euo pipefail

FOUNDRY_ROOT="${FOUNDRY_ROOT:-/srv/foundry}"
CORPUS_ROOT="${CORPUS_ROOT:-${FOUNDRY_ROOT}/data}"
NAMESPACE="corpus-promote-v1"
ALLOWED_SIGNERS="${FOUNDRY_ROOT}/identity/allowed_signers"
SENIOR_IDENTITY="${SENIOR_IDENTITY:-jwoodfine}"
DRY_RUN=0

for arg in "$@"; do
    case "${arg}" in
        --dry-run) DRY_RUN=1 ;;
        *) echo "Unknown argument: ${arg}" >&2; exit 1 ;;
    esac
done

if ! command -v jq &>/dev/null; then
    echo "ERROR: jq is required." >&2; exit 1
fi

APPRENTICESHIP_DIR="${CORPUS_ROOT}/training-corpus/apprenticeship"
if [[ ! -d "${APPRENTICESHIP_DIR}" ]]; then
    echo "No apprenticeship corpus at ${APPRENTICESHIP_DIR}" >&2; exit 0
fi

echo "=== F12 Corpus Promotion Gate (SYS-ADR-10) ==="
echo "Corpus: ${APPRENTICESHIP_DIR}"
echo ""

MANIFEST="$(mktemp /tmp/corpus-f12-manifest-XXXX.txt)"
trap 'rm -f "${MANIFEST}" "${MANIFEST}.sig"' EXIT

ELIGIBLE=0
while IFS= read -r -d '' tuple_file; do
    verdict=$(jq -r '.verdict.verdict // "null"' "${tuple_file}" 2>/dev/null || echo "null")
    stage=$(jq -r '.stage_at_capture // "null"' "${tuple_file}" 2>/dev/null || echo "null")
    promoted_at=$(jq -r '.promoted_at // "null"' "${tuple_file}" 2>/dev/null || echo "null")
    if [[ "${verdict}" == "accept" \
        && "${stage}" != "training-eligible" \
        && "${promoted_at}" != "null" ]]; then
        echo "${tuple_file}" >> "${MANIFEST}"
        task_type="$(basename "$(dirname "${tuple_file}")")"
        brief_id=$(jq -r '.brief.brief_id // "(unknown)"' "${tuple_file}" 2>/dev/null || echo "(unknown)")
        echo "  ELIGIBLE  ${task_type} / $(basename "${tuple_file}") [brief=${brief_id}]"
        ELIGIBLE=$((ELIGIBLE + 1))
    fi
done < <(find "${APPRENTICESHIP_DIR}" -name 'shadow-*.jsonl' -print0 2>/dev/null)

if [[ "${ELIGIBLE}" -eq 0 ]]; then
    echo "No eligible tuples found (need: verdict=accept, promoted_at set, stage≠training-eligible)."
    exit 0
fi

echo ""
echo "${ELIGIBLE} tuple(s) eligible for F12 promotion."

if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "[DRY-RUN] Nothing modified."
    exit 0
fi

echo ""
echo "Sign the manifest to authorise promotion:"
echo ""
echo "  ssh-keygen -Y sign \\"
echo "    -f ~/Foundry/identity/${SENIOR_IDENTITY}/id_${SENIOR_IDENTITY} \\"
echo "    -n ${NAMESPACE} \\"
echo "    < ${MANIFEST} > ${MANIFEST}.sig"
echo ""
printf "Press Enter after signing... "
read -r

if [[ ! -s "${MANIFEST}.sig" ]]; then
    echo "ERROR: Signature file empty or missing: ${MANIFEST}.sig" >&2; exit 1
fi

PRINCIPAL="${SENIOR_IDENTITY}@users.noreply.github.com"
if ! ssh-keygen -Y verify \
        -f "${ALLOWED_SIGNERS}" \
        -I "${PRINCIPAL}" \
        -n "${NAMESPACE}" \
        -s "${MANIFEST}.sig" \
        < "${MANIFEST}"; then
    echo "ERROR: Signature verification failed." >&2; exit 1
fi

echo "Signature verified. Promoting ${ELIGIBLE} tuple(s)..."
NOW="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
PROMOTED=0
while IFS= read -r tuple_file; do
    jq --arg now "${NOW}" \
       '.stage_at_capture = "training-eligible" | .f12_promoted_at = $now' \
       "${tuple_file}" > "${tuple_file}.tmp" \
    && mv "${tuple_file}.tmp" "${tuple_file}"
    echo "  PROMOTED  $(basename "${tuple_file}")"
    PROMOTED=$((PROMOTED + 1))
done < "${MANIFEST}"

LOG="${CORPUS_ROOT}/training-corpus/f12-promotions.jsonl"
jq -nc \
    --arg ts "${NOW}" \
    --arg identity "${PRINCIPAL}" \
    --argjson n "${PROMOTED}" \
    '{ts: $ts, signed_by: $identity, promoted_count: $n}' \
    >> "${LOG}"

echo ""
echo "F12 gate complete: ${PROMOTED} tuple(s) marked training-eligible."
echo "Log: ${LOG}"
