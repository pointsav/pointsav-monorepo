---
artifact: brief
status: active
artifact: brief
status: active
title: project-intelligence — Totebox fleet architecture (rebuilt on the $7-node doctrine)
created: 2026-05-21
rebuilt: 2026-05-22
author: task@project-intelligence (claude-code, Opus — 13-agent investigation, 5 rounds)
supersedes: the original "flow-restructure" framing of this file (GPU-flow premise — retired)
grounds_in:
  - DOCTRINE.md claim #49 (Tier 0 Customer-Side Sovereign Specialist — the $7/mo e2-micro neutral state)
  - DOCTRINE.md claim #54 (Substrate-Without-Inference Base Case — AI is value-add, not load-bearing)
  - conventions/four-tier-slm-substrate.md (the Tier 0–3 ladder)
  - conventions/tier-zero-customer-side-sovereign-specialist.md (Tier A = OLMo 1B narrow specialist)
  - conventions/substrate-without-inference-base-case.md
scope: project-intelligence archive · service-slm · service-content
purpose: the plan of record + per-repo to-do list to align Totebox for its real deployment — a $7/mo cloud node
---

# BRIEF — project-intelligence: Totebox Fleet Architecture

> **This BRIEF was rebuilt 2026-05-22.** Its earlier "flow-restructure" version
> assumed the wrong deployment target (a GPU box / the workspace VM) and
> concluded interactive AI must route to a GPU. A 13-agent Opus investigation
> traced that to a drift from ratified Doctrine: the Totebox fleet runs on a
> **$7/month GCP e2-micro**, named verbatim in DOCTRINE.md claims #49 and #54.
> The filename is historical; the content below is the corrected plan.

---

## 0. What changed and why

The original investigation measured a 7B model on the 8-vCPU workspace VM, found
~1.95 tok/s, and concluded "CPU can't do flow — route interactive AI to a GPU."
Five rounds of review corrected three compounding errors:

1. **Wrong model** — service-slm ships OLMo 2 **1B**, not 7B; the 1.95 figure
   benched a pending upgrade.
2. **Wrong hardware** — the fleet does not run on the workspace VM.
3. **Wrong deployment premise** — the fleet runs on a **$7/mo e2-micro**, which
   is *ratified doctrine* (#49/#54), not a choice this BRIEF gets to make.

The corrected picture is simpler and is already constitutional: the Totebox is a
**deterministic substrate** that runs on a tiny cheap node; **AI is value-add,
not load-bearing** (claim #54); on-node AI is a property of *bigger* node
classes, not the fleet default.

---

## Status — resume point (2026-05-22, session 2)

**lbug decision — LOCKED as Option 1** (2026-05-22). Agent-confirmed: current
binary is already shared-linked at 4.2 MB + 27 MB liblbug.so; static build
would be ~17.7 MB total (~13.5 MB delta). The ~2 GB RAM problem is LadybugDB
mmap — `SqliteGraphStore` solves that independently. Disk bloat accepted; one
binary serves all tiers. Do NOT revisit.

**Phase 0 — DONE** (Yo-Yo cost hardening): `35e2dea7`, `ed63476c`, `a10539c6`.

**Phase 0 inbox tasks — DONE** (2026-05-22 session 2):
- `b2a09597` — `.agent/binary-targets.yaml` declared (slm-doorman-server)
- `9fbff79d` — briefs migration: all `.agent/plans/` → `.agent/briefs/BRIEF-*`
- `335a8575` — frontmatter (`artifact: brief` / `status`) on all 18 briefs

**AUTO-TODO created:** `.agent/AUTO-TODO.md` — comprehensive phase-by-phase
plan (Phases 0–8) with gates and commit guidance. Start here tomorrow.

**Phase 1 — DONE** (2026-05-22 session 2): NEXT.md conflict note, outbox to Command, manifest contamination flagged, service-slm/CLAUDE.md updated, MEMORY.md pointer added.
**Phase 2 — DONE** (2026-05-22 session 2): `foundry-nodeclass` crate — 392 LOC, 11 unit tests + doctest, NodeClass::as_str() added session 3.
**Phase 3 — DONE** (2026-05-22 session 2): `SqliteGraphStore` + runtime backend selection + background CORPUS drain + hardcoded base_dir fixed. cargo check clean. lbug linker (cargo build) pre-existing blocker — Option 1 locked.
**Phase 4 — DONE** (2026-05-22 session 3): `build_doorman()` gates local on `supports_on_node_ai()` + `SLM_FORCE_BROKER_MODE`; `select_tier()` invariant test; `/readyz` reports node_class/tier_a/tier_a_reason/ai_available; `local-doorman.service` → soft `Wants=`; 241+ tests green.
**Phase 5 — DONE** (2026-05-22 session 4): `tests/micro_node.rs` (5 integration tests — readyz, 503, healthz, force-broker, hardware sanity); 8 `SqliteGraphStore` round-trip tests in `service-content/src/graph.rs`; `scripts/run-micro-sandbox.sh` cgroup sandbox (MemoryMax=1G, CPUQuota=25%); 260/260 tests pass. Commit: `32213020`.

**▶ RESUME HERE — Phase 6 (§8.F, deferred items):**
- `latency_class` field in `slm-core` (corrected W1 — see §8.F)
- Broker discipline — quarantine `idle_monitor.rs` behind `BackendLifecycle` trait
- Reconcile Tier A model drift (1B vs 7B-Think in env files)
- GF-1 async audit off hot path · GF-2 Tier A client timeouts

**Pending — Command Session (not Totebox scope):**
- Rebuild the `slm-yoyo` Packer image so Phase-0 G3/G17 take effect on the VM.
- Stage 6 promote — local `main` is now 7 commits ahead of origin/main; needs
  `git rebase origin/main` first (see inbox `command-20260520-stage6-rebase-required`).
- Outbox note re: original investigation drifting from ratified doctrine (§6) — queued in §8.A.

**BRIEF consolidation — DONE** (2026-05-23 session 5):
- BRIEF-service-slm-hardening → archived (pre-Phase-4 state, superseded)
- BRIEF-vm-hardening-and-consolidation → archived (§3A conflicts resolved; ops content → §12)
- BRIEF-tier-architecture §2 corrected (7B recommendation deleted; 1B doctrine applied)
- BRIEF-learning-loop-master-plan → Phase 7+ deferred (7B premise corrected)
- BRIEF-sovereign-routing-comprehensive → Sprint 0a marked done

**Also pending (inbox, not Phase 6 critical path):**
- `project-editorial-20260521-e4-triage-naming-blockers` — 2 of 3 drafts resolved:
  `topic-elastic-compute-lora-training-pipeline` (+ `.es`) and `guide-elastic-compute-nightly-pipeline`
  are renamed and in `drafts-outbound/`. Third draft (`topic-jennifer-datagraph-rebuild`)
  confirmed covered by `topic-service-slm-graph-store-migration.md` (already staged).
  Inbox message marked actioned; outbox notified project-editorial.

**In-flight / uncommitted:** none — BRIEF + all Phase 0 work committed. Untracked
and intentionally so: `compute/packer/scripts/yoyo-stability-gate.{service,sh}`
(W5/G4, spot-only — Phase 4) and `.claude/` + the archive `CLAUDE.md`
(harness/pre-existing, not this session's work).

---

## 1. The deployment model — ratified doctrine

`DOCTRINE.md` claim #49 and #54, verbatim: *"the neutral state ... is a
**$7/month GCP e2-micro (shared-core CPU, no GPU)** — the full deterministic
substrate runs at this size ... with zero inference dependency."*

So the 3+ billion Totebox instances are **$7/mo e2-micro nodes** (~1 GB RAM,
~0.25 vCPU burstable, no GPU). Hardware Toteboxes (NUC-class and up) and the
Yo-Yo GPU burst are *higher rungs of one ladder* — the revenue tiers — not the
fleet default.

---

## 2. The node-class matrix (the spine of this BRIEF)

| Node class | Who / scale | Runs | On-node AI? | AI source |
|---|---|---|---|---|
| **$7/mo e2-micro** | The fleet — ~3 billion instances; doctrine "neutral state" | Deterministic substrate (Rings 1+2) + **service-slm Doorman as pure broker** | **None** | External only — Yo-Yo / Tier C API, if the customer pays; else no AI (still a complete product, claim #54) |
| **NUC / mini-PC Totebox** | Customers who buy hardware (≥ ~8 GB RAM) | Substrate + **Tier A — the OLMo 1B narrow specialist**, on-device | **Yes — Tier A switches on here** | Local Tier A; Yo-Yo / C opt-in |
| **+ Yo-Yo GPU / appliance** | The paying tier | + interactive big-model AI | Yes | Tier B Yo-Yo (L4, on-demand), Tier C |

**The answer to "should there be a Tier A / any AI?"** — node-class-dependent.
On the $7 fleet node: **no on-node AI; AI is external.** Tier A (the 1B
specialist) exists only at the hardware-Totebox rung. This is exactly claim #54.

---

## 3. Architecture per node class

### 3.1 The $7 node (the fleet)
- Runs the **deterministic substrate**: `service-fs` + WORM ledger,
  `service-content` (knowledge graph), `service-input/extraction/egress`,
  `service-people/email/search`. Query, audit, graph, search, export,
  ownership-transfer — all deterministic, all on-node.
- Runs **`service-slm` as a pure broker** — route / sanitise (SYS-ADR-07) /
  audit / MCP. **Hard rule: the Doorman does not operate inference
  infrastructure and hosts no model.** On a 1 GB node this is non-negotiable.
- **No Tier A.** The Doorman's `/readyz` reports `tier_a: unavailable` by
  node-class detection — it must not attempt a model load and OOM-crash.
- AI, if the customer wants it, leaves the boundary sanitised → Tier B (Yo-Yo)
  or Tier C (API). If they want neither, the node is still a complete product.

### 3.2 The NUC / hardware Totebox
- Same substrate + **Tier A on-device**: the OLMo 1B narrow specialist
  (`conventions/tier-zero-customer-side-sovereign-specialist.md`) — short,
  narrow, sysadmin-class turns at ~5–15 tok/s. Not interactive 7B chat.
- The Doorman's node-class probe detects ≥8 GB + adequate CPU → `tier_a: ok`.

### 3.3 The Yo-Yo / GPU tier
- The paid interactive-AI accelerator. Phase 0 hardening (below) is done and
  correct. At fleet scale the *multi-tenant* Tier 2 Yo-Yo (one GPU amortised
  across many nodes via moduleId) is the economically coherent shared backend —
  the BRIEF's ~$120/mo single-tenant figure is dogfood cost, not per-customer.

---

## 4. The blocker — `service-content` does not fit a $7 node

`service-content` as built uses **LadybugDB**, measured at **~2 GB RSS** on
startup (`.agent/plans/lbug-build-blocker.md`) — it does not fit a 1 GB node, or
even a 2 GB node. The fix already exists in the design: `service-content/ARCHITECTURE.md`
§4 specifies a **SQLite-graph `GraphStore` backend** explicitly "to fit Tier 0
hardware" — but the deployed path is still LadybugDB.

**Swapping `service-content` to the SQLite-graph backend for Tier-0 / $7-node
deployments is THE prerequisite for the fleet to boot.** Everything else is
downstream of it. LadybugDB stays available as an opt-in for large nodes.

**§4.1 — Ring 2/3 coupling defect (approved addition 2026-05-23):** the current
`service-content` binary halts graph growth whenever the Doorman is unavailable
— Ring 2 should be fully functional without Ring 3 (DOCTRINE claim #54). Sprint 1
fix: write a deterministic `Source` node to the graph before calling the Doorman
(~30 LOC, `main.rs:198`). Full defect list and 5-sprint PUSH inversion plan:
`BRIEF-service-content-architecture.md`.

---

## 5. What stays — Phase 0 Yo-Yo hardening (DONE)

The Yo-Yo cost hardening shipped 2026-05-21 and is **unaffected** — the Yo-Yo is
the paid AI tier and the work is correct for it. Three signed commits on local
`main`: `35e2dea7` (G1 attempt cap, G3 dead-man's-switch, G8 $3/day cap),
`ed63476c` (G7 VM-hours ledger, G10 verified shutdown), `a10539c6` (G17 sticky
deliberate stops). Pending (Command Session): rebuild the `slm-yoyo` Packer
image so G3/G17 take effect; Stage 6 promote.

---

## 6. Doctrine reconciliation (surface to Command)

- **Conflict to log in NEXT.md:** claim #49 says "the full substrate runs at
  [e2-micro] size" but `conventions/tier-zero-customer-side-sovereign-specialist.md`
  §1 specs a "2–4 GB working set." Resolution: the $7 node runs the *claim-#54
  deterministic* substrate (no model); "full substrate incl. the 1B specialist"
  is the NUC rung.
- The original flow-restructure investigation **missed four ratified
  conventions/claims** (#49, #54, `four-tier-slm-substrate.md`,
  `tier-zero-customer-side-sovereign-specialist.md`). Command should be told —
  an outbox note is queued in §8.

---

## 7. Tier routing — corrected W1

Keep the `latency_class` idea, but routing is **node-class-first**:

- The Doorman has a **node-class probe** → knows which tiers physically exist.
- `$7 node`: only Tier B/C can exist; `select_tier` never routes to a (absent)
  Tier A; if no GPU/API tier is configured, AI requests get a clean
  `TierUnavailable` with an honest "this node has no AI tier" signal — **not** an
  OOM, **not** a silent slow path.
- `NUC node`: Tier A is the on-device interactive tier; **interactive prefers
  local Tier A** (the 1B specialist); Yo-Yo/C escalate for heavy work. Tier A is
  never hard-refused where it exists.
- The original W1 line "Interactive never defaults to Local" is **deleted** — it
  silently contradicted ratified doctrine.

---

## 8. THE TO-DO LIST — per repo

Concrete engineering plan, verified by an Opus software-engineering review
(2026-05-22). **Single-binary principle:** ONE build of `service-slm` and ONE of
`service-content` runs every node class with **no code alterations** — a runtime
probe detects the node and selects tiers/backends. No `#[cfg]` tier flags, no
per-tier builds. `[ ]` todo, `[x]` done. Execution order in §9.

### 8.A — project-intelligence (archive-level)
- [x] Rebuild this BRIEF on the $7-node doctrine (this document)
- [x] Log the #49-vs-convention working-set conflict in `NEXT.md` (§6)
- [x] Outbox note to Command: original investigation drifted from ratified
  conventions #49/#54/four-tier/tier-zero
- [ ] Update `.agent/manifest.md` `deployment:` leg — BLOCKED by cross-cluster contamination (Command scope)
- [x] Update `service-slm/CLAUDE.md` header with the node-class model
- [ ] `service-content/CLAUDE.md` — file doesn't exist; create when convenient (not on critical path)
- [x] Add a BRIEF pointer in `.agent/memory/MEMORY.md`

### 8.B — `foundry-nodeclass` (NEW shared crate)
- [x] Create `foundry-nodeclass` — 392 LOC, `NodeClass { Micro, Hardware, Accelerated }` + `Capabilities::detect()` + `TOTEBOX_NODE_CLASS` env override + 11 unit tests + doctest. `NodeClass::as_str()` added session 3.

### 8.C — service-slm  (clean single-binary — no obstacle)
- [x] `build_doorman()` (`main.rs`) — gates `local` on `caps.supports_on_node_ai()` + `SLM_FORCE_BROKER_MODE`. Reports `node_class`/`tier_a_reason` through `DoormanBoot`.
- [x] `select_tier()` (`router.rs`) — `micro_class_no_local_tier_unavailable` invariant test added.
- [x] `/readyz` (`http.rs`) — reports `node_class`, `tier_a`, `tier_a_reason`, `ai_available`.
- [x] `local-doorman.service` — `Requires=local-slm.service` → soft `Wants=`.
- [ ] `latency_class` field in `slm-core` (corrected W1) — deferred to Phase 6.
- [ ] Broker discipline — quarantine `idle_monitor.rs` behind a `BackendLifecycle` trait — deferred Phase 6.
- [ ] Reconcile the Tier A model drift (1B vs 7B-Think in env files) — deferred Phase 6.
- [ ] GF-1 async audit off the hot path · GF-2 Tier A client timeouts — deferred Phase 6.
- [ ] W5 remainder (Yo-Yo paid tier) — G5/G6/G9/G11–G16/G18 — deferred Phase 7.

### 8.D — service-content  (single-binary achievable; one caveat)
- [x] **`SqliteGraphStore`** — full implementation in `src/graph.rs`, 11-column schema matching LadybugDB, WAL mode, ON CONFLICT upsert.
- [x] Runtime backend selection — `Sqlite` on `Micro`, `Ladybug` on `Hardware`+; `SERVICE_CONTENT_GRAPH_BACKEND` env override.
- [x] Background CORPUS drain — `std::thread::spawn`; `ready: Arc<AtomicBool>`; `/healthz` warming gate; `/v1/graph/context` 503-while-warming.
- [x] Fixed legacy hardcoded `base_dir` default (`main.rs`).
- [x] **lbug single-binary — Option 1 LOCKED**: accept disk bloat, ship now. cargo check clean; cargo build fails at link (pre-existing lbug static-link issue).
- [x] Content reliability pass — `processed_ledgers` → `Arc<Mutex<HashSet<String>>>` (drain + watcher thread-safe).

### 8.E — CI / base-tier testing  (the never-built no-tier suite)
How we finally TEST the $7-node base tier the dev environment never hits.
- [ ] Forced-class tests — `TOTEBOX_NODE_CLASS=micro` integration tests in both
  services (`tests/micro_node.rs`): broker has no Tier A, `/readyz` honest, an
  AI request → clean 503 with no model-load attempt; `SqliteGraphStore` round-trips.
- [ ] **cgroup sandbox** — `scripts/run-micro-sandbox.sh` via
  `systemd-run --user -p MemoryMax=1G -p CPUQuota=25%` (no container —
  `zero-container-runtime.md`). The real test: the substrate must boot + serve
  in 1 GB; `MemoryMax=1G` auto-OOM-kills a wrong (LadybugDB) backend selection.
- [ ] The no-tier CI matrix (`node_class_matrix.rs`) — every (node-class × tiers)
  cell; the deterministic-operations suite passes in every row. This *is* the
  `substrate-without-inference-base-case.md` §8 suite — mandated, never built.
- [ ] `compute/opentofu/` — a `node-micro` e2-micro profile; real-hardware
  matrix run gated to MINOR releases.

### 8.F — gated (NUC on-device AI; not now)
- [ ] Tier A on-device 1B specialist as a real product — `LocalInferenceBackend`
  trait, accelerator backends, model packaging. **Gated** behind: §8.D done +
  the $7-node fleet verified booting + a named hardware-Totebox customer.

---

## 9. Execution order

1. **`foundry-nodeclass` crate (§8.B)** — both services depend on it; build first.
2. **`SqliteGraphStore` + runtime backend selection (§8.D)** — THE blocker;
   nothing about the $7-node fleet is real until `service-content` boots tiny.
3. **service-slm `build_doorman()` Tier-A gate + `/readyz` (§8.C)** — stops the
   Doorman falsely reporting Tier A / OOMing on a $7 node.
4. **§8.E base-tier tests + cgroup sandbox** — stand up early; it is how every
   later change is verified against the $7-node target.
5. **§8.C `select_tier`/`latency_class` + §8.D fit-boot/reliability** — parallel.
6. **Archive alignment (§8.A)** — manifest, NEXT.md, Command outbox — any time.
7. **GF-1 / GF-2** — small wins after the foundation.
8. **W5 remainder** — the Yo-Yo paid tier; independent.
9. **§8.F on-device AI build** — gated.

---

## 10. Definition of done

1. The deterministic substrate **boots and runs on a $7/mo e2-micro** (or the
   recorded e2-small floor) — `service-content` on the SQLite-graph backend,
   startup in seconds, total RSS within the node.
2. The Doorman runs as a **pure broker** on the $7 node, hosts no model, and
   reports tier availability honestly from the node-class probe — never OOMs.
3. A $7 node with no GPU/API tier configured is a **complete, useful product**
   (deterministic substrate; claim #54) — and says so clearly rather than
   pretending to have AI.
4. On a NUC-class node, **Tier A (the 1B specialist) switches on** and serves
   interactive on-device turns.
5. The Yo-Yo paid tier is cost-hardened (Phase 0 — done) and the BRIEF, manifest,
   and per-project docs all describe the node-class model consistently.
6. `cargo test` green across service-slm + service-content; clippy clean.

---

## 11. Provenance

Five rounds, ~13 Opus agents, 2026-05-21/22: flow-restructure (7 agents) →
W1/CPU-wall reversal (2) → Tier-A adversarial pair (2) → $7-node architecture +
feasibility (2). Plus a live GCP audit of the Yo-Yo fleet. The decisive round
grounded everything in DOCTRINE.md #49/#54. This BRIEF supersedes the
"flow-restructure" framing; the prior Phase 0 Yo-Yo work (§5) is retained intact.

---

## 12. Command Session — operational transition checklist

**Scope: Command Session only. Totebox does not run these steps.**
Prerequisite: Stage 6 promote must run first (local main is ahead of origin/main).

### 12.1 — Remove the 7B model from the workspace VM

The workspace VM is e2-standard-8 (Hardware-class). The 7B model is wrong for both
tiers (too large for Tier A = 1B specialist; too slow on CPU for Tier B = GPU-speed).
Weights file stays on disk; only the running process is stopped.

```bash
# Belt-and-suspenders: force broker mode in OLD binary before Phase 4 binary lands
echo "SLM_FORCE_BROKER_MODE=true" | sudo tee -a /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool  # verify has_local: false

# Stop and disable llama-server
sudo systemctl stop local-slm.service
sudo systemctl disable local-slm.service
systemctl status local-slm.service   # expect: inactive (dead)
free -h                              # expect: ~6 GB released; swap drops over 2-3 min
```

### 12.2 — Stage 6 + build + deploy new binaries

Prereq: Stage 6 promote (`bin/promote.sh`); then `bin/sync-local.sh --all`.

```bash
# Build slm-doorman-server (Phase 4+5 — node-class gate, 260 tests)
cd /srv/foundry/clones/project-intelligence/service-slm
cargo build --release -p slm-doorman-server
sudo cp /srv/foundry/cargo-target/mathew/release/slm-doorman-server \
    /usr/local/bin/slm-doorman-server
sudo systemctl restart local-doorman.service
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
# Expect: node_class, tier_a: false, tier_a_reason: micro-node-class, ai_available: false

# Build service-content (requires liblbug.so at link time)
ls /usr/local/lib/liblbug.so*    # confirm present
cargo build --release -p service-content 2>&1 | tail -5
# If fails: see BRIEF-lbug-build-blocker Options A/B/C
sudo systemctl stop local-content.service
sudo cp /srv/foundry/cargo-target/mathew/release/service-content \
    /usr/local/bin/service-content
sudo systemctl start local-content.service
curl -s http://127.0.0.1:9081/healthz   # expect: 200

# Update binary ledger (AGENT.md §10)
# Verify data/binary-ledger/slm-doorman-server.jsonl + service-content.jsonl
# last entries match sha256 of installed binaries
```

### 12.3 — Rebuild Elastic Compute Packer image

So Phase 0 cost guardrails (G3 dead-man's-switch, G17 sticky stops) take effect
on the next Elastic Compute VM. Without this the running image is pre-Phase-0.

```bash
cd /srv/foundry/clones/project-intelligence/compute/packer
packer build slm-yoyo.pkr.hcl
# Update manifest with new image ID
```

### 12.4 — End-to-end flow verification (requires Elastic Compute VM running)

```bash
curl http://127.0.0.1:9081/healthz    # service-content up
curl http://127.0.0.1:9080/readyz | python3 -m json.tool  # Doorman
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"olmo","messages":[{"role":"user","content":"ping"}]}'
```
