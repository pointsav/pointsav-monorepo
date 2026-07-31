#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# smoke-test-service-content.sh — advisory smoke-test for service-content's
# HTTP server, called directly on its own port (9081) rather than through the
# Doorman proxy. No prior smoke-test script targeted service-content directly
# before this one — see BRIEF-datagraph-tenant-isolation.md's Session 3 test
# pass for context.
#
# Hits every endpoint and reports PASS/FAIL without blocking the deploy.
# Exit 0 if all tests ran (advisory mode); exit 1 only if the script itself crashed.
#
# Usage:
#   SERVICE_CONTENT_URL=http://127.0.0.1:9081 ./scripts/smoke-test-service-content.sh
#
# Default SERVICE_CONTENT_URL: http://127.0.0.1:9081

set -euo pipefail

SERVICE_CONTENT_URL="${SERVICE_CONTENT_URL:-http://127.0.0.1:9081}"

PASS=0
FAIL=0
TOTAL=0

# ─── helpers (mirrors service-slm/scripts/smoke-test-doorman.sh) ─────────────

_pass() {
    PASS=$(( PASS + 1 ))
    TOTAL=$(( TOTAL + 1 ))
    echo "  [PASS] $1"
}

_fail() {
    FAIL=$(( FAIL + 1 ))
    TOTAL=$(( TOTAL + 1 ))
    echo "  [FAIL] $1"
}

_check() {
    local name="$1"
    local expected="$2"
    local actual="$3"
    local preview="$4"
    echo ""
    echo "TEST: $name"
    echo "  expected: HTTP $expected  got: HTTP $actual"
    echo "  body: ${preview:0:120}"
    if [[ "$actual" == "$expected" ]]; then
        _pass "$name"
    else
        _fail "$name"
    fi
}

_curl() {
    local method="$1"
    local url="$2"
    shift 2
    local extra_args=("$@")

    local tmpfile
    tmpfile="$(mktemp)"
    local http_code
    if http_code="$(curl -s -o "$tmpfile" -w "%{http_code}" \
            --connect-timeout 5 --max-time 15 \
            -X "$method" "$url" "${extra_args[@]}" 2>/dev/null)"; then
        local body
        body="$(cat "$tmpfile")"
        rm -f "$tmpfile"
        echo "${http_code}|${body}"
    else
        local curl_exit="$?"
        rm -f "$tmpfile"
        echo "000|curl error (exit ${curl_exit}) — server may be unreachable"
    fi
}

# ─── tests ───────────────────────────────────────────────────────────────────

echo "=== service-content smoke-test ==="
echo "URL: $SERVICE_CONTENT_URL"
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

SMOKE_TEST_MODULE_ID="smoke-test-service-content"
SMOKE_TEST_ENTITY="SmokeTestEntity-$(date +%s)"

# 1. GET /healthz — expect 200 with status + entity_count fields
{
    IFS='|' read -r status body <<< "$(_curl GET "$SERVICE_CONTENT_URL/healthz")"
    _check "GET /healthz → 200" "200" "$status" "$body"
    if [[ "$status" == "200" ]]; then
        if ! echo "$body" | grep -q "entity_count"; then
            echo "    WARNING: healthz body missing entity_count field"
        fi
    fi
}

# 2. POST /v1/graph/mutate (direct, no capability header — the local-Doorman
# trusted path) → expect 200, writes a clearly-tagged synthetic entity.
{
    mutate_payload="{\"module_id\":\"${SMOKE_TEST_MODULE_ID}\",\"entities\":[{\"entity_name\":\"${SMOKE_TEST_ENTITY}\",\"classification\":\"architecture-reference\",\"role_vector\":null,\"location_vector\":null,\"contact_vector\":null,\"module_id\":\"${SMOKE_TEST_MODULE_ID}\",\"confidence\":1.0,\"source_doc\":null}]}"
    IFS='|' read -r status body <<< "$(
        _curl POST "$SERVICE_CONTENT_URL/v1/graph/mutate" \
            -H 'Content-Type: application/json' \
            -d "$mutate_payload"
    )"
    _check "POST /v1/graph/mutate (direct, tagged test entity) → 200" "200" "$status" "$body"
}

# 3. GET /v1/graph/context — expect 200, must return the entity just written,
# scoped strictly to its own module_id (no cross-tenant merge).
{
    IFS='|' read -r status body <<< "$(
        _curl GET "${SERVICE_CONTENT_URL}/v1/graph/context?q=${SMOKE_TEST_ENTITY}&module_id=${SMOKE_TEST_MODULE_ID}&limit=5"
    )"
    _check "GET /v1/graph/context (read back tagged entity) → 200" "200" "$status" "$body"
    if [[ "$status" == "200" ]]; then
        if ! echo "$body" | grep -q "$SMOKE_TEST_ENTITY"; then
            echo "    WARNING: context query did not return the entity just written — write/read round-trip may be broken"
        fi
    fi
}

# 4. GET /v1/graph/context with an unrelated module_id — expect 200 with an
# empty/non-matching result, proving no cross-tenant bleed on this direct path.
{
    IFS='|' read -r status body <<< "$(
        _curl GET "${SERVICE_CONTENT_URL}/v1/graph/context?q=${SMOKE_TEST_ENTITY}&module_id=some-other-tenant&limit=5"
    )"
    _check "GET /v1/graph/context (different tenant, must not see it) → 200" "200" "$status" "$body"
    if [[ "$status" == "200" ]]; then
        if echo "$body" | grep -q "$SMOKE_TEST_ENTITY"; then
            echo "    FAIL: cross-tenant leak — a different module_id saw the smoke-test entity"
            FAIL=$(( FAIL + 1 ))
            TOTAL=$(( TOTAL + 1 ))
        fi
    fi
}

# 5. capability_gate — a request WITHOUT X-Foundry-Capability must still pass
# through unchanged (the local-Doorman trusted path, tests 2-4 above already
# implicitly prove this, but check explicitly against /v1/graph/context too).
{
    IFS='|' read -r status body <<< "$(
        _curl GET "${SERVICE_CONTENT_URL}/v1/graph/context?q=x&module_id=${SMOKE_TEST_MODULE_ID}&limit=1"
    )"
    _check "GET /v1/graph/context (no capability header) → 200" "200" "$status" "$body"
}

# 6. capability_gate — a request WITH a deliberately-malformed X-Foundry-
# Capability header must be rejected, not silently ignored.
{
    IFS='|' read -r status body <<< "$(
        _curl GET "${SERVICE_CONTENT_URL}/v1/graph/context?q=x&module_id=${SMOKE_TEST_MODULE_ID}&limit=1" \
            -H 'X-Foundry-Capability: not-a-valid-token'
    )"
    echo ""
    echo "TEST: GET /v1/graph/context (malformed capability header) → 4xx"
    echo "  expected: HTTP 4xx  got: HTTP $status"
    echo "  body: ${body:0:120}"
    if [[ "$status" == 4* ]]; then
        _pass "GET /v1/graph/context (malformed capability header) → 4xx"
    else
        _fail "GET /v1/graph/context (malformed capability header) → 4xx"
    fi
}

# 7. GET /v1/pair/token — expect 200 with a signed token + public_key
{
    IFS='|' read -r status body <<< "$(_curl GET "${SERVICE_CONTENT_URL}/v1/pair/token?role=USER&node_label=smoke-test")"
    _check "GET /v1/pair/token → 200" "200" "$status" "$body"
    if [[ "$status" == "200" ]]; then
        if ! echo "$body" | grep -q "public_key"; then
            echo "    WARNING: pair/token response missing public_key field"
        fi
    fi
}

# ─── summary ─────────────────────────────────────────────────────────────────

echo ""
echo "========================================"
echo "  $PASS tests passed / $TOTAL total"
if [[ $FAIL -gt 0 ]]; then
    echo "  $FAIL tests FAILED — review output above"
fi
echo "========================================"

# Advisory mode: always exit 0 so the script does not block a deploy.
exit 0
