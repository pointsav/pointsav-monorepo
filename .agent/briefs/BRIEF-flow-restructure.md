---
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

Work it top to bottom within each repo. `[ ]` todo, `[x]` done. Execution order
across repos is in §9.

### 8.A — project-intelligence (archive-level)
- [x] Rebuild this BRIEF on the $7-node doctrine (this document)
- [ ] Log the #49-vs-convention working-set conflict in `NEXT.md` (§6)
- [ ] Outbox note to Command: original flow-restructure investigation drifted
  from ratified conventions #49/#54/four-tier/tier-zero — recommend a doctrine
  cross-check step for future architecture briefs
- [ ] Update `.agent/manifest.md` `deployment:` leg — the target deployment
  shape is the $7/mo e2-micro fleet node, not the workspace VM
- [ ] Update `service-slm/CLAUDE.md` + `service-content/CLAUDE.md` headers to
  state the node-class model (fleet = $7 node, deterministic + broker)
- [ ] `.agent/memory/` already carries `project_flow_cpu_wall_correction`; add a
  pointer to this rebuilt BRIEF

### 8.B — service-slm
- [ ] **Node-class capability probe** — at startup the Doorman detects RAM / CPU
  / GPU and computes which tiers are physically possible. `/readyz` reports
  `tier_a: ok|unavailable` from the probe, never from a model-load attempt.
- [ ] **Corrected W1 routing** — `latency_class` field in `slm-core`;
  `select_tier` keyed node-class-first (§7). Delete "Interactive never defaults
  to Local." Add the invariant: on a node where Tier A exists, interactive never
  hard-refuses it.
- [ ] **Broker discipline** — enforce "the Doorman operates no inference
  infrastructure / hosts no model." Quarantine `idle_monitor.rs` (Yo-Yo
  lifecycle) behind a `BackendLifecycle` trait; the broker discovers backends,
  never builds them.
- [ ] Reconcile the **Tier A model drift** — `local-slm.service` vs
  `local-doorman.service` disagree (1B vs 7B-Think); pin to OLMo 2 1B for the
  NUC Tier A; surface to Command for `permissible-model-substrate.md`.
- [ ] GF-1 — async audit/metrics/cost-ledger off the response hot path.
- [ ] GF-2 — Tier A HTTP client timeouts (`tier/local.rs` has none).
- [ ] W5 remainder (Yo-Yo hardening, the paid tier) — G5/G6/G9/G14/G18 +
  G11–G16: orphan reaper, `yoyo-status`, per-instance state, etc. Phase 0
  (G1/G3/G7/G8/G10/G17) already done.
- [ ] Audit cgroup/`MemoryMax` settings for the $7-node footprint (Doorman is
  `512M` — confirm it holds; the broker must be tiny).

### 8.C — service-content
- [ ] **LadybugDB → SQLite-graph backend (THE blocker, §4)** — finish/implement
  the SQLite-graph `GraphStore` impl that `ARCHITECTURE.md` §4 already specifies
  for Tier 0; make it the default for $7-node / Tier-0 deployments. Keep
  LadybugDB as an opt-in for large nodes.
- [ ] Fit + boot on a tiny node — clamp any buffer pool to the code default;
  background the synchronous CORPUS-drain so startup is seconds, not 16 min
  (the old GF-5 work — now a fitness-to-boot prerequisite, not an optimization).
- [ ] Readiness gating — `/healthz` reports `warming` during load;
  `/v1/graph/context` returns a 503 signal, never a silently-empty graph.
- [ ] Verify the deterministic substrate's total RSS fits ~1 GB on the SQLite
  backend; if it cannot, commit the floor to e2-small (~$14/mo) and record it.
- [ ] Content reliability pass — `processed_ledgers` → `HashSet`; remove
  panic-on-write surfaces; persistent deferred-retry queue.

### 8.D — gated (NUC rung; not now)
- [ ] Tier A on-device 1B specialist as a real product — `LocalInferenceBackend`
  trait, accelerator backends, model packaging. **Gated** behind: §8.C done +
  the $7-node fleet verified booting + a named hardware-Totebox customer.

---

## 9. Execution order

1. **service-content SQLite-graph backend (§8.C item 1)** — the blocker;
   nothing about the $7-node fleet is real until `service-content` boots on a
   tiny node. Start here.
2. **service-slm node-class probe (§8.B item 1)** — so the Doorman behaves
   correctly (no OOM, honest `/readyz`) on a $7 node.
3. **§8.C fit/boot + §8.B corrected W1** — in parallel; both are small once 1–2
   land.
4. **Archive alignment (§8.A)** — manifest, NEXT.md conflict log, Command
   outbox — can happen any time; do it alongside 1–3.
5. **GF-1 / GF-2 + service-content reliability** — small wins, after the
   foundation.
6. **W5 remainder** — the Yo-Yo paid tier; independent, schedule as capacity
   allows.
7. **§8.D Tier A on-device build** — gated; not until the fleet foundation is
   verified.

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
