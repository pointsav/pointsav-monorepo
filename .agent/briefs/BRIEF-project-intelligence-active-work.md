---
artifact: brief
status: active
created: 2026-06-01
author: totebox@project-intelligence (claude-sonnet-4-6)
replaces: BRIEF-active-work.md (missing — never existed on disk)
companion:
  - BRIEF-slm-substrate-master.md  (SLM operations reference)
  - BRIEF-slm-learning-loop.md     (training pipeline spec)
---

# BRIEF — project-intelligence Active Work

> **Session-start reading.** Read this before asking what to work on.
> Companions: substrate master (Yo-Yo ops, tier routing), learning loop (training spec, corpus).

---

## Current service state (2026-05-31)

| Service | State | Note |
|---|---|---|
| `local-doorman.service` | **active** | Tier A ready; /healthz 404 (known — route missing); /readyz OK |
| `local-slm.service` | **active** | OLMo 2 7B Q4_K_M; Tier A primary |
| `service-content` | **active** | 7,201 entities; Tier B extraction deferred (Yo-Yo down) |
| `local-orchestration-slm.service` | **inactive** | Unit file installed; operator deploy pending (§3) |
| `yoyo-tier-b-1` | TERMINATED | europe-west4-a L4 stockout; restart when capacity returns |
| Apprenticeship queue | **5 pending / 550 done / 78 poison** | See §1 |

---

## §1 — Poison queue investigation item

78 entries in `queue-poison/` as of session 13, up from 0 at session 11 close.
Newest entries (May 31 04:47–04:58 UTC, post-Fix-B deploy at 00:41 UTC):
- `actual_diff: ""` at top level — Fix A was not applied to these entries
- No `response_raw`, no `escalate` field — never dispatched to OLMo

**Two hypotheses:**
- **H1 (likely)**: Pre-Fix-A briefs carried forward from session 11's recovery batch.
  Sprint 2C `write_shadow_tuple` guard rejects `actual_diff: ""` entries and moves them
  to poison/ without dispatching. These are already known-bad; quarantine resolves it.
- **H2**: New commits post-Fix-A are producing briefs with empty `actual_diff` — meaning
  the hook fix in `/srv/foundry/bin/capture-edit.py` is not working for some commits.

**Verify H1 vs H2:**
```bash
python3 -c "
import json, os
d = '/srv/foundry/data/apprenticeship/queue-poison'
for f in sorted(os.listdir(d))[-10:]:
    data = json.load(open(f'{d}/{f}'))
    b = data.get('brief', data)
    print(f, b.get('created','?'), 'diff_len:', len(data.get('actual_diff','')))
"
```
If `created` timestamps predate 2026-05-31T00:41Z → H1 (carry-forward; quarantine).
If `created` is after 00:41Z → H2 (hook broken; investigate capture-edit.py).

**Fix C (GBNF grammar) does NOT help with H2.** Fix C addresses OLMo preamble before `---`.
Empty `actual_diff` means the brief was never dispatched; grammar constraints are irrelevant.

---

## §2 — Next 3 sessions: short-term queue

### Session 13 (this session)

- [ ] **Poison queue investigation** (§1 above) — determine H1 vs H2; quarantine or investigate
- [ ] **Brief consolidation** — this session's deliverable (underway)

### Session 14

- [ ] **P1: /readyz structured circuit_breaker_state** (~30 LOC)
  File: `service-slm/crates/slm-doorman-server/src/http.rs`
  Add `reason: Option<String>` and `zone: Option<String>` to the Tier B circuit state JSON.
  Populate from `circuit_breaker.rs`. Benefit: app-console-slm status command gets human-readable reason.

- [ ] **app-console-slm Sprint 4a** — `app-console-slm status` command
  Outbox `project-intelligence-20260530-console-wiring` is addressed to project-console.
  If that session is inactive, implement here — the crate lives in this archive.
  Full spec (endpoints, output format) in that outbox message.

- [ ] **P2: service-content path decoupling** (~20 LOC)
  File: `service-content/src/main.rs`
  Replace hardcoded `/srv/foundry/...` paths with env vars:
  `INFRASTRUCTURE_ROOT` (default `/srv/foundry`) and `CORPUS_ROOT` (default `/srv/foundry/data/apprenticeship`).

### Session 15

- [ ] **Fix C: GBNF grammar** — only if H1 confirmed AND post-Fix-B drain cycles still show
  OLMo preamble escalations (check via `grep escalate:true queue-done/*.jsonl | wc -l`)
  File: `service-slm/crates/slm-doorman-server/src/apprenticeship.rs` lines 181, 279
  Add `grammar: Some(GrammarConstraint::Gbnf(APPRENTICE_GBNF_GRAMMAR))` to both
  `dispatch_shadow()` calls. Wiring exists in `LocalTierClient::complete()`.

- [ ] **P0: Doorman audit ledger sha256**
  File: `service-slm/crates/slm-doorman-server/src/ledger.rs`
  Add `sha256: String` field to `LedgerEntry`; compute `blake3::hash(serialized_entry)` on write.
  Not the same as system-ledger — this is Doorman's own per-inference audit log.

- [ ] **Yo-Yo 1h test** — when europe-west4-a L4 capacity returns:
  ```bash
  service-slm/scripts/start-yoyo.sh --wait-ready=120 --runtime=1h
  ```

---

## §3 — Operator-gated actions (require sudo or GCP console)

Cannot be done by Totebox session alone. All from outbox `project-intelligence-20260530-stage6-orchestration-deploy`.

1. **Deploy orchestration-slm binary** — `cargo build --release -p orchestration-slm-server` + `sudo cp` + update binary ledger
2. **Install chassis env** — `sudo cp infrastructure/env/local-orchestration-slm.env.template /etc/foundry/local-orchestration-slm.env` + set `ORCHESTRATION_YOYO_BEARER`
3. **Enable chassis service** — `sudo systemctl enable --now local-orchestration-slm.service`
4. **Wire Doorman to chassis** — append `SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180` to local-doorman.env + restart
5. **Stage 6** — 2 shutdown-ops commits ahead of origin/main; Command Session `bin/promote.sh`

---

## §4 — Medium-term (sessions 16–20)

- [ ] **P3: orchestration-slm persistence** — replace ephemeral HashMap metering with Redb or SQLite; required for production audit trail (1 session)
- [ ] **Claude Code CORPUS bridge deployment** — install `local-claude-bridge.service` systemd unit
- [ ] **drain-apprenticeship.service/timer cleanup** — these units are disabled but still in `/etc/systemd/system/`; remove or archive
- [ ] **Stale shim test fields** — `anthropic_shim_test.rs` `tier_a_reason` / `idle_monitor` fields are stale
- [ ] **max_tokens 2048 → 512–768 for CPU shadow briefs** — reduces per-brief latency from 17–60 min to under 10 min (separate config decision; lower quality tradeoff)
- [ ] **DPO corpus audit** — after Fix C or drain stability confirmed, run `export-dpo.sh` and verify ≤ 5 tuples output; `corpus-threshold.py` honest count check

---

## §5 — Leapfrog 2030 trajectory pointer

Full competitive analysis in `BRIEF-slm-learning-loop.md §6`. Three moats:
1. OLMo verifiable training provenance (OLMoTrace) — cite in regulated-sector procurement
2. Per-inference Ed25519 signed audit trail — P0 ledger sha256 is the next step
3. Customer-owned compounding LoRA weights

**Next milestone:** stable drain (Fix C if needed) → Yo-Yo 1h test → first non-degenerate
DPO tuples → Sunday 02:00 UTC `corpus-threshold.py` threshold reached → LoRA training trigger.
