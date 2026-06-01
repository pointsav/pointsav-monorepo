---
artifact: brief
status: active
created: 2026-06-01
updated: 2026-06-01 (Yo-Yo Tier B validated; /v1/extract grammar fix RESOLVED)
author: totebox@project-intelligence (claude-sonnet-4-6)
replaces: BRIEF-active-work.md (missing — never existed on disk)
companion:
  - BRIEF-slm-substrate-master.md  (SLM operations reference; §2.8 = Yo-Yo findings)
  - BRIEF-slm-learning-loop.md     (training pipeline spec; §9/§10 = SFT-first plan)
---

# BRIEF — project-intelligence Active Work

> **Session-start reading.** Read this before asking what to work on.
> Companions: substrate master (Yo-Yo ops, tier routing), learning loop (training spec, corpus).

---

## Current service state (2026-06-01)

| Service | State | Note |
|---|---|---|
| `local-doorman.service` | **active** | New binary `2c96603b` (Tier B grammar fix); Tier A primary; `/healthz` returns plain `ok` (no entity_count — known gap); `/readyz` OK incl. P1 zone field |
| `local-slm.service` (Tier A) | **active** | OLMo 2 7B Q4_K_M; idle ~2% (wedge fixed — was 69 h stuck); serves interactive in ~2–4s |
| `service-content` | **active** | 7,203 entities; **Tier A fallback persisted OFF** (drop-in) → defers the 41k backlog safely |
| `yoyo-tier-b-1` (Tier B) | **TERMINATED** (stopped) | Stockout cleared 2026-06-01; tested + validated; stopped to save cost. `start-yoyo.sh` to bring up |
| Apprenticeship drain | **paused** | `SLM_DRAIN_PAUSED=true` — CPU drain off; capture continues |
| Apprenticeship queue | **~170 pending / 550 done / 0 poison** | poison cleared session 13; capture live |
| `local-orchestration-slm.service` | **inactive** | Operator deploy pending (§3) |

---

## §0 — Resolved this session (2026-06-01) — read before picking work

- ✅ **Tier B `/v1/extract` grammar — FIXED + validated live.** `yoyo.rs` sent the schema in
  vLLM `extra_body` format; server is llama.cpp (top-level `json_schema`/`grammar`). Fixed →
  live: 7.2s, `extraction_ok:true`, 4 entities classified. Commit `dee8d050`; binary `2c96603b`.
  Full detail: BRIEF-slm-substrate-master.md §2.8.
- ✅ **Yo-Yo truncation** — image had `-c 4096 -np 4` (1024 tok/slot). Packer template fixed to
  `-np 1` + `-fa on` (`compute/packer/scripts/llama-server.service`, commit `3b8a952e`).
- ✅ **Tier A wedge** — 69 h stuck llama-server cleared (restart); now idle/healthy.
- ✅ **service-content was DOWN** — started; 7,203 entities; fallback persisted OFF.
- ✅ **Local test series** — cargo (191+10 pass), Doorman endpoints, drain-pause, graph proxy.
- ✅ **P1/P2/Sprint 4a** (sessions 13/14): /readyz reason+zone, service-content base_dir,
  app-console-slm status.
- ✅ **Brief consolidation** (session 13): archived contamination; AI-AUDIT integrated.

**Open for Command (outbox):** Stage 6 promote (`dee8d050`, `3b8a952e`); commit binary ledger
+ fallback drop-in mirror; Yo-Yo Packer rebuild now SAFE to queue (persists -np/-fa).

---

## §1 — Poison queue (RESOLVED session 13)

Was 78 entries; investigated (68 pre-Fix-A empty-diff → quarantined; 10 llama-server-outage
artifacts → recovered). **queue-poison now 0.** Kept for reference:

Newest entries (May 31 04:47–04:58 UTC, post-Fix-B deploy at 00:41 UTC):
- `actual_diff: ""` at top level — Fix A was not applied to these entries
- No `response_raw`, no `escalate` field — never dispatched to OLMo

**Two hypotheses (H1 confirmed):**
- **H1 (confirmed)**: Pre-Fix-A briefs carried forward from session 11's recovery batch.
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

- [ ] **P3: orchestration-slm persistence** — replace ephemeral HashMap metering with Redb or SQLite (1 session)
- [ ] **SFT extraction script** — `scripts/extract-sft-pairs.py`: read `queue/*.jsonl`, filter `actual_diff != ""`, output clean SFT JSONL for LoRA. Target: 77 post-Fix-A entries.
- [ ] **CodeDPO scaffold** (GPU-gated) — script to generate N candidate diffs per brief via OLMo 3 32B-Think, run `cargo check`, output validated DPO pairs. Only run on Yo-Yo.
- [ ] **LoRA fine-tuning first run** — after SFT extraction + CodeDPO pairs ready. Checklist: BRIEF-slm-learning-loop.md §9. Rank=16, alpha=32, 5–10 epochs.
- [ ] **Exclude corrupt DPO tuples** — the 548 `training-corpus/apprenticeship/shadow-capture/` tuples have empty OLMo diffs and are HARMFUL for DPO. Quarantine before training run.
- [ ] **drain-apprenticeship.service/timer cleanup** — disabled units in `/etc/systemd/system/`; remove
- [ ] **Stale shim test fields** — `anthropic_shim_test.rs` `tier_a_reason` / `idle_monitor`
- [ ] **Claude Code CORPUS bridge deployment** — lower priority now; SFT path uses queue/ directly

---

## §5 — Leapfrog 2030 + training architecture (revised 2026-05-31)

Full competitive analysis in `BRIEF-slm-learning-loop.md §6`. Three moats:
1. OLMo verifiable training provenance (OLMoTrace) — cite in regulated-sector procurement
2. Per-inference Ed25519 signed audit trail — P0 ledger sha256 is the next step
3. Customer-owned compounding LoRA weights

**Revised milestone (2026-05-31):** CPU drain paused (`SLM_HOLD_THRESHOLD_SECS=1`).
Next path → **SFT first**: extract 77 post-Fix-A queue entries → LoRA SFT run on GPU →
validate model improvement on held-out Foundry commits → THEN CodeDPO on Yo-Yo →
DPO fine-tune on execution-validated pairs. Fix C deferred — GPU OLMo 3 32B-Think
handles format constraints natively.

**What NOT to do:**
- Do NOT train on the 548 existing shadow-capture tuples (empty OLMo diffs — harmful)
- Do NOT run CPU drain for DPO — OLMo 7B cannot generate useful code critique
- Do NOT combine SFT+DPO at this data scale (<5K samples) per research findings
