# GUIDE: Post-Commit Training Hook — Automatic DPO Tuple Capture

This hook fires after every git commit. When the commit was authored by Claude Code and
routed through Tier A or Tier B, it submits the commit message and diff to the Doorman's
`/v1/shadow` endpoint as a DPO training tuple. Doorman applies MIN_DIFF_CHARS and PII
gate checks before storage.

> **Legal gate.** Do not set `SLM_SHIM_TRAINING_CAPTURE=true` until legal review has
> confirmed that capturing Tier A/B (OLMo) session outputs is clear of Anthropic's
> competing-models clause and any other applicable terms. See
> `topic-tos-training-constraints.md`.

## How Detection Works

The hook identifies Claude Code commits by checking for a `Co-Authored-By: Claude` trailer
in the commit message. This trailer is already required by the workspace commit convention
(`CLAUDE.md §8` / `AGENT.md` commit rules). No additional tagging is needed.

The `SLM_SHIM_TRAINING_CAPTURE=true` environment variable gates submission. The Doorman
shim sets this variable in the shell environment when a request was served by Tier A or
Tier B — it is absent for Tier C sessions. This ensures Claude (Tier C) outputs never
enter the training corpus regardless of the legal review outcome.

## Prerequisites

- [ ] Sprint 0a shim active (`POST /v1/messages` responding)
- [ ] `SLM_APPRENTICESHIP_ENABLED=true` in Doorman env (currently unset — requires Master-tier action)
- [ ] `jq` installed: `which jq`
- [ ] `curl` installed: `which curl`
- [ ] Legal review complete (see above)

## Step 1 — Create the Hooks Directory

```bash
mkdir -p ~/Foundry/.githooks
```

## Step 2 — Write the Hook

Create `~/Foundry/.githooks/post-commit`:

```bash
#!/bin/bash
# post-commit: shadow brief capture — submits commit diff to /v1/shadow for apprenticeship.
# Backgrounded and disowned — never blocks the developer terminal.

COMMIT_MSG=$(git log -1 --pretty=%B)
DIFF=$(git show --no-color --format= HEAD)

# Gate 1: diff must be substantive (skip tiny doc-only commits)
[ ${#DIFF} -lt 80 ] && exit 0

# Build full ApprenticeshipBrief payload using Python (all fields required by Doorman)
PAYLOAD=$(python3 - "$COMMIT_MSG" <<'PYEOF'
import json, sys, uuid, datetime
diff_text = sys.stdin.read()
commit_msg = sys.argv[1] if len(sys.argv) > 1 else "git-commit"
brief_id = uuid.uuid4().hex.upper()
now = datetime.datetime.now(datetime.timezone.utc).isoformat()
data = {
    "brief": {
        "brief_id": brief_id,
        "created": now,
        "senior_role": "master",
        "senior_identity": "pwoodfine",
        "task_type": "git-commit",
        "scope": {"files": []},
        "acceptance_test": "",
        "shadow": True,
        "body": "git-commit diff: " + commit_msg
    },
    "actual_diff": diff_text
}
print(json.dumps(data))
PYEOF
)

# Submit async — fire and forget; Doorman deduplicates by brief_id
echo "$PAYLOAD" | curl -sS --max-time 10 -X POST \
    -H 'content-type: application/json' \
    -H 'X-Foundry-Module-ID: git-hook' \
    --data-binary @- \
    http://127.0.0.1:9080/v1/shadow >/dev/null 2>&1 &
disown
```

## Step 3 — Make Executable

```bash
chmod +x ~/Foundry/.githooks/post-commit
```

## Step 4 — Install VM-Wide

```bash
git config --global core.hooksPath ~/Foundry/.githooks
```

This applies to every git repository on the VM without per-repo setup. The hook runs
after every commit in every clone.

## Step 5 — Enable Apprenticeship in Doorman

Add to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_APPRENTICESHIP_ENABLED=true
SLM_SHIM_TRAINING_CAPTURE=false   # change to true after legal review
```

Restart:

```bash
sudo systemctl restart local-doorman.service
```

## Verification

After any commit (the gate checks run; the hook fires if the diff is large enough):

```bash
# Doorman logs — look for shadow queue entry
journalctl -u local-doorman.service | grep -i shadow | tail -10
```

A successful submission returns a `202 Accepted` response (invisible since the hook
backgrounded the curl call) and logs a line such as:

```
shadow: queued brief_id=A3F2... at queue_position=4
```

Deduplication is by `brief_id` (a UUID generated per commit), so resubmitting the
same commit does not create duplicate entries.

## Training Schedule

Once the corpus is accumulating:

| Cadence | Action |
|---|---|
| Per commit | Hook fires, submits to /v1/shadow (async, non-blocking) |
| Daily | `bin/export-dpo.sh` — walks shadow corpus, emits TRL-conversational JSONL to `~/Foundry/corpus/dpo/<date>.jsonl` |
| Weekly (≥100 new tuples) | `bin/lora-update.sh` — Unsloth + TRL DPO run on OLMo-2-7B; regression check; promote if passing |
| Monthly | Full re-train from cumulative corpus (prevents catastrophic forgetting) |

**Do not trigger the first real LoRA update until ≥1,000 tuples have accumulated.**
Below that threshold, training produces noise not signal (LIMA threshold for narrow
task distribution). At normal development pace this takes approximately six to eight
weeks after Sprint 0b activation.

## DPO Tuple Format (TRL Conversational)

The export script emits one JSONL record per tuple:

```json
{
  "prompt": [{"role": "user", "content": "<commit message / task brief>"}],
  "chosen": [{"role": "assistant", "content": "<git diff — actual implementation>"}],
  "rejected": [{"role": "assistant", "content": "<apprentice attempt, if any>"}]
}
```

Where no apprentice attempt was captured, `rejected` is an empty or placeholder entry.
The corpus is useful for SFT before enough rejected samples accumulate for DPO.

## LoRA Hyperparameters (OLMo-2-7B, single L4 GPU)

```
r=16, lora_alpha=32, lora_dropout=0.05
target_modules: q_proj, k_proj, v_proj, o_proj, gate_proj, up_proj, down_proj
learning_rate: 1e-5 (DPO) / 2e-4 (SFT)
bf16: true, gradient_checkpointing: true
replay_ratio: 0.20–0.30  (prevents catastrophic forgetting)
beta: 0.1  (DPO KL anchor)
eval_set: 100 pairs held out from early corpus; reject adapter on >5% regression
```

## References

- `topic-tos-training-constraints.md` — legal constraints on training data sources
