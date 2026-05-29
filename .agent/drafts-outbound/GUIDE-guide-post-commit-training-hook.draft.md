---
schema: foundry-draft-v1
state: draft
language_protocol: PROSE-GUIDE
originating_cluster: project-intelligence
target_repo: vendor/woodfine-fleet-deployment/cluster-totebox-intelligence
target_path: guide-post-commit-training-hook.md
audience: internal
bcsc_class: no-disclosure-implication
authored: 2026-05-29
authored_by: project-intelligence Totebox (claude-sonnet-4-6, Sprint 1 closeout)
authored_with: claude-sonnet-4-6
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Source: service-slm/docs/guide-post-commit-training-hook.md (original research session 2026-05-16)
research_inline: true
notes_for_editor: |
  Operations guide — installing the git post-commit hook for DPO tuple capture.
  Bloomberg-register pass required. Bilingual ES sibling required for TOPIC artifacts.
  Remove any internal crate paths or infrastructure-specific detail before publication.
  Verify all code references match HEAD (Sprint 1 updated several line numbers).
---

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
- [ ] `SLM_APPRENTICESHIP_ENABLED=true` in Doorman env (currently unset — Master-tier
      action; see `service-slm/ARCHITECTURE.md §11`)
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
# post-commit: DPO training tuple capture for Tier A/B Claude Code sessions.
# Tier C (Anthropic API) outputs are excluded — SLM_SHIM_TRAINING_CAPTURE gate enforces this.
# Backgrounded and disowned — never blocks the developer terminal.

MSG=$(git log -1 --pretty=%B)
DIFF=$(git show --no-color --format= HEAD)

# Gate 1: must be a Claude Code commit (Co-Authored-By: Claude trailer)
AUTHOR_TRAILER=$(echo "$MSG" | grep -Ei '^Co-Authored-By: Claude')
[ -z "$AUTHOR_TRAILER" ] && exit 0

# Gate 2: diff must be substantive
[ ${#DIFF} -lt 80 ] && exit 0

# Gate 3: must be a Tier A/B session (Doorman sets this; absent for Tier C)
[ "$SLM_SHIM_TRAINING_CAPTURE" != "true" ] && exit 0

# Submit async — fire and forget
jq -nc --arg b "$MSG" --arg d "$DIFF" \
  '{"brief":{"id":env.GIT_COMMIT,"body":$b},"actual_diff":$d}' \
  | curl -sS --max-time 5 -X POST \
      -H 'content-type: application/json' \
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

After a Claude Code commit that was routed through Tier A or Tier B:

```bash
# Check shadow queue files
ls -la /srv/foundry/clones/project-intelligence/service-slm/data/apprenticeship/queue/

# Check Doorman logs for shadow activity
journalctl -u local-doorman.service | grep -i shadow | tail -10
```

A successful submission logs a shadow tuple ID. Doorman's first-write-wins deduplication
uses the commit SHA as the tuple ID, so resubmitting the same commit is idempotent.

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
- `moonshot-slm/README.md` — long-term training pipeline architecture
- `service-slm/ARCHITECTURE.md §11` — apprenticeship substrate design
- `.agent/plans/sovereign-routing-comprehensive.md §4` — full flywheel design
