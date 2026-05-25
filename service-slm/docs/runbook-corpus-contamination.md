# Runbook — Training Corpus Contamination Response

> **Scope:** `service-slm` apprenticeship training corpus at
> `/srv/foundry/data/training-corpus/`.
> **Owner:** workspace operator (Master) + Command Session.
> **Severity classification:** SHIP-STOPPER if a Tier-C (Anthropic Claude)
> output is found in the corpus. Anthropic Terms of Service prohibit
> using Claude outputs to train a competing model. Discovery requires
> immediate containment and legal notification.

This runbook covers two scenarios:

1. **Contamination detected** — a corpus tuple is found whose
   `tier_used == "external"` OR whose `attempt.tier == "external"` field
   indicates Tier-C provenance.
2. **Suspected contamination** — code path audit or post-incident review
   reveals a window during which the contamination guard was bypassed,
   even if no specific tuple is yet identified.

In both cases the response is the same: **burn the corpus and restart
from clean seeds.** Adapter weights trained on a contaminated corpus
are themselves contaminated and MUST be reverted to the base model.

---

## Phase 1 — Stop the bleeding (within 5 minutes of detection)

### 1.1 Stop the drain worker

```bash
sudo systemctl stop local-doorman
```

This halts:
- All apprenticeship dispatches (`/v1/brief`, `/v1/shadow`)
- The background drain worker that consumes
  `/srv/foundry/data/apprenticeship/queue/*.brief.jsonl`
- Verdict signing path (`/v1/verdict`)

Verify:
```bash
systemctl status local-doorman | grep "Active:"
# Expect: Active: inactive (dead)
```

### 1.2 Freeze the apprenticeship queue

```bash
sudo mv /srv/foundry/data/apprenticeship/queue \
        /srv/foundry/data/apprenticeship/queue.frozen-$(date -u +%Y%m%dT%H%M%SZ)
sudo mkdir -p /srv/foundry/data/apprenticeship/queue
sudo chown root:foundry /srv/foundry/data/apprenticeship/queue
sudo chmod 2775 /srv/foundry/data/apprenticeship/queue
```

The frozen directory preserves the queue state for forensic analysis;
the new empty directory replaces it so the queue substrate is clean
when the doorman restarts.

### 1.3 Quarantine the training corpus

```bash
sudo mv /srv/foundry/data/training-corpus \
        /srv/foundry/data/training-corpus.quarantine-$(date -u +%Y%m%dT%H%M%SZ)
sudo mkdir -p /srv/foundry/data/training-corpus/apprenticeship
sudo mkdir -p /srv/foundry/data/training-corpus/engineering
sudo mkdir -p /srv/foundry/data/training-corpus/feedback
sudo mkdir -p /srv/foundry/data/training-corpus/eval
sudo chown -R root:foundry /srv/foundry/data/training-corpus
sudo chmod -R 2775 /srv/foundry/data/training-corpus
```

The quarantine directory is preserved indefinitely for legal review.
**Do NOT delete it.** Encrypt it for off-site backup before any decision
to remove.

---

## Phase 2 — Revert adapters (within 30 minutes)

Any LoRA adapter trained on the contaminated corpus is itself
contaminated. Adapters MUST be removed from production inference.

### 2.1 Remove all loaded adapters

```bash
# Stop the Tier A inference server
sudo systemctl stop local-slm

# Confirm no --lora flag in the service unit's effective command
systemctl cat local-slm | grep -- '--lora'
# Expect: no output (no LoRA loaded)

# If a --lora flag is present, remove the drop-in that adds it:
sudo rm /etc/systemd/system/local-slm.service.d/lora.conf
sudo systemctl daemon-reload
```

### 2.2 Move all adapter artifacts to quarantine

```bash
if [ -d /srv/foundry/data/adapters ]; then
    sudo mv /srv/foundry/data/adapters \
            /srv/foundry/data/adapters.quarantine-$(date -u +%Y%m%dT%H%M%SZ)
fi
```

### 2.3 Restart Tier A on base model only

```bash
sudo systemctl start local-slm
sleep 30
curl -sS http://127.0.0.1:8080/health | jq
# Expect: {"status":"ok", "slots_idle":1, ...}
```

The Doorman remains down until Phase 3 completes.

---

## Phase 3 — Audit + NOTAM + legal notification (within 2 hours)

### 3.1 Forensic scan of the quarantined corpus

Find every tuple with Tier-C provenance:

```bash
QUARANTINE=$(ls -1dt /srv/foundry/data/training-corpus.quarantine-* | head -1)
find "$QUARANTINE" -name '*.jsonl' -print0 | \
    xargs -0 grep -l '"tier_used":"external"' || true
find "$QUARANTINE" -name '*.jsonl' -print0 | \
    xargs -0 grep -l '"tier":"external"' || true
```

Save the full file list to `/srv/foundry/incident/contamination-<date>/`.

For each contaminated file, capture:
- File path
- `brief_id`
- `created` timestamp
- `module_id` and `tenant`
- The full `attempt` subobject (which Claude model, latency, token
  count, cost)

### 3.2 Audit-log cross-reference

```bash
grep -h '"tier":"external"' \
    /srv/foundry/.service-slm/audit/*.jsonl 2>/dev/null | \
    jq -r '[.timestamp_utc, .request_id, .module_id, .model] | @tsv'
```

This catches Tier-C requests that DID transit the gateway but whose
provenance may not have been captured in a corpus tuple (the
contamination guard correctly suppressed the write). These rows are
NOT contamination — they are the guard working as designed. Document
them separately as "guard activations" not "contamination events".

### 3.3 Post a NOTAM

Append to `/srv/foundry/NOTAM.md`:

```markdown
## NOTAM <DATE>-<NN> — HAZARD — Training corpus contamination response active

**Affects:** All sessions referencing `/srv/foundry/data/training-corpus/`,
all `service-slm` adapter training, all `app-console-slm` operations
relying on locally-trained models.

**Description:** Training corpus contamination detected <DATE>. Corpus
quarantined; adapters reverted to base; legal review in progress.
Workspace sessions MUST NOT consume tuples from
`/srv/foundry/data/training-corpus.quarantine-<TS>/`.

**Action required:** Sessions touching service-slm: STOP. Do not start
adapter training. Do not query `/v1/brief` or `/v1/shadow` (Doorman is
down). Awaiting operator green-light.

**Issued:** <DATE>T<TIME>Z by <session-id>
**Expires:** manual (remove after operator confirms re-seed + legal
sign-off)
```

### 3.4 Legal notification

If contamination is confirmed (not just suspected):

- Email `legal@<corp-domain>` with: incident timestamp, contaminated
  tuple count, Anthropic API account ID (if Tier-C was wired), proposed
  remediation plan (this runbook).
- Do NOT delete the quarantine directory until legal authorizes.
- If Anthropic API key was used (Commercial), notify Anthropic of the
  incident per their terms.

---

## Phase 4 — Re-seed from clean inputs (within 1 week)

### 4.1 Verify defensive layers are in place

Before re-enabling capture, confirm all three defense layers are coded
and tested:

1. **`pick_tier_for_brief`** — structural invariant returns only
   `Tier::Local` or `Tier::Yoyo`. Cannot return `Tier::External`.
   ```bash
   cd /srv/foundry/clones/project-intelligence/service-slm
   grep -A 10 'fn pick_tier_for_brief' \
       crates/slm-doorman/src/apprenticeship.rs
   ```

2. **`/v1/shadow` 403 on `source_tier=="external"`** — code-level reject
   in `shadow()` handler in `crates/slm-doorman-server/src/http.rs`.

3. **`write_shadow_tuple` Tier::External early-return** —
   `crates/slm-doorman/src/apprenticeship.rs` near the start of
   `write_shadow_tuple()`.

4. **`corpus_gate::check`** — second-layer write-time gate with
   Do-Not-Use scan and BCSC flag in
   `crates/slm-doorman/src/corpus_gate.rs`.

Run the integration test:
```bash
cargo test --workspace shadow_with_external_source_tier
# Expect: 1 passed
cargo test --workspace write_shadow_tuple_refuses_tier_external
# Expect: 1 passed
```

### 4.2 Re-seed graph from Tier A / Tier B native extractions

The `service-content` LadybugDB graph (10K+ entities) may also need
re-seeding if any graph entries were extracted via a path that included
Tier C:

```bash
# Inspect extraction audit log
grep -h '"tier_used":"external"' \
    /srv/foundry/.service-slm/audit/*.jsonl | wc -l
# Expect: 0 (extraction MUST NOT use Tier C per SYS-ADR-07 path policy)
```

If non-zero, run a re-extraction pass on the affected CORPUS ledger
files via the Tier A / Tier B native extractor.

### 4.3 Re-enable capture in dry-run mode

```bash
# Start Doorman with SLM_APPRENTICESHIP_ENABLED=false to verify other
# endpoints work without re-enabling capture yet.
sudo systemctl start local-doorman
curl -sS http://127.0.0.1:9090/healthz | jq
# Expect: {"status":"ok", "apprenticeship_enabled":false, ...}

# Verify no Tier C key is configured (operator directive 2026-05-18):
sudo systemctl show local-doorman -p Environment | \
    grep -i 'SLM_TIER_C_ANTHROPIC_API_KEY' || echo "OK — no Tier C key"
```

### 4.4 Re-enable apprenticeship

Only after legal sign-off:

```bash
sudo sed -i 's/SLM_APPRENTICESHIP_ENABLED=false/SLM_APPRENTICESHIP_ENABLED=true/' \
    /etc/systemd/system/local-doorman.service
sudo systemctl daemon-reload
sudo systemctl restart local-doorman
```

Verify the first new captures land in the (now-empty) corpus:
```bash
sleep 60
ls /srv/foundry/data/training-corpus/apprenticeship/*/ 2>/dev/null
# Expect: new captures appearing in task-type subdirs
```

### 4.5 Clear the NOTAM

Edit `/srv/foundry/NOTAM.md` to move the contamination NOTAM from
"Active notices" to "Recently expired" with a `*(cleared <date>:
<details>)*` line.

---

## Reference — defense layers

The contamination guard is defense-in-depth across four layers:

| Layer | Where | Mechanism |
|---|---|---|
| 1. Routing | `apprenticeship::pick_tier_for_brief` | Returns only `Tier::Local` or `Tier::Yoyo`; cannot select Tier C |
| 2. Enqueue | `http.rs::shadow` handler | 403 FORBIDDEN if `source_tier == "external"` on the wire |
| 3. Dispatch | `apprenticeship::write_shadow_tuple` | Early-return Ok(()) with contamination_guard warn log if `attempt.tier == Tier::External` |
| 4. Audit | JSONL `tier_used` top-level field | O(n) audit-replay via `jq 'select(.tier_used=="external")'` |

The corpus quality gate (`corpus_gate.rs`) adds further checks (BCSC
posture, Do-Not-Use vocabulary, max-diff cap, dedup) on top of these
four layers.

---

## Reference — Anthropic ToS constraint

Per the Anthropic Commercial API Terms of Service (and reinforced by
operator directive 2026-05-18: stay with Claude Pro Max 20x for now,
no Commercial API key in production):

> "You may not use Claude to train, fine-tune, or improve any model
> that competes with Anthropic's models."

Operational interpretation:
- Tier A (OLMo, locally hosted): outputs MAY enter the training corpus.
- Tier B (OLMo on Yo-Yo, also locally controlled): outputs MAY enter
  the training corpus.
- Tier C (Anthropic Claude via API): outputs **MUST NOT** enter the
  training corpus. The contamination guards above enforce this.
- Claude Pro Max subscription: outputs do NOT transit the gateway and
  are not captured by `/v1/messages` shim. The git post-commit hook
  (`bin/capture-edit.py`) captures committed diffs regardless of which
  AI authored them — by Anthropic ToS the developer is the author of
  their commits, so committed code is permitted feedstock. Suspected
  Claude-authored contamination via this path is best mitigated by
  ensuring developer commits represent meaningful human review.

---

## Reference — when this runbook does NOT apply

- **First-layer queue gate rejection** — `quality_gate_shadow` rejecting
  a brief at enqueue time is normal operation, not contamination.
- **`corpus_gate` Do-Not-Use term rejection** — also normal operation.
- **`contamination_guard` tracing log on Tier C audit** — also normal
  (defense layer activating correctly). Document as a guard activation
  in the audit ledger; do NOT trigger this runbook unless a tuple is
  found on disk with Tier-C provenance.

When in doubt, escalate to Master before quarantining. The cost of
unnecessary quarantine is the cost of one workday of re-seeding; the
cost of missed contamination is legal exposure.

---

*Phase 3 (P3-3.6) of `learning-loop-master-plan-2026-05-18.md`.
Authored 2026-05-18 by task@claude-code (project-intelligence).
Ratification: outbox to project-editorial for GUIDE-emergency-corpus-quarantine
authoring, and to Command for any workspace-level conventions/ link.*
