---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-active-work
title: "project-intelligence Active Work — Memory Hardening + Drain"
status: archived
owner: project-intelligence
created: 2026-06-01
updated: 2026-06-12
author: totebox@project-intelligence (claude-sonnet-4-6)
replaces: BRIEF-active-work.md (missing — never existed on disk)
companion:
  - BRIEF-slm-substrate-master.md
  - BRIEF-slm-learning-loop.md
moved_to: project-intelligence
archived: 2026-06-12
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

## §0 — Resolved this session (2026-06-01)

- ✅ SLM substrate testing — ALL 4 LAYERS DONE (2026-06-02)
- ✅ Persistent `processed_ledgers` — DONE (commit `5ad06ec9`, binary deployed)
- ✅ P0: Doorman audit sha256 — DONE (commit `3a64431e`, binary deployed)
- ✅ Preemption-safe DataGraph watcher — FIXED + DEPLOYED (commit `a5f573f6`)
- ✅ Tier B `/v1/extract` grammar — FIXED + validated live (commit `dee8d050`)
- ✅ Yo-Yo truncation — image fixed to `-np 1` + `-fa on` (commit `3b8a952e`)
- ✅ Tier A wedge — cleared (restart)
- ✅ Brief consolidation (session 13): archived contamination; AI-AUDIT integrated

---

## §1 — Poison queue (RESOLVED session 13)

Was 78 entries; investigated (68 pre-Fix-A empty-diff → quarantined; 10 llama-server-outage
artifacts → recovered). **queue-poison now 0.**

---

## §2 — Next items: short-term queue (updated 2026-06-01)

- [ ] **SFT extraction script** — `service-slm/scripts/extract-sft-pairs.py`
  Extract 544 ground-truth pairs from `queue-done/*.jsonl` for LoRA training.

- [ ] **Stage 6** — commits pending promote after this session's work

- [ ] **Disabled systemd unit cleanup** — check/remove `drain-apprenticeship.service`/timer

- [ ] **Yo-Yo 1h test** — when us-central1-a L4 capacity returns:
  `service-slm/scripts/start-yoyo.sh --wait-ready=120 --runtime=1h`

---

## §3 — Operator-gated actions (require sudo or GCP console)

1. Deploy orchestration-slm binary
2. Install chassis env at `/etc/foundry/local-orchestration-slm.env`
3. Enable `local-orchestration-slm.service`
4. Wire Doorman: `SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180`
5. Stage 6 promote via Command `bin/promote.sh`

---

## §4 — Medium-term (sessions 16–20)

- [ ] P3: orchestration-slm persistence (Redb/SQLite)
- [ ] SFT extraction script → first LoRA fine-tuning run
- [ ] CodeDPO scaffold (GPU-gated)
- [ ] Exclude corrupt DPO tuples (548 empty-OLMo-diff entries)
- [ ] drain-apprenticeship.service/timer cleanup

---

## §5 — What NOT to do

- Do NOT train on the 548 existing shadow-capture tuples (empty OLMo diffs — harmful)
- Do NOT run CPU drain for DPO — OLMo 7B cannot generate useful code critique
- Do NOT combine SFT+DPO at this data scale (<5K samples)
