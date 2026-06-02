---
artifact: brief
status: active
redistribution_note: >-
  Previously contamination-flagged (landed in project-data via Stage-6 rebase 2026-05-22).
  Now confirmed correct location: clones/project-intelligence/.agent/briefs/archive/.
  Reactivated 2026-06-02 — Cloud Run migration plan added (§10).
title: SLM Substrate Master — Yo-Yo + DataGraph + Learning Loop
created: 2026-05-24
updated: 2026-06-02 (§10 added: Cloud Run migration plan for Tier B)
author: totebox@project-intelligence (claude-sonnet-4-6)
grounds_in:
  - service-slm/ARCHITECTURE.md
  - service-slm/docs/deploy/deploy-yoyo-tier-b.md
  - service-slm/scripts/start-yoyo.sh
  - service-content/CLAUDE.md
  - DOCTRINE.md claims #49, #54
  - conventions/four-tier-slm-substrate.md
replaces:
  - BRIEF-flow-restructure.md (deleted — Stage-6 rebase contamination 2026-05-22)
  - BRIEF-vm-hardening-and-consolidation.md (absorbed)
  - BRIEF-service-content-architecture.md (absorbed)
  - BRIEF-sovereign-routing-comprehensive.md (absorbed)
  - BRIEF-universal-ai-gateway.md (absorbed)
  - BRIEF-learning-loop-master-plan.md (deferred items carried forward to §6)
  - BRIEF-tier-architecture.md (absorbed)
  - BRIEF-phase-3c-service-content-loRA-stub.md (deferred items carried forward to §6)
notes: >
  BRIEF-flow-restructure.md was the PRIMARY PLAN OF RECORD. It no longer exists on disk
  (Stage-6 rebase 2026-05-22 overwrote .agent/ with project-knowledge content).
  This BRIEF is its successor, reconstructed from session context (Sessions 4/6/16),
  service-slm/NEXT.md, ARCHITECTURE.md, and the deploy runbook. All remaining items
  in the absorbed BRIEFs are carried forward into §5 and §6 below.
---

# BRIEF — SLM Substrate Master

> **This is the PRIMARY PLAN OF RECORD** for the service-slm / service-content /
> Yo-Yo substrate. All SLM engineering sessions read this first.
>
> Reference docs (do not duplicate here): `service-slm/ARCHITECTURE.md`,
> `service-slm/docs/deploy/deploy-yoyo-tier-b.md`, `service-content/CLAUDE.md`.

---

## §1 — Current live state (as of 2026-05-30T22:17Z — session 10: lease expiry fix + flow confirmed)

| Component | Version | Status | Notes |
|---|---|---|---|
| `slm-doorman-server` | rebuilt 2026-05-30T21:14Z | **active** | sha256=`bd91eafc7c2a232c10e0c449f31474d9d994568df9c4054eb8f591f93ce3360d`; **Sprint 3D: timeout 1800s**; Sprint 3A: `SLM_TIER_A_FIRST=true`; Sprint 3C: drain hold bypass; commit `1398522b` |
| `service-content` | rebuilt 2026-05-29T19:26Z | **active** | sha256=`2362ea5c580a9869c5e307b645d60219cb9535dbf4218bd8762da870a4c62f7b`; Sprint 2A: `entity_count` in `/healthz` (7,201 entities); Sprint 3B: Tier A fallback enabled; commit `5493a8f4` |
| `orchestration-slm-server` | built session 9 | **NOT YET DEPLOYED** | unit file committed `d445b5ea`; Command operator install needed (see outbox `project-intelligence-20260530-stage6-orchestration-deploy`) |
| `yoyo-tier-b-1` | 2026-05-13 Packer image | **TERMINATED** | europe-west4-a L4 stockout; restart with `start-yoyo.sh --wait-ready=120 --runtime=1h` when capacity returns |
| `local-slm.service` | OLMo 2 1124 7B Instruct Q4_K_M (4.16 GiB) | **active** | Tier A is the confident primary; `SLM_TIER_A_FIRST=true` confirmed in startup log |
| `local-doorman.env` | — | current | `SLM_TIER_A_FIRST=true`; `SLM_HOLD_THRESHOLD_SECS=3600`; `SLM_APPRENTICESHIP_ENABLED=true`; `SLM_FORCE_BROKER_MODE=false`; **`SLM_QUEUE_LEASE_EXPIRY_SEC=2100`** (session 10) |
| `local-content.env` | — | current | Drop-in: `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=true`; `SERVICE_CONTENT_TIER_A_FALLBACK_INTERVAL_SECS=300` |

**Tier routing (current):**
- Tier A: **ENABLED + PRIMARY** — `SLM_TIER_A_FIRST=true`; all chat/shadow routes here unless Tier B explicitly hinted AND circuit closed
- Tier B: **circuit initialising → OPEN** — Yo-Yo TERMINATED; health probes failing; circuit will open within ~90s of restart
- Tier C: not configured — ToS hard constraint; never enable for training loop
- Result: `ai_available: true` (Tier A primary); WATCHER Tier A fallback active (rate-limited, 300s interval); drain holds when all Tier B open >1h

**Think-model fixes deployed (prev session commit `d835cab5`):**
- `SOCKET_TIMEOUT` raised 60s → 180s; `OUTER_DEADLINE` raised 90s → 300s
- `strip_think_blocks()` added to extract handler — strips `<think>...</think>` before JSON parse
- Shadow briefs capped at `max_tokens: 2048` (prevents runaway Think generation)
- Root cause: OLMo-3 32B Think spends ~500 tokens on reasoning before JSON answer; 60s timeout
  fired before `</think>` was emitted; new 180s timeout + stripping fixes this

**Flow debug session fixes (this session — code complete; binaries need rebuild):**
- `reasoning_content` field added to `ComputeResponse`; extract handler uses it when
  `--reasoning-format deepseek` is active (clean JSON in content; no stripping needed)
- reqwest decode errors (issue #2839) reclassified as `TierBTimeout` for correct backoff
- `start-yoyo.sh` `update_doorman_env()` now restarts local-doorman.service after writing
  new IP to env file — previously IP was written but Doorman kept running with old endpoint
- `llama-server.service` Packer template: `-fa`, `--reasoning-format deepseek`,
  `--reasoning-budget 1024` added (active after next Packer rebuild)
- service-content SC-3 (Doorman startup health-check), SC-5 (log CORPUS errors),
  SC-2 (defer_reason differentiation), SC-3d (30s retry loop), SC-3e (graph-first write),
  SC-3f (buffer pool env var) — all in commit `e263d6f0`

**Shadow capture state (2026-05-31 session 13):**
- queue/: 5 pending briefs
- queue-done/: 550 briefs (dispatched)
- queue-poison/: 78 files (up from 0 at session 11 close; newest have `actual_diff: ""` + no `response_raw` — never dispatched to OLMo; root cause under investigation — see §5)
- Training corpus: 591 DPO tuples (DEGENERATE — see §Circuit resilience plan) + 1,410 engineering SFT tuples (valid)

**Stage 6 state:** archive ahead of `origin/main`; rebase required per inbox
`command-20260520-stage6-rebase-required` before promote.

---

## §2 — The Yo-Yo VM: what "permanent" means

> This section answers: why do we keep the same VM and disk rather than provisioning fresh?

### §2.1 — What is permanent (and what is not)

**Permanent (persists across stop/start cycles):**
- `yoyo-tier-b-1` — the GCE instance definition (name, zone, machine type, firewall rule, IAM bindings)
- `yoyo-tier-b-1-weights` — the 256GB `pd-balanced` disk attached to the VM in europe-west4-a
- Model weights — OLMo-3-1125-32B-Think Q4_K_M (~20GB GGUF) pre-loaded on the weights disk
- Bearer token — stored in GCE instance metadata; used by Doorman to authenticate every request
- OpenTofu state — `service-slm/compute/opentofu/` tracks the VM, disk, firewall, IAM

**NOT permanent (intentional):**
- The running VM instance — `g2-standard-4` Spot VM with L4; Google can preempt it within ~24h
- The running vLLM process — restarts each time the VM boots (~2 min to load weights from disk)
- KV cache (Ring 2) — ephemeral; rebuilt per session

**Why this design:**
- Model weights take 30–60 min to download from scratch (or from GCS). Pre-loading on disk reduces restart to ~2 min.
- Spot pricing saves ~70% vs on-demand; preemption is accepted in exchange.
- The persistent disk survives preemption — Google stops the VM, the disk stays attached.

### §2.2 — Zone discipline: why europe-west4-a is locked

The 256GB weights disk lives in europe-west4-a. GCE persistent disks are zone-bound — you
cannot attach a disk from europe-west4-a to a VM in any other zone. Therefore:

**The VM MUST restart in europe-west4-a (Mode 1 restart).** Zone fallback (Mode 2) creates
a new disk in a different zone — the weights are NOT there and must be re-downloaded.

> **POLICY — NO ZONE FALLBACK FOR STOCKOUTS.**
> `SLM_YOYO_ALLOW_ZONE_FALLBACK` MUST remain `false` (its default). Zone fallback (Mode 2)
> is a migration-only tool — it is NEVER a response to a stockout. Provisioning an alternative
> VM to work around a stockout creates a VM with an empty weights disk in the wrong zone,
> costs money to clean up, and creates confusion about which VM is authoritative. The cost
> of waiting is zero. The cost of a misplaced VM is not.

If europe-west4-a has no L4 capacity (stockout, `start-yoyo.sh` exit code 3):
- **Wait 15–30 min and retry.** Stockouts are transient; capacity rotates continuously.
- Use `--retry-cycles=6 --retry-wait-seconds=600` to retry automatically over 1 hour:
  ```bash
  service-slm/scripts/start-yoyo.sh --runtime=2h --retry-cycles=6 --retry-wait-seconds=600
  ```
- If capacity does not return within a day, flag in inbox and wait. Do NOT provision elsewhere.

### §2.3 — Daily operation: starting the Yo-Yo

```bash
# From the project-intelligence archive root:
service-slm/scripts/start-yoyo.sh --runtime=2h

# Exit codes:
# 0 = VM started; Doorman circuit closes within 30s of next probe
# 3 = L4 stockout in europe-west4-a; wait 10–30 min and retry
# 4 = daily budget cap hit ($3/day default); check SLM_YOYO_DAILY_BUDGET_USD
```

After exit 0: no manual env update needed. `yoyo-tier-b-1` retains its external IP
(`34.6.204.25`) across stop/start cycles — it is a static assignment.

**Verifying the Doorman closed the circuit:**
```bash
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
# Expect: "tier_b_circuit_state": "closed", "ai_available": true
```

**VM auto-stops via two independent mechanisms (whichever fires first):**
1. **Idle monitor** (Doorman-side): stops the VM after 30 min of no inference requests
   (`SLM_YOYO_IDLE_MINUTES=30` in `local-doorman.env`)
2. **Dead-man's switch** (VM-side `yoyo-deadman.service`): stops the VM after `--runtime` wall
   clock expires, even if Doorman is unreachable (guards against billing runaway)

### §2.4 — If the VM gets preempted

Google preempts Spot VMs without warning. When preempted:
- The VM stops; `last-stop-reason=preempted` is written to instance metadata
- The weights disk is unaffected — stays attached, ready for next boot

**With `SLM_YOYO_AUTO_RESTART=false` (current setting):**
The Doorman does not attempt to restart the VM. Operator must restart manually:
```bash
service-slm/scripts/start-yoyo.sh --runtime=2h
```

**To enable auto-restart** (adds cost risk if circuit stays closed after preemption):
```bash
sudo sed -i 's/SLM_YOYO_AUTO_RESTART=false/SLM_YOYO_AUTO_RESTART=true/' \
    /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
```

### §2.5 — Zone fallback (Mode 2): planned migration only

Mode 2 provisions a new VM in a different zone. **It is NOT for stockouts — see §2.2 policy.**
Use ONLY when executing a deliberate zone migration (rare, operator-approved, planned in advance).

**Mode 2 requires manual follow-up steps (the script prints these on exit 0):**
1. Restore weights to the new disk — `vllm-weights-prep.service` pulls from GCS bucket
   `woodfine-node-gcp-free-foundry-substrate` using snapshot `SLM_YOYO_WEIGHTS_SNAPSHOT`
   if set. Takes 30–60 min. Monitor: `gcloud compute ssh yoyo-tier-b-1 --zone=<new-zone> --command='journalctl -u vllm-weights-prep -f'`
2. Update zone + IP in Doorman env:
   ```bash
   NEW_ZONE=<zone>; NEW_IP=$(gcloud compute instances describe yoyo-tier-b-1 \
     --zone=${NEW_ZONE} --project=woodfine-node-gcp-free \
     --format='value(networkInterfaces[0].accessConfigs[0].natIP)')
   sudo sed -i "s|^SLM_YOYO_GCP_ZONE=.*|SLM_YOYO_GCP_ZONE=${NEW_ZONE}|" /etc/local-doorman/local-doorman.env
   sudo sed -i "s|^SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=https://${NEW_IP}:9443|" /etc/local-doorman/local-doorman.env
   sudo sed -i "s|^SLM_YOYO_TRAINER_ENDPOINT=.*|SLM_YOYO_TRAINER_ENDPOINT=https://${NEW_IP}:9443|" /etc/local-doorman/local-doorman.env
   sudo sed -i "s|^SLM_YOYO_GRAPH_ENDPOINT=.*|SLM_YOYO_GRAPH_ENDPOINT=https://${NEW_IP}:9443|" /etc/local-doorman/local-doorman.env
   sudo systemctl restart local-doorman.service
   ```
3. Update `service-slm/compute/opentofu/main.tf` zone variable + `tofu apply` (Command Session)
4. Take a new weights snapshot in the new zone for future Mode 1 restarts (§2.6)

### §2.6 — Weights snapshot management

A snapshot allows Mode 2 zone fallback to restore weights quickly instead of re-downloading.
Current snapshot: `yoyo-tier-b-1-weights-20260513-1923` in `woodfine-node-gcp-free-foundry-substrate`

To take a new snapshot (after any model update, or post-zone-migration):
```bash
service-slm/scripts/create-yoyo-snapshot.sh
```
Update `SLM_YOYO_WEIGHTS_SNAPSHOT` in `/etc/local-doorman/local-doorman.env` with the
new snapshot name. The script sets `instance-termination-action=STOP` so the disk persists.

### §2.7 — First-time setup (rebuild from scratch)

Only needed if the VM and disk are deleted and must be recreated from nothing.

Full procedure: `service-slm/docs/deploy/deploy-yoyo-tier-b.md`

Summary:
1. Build Packer image: `cd service-slm/compute/packer/ && packer build yoyo-image.pkr.hcl` (~20–30 min)
2. Provision infrastructure: `cd service-slm/compute/opentofu/ && tofu apply -var bearer_token=... -var workspace_ip=...`
3. Upload weights to the new disk via `gcloud compute scp` or GCS download
4. Set bearer token in instance metadata
5. Wire Doorman env (`SLM_YOYO_ENDPOINT`, `SLM_YOYO_BEARER`, zone, etc.)
6. Restart Doorman; verify circuit closes

**Not needed now** — `yoyo-tier-b-1` and its weights disk are intact in europe-west4-a.

---

## §3 — DataGraph flow (service-content ↔ Doorman ↔ Yo-Yo)

How data moves through the substrate once the Yo-Yo is running:

```
CORPUS_<worm_id>.json files (from service-extraction)
    ↓
service-content corpus drain loop
    ├── Sprint 5: is_already_processed(worm_id) → graph query → SKIP if extracted
    ├── Sprint 2: write Source entity to graph BEFORE calling Doorman
    └── POST /v1/extract → Doorman
                ↓
           Doorman routes to Yo-Yo (Tier B)
                ↓
           OLMo-3-32B-Think extracts entities
                ↓
           POST /v1/graph/mutate → Doorman proxies to service-content
                ↓
           Entities written to LadybugDB (Tier A hardware) or SQLite (Micro)

On every /v1/messages call:
    Doorman → GET /v1/graph/context → service-content
    Returns entity context → injected into system prompt
```

**Current state with Yo-Yo TERMINATED:**
- `source node written` → logged per CORPUS file
- `extraction deferred — tier B unavailable` → logged; CORPUS file will not be re-tried
  on restart because Sprint 5 `is_already_processed` checks for non-Source entities.
- When Yo-Yo starts, only CORPUS files WITHOUT extracted entities will be processed.
  Files with a Source node but no extracted entities are picked up automatically.

**Key env vars (service-content):**

| Var | Value | Purpose |
|---|---|---|
| `SERVICE_CONTENT_GRAPH_BACKEND` | `lbug` | LadybugDB on hardware-class nodes |
| `SERVICE_CONTENT_LBUG_BUFFER_POOL_MB` | `2048` | 2GB buffer pool |
| `SERVICE_CONTENT_DOORMAN_ENDPOINT` | `http://127.0.0.1:9080` | Doorman for Tier B extraction |

---

## §4 — Tier routing reference

| Tier | Model | Host | Gate | Current |
|---|---|---|---|---|
| A | OLMo 2 1124 7B Instruct Q4_K_M | workspace VM (`local-slm.service`) | NOT force-broker-mode | DISABLED — `SLM_FORCE_BROKER_MODE=true` — re-enable Sprint 0 |
| B | OLMo-3-32B-Think Q4_K_M | `yoyo-tier-b-1` GCE L4 | Tier B circuit Closed | CIRCUIT OPEN (VM terminated) |
| C | External API (Anthropic) | external | `ANTHROPIC_API_KEY` set, Tier B unavailable | NOT CONFIGURED |

**LatencyClass routing** (shipped Session 6, 2026-05-23):
- `Interactive` / `Background` → Tier A first, fallback Tier B
- `Batch` → Tier B (Yo-Yo) first — corpus extraction uses Batch

**Tier A is now the enabled primary (2026-05-29):**
`SLM_FORCE_BROKER_MODE=false` confirmed in live env. OLMo 2 1124 7B Instruct Q4_K_M runs
as `local-slm.service`. Goose round-trip verified 2026-05-29T04:10Z (`tier="local"`).
Yo-Yo (Tier B) is the optional nightly accelerator — circuit OPEN due to VM termination.

---

## §4b — Circuit resilience plan (active, 2026-05-29)

Five concrete defects arise when Tier B has been unavailable for 1,460+ consecutive requests.
Full plan: `/home/mathew/.claude/plans/make-plan-for-what-fluffy-whale.md`

**The five defects:**
1. **591 degenerate DPO tuples** — shadow briefs escalate to Tier B; `attempt.diff=""` → empty rejected sample → meaningless training signal
2. **readyz lies** — `has_yoyo: true` reflects config presence, not circuit breaker runtime state
3. **entity_count always 0** — `/healthz` has no entity_count field; monitors see 0 via `jq .entity_count // 0`
4. **WATCHER stalls** — all CORPUS files marked skip-until-restart when Tier B unavailable
5. **Drain worker accumulates poison** — no circuit-aware pause; queue fills with unprocessable briefs

**Sprints:**

| Sprint | Scope | Files | Status |
|---|---|---|---|
| Sprint 0 | Documentation + artifact delivery | drafts-outbound/, briefs/ | ✅ DONE — commit `586edf2b` |
| Sprint 1A | Filter degenerate DPO tuples (scripts) | export-dpo.sh, corpus-threshold.py | ✅ DONE — commit `2f85cb48` |
| Sprint 1B | Improved extraction system prompt | http.rs, jennifer-datagraph-rebuild.sh | ✅ DONE — commit `30be4a1f` |
| Sprint 2A | entity_count in /healthz | graph.rs, http.rs (service-content) | ✅ DONE — commit `30be4a1f`; live: 7,201 entities |
| Sprint 2B | readyz exposes circuit state | circuit_breaker.rs, router.rs, http.rs (doorman) | ✅ DONE — commit `30be4a1f`; `tier_b` field live |
| Sprint 2C | Degenerate tuple guard at write time | apprenticeship.rs | ✅ DONE — commit `5493a8f4` |
| Sprint 3A | SLM_TIER_A_FIRST flag | router.rs, apprenticeship.rs, main.rs (doorman) | ✅ DONE — commit `5493a8f4`; `SLM_TIER_A_FIRST=true` deployed |
| Sprint 3B | WATCHER Tier A fallback (rate-limited) | main.rs (service-content) | ✅ DONE — commit `5493a8f4`; enabled, 300s interval |
| Sprint 3C | Drain worker pause when Tier B long-OPEN | main.rs (doorman) | ✅ DONE — commit `5493a8f4`; 1h threshold |

**Not changing:** OLMo-only policy; `/v1/extract` Tier B-only boundary (ADR-07); three-tier architecture; zone fallback; GCS training upload.

---

## §5 — Immediate open items

- [x] **Tier A re-enable (Command Session)** ✓ DONE 2026-05-29
  `SLM_FORCE_BROKER_MODE=false` confirmed; `local-slm.service` active; Goose verified.

- [x] **Tool_use shim (Sprint 1 → Jennifer)** ✓ DONE 2026-05-29T04:00Z
  Commit `1b47d3eb` — 51/51 http_test pass; 102/102 slm-doorman tests pass.
  `/v1/messages/count_tokens`, `/v1/models`, tool_use SSE blocks, thinking suppression all shipped.

- [x] **Training pipeline wiring (Sprint 2 → Peter)** ✓ DONE 2026-05-29
  Commit `1d819d7c` — `git-post-commit-hook.sh` + `claude-session-bridge.py`.
  Hook install per archive: `cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x`
  Command Session action needed: install hook in active archives.

- [x] **Goose round-trip verification** ✓ DONE 2026-05-29T04:10Z
  Goose v1.36.0 replied "Hello! The result of 2+2 is 4." Doorman log: `dispatching ... tier="local"`.

- [x] **Circuit resilience Sprint 1A** ✓ — filter degenerate DPO tuples (scripts only) — 2026-05-29
- [x] **Circuit resilience Sprint 1B** ✓ — structured extraction system prompt (5 categories with examples) — 2026-05-29
- [x] **Circuit resilience Sprint 2A** ✓ — entity_count in /healthz; live: 7,201 entities — 2026-05-29
- [x] **Circuit resilience Sprint 2B** ✓ — tier_b circuit state + opened_for_secs in readyz — 2026-05-29
- [x] **Circuit resilience Sprint 2C** ✓ — degenerate tuple guard in write_shadow_tuple — 2026-05-29
- [x] **Circuit resilience Sprint 3A** ✓ — SLM_TIER_A_FIRST=true deployed; shadow briefs route Tier A — 2026-05-29
- [x] **Circuit resilience Sprint 3B** ✓ — WATCHER Tier A fallback enabled (300s rate limit) — 2026-05-29
- [x] **Circuit resilience Sprint 3C** ✓ — drain worker holds queue when Tier B open >1h — 2026-05-29
- [x] **service-content rebuilt + deployed** ✓ sha256=`2362ea5c580a9869c5e307b645d60219cb9535dbf4218bd8762da870a4c62f7b` — 2026-05-29
- [x] **slm-doorman-server rebuilt + deployed** ✓ sha256=`81b8629cf474104fe33274244c6db832a1f2f5dca898c80a98cd524bf3269e2f` — 2026-05-29
- [x] **Binary ledger updated** ✓ both entries appended to `data/binary-ledger/*.jsonl` — 2026-05-29

- [ ] **Poison queue root cause (session 13)** — 78 entries (up from 0 at session 11).
  Newest entries (May 31 04:47–04:58 UTC, post-Fix-B) have `actual_diff: ""` + no `response_raw`.
  Two hypotheses: H1 = pre-Fix-A carry-forward briefs; H2 = hook still broken for some commits.
  Verify via `brief.created` timestamps vs Fix-A deploy (00:41 UTC May 31).
  Action: quarantine if H1; investigate hook if H2. See BRIEF-project-intelligence-active-work.md §1.

- [ ] **P0 (Gemini audit) — Doorman audit ledger sha256**
  File: `service-slm/crates/slm-doorman-server/src/ledger.rs`
  Add `sha256: String` field to `LedgerEntry`; compute `blake3::hash(serialized_entry)` on every write.
  Brings Doorman's own inference log to WORM baseline. Does NOT depend on system-ledger.

- [ ] **P1 (Gemini audit) — /readyz structured circuit_breaker_state**
  File: `service-slm/crates/slm-doorman-server/src/http.rs`
  Augment Tier B circuit JSON with `reason: Option<String>` (stockout | timeout | error) and
  `zone: Option<String>`. ~30 LOC. Benefit: app-console-slm status command can display reason.

- [ ] **P2 (Gemini audit) — service-content path decoupling**
  File: `service-content/src/main.rs`
  Replace hardcoded `/srv/foundry/...` paths with env vars:
  `INFRASTRUCTURE_ROOT` (default `/srv/foundry`) and `CORPUS_ROOT` (default
  `/srv/foundry/data/apprenticeship`). Defaults preserve current behaviour. ~20 LOC.

- [ ] **P3 (Gemini audit) — orchestration-slm persistence**
  File: `app-orchestration-slm/` (metering/registry module)
  Replace ephemeral `HashMap` metering with Redb or SQLite persistent store.
  Required for production-grade audit trail. Estimate: 1 session.

- [ ] **Verify CORPUS extraction via Tier A fallback** — drop a CORPUS file and confirm `[WATCHER-TIER-A]` log entry within 300s

- [x] **Goose §7.2 verified** ✓ 2026-05-29T04:10Z (Doorman log confirms tier="local")

---

## §6 — Pending work (ordered by priority)

### Command Session scope

- [ ] **Stage 6 promote** — 16+ commits ahead of `origin/main` (2 more added this session: `446df43f` Tier 2, `e263d6f0` Tier 3)
  Prerequisite: rebase per inbox `command-20260520-stage6-rebase-required`
  ```bash
  # From project-intelligence archive:
  git rebase origin/main   # or merge
  bin/promote.sh
  bin/sync-local.sh --all
  ```

- [ ] **Infrastructure tracking** — two drop-in files not tracked in `~/Foundry/infrastructure/`:
  - `/etc/systemd/system/local-content.service.d/memory.conf` (MemoryMax=4G, MemoryHigh=3800M)
  - `/etc/systemd/system/local-content.service.d/crash-loop-guard.conf`
  Copy to `~/Foundry/infrastructure/local-content/local-content.service.d/` and commit.

- [ ] **Binary ledger update** — after today's deploys, verify fresh sha256 entries:
  - `data/binary-ledger/service-content.jsonl`
  - `data/binary-ledger/slm-doorman-server.jsonl`

- [ ] **Packer image rebuild** (low priority, deferred) — rebuild `slm-yoyo` image for
  G3 dead-man's-switch + G17 sticky stops (Phase 0 hardening). The VM currently runs
  the 2026-05-13 image which predates G3/G17. Command Session: `cd service-slm/compute/packer && packer build yoyo-image.pkr.hcl`

### Totebox next coding session

- [ ] **Sprint 3 — PUSH inversion** (~150 LOC)
  Delete PULL path from service-content; queue graph mutations in Doorman in-memory queue
  (`slm-doorman/src/graph_queue.rs`). service-content becomes write-only via Doorman proxy.
  Files: `service-content/src/main.rs`, `service-content/src/http.rs`,
  `service-slm/crates/slm-doorman/src/` (new queue module)

- [ ] **Sprint 4 — /v1/draft/generate migration** (~80 LOC)
  Move endpoint from `service-content/src/http.rs` to `slm-doorman-server/src/http.rs`.
  Currently returns 503 ("Doorman unconfigured for Tier C auth"); migration unblocks Tier C routing.

- [ ] **service-slm audit fix** (~10 LOC)
  Add `"graph-query"` and `"graph-mutation"` to `AUDIT_CAPTURE_VALID_EVENT_TYPES` in
  `service-slm/crates/slm-doorman-server/src/http.rs` — graph proxy handlers currently
  bypass audit validation.

- [x] **Yo-Yo env IP update + Doorman restart** — `update_doorman_env()` now restarts
  local-doorman.service after writing new IP; commit `446df43f` ✓
  (Remaining: non-writable path still prints instructions only — low priority)

- [ ] **is_already_processed integration test (LbugGraphStore)**
  Sprint 5 test only covers `SqliteGraphStore`. A live `LbugGraphStore` integration test
  requires a real lbug DB file and is deferred until CI has a lbug-capable runner.

### Deferred / operator decision required

- [ ] **LoRA training pipeline activation** (`scripts/lora-update.sh` HARD DISABLED)
  Requires explicit operator approval + `SLM_LORA_AUTO_ENABLE=true` in env.
  Full pipeline documented in `service-slm/docs/yoyo-training-substrate-and-service-content-integration.md`.

- [ ] **Tier C activation** (Anthropic external API)
  Add `ANTHROPIC_API_KEY` to `/etc/local-doorman/local-doorman.env` and restart Doorman.
  Tier C becomes the fallback when Tier B is unavailable. Cost: per-token billing.
  Operator decision: accept external API dependency for resilience.

- [ ] **G-items from BRIEF-flow-restructure.md** (G5/G6/G9/G11-G16/G18)
  Brief no longer exists; G-items unrecoverable. Phase 6 AUTO-TODO treated as superseded
  by Session 6 (LatencyClass/BackendLifecycle/GF-1/GF-2). Command to confirm disposition.

---

## §7 — Definition of done (next gate)

The substrate is in full operational state when ALL of the following pass:

1. `start-yoyo.sh` exits 0 (europe-west4-a L4 available)
2. `curl /readyz` → `tier_b_circuit_state: "closed"`, `ai_available: true` within 3 min of start
3. One round-trip inference: `POST /v1/messages` → Yo-Yo → valid response with `X-Foundry-Tier-Used: yoyo`
4. `curl http://127.0.0.1:9081/healthz` → `entity_count` rising above 1,529 over ~10 min
5. `service-content` logs show `entities extracted` (not just `extraction deferred`)
6. Stage 6 promoted; `origin/main` up to date

Items 1–5 are Totebox scope. Item 6 is Command Session scope.

---

## §8 — Reference documents (do not duplicate, read instead)

| Document | What it covers |
|---|---|
| `service-slm/ARCHITECTURE.md` | Three-ring memory model, Doorman protocol, tier routing, audit ledger |
| `service-slm/docs/deploy/deploy-yoyo-tier-b.md` | Full first-time Yo-Yo setup runbook (Packer → OpenTofu → weights → wire) |
| `service-slm/docs/yoyo-training-substrate-and-service-content-integration.md` | LoRA training pipeline + corpus ingestion integration |
| `service-slm/docs/audit-endpoints-contract.md` | Audit ledger schema + endpoint contracts |
| `service-slm/scripts/start-yoyo.sh` | Start script — exit codes, modes, cost guardrails (G1/G3/G8) |
| `service-slm/compute/packer/yoyo-image.pkr.hcl` | Packer template — vLLM + CUDA + Nginx + LoRA systemd units |
| `service-slm/compute/opentofu/main.tf` | OpenTofu — VM definition, persistent disk, firewall, IAM, Instance Schedule |
| `service-content/CLAUDE.md` | service-content project card — feature table, env vars, build commands |
| `service-content/src/graph.rs` | GraphStore trait + both backends (LbugGraphStore, SqliteGraphStore) |

---

## §9 — Audit findings (2026-05-25)

Pre-Stage-6 promote audit by 3 parallel Explore agents. Findings are evidence-based
(exact file:line). All BLOCKER items were fixed in the same session.

### service-slm

| # | File:Line | Severity | Finding | Status |
|---|---|---|---|---|
| SLM-1 | `crates/slm-doorman/src/error.rs:245/249` | BLOCKER | User-facing error message + doc said "90 s outer deadline"; OUTER_DEADLINE is now 180 s | **FIXED** `error.rs` updated this session |
| SLM-2 | `crates/slm-doorman/src/router.rs:750/779` | BLOCKER | `try_auto_start_yoyo()` doc comment + `Duration::from_secs(90)` stale after OUTER_DEADLINE 90→180 raise | **FIXED** `router.rs` updated this session |
| SLM-3 | `crates/slm-doorman/src/tier/circuit_breaker.rs` | — | Circuit breaker state machine correct (all 4 transitions + failure counter reset verified) | CORRECT |
| SLM-4 | `infrastructure/` | LOW | systemd unit files for `local-doorman.service` / `local-content.service` not in this repo; `PartOf=`/`Requires=` cause unknown | NOT-IN-REPO — infra gap; root cause deferred |
| SLM-5 | `scripts/start-yoyo.sh:398–450` | — | `update_doorman_env()` correctly updates all 4 env vars (ZONE + ENDPOINT + HEALTH + WEIGHTS); default `--wait-ready` is 5400 s | CORRECT |
| SLM-6 | `crates/slm-doorman-server/src/http.rs:1382–1389` | — | `graph-query` and `graph-mutation` present in `AUDIT_CAPTURE_VALID_EVENT_TYPES` (6 types total) | CORRECT |
| SLM-7 | `scripts/test-yoyo-flows.sh` | LOW | 11 tests (not 10); test 9 (SSH kill/recovery) is a SKIP placeholder | DEFERRED — Sprint 9 |

### service-content

| # | File:Line | Severity | Finding | Status |
|---|---|---|---|---|
| SC-1 | `service-content/src/graph.rs:628–639 / 384–401` | — | `is_already_processed()` logic correct in both SQLite and Lbug backends; correctly distinguishes Source-only (false) from extracted (true) | CORRECT |
| SC-2 | `service-content/src/main.rs:418–425` | MEDIUM | Code acknowledges `defer_reason` but does not differentiate `yoyo-transient` vs `yoyo-circuit-open` at the drain level — operational visibility gap, not a correctness bug | DEFERRED — Sprint 2 follow-up |
| SC-3 | `service-content/src/main.rs:35–36` | MEDIUM | No startup retry/wait loop; Doorman unavailable at boot causes first drain cycle to defer all files silently | DEFERRED — Sprint 3 |
| SC-4 | `service-content/src/graph.rs:88` | — | WAL checkpoint 4 MB hardcoded (correct); checkpoint triggered automatically by lbug + manually after drain | CORRECT |
| SC-5 | `service-content/src/main.rs:320–327` | MEDIUM | File-read + JSON-parse failures silently skip CORPUS files with no error log; malformed files are silently marked done | DEFERRED — add `error!()` + return Err before Sprint 3 |
| SC-6 | Sprint 3 map | — | Drain loop at `main.rs:185`; Doorman call `main.rs:403`; entity write `main.rs:524`; new PUSH endpoint would be POST `/v1/extraction/push` in `http.rs` | REFERENCE |
| SC-7 | LbugGraphStore vs SqliteGraphStore | — | All three trait methods (`write_entity`, `write_related_to`, `is_already_processed`) behave identically across both backends | CORRECT |

### Brief cleanup

17 contaminated project-gis briefs (9 active + 8 archive) introduced by Stage-6 rebase
2026-05-22 removed this session via `git rm`. Archive items verified as GIS cluster content
before deletion. 3 Wikipedia Parity archive briefs confirmed legitimate and retained.

---

## §10 — Tier B: Cloud Run Migration Plan (2026-06-02)

> **Why:** `yoyo-tier-b-1` is TERMINATED; europe-west4-a L4 stockout means VM restarts
> fail unpredictably. Cloud Run eliminates the zone lottery — Google manages the GPU pool,
> no quota request or zone-capacity fight required. Replaces the GCE Spot VM pattern.

### What changes

| Resource | Before | After |
|---|---|---|
| Compute | GCE g2-standard-4 Spot VM (`yoyo-tier-b-1`) | Cloud Run service (`yoyo-tier-b`) |
| Model storage | 256 GB `pd-balanced` disk, zone-locked | GCS object, zone-free |
| Auth | Nginx TLS + static bearer token | `--allow-unauthenticated` (URL unguessable); follow-up: IAM |
| Endpoint | `https://34.6.204.25:9443` (static IP) | `https://yoyo-tier-b-<hash>-ew.a.run.app` |
| Cost at 2 hrs/day | ~$165/month (often unavailable) | ~$70/month (always available) |
| Docker required | No (Packer image) | No (pre-built image from llama.cpp project) |

**No Rust code changes. No new binaries. Tier A untouched.**
Only `/etc/local-doorman/local-doorman.env` changes (3 env vars).

### Critical llama-server flags (verified correct per §2.8)

| Flag | Value | Note |
|---|---|---|
| `-ngl` | `99` | All layers on GPU — omitting falls back to CPU |
| `-np` | `1` | NOT `-np 4`; with `-c 4096`, -np 4 truncates to 1024 tokens/slot |
| `-fa` | `on` | NOT bare `-fa`; bare flag consumes next arg as its value → crash |
| `--host` | `0.0.0.0` | Changed from `127.0.0.1`; Cloud Run routes externally |
| `--port` | `8080` | Cloud Run default |
| `-c` | `4096` | Context size |
| `-a` | `Olmo-3-1125-32B-Think` | Three aliases: also `Olmo-3-32B-Think`, `olmo-3-32b` |
| `--reasoning-format` | `deepseek` | Required for OLMo-3 Think |
| `--reasoning-budget` | `1024` | |

### Migration checklist

**Pre-flight**
- [ ] Confirm Cloud Run GPU quota in europe-west4:
  `gcloud run regions describe europe-west4 --project woodfine-node-gcp-free`
- [ ] Confirm Compute Engine SA has `roles/storage.objectViewer` on the weights bucket

**Step 1 — Get model into GCS as a plain object**
```bash
gsutil ls gs://woodfine-node-gcp-free-foundry-substrate/weights/
# If not present: restore disk snapshot yoyo-tier-b-1-weights-20260513-1923 to a temp disk,
# mount it, gsutil cp /data/weights/olmo-3-32b-think-q3.gguf gs://...weights/, delete temp disk
```

**Step 2 — Deploy Cloud Run (pre-built image, no Docker)**
```bash
gcloud run deploy yoyo-tier-b \
  --image ghcr.io/ggerganov/llama.cpp:server-cuda \
  --region europe-west4 --project woodfine-node-gcp-free \
  --gpu 1 --gpu-type nvidia-l4 \
  --cpu 4 --memory 16Gi \
  --min-instances 0 --max-instances 1 \
  --concurrency 1 --timeout 1800 \
  --allow-unauthenticated \
  --add-volume name=weights,type=cloud-storage,bucket=woodfine-node-gcp-free-foundry-substrate \
  --add-volume-mount volume=weights,mount-path=/models \
  --args="--model,/models/weights/olmo-3-32b-think-q3.gguf,-ngl,99,--host,0.0.0.0,--port,8080,-c,4096,-a,Olmo-3-1125-32B-Think,-a,Olmo-3-32B-Think,-a,olmo-3-32b,-np,1,-fa,on,--reasoning-format,deepseek,--reasoning-budget,1024,--metrics" \
  --port 8080
```

**Step 3 — Test endpoint directly before touching Doorman**
```bash
CLOUD_RUN_URL=https://yoyo-tier-b-<hash>-ew.a.run.app
curl $CLOUD_RUN_URL/health                         # expect {"status":"ok"}
curl -s $CLOUD_RUN_URL/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"Olmo-3-1125-32B-Think","messages":[{"role":"user","content":"ping"}],"max_tokens":10}'
```
**Do not proceed to Step 4 until both pass.**

**Step 4 — Update `/etc/local-doorman/local-doorman.env`**
```
SLM_YOYO_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
SLM_YOYO_TRAINER_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
SLM_YOYO_GRAPH_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
```
`SLM_YOYO_BEARER` stays — Doorman sends it, Cloud Run ignores it.
Then: `sudo systemctl restart local-doorman`

**Step 5 — Verify end-to-end through Doorman**
```bash
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"Olmo-3-1125-32B-Think","messages":[{"role":"user","content":"test"}],"max_tokens":20}'
journalctl -u local-doorman -f   # confirm Tier B hit
```

### Rollback
Revert the three `SLM_YOYO_*` vars to `https://34.6.204.25:9443`, restart Doorman.
Tier A continues serving throughout — unaffected by any step above.

### Cold start behaviour
First request after idle: ~60–90s (GCS FUSE reads 20 GB GGUF into GPU memory).
If unacceptable: set `--min-instances 1` (~$0.67/hr continuous, ~$490/month GPU alone).

### Follow-up items (post-migration)
- [ ] Add Cloud Run IAM auth (replace `--allow-unauthenticated`; update Doorman to fetch GCP
      identity token from metadata service instead of static bearer token)
- [ ] Update `service-slm/docs/deploy/deploy-yoyo-tier-b.md` with Cloud Run procedure
- [ ] Release static IP `34.6.204.25` if old VM is permanently retired
- [ ] Update §7 definition-of-done item 1 (replace `start-yoyo.sh` gate with Cloud Run health check)
