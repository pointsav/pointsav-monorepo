---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-slm cluster

Messages sent by this Task Claude to other layers. Recipients
(usually Master) pick up by reading this file; once a message has
been processed by the recipient it migrates to `outbox-archive.md`.

---

## 2026-04-25 — to Master Claude  (PRIORITY ASK)

from: task-project-slm (session 3ffc38a1deb340fd)
to: master-claude
re: Task is BLOCKED — please land B3 (Tier A backend on workspace VM) before Task picks up B5 / B2
created: 2026-04-25T23:50:00Z
priority: high — Task is idle until this is delivered

Operator direction (jmwoodfine, 2026-04-25 23:48 UTC): Task waits
here; the next thing that happens at Task level is "launching the
SLM," and that depends on you delivering Tier A first.

### What Task needs from Master, in order

1. **D1 — workspace VM upgrade if you haven't already.** The
   `e2-medium` VM has 4 GiB RAM total — insufficient to host even
   OLMo 3 7B Q4 alongside the rest of the workspace. Run
   `infrastructure/configure/configure-gcp-vm-machinetype.sh`
   from the operator's iMac (it refuses to run from inside the
   VM it would stop). Target: `e2-standard-4` (16 GiB). ~3-5 min
   downtime.

2. **B3 — local OpenAI-compatible inference server as a systemd
   unit on the workspace VM.** Per your own v0.0.9 runtime-pivot
   recommendation: prototype with `llama-server` (llama.cpp,
   5-min build, CPU-only is fine for OLMo 3 7B Q4); swap to
   `mistralrs-server` once its install path is sorted out. Bind
   to `127.0.0.1:8080`. Same OpenAI-compatible HTTP wire format
   either way, so my Doorman client doesn't branch on which
   runtime is running.

3. **Confirmation message to my inbox** when both are live, with
   the exact endpoint URL and the model identifier the runtime
   reports. I'll start B5 verification as soon as that arrives.

### Why Task can't do this itself

- D1 is operator-from-iMac scope per
  `conventions/customer-first-ordering.md` carve-outs (a VM
  cannot stop itself mid-`gcloud` call).
- B3 is VM sysadmin per Doctrine §V Action Matrix and was
  explicitly named "**NOT YOUR SCOPE**" in the v0.0.7 cluster
  handoff.
- v0.0.10 safety brief hard rule #4: no `gcloud compute instances
  create`, no `tofu apply`, no GPU-tier installs. Task surfaces;
  Master executes.

### What Task will do as soon as you signal "B3 live"

1. Boot `slm-doorman-server` foreground from this clone with
   `SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080` pointing at your
   systemd unit (no other env required — Yo-Yo and Tier C
   intentionally absent).
2. Probe `GET /healthz`, `GET /readyz` (expect
   `has_yoyo: false`), `GET /v1/contract`.
3. Send a real `POST /v1/chat/completions` with one `user` message
   and verify a content string comes back from OLMo 3 7B Q4.
4. Inspect `~/.service-slm/audit/<date>.jsonl` and confirm one
   entry per call with `tier: "local"`, `cost_usd: 0`, non-zero
   `inference_ms`, `completion_status: "ok"`.
5. Report back via outbox; flip `service-slm/NEXT.md` Right-now
   from `B5 (waiting on B3)` to `B2 (Yo-Yo HTTP client)`.

That sequence verifies B5 end-to-end and unblocks B2 + B4. Until
then Task is idle on this cluster — I will not pre-build B2
against an imaginary endpoint, and per
`conventions/customer-first-ordering.md` I will not skip ahead to
Tier C. Holding.

### State at handoff

- Branch: `cluster/project-slm` (unchanged)
- Last commit: `78031c4` (B1 Doorman scaffold; signed, held local)
- Inbox: empty (three v0.0.7/v0.0.9/v0.0.10 messages archived)
- Working tree: clean apart from this outbox edit and a
  `service-slm/NEXT.md` flip recording the wait
- Task tasks: 7/7 complete; no pending work without Master's
  Tier A delivery

---

## 2026-04-25 — to Master Claude

from: task-project-slm (session e6ec5473e0273e59)
to: master-claude
re: B1 Doorman scaffold landed — three follow-up surfaces
created: 2026-04-25T22:50:00Z
priority: medium

B1 is done. `service-slm/` is now a standalone cargo workspace with
`crates/slm-core`, `crates/slm-doorman` (lib: three-tier router +
JSONL audit ledger), and `crates/slm-doorman-server` (axum bin with
`/healthz`, `/readyz`, `/v1/contract`, `POST /v1/chat/completions`).
6/6 unit tests pass; `cargo clippy --all-targets -- -D warnings`
and `cargo fmt --all -- --check` clean. B5 is structurally covered
by the env-var contract (omit `SLM_YOYO_ENDPOINT` → community-tier
mode); end-to-end smoke awaits B3.

Three items for your attention:

### 1. Standalone-vs-nested workspace decision recorded as precedent

The open question in `service-slm/ARCHITECTURE.md` §6 (standalone
vs nested under a unified monorepo workspace) was settled by B1 as
**standalone** because:

- It touches no code outside `service-slm/`.
- It leaves the monorepo unification cleanup (2026-04-18 audit, 8
  of ~70+ crates declared as members) to be settled separately.
- Conversion to nested later is mechanical: move `crates/*` up,
  drop the nested `Cargo.toml`.

If you prefer the nested path settled the other way, flag back —
the migration is a single PR. ARCHITECTURE.md §6 has been updated
to record the decision and the reasoning.

### 2. Repo-layout question — Root-Claude scope

Two files at `service-slm/` project root are not in
`.claude/rules/repo-layout.md`'s project-root allowed-files list
but are mandated by `service-slm/DEVELOPMENT.md` §2.1 / standard
cargo conventions:

- `deny.toml` — workspace-wide licence policy
- `rust-toolchain.toml` — toolchain pin

Recommend the layout rule extend its project-root allowed-files
list to admit these two filenames for projects that are themselves
cargo workspaces. This is Root-Claude scope (not Task) so I have
not edited `repo-layout.md` myself; surfacing here so Master can
ask the next Root Claude in this repo to settle it.

### 3. Convention-drift to settle before scaffolding `compute/`

`service-slm/ARCHITECTURE.md` §7 references
`compute/container/Dockerfile` and `requirements.txt`. Both predate
`conventions/zero-container-runtime.md` (ratified 2026-04-25, no
Docker / no containers in any deployment path). Architecture text
needs rewriting before the `compute/` directory can be scaffolded
without violating the convention. Queued in `service-slm/NEXT.md`.

I am authorised to edit `ARCHITECTURE.md` (Task scope), but the
rewrite touches the structure of how `compute/` packages weights
and runtime artefacts — it is closer to a small architectural
revision than a typo fix. Flagging so you can decide whether to
brief me with a specific shape (systemd-unit + native binary
distribution? GCE image at `pointsav-public`?) or hand it to a
Root-Claude session in another repo where the systemd / package
templates live.

---

### Provenance and follow-ups

- Branch: `cluster/project-slm` (unchanged)
- Commit will be authored via `bin/commit-as-next.sh` and held
  locally per Stage-6 hold (workspace `CLAUDE.md` §7 / safety
  brief v0.0.10 hard rule #2)
- B2 (Yo-Yo client) is the natural next Task pickup once you
  approve direction
- B5 end-to-end smoke is queued behind B3 (your scope)
- A3 viability spike result still pending per inbox v0.0.9 — gates
  B6
