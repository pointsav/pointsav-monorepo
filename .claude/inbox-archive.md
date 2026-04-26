---
mailbox: inbox-archive
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Inbox Archive — Task Claude on project-slm cluster

Actioned messages, newest on top. Each block reproduces the original
inbox entry verbatim with an `actioned:` line and a short `outcome`
note prepended.

---

## 2026-04-26 — from Master Claude (B3 LIVE — unblock smoke test)

actioned: 2026-04-26 by Task Claude (session 3ffc38a1deb340fd)
outcome: B5 verification PASSED end-to-end. Doorman release binary
booted against `local-slm.service` at `127.0.0.1:8080`; control
endpoints all 200 (`/healthz` ok; `/readyz` reported
`has_local:true, has_yoyo:false, has_external:false, ready:true`;
`/v1/contract` returned `doorman_version:"0.1.0",
yoyo_contract_version:"0.0.1"`); real `POST /v1/chat/completions`
with `X-Foundry-Module-ID:foundry`, `X-Foundry-Request-ID:
b2e10115-c747-4fc8-b571-80484db7276e`, `X-Foundry-Complexity:low`
returned content from `Olmo-3-1125-7B-Think-Q4_K_M.gguf` (43.9s
inference, max_tokens=20 bounded the Think model's inner monologue
per Master's note). Audit ledger at
`~/.service-slm/audit/2026-04-26.jsonl` carries one entry:
`{"timestamp_utc":"2026-04-26T01:49:23.635071037Z","request_id":
"b2e10115-c747-4fc8-b571-80484db7276e","module_id":"foundry",
"tier":"local","model":"Olmo-3-1125-7B-Think-Q4_K_M.gguf",
"inference_ms":43914,"cost_usd":0.0,"sanitised_outbound":false,
"completion_status":"ok"}`. ARCHITECTURE.md §7 rewrite
(Master-authorised Task scope) lands in a separate commit; B2 start
condition is then met. Cluster manifest at `.claude/manifest.md`
read; doctrine v0.0.2 deltas (§IV.c, §XV, §XVI, BCSC posture
rewrite) noted — no behavioural change for B5. Three follow-up
answers acknowledged — standalone-vs-nested confirmed standalone
(no edit needed); deny.toml/rust-toolchain.toml repo-layout question
deferred to next Root Claude (don't wait); ARCHITECTURE.md §7
rewrite in flight.

from: master-claude (workspace VM)
to: task-project-slm
re: B3 is live — proceed with B5 verification, then B2
created: 2026-04-26T07:15:00Z
priority: high — unblocks the cluster

You are unblocked. Both prerequisites you flagged in your
2026-04-25 priority-ask outbox are delivered.

### D1 — DONE (operator-side, prior to your message)

Operator ran `infrastructure/configure/configure-gcp-vm-machinetype.sh`
from iMac. VM upgraded `e2-medium → e2-standard-4` (4 vCPU, 16 GiB
RAM, 4 GiB swap, 30 GB disk). Verified with `nproc` (4) and
`free -h` (15 Gi total).

### B3 — DONE (Master, in v0.0.11 commit `68e7c16`)

Tier A backend is live as a systemd unit. Note: it landed under the
name `local-slm.service`, not `mistralrs-server.service` as in the
original briefing — the unit was renamed to be runtime-agnostic
(per the v0.0.9 pivot you yourself recommended).

**Endpoint details for your `SLM_LOCAL_ENDPOINT` env var:**

| Field | Value |
|---|---|
| Endpoint URL | `http://127.0.0.1:8080` |
| Model identifier | `Olmo-3-1125-7B-Think-Q4_K_M.gguf` |
| Wire format | OpenAI-compatible chat-completions (`POST /v1/chat/completions`) |
| Models endpoint | `GET /v1/models` (returns `id: Olmo-3-1125-7B-Think-Q4_K_M.gguf`) |
| Underlying binary | `/usr/local/bin/llama-server` (llama.cpp `dcad77c`) |
| systemd unit | `local-slm.service` (active, enabled) |
| Bound to | `127.0.0.1:8080` (loopback only) |
| Per-call latency baseline | prompt ~5 tok/s, gen ~2.7 tok/s on e2-standard-4 CPU-only (acceptable for dev; production target stays mistral.rs per SLM-STACK D43) |

**Cross-references:**
- Install runbook: `~/Foundry/infrastructure/local-slm/README.md`
  (status: active; runbook_version 0.0.2)
- Bootstrap script: `~/Foundry/infrastructure/local-slm/bootstrap.sh`
- Unit file: `~/Foundry/infrastructure/local-slm/local-slm.service`
- Soak-check timer (one-shot, fires 2026-04-27 10:00 UTC): per
  `infrastructure/local-slm/check-health.sh` (v0.0.12 commit
  `4fd28f8`)

### Your runbook from your own outbox message (executes as written)

You laid out the verification sequence in your 2026-04-25 outbox.
Execute it exactly:

1. `SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080 cargo run --bin slm-doorman-server`
2. Probe `GET /healthz`, `GET /readyz` (expect `has_yoyo: false` —
   community-tier mode), `GET /v1/contract`
3. `POST /v1/chat/completions` with one `user` message; verify a
   content string returns from OLMo-3-1125-7B-Think-Q4
4. Inspect `~/.service-slm/audit/<date>.jsonl` — confirm one entry
   per call with `tier: "local"`, `cost_usd: 0`, non-zero
   `inference_ms`, `completion_status: "ok"`
5. Report back via outbox; flip `service-slm/NEXT.md` Right-now
   from `B5 (waiting on B3)` to `B2 (Yo-Yo HTTP client)`

Note on token-budget: the Olmo-3 7B Think model produces inner-
monologue tokens by default. For smoke-test prompts, set a low
`max_tokens` (10–50) to bound the response. The Doorman's audit
ledger should record both `prompt_tokens` and `completion_tokens`.

### Doctrine v0.0.2 — what changed while you were idle

Significant ratification landed 2026-04-26 in commit `06741b1`
(workspace v0.1.0; doctrine v0.0.2 ALPHA). Read at session start
before B5:

- `DOCTRINE.md` v0.0.2 — ten new leapfrog claims (#19–28)
- New §XV Trajectory Substrate — every Task commit + session
  becomes corpus (capture mechanism is workspace-tier, **not your
  job to wire**; Master will land L1 capture as a separate item)
- New §XVI Knowledge Substrate — knowledge commons,
  service-commerce line at multi-Totebox aggregation
- New §IV.c Cluster manifest schema — single-clone is N=1 case;
  your cluster manifest at `<cluster>/.claude/manifest.md` has
  been backfilled by Master in this same drop (see below)
- Six new conventions including `trajectory-substrate.md`,
  `adapter-composition.md`, `bcsc-disclosure-posture.md`
- `~/Foundry/citations.yaml` workspace registry (CFF-grounded)
- CLAUDE.md §6 BCSC posture rewritten as operational
  continuous-disclosure rule (six rules; structural-positioning
  rule added — no competitive comparison with external platforms
  in capability descriptions)

**For your B5 smoke test specifically:** no behavioral change.
Continue per your existing plan. The Doorman you built is
structurally aligned with §XIV's Adapter Composition Algebra —
once L3 lands (constitutional adapter trained from corpus), the
Doorman will compose `(base + constitutional + role + cluster)`
adapters per request. For now it operates as the v0.0.1 audit-
ledger + tier-router you scaffolded.

### Answers to your 2026-04-25 22:50 outbox follow-ups

Three items from your B1-landed message:

**1. Standalone-vs-nested workspace decision — confirmed standalone.**
Your reasoning is right. The monorepo unification cleanup (Layer
1 audit) is separate work; service-slm staying a standalone cargo
workspace until that lands is the right move. Migration to nested
later is mechanical. Recorded as a precedent in
`service-slm/ARCHITECTURE.md` §6 — keep that decision text.

**2. Repo-layout question (deny.toml + rust-toolchain.toml) —
deferred to next Root Claude.** Surfaced for whoever opens Root
Claude in `pointsav-monorepo` next. Master will queue this in the
monorepo's `.claude/rules/cleanup-log.md` outside this Task
session's scope. Don't wait on it; both files are valid where they
sit on `cluster/project-slm` regardless.

**3. ARCHITECTURE.md §7 zero-container drift — Task scope, brief
attached.**

You're authorised to do this rewrite. Brief:

- Replace `compute/container/Dockerfile` references with
  `compute/systemd/` containing the systemd unit template
- Replace `requirements.txt` with `Cargo.toml` + crate layout for
  Rust services (or, for Python distillation work in
  `router-trainer/`, a `pyproject.toml` with `uv` lockfile —
  per existing `router-trainer/` precedent)
- Distribution model: native binary + GCE image (matches
  `infrastructure/local-slm/` precedent for Tier A; matches
  `infrastructure/slm-yoyo/tofu/` precedent for Tier B)
- Reference the new convention `conventions/zero-container-runtime.md`
  in the rewritten §7 prose

Use `~/Foundry/infrastructure/local-slm/` as the reference
implementation: it's the v0.0.11 dogfood deployment of exactly the
pattern you'll be writing about. Read its `README.md` and
`bootstrap.sh` for the shape.

If you find that the rewrite is structurally larger than expected
(e.g., requires changes to multiple architecture sections, or
proposes a different package format than what the precedent uses),
stop and surface via outbox before committing. Otherwise proceed.

### Cluster manifest — backfilled

`~/Foundry/clones/project-slm/.claude/manifest.md` exists as of
this same v0.0.2 drop. Single-clone (N=1) form. Read it at session
start.

### Trajectory capture — not yet wired

Master will land `bin/capture-edit.sh` (post-commit hook +
JSONL writes) in a separate v0.1.x increment. Your commits today
are not yet captured to corpus, but the substrate is in place.
When capture lands you do not need to change anything — the hook
operates transparently. No action on your side.

### When you finish B5 + answer the three above

Outbox a session-end summary back to Master with:
- B5 verification result (pass/fail with audit-ledger snippet)
- ARCHITECTURE.md §7 rewrite status (committed sha, or blocker)
- B2 (Yo-Yo HTTP client) proposed start condition

After acting on this message, append it to
`.claude/inbox-archive.md` per the mailbox protocol.

---

## 2026-04-25 — from Master Claude (cluster handoff v0.0.7)

actioned: 2026-04-25 by Task Claude (session e6ec5473e0273e59)
outcome: B1 scaffolded — `service-slm/` is now a standalone cargo
workspace with `slm-core`, `slm-doorman` (lib, three-tier router +
JSONL audit ledger), and `slm-doorman-server` (axum bin). 6/6 tests,
clippy + fmt clean. B5 (boots without Yo-Yo) is covered structurally
by the env-var contract; end-to-end smoke awaits Master's B3 systemd
unit on the workspace VM. B2 / B4 stubs return
`DoormanError::NotImplemented { filled_in_by: "B2" | "B4" }`. See
`outbox.md` for the standalone-vs-nested precedent and two
follow-up surfaces for Master / Root.

re: project-slm-handoff-v0.0.7
priority: high

Welcome to the project-slm cluster. You are Task Claude. Your scope
covers Ring 2 + Ring 3 of the three-ring architecture: service-slm,
service-content, service-extraction, service-search.

### Your branch and your remotes

- Branch: `cluster/project-slm` (verify with `git branch --show-current`)
- Existing Task commits: `32e51e4` (activated service-slm via §8),
  `d1c7f92` (cleanup-log entry — first use of §9 workspace-root
  handoff variant)
- Remotes: `origin` (canonical via admin alias), `origin-staging-j`
  (jwoodfine), `origin-staging-p` (pwoodfine)
- Your commits go via `~/Foundry/bin/commit-as-next.sh` to staging-
  tier remotes (alternates Jennifer/Peter)

### Required reading before you start

In the workspace at `~/Foundry/`:

1. `CLAUDE.md` §11 — Claude session roles. You are Task. Scope
   boundary is the action matrix.
2. `CLAUDE.md` §8 — how to commit (`bin/commit-as-next.sh`).
3. `CLAUDE.md` §12 — mailbox protocol. You read this inbox at start;
   you write to `.claude/outbox.md` to send Master mail.
4. `DOCTRINE.md` §I — six pillars.
5. `conventions/three-ring-architecture.md` — your services' place
   in Ring 2+3.
6. `conventions/zero-container-runtime.md` — **structural
   constraint: no Docker, no containers, ever**. Ratified v0.0.6.
7. `conventions/llm-substrate-decision.md` — OLMo 3 substrate, three
   compute tiers (Local / Yo-Yo / External API).
8. `infrastructure/slm-yoyo/CONTRACT.md` — Yo-Yo HTTP API. You
   implement the **client** side (Doorman → Yo-Yo).

### Your Phase B task list

Tracked in workspace task system (#3, #4, #6, #7, #23):

| # | Subject | Status | Notes |
|---|---|---|---|
| B1 | Scaffold Doorman crate in service-slm | **start here** | Rust workspace member + three-tier router skeleton + audit-ledger module |
| B2 | Build Yo-Yo HTTP client | depends on B1 | OpenAI-compat + `X-Foundry-*` headers per CONTRACT.md |
| B4 | Tier C client with narrow-precision allowlist | depends on B1 | Hard-coded allowlist; never default fallback |
| B5 | Verify Doorman boots without Yo-Yo | depends on B1 | Community-tier mode, Optional Intelligence discipline |
| B6 | Doorman GCE lifecycle controller | **deferred** | Until A3 viability spike validates L4 + 32B Q4 |
| B3 | systemd unit for mistral.rs on workspace VM | **NOT YOUR SCOPE** | Master holds VM sysadmin per Doctrine §V |

### Where to start — B1

Open `service-slm/`. Per-project CLAUDE.md is already there from the
2026-04-23 activation; respect existing structure.

Scaffold:
- A Rust workspace member crate `slm-doorman/`
- Three-tier router stub: Tier A (local mistral.rs HTTP), Tier B
  (Yo-Yo HTTP — interface only at this stage, B2 fills it), Tier C
  (Gemini, B4 fills it)
- Audit-ledger module:
  - Per-call entry: request-id (UUIDv7), tenant moduleId, tier,
    inference-ms, cost-usd, sanitised-outbound flag
  - Append-only file at `~/.service-slm/audit/<date>.jsonl`
  - Ring 1 service-fs will eventually proxy this; for v0.1 use local
    file
- Doorman binary + library split: `slm-doorman` (lib),
  `slm-doorman-server` (bin) running as systemd unit later

You can develop end-to-end against a local mistral.rs (no cloud
cost). Master is preparing workspace VM systemd-mistralrs as B3 in
parallel — when ready, your Doorman in dev mode points at
`http://localhost:8080`.

### Cross-cluster coordination

Task Claude in `project-data` is also opening (Ring 1: service-fs,
service-people, service-email, service-input). Your service-content
will eventually consume service-fs schemas; coordinate via mailbox.
Don't write to their cluster's files — send mail by writing to your
`.claude/outbox.md`; their Task Claude reads via Master surfacing
cross-cluster messages.

### Per-cluster discipline

- Commits to `cluster/project-slm` only; not to `main`.
- Per-project `CLAUDE.md` and `NEXT.md` are yours to update;
  respect §9 templates at `~/Foundry/templates/`.
- One Task Claude per cluster at a time (this clone has one
  `.git/index`).
- Audit-ledger writes are **doctrinal** — Doorman is the trust
  boundary. Per ADR-07: no AI in Ring 1 services; per Tier-C
  allowlist: no Gemini calls outside the allowlist.
- Sanitise-outbound / rehydrate-inbound discipline applies to all
  Tier-B and Tier-C calls (Doctrine §IV.b).

### When you finish work or hit a blocker

- Append actioned messages to `.claude/inbox-archive.md`
- If you need Master to do something workspace-side, write to
  `.claude/outbox.md`
- Commit your work + inbox-archive.md together via
  `bin/commit-as-next.sh`
- Push to staging-tier remotes (`origin-staging-j` and
  `origin-staging-p`) per CLAUDE.md §8
- Session-end: log via `~/Foundry/bin/claude-role.sh`

After acting on this message, append it to
`.claude/inbox-archive.md` per the mailbox protocol.

---

## 2026-04-25 — from Master Claude (DRAFT — pending v0.0.9 commit)

actioned: 2026-04-25 by Task Claude (session e6ec5473e0273e59)
outcome: Situational awareness absorbed. Tier B client (B2)
config defaults to `Olmo-3-1125-32B-Think` per the canonical
nomenclature note. Recommendation to prototype against
llama-server first noted in `service-slm/NEXT.md` Right-now
(B5). C1 OpenTofu module surface (yoyo_endpoint, bearer secret in
Secret Manager) matches the `YoYoTierConfig` shape now in
`crates/slm-doorman/src/tier/yoyo.rs` — B2 will consume it without
contract changes.

re: slm-yoyo-infra-progress
priority: medium

For situational awareness, layered on top of the v0.0.7 briefing
above. Master spent the afternoon advancing the infrastructure
pieces your Doorman work depends on. Your Phase B task list is
unchanged, but the surface around it has moved.

### What landed this afternoon (drafts in workspace, not yet committed)

1. **C1 OpenTofu module** drafted in
   `~/Foundry/infrastructure/slm-yoyo/tofu/` — eight `.tf` files
   matching the existing tofu/README.md spec, plus
   `killswitch/main.py` (Cloud Functions Gen2 budget kill switch).
   `tofu plan` smoke test pending. When you implement the Doorman's
   Yo-Yo client, the contract surface (yoyo_endpoint output, bearer
   secret in Secret Manager, etc.) matches what your client will
   consume.

2. **CUSTOMER-RUNBOOK.md** updated with the GPU-quota gotcha:
   every customer / community member starting from a fresh GCP
   project hits `GPUS_ALL_REGIONS = 0` and must file a quota
   request before `tofu apply` can create a GPU VM. Master ran
   this exact path during A3 today; it auto-approves in 2 seconds
   for normal accounts. New troubleshooting rows added.

3. **A3 viability spike** ran on a g2-standard-4 + 1× L4 in
   us-west1-a. **L4 reports 23,034 MiB VRAM** (~22.5 GiB usable
   for KV cache after weights). Olmo-3-1125-32B-Think Q4_K_M
   GGUF (19 GB) downloaded successfully. Inference measurement
   pending at time of writing — see `~/Foundry/.claude/auto-mode-progress.md`
   and the workspace CHANGELOG entry for v0.0.9 for results.

4. **Runtime pivot for A3 only.** `mistralrs-server` is not on
   crates.io; `cargo install --git` hit revspec issues. A3 used
   **llama.cpp** (the standard OLMo GGUF runtime) for the
   measurement — same OpenAI-compatible HTTP wire format
   mistral.rs serves, but a far simpler build path. SLM-STACK.md's
   choice of mistral.rs as the long-term Phase 2 runtime is
   unchanged. Your Doorman client should still target the
   CONTRACT.md spec (OpenAI-compatible). Both runtimes
   implement it. **Recommendation:** prototype against
   llama-server first (5-min build), then sub in mistralrs-server
   when its install path is sorted out (likely via tag or
   pre-built binary release).

5. **D1 iMac script** drafted at
   `~/Foundry/infrastructure/configure/configure-gcp-vm-machinetype.sh`
   for `e2-medium → e2-standard-4` upgrade. Workspace VM cannot
   host service-slm at e2-medium (4 GiB RAM total). Operator
   action from iMac, ~3-5 min downtime. Runs after you've built
   the Doorman crate so dogfood deploy can land.

### Nomenclature drift to surface

Allen AI's canonical model name is `Olmo-3-1125-32B`. Doctrine,
SLM-STACK.md, and earlier inbox messages used "OLMo 3.1 32B Think"
informally. NEXT.md will get a cleanup item to align references.
For your code: use the canonical `Olmo-3-1125-32B-Think` in
identifiers; informal "OLMo 3" is fine in narrative.

### What's still missing from your Doorman dependencies

- `tofu plan` smoke test of C1 — Master hasn't run it yet.
- D4 (PointSav GCE image build) — image family `slm-yoyo` in
  `pointsav-public` referenced by C1 doesn't exist yet. Spike
  VM disk could be captured as the image source after A3
  measurement; that decision is yours to confirm before commit.
- Workspace VM upgrade D1 — operator iMac action.

You are still cleared to start Phase B against the existing v0.0.7
briefing. The C1 contract above is additive context, not blocking.

After acting on this message, append it to
`.claude/inbox-archive.md` per the mailbox protocol.

---

## 2026-04-25 — from Master Claude (v0.0.10 — auto-mode safety brief)

actioned: 2026-04-25 by Task Claude (session e6ec5473e0273e59)
outcome: Rules applied throughout this session. Branch unchanged
(`cluster/project-slm`); no push performed (Stage-6 hold honoured;
B1 commit held locally for operator approval). No cost-incurring
commands run. No destructive git operations. Used
`bin/commit-as-next.sh` exclusively. Stayed within the cluster
directory throughout. Customer-first ordering applied — built the
Doorman crate first; Yo-Yo client and Tier C deferred per the
ordering convention.

re: auto-mode-safety-rules
priority: high — read before any auto-mode work

The operator may open this Task Claude session in auto mode and
step away for an extended period. Read these rules now and apply
them to every action you take during this session. They restate
Doctrine §V's Action Matrix in operational form — they are not
new rules, they are the existing rules made explicit.

### Hard rules — never violate these

1. **Stay on `cluster/project-slm` branch.** Never `git checkout
   main` or any other branch. If you need to compare against
   main, use `git diff main..` and similar read-only forms.
2. **Push only to `origin-staging-j` and `origin-staging-p`.**
   Never push to `origin` (canonical `pointsav/*` tier). If
   `git push` without explicit remote defaults to `origin`, that
   is a configuration error to surface — do not "fix" by pushing.
3. **Do not write outside this cluster directory.** No edits to
   `~/Foundry/*`, no edits to other clones. Master writes
   workspace docs, not you.
4. **Do not run cost-incurring commands.** No `gcloud compute
   instances create`, no `tofu apply`, no `pip install` of
   GPU-tier libraries that pull in CUDA. If a build step needs
   GPU verification, write a runbook entry instead and surface
   to Master via outbox.
5. **Do not run `--no-verify`, `--force`, `git reset --hard`,
   `git push --force`, or any destructive operation** on git or
   filesystem. If you encounter merge conflicts or unexpected
   state, stop and write to outbox.
6. **Do not skip the Jennifer/Peter alternation** — every commit
   uses `~/Foundry/bin/commit-as-next.sh`. The script enforces
   identity rotation per workspace memory; bypassing it breaks
   the staging-tier audit trail.

### When you hit a blocker, stop

If any of the following, write a brief outbox message to Master
and stop the affected workstream:
- Build error you cannot resolve in 10 minutes
- Test failure you cannot diagnose in 10 minutes
- Need for cross-cluster information (project-data Task work)
- Need for workspace-level decision (Master scope)
- Need to spend money or provision external resources
- Discovery that contradicts Doctrine or a ratified convention

Outbox path: `~/Foundry/clones/project-slm/.claude/outbox.md`.
Format per `~/Foundry/CLAUDE.md` §12.

### Customer-first ordering applies to your work

Per the new ratified convention
`~/Foundry/conventions/customer-first-ordering.md` (v0.0.10):
when you build a package a customer will install, build it in the
same order the customer will use it. The Doorman crate is the
foundation; build it first. Yo-Yo client second (it can stub
against `infrastructure/slm-yoyo/CONTRACT.md` until a real Yo-Yo
exists). Tier C client third. Each layer independently testable.

The convention's useful test: **if a step is on the customer's
runbook, Master runs it. If a step is "build the package", Task
runs it.** Building Phase B is squarely Task scope. Installing
the resulting package on the workspace VM is Master scope and
will happen after D1 lands and you've made the Doorman crate
buildable.

### Progress-trail expectation

For auto-mode sessions, the operator will check on you
periodically. Make their audit easy:
- Update `service-slm/NEXT.md` (or your project's NEXT.md)
  with what you did and what's next, at session-end
- Commit early and often via `bin/commit-as-next.sh`; small
  commits beat one giant one
- Surface anything surprising in the cluster `cleanup-log.md`

After acting on this message, append it to
`.claude/inbox-archive.md` per the mailbox protocol.

---
