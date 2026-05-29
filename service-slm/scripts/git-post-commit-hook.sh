#!/bin/bash
# Git post-commit hook — send diff to Doorman /v1/shadow for apprenticeship capture.
# Install: cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x .git/hooks/post-commit
# Runs asynchronously (&) so it never blocks the commit.
set -euo pipefail

DOORMAN_ENDPOINT="${SLM_DOORMAN_ENDPOINT:-http://127.0.0.1:9080}"

DIFF=$(git diff HEAD~1 HEAD --unified=3 2>/dev/null || git show HEAD --unified=3)

if [ -z "$DIFF" ]; then
    exit 0
fi

PAYLOAD=$(python3 -c 'import json,sys; print(json.dumps({"actual_diff": sys.stdin.read()}))' <<< "$DIFF")

curl -s -X POST "${DOORMAN_ENDPOINT}/v1/shadow" \
    -H "Content-Type: application/json" \
    -H "X-Foundry-Module-ID: git-hook" \
    -d "$PAYLOAD" \
    > /dev/null 2>&1 &
