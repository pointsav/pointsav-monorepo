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

## Status — resume point (2026-05-22)

**Phase 0 — DONE** (Yo-Yo cost hardening, the paid AI tier): 3 signed commits
— `35e2dea7`, `ed63476c`, `a10539c6`. Yo-Yo spend is hard-capped at $3/day,
shutdown is verified, deliberate stops are sticky. Details in §5.

**This BRIEF — rebuilt 2026-05-22** on the $7-node doctrine. §8 is the full
per-repo to-do list; §9 the execution order; §10 the definition of done. BRIEF
commits this session: `5e41beb9` (rebuild), `8b5bd01b` (§8/§9 concrete plan),
+ this Status section.

**▶ RESUME HERE — §9 execution order, step 1:** build the `foundry-nodeclass`
crate (§8.B), then `SqliteGraphStore` (§8.D) — THE blocker; nothing about the
$7-node fleet is real until `service-content` boots on a 1 GB node.

**Open decision for the operator:** the lbug single-binary caveat in §8.D —
Option 1 (accept ~tens-of-MB disk bloat, ship now — *recommended*) vs Option 3
(LadybugDB as a side-car process, a Leapfrog follow-up). Resolve before §8.D
coding starts.

**Pending — Command Session (not Totebox scope):**
- Rebuild the `slm-yoyo` Packer image so Phase-0 G3/G17 take effect on the VM.
- Stage 6 promote — local `main` carries ~16 unpromoted commits (Phase 0 ×3 +
  3 BRIEF commits + pre-session backlog); needs a `git rebase origin/main`
  first (archive inbox `command-20260520-stage6-rebase-required`).
- Outbox note re: the original investigation drifting from ratified doctrine
  (§6) — queued as a §8.A to-do, not yet sent.

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
- [ ] Log the #49-vs-convention working-set conflict in `NEXT.md` (§6)
- [ ] Outbox note to Command: original investigation drifted from ratified
  conventions #49/#54/four-tier/tier-zero — recommend a doctrine cross-check
  step for future architecture briefs
- [ ] Update `.agent/manifest.md` `deployment:` leg — target shape is the
  $7/mo e2-micro fleet node, not the workspace VM
- [ ] Update `service-slm/CLAUDE.md` + `service-content/CLAUDE.md` headers with
  the node-class model
- [ ] Add a BRIEF pointer in `.agent/memory/MEMORY.md`

### 8.B — `foundry-nodeclass` (NEW shared crate)
The single mechanism both services use to adapt at runtime.
- [ ] Create `foundry-nodeclass` (~150 LOC, leaf crate): `NodeClass { Micro,
  Hardware, Accelerated }` + `Capabilities`. `detect()` reads RAM (cgroup v2
  `memory.max` / v1 / `/proc/meminfo` — take the min), vCPU (cgroup `cpu.max` /
  `nproc`), GPU (`/dev/nvidia*` `/dev/dri` filesystem probe — no CUDA link).
  Classify: GPU→`Accelerated`; ≥6 GiB & ≥1.5 vCPU→`Hardware`; else `Micro`.
  `TOTEBOX_NODE_CLASS` env override (the test lever) + synthetic/fixture
  constructors for unit tests.

### 8.C — service-slm  (clean single-binary — no obstacle)
- [ ] `build_doorman()` (`main.rs`) — gate `local` on `caps.supports_on_node_ai()`
  + `SLM_FORCE_BROKER_MODE`. **Today `local` is unconditionally `Some` — the
  Doorman falsely reports Tier A exists. This is the load-bearing fix.** Thread
  `node_class` onto `Doorman`.
- [ ] `select_tier()` (`router.rs`) — node-class-first policy; `Micro` never
  defaults to `Tier::Local`. Delete the "Interactive never defaults to Local"
  doc line. Add the invariant test.
- [ ] `/readyz` (`http.rs`) — report `node_class`, `tier_a` + `tier_a_reason`,
  `ai_available` from the probe, never from a model-load attempt.
- [ ] `slm-doorman.service` — `local-slm.service` becomes a soft `Wants=`.
- [ ] `latency_class` field in `slm-core` (corrected W1).
- [ ] Broker discipline — quarantine `idle_monitor.rs` behind a `BackendLifecycle`
  trait (tidiness; not a node-class blocker).
- [ ] Reconcile the Tier A model drift (1B vs 7B-Think in env files); pin
  OLMo 2 1B for the NUC tier; surface to Command for `permissible-model-substrate.md`.
- [ ] GF-1 async audit off the hot path · GF-2 Tier A client timeouts.
- [ ] W5 remainder (Yo-Yo paid tier) — G5/G6/G9/G11–G16/G18. Phase 0 done.

### 8.D — service-content  (single-binary achievable; one caveat)
- [ ] **Build `SqliteGraphStore`** (`src/graph_sqlite.rs`, ~250 LOC, `rusqlite`
  bundled) — implement every `GraphStore` trait method over a 2-table SQLite
  schema 1:1 with the LadybugDB `Entity` columns (preserves `worm_id`/`cites`
  provenance). The trait exists; LadybugDB is the only impl today — this is the
  missing piece.
- [ ] Runtime backend selection in `main.rs` — `Sqlite` on `Micro`, `Ladybug`
  on `Hardware`+; `SERVICE_CONTENT_GRAPH_BACKEND` env override.
- [ ] Background the CORPUS drain (16-min synchronous scan → `tokio::spawn`);
  `/healthz` warming/ready; `/v1/graph/context` 503-while-warming.
- [ ] Fix the legacy hardcoded `base_dir` default (`main.rs`).
- [ ] **lbug single-binary caveat — operator decision:** `lbug` (LadybugDB) is a
  C++ FFI crate, statically linked → its engine compiles into the binary even on
  a $7 node (~tens of MB *disk* bloat, not RAM). **Option 1:** accept it, ship
  now (recommended). **Option 3:** make LadybugDB a side-car process behind
  `GraphStore` (cleanest; Leapfrog follow-up). Option 2 (a Cargo feature = two
  builds) is rejected — it breaks the one-build rule.
- [ ] Content reliability pass — `processed_ledgers` → `HashSet`; remove
  panic-on-write surfaces; persistent deferred-retry queue.

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
