#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# Git post-commit hook — send diff to Doorman /v1/shadow for apprenticeship capture.
# Install: cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x .git/hooks/post-commit
# Runs asynchronously (&) so it never blocks the commit.
#
# 2026-07-03: restored secret-redaction + diff truncation that the 2026-05-29 swap
# (dacffb1) silently dropped, and restored task_type "shadow-capture" (confirmed via
# git archaeology to be an accidental collapse, not an intentional rename — no prior
# history for "git-commit" anywhere in bin/capture-edit.py's lineage, the sibling copy
# fixed 2026-07-02 in commit 45cfb78). Redaction patterns ported verbatim from that fix.
# This is the canonical template every archive's .git/hooks/post-commit is copied from —
# fixing bin/capture-edit.py alone left this file (and any newly-provisioned archive)
# still vulnerable.
#
# 2026-07-30 (Command broadcast, originally fixed 2026-07-15 in the global hook):
# diff and payload are now passed via temp file + stdin instead of an environment
# variable / argv element — a large diff (e.g. a 16k-line commit) could exceed the
# OS's single-string env/argv length limit, failing with "Argument list too long"
# (non-fatal — telemetry only — but the capture was silently lost). This template
# had not picked up that fix until now.

DOORMAN_ENDPOINT="${SLM_DOORMAN_ENDPOINT:-http://127.0.0.1:9080}"
TOGGLE_FILE="${FOUNDRY_ROOT:-/srv/foundry}/identity/.toggle"

DIFF_FILE=$(mktemp)
git diff HEAD~1 HEAD --unified=3 > "$DIFF_FILE" 2>/dev/null || git show HEAD --unified=3 > "$DIFF_FILE" 2>/dev/null

if [ ! -s "$DIFF_FILE" ]; then
    rm -f "$DIFF_FILE"
    exit 0
fi

COMMIT_MSG=$(git log -1 --pretty=%s 2>/dev/null || echo "git-commit")

# Read identity from toggle file (0=jwoodfine, 1=pwoodfine) — restores the
# same identity-detection this template already had before this fix, since
# senior_identity was previously hardcoded to "pwoodfine" below.
if [ -f "$TOGGLE_FILE" ]; then
    TOGGLE=$(cat "$TOGGLE_FILE" 2>/dev/null || echo "0")
    [ "$TOGGLE" = "1" ] && IDENTITY="pwoodfine" || IDENTITY="jwoodfine"
else
    IDENTITY="jwoodfine"
fi

PY_SCRIPT=$(mktemp)
cat > "$PY_SCRIPT" <<'PYEOF'
import json, sys, uuid, datetime, re

identity = sys.argv[1] if len(sys.argv) > 1 else "jwoodfine"
commit_msg = sys.argv[2] if len(sys.argv) > 2 else "git-commit"
diff_text = sys.stdin.read()
brief_id = uuid.uuid4().hex.upper()
now = datetime.datetime.now(datetime.timezone.utc).isoformat()

DIFF_LINE_LIMIT = 1000

REDACTIONS = [
    (
        re.compile(
            r"-----BEGIN (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----"
            r".*?"
            r"-----END (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----",
            re.DOTALL,
        ),
        "[REDACTED PRIVATE KEY]",
    ),
    (re.compile(r"\bAKIA[0-9A-Z]{16}\b"), "[REDACTED AWS KEY]"),
    (re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_\-]{32,}\b"), "[REDACTED API KEY]"),
    (re.compile(r"\bghp_[A-Za-z0-9]{36,}\b"), "[REDACTED GITHUB TOKEN]"),
    (re.compile(r"\bgho_[A-Za-z0-9]{36,}\b"), "[REDACTED GITHUB OAUTH]"),
    (re.compile(r"\bxox[abprs]-[A-Za-z0-9-]{10,}\b"), "[REDACTED SLACK TOKEN]"),
    (
        re.compile(
            r'(?i)\b(?:bearer|api[_-]?key|secret|token|password)\s*[:=]\s*'
            r'["\']?([A-Za-z0-9/+_\-]{32,})["\']?'
        ),
        lambda m: m.group(0).replace(m.group(1), "[REDACTED]"),
    ),
]


def sanitize(text):
    for pattern, replacement in REDACTIONS:
        text = pattern.sub(replacement, text)
    return text


def truncate_diff(diff):
    lines = diff.split("\n")
    if len(lines) > DIFF_LINE_LIMIT:
        return "\n".join(lines[:DIFF_LINE_LIMIT]) + "\n... [TRUNCATED at {} lines]".format(DIFF_LINE_LIMIT), True
    return diff, False


diff_text, truncated = truncate_diff(diff_text)
diff_text = sanitize(diff_text)
commit_msg = sanitize(commit_msg)

data = {
    "brief": {
        "brief_id": brief_id,
        "created": now,
        "senior_role": "master",
        "senior_identity": identity,
        "task_type": "shadow-capture",
        "scope": {"files": []},
        "acceptance_test": "",
        "shadow": True,
        "body": "shadow-capture diff: " + commit_msg
    },
    "actual_diff": diff_text,
    "truncated": truncated
}
json.dump(data, sys.stdout)
PYEOF

(
    PAYLOAD_FILE=$(mktemp)
    python3 "$PY_SCRIPT" "$IDENTITY" "$COMMIT_MSG" < "$DIFF_FILE" > "$PAYLOAD_FILE" 2>/dev/null
    curl -s --max-time 5 -X POST "${DOORMAN_ENDPOINT}/v1/shadow" \
        -H "Content-Type: application/json" \
        -H "X-Foundry-Module-ID: git-hook" \
        --data-binary "@${PAYLOAD_FILE}" \
        > /dev/null 2>&1
    rm -f "$PAYLOAD_FILE" "$DIFF_FILE" "$PY_SCRIPT"
) &
disown
