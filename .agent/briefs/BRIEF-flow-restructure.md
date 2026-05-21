---
artifact: brief
status: active
title: project-intelligence flow restructure
created: 2026-05-21
updated: 2026-05-21
author: task@project-intelligence (claude-code, Opus — 7-agent investigation + live GCP audit)
supersedes: .agent/plans/flow-bottleneck-strategic-review-2026-05-21.md
companion:
  - .agent/plans/tier-architecture-2026.md
  - .agent/plans/universal-ai-gateway.md
  - .agent/plans/sovereign-routing-comprehensive.md
  - .agent/plans/service-slm-hardening-2026-05-18.md
  - .agent/plans/olmo-performance-tuning.md
scope: service-slm, service-content, the Yo-Yo (Tier B) fleet
purpose: single plan of record AND full execution to-do list to upgrade project-intelligence to "flow"
---

# BRIEF — project-intelligence Flow Restructure

The consolidated plan **and the working to-do list** to restructure
`project-intelligence` so `service-slm` and `service-content` achieve "flow" —
a fluid, fast, trustworthy, cost-bounded experience for Customers and Community
Members. §7 is the checklist; work it top to bottom.

Built from a seven-agent Opus investigation (three rounds) plus a live GCP audit
of the Yo-Yo fleet (2026-05-21).

---

## Status — 2026-05-21 (resume point)

**Phase 0 — COMPLETE.** Three signed commits on local `main`:
- `35e2dea7` — G1 attempt cap · G3 dead-man's-switch · G8 $3/day cap
- `ed63476c` — G7 VM-hours ledger · G10 verified shutdown
- `a10539c6` — G17 sticky deliberate stops

The Yo-Yo is now cost-capped at **$3/day**, every VM-hour is ledgered, "shut
down" is verified (`TERMINATED` confirmed), and deliberate stops are sticky (no
self-restart). Gates green: `cargo check`/`clippy` clean; `cost_ledger` 5/5,
`idle_monitor` 24/24.

**Pending — Command Session (not Totebox scope):**
- Rebuild the `slm-yoyo` Packer image — G3 (dead-man's-switch) and the G17
  metadata wiring only take effect on a freshly built image. G1/G8 in
  `start-yoyo.sh` are live immediately.
- Stage 6 promote — local `main` carries ~13 unpromoted commits (incl. the 3
  above) plus the pre-session backlog; needs `bin/promote.sh` + a rebase first
  (see archive inbox `command-20260520-stage6-rebase-required`).

**In-flight / uncommitted:** none — Phase 0 is fully committed. Two untracked
files `yoyo-stability-gate.{service,sh}` sit in `compute/packer/scripts/` for
Phase 4 (G4 — spot-only, deliberately deferred).

**Resume here →** Phase 1 (§7): W1 `latency_class` routing + GF-1 async audit
writes + GF-2 Tier A client timeouts. Near-zero-risk, same-day; the one routing
change that stops interactive traffic landing on the 2 tok/s CPU.

---

## 1. The verdict — why there is no flow

1. **It is compute physics, not a code defect.** CPU inference of a 7B model is
   ~1.95 tok/s, memory-bandwidth bound — 13 s for a trivial reply, 75–130 s per
   paragraph. No engineering moves it. "Get a better model" is not the answer.
2. **Flow is achievable only on a GPU** — on an on-prem GPU box it is achievable
   today, one sprint series away from excellent.
3. **service-slm conflates two roles** — a lightweight *broker* and an *inference
   host*. The conflation is the structural defect.
4. **The deepest defect is positioning** — the architecture markets a "community
   tier" implying interactive AI on a CPU laptop; that config delivers a
   100-second wait per paragraph.

| Segment | Flow today? | Bottleneck |
|---|---|---|
| On-prem **GPU box** | ✅ yes (single user) | 16-min content startup; single-slot concurrency. Fixable. |
| **Rentable cloud GPU** (Yo-Yo) | Batch only | 3–4 min cold start; cost-leak + shutdown-trust defects. |
| **CPU commodity** (community default) | ❌ never | Memory bandwidth — hardware physics. |
| **Managed/hosted** | N/A | `app-console-slm` is an empty scaffold. |

---

## 2. Target architecture

- **service-slm = pure broker** — route / sanitise / audit / MCP / apprenticeship
  capture. New hard rule: **do not operate inference infrastructure**.
- **service-content = fast-start grounded graph** — serves in < 15 s, not 16 min.
- **Tier A (local CPU) = the async/batch tier**, never the interactive default.
- **service-yoyo** (NEW headless `service-*` fleet daemon) — Yo-Yo lifecycle.
  Gated on the GPU fleet growing past one node.
- **vendor-slm-engine** — Yo-Yo image + adapter artefacts. **app-console-slm** —
  operator surface. **Do NOT create `app-orchestration-slm`.**

---

## 3. Decisions locked — 2026-05-21

These are operator-ratified; the to-do list assumes them.

| # | Decision |
|---|---|
| D1 | **Daily Yo-Yo runs ON-DEMAND, not spot.** No preemption → no kicked-off boots → no per-boot waste. Spot is kept only for the periodic heavy Yo-Yo (batch). |
| D2 | **Co-locate the Yo-Yo fleet in `us-west1`** — same region as the workspace VM. Move the weights GCS bucket to `us-west1` too. Ends cross-region latency and cross-continent egress. |
| D3 | **Hard daily spend cap = $3.00/day** to start (env-tunable `SLM_YOYO_DAILY_BUDGET_USD`). At ~$0.71/hr that is ~4 h/day of GPU runtime. |
| D4 | **Two-Yo-Yo model** — a *daily* Yo-Yo (`l4-small`, on-demand, us-west1, $3/day) and a *periodic* heavy Yo-Yo (`h100`, monthly/quarterly, campaign-budgeted, may chase spot in a nighttime zone). |
| D5 | service-slm stays a `service-*`; do not create `app-orchestration-slm`. |

---

## 4. service-slm restructure

### 4.1 Tier-A repositioning (W1) — the single highest-leverage fix
`router.rs::select_tier` defaults `Complexity::Low | Medium → Tier::Local` — that
line makes a 2 tok/s CPU the front door. Add a `latency_class: { Interactive,
Batch }` field to `ComputeRequest`; re-key tier selection on it. `Interactive`
never defaults to Local. ~50 LOC, pure refactor, no ratification.

### 4.2 Broker/inference split — phases 1–2 only for now
Leaks are narrow: `idle_monitor.rs`, `adapter_registry.rs`, `compute/`,
`router-trainer/`. Phase 1 = W1. Phase 2 = `idle_monitor.rs` behind a
`BackendLifecycle` trait. Phases 3–5 (new projects) gated on fleet > 1 node.

### 4.3 GPU-segment hot-path sprints (GF-*)
| # | Sprint | Risk | LOC |
|---|---|---|---|
| GF-1 | Async audit/metrics/cost-ledger off the hot path (`router.rs:192`) | near-zero | ~70 |
| GF-2 | Tier A HTTP client timeouts (`tier/local.rs:38` — none today) | near-zero | ~40 |
| GF-3 | Drop hot-path `/health` busy-probe; map upstream 503→`TierABusy` | low | ~60 |
| GF-4 | Bound/opt-out graph-context query on `/v1/chat/completions` | low | ~50 |
| GF-6 | Concurrency: llama-server `--parallel N` + Doorman bounded queue | medium | ~190 |
| GF-7 | Streaming hardening (real token/stop accounting, idle-deadline) | low-med | ~120 |

---

## 5. service-content restructure

### 5.1 Near-instant startup (GF-5)
The 16-min startup is misdiagnosed: a 2 GB LadybugDB buffer pool
(`SERVICE_CONTENT_LBUG_BUFFER_POOL_MB=2048` env override; code default 64 MB)
thrashing a 3 GB cgroup, **plus** 131 CORPUS files drained synchronously on the
boot thread. Fix (~180 LOC): drop the env override + clamp guard; background the
backlog drain after HTTP bind; readiness-gate `/healthz` + `/v1/graph/context`
(503 `warming`, never a silent empty `200 []`).

### 5.2 Deferred-retry queue
The persistent extraction queue + `POST /v1/yoyo/up` notification
(`tier-architecture-2026.md §1`) was never implemented — every restart re-storms
all deferred files. ~50 LOC + small Doorman extension.

### 5.3 No local deterministic extractor — flagged
Without a GPU the graph fills only with empty `Source` stubs. `service-extraction`
(deterministic, zero-AI) is described but not built. Community-tier honesty gap.

---

## 6. Yo-Yo (Tier B) hardening — W5

### 6.A The $50 incident — root cause (CONFIRMED by live audit)
- **Live state:** `yoyo-tier-b-1` is a stopped SPOT VM in `europe-west4`; the
  workspace is `us-west1`; the weights GCS bucket is `us-central1` — **three
  regions, two continents.**
- **Free probing already works on the normal path:** the VM persists stopped with
  its disks, so a routine start is `instances start` (Mode 1) — a stockout there
  bills **$0**. The topology is right in that respect.
- **The $50 was NOT a free probe failing.** It was (a) a Mode-2 zone migration
  (`SLM_YOYO_ALLOW_ZONE_FALLBACK=true` — creates disks/VMs in new zones; the
  2026-05-13 us-west1→europe-west4 move), compounded by cross-continent weights
  egress (~18 GB from a `us-central1` bucket ≈ $1–2/pull); and (b) **spot
  preemption churn** — the model auto-loads at boot, the spot VM is preempted
  mid-load, the cycle repeats, each cycle paying a doomed load.
- **Accounting blind spot:** `cost_usd` counts only inference-ms (`yoyo.rs:120`).
  Boot/idle/load/preempt VM-hours record `$0.00` — the $50 was invisible to the
  service's own ledger.

### 6.B Hardened LAUNCH protocol
Attempt cap + dollar budget on `start-yoyo.sh`; stability gate before model load;
create the VM before the disk. Under D1+D2 (on-demand, fixed us-west1 zone) the
zone-scanning path is largely retired — on-demand has capacity in a fixed zone.

### 6.C Hardened SHUTDOWN protocol — "shut down" must be trustworthy
1. **Never assert "shut down" without verification** — poll `instances describe
   status` until `TERMINATED`.
2. **Shutdown intent suppresses auto-restart** — a `stop_reason`; `Operator` /
   `Deadman` / `BudgetCeiling` stops are sticky.
3. **Single-glance fleet truth** — a `yoyo-status` command + `/v1/yoyo/status`.
4. **VM-side dead-man's-switch** — self-stop at `max-lifetime`.
5. **Orphan reaper** — all-zones sweep.

### 6.D Cost model + the two-Yo-Yo pattern (D3 / D4)

**On-demand pricing:** billed per-second only while `RUNNING`; **$0 while
stopped** (disks still bill ~$30/mo). `g2-standard-4` + L4 ≈ **$0.71/hr**
on-demand (confirm the `us-west1` SKU). No preemption → cost is deterministic:
`cost = rate × hours_running`.

**The $3/day cap (D3)** converts cleanly to a runtime ceiling:
```
$3.00/day ÷ ~$0.71/hr ≈ 4.2 hours of GPU runtime per day
```
Enforced three ways:
1. **VM-hours ledger (G7)** — date-keyed; `spend_today` always known.
2. **Start refusal (G8)** — if `spend_today ≥ $3`, the launch is refused with an
   explicit "daily budget reached" status.
3. **Budget-derived dead-man's-switch** — `start-yoyo.sh` sets
   `max-lifetime = min(default, (3.00 − spend_today) ÷ rate)`; the VM self-stops
   when the day's money is spent, with zero Doorman dependence.

| Role | Class | Provisioning | Cadence | Budget | Zone |
|---|---|---|---|---|---|
| **Daily Yo-Yo** | `l4-small` | **on-demand** | most days | **Daily — $3/day** | `us-west1` |
| **Periodic heavy Yo-Yo** | `h100` | spot OK | monthly/quarterly | **Campaign** (separate, operator-authorised) | nighttime zone OK |

### 6.E Canonical zero-config defaults
`l4-small`; **on-demand**; `us-west1`; daily ceiling **$3/day**; launch attempt
cap 3; launch budget $10/campaign (backstop — the daily cap is the real control);
stability window 120 s; max VM lifetime 4 h (or budget-derived); idle timeout
30 min; cold-start UX = explicit "warming ~Ns" status, never a silent CPU drop.

### 6.F Tiers and the break point
| Tier | Config | First-request latency | Honest $/mo |
|---|---|---|---|
| **Community Entry** | `l4-small`, spot, lazy | 3–4 min cold, then interactive | ~$32 (mostly disk) |
| **Customer Standard** (the daily Yo-Yo) | `l4-small`, **on-demand, us-west1**, $3/day ≈ 4 h | ~1 s, no preemption, no cross-region | ~$90 compute + ~$30 disk ≈ **~$120**, hard-capped $3/day |
| **Customer Premium** | `l4-std`/`h100`, reservation, warm | ~1 s always | ~$330–500 |

(Correction: an earlier draft used ~$0.45/hr — too low. Realistic on-demand
`g2-standard-4`+L4 is ~$0.71–0.85/hr.)

### 6.G YOYO topology fix (G19 — from the live audit)
Current: YOYO `europe-west4`, bucket `us-central1`, workspace `us-west1`. Target:
**all in `us-west1`.** Move the weights bucket to `us-west1`; reprovision the
daily Yo-Yo in `us-west1` on-demand (one-time migration; same-continent weights
pull is cheap). Result: +150 ms cross-region latency → ~0; cross-continent egress
→ 0; no zone-scanning.

### 6.H Gap table (G1–G19)
| # | Sev | Location | Gap → fix | ~LOC |
|---|---|---|---|---|
| G1 | CRIT | `start-yoyo.sh` | No attempt cap → `SLM_YOYO_MAX_LAUNCH_ATTEMPTS` | 40 |
| G2 | CRIT | `start-yoyo.sh` | No dollar guard → `SLM_YOYO_LAUNCH_BUDGET_USD`, persisted | 70 |
| G3 | CRIT | `compute/packer/` | No VM-side teardown → `yoyo-deadman.service/.sh` | 50 |
| G4 | CRIT | `compute/packer/` | Model auto-loads at boot → `yoyo-stability-gate` | 80 |
| G5 | HIGH | `start-yoyo.sh:224` | No `--labels`/`resource_policies` → add both | 10 |
| G6 | HIGH | new `yoyo-reaper` | No orphan detection → all-zones sweep | 120 |
| G7 | HIGH | `cost_ledger.rs` | Ledger sees only inference-ms → `VmHoursRow` | 110 |
| G8 | HIGH | `cost_ledger.rs`, router, `start-yoyo.sh` | No daily cap → **$3/day** ledger + start-refusal + budget-derived `max-lifetime` | 110 |
| G9 | HIGH | `router.rs:744`, `yoyo.rs:49` | 90 s auto-start budget < cold start → raise, "warming" status | 50 |
| G10 | HIGH | `stop-yoyo.sh`, `idle_monitor.rs` | Stop trusts op-accepted → poll until `TERMINATED` | 50 |
| G11 | MED | `idle_monitor.rs:318` | Crash-guard stop only when auto-restart off → absolute max-lifetime stop | 40 |
| G12 | MED | `start-yoyo.sh:186` | Disk created before VM → invert order | 25 |
| G13 | MED | `start-yoyo.sh:481` | In-process watchdog dies with shell → delete; rely on G3 | -15 |
| G14 | MED | `router.rs:34` | No per-instance state → `YoYoInstanceState` (+ `budget_class`) | 120 |
| G15 | LOW | `opentofu/main.tf:42` | Disk spec drift → reconcile | 10 |
| G16 | LOW | `guide-operating-yoyo.md` | "on-demand, no preemption" contradicts SPOT → editorial fix | doc |
| G17 | HIGH | `idle_monitor.rs` | Auto-restart can't tell deliberate stop from idle → `stop_reason`, sticky stops | 60 |
| G18 | HIGH | new `yoyo-status` + `/v1/yoyo/status` | No single-glance fleet truth → all-zones status report | 70 |
| G19 | HIGH | topology / `opentofu` / bucket | YOYO+bucket+workspace span 3 regions → co-locate all in `us-west1`; daily Yo-Yo on-demand | ops + ~30 |

---

## 7. THE TO-DO LIST — phased execution checklist

Work top to bottom. `[ ]` = todo, `[x]` = done+committed. Each phase is a
commit boundary. Phases 0–6 are Totebox-scope code in this archive — no
ratification needed to start.

### Phase 0 — DONE ✅ — YOYO cost control + dead-man's-switch + shutdown trust
Committed 2026-05-21 in three signed commits: `35e2dea7`, `ed63476c`, `a10539c6`.
- [x] G1 — launch attempt cap in `start-yoyo.sh` (`35e2dea7`)
- [x] G3 — `yoyo-deadman.service`/`.sh` VM-side dead-man's-switch (`35e2dea7`)
- [x] G8 — **$3/day** daily ceiling: GCP-timestamp reconciliation + start-refusal + budget-derived `max-lifetime` (`35e2dea7`). G2 folded in — one dollar budget (the daily cap), not two.
- [x] G7 — `VmHoursRow` in `cost_ledger.rs` + `daily_rollup.vm_hours_cost_usd` (`ed63476c`)
- [x] G10 — verify `TERMINATED` before reporting shutdown — `stop-yoyo.sh` + `idle_monitor.rs` (`ed63476c`)
- [x] G17 — `last-stop-reason` metadata; sticky operator/deadman/idle stops; the idle monitor will not self-restart a deliberate stop (`a10539c6`)
- [x] gates: `cargo check`/`clippy` clean; cost_ledger 5/5 + idle_monitor 24/24 pass
- [ ] **Pending — Command Session:** rebuild the `slm-yoyo` Packer image so G3 (dead-man's-switch) + the G17 metadata wiring take effect on the VM; Stage 6 promote the three commits

### Phase 1 — same-day, near-zero-risk latency win
- [ ] W1 — `latency_class` field in `slm-core` + rewrite `router.rs::select_tier`
- [ ] GF-1 — async audit/metrics/cost-ledger off the hot path
- [ ] GF-2 — Tier A HTTP client timeouts
- [ ] `cargo test --workspace` green; commit

### Phase 2 — YOYO fleet observability + orphan-proofing
- [ ] G18 — `yoyo-status` command + `/v1/yoyo/status` endpoint (single-glance fleet truth)
- [ ] G5 — `--labels` + `resource_policies` on instance creation
- [ ] G6 — `yoyo-reaper` all-zones orphan sweep
- [ ] G9 — raise auto-start budget past cold start; explicit "warming" status
- [ ] G14 — `YoYoInstanceState` per-instance state (incl. `budget_class`)
- [ ] `cargo test` green; commit

### Phase 3 — co-locate the Yo-Yo fleet (G19) — needs operator/Command
- [ ] Create a `us-west1` weights GCS bucket; copy the GGUF artefact into it
- [ ] Reprovision the daily Yo-Yo in `us-west1` **on-demand** (one-time migration)
- [ ] Update `SLM_YOYO_*` env (zone, endpoint, bucket) + `opentofu/main.tf`
- [ ] Decommission the `europe-west4` VM + disks once verified
- [ ] G15 — reconcile the disk-spec drift in `opentofu/main.tf`
- [ ] G16 — editorial fix to `guide-operating-yoyo.md` (route via project-editorial)

### Phase 4 — GPU-segment hot path
- [ ] GF-3 — drop hot-path busy-probe; map upstream 503→`TierABusy`
- [ ] GF-4 — bound/opt-out graph-context on `/v1/chat/completions`
- [ ] G11 — absolute max-lifetime stop in `idle_monitor.rs`
- [ ] G12 — invert disk/VM create order in `start-yoyo.sh`
- [ ] G13 — delete the in-process `--runtime` watchdog (G3 replaces it)
- [ ] G4 — `yoyo-stability-gate` (spot/periodic Yo-Yo only — the on-demand daily Yo-Yo has no preemption, so it does not need this)
- [ ] `cargo test` green; commit

### Phase 5 — service-content near-instant startup
- [ ] GF-5 — buffer-pool clamp + background backlog drain + readiness gating
- [ ] 5.2 — persistent deferred-retry queue + `POST /v1/yoyo/up`
- [ ] content reliability pass (`processed_ledgers` → `HashSet`; panic surfaces)
- [ ] `cargo test` (service-content) green; commit

### Phase 6 — concurrency + streaming honesty
- [ ] GF-6 — llama-server `--parallel N` + Doorman bounded admission queue
- [ ] GF-7 — streaming hardening (token/stop accounting, idle-deadline, tool-use)
- [ ] `cargo test --workspace` green; commit

### Phase 7 — positioning + governance (gated)
- [ ] W4 — positioning correction: community = Rings 1+2, no interactive-AI promise (route via project-editorial / Command)
- [ ] `conventions/three-ring-architecture.md` one-row edit (Command/Master)
- [ ] Broker split phases 3–5 — `service-yoyo` + `vendor-slm-engine` promotion — **gated on the GPU fleet growing past one node**
- [ ] Remove committed `compute/opentofu/terraform.tfstate` from git (operator interrogation protocol)

---

## 8. Governance / ratification flags
- `conventions/three-ring-architecture.md` — one-row edit after the broker split (Command/Master).
- `service-yoyo` new project + `vendor-slm-engine` promotion — `project-registry.md` rows (Command). Gated on fleet growth.
- `compute/opentofu/terraform.tfstate` committed in git — should not be; removal needs operator interrogation.
- W4 positioning touches BCSC disclosure posture — Command/editorial review.

---

## 9. Definition of done — the measurable "110%"
1. Interactive `/v1/messages` p99 time-to-first-token < 3 s on a GPU box.
2. `service-content` serving `/healthz` + `/v1/graph/context` within 15 s of start.
3. Interactive traffic never lands on the 2 tok/s CPU tier.
4. Graceful concurrency — overflow is a queued hold, not a hard 503.
5. No infinite hangs; every tier client has a bounded timeout.
6. Yo-Yo spend is hard-capped at **$3/day**; every VM-hour is in the ledger; a
   launch campaign cannot exceed its budget.
7. "Shut down" is verified (`TERMINATED` confirmed) before reported; a deliberate
   stop is never silently reversed; `yoyo-status` answers "is anything running?"
   in one command.
8. The Yo-Yo fleet, weights bucket, and workspace are co-located in `us-west1`.
9. `cargo test --workspace` (service-slm) + `cargo test` (service-content) green;
   `clippy -D warnings` + `fmt --check` clean.

---

## 10. Provenance
Seven Opus investigation agents (three rounds) + a live GCP audit, 2026-05-21.
R1: slm internals; content internals; customer-environment simulation.
R2: broker/inference split; Yo-Yo cold start; GPU-segment sprint plan.
R3: Yo-Yo launch/shutdown hardening. Live audit: Yo-Yo fleet GCP state.
Supersedes `.agent/plans/flow-bottleneck-strategic-review-2026-05-21.md`.
