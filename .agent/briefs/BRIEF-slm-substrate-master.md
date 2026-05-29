---
artifact: brief
status: active
title: SLM Substrate Master — Yo-Yo + DataGraph + Learning Loop
created: 2026-05-24
updated: 2026-05-29
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

## §1 — Current live state (as of 2026-05-28T~18:00Z)

| Component | Version | Status | Notes |
|---|---|---|---|
| `slm-doorman-server` | rebuilt 2026-05-28 (prev session) | **active** | Think-strip + 180s/300s timeouts; drain-backoff; `SLM_FORCE_BROKER_MODE=true`; idle 120min; **NEEDS REBUILD** for reasoning_content + reqwest-reclassify fixes (commit `446df43f`) |
| `service-content` | rebuilt 2026-05-24 | **active** | LadybugDB loaded; CORPUS drain deferred until Yo-Yo restarts; **NEEDS REBUILD** for SC-2/3/5 fixes (commit `e263d6f0`) |
| `yoyo-tier-b-1` | 2026-05-13 Packer image | **TERMINATED** | Restart with `start-yoyo.sh --runtime=2h`; next rebuild adds -fa/deepseek/budget flags |
| `local-slm.service` | OLMo 2 1124 7B Instruct Q4_K_M (4.16 GiB) | active | Tier A disabled by FORCE_BROKER_MODE — **re-enable pending Sprint 0** |
| `local-doorman.env` | — | current | `SLM_YOYO_GCP_ZONE=europe-west4-a`; `SLM_YOYO_IDLE_MINUTES=120`; `SLM_APPRENTICESHIP_ENABLED=true`; `SLM_BRIEF_TIER_B_THRESHOLD_CHARS=0` |

**Tier routing (current):**
- Tier A: disabled (`SLM_FORCE_BROKER_MODE=true`) — **Sprint 0: set false to re-enable OLMo 7B as always-on primary**
- Tier B: **circuit OPEN** (Yo-Yo TERMINATED) — nightly 1-hour cron (`0 2 * * *`) pending setup; extend to 4h after first verified run
- Tier C: not configured — ToS hard constraint; never enable for training loop
- Result: `ai_available: false` until Tier A is re-enabled (Sprint 0) or Yo-Yo restarts

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

**Shadow capture state:**
- Queue: `8GKR3472S2X79VC10Q4ECZHNE1` (retrying — will succeed on next Yo-Yo start)
- queue-done: 539 briefs
- queue-poison: `FECH83K3N665A8H8AZ3MTVNCKZ` (accumulated pre-backoff-fix retries)
- Training corpus: 1,900+ tuples

**Stage 6 state:** project-intelligence archive is 16+ commits ahead of `origin/main`.
Rebase required per inbox `command-20260520-stage6-rebase-required` before promote.

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

**Tier A re-enable decision (2026-05-29):**
`SLM_FORCE_BROKER_MODE=true` was a development workaround, not a permanent design decision.
OLMo 2 1124 7B Instruct Q4_K_M is deployed and running (`local-slm.service`, 4.9G/8G).
The 7B model is the always-on interactive tier for the sovereign coding agent architecture.
Yo-Yo (Tier B) is the nightly bonus tier; it does not need to be the only inference path.
**Sprint 0 (Command Session): set `SLM_FORCE_BROKER_MODE=false` in `/etc/local-doorman/local-doorman.env`
and `sudo systemctl restart local-doorman.service`.**

---

## §5 — Immediate open items (no prerequisites)

- [ ] **Sprint 0 (Command Session) — Re-enable Tier A + nightly Yo-Yo cron**
  1. `sudo sed -i 's/SLM_FORCE_BROKER_MODE=true/SLM_FORCE_BROKER_MODE=false/' /etc/local-doorman/local-doorman.env`
  2. `sudo systemctl restart local-doorman.service`
  3. Verify: `curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool` → `has_local: true`
  4. Add crontab: `0 2 * * * /srv/foundry/clones/project-intelligence/service-slm/scripts/start-yoyo.sh --runtime=1h`
  5. Binary rebuild: `cargo build --release -p slm-doorman-server -p service-content`
  6. `sudo systemctl restart local-doorman.service local-content.service`
  7. Drain/purge 491 poison apprenticeship briefs from `data/apprenticeship/queue/`

- [ ] **Sprint 1 (Jennifer, ~200 LOC) — tool_use shim for Goose**
  File: `service-slm/crates/slm-doorman-server/src/http.rs`
  See plan file `/home/mathew/.claude/plans/fancy-riding-turtle.md` §Sprint 1 for full spec.
  7 changes: `tools`/`tool_choice` fields, thinking suppression on tool turns (llama.cpp #20345),
  tool_use SSE blocks, `stop_reason:"tool_use"`, `count_tokens` endpoint, unknown-field passthrough, `/v1/models`.

- [ ] **Sprint 2 (Peter) — training pipeline wiring**
  2a. `service-slm/scripts/git-post-commit-hook.sh` → diff capture to `/v1/shadow`
  2b. `service-slm/scripts/claude-session-bridge.py` → Claude Code CORPUS bridge (no ToS conflict — OLMo extracts entities from Claude text)

- [x] **Start Yo-Yo when europe-west4-a L4 capacity is available** ✓ DONE 2026-05-28 04:50 UTC
  VM restarted Mode 1; llama-server loaded OLMo-3 32B Think in ~2 min; circuit closed.

- [x] **End-to-end flow test** ✓ DONE 2026-05-28 04:58 UTC
  `POST /v1/messages` with `model: claude-sonnet-4-6` → routes to Tier B trainer → OLMo-3 32B
  Think replied with `<think>` tokens. Shadow brief dispatch also confirmed OK (JARG5G8T45W0QMD3DTWKH29GTC).

  **Routing note:** Direct `/v1/messages` requests require `model: claude-sonnet-*` (or `claude-opus-*`)
  to route to Tier B. Model `olmo` maps to Complexity::Medium → Local (fails with FORCE_BROKER_MODE).
  Shadow briefs bypass this (ApprenticeshipDispatcher forces tier_hint=Yoyo).

- [x] **Verify service-content drains deferred CORPUS files** ✓ PARTIAL 2026-05-28
  service-content loads in ~3.5 min (not 16), restarted 05:06 UTC, HTTP up at 05:09.
  CORPUS drain is still deferring with "yoyo-transient" — root cause found and fixed:
  OLMo-3 32B Think emits `<think>` blocks before JSON; old 60s timeout fires before `</think>`.
  **Fix deployed:** `strip_think_blocks()` + SOCKET_TIMEOUT 180s + OUTER_DEADLINE 300s.
  CORPUS drain will succeed on next Yo-Yo start.

- [x] **Drain worker backoff — new briefs should dispatch cleanly** ✓ DONE 2026-05-28
  Observed: drain backoff is working. `J7BFN7NTRZ1SCTV131GDCPMBFF` succeeded at 05:13:16
  after retry (first two attempts timed at 60s; third try succeeded in 58s = marginal win).
  New 180s timeout means shadow briefs no longer race against the socket limit.

- [ ] **Rebuild + deploy slm-doorman-server and service-content** to pick up this session's fixes
  ```bash
  cargo build --release -p slm-doorman-server
  cargo build --release -p service-content
  sudo systemctl restart local-doorman.service local-content.service
  ```

- [ ] **Verify CORPUS extraction succeeds** after next Yo-Yo start + binary rebuild
  service-content CORPUS drain should complete with "entities extracted: N" messages.
  With SC-2/3/5 fixes + deepseek format (after Packer rebuild), extraction should succeed cleanly.
  ```bash
  sudo journalctl -u local-content -f | grep -E 'entities extracted|WATCHER|deferred|RETRY'
  ```

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
