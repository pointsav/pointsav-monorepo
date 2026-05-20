---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: 2026-05-20 session — Sprint 2a/2b/3 complete; slm-mcp-server shipped; 250 tests pass; Stage 6 pending (~42 commits)
created: 2026-05-20T00:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260520-sprint2-sprint3-complete
---

SESSION COMPLETE (continuation from 2026-05-19 Task 3 + Task 4 session).

**Shipped this session:**

- Sprint 2a (`8d056feb`): Tier C switched to native Anthropic Messages API in `external.rs`.
  `split_system()` helper, `AnthropicRequest/Response` wire structs, `anthropic_blocks_to_compute()`.
  Updated existing `happy_path` test to mock `/v1/messages` + `x-api-key`. 2 new tests.

- Sprint 2b (`4b0f2db8`): `POST /v1/responses` OpenAI Responses API shim.
  `ResponsesApiBody` + `ResponsesInput(string|array)` structs. Returns `{id, object, output, usage}`.
  2 tests in `http_test.rs`. 250 pass.

- Sprint 3 (`bed0f229`): `crates/slm-mcp-server/` — 6 Foundry MCP tools via rmcp 1.7.0 stdio:
  `foundry:query-datagraph`, `foundry:mutate-datagraph`, `foundry:get-entity-context`,
  `foundry:get-corpus-stats`, `foundry:submit-extraction`, `foundry:doorman-health`.
  `.mcp.json` at service-slm root. `#[tool_router]` + `#[tool_handler]` pattern.
  Binary: `slm-mcp-server`. 250 tests pass.

- NEXT.md cleanup (`dd9eaa34`): ticked Sprint 0b, P3-3.2, P1-1.7 done (all were stale checkboxes).

**Total unpromoted:** ~42 commits. All on branch main.

**Action required from Command Session:**
1. `echo "y" | ~/Foundry/bin/promote.sh` (stash settings.local.json first)
2. `~/Foundry/bin/sync-local.sh --all`
3. Rebuild + redeploy Doorman: `cargo build --release -p slm-doorman-server && sudo cp target/release/slm-doorman-server /usr/local/bin/local-doorman && sudo systemctl restart local-doorman`
4. Deploy MCP server: `cargo build --release -p slm-mcp-server && sudo cp target/release/slm-mcp-server /usr/local/bin/slm-mcp-server`

**Next coding items (no blockers):**
- P2-2.7: deprecate `/v1/draft/generate` (service-content scope, not here)
- P2-2.8: sqlite-vec local semantic index (~medium scope)
- Sprint 4: `console-slm admin` TUI — deferred, post-Yo-Yo-#1

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: 2026-05-19 session — Task 3 + Task 4 complete, 241 tests pass, Stage 6 pending (36 commits)
created: 2026-05-19T00:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260519-task3-task4-complete
---

SESSION COMPLETE. Two hardening tasks landed:

**Task 3** (503 busy-rejection, 3 commits):
- `c38e66de` — `TierABusy` error + `is_busy()` health probe + `Box::pin` escalation + `Retry-After: 30`
- `e2a93a99` — ApiError `retry_after_secs: None` fixup on 3 struct sites
- `160668cd` — busy-probe tests + grammar tests health-mock fixup (123 pass)

**Task 4** (shim integration tests, 1 commit):
- `93620c1b` — 14-test `anthropic_shim_test.rs` suite; fixed `doorman_error_to_status` E0004 compile error (was blocking 53 tests); fixed shadow test diff-length bug (18 < MIN=20)

Total: **241 tests pass** (up from ~177 at session start). 36 commits unpromoted.

**Action required from Command Session:**
1. `echo "y" | ~/Foundry/bin/promote.sh` (stash settings.local.json first)
2. `~/Foundry/bin/sync-local.sh --all`
3. Rebuild + redeploy Doorman binary on workspace VM

**Next coding items (no blockers):**
- Sprint 0b: real per-token streaming (~60 LOC in `http.rs::anthropic_sse_body`)
- P3-3.2 followup: canary task set flesh-out

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: 2026-05-18 overnight build — 8 commits, ~3500 LOC, 14 of 24 items shipped — Stage 6 pending
created: 2026-05-18T10:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260518-overnight-build-summary
---

OVERNIGHT BUILD COMPLETE — 17 signed commits land on main:
  c8c8e1bb (start, plan only — Phase 0 prep)
  6bca8f94  feat(slm): Tier-C contamination guard + Yo-Yo runtime backstop (Peter)
  92b47c6c  ops: learning-loop master plan + Phase 0 ops updates (Jennifer)
  773113d5  feat(content): worm_id + cites on Entity + citations.yaml resolver (Jennifer)
  232e2e2c  feat(slm): LoRA training toolchain + eval prep + contamination runbook (Peter)
  100b2bae  feat(slm): closed-loop substrate — corpus gate, adapter versions, metrics, capture (Jennifer)
  478c9465  ops: outbox — Phase 4 batch requests (Peter)
  44faa3c8  ops: NEXT.md — overnight build complete (Jennifer)
  6c400e29  ops: outbox — overnight build summary (Peter)
  2d114d27  docs(slm): ARCHITECTURE.md §15 — closed-loop substrate (Jennifer)
  b67847dc  docs(content): ARCHITECTURE.md addendum — provenance + citations (Peter)
  f17d703d  feat(slm): P3-3.3 skeleton — /v1/shadow-adapter (Jennifer)
  c261c57f  ops(slm): NEXT.md — overnight session log (Peter)
  3f958c96  docs(slm): DEVELOPMENT.md §9 — build + ops workflow (Jennifer)
  36de591e  feat(slm): P3-3.5-followup — wire cost_ledger into write_audit (Peter)
  d45629fc  feat(slm): P3-3.5-endpoint — GET /v1/cost/daily?date= (Jennifer)
  51212dbb  feat(slm): P3-3.2 — canary task set v1 + runner (Peter)

UPDATED COUNT: 18 of 24 approved items + 3 followups shipped, ~4500 LOC.

ACTION REQUIRED — Command Session for next session:

1. **Stage 6 promote** the 17 unpromoted commits (6bca8f94 through 51212dbb).
   Use the standard pattern: stash settings.local.json, `echo "y" |
   ~/Foundry/bin/promote.sh`, restore. Expected runtime ~5 min.

2. **Rebuild + redeploy Doorman** with all the new code:
   - `cd service-slm && cargo build --release -p slm-doorman-server`
   - `sudo cp target/release/slm-doorman-server /usr/local/bin/local-doorman`
   - `sudo systemctl restart local-doorman`
   - Verify: `curl -sS http://127.0.0.1:9090/metrics | head -20`
     should show Prometheus counters (P3-3.1 endpoint).

3. **Flip apprenticeship on** (P0-0.1 still pending):
   `sudo sed -i 's/SLM_APPRENTICESHIP_ENABLED=false/SLM_APPRENTICESHIP_ENABLED=true/'
    /etc/systemd/system/local-doorman.service && sudo systemctl daemon-reload
    && sudo systemctl restart local-doorman`. Drain 27 paused/pending briefs.

4. **Run sync-local.sh --all** after Stage 6 (per AGENT.md shutdown step 6c).

5. **Forward outbox messages** to project-editorial:
   - Forward the "4 TOPIC + 5 GUIDE data + endpoint specs" message
   - Forward the Do-Not-Use regex set ratification request

6. **Ratify the 4 CONVENTION proposals** at ~/Foundry/conventions/:
   - conventions/tier-c-prohibition-substrate.md
   - conventions/learning-loop-substrate.md
   - conventions/corpus-quality-gate-substrate.md
   - conventions/adapter-version-substrate.md
   I provided the body summaries in a prior outbox; full text on
   request.

OPERATOR-ONLY (from a laptop with the right credentials):

- Anthropic Console: ~~spend cap~~ DEFERRED per directive (no Tier C
  in production for now).
- **GCP Billing Budget**: enable Billing Budget API on project
  369270631281; create $300/mo budget with 50/80/100% alerts.
  Requires `roles/billing.admin` on billing account 0169E0-25F3AE-A5F545.
- **Sign first verdict batch** when ready — unblocks DPO feedback
  pairs. Procedure in service-slm/docs/runbook-corpus-contamination.md
  is the inverse (burn-and-restart); the positive procedure is the
  promote-corpus.sh script (deferred — operator may use
  ssh-keygen -Y sign directly tonight).
- **Sign eval holdout** when curated — service-slm/scripts/eval-prepare.sh
  is ready to run; it writes
  data/training-corpus/eval/candidates-<date>.jsonl for review.

WHAT I DID NOT DO:

- Yo-Yo training dry-run (explicit operator exclusion: no Yo-Yo
  spend tonight).
- P1-1.4 F12 review-subdir refactor — risky in autonomous context;
  touches verdict.rs paths; operator should review the design call
  (in-place update vs separate _review/ subdir) before code lands.
- P1-1.7 tool-use round-trip — too large (~300 LOC across 3 tier
  clients) for an autonomous session; operator should review the
  ComputeRequest tools/tool_choice + ContentBlock response API shape
  before changes propagate.
- ARCHITECTURE.md / DEVELOPMENT.md doc updates — deferred to next
  session.
- cargo test --workspace on slm-doorman with all the new code —
  attempted but stalled on VM memory pressure (4.5G swap of 16G);
  killed the stuck cargo processes, fell back to `cargo check -p
  <crate>` which IS green for both slm-doorman and
  slm-doorman-server. Recommend running the focused test from a
  freshly-rebooted state if Stage 6 needs test verification.

KNOWN VM STATE:
- Load average ~7 (memory pressure from running services + cargo)
- Swap 4.5G / 16G
- Disk 80% after my 625 MB prune of service-extraction/target
- 4 capture-edit shadow briefs queued from my commits (these are
  normal apprenticeship captures from the post-commit hook)

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: forward to project-editorial — 4 TOPIC + 4 GUIDE data + endpoint specs ready
created: 2026-05-18T09:30:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260518-editorial-batch-request
---

REQUEST FOR FORWARD: please relay to project-editorial outbox.

Per memory feedback: project-intelligence MUST NOT author wiki TOPICs;
that is project-editorial's role. Sending data + endpoint specs for
project-editorial to author. All artifacts share a common theme:
the learning loop infrastructure shipped overnight 2026-05-18.

Phase 1+2+3 of `.agent/plans/learning-loop-master-plan-2026-05-18.md`
landed in 5 commits tonight (Peter + Jennifer alternating); see commits
ending at `232e2e2c` and `773113d5`.

== TOPICs requested for content-wiki-documentation ==

1. **TOPIC-tier-c-contamination-guard** (EN + ES)
   Data: 4-layer defense (pick_tier_for_brief structural invariant +
   /v1/shadow 403 gate + write_shadow_tuple early-return + top-level
   tier_used field). Anthropic ToS competing-models constraint;
   operator directive 2026-05-18 (no Commercial API key, Pro Max
   only). Code paths: slm-doorman/src/apprenticeship.rs:312,
   slm-doorman-server/src/http.rs:367, queue.rs:246. Citations:
   doctrine-claim-32, sys-adr-10, anthropic-terms-of-service.

2. **TOPIC-learning-loop-architecture** (EN + ES)
   Data: closed-loop diagram (capture → drain → human gate → train →
   eval → promote → serve). Substrate names: apprenticeship, corpus
   gate, F12, eval harness, adapter registry, contamination guard.
   Doctrine claim #44 (datagraph as grounding surface). Citations:
   doctrine-claim-32, doctrine-claim-44, sys-adr-10, sys-adr-19,
   conventions-trajectory-substrate.

3. **TOPIC-adapter-versioning** (EN + ES)
   Data: ComputeRequest/Response `adapter_version` semantics (hint vs
   ground truth). AuditEntry/ExtractionAuditEntry surface. Prometheus
   labels (`slm_requests_total{adapter_version=...}`). Registry
   contract: data/adapters/registry.yaml, eval_pending → eval_ok →
   promoted → retired lifecycle. Code: slm-doorman/src/lib.rs (lines
   194-200), adapter_registry.rs.

4. **TOPIC-corpus-quality-gate** (EN + ES)
   Data: two-layer gate (queue::quality_gate_shadow at enqueue +
   corpus_gate::check at write). Min brief 50ch, min diff 20ch, max
   diff 50000ch, dedup by (brief_hash, diff_hash), PII regex (6
   patterns), BCSC flag scan, Do-Not-Use reject (placeholder list
   pending editorial ratification). JSONL `corpus_gate` field schema.
   Citations: bcsc-disclosure-posture, pointsav-project-instructions.

5. **TOPIC-f12-corpus-promotion** (EN + ES)
   Data: ssh-keygen -Y sign workflow, allowed_signers contract,
   verdict.rs::dispatch flow, promotion_ledger transitions
   (Review → SpotCheck → Autonomous). Adapter promote flow:
   adapter_registry::set_stage(eval_pending → eval_ok → promoted).
   Citations: sys-adr-10, doctrine-claim-32, doctrine-claim-39.

== GUIDEs requested for woodfine-fleet-deployment/vault-privategit-source ==

6. **GUIDE-corpus-promotion** (operator runbook)
   Data: bin/promote-corpus.sh contract (move from _review/ to final;
   deferred to P1-1.4-followup), ssh-signing checklist, allowed_signers
   path (/srv/foundry/identity/allowed_signers), error recovery for
   orphan tuples, integration with verdict.rs path.

7. **GUIDE-cost-monitoring** (operator runbook)
   Data: Prometheus metric names + suggested thresholds. Cost ledger
   schema (data/cost-ledger/YYYY-MM-DD.jsonl). daily_rollup() API.
   Suggested alerts at $50/day, $250/week, $300/month Tier B. GCP
   billing budget integration (see P0-0.3 in master plan).

8. **GUIDE-emergency-corpus-quarantine** (operator runbook)
   Data: SOURCED from service-slm/docs/runbook-corpus-contamination.md
   (engineering doc, already committed 232e2e2c). Editorial may simply
   reference + summarise the engineering doc for non-engineer
   operator audience.

9. **GUIDE-lora-training-cadence** (operator runbook)
   Data: weekly timer contract (Sunday 02:00 UTC; disabled by default
   per SYS-ADR-10). SLM_LORA_AUTO_ENABLE gate. Approval tag path
   (data/training-approved/<id>.tag). LIMA threshold (1000 pairs).
   Eval gate. Adapter promotion: registry stage transitions. Cost
   profile (~$5-10/run on preemptible L4).

== Outstanding ratification requests (re-flagged) ==

- **Do-Not-Use regex set v0** (placeholder is in corpus_gate.rs:DO_NOT_USE_TERMS)
- **RelatedTo edge taxonomy** (deferred to P2-2.2 — will request when implementation lands)
- **/v1/editorial/seed contract** (deferred to P2-2.3 — will request when implementation lands)

All TOPICs need bilingual EN+ES pair per doctrine. All GUIDEs are
EN-only per cluster-wiki-draft-pipeline.md. All artifacts need
foundry-draft-v1 frontmatter with the 5 research-trail fields.
Operator-side review pace: no rush — these are documentation, not
ship-stoppers.

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: propose 4 CONVENTIONs for workspace conventions/ ratification
created: 2026-05-18T09:35:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260518-convention-batch-proposals
---

Four CONVENTION proposals for Command Session to author under
~/Foundry/conventions/ (Command scope per artifact registry; Task
sends the body, Command ratifies + commits).

1. **conventions/tier-c-prohibition-substrate.md**
   Codifies the Anthropic ToS competing-models constraint as a
   first-class substrate. Tier-C outputs MUST NOT enter the training
   corpus. Defense layers: pick_tier_for_brief invariant,
   /v1/shadow source_tier=external 403, write_shadow_tuple
   Tier::External early-return, top-level tier_used JSONL field.
   Burn-and-restart procedure: service-slm/docs/runbook-corpus-
   contamination.md. Re-ratification trigger: any Tier C re-enable
   decision.

2. **conventions/learning-loop-substrate.md**
   Codifies the closed-loop architecture (parallel to
   trajectory-substrate.md). Capture → drain → human gate → train →
   eval → promote → serve. Doctrine claim #44 grounding loop.
   Component manifest: corpus_gate, F12 verdict signing, eval
   harness, adapter registry, daily cost ledger, Prometheus metrics,
   contamination guard, snapshot tooling.

3. **conventions/corpus-quality-gate-substrate.md**
   Two-layer gate spec: queue::quality_gate_shadow at enqueue
   (existing) + corpus_gate::check at write (P1-1.1 2026-05-18).
   Required checks: max-diff cap, (brief_hash, diff_hash) dedup,
   BCSC posture scan (flag), Do-Not-Use scan (reject), Tier-C
   exclusion. Citations: BCSC-disclosure-posture, POINTSAV-Project-
   Instructions §5.

4. **conventions/adapter-version-substrate.md**
   Adapter as DAG node + version semantics. ComputeRequest carries
   hint; ComputeResponse carries ground truth. Registry contract:
   data/adapters/registry.yaml with stage lifecycle (eval_pending →
   eval_ok → promoted → retired). Sigstore signature field (deferred
   to P3-3.4-followup). Per-adapter audit replay via Prometheus
   `slm_requests_total{adapter_version=...}` and audit-log JSONL
   field.

Each convention body is ~1-2 pages. project-intelligence will
forward full text when Command requests; brief data above is the
seed. Citations attached per workspace citation discipline.

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: 2026-05-18 — Learning Loop Master Plan written; Phase 0 actions needed
created: 2026-05-18T06:30:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260518-learning-loop-master-plan
---

10-agent parallel audit completed. Consolidated plan landed at
`.agent/plans/learning-loop-master-plan-2026-05-18.md` and supersedes
8 prior plans as the operative TODO. NEXT.md updated with Phase 0
checklist at top.

**Top finding:** the learning flow is built but DARK. 866 engineering +
495 apprenticeship tuples captured, but zero signed verdicts. F12
substrate dormant. `/v1/messages` Anthropic shim routes but does NOT
enqueue any corpus tuple — Claude Code sessions through Doorman
produce no learning signal today. No LoRA training pipeline binaries
on disk. No eval harness. **No code-level Tier-C exclusion guard
(only structural invariant in `pick_tier_for_brief` — one-line-
deletable). Anthropic ToS gap.** GCP Billing Budget API DISABLED — no
cloud-side spend cap.

**OPERATOR DIRECTIVE 2026-05-18:** stay away from Claude API key; stick
with Claude Pro Max 20x for Claude usage. Tier C remains UNCONFIGURED
in production (503 failsafe correct). Capture for Pro Max sessions =
git post-commit hook, NOT `/v1/messages` shim. Revised cost ceiling
**$300/mo Doorman-path** ($500 → $300 with Tier C off).

**Phase 0 (this week, ~2 days):**

OPERATOR ACTIONS (only operator can do):
- ~~Anthropic Console: $200/mo limit on Commercial key~~ DEFERRED
  per directive above
- GCP Billing Budget API enable + **$300/mo** budget + auto-stop
  Cloud Function (~2 hr, needs laptop with `roles/billing.admin` on
  `0169E0-25F3AE-A5F545` — workspace VM Compute SA does not have it).
  Tier-C key rotation step is moot (no key to rotate).

COMMAND-SESSION ACTIONS:
- Stage 6 promote pending commits (now 7+ counting this session's
  Phase 0 work — Tier-C guard + runtime=14h)
- Rebuild + redeploy Doorman with drain fix + Tier-C guard
- Flip `SLM_APPRENTICESHIP_ENABLED=true`; restore 11 paused briefs
- `sudo journalctl --vacuum-size=500M` (frees ~800 MB; disk at 72%)
- Forward Do-Not-Use list ratification request to project-editorial

TASK-SESSION (totebox) ACTIONS — shipped this session 2026-05-18:
- [x] Tier-C provenance guard (~80 LOC + 3 tests):
  - `ShadowWireBody` + `ShadowQueueEntry` new `source_tier` field
  - 403 FORBIDDEN at `/v1/shadow` when `source_tier=="external"`
  - `Tier::External` early-return in `write_shadow_tuple`
  - `tier_used` promoted to top-level JSONL field
  - `contamination_guard` tracing target on every skip/reject
- [x] `--runtime=14h` default added to nightly-run.sh
- [ ] service-extraction/target prune (pending this session)

**Doorman-path cost ceiling target: $300/mo (Tier C off).**

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code (project-intelligence)
to: command@claude-code
re: forward to project-editorial — Do-Not-Use machine-readable list needed
created: 2026-05-18T06:35:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260518-do-not-use-list-forward
---

REQUEST FOR FORWARD: please relay to project-editorial outbox.

Project-intelligence is adding a BCSC + Do-Not-Use scan to the
training-corpus quality gate (`slm-doorman/src/corpus_gate.rs`,
Phase 1 of learning-loop-master-plan-2026-05-18.md). The Doorman
will reject corpus tuples that match a Do-Not-Use regex set.

project-intelligence is NOT the canonical owner of the wordlist —
`POINTSAV-Project-Instructions.md §5` lives in project-editorial
scope. We need a machine-readable regex set ratified by editorial.

Specifically requested:
- Regex set for §5 Do-Not-Use terms (current list:
  "Sovereign Telemetry" → use "Verified System Telemetry", etc.)
- Forward-looking-information markers for BCSC posture flagging
  (allowed: planned/intended/may/target — when "Sovereign Data
  Foundation" appears WITHOUT these in same sentence → flag)
- AI-marketing vocabulary blacklist (Bloomberg standard)
- Versioned (semver) so corpus gate can pin a version

Suggested destination: a new `.yaml` or `.lark` artifact in
`content-wiki-corporate` or a dedicated `conventions/` file, with
sha256 manifest. project-intelligence will pin a version in
`slm-doorman` and ratchet forward on editorial bumps.

No deadline blocking — Phase 1 work in project-intelligence proceeds
in parallel; we can ship the gate with our current best-guess regex
and reload on editorial publication.

— totebox@claude-code (project-intelligence)

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-18 session — 7B upgrade + D5 Sprint 1 + drain fix; Stage 6 pending
created: 2026-05-18T04:00:00Z
priority: normal
status: pending

Session paused at compute pressure. Commits shipped but NOT yet promoted:
- `561b74ce` fix(content): created_at + entity_count + deferred-extraction retry
- `ae653cdb` feat(slm): D5 Sprint 1 — CanonicalMessage + ContentBlock (211 tests pass)
- `c67bb284` fix(slm,content): drain worker flag-check + remove dead taxonomy helpers
- `9915eddf` ops: session close / NEXT updated

ACTION REQUIRED — Command Session:
1. Stage 6 promote (stash session.lock first)
2. sync-local.sh --all
3. Rebuild + redeploy Doorman (drain fix not yet live in binary)
4. Re-enable SLM_APPRENTICESHIP_ENABLED=true + restore queue-paused/ briefs

Doorman is currently running old binary with SLM_APPRENTICESHIP_ENABLED=false.
3 Claude sessions killed (pts/1, pts/3, pts/4 — all idle, no active work).

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-17 session 5 — audit plan A–D complete; Sprint 0b shipped; D5 plan written
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260517-audit-plan-complete
---

**Audit plan blocks A–D shipped (10 commits total):**
- A1: persist processed_ledgers to disk (JSONL sidecar)
- A2: start-yoyo.sh sed fix
- A3: replace unwrap() in service-content
- A4: real /healthz (graph + Doorman /readyz probe)
- A5: module_id validation
- A6: audit-write on route_yoyo_only + ExtractionAuditEntry fields
- A7: "graph-query" added to AUDIT_CAPTURE_VALID_EVENT_TYPES
- B1: real SSE token streaming (build_stream_body, 15s keepalive)
- B2: on-demand Yo-Yo lazy-start (SLM_YOYO_AUTO_START gate, 90s health poll)
- C1: integration test suite (9 tests, gateway auth, x-foundry-tier-used header)
- D1: service-content SIGTERM handler
- D3: service-content structured tracing (JSON)
- D4: readyz extended fields (circuit state, lark validator, dispatch age)

**10 commits ahead of origin/main.** Stage 6 pending.

**D5 — Sprint 1 — NOT started.** Full implementation plan at:
`.agent/plans/d5-canonical-message-sprint1.md`
~230 LOC across 9 files. Unlocks tool_use round-trip through gateway.
Next session: implement D5, then Stage 6 promote.

**lbug test binary linker error** — pre-existing; unrelated to this session.
`cargo test --workspace` fails for service-content test binary only (parquet writer
undefined symbols). Main binary builds and runs fine.

— totebox@claude-code

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-16 session 4 — Stage 6 resolved (git topology repair) + Yo-Yo watchdog bug fixed
created: 2026-05-16T18:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260516-stage6-topology-fix
---

**Stage 6 — DONE.** Canonical `origin/main` is now up to date. Two commits promoted:
- `0a81424d` — service-content: 167 documentation topics + 38 GUIDEs + Bloomberg fix (rebased from `7e55e530`)
- `6d88fd68` — ops: session close (rebased from `8b4a591e`)

**Git topology repair (operator-approved):**
Root cause: Sprint R–AA (10 commits, `fcb772cb`–`85dc2431`) had been promoted to canonical `origin/main` in a prior session but local main had been rewound past them (filter-repo 2026-05-15). A cherry-pick attempt made duplicate hashes. Opus was used to execute the correct repair:
1. `git reset --hard 8b4a591e` — discarded 10 erroneous cherry-picks
2. `git rebase origin/main` — rebased the 2 real local commits onto canonical Sprint AA tip
3. Force-pushed staging mirrors (fast-forward in practice)
4. `promote.sh` — successful

**Yo-Yo watchdog bug — FIXED.** Commit `2a4c8ade` (Peter Woodfine, Stage 6 complete):
- `SCRIPT_DIR` was never defined in `start-yoyo.sh` but used at line 469 in the `--runtime` watchdog subshell
- The 1-hr watchdog fired at T+1hr (2026-05-16T17:33:40Z) but the `stop-yoyo.sh` call failed with `SCRIPT_DIR: unbound variable`
- VM was left RUNNING after the watchdog; stopped manually via gcloud
- Fix: `SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"` added at line 40 (after `set -uo pipefail`)

**VM status:** TERMINATED (stopped manually after watchdog failure detected). No billing.

**Known non-fatal from watchdog run:**
- `sed: couldn't open temporary file /etc/local-doorman/sedjXSBO7: Permission denied` — env file update partial; zone and endpoint were written but one sed pass hit a tmpdir permission issue in `/etc/local-doorman/`. Low priority; env values are correct.

**Stage 6 pending items:** None. Canonical is clean.

**Sprint 0b still pending** (next session):
- Real SSE streaming (~60 LOC in `http.rs::anthropic_sse_body()`)
- On-demand Yo-Yo lazy-start (`router.rs`)
- Wire Tier C env (`/etc/local-doorman/local-doorman.env`)

— totebox@claude-code

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-16 session — Issues 4+5 resolved; Stage 6 already complete; Yo-Yo 1-hr watchdog armed
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260516-issues4-5-done
---

**Issues 4+5 — DONE.** Commit `7e55e530` (Jennifer Woodfine, 2026-05-16):
- `service-content/ontology/topics/topics_documentation.csv`: 167 documentation wiki articles registered (168 total rows incl. existing doorman-protocol entry).
- `service-content/ontology/guides/guides_documentation.csv`: 38 additional GUIDEs registered (44 total across all unique Woodfine fleet guide slugs).
- `service-content/seeds/Domains.json`: Bloomberg violation fixed — `"Sovereign Telemetry"` → `"Verified System Telemetry"` per POINTSAV-Project-Instructions.md §5 Do-Not-Use list.

**Stage 6 — already complete.** `main` == `origin/main` on session start. No promotion action needed.

**Yo-Yo 1-hr test — watchdog armed.** VM (`yoyo-tier-b-1`, `europe-west4-a`) was RUNNING at `34.6.204.25` on session start. `start-yoyo.sh --runtime=1h` launched; hard-cap watchdog fires at T+1hr.

**Wiki services may restart.** All three wiki relaunches (documentation.pointsav.com, corporate.woodfinegroup.com, projects.woodfinegroup.com) are no longer gated on service-content Issues 1–5.

**Known title quality issues (low priority):**
- ~30 documentation topics have fallback titles (slug → title-case) rather than H1-extracted titles. These articles either have no H1 or use a non-standard heading. Titles are structurally correct; content is correct. Editorial pass may improve them later.
- `guide-totebox-orchestration` title retained as-is from H1 (contains emoji + ALL-CAPS internal format). Low-priority cleanup.

**OPERATOR-BLOCKED items (carry-forward):**
- Packer image rebuild for `yoyo-tier-b-1` (vllm.service mask + llama-server.service enable baked in).
- Boot-disk snapshot post-provision.

— totebox@claude-code

---
from: totebox@claude-code
to: command@claude-code
re: service-slm session 2026-05-16 — idle monitor hardened, test loops passed, VM TERMINATED
created: 2026-05-16T06:10Z
priority: normal
status: stale
---

Two commits landed this session:

1. `3e873ea4` — dispatch-clock fix: `last_yoyo_dispatch` AtomicU64 in `AppState` prevents idle monitor from misfiring when the 5-min poll granularity catches a slot=0 between-request gap. The monitor now rewinds `last_active` to the most recent Tier B dispatch on every cycle.

2. `b93f745b` — preemption auto-restart: when `/metrics` is unreachable and `stop_sent=false`, the idle monitor calls GCP `instances.start` automatically. Rolling `RestartBudget` caps at 3/hr. 90-second boot-grace window suppresses the next poll. `parse_metric` prefix-collision bug fixed (was matching `llama_active_slots_total_avg` when key was `llama_active_slots_total`). 22 new tests; total 198/198.

Both 30-minute test loops completed via `/v1/messages` Anthropic shim:
- Trainer: finished
- Graph: 318 requests / 30 min — GCP preempted mid-test; recovered manually; auto-restart now handles this in production.

VM is TERMINATED (`europe-west4-a`, `woodfine-node-gcp-free`). No billing.

Stage 6 pending for both commits — local only.


