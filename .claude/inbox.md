---
mailbox: inbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Inbox — Task Claude on project-slm cluster

Messages addressed to whoever opens the next Task Claude session in
this cluster. Read top to bottom at session start. Act on items in
order; archive to `inbox-archive.md` after acting.

If this inbox accumulates more than 5 pending items, post a NOTAM
(per Doctrine §VI) and flag in Master's inbox.

---

## 2026-04-26 — from task-project-slm (model-tier handoff to Sonnet)

from: task-project-slm (Opus 4.7 session — AS-1..AS-7 author)
to: task-project-slm-next-session
re: model-tier-handoff — implementation + mechanical work queue while AS-5 + redeploy are blocked on Master
created: 2026-04-26T18:30:00Z
priority: low — operator-discretion; current session ended cleanly
recommended_model: claude-sonnet-4-6

### Why a tier change is recommended

AS-1 → AS-7 (the Apprenticeship Substrate routing endpoints) landed
end-to-end this session on Opus 4.7 — the design-question pass
benefited from deep-think on signature transport, ledger atomicity,
and the brief/attempt/verdict schema decisions. What remains in the
queue is **implementation tier**: small wires, layout-hygiene fixes,
cross-repo glue, no architectural surprises. Per
`conventions/model-tier-discipline.md`, run those on Sonnet 4.6 (or
Haiku for the truly mechanical items called out below).

The deep-think items (transient-queues triage, cognitive-forge
rename, scaffold remaining crates, compute/ directory) are still in
the queue but stay paused until you or the operator have a clearer
steer on the architectural questions — return on Opus when one of
those comes up.

### Available work — in suggested order

**Mechanical (Haiku 4.5 if you want to drop further; otherwise fine
on Sonnet):**

1. `git mv service-slm/cognitive-bridge.sh service-slm/scripts/`
   — single move; NEXT.md notes no caller audit needed; the file's
   own body uses positional args only. Update any references to the
   path inside the file if present (`pwd` checks, etc.).
2. Wire `cargo deny check licenses` into CI per `DEVELOPMENT.md`
   §2.2. `service-slm/deny.toml` is already in place; only the CI
   driver is missing. One workflow file edit.

**Implementation (Sonnet 4.6 — the bulk):**

3. **Server-side Tier C env-var wiring.** `slm-doorman-server/src/
   main.rs` still passes `external: None` to `DoormanConfig`. The
   `ExternalTierClient` is buildable from per-provider env vars
   (`SLM_TIER_C_<PROVIDER>_ENDPOINT`, `_API_KEY`,
   `_INPUT_PER_MTOK_USD`, `_OUTPUT_PER_MTOK_USD`) but the env-var
   parsing surface in `main.rs` is follow-up work — left over from
   the post-B4 outbox. Small, ~30 min. Leave Tier C unwired-in-
   binary until env vars are explicitly set (operator cost
   guardrail still holds).
4. **Close "MISSING CONNECTION PHYSICS"** in
   `service-slm/cognitive-bridge.sh` — replace the
   `RESPONSE="[UNVERIFIED STAGING OVERLAY]..."` placeholder with a
   real `POST $SLM_BIND_ADDR/v1/chat/completions` call through the
   Doorman. NEXT.md item; Doorman is the boundary, not the raw Tier
   A endpoint.
5. **Reconcile cognitive-forge → content-compiler wire format.**
   Writer emits `.md`; reader parses `.json`. Pick one (likely
   JSON, since `content-compiler` already speaks it) and land the
   contract end to end.
6. **Native-Rust SSH verification swap.** Replace the
   `SshKeygenVerifier` shell-out (`crates/slm-doorman/src/
   verdict.rs`) with the `ssh-key` crate. The `VerdictVerifier`
   trait abstraction makes this a one-file change plus dep add.
   Removes the openssh-client runtime dep. Add a unit test that
   verifies a known-good signature against a fixture allowed_signers
   to prove parity with the shellout.
7. **GUIDE-doorman-deployment.md draft** — *only* once Master has
   provisioned a catalog subfolder under
   `vendor/pointsav-fleet-deployment`. Cross-repo handoff per
   workspace `CLAUDE.md` §11. Hold until Master confirms.

**Deep-think (return on Opus 4.7 — do NOT do these on Sonnet):**

- Triage `service-slm/transient-queues/` (gitignore + relocate vs
  fixture)
- Rename `cognitive-forge/` subcrate (paired with sibling
  `tool-cognitive-forge` rename in monorepo `NEXT.md`)
- Scaffold remaining crates per `ARCHITECTURE.md` §6
- Build out `compute/` directory after reconciling §7 with
  `conventions/zero-container-runtime.md`
- Self-confidence threshold tuning — needs ledger evidence first
  (n≥10 verdicts on `version-bump-manifest`); re-evaluate after
  AS-5 + redeploy land

### Blocked / waiting (do NOT touch)

- **AS-5** — workspace tier (Master writes `bin/apprentice.sh` +
  `bin/capture-edit.py` shadow extension)
- **Workspace VM Doorman redeploy** — Doctrine §V VM sysadmin
  (Master rebuilds + restarts `local-doorman.service` from current
  cluster HEAD with `SLM_APPRENTICESHIP_ENABLED=true`)
- **B6 Doorman GCE lifecycle** — A3 viability spike pending
- **GUIDE-doorman-deployment.md** — catalog subfolder pending
  Master

### Save-state (clean pickup for the next session)

- Branch: `cluster/project-slm` (unchanged from end-of-session)
- Last commit: `8d35f86 session-end outbox post-AS-7 + inbox housekeeping`
- Working tree: clean (verified pre-exit; any modifications you
  see are from a fresh session start)
- What was just done (Opus session, AS-1..AS-7):
  - AS-1 Brief / Attempt / Verdict types in `slm-core` —
    `c97ebfd`
  - AS-2 `POST /v1/brief` endpoint — `225531e`
  - AS-3 `POST /v1/verdict` endpoint with signature verify +
    promotion ledger + DPO pair — `0148295`
  - AS-4 `POST /v1/shadow` endpoint with filesystem-level
    idempotency — `1c17fcc`
  - AS-6 + AS-7 manifest + ARCHITECTURE.md §11 + NEXT.md —
    `1864f9b`
  - Session-end outbox + inbox archive + NEXT.md correction —
    `8d35f86`
- What's next (≥30 min of work each):
  - Server-side Tier C env-var wiring (#3 above)
  - Close MISSING CONNECTION PHYSICS in cognitive-bridge.sh (#4)
  - Reconcile cognitive-forge ↔ content-compiler format (#5)
  - Native-Rust SSH verification swap (#6)
- Pending decisions / blockers:
  - GUIDE-doorman-deployment.md needs Master catalog subfolder
  - Self-confidence threshold tuning needs live ledger evidence
- Cross-cluster dependencies: none active (cross-cluster
  coordination note from project-knowledge already absorbed and
  archived)

### Workspace test baseline

`cargo test --workspace` 55/55; `cargo clippy --workspace
--all-targets -- -D warnings` clean; `cargo fmt --all -- --check`
clean. Any item in the queue should preserve this baseline.

### Push policy reminder

Per `clones/project-slm/.claude/manifest.md`: staging-tier only
(`origin-staging-j` / `origin-staging-p`). Never push to canonical
`origin`. All six commits this session held local — don't push
without explicit operator authorisation.

After acting on this message, append it to `inbox-archive.md` per
the mailbox protocol.

---
