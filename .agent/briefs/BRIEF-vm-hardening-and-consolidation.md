---
artifact: brief
status: absorbed
archived: 2026-05-23
absorbed: 2026-05-24
absorbed_into: BRIEF-slm-substrate-master.md
archived_reason: >
  §3A conflicts (lbug + Ring 2/3 coupling) resolved and recorded in
  flow-restructure §8.D and §4.1 (session 4, operator-approved). §3B/3C/3D
  BRIEF classification executed session 4. §1+§2 Command Session operational
  steps absorbed into flow-restructure §12. Nothing unique remains.
absorbed_reason: >
  All remaining open items carried into BRIEF-slm-substrate-master.md §5/§6
  (2026-05-24). That BRIEF is now PRIMARY PLAN OF RECORD replacing
  flow-restructure.md (lost in 2026-05-22 rebase). This BRIEF is fully superseded.
title: VM hardening, single-binary all-tier deploy, and BRIEF consolidation
created: 2026-05-23
author: totebox@project-intelligence (claude-sonnet-4-6)
grounds_in:
  - BRIEF-flow-restructure.md (PRIMARY PLAN OF RECORD)
  - DOCTRINE.md claims #49, #54
  - conventions/four-tier-slm-substrate.md
scope: project-intelligence archive · service-slm · service-content · workspace VM
purpose: |
  Four-item to-do list: (1) remove the 7B model from the workspace VM,
  (2) harden service-slm + service-content for all-tier operation on a single
  binary, (3) consolidate all active BRIEFs against the PRIMARY PLAN with
  conflicts surfaced for operator approval, (4) complete pending artifacts
  (Stage 6, binary deploy, README).
---

# BRIEF — VM hardening, single-binary deploy, BRIEF consolidation

> **Read BRIEF-flow-restructure.md first.** This BRIEF is downstream of it.
> All architectural decisions are in flow-restructure; this BRIEF is the
> operational execution checklist for the next session block.

---

## 1 — Remove the 7B model from the workspace VM

**Why:** The workspace VM is e2-standard-8 (Hardware-class, not Accelerated).
The 7B model is wrong for both tiers it could be (too large for Tier A = 1B
specialist; too slow on CPU for Tier B = GPU-speed). It consumes 6+ GB RAM
and is the cause of swap pressure and inference hangs. BRIEF-flow-restructure
§8.F gates on-device AI behind a named hardware-Totebox customer — not now.

**What "remove" means:** stop the model process and prevent the Doorman from
configuring Tier A locally. The weights file stays on disk (do not delete — it
may be needed for future NUC-rung testing). The Doorman becomes a pure broker
routing to Tier B (Yo-Yo) when the Yo-Yo is running.

### Todo

- [ ] **1a. Deploy Phase 4 binary** (slm-doorman-server) — the new binary
  detects node class at startup and gates Tier A correctly. Without it, the
  running Doorman has `has_local: true` (old binary) and will keep trying to
  reach llama-server. Stage 6 must run first (Command Session scope).
  *Prerequisite: §4 Stage 6 promote.*

- [ ] **1b. Stop `local-slm.service`**
  ```
  sudo systemctl stop local-slm.service
  sudo systemctl disable local-slm.service
  ```
  Disable prevents auto-restart on VM reboot until we decide what the VM
  is standing in for (NUC-rung or Yo-Yo dev stand-in).

- [ ] **1c. Set `SLM_FORCE_BROKER_MODE=true`** in `/etc/local-doorman/local-doorman.env`
  as a belt-and-suspenders until the Phase 4 binary is deployed. This makes
  the OLD binary also stop routing to Tier A. Add line:
  ```
  SLM_FORCE_BROKER_MODE=true
  ```
  Then `sudo systemctl restart local-doorman.service`.

- [ ] **1d. Verify `/readyz`** shows `has_local: false` (old binary) or
  `tier_a: false, tier_a_reason: force-broker-mode` (Phase 4 binary).
  ```
  curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
  ```

- [ ] **1e. Verify RAM freed** — `free -h` should show ~6 GB released.
  `systemctl status local-slm.service` should show `inactive (dead)`.

- [ ] **1f. Verify swap drops** — give the kernel 2–3 minutes to reclaim.
  `free -h` swap should fall from ~4.9 GB toward ≤1 GB.

---

## 2 — Harden service-slm + service-content: single binary, all tiers

**Why:** The Phase 4/5 code (committed, Stage 6 pending) is the ratified
single-binary design. The running binaries on the VM are the OLD versions
(pre-Phase 4). The new binaries must be built and deployed before the
node-class gate, honest `/readyz`, and SqliteGraphStore take effect in
production.

**Single-binary principle** (BRIEF-flow-restructure §8, ratified):
ONE build of each service adapts at runtime — no `#[cfg]` tier flags, no
per-tier builds. `TOTEBOX_NODE_CLASS` env override + `foundry-nodeclass`
probe handles class selection; `SERVICE_CONTENT_GRAPH_BACKEND` env override
handles backend selection.

### Todo

- [ ] **2a. Stage 6 promote** (Command Session — prerequisite for all below).
  Local `main` is 17+ commits ahead of `origin/main`. Command must:
  ```
  git rebase origin/main   # or git merge — see memory: merge vs rebase canonical
  bin/promote.sh
  bin/sync-local.sh --all
  ```

- [ ] **2b. Build new `slm-doorman-server` binary**
  ```
  cd /srv/foundry/clones/project-intelligence/service-slm
  cargo build --release -p slm-doorman-server
  ```
  Expected: the binary at `target/release/slm-doorman-server` includes
  Phase 4 node-class gate + Phase 5 test suite (260 tests).

- [ ] **2c. Deploy `slm-doorman-server`**
  ```
  sudo cp /srv/foundry/cargo-target/mathew/release/slm-doorman-server \
      /usr/local/bin/slm-doorman-server
  sudo systemctl restart local-doorman.service
  ```
  Verify: `curl http://127.0.0.1:9080/readyz` — must include `node_class`,
  `tier_a`, `tier_a_reason`, `ai_available` fields (Phase 4 additions).

- [ ] **2d. Build new `service-content` binary**
  Note: `cargo build -p service-content` requires `liblbug.so` at link time
  (pre-existing; see BRIEF-lbug-build-blocker §conflict below). The shared
  library must be present on the build VM. Check first:
  ```
  ls /usr/local/lib/liblbug.so*
  cargo build --release -p service-content 2>&1 | tail -20
  ```
  If linker fails, the deployed binary at `/usr/local/bin/service-content`
  is still the old one — flag and defer; the rest of §2 still applies to
  the Doorman.

- [ ] **2e. Deploy `service-content`** (if 2d succeeds)
  ```
  sudo systemctl stop local-content.service
  sudo cp /srv/foundry/cargo-target/mathew/release/service-content \
      /usr/local/bin/service-content
  sudo systemctl start local-content.service
  ```
  New binary selects backend at startup:
  - `Hardware` class (this VM) → `LadybugDB` (unchanged for large nodes)
  - Override with `SERVICE_CONTENT_GRAPH_BACKEND=sqlite` if you want SQLite
    on this VM for testing.

- [ ] **2f. Verify end-to-end flow** once Yo-Yo VM is running:
  ```
  # service-content up:
  curl http://127.0.0.1:9081/healthz
  # Doorman can reach it:
  curl http://127.0.0.1:9080/readyz | python3 -m json.tool
  # Full flow (Yo-Yo must be on):
  curl -s http://127.0.0.1:9080/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{"model":"olmo","messages":[{"role":"user","content":"ping"}]}'
  ```

- [ ] **2g. Rebuild Yo-Yo Packer image** (Command Session) so Phase 0
  hardening (G3 dead-man's-switch, G17 sticky stops) takes effect. Without
  this, the Yo-Yo VM runs on the old image and the cost guardrails are not live.

---

## 3 — BRIEF consolidation audit (operator decision required)

The Explore agent audited all 17 active BRIEFs against BRIEF-flow-restructure.
Findings below. Operator must approve the CONFLICT resolutions before BRIEFs
are marked archived.

### 3A — CONFLICTS (operator decision required before proceeding)

These two BRIEFs contain claims that BRIEF-flow-restructure does NOT address.
They are not wrong — they are gaps in the PRIMARY PLAN.

**BRIEF-lbug-build-blocker.md vs. flow-restructure:**
- flow-restructure §8.D says "lbug single-binary — Option 1 LOCKED: accept
  disk bloat, ship now. cargo check clean."
- lbug-build-blocker says `cargo build -p service-content` fails at link
  time (static archive missing antlr4/utf8proc symbols) — the binary cannot
  currently be rebuilt from source.
- **Gap:** flow-restructure assumes the build works; lbug-build-blocker says
  it does not. Todo §2d above hits this directly.
- **Proposed resolution:** add a note to flow-restructure §8.D: "cargo build
  requires `liblbug.so` present on the build machine (shared-link path);
  build from source is blocked until lbug publishes a complete static
  archive." Keep lbug-build-blocker active as the detailed record.
- [ ] **Operator: approve or redirect this resolution?**

**BRIEF-service-content-architecture.md vs. flow-restructure:**
- service-content-architecture identifies a Ring 2/Ring 3 coupling violation:
  the graph halts when the Doorman is unavailable (deterministic substrate
  depends on AI boundary — backwards).
- flow-restructure does not mention this coupling issue.
- service-content-architecture also outlines a PUSH inversion (5-sprint fix)
  that flow-restructure does not cover.
- **Gap:** the PRIMARY PLAN assumes service-content is structurally sound;
  it is not.
- **Proposed resolution:** add a §4.1 note to flow-restructure: "service-
  content has a Ring 2/3 coupling defect (Doorman unavailability halts graph
  — backwards). Fix is a 30-LOC Source node write before the Doorman call.
  Tracked in BRIEF-service-content-architecture §Sprint 1." Keep
  service-content-architecture active.
- [ ] **Operator: approve or redirect this resolution?**

### 3B — EXTENDS (keep active, no conflict)

These BRIEFs operationalize what flow-restructure sketches. No changes needed.

| BRIEF | Keeps because |
|---|---|
| BRIEF-sovereign-routing-comprehensive.md | Legal research, Sprint 0–5 engineering detail, training data format, LoRA hyperparameters |
| BRIEF-universal-ai-gateway.md | Exact LOC counts, file-by-file Sprint breakdown, pricing/customer transition milestones |
| BRIEF-learning-loop-master-plan.md | Corpus quality gate, eval harness, DPO pairs, Sigstore signing — Phase 1.1–1.10 specifics |
| BRIEF-tier-architecture.md | Model family ratification, BCSC-permissible families, 7 gap priorities, annual refresh policy |
| BRIEF-service-slm-hardening.md | Post-crash recovery state, Task 2–5 immediate next steps |
| BRIEF-phase-3c-service-content-loRA-stub.md | Deferred Phase 3.6–3.10 items (draft generation, citation linkage, LoRA scheduler) |

- [ ] **Mark `BRIEF-flow-bottleneck-strategic-review.md` `status: archived`**
  — it already declares itself superseded by flow-restructure.

### 3C — ABSORBED (safe to archive after operator confirms)

These BRIEFs contain only information already subsumed into flow-restructure
or its EXTENDS companions. No unique claims.

| BRIEF | Safe to archive because |
|---|---|
| BRIEF-MASTER-PLAN-2026.md | Entry-point index; flow-restructure IS the synthesis |
| BRIEF-olmo-performance-tuning.md | Measurement log for the 7B model we are now removing |
| BRIEF-service-audit.md | Defect list from pre-Phase-4 state; all items in flow-restructure §8 |
| BRIEF-service-slm-architecture.md | Sprint 0a prerequisite check; verified and executed |

- [ ] **Operator: approve archiving these four?** Action: set `status: archived`
  in frontmatter of each.

### 3D — UNRELATED (no action)

These BRIEFs are different domains. Leave as-is.

- BRIEF-layer3-compliance-report.md (security/WireGuard — URGENT, separate track)
- BRIEF-KNOWLEDGE-PLATFORM-PLAN.md (wiki architecture)
- BRIEF-KNOWLEDGE-PLATFORM-VISION.md (wiki architecture)
- BRIEF-claim-authoring-convention.md (editorial convention)

---

## 4 — Complete pending artifacts

### 4A — Stage 6 + binary deploy (Command Session scope)

- [ ] **Stage 6 promote** — `bin/promote.sh` from workspace root.
  17 commits on local `main` ahead of `origin/main`. Includes:
  - Phase 2: `foundry-nodeclass` crate (392 LOC, 12 tests)
  - Phase 3: `SqliteGraphStore` + runtime backend selection
  - Phase 4: Doorman node-class gate + `/readyz` (255 tests)
  - Phase 5: `micro_node.rs` integration tests + cgroup sandbox (260 tests)
  After promote: `bin/sync-local.sh --all` to pull canonical into live-service paths.

- [ ] **Binary ledger update** — after each binary deploy, verify
  `data/binary-ledger/<name>.jsonl` has a fresh entry with sha256 matching
  the installed binary (AGENT.md §10).

### 4B — README index update

- [ ] **Update `.agent/briefs/README.md`** — add this BRIEF, mark
  flow-bottleneck as archived, update status of absorbed BRIEFs once operator
  approves §3C.

### 4C — session-context update

- [ ] **Update `.agent/memory/session-context.md`** at shutdown — prepend
  new session entry per AGENT.md shutdown step §2b.

---

## Resume order

1. **§1c** (SLM_FORCE_BROKER_MODE) — immediate, no build needed, frees 6 GB RAM
2. **§1b** (stop local-slm) — after 1c is verified
3. **§3B/3C** operator approval of BRIEF archive decisions
4. **§4A** Stage 6 (Command Session) — unblocks §2b–2e
5. **§2b–2e** binary build + deploy — after Stage 6
6. **§2g** Yo-Yo Packer rebuild (Command Session)
7. **§2f** end-to-end flow verification — after Yo-Yo is live

---

## Definition of done

1. `local-slm.service` inactive; `free -h` shows ≤1 GB swap.
2. Doorman readyz includes `node_class`, `tier_a: false`, `tier_a_reason: force-broker-mode` (or `micro-node-class` if Phase 4 binary deployed and node-class probe fires correctly on this VM).
3. `service-content` running, `/healthz` returns 200.
4. Full flow via Yo-Yo verified (one round-trip inference through Doorman → graph context → Yo-Yo → response).
5. All BRIEF conflict resolutions approved and recorded; absorbed BRIEFs marked archived.
6. Stage 6 promoted; `origin/main` up to date.
