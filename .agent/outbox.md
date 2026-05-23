---
mailbox: outbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
contamination-resolved: 2026-05-22 — project-knowledge content excised; manifest restored from git history; all three blocker messages actioned
---

# Outbox — project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: Phase 6 AUTO-TODO complete — Stage 6 + redeploy needed
created: 2026-05-23T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260523-phase6-complete
---

Phase 6 of AUTO-TODO shipped. 262/262 tests. 3 commits:

1. `21281703` — `BackendLifecycle` trait; `idle_monitor` quarantined behind it; `AppState` wired.
2. `a689ec1e` — GF-1: `AuditLedger` Clone + `write_audit()` → `spawn_blocking` fire-and-forget.
3. `28f666bf` — GF-2: `LocalTierClient` inference client `connect_timeout(5s)` + `timeout(180s)`.

Plus earlier Phase 1–5 commits from session 3–5. Local `main` is now 11+ commits ahead of `origin/main`.

**Command Session action required:**
1. Check inbox `command-20260520-stage6-rebase-required` — rebase still needed before promote.
2. `git rebase origin/main` from project-intelligence archive.
3. `~/Foundry/bin/promote.sh` Stage 6.
4. `bin/sync-local.sh --all`.
5. Rebuild + redeploy `slm-doorman-server` on workspace VM.
6. Update `local-doorman.service` env: add `SLM_LOCAL_MODEL=olmo-2-0425-1b-instruct` if not already set.

---
from: totebox@project-intelligence
to: totebox@project-editorial
re: E4 naming blockers resolved — 3 drafts ready for language pass
created: 2026-05-23T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260523-e4-naming-resolved
---

All three drafts from msg-id `project-editorial-20260521-e4-triage-naming-blockers`
are now corrected and ready for language pass:

1. `topic-elastic-compute-lora-training-pipeline.md` (+ `.es`) — Yo-Yo → Elastic
   Compute rename complete. Target: content-wiki-documentation.

2. `guide-elastic-compute-nightly-pipeline.md` — Yo-Yo → Elastic Compute rename
   complete. Target: woodfine-fleet-deployment.

3. `topic-service-slm-graph-store-migration.md` (+ `.es`) — was
   `topic-jennifer-datagraph-rebuild`. Personal name removed from filename, title,
   and notes_for_editor. `target_filename` updated. Cross-reference to
   `topic-yo-yo-lora-training-pipeline` corrected to `topic-elastic-compute-lora-training-pipeline`.
   Body text references to "jennifer" remain — these are body-content issues for
   your language pass to clean up (role-noun substitution throughout).

All three are in `clones/project-intelligence/.agent/drafts-outbound/` with
`state: draft-pending-language-pass`.

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: tonight's build session — full todo list (operator approved)
created: 2026-05-23T00:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260523-build-session-todo
---

Full todo list for tonight's Command build session. Operator-approved.
Source: BRIEF-vm-hardening-and-consolidation.md (committed b1b51c91 + updates).

**Prerequisites (Command scope, do first):**
- [ ] Stage 6 promote: `git rebase origin/main && bin/promote.sh && bin/sync-local.sh --all`
      (local main is ahead; 17+ commits including Phases 2–5)
- [ ] `systemctl daemon-reload` after any unit file changes

**Step 1 — Remove 7B model (immediate, no build needed):**
- [ ] Add `SLM_FORCE_BROKER_MODE=true` to `/etc/local-doorman/local-doorman.env`
- [ ] `sudo systemctl restart local-doorman.service`
- [ ] Verify: `curl http://127.0.0.1:9080/readyz` → `has_local: false`
- [ ] `sudo systemctl stop local-slm.service && sudo systemctl disable local-slm.service`
- [ ] Verify `free -h` swap drops (~6 GB freed)

**Step 2 — service-content fixes (code; ~150 LOC total):**
All in `service-content/src/main.rs` and `http.rs`:
- [ ] Ring 2/3 decoupling: write `Source` node before Doorman call (main.rs:198, ~30 LOC)
      — graph must grow even when Doorman is unavailable (DOCTRINE claim #54)
- [ ] Persist `processed_ledgers`: replace RAM Vec with disk-backed HashSet (main.rs:102)
      — prevents 114-file retry storm on every restart (was root cause of 2026-05-13 VM crash)
- [ ] Real `/healthz`: probe graph store, return 503 if not ready (http.rs:84)
- [ ] `module_id` validation: reject `__` prefix; validate `[a-z0-9-]{1,64}` (http.rs:99-108)
- [ ] Replace `unwrap()` on startup + write paths (main.rs:47,48,53,293)

**Step 3 — service-slm fixes (code; ~10 LOC):**
- [ ] Add `"graph-query"` and `"graph-mutation"` to `AUDIT_CAPTURE_VALID_EVENT_TYPES`
      (http.rs) — graph proxy handlers currently bypass their own audit validation
- [ ] Fix Yo-Yo env var update on VM start: `start-yoyo.sh` sed silently fails to update
      all three endpoint env vars on IP change (SLM_YOYO_ENDPOINT, _TRAINER_ENDPOINT,
      _GRAPH_ENDPOINT) — consider single `SLM_YOYO_IP` that Doorman interpolates at runtime

**Step 4 — Build + deploy (after Stage 6):**
- [ ] `cargo build --release -p slm-doorman-server`
      → `sudo cp target/release/slm-doorman-server /usr/local/bin/`
      → `sudo systemctl restart local-doorman.service`
      → verify Phase 4 readyz: `node_class`, `tier_a`, `tier_a_reason`, `ai_available` fields present
- [ ] `cargo build --release -p service-content` (requires liblbug.so — already at /usr/local/lib/)
      → `sudo systemctl stop local-content.service`
      → `sudo cp target/release/service-content /usr/local/bin/`
      → `sudo systemctl start local-content.service`
      → verify: `curl http://127.0.0.1:9081/healthz` → 200

**Step 5 — Yo-Yo Packer image rebuild:**
- [ ] Rebuild `slm-yoyo` Packer image so Phase 0 hardening (G3 dead-man's-switch,
      G17 sticky stops) takes effect. Infrastructure code is at
      `service-slm/compute/packer/`. Prerequisite: Stage 6 first.

**Step 6 — Binary ledger update:**
- [ ] After each binary deploy: verify `data/binary-ledger/<name>.jsonl` has fresh
      entry with sha256 matching installed binary (AGENT.md §10)

**Definition of done for tonight:**
1. `local-slm.service` inactive; swap ≤1 GB
2. `local-doorman` readyz shows `tier_a: false`
3. `local-content` running, `/healthz` returns 200 (not always-OK stub)
4. Both new binaries deployed with Phase 4/5 changes
5. Stage 6 promoted; `origin/main` up to date

Reference: BRIEF-vm-hardening-and-consolidation.md

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: lbug 0.16.1 packaging defect — request upstream contact
created: 2026-05-23T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260523-lbug-packaging-bug
---

During service-content build work (BRIEF-lbug-build-blocker.md), we confirmed
lbug 0.16.1 has a packaging defect: the prebuilt `liblbug.a` is missing companion
static archives (fastpfor, antlr4, parquet, thrift, snappy, etc.), making
`cargo build` with the static path fail with undefined symbols at link time.

The shared-library path (`liblbug.so.0.16.1`) works and is what is currently
deployed. However, any fresh machine (CI runner, new fleet node) that does not
have `liblbug.so` pre-installed cannot rebuild `service-content` from source.

**Request:** please send a bug report to lbug / LadybugDB upstream (crates.io
maintainer, or GitHub issue tracker for the LadybugDB project) requesting that
0.16.2+ either:
  (a) ships a self-contained fat static archive with all companion libs, OR
  (b) documents the `LBUG_SHARED=1` path as the supported build mode and
      provides a bootstrap script that installs `liblbug.so` before `cargo build`.

Workaround in place — no blocker to current operations. Informational upstream
contact only.

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: infrastructure/local-doorman/local-doorman.service — uncommitted workspace change (Phase 4)
created: 2026-05-22T00:00:00Z
priority: normal
status: actioned
msg-id: project-intelligence-20260522-doorman-service-commit
---

Phase 4 of BRIEF-flow-restructure modified `infrastructure/local-doorman/local-doorman.service`
(workspace scope — Command must commit this):

  - `Requires=local-slm.service` → `Wants=local-slm.service` (soft dependency)
  - Comment updated to reference DOCTRINE.md claims #49/#54

Change is already on disk (`git diff infrastructure/local-doorman/` shows it).
Please stage + commit from workspace root via `bin/commit-as-next.sh`.

Once committed and `systemctl daemon-reload` run, the Doorman can start on Micro
nodes without local-slm being present.

---
from: totebox@project-intelligence
to: command@claude-code
re: doctrine drift — architecture brief missed claims #49/#54; recommend cross-check step
created: 2026-05-22T17:00:00Z
priority: normal
status: actioned
msg-id: project-intelligence-20260522-doctrine-drift-architecture-briefs
---

Surfacing per BRIEF-flow-restructure.md §6 (outbox item queued in §8.A).

During the 13-agent investigation that produced BRIEF-flow-restructure.md, the
original flow-restructure brief concluded "interactive AI must route to a GPU"
based on a 7B benchmark on the workspace VM. Five rounds of review identified
three compounding errors and traced them to **four ratified conventions/claims
that were missed**:

1. DOCTRINE.md claim #49 — the Totebox fleet default is the $7/mo e2-micro, verbatim
2. DOCTRINE.md claim #54 — AI is value-add, not load-bearing; the deterministic
   substrate is a complete product without any AI tier
3. `conventions/four-tier-slm-substrate.md` — the Tier 0–3 ladder
4. `conventions/tier-zero-customer-side-sovereign-specialist.md` — Tier A (1B
   specialist) is a NUC-rung property, not the fleet default

**Recommendation:** add a "doctrine cross-check" step to the architecture-brief
process — before any architecture investigation begins, enumerate the relevant
ratified doctrine claims and conventions, and verify the premise against them.
This would have caught the drift immediately.

A NEXT.md item was added: doctrine conflict between claim #49 and
`tier-zero-customer-side-sovereign-specialist.md` §1 (the "2–4 GB working set"
language reads as though it applies to the $7 node, when it actually describes
the NUC rung). Recommend a one-sentence gloss in claim #49 to resolve the
ambiguity. Command's call on how/whether to amend.

No code blocker — the corrected architecture is committed in BRIEF-flow-restructure.md
and Phase 1/2 execution has started. Informational.

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: BLOCKER — .agent/outbox.md and manifest.md contaminated by Stage-6 rebase
created: 2026-05-22T17:00:00Z
priority: high
status: actioned
msg-id: project-intelligence-20260522-agent-contamination
---

The Stage-6 rebase 2026-05-22 pulled project-knowledge's `.agent/` content into
the project-intelligence working tree. Three files are affected:

- `.agent/outbox.md` — replaced with project-knowledge's outbox (all the
  pending messages in this file below the new header are project-knowledge's)
- `.agent/manifest.md` — replaced with project-knowledge's manifest
  (`cluster_name: project-knowledge`)
- `.agent/memory/` — partially or wholly replaced with project-knowledge content

**Required actions (Command):**

1. Restore correct project-intelligence `.agent/manifest.md` (the archive cluster
   manifest for the SLM/service-content engineering cluster, not project-knowledge)
2. Excise the project-knowledge messages that are contaminating this outbox
3. Relay the project-knowledge outbox messages to project-knowledge's actual outbox
   if they have not already been received there
4. Add a NEXT.md note or convention update so that Stage-6 rebase of the monorepo
   does not accidentally pull one cluster's `.agent/` over another's

`.agent/manifest.md` must NOT be edited by this Totebox session until Command
confirms the correct content is in place. Logged in NEXT.md.

— totebox@project-intelligence

