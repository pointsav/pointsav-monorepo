#!/usr/bin/env bash
# Weekly DataGraph cleanup — re-seeds taxonomy entities from current CSVs.
#
# Calls service-content HTTP API to delete and re-seed topics and guides,
# removing stale entities and replacing them with the current CSV state.
# Run weekly on Sunday (after corpus-threshold.py training check).
#
# Usage:
#   ./scripts/graph-cleanup.sh
#   SERVICE_CONTENT_URL=http://127.0.0.1:9081 ./scripts/graph-cleanup.sh
set -uo pipefail

SERVICE_CONTENT_URL="${SERVICE_CONTENT_URL:-http://127.0.0.1:9081}"

echo "[graph-cleanup] Targeting service-content at: ${SERVICE_CONTENT_URL}"

# Check service-content is up
if ! curl -sf "${SERVICE_CONTENT_URL}/v1/health" > /dev/null 2>&1; then
    echo "[graph-cleanup] ERROR: service-content not reachable at ${SERVICE_CONTENT_URL}" >&2
    exit 1
fi

echo "[graph-cleanup] Reloading taxonomy from CSVs..."

# Re-trigger taxonomy load by calling the taxonomy reload endpoint.
# service-content loads taxonomy at startup; this endpoint forces a reload
# without a restart.
reload_response=$(curl -sf -X POST "${SERVICE_CONTENT_URL}/v1/config/taxonomy/reload" \
    -H "Content-Type: application/json" 2>&1)

if [[ $? -eq 0 ]]; then
    echo "[graph-cleanup] Taxonomy reload: OK"
    echo "${reload_response}" | python3 -m json.tool 2>/dev/null || echo "${reload_response}"
else
    echo "[graph-cleanup] Taxonomy reload endpoint not available (service may need restart for CSV reload)."
    echo "[graph-cleanup] Manual path: restart service-content to re-seed taxonomy from CSVs."
fi

echo "[graph-cleanup] Done."
