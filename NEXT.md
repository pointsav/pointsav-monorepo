# NEXT.md — pointsav-monorepo

> **Scope: this repo only.** Cross-repo and workspace-level open
> items live at `~/Foundry/NEXT.md`.
>
> Read at session start when a Root Claude opens in this repo. Update
> at session end when repo-scope open items change.

Last updated: 2026-05-16.

---

## Currently open

### VM stability — crash prevention [2026-05-16 task@claude-code]

Root causes identified and addressed after 2× daily crash pattern (GCP host maintenance + cgroup OOM).

- [x] **LadybugDB buffer pool blowup** — `SystemConfig::default()` allocated 12.8 GB (80% RAM). Fixed: explicit `buffer_pool_size` from env var `SERVICE_CONTENT_LBUG_BUFFER_POOL_MB` (default 64 MB). Deployed `7672e76f`. Dropin: `MemoryMax=3G`, pool=2048 MB.
- [x] **local-slm MemoryMax reverts to 3G on daemon-reload** — created `/etc/systemd/system/local-slm.service.d/memory.conf` with `MemoryMax=6G`. Verified `6442450944` bytes after reload.
- [x] **vm.swappiness=10** — set via `/etc/sysctl.d/99-foundry-inference.conf`. Prevents inference workload swap.
- [x] **Retry storm on circuit-open extract** — added `Retry-After: 300` header to `/v1/extract` when `yoyo-circuit-open`. Deployed `31397dad`.
- [x] **GCP host maintenance — MIGRATE confirmed** — `onHostMaintenance=MIGRATE`, `automaticRestart=True`, `preemptible=False`. VM already correctly configured. Crashes were OOM-only, not host maintenance.
- [ ] **journald cap** — create `/etc/systemd/journald.conf.d/foundry-cap.conf` with `SystemMaxUse=2G` and run `sudo systemctl restart systemd-journald`. (Minor risk factor; junk fill on `/var`.)
- [ ] **Delete unused 7B-Think weights** — `/var/lib/local-slm/weights/` has wrong 7B variant (4.5 GB). Recover disk space once 7B → OLMo 2 1B is confirmed stable.

### service-slm / service-content — Sprint 0a prerequisites [2026-05-14 task@claude-code]

**Sprint 0a SHIPPED** — `POST /v1/messages` live on workspace VM (`fdd1a223` + `7cd9ca61`).

- [x] **Add `graph_context_enabled: Option<bool>` to `ComputeRequest`** — done; shim sets `Some(false)` (`slm-core/src/lib.rs:116`, `http.rs:1308`)
- [x] **Decide opus → Tier C path** — Path A shipped (2026-05-16): `claude-opus-*` routes `tier_hint: External`, `tier_c_label: "editorial-refinement"` (`31397dad`). Requires `has_external=true` at runtime (Tier C env config). Currently returns 503 (unconfigured) which is correct failsafe.
- [x] **Reconcile apprenticeship flag drift** — `compute/systemd/slm-doorman.service:37` updated to `true` (2026-05-15)

**Sprint 0b (next):**
- [ ] **Real per-token SSE streaming** in `http.rs::anthropic_sse_body()` (~60 LOC). Currently buffers full response then emits 6 events at once.
- [ ] **On-demand Yo-Yo lazy-start** in `router.rs` — start Yo-Yo VM when Tier B request arrives and VM is stopped.
- [ ] **Wire `SLM_TIER_C_ANTHROPIC_*` env** for opus → Tier C passthrough (routing is wired in `31397dad`; ExternalTierClient needs API key + endpoint env vars set in `local-doorman.env`).

### service-content — Ring 2/Ring 3 decoupling [2026-05-14 task@claude-code]

Current `main.rs` is the **legacy watcher** that `service-content/ARCHITECTURE.md` designates
deprecated. Ring 2 ingest halts completely when Ring 3 (Doorman) is unavailable — the Community
Tier principle is aspirational, not real. See `.agent/plans/service-content-architecture-2026.md`.

- [x] **Sprint 1 — deterministic Source node write** — done (2026-05-15, `889bc993`). Source node written before Doorman call; graph grows regardless of Tier B reachability.
- [ ] **Persistent extraction queue** (replace per-boot retry)
  `processed_ledgers: Vec<String>` resets on restart. 114 deferred files retry every boot.
  Fix: disk-backed set (sidecar JSONL or SQLite) + Yo-Yo-up notification trigger.
- [x] **Validate `module_id`; reject `__` prefix** — done (2026-05-15, `889bc993`). Rejects `__`-prefixed overrides.
- [ ] **Wire `RelatedTo` edges in graph store**
  `graph.rs:66-72` declares `RelatedTo` table; it is never populated anywhere. Graph is
  node-only. Everything in ARCHITECTURE.md §8 about linked nodes is unmet.
- [x] **Fix `main.rs:293` unwrap** — done (2026-05-15, `889bc993`).
- [ ] **Move `/v1/draft/generate` to Doorman** (Ring violation — Ring 2 generating text via Ring 3).

### service-slm — audit ledger completeness [2026-05-14 task@claude-code]

- [x] **`ExtractionAuditEntry` missing fields** — done (2026-05-15, `889bc993`). `model`, `cost_usd`, `sanitised_outbound` added.
- [x] **Add `"graph-query"` to `AUDIT_CAPTURE_VALID_EVENT_TYPES`** — done (2026-05-15, `889bc993`).

### Leapfrog compound loop — close the flywheel [2026-05-14 task@claude-code]

The compound moat (apprenticeship → LoRA → sovereign model) requires these steps in order.
See `.agent/plans/leapfrog-2026.md` for full strategic analysis.

- [x] **1. Git post-commit hook** — done (2026-05-15). `service-slm/scripts/capture-edit.sh` (54 LOC). Reads `.git/foundry-brief-id`; POSTs diff to `/v1/shadow`. Install: `ln -sf ... .git/hooks/post-commit`. Agent session writes brief_id to file before committing; clears at session end.
- [ ] **2. Eval harness** — held-out eval set + regression test for Tier A and Tier B tasks.
  Must exist BEFORE first LoRA training run (no way to measure improvement otherwise).
- [x] **3. Corpus quality gate** — shipped (2026-05-16, `31c389b7`): MIN_BRIEF_BODY_CHARS=50, MIN_DIFF_CHARS=20, PII patterns (API keys, SSH private keys). 422 on rejection.
- [ ] **4. Ratify `conventions/permissible-model-substrate.md`** — BCSC posture, OLMo-only
  rule, upgrade procedure as policy. Excludes Qwen/DeepSeek/Yi/GLM (PRC-headquartered).
- [ ] **5. Tier A upgrade** — `OLMo-2-1124-7B-Instruct-Q4_K_M.gguf`, `MemoryMax=6G`.
  Current 1B cannot produce reliable flat-schema tool-call args (blocks haiku-tier shim).
  Requires weights download to `/var/lib/local-slm/weights/` + unit file update + redeploy.
- [ ] **6. First LoRA training run** — on Yo-Yo #1 after steps 1–3 complete.
- [ ] **7. mistralrs-server migration** — at LoRA milestone; enables hot-swap adapters at runtime.

### app-mediakit-knowledge — Phase 4 continuation

**CLOSED (2026-05-15).** Steps 4.1–4.8 all confirmed shipped in source:
`src/mcp.rs` (Step 4.6, `POST /mcp` default-off), `src/git_protocol.rs` (Step 4.7
smart-HTTP). Project-root `NEXT.md` already says "Phase 4 COMPLETE". CLAUDE.md and
project NEXT.md are authoritative.

Remaining open item: **Deploy** — rebuild release binary, restart
`local-knowledge-documentation.service` and `local-knowledge-projects.service`.
This requires operator presence on the workspace VM; no code work needed.

### Leapfrog 2030 Architecture & Multi-Yo-Yo Roadmap
- **Software layer complete** (180/180 tests as of 2026-05-15). See `service-slm/NEXT.md`.
- **Yo-Yo #1 VM live** — `yoyo-tier-b-1` in `europe-west4-a` (relocated from `us-central1-a` via Mode 2 stockout cascade; confirmed 2026-05-15). L4, image `slm-yoyo-20260507-061137`. Doorman wired; nginx TLS + bearer auth verified working.
- **Idle monitor fixed** (`890b3f6`) — was returning HTTP 411 (missing `Content-Length: 0`
  on GCP POST); fixed with `.body("")`. The SA (Editor role) can stop instances without
  additional IAM grant — step 2 below is no longer required.
- **VM currently TERMINATED** — manually stopped 2026-05-07; Instance Schedule will
  restart at 02:00 UTC nightly once weights are loaded.
- **Remaining operator steps:**
  1. Upload OLMo 3 32B-Think Q4 weights (~20 GB) to `/data/weights/olmo-3-32b-think-q4.gguf`
     on the Yo-Yo VM via `gcloud compute scp`. This is the only blocker for full
     nightly drain cycle. Once loaded, VM starts at 02:00 UTC, vLLM serves, drain
     worker routes briefs to Tier B, idle monitor stops VM after 30 min idle.
  2. ~~Grant `roles/compute.instanceAdmin.v1`~~ — not needed; Editor role sufficient.
  3. Run smoke test per `service-slm/docs/deploy/deploy-yoyo-tier-b.md` §8.
  4. Re-enable apprenticeship: set `SLM_APPRENTICESHIP_ENABLED=true` in `local-doorman.env`.
- Runbook: `service-slm/docs/deploy/deploy-yoyo-tier-b.md`.

### Layout hygiene — defect closures queued

Rule source: `.agent/rules/repo-layout.md` (introduced 2026-04-23).
Each item below is a separate commit via `tool-commit-as-next.sh`.

*(queue empty — Tier-2 project-root scripts closed 2026-04-23;
see Recently closed below and `cleanup-log.md`)*

### Awaiting cross-repo handoff

Entries lodged in `.agent/rules/handoffs-outbound.md`. Pattern is
passive — nothing moves until Master Claude or a Root Claude in
the destination repo picks up the entry and commits the add-side.
Source files remain in place here until the destination has
committed; only then does a follow-up Root Claude session commit
the source-remove.

- **`guide-operations.md` → `content-wiki-documentation`** — see
  outbox for destination path and rationale.
- **`USER_GUIDE_2026-03-30_V2.md` → `content-wiki-documentation`**
  (with `_V2` dropped in transit) — see outbox.

### Framework follow-ups

- **BIM project activations** — three of four BIM projects are still
  Reserved-folder. Follow the `app-console-bookkeeper` pilot pattern
  (framework §8): `app-console-bim`, `app-orchestration-bim`,
  `app-workplace-bim`, `service-bim` (the fourth, which triggered
  the taxonomy expansion).
- **`service-bookkeeper` forward reference** — the
  `app-console-bookkeeper` view reads "Awaiting service-bookkeeper
  sync" but that service is not in the registry. Decide: register
  as Reserved-folder, redirect to `service-fs/data/`, or correct
  the reference.
- **HTML-plugin vs Rust-crate `Type`-column refinement.**
  `app-console-*` and `app-network-*` projects contain both
  patterns; the registry's `Type` column does not distinguish.
  Surfaced during bookkeeper activation.
- **`BIM.zip` triage** — verified 2026-05-07: no zip artefact present on disk; item closed.

### Rename series

*(queue empty — all five rename-series items closed 2026-04-23;
see Recently closed below and `cleanup-log.md` Completed
migrations)*

### Structural defects

- **lbug 0.16.1 prebuilt packaging regression** [2026-05-13] — The prebuilt `liblbug.a`
  (both `compat` and `perf` Linux x86_64 variants) shipped without the companion
  `libfastpfor.a`, causing undefined `__fastpack*` symbols at link time. Workaround:
  build from source via `LBUG_SHARED=1`. Resolution options:
  (a) pin `lbug` to the last version with a self-contained static prebuilt (was working
  with lbug as of 2026-05-08 binary), or
  (b) add a `build.rs` env override to force shared-lib path by default.
  Upstream: report packaging regression to lbug crate maintainers.

- ~~**`start-yoyo.sh` Mode 2 Doorman env bug**~~ — **CLOSED (2026-05-15).** `update_doorman_env` already called at line 421 in Mode 2 path (confirmed in code). Both Mode 1 (line 388) and Mode 2 (line 421) call it unconditionally.

- **Workspace `Cargo.toml` unification** — per 2026-04-18 audit,
  workspace declares only 8 of ~70+ crates as members. Other crates
  are treated as standalone workspaces (hence 23 stray
  `Cargo.lock` files). Unifying would consolidate targets and
  resolve profile inheritance.
- **Large binaries** — tracked artefacts that should move to
  build-time fetch:
  - `app-mediakit-telemetry/assets/GeoLite2-City.mmdb` (63.5 MB)
    — **still tracked**. Next candidate for fetch-at-build
    treatment. Paths reclassified 2026-04-23.
  - `service-slm/router-trainer/engine/llamafile` (35 MB) —
    **untracked since 2026-04-23** via `git rm --cached` + new
    `.gitignore` pattern. Physical file remains at path for the
    Python workflow. History still contains the blob; shrinking
    the repo requires `git-filter-repo`, separate task.
  - `service-slm/router-trainer/engine/weights/qwen2.5-coder-1.5b.gguf`
    (15 MB) — already covered by existing `**/weights/*` +
    `*.gguf` ignore patterns. Same history-blob caveat applies.
  - ISO / IMG artefacts in `os-infrastructure/`,
    `os-network-admin/`, `os-totebox/` (tracking status TBD).

### Conformance and activations

*(queue empty — see Recently closed 2026-05-07 below)*

### Stashes parked in this repo

- `stash@{0}` — 2026-04-22 — "task21 WIP before worktree removal"
  (on `audit-layer-1-findings`; engineering work on `slm-memory-kv`
  crate, renames, untracked research doc). Restore with
  `git stash pop` when ready to resume.
- `stash@{1}` — pre-existing — "On service-extraction-v04: main:
  registry + BIM untracked — parked before task [21] resume".

## Recently closed (2026-05-07)

- **Reverse-Flow Substrate project registrations (Doctrine claim #52)** — six new
  Reserved-folder projects created with bilingual READMEs and registry rows in one
  commit each: `service-market`, `service-exchange`, `app-orchestration-market`,
  `app-orchestration-exchange`, `app-console-market`, `app-console-exchange`.
- **`app-orchestration-gis` registry drift** — directory created; Reserved-folder row
  added to registry. Deployed instance `gateway-orchestration-gis-1` was missing from
  the project registry.
- **`.gitignore` deduplication** — "Asymmetric Storage Protocol: Enforce Tier-1
  Quarantine" block was duplicated 4× (lines 4–18). Normalised to a single copy.
- **`service-extraction/CLAUDE.md`** — CLAUDE.md created; describes the 149-line
  filesystem-watching router accurately (replaces the stale v0.2/v0.4 framing in README).
- **`app-workplace-memo` activation** — CLAUDE.md + NEXT.md added; registry row
  promoted from Scaffold-coded → Active per framework §8.
- **`app-workplace-proforma/CLAUDE.md`** — local-only file committed to git; header
  updated to standard CLAUDE.md format.

## Recently closed (2026-04-23)

- Repo-layout rule introduced — `.agent/rules/repo-layout.md`
  codifies allowed files at the monorepo root and at each project
  directory root; names the sibling repos
  (`content-wiki-documentation`, `pointsav-design-system`, etc.)
  where cross-cutting content belongs. Anchor for the "Layout
  hygiene" queue above.
- `force_build.sh` relocated — root → `vendor-sel4-kernel/scripts/`.
  Zero runtime callers; script uses absolute paths so no content
  edits were needed. Repo root is now one file lighter against the
  new rule.
- `os-infrastructure/build_iso/forge_iso.sh` renamed to
  `compile_binary.sh` — resolves filename collision with the
  sibling ISO-assembly script at the project root. In-file header
  updated. Zero external callers. New open question logged in
  `cleanup-log.md`: the compile and assembly scripts are not wired
  together.
- `app-console-content/src/{pointsav-surveyor.sh,surveyor.py}`
  relocated to `app-console-content/scripts/`. Both files moved as
  100% renames. Shell wrapper is relative (`$(dirname "$0")`),
  Python script uses absolute paths — neither needed content
  edits. Throttle open-question row in `cleanup-log.md` updated
  with a code-reference pointer to the new path; the operator
  decision on `MAX_DAILY_VERIFICATIONS = 10` remains open.
- Handoff-outbound pattern introduced —
  `.agent/rules/handoffs-outbound.md` logs cross-repo file moves
  kept in place here until a Root Claude in the destination repo
  commits them. Two entries lodged (`guide-operations.md`,
  `USER_GUIDE_2026-03-30_V2.md`, both to
  `content-wiki-documentation`). Formalisation of the pattern in
  `~/Foundry/CLAUDE.md` §9 and §10 surfaced for Master Claude in
  `cleanup-log.md`.
- Tier-2 project-root scripts relocated — 18 files across 9
  projects moved to their respective `scripts/` subfolders in 9
  separate commits (`8f5cc48` through `faae141`). Every file
  registered as a 100% rename; no callers needed updating.
  Projects touched: `os-totebox`, `service-content`,
  `service-email`, `service-slm`, `tool-cognitive-forge`,
  `os-network-admin`, `vendor-phi3-mini`, `service-vpn`,
  `app-mediakit-telemetry`. Stray `tool-cognitive-forge/llama.log`
  surfaced as a separate housekeeping item.
- `service-parser/` removed — first rename-series closure.
  Directory contained only a README describing a superseded
  AI-routing framing; zero runtime references, never a workspace
  member, one commit in history. Nothing recyclable into
  `service-extraction` (which describes a different, deterministic
  Parser-Combinators approach). Rename-table row moved to
  Completed migrations; registry row removed (Defect count
  5 → 4, Total rows 100 → 99).
- `pointsav-pty-bridge` → `service-pty-bridge` — second
  rename-series closure. Directory renamed via `git mv` (4 files,
  all 100% renames); `Cargo.toml` `name` field updated in the
  same commit. Registry row moved from "Other / special" into
  the Service table; reclassified Defect → Scaffold-coded
  (Defect 4 → 3, Scaffold-coded 51 → 52). Zero external import
  references; not a workspace member; stray `Cargo.lock` left
  in place (resolves with workspace unification).
- Fifth (final) rename-series closure — Cognitive Forge term
  retired in one commit. `service-slm/cognitive-forge/` renamed
  to `service-slm/router/`; former top-level `tool-cognitive-forge/`
  moved to `service-slm/router-trainer/`. Rust runtime
  (`router/`) and Python distillation workflow
  (`router-trainer/`) now live together as producer/consumer.
  Cargo.toml `name` + `main.rs` usage string updated.
  `distill_knowledge.py` moved from non-canonical `src/` to
  `scripts/`. Three binary/log files untracked via `git rm
  --cached` + new `.gitignore` patterns (llamafile 35 MB,
  engine.log, llama.log) — physical files remain at new paths.
  Registry Scaffold-coded 54 → 53, Total 98 → 97. Closes the
  rename-series queue entirely (5 of 5) and the separate
  `llama.log` housekeeping item.
- `service-email-egress-{ews,imap}` wrappers flattened — fourth
  rename-series closure. Consolidation-to-`service-email-egress`
  plan reversed after sub-crate review: EWS and IMAP are two
  protocol adapters, not duplicates, and merging them would erase
  the architectural distinction. Instead, the redundant
  doubly-nested wrapper directories were flattened — 73 files
  promoted up one level. Registry reclassified both from
  Defect → Scaffold-coded; Defect count 2 → 0 (registry is now
  Defect-free). The 13 dir-name / Cargo-name mismatches from the
  2026-04-18 audit remain separate.
- `vendors-maxmind` reclassified to
  `app-mediakit-telemetry/assets/` — third rename-series closure.
  Data-only directory moved to the authoritative path already
  documented in the vendor's README; `.mmdb` (63.5 MB) + both
  READMEs travelled together; empty `vendors-maxmind/` removed.
  Open question "does it belong as a `vendor-*` crate at all?"
  closed (answer: no; non-workspace data directory).
  `repo-layout.md` extended to name `assets/` and `data/` as
  conventional subfolders. Registry Defect 3 → 2, Total rows
  99 → 98. In-transit edit to `USER_GUIDE_2026-03-30_V2.md`
  line 902 updates the path reference — travels with the pending
  cross-repo handoff. Separate `.mmdb` → build-time-fetch task
  remains open under Structural defects.

## Recently closed (2026-04-22)

- Audit cleanup — removed 2 `__MACOSX/` directories and 16 tracked
  `.DS_Store` / AppleDouble files from egress extraction-artefact
  scaffolding. `.DS_Store` added to `.gitignore`. Commit `0eeaeba`.
- Project registry bootstrap — 96-row inventory covering every
  top-level directory. Commit `fd7811f`.
- BIM-research project rows + cleanup-log bootstrap on `main` (drift
  closed) + taxonomy-expansion session entry. Commit `3cc8f4a`.
- `app-console-bookkeeper` activation pilot — Reserved-folder
  (mis-classified) → Active. Commit `27ad6d2`.

## Pointers

- Workspace-level open items: `~/Foundry/NEXT.md`
- Workspace changelog: `~/Foundry/CHANGELOG.md`
- Project registry: `.agent/rules/project-registry.md`
- Cleanup log: `.agent/rules/cleanup-log.md`
- Repo layout rule: `.agent/rules/repo-layout.md`
- Handoffs outbound: `.agent/rules/handoffs-outbound.md`
