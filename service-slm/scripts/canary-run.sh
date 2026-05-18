#!/usr/bin/env bash
# canary-run.sh — Run the canary task set against the running Doorman.
#
# Phase 3 (P3-3.2) of learning-loop-master-plan-2026-05-18.md.
#
# Loads `service-slm/data/canary/v1.yaml`, sends each task's prompt to
# the Doorman's /v1/messages endpoint, scores the response against the
# task's `contains` + `not_contains` patterns, and writes results to
# `data/canary/results/<adapter_version>-<date>.json`.
#
# Used by:
# - Operator regression check before promoting a new adapter
# - Weekly nightly-run.sh extension (future) for continuous tracking
#
# Usage:
#   ./canary-run.sh [--endpoint=http://127.0.0.1:9090]
#                   [--adapter-version=coding-lora-2026-05-18]
#                   [--task-set=service-slm/data/canary/v1.yaml]
#                   [--out=data/canary/results/<auto>.json]
#                   [--dry-run] [--strict]
#
# Exit codes:
#   0  All tasks passed (or non-strict mode regardless of result)
#   1  Argument error
#   2  Task set file not found
#   3  Required tool missing (jq, curl, yq)
#   4  Doorman unreachable
#   5  (--strict only) Any task failed below promote threshold

set -euo pipefail

ENDPOINT="${SLM_CANARY_ENDPOINT:-http://127.0.0.1:9090}"
ADAPTER_VERSION="${SLM_CANARY_ADAPTER:-baseline}"
FOUNDRY_ROOT="${FOUNDRY_ROOT:-/srv/foundry}"
ARCHIVE_ROOT="${FOUNDRY_ROOT}/clones/project-intelligence"
TASK_SET="${ARCHIVE_ROOT}/service-slm/data/canary/v1.yaml"
DATE_STAMP="$(date -u +%Y-%m-%dT%H%M%SZ)"
OUT_PATH=""
DRY_RUN=0
STRICT=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --endpoint=*) ENDPOINT="${1#--endpoint=}"; shift ;;
        --adapter-version=*) ADAPTER_VERSION="${1#--adapter-version=}"; shift ;;
        --task-set=*) TASK_SET="${1#--task-set=}"; shift ;;
        --out=*) OUT_PATH="${1#--out=}"; shift ;;
        --dry-run) DRY_RUN=1; shift ;;
        --strict) STRICT=1; shift ;;
        --help|-h) sed -n '2,30p' "$0"; exit 0 ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "${OUT_PATH}" ]]; then
    OUT_PATH="${FOUNDRY_ROOT}/data/canary/results/${ADAPTER_VERSION}-${DATE_STAMP}.json"
fi

for tool in jq curl yq; do
    command -v "${tool}" >/dev/null 2>&1 || {
        echo "ERROR: required tool not installed: ${tool}" >&2
        exit 3
    }
done

[[ -f "${TASK_SET}" ]] || {
    echo "ERROR: task set not found: ${TASK_SET}" >&2
    exit 2
}

# Health check the Doorman first
if ! curl -sS --max-time 5 "${ENDPOINT}/healthz" >/dev/null 2>&1; then
    echo "ERROR: Doorman unreachable at ${ENDPOINT}/healthz" >&2
    exit 4
fi

mkdir -p "$(dirname "${OUT_PATH}")"

TASK_COUNT="$(yq '.tasks | length' "${TASK_SET}")"
echo "[$(date -u +%H:%M:%S)] canary-run: ${TASK_COUNT} tasks against ${ENDPOINT}"
echo "                       adapter_version=${ADAPTER_VERSION}"
echo "                       task_set=${TASK_SET}"
echo "                       out=${OUT_PATH}"

PROMOTE_THRESHOLD="$(yq '.scoring.promote_threshold' "${TASK_SET}")"
FLAG_THRESHOLD="$(yq '.scoring.flag_threshold' "${TASK_SET}")"

if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo ""
    echo "DRY RUN — first task:"
    yq '.tasks[0]' "${TASK_SET}"
    echo "(no requests sent; no output written)"
    exit 0
fi

declare -a RESULTS=()
TOTAL_PASS=0
TOTAL_FAIL=0
declare -A CAT_PASS=()
declare -A CAT_FAIL=()

for i in $(seq 0 $((TASK_COUNT - 1))); do
    TASK_ID="$(yq ".tasks[${i}].id" "${TASK_SET}")"
    CATEGORY="$(yq ".tasks[${i}].category" "${TASK_SET}")"
    PROMPT="$(yq ".tasks[${i}].prompt" "${TASK_SET}")"
    MAX_TOKENS="$(yq ".tasks[${i}].max_tokens" "${TASK_SET}")"
    TIMEOUT_SEC="$(yq ".tasks[${i}].timeout_sec" "${TASK_SET}")"
    CONTAINS_JSON="$(yq -o=json ".tasks[${i}].contains" "${TASK_SET}")"
    NOT_CONTAINS_JSON="$(yq -o=json ".tasks[${i}].not_contains" "${TASK_SET}")"

    REQ_BODY="$(jq -nc \
        --arg model "${ADAPTER_VERSION}" \
        --arg prompt "${PROMPT}" \
        --argjson max_tokens "${MAX_TOKENS}" \
        '{
            model: $model,
            messages: [{role: "user", content: $prompt}],
            max_tokens: $max_tokens,
            stream: false
        }')"

    START_NS="$(date +%s%N)"
    RESPONSE="$(curl -sS --max-time "${TIMEOUT_SEC}" \
        -X POST "${ENDPOINT}/v1/messages" \
        -H 'content-type: application/json' \
        -d "${REQ_BODY}" 2>/dev/null || echo '{"_canary_timeout": true}')"
    END_NS="$(date +%s%N)"
    LATENCY_MS=$(((END_NS - START_NS) / 1000000))

    if echo "${RESPONSE}" | jq -e '._canary_timeout' >/dev/null 2>&1; then
        STATUS="timeout"
        CONTENT=""
    elif echo "${RESPONSE}" | jq -e '.error' >/dev/null 2>&1; then
        STATUS="error"
        CONTENT="$(echo "${RESPONSE}" | jq -rc '.error')"
    else
        STATUS="ok"
        CONTENT="$(echo "${RESPONSE}" | jq -r '.content[0].text // .content // ""' 2>/dev/null || echo "")"
    fi

    PASSED=1
    if [[ "${STATUS}" != "ok" ]]; then
        PASSED=0
    else
        CONTENT_LC="$(echo "${CONTENT}" | tr '[:upper:]' '[:lower:]')"
        for needle in $(echo "${CONTAINS_JSON}" | jq -r '.[]'); do
            needle_lc="$(echo "${needle}" | tr '[:upper:]' '[:lower:]')"
            if [[ "${CONTENT_LC}" != *"${needle_lc}"* ]]; then
                PASSED=0
                break
            fi
        done
        if [[ "${PASSED}" -eq 1 ]]; then
            for forbidden in $(echo "${NOT_CONTAINS_JSON}" | jq -r '.[]'); do
                forbidden_lc="$(echo "${forbidden}" | tr '[:upper:]' '[:lower:]')"
                if [[ "${CONTENT_LC}" == *"${forbidden_lc}"* ]]; then
                    PASSED=0
                    break
                fi
            done
        fi
    fi

    if [[ "${PASSED}" -eq 1 ]]; then
        SYMBOL="PASS"
        TOTAL_PASS=$((TOTAL_PASS + 1))
        CAT_PASS[${CATEGORY}]=$((${CAT_PASS[${CATEGORY}]:-0} + 1))
    else
        SYMBOL="FAIL"
        TOTAL_FAIL=$((TOTAL_FAIL + 1))
        CAT_FAIL[${CATEGORY}]=$((${CAT_FAIL[${CATEGORY}]:-0} + 1))
    fi

    printf "  %-40s %-22s %s (%dms)\n" "${TASK_ID}" "${CATEGORY}" "${SYMBOL}" "${LATENCY_MS}"

    RESULT_LINE="$(jq -nc \
        --arg task_id "${TASK_ID}" \
        --arg category "${CATEGORY}" \
        --arg status "${STATUS}" \
        --arg content "${CONTENT}" \
        --argjson passed $([ "${PASSED}" -eq 1 ] && echo "true" || echo "false") \
        --argjson latency_ms "${LATENCY_MS}" \
        '{task_id: $task_id, category: $category, status: $status, passed: $passed, latency_ms: $latency_ms, content: $content}')"
    RESULTS+=("${RESULT_LINE}")
done

PASS_RATE="$(awk -v p="${TOTAL_PASS}" -v t="${TASK_COUNT}" 'BEGIN { printf "%.4f", p/t }')"

BY_CATEGORY="{}"
for cat in "${!CAT_PASS[@]}" "${!CAT_FAIL[@]}"; do
    [[ -z "${BY_CATEGORY##*\"${cat}\"*}" ]] && continue
    p="${CAT_PASS[${cat}]:-0}"
    f="${CAT_FAIL[${cat}]:-0}"
    total=$((p + f))
    rate="$(awk -v p="${p}" -v t="${total}" 'BEGIN { printf "%.4f", p/t }')"
    BY_CATEGORY="$(echo "${BY_CATEGORY}" | jq --arg c "${cat}" --argjson p "${p}" --argjson f "${f}" --argjson r "${rate}" '. + {($c): {pass: $p, fail: $f, rate: $r}}')"
done

REPORT="$(jq -nc \
    --arg adapter "${ADAPTER_VERSION}" \
    --arg ts "${DATE_STAMP}" \
    --arg endpoint "${ENDPOINT}" \
    --argjson task_count "${TASK_COUNT}" \
    --argjson pass "${TOTAL_PASS}" \
    --argjson fail "${TOTAL_FAIL}" \
    --argjson rate "${PASS_RATE}" \
    --argjson promote_threshold "${PROMOTE_THRESHOLD}" \
    --argjson flag_threshold "${FLAG_THRESHOLD}" \
    --argjson by_category "${BY_CATEGORY}" \
    --argjson results "[$(IFS=,; echo "${RESULTS[*]}")]" \
    '{
        adapter_version: $adapter,
        ran_at_utc: $ts,
        endpoint: $endpoint,
        task_count: $task_count,
        pass: $pass,
        fail: $fail,
        pass_rate: $rate,
        promote_threshold: $promote_threshold,
        flag_threshold: $flag_threshold,
        by_category: $by_category,
        results: $results
    }')"

echo "${REPORT}" | jq . > "${OUT_PATH}"
echo ""
echo "[$(date -u +%H:%M:%S)] canary-run summary:"
echo "  task_count:        ${TASK_COUNT}"
echo "  pass:              ${TOTAL_PASS} / ${TASK_COUNT}  (rate ${PASS_RATE})"
echo "  fail:              ${TOTAL_FAIL}"
echo "  promote threshold: ${PROMOTE_THRESHOLD}"
echo "  out:               ${OUT_PATH}"

PASS_RATE_AS_INT="$(awk -v r="${PASS_RATE}" 'BEGIN { print int(r * 100) }')"
PROMOTE_AS_INT="$(awk -v r="${PROMOTE_THRESHOLD}" 'BEGIN { print int(r * 100) }')"
FLAG_AS_INT="$(awk -v r="${FLAG_THRESHOLD}" 'BEGIN { print int(r * 100) }')"

if [[ "${PASS_RATE_AS_INT}" -ge "${PROMOTE_AS_INT}" ]]; then
    echo "  verdict:           PROMOTABLE (>= ${PROMOTE_AS_INT}%)"
    exit 0
elif [[ "${PASS_RATE_AS_INT}" -ge "${FLAG_AS_INT}" ]]; then
    echo "  verdict:           OPERATOR REVIEW (${FLAG_AS_INT}-${PROMOTE_AS_INT}%)"
    [[ "${STRICT}" -eq 1 ]] && exit 5 || exit 0
else
    echo "  verdict:           REJECTED (< ${FLAG_AS_INT}%)"
    [[ "${STRICT}" -eq 1 ]] && exit 5 || exit 0
fi
