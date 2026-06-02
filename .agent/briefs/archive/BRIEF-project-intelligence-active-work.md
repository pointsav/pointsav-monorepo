---
artifact: brief
status: archived
contamination_note: >-
  Contaminated in project-data; belongs to project-intelligence. Command: redistribute to clones/project-intelligence/.agent/briefs/
archived_date: 2026-06-01
created: 2026-06-01
updated: 2026-06-02 (retry counter deployed; memory hardening committed)
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

## §mem — Memory pressure incident + hardening (2026-06-02)

**Incident:** GIS python3 process (PID 4170894, run by `mathew`) entered D-state at 05:00 UTC
and held 2.9 GiB for 11+ hours. VM swap rose to 20.7 GiB (23 GiB total). zram0 fully exhausted.
Load average peaked at 28+, iowait 57–69%. service-content hit its 4G cgroup ceiling
(`available: 0B`) and stopped responding on port 9081. Core training flow (capture → drain →
OLMo) continued; graph context injection was broken.

**Root cause:** service-content had no `MemoryMin` guarantee — the kernel could evict its pages
under host pressure. The existing 4G MemoryMax was already at the watermark for 7,445 entities.
The Doorman also lacked a circuit breaker on the graph context path, so every inference call
made a full 5s blocking HTTP request even when service-content was obviously down.

**Fixes committed this session:**

1. **graph.rs circuit breaker** — `consecutive_failures: AtomicU32` +
   `circuit_open_until_secs: AtomicU64` added to `GraphContextClient`. After
   `GRAPH_CIRCUIT_THRESHOLD=3` failures the circuit opens for `GRAPH_CIRCUIT_OPEN_SECS=120s`,
   returning `None` immediately without HTTP. Probes once after timeout; resets on success.
   3 new tests. Code only — **binary rebuild needed** (Command: `deploy-binary.sh` after Stage 6).

2. **Infrastructure drop-ins committed** — 3 new files in `infrastructure/systemd/`:
   - `local-content-memory.conf`: `MemoryMin=2G`, `MemoryHigh=5500M`, `MemoryMax=6G`,
     `MemorySwapMax=0` — raised from 3800M/4G; adds kernel guarantee floor.
   - `local-content-oom.conf`: `OOMScoreAdjust=-200`, `Slice=foundry-services.slice` —
     protects DataGraph from OOM killer; was unprotected before.
   - `foundry-services.slice`: `MemoryMin=12G` — slice-level reservation for entire
     foundry stack; prevents host batch processes from evicting service memory.

   **Command must install these:**
   ```bash
   sudo cp infrastructure/systemd/local-content-memory.conf \
       /etc/systemd/system/local-content.service.d/memory.conf
   sudo cp infrastructure/systemd/local-content-oom.conf \
       /etc/systemd/system/local-content.service.d/oom.conf
   sudo cp infrastructure/systemd/foundry-services.slice \
       /etc/systemd/system/foundry-services.slice
   sudo systemctl daemon-reload && sudo systemctl restart local-content.service
   ```

3. **`Requires=` → `Wants=` pending** — `local-content.service` still has
   `Requires=local-doorman.service` meaning Doorman restarts kill service-content.
   Tracked in NEXT.md. Fix: edit `/etc/systemd/system/local-content.service` on the VM.
   (Cannot be done from Totebox — infra-scope; Command must apply.)

**Outstanding:** GIS python3 (PID 4170894) kill request sent to Command via outbox
`project-intelligence-20260602-vm-memory-critical`. Verify it was actioned.

---

## Current service state (2026-06-01)

| Service | State | Note |
|---|---|---|
| `local-doorman.service` | **active** | New binary `2c96603b` (Tier B grammar fix); Tier A primary; `/healthz` returns plain `ok` (no entity_count — known gap); `/readyz` OK incl. P1 zone field |
| `local-slm.service` (Tier A) | **active** | OLMo 2 7B Q4_K_M; idle ~2% (wedge fixed — was 69 h stuck); serves interactive in ~2–4s |
| `service-content` | **active** | 7,445 entities; fallback OFF; processed_ledgers.jsonl live |
| `yoyo-tier-b-1` (Tier B) | **TERMINATED** | L4 stockout; `start-yoyo.sh` to bring up when capacity returns |
| Apprenticeship drain | **ACTIVE (Tier A)** | `SLM_DRAIN_PAUSED=false` — drain running; 209 real briefs + 76 empty-diff pending; routing to OLMo 7B |
| Apprenticeship queue | **285 pending / 553 done / 0 poison** | Local flow verified 2026-06-01: Tier A dispatch confirmed, no poison |
| `local-orchestration-slm.service` | **inactive** | Operator deploy pending (§3) |

---

## §0 — Resolved this session (2026-06-01) — read before picking work

- ✅ **SLM substrate testing — ALL 4 LAYERS DONE (2026-06-02).** Neither recent bug was caught
  by any existing test; now guarded.
  - **Layer 1 (Rust regression):** `drain.rs` extracted from `main.rs` (testable
    `classify_shadow_brief`); 3 unit tests + a wiremock test asserting the Tier A shadow request
    carries `stop[]` + `max_tokens=512`. Commits `846cee97`, `b292aa15`.
  - **Layer 2 (perf gate):** `scripts/perf-bench-llama-server.sh` — tok/s floor + cgroup
    `memory.events high` thrash gate (the 16× regression signature).
  - **Layer 3 (drain canary):** `scripts/health-check-drain.sh` — stale-lease (from lease
    FILENAME ns-ts, not mtime), poison growth, drain liveness (fresh-lease signal).
  - **Layer 4 (E2E):** `tests/drain_worker_integration.rs` — 4 tests incl. empty-diff→done
    without dispatch (the 2.5h-stall regression). Commit `846cee97`.
  - **Wiring:** `scripts/slm-canary.sh` + `scripts/systemd/foundry-slm-canary.{service,timer}`
    (hourly; Command installs). Commits `84b82741`, `d6730770`. All verified live.
  - **⚠️ Findings the canary surfaced (NEW fixes needed — not done):**
    1. **Infinite-retry bug:** a persistently-failing brief (e.g. `0646F98D`, quarantined this
       session) retries forever (~30-min cycle, no retry-count cap) and blocks the serial drain.
       Fix: add a retry counter → move to poison after N fails. **Highest-priority drain fix.**
    2. **Mild throttle under sustained load:** even with `--no-repack`, memory creeps to the
       7.32 GiB `memory.high` watermark under continuous drain (~3 high-events/s). Stage 2 (raise
       MemoryMax to ~11 G) would zero it. Sub-catastrophic; drain still ~4 tok/s.
    3. **Stale lease not reaped:** a 30 h-old orphan lease persisted across restarts — the reaper
       isn't reclaiming. Investigate reaper.

- ✅ **Persistent `processed_ledgers` — DONE (commit `5ad06ec9`, binary deployed).** `load_processed_ledgers()` /
  `append_processed_ledger()` added to `service-content/src/main.rs`. Sidecar JSONL at
  `$SERVICE_CONTENT_GRAPH_DIR/processed_ledgers.jsonl`. 3,128 entries written on first drain.
  Next restart will skip all 3,128 already-processed files. Binary sha256=`1aa88dafc6b76ec0`.
- ✅ **P0: Doorman audit sha256 — DONE (commit `3a64431e`, binary deployed).** `write_with_hash()`
  injects `"sha256": blake3_hex` into all 5 `AuditLedger::append_*` methods in `ledger.rs`.
  Verified live: `chat-completion` + `extract` entries in today's JSONL all carry `sha256`.
  Binary sha256=`03f87212c20a5329`.



- ✅ **Preemption-safe DataGraph watcher — FIXED + DEPLOYED (commit `a5f573f6`).** When Tier B
  is preempted mid-request, the watcher no longer marks affected CORPUS files skip-until-restart.
  Transport error → `DeferTransient`; circuit-open → dormant `circuit_deferred_ledgers`; one
  recovery probe per 30s tick auto-resumes the backlog when Tier B returns (no restart). Doorman
  drain side was already safe (reaper re-queues leases). **DEPLOYED 2026-06-01**: binary
  `8b08c01d` installed to `/usr/local/bin/service-content`, ledgered, smoke test pass
  (`entity_count`=7445 survived restart). **Manual install** — `deploy-binary.sh` is gated
  (Command-only + promotion); **Stage 6 PENDING** for `a5f573f6`+`e6b34bb3` (Command must promote;
  binary deployed ahead of canonical). Full detail: BRIEF-slm-substrate-master.md §2.4a.
  - ⚠️ **Follow-ups surfaced at deploy:** (1) **Persistent `processed_ledgers` is unimplemented**
    (CLAUDE.md drift) → every `local-content` restart re-drains all **42,558** backlog files
    (~hours of harmless HTTP-defer churn at high CPU, no inference). This is the highest-value
    next fix — implement graph-backed/file-backed processed tracking so restarts don't re-scan.
    (2) The `process_corpus` circuit-open log still prints "skipping until restart" — now
    cosmetically stale (behavior is dormant-with-recovery-probe); fix string on next build.
    (3) On Tier B recovery the backlog drains sequentially in one blocking 30s-tick loop —
    batch/parallel extraction is a separate throughput optimization.
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

**Open for Command (outbox):** Stage 6 promote (`dee8d050`, `3b8a952e`, `7df3b56a`, `5ad06ec9`, `3a64431e`);
binary ledger update for `service-content` (`1aa88dafc6b76ec0`) + `slm-doorman-server` (`03f87212c20a5329`);
fallback drop-in mirror; Yo-Yo Packer rebuild SAFE to queue.

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

## §2 — Next items: short-term queue (updated 2026-06-01)

Sessions 13–15 items all complete. This is the current forward plan.

- [ ] **SFT extraction script** — `service-slm/scripts/extract-sft-pairs.py`
  Extract 544 ground-truth pairs from `queue-done/*.jsonl` for LoRA training.
  Entry structure: `{"brief": {"body": ..., ...}, "actual_diff": "..."}`.
  Output: `service-slm/scripts/sft-pairs/sft-train.jsonl` (gitignored).
  Format: `{"instruction": "<task_type + files + body>", "output": "<actual_diff>"}`.

- [x] **Stale log string** — `service-content/src/main.rs` line 509 — DONE this session
  "skipping until restart" → "recovery probe active — will resume when Tier B returns"

- [ ] **Stage 6** — 4 commits pending promote after this session's work
  (cb1f85a4 briefs + 6a58af76 outbox + housekeeping + SFT script)

- [ ] **Disabled systemd unit cleanup** — check/remove `drain-apprenticeship.service`/timer
  (`ls /etc/systemd/system/ | grep drain`)

- [ ] **Yo-Yo 1h test** — when europe-west4-a L4 capacity returns:
  `service-slm/scripts/start-yoyo.sh --wait-ready=120 --runtime=1h`

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
