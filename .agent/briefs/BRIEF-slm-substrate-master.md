---
artifact: brief
status: active
title: SLM Substrate Master — Yo-Yo + DataGraph + Learning Loop
created: 2026-05-24
updated: 2026-05-29 (session 8 — circuit resilience plan; Tier A confirmed primary; five-defect inventory)
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

## §1 — Current live state (as of 2026-05-29T~18:00Z)

| Component | Version | Status | Notes |
|---|---|---|---|
| `slm-doorman-server` | rebuilt 2026-05-29T04:05Z | **active** | sha256=`d3c2d37db77c46e480da04006f2b1ad76e57ad493bf0a1fd2e36ee2a147827dc`; Think-strip + 180s/300s timeouts; drain-backoff; SLM_FORCE_BROKER_MODE=**false** (Tier A enabled); tool_use shim (Sprint 1) |
| `service-content` | rebuilt 2026-05-25 | **active** | sha256=`00b075d0a114659aec84012b84dafda007b488855833681bb6d75c57d98ba1d5`; **STALE** — missing SC-2/3/5 fixes; rebuild required after Sprint 2A (entity_count in healthz) |
| `yoyo-tier-b-1` | 2026-05-13 Packer image | **TERMINATED** | europe-west4-a L4 stockout; restart with `start-yoyo.sh --runtime=2h` when capacity returns |
| `local-slm.service` | OLMo 2 1124 7B Instruct Q4_K_M (4.16 GiB) | **active** | Tier A is the always-on primary; Goose verified round-tripping 2026-05-29 |
| `local-doorman.env` | — | current | `SLM_YOYO_GCP_ZONE=europe-west4-a`; `SLM_YOYO_IDLE_MINUTES=120`; `SLM_APPRENTICESHIP_ENABLED=true`; `SLM_FORCE_BROKER_MODE=false` |

**Tier routing (current):**
- Tier A: **ENABLED** — OLMo 7B handles all chat/shadow; `has_local: true` in readyz
- Tier B: **circuit OPEN** — Yo-Yo TERMINATED; 1,460+ consecutive failures; `opened_at` recorded
- Tier C: not configured — ToS hard constraint; never enable for training loop
- Result: `ai_available: true` (Tier A); entity extraction deferred (Tier B circuit open)

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

**Shadow capture state (2026-05-29):**
- queue/: 0 pending briefs
- queue-done/: 550 briefs (dispatched)
- queue-poison/: 590 files (accumulated during Tier B outage; mv to quarantine/ before next Yo-Yo start)
- queue-paused/: 11 files
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
| Sprint 0 | Documentation + artifact delivery | drafts-outbound/, briefs/ | **IN PROGRESS** |
| Sprint 1 | Script fixes (no build) | export-dpo.sh, corpus-threshold.py | pending |
| Sprint 2 | Honest observability (Rust, two binaries) | graph.rs, http.rs (both services), circuit_breaker.rs, router.rs | pending |
| Sprint 3 | Tier A as confident primary (Rust, larger) | router.rs, apprenticeship.rs, main.rs (both services) | pending |

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

- [ ] **Circuit resilience Sprint 1** — filter degenerate DPO tuples (scripts only, no build)
  1A: `export-dpo.sh` — add `select(.attempt.diff != null and .attempt.diff != "")` before chosen/rejected mapping
  1A: `corpus-threshold.py` — skip files with empty attempt.diff in count
  (Sprint 1B, improved extraction system prompt, batches with Sprint 2 build)

- [ ] **Circuit resilience Sprint 2** — honest observability (two Rust binaries)
  2A: `count_all()` to GraphStore trait + entity_count in /healthz (service-content rebuild)
  2B: `state_label()` + `opened_for_secs()` on CircuitBreaker; `tier_b_status()` on Doorman; readyz update (Doorman rebuild)
  2C: guard in `write_shadow_tuple` — skip when `attempt.escalate && attempt.diff.is_empty()`

- [ ] **Circuit resilience Sprint 3** — Tier A as confident primary (larger Rust changes)
  3A: `SLM_TIER_A_FIRST` env var; change select_tier + pick_tier_for_brief; startup guard
  3B: WATCHER Tier A fallback (rate-limited) — `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED`
  3C: drain worker pause when all Tier B circuits open for > `SLM_HOLD_THRESHOLD_SECS`

- [ ] **Quarantine 590 poison briefs** (Command Session, before next Yo-Yo start)
  ```bash
  mv /srv/foundry/data/apprenticeship/queue-poison/* /srv/foundry/data/apprenticeship/quarantine/
  ```

- [ ] **service-content rebuild** (after Sprint 2A)
  ```bash
  cargo test -p service-content && cargo build --release -p service-content
  sudo cp target/release/service-content /usr/local/bin/service-content
  sudo systemctl restart local-content.service
  ```

- [ ] **Binary ledger update** — slm-doorman-server sha256=`d3c2d37db77c46e480da04006f2b1ad76e57ad493bf0a1fd2e36ee2a147827dc` (built 2026-05-29); update `data/binary-ledger/slm-doorman-server.jsonl`

- [ ] **Verify CORPUS extraction** after next Yo-Yo start
  With Sprint 3B in place (WATCHER fallback), extraction will proceed via Tier A at rate limit.
  Until then, deferred until Tier B returns.

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
