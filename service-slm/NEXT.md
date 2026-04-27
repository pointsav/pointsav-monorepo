# NEXT.md — service-slm

> Last updated: 2026-04-27
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- **AS-1 → AS-4 landed** — Apprenticeship Substrate routing
  endpoints live in `slm-doorman` + `slm-doorman-server`,
  mock-tested. See "Recently done" section for per-stage
  detail. End-to-end verification waits on Master shipping
  AS-5 (helper scripts) plus the systemd unit redeployment so
  `SLM_APPRENTICESHIP_ENABLED=true` lands on the workspace VM.
- **WAITING — workspace VM Doorman redeploy.** Master's
  2026-04-26T14:00 cross-cluster inbox note recorded that the
  systemd unit was delivered as v0.1.13: `local-doorman.service`
  serves at `http://127.0.0.1:9080` from B2-era commit
  `2e317ab` (community-tier mode). After AS-1..AS-4 a rebuild
  from current `cluster/project-slm` HEAD plus
  `SLM_APPRENTICESHIP_ENABLED=true` in the unit Environment=
  block is required before the Doorman serves apprenticeship
  traffic. Workspace-tier scope (Doctrine §V VM sysadmin).
- **WAITING — Master holds AS-5** — `bin/apprentice.sh` (round-
  trip helper) and `bin/capture-edit.py` extension (post-commit
  shadow firing). Per Master's brief: "Don't write these
  yourself." Once AS-5 lands, every cluster Task Claude on the
  VM exercises the apprentice on every code-shaped commit.
- **GUIDE-doorman-deployment.md (Customer-tier draft)** —
  Master's manifest update names this as Task work in the
  customer-tier "leg-pending" item. Drafts under
  `customer/woodfine-fleet-deployment/<deployment-name>/`.
  Cross-repo handoff per workspace `CLAUDE.md` §11 — needs
  outbox to Master to land in `vendor/pointsav-fleet-deployment`
  catalog first; Task here drafts the content per the §9
  workspace-root variant of the §11 outbox pattern. Hold until
  destination catalog subfolder is provisioned.

## Earlier-stage items

- **(historical — referenced by B4 work below)** Fill
  `crates/slm-doorman/src/tier/yoyo.rs` per
  `infrastructure/slm-yoyo/CONTRACT.md`. Required: bearer-token
  acquisition (GCP Workload Identity for `*.run.app`; provider API
  keys from Secret Manager for RunPod / Modal; customer mTLS /
  shared secret for on-prem), POST `/v1/chat/completions` with the
  four required `X-Foundry-*` headers, retry-on-503 honouring
  `Retry-After`, auth-refresh on 401/403, MAJOR mismatch on 410.
  Wire format already laid out in `tier/yoyo.rs::YoYoTierConfig`;
  the `complete()` body is the only stub left to fill. Holds
  until Tier A is verified working — per
  `conventions/customer-first-ordering.md`, build in the order the
  customer will install.

## Queue

- **AS-2 grammar artefact** — author the `llguidance` Lark
  grammar at `vendor/pointsav-monorepo/service-content/schemas/banned-vocab.lark`
  (top-level rule `response`; validate with Python `lark` before
  shipping; JSON-Schema sibling + usage `.md` if needed). Per
  Master's 2026-04-27 v0.1.26 ack of the AS-2 library decision:
  3-4 week implementation timeline, develops independently of
  project-language Phase 1B. Surface to Master via outbox only
  if anything changes that would affect the contract.
- **ARCH/DEVELOPMENT.md zero-container drift FOURTH-pass —
  Master sign-off needed.** Third-pass scope (§5.10 SkyPilot
  row + §2 Bootstrap items 3+4) was cleared by commit
  `8c3212e` (2026-04-26); this NEXT.md was stale. Three new
  drift sites surfaced 2026-04-27 the third-pass missed:
  - ARCHITECTURE.md §3 line 132: "External calls (Cloud Run,
    Mooncake sidecar, Claude API, LadybugDB ...)" — generic
    mention; suggest drop "Cloud Run" or replace with "GCE
    instances".
  - ARCHITECTURE.md §5.2 line 197 — `hyper` crate role:
    "HTTP client (Cloud Run, Claude API, LMCache master)";
    suggest replace "Cloud Run" with "Yo-Yo GCE endpoints".
  - DEVELOPMENT.md §4 Phase 2 step 5: "Port the Cloud Run
    driver (`crates/slm-compute`, `crates/slm-inference-remote`)";
    suggest "Port the GCE compute driver".
  Surface to Master in next outbox; do not act without
  authorisation per the third-pass pattern.
- **Workspace-root handoff in flight.** The 2026-04-23 activation
  commit was the Task-scope half of the `SLM-STACK.md` /
  `YOYO-COMPUTE.md` rehoming (workspace `CLAUDE.md` §9 variant).
  The Root-scope half — drafting
  `content-wiki-documentation/topic-service-slm.md` and
  `topic-yoyo-compute.md` — remains open in that repo. Master
  deletes the workspace-root originals only after both halves land.
- **B6 — Doorman GCE lifecycle controller.** Deferred until A3
  viability spike validates L4 + 32B Q4 (per inbox v0.0.9: A3
  measurement still pending at handoff).
- Rename the `cognitive-forge/` subcrate — inherits the Do-Not-Use
  "Forge" concern. Pair with the sibling `tool-cognitive-forge`
  rename queued in the monorepo `NEXT.md` rename series so one
  decision covers both. Until renamed, the subcrate stays in the
  workspace `exclude` list and is built in isolation.
- Scaffold remaining crates per `ARCHITECTURE.md` §6:
  `slm-ledger` (split from `slm-doorman::ledger` once a SQLite
  index sits alongside the JSONL append log), `slm-compute`,
  `slm-memory-kv` (Phase 2), `slm-memory-adapters` (Phase 3),
  `slm-inference-local`, `slm-inference-remote`, `slm-api`,
  `slm-cli`. Each waits for a real consumer before scaffolding.
- Build out `compute/` directory per Ring 1 spec in
  `ARCHITECTURE.md` — `manifest.yaml`, `weights/registry.yaml`,
  `keys/secret-refs.yaml`, plus the OpenTofu module shape per
  `conventions/zero-container-runtime.md` (replacing the
  pre-convention `container/Dockerfile` + `sky/*.yaml`
  references that the §7 third-pass rewrite already cleared).
  Blocks on consumer materialising.
- Build out `ledger/events.csv` per `ARCHITECTURE.md` §8 once a
  consumer (Ring 1 `service-fs` proxy or SOC3 export job)
  materialises. The current B1 JSONL log at
  `~/.service-slm/audit/<date>.jsonl` is the v0.1 substrate.

## Blocked

- **Mooncake / LMCache licence audit for Ring 2.** Blocked on:
  operator confirmation at adoption time.
- **Mooncake master hosting.** Blocked on: choice between small
  always-on GCE VM, Totebox co-host, or SkyPilot pool.
- **Secret Manager migration.** Blocks Phase 2 key management —
  currently SSH env vars per Phase 1.
- **Adapter training hardware + evaluation protocol.** Blocks
  Ring 3b build-out.
- **A3 viability spike result.** Pending per inbox v0.0.9 — gates
  `B6`, the Doorman's GCE lifecycle controller.

## Deferred

- CUDA checkpoint/restore integration — deferred until vLLM
  RFC #34303 ships upstream.
- C-LoRA single-adapter migration — deferred until project count
  exceeds ten.
- Multi-cloud KV pool — deferred until single-cloud Ring 2 proves
  in production.
- FP8 KV-cache quantisation — deferred as Phase-2 polish.

## Recently done

- **2026-04-27 — NEXT.md Queue refresh.** Six items closed by
  recent commits but still listed under Queue have been moved
  here: cognitive-bridge.sh → scripts/ (`badd447`); cargo deny
  check in CI (`d97a994`); MISSING CONNECTION PHYSICS in
  cognitive-bridge.sh (`3c0c8e5`); cognitive-forge ↔
  content-compiler wire format reconciliation (`5da4676`); B4
  Tier C client mock-only (`d8ef1ec` + server-side env-var
  wiring `fab047e`); ARCH §5.10/§2 third-pass zero-container
  drift (`8c3212e`). Three new drift sites the third-pass
  missed (ARCH §3 line 132, ARCH §5.2 line 197, DEVELOPMENT.md
  §4 Phase 2 step 5) added as a fourth-pass Queue item pending
  Master sign-off.
- **2026-04-26 — Move `cognitive-bridge.sh` → `scripts/`
  (`badd447`).** Layout-hygiene defect closed; positional-args
  script body needed no caller audit.
- **2026-04-26 — Land `cargo deny check licenses` in CI
  (`d97a994`).** `deny.toml` policy now enforced in CI driver
  per `DEVELOPMENT.md` §2.2.
- **2026-04-26 — Close MISSING CONNECTION PHYSICS in
  `cognitive-bridge.sh` (`3c0c8e5`).** Bridge now calls the
  Doorman at `POST $SLM_BIND_ADDR/v1/chat/completions`
  (replacing the `[UNVERIFIED STAGING OVERLAY]` placeholder).
- **2026-04-26 — Reconcile `cognitive-forge` ↔
  `content-compiler` wire format (`5da4676`).** Format
  contract landed; writer and reader interoperate.
- **2026-04-26 — AS-4 POST /v1/shadow endpoint landed
  (mock-only).** `ApprenticeshipDispatcher::dispatch_shadow()`
  added — same prompt + dispatch as `/v1/brief` but the
  attempt is not returned to the caller. Tuple captured at
  the deterministic path
  `${FOUNDRY_ROOT}/data/training-corpus/apprenticeship/<task-type>/shadow-<brief_id>.jsonl`
  with `verdict: null` + `stage_at_capture: shadow`. Idempotent
  on `brief_id`: filesystem `create_new(true)` enforces
  first-write-wins even under race. Two new tests cover
  happy-path internal capture + dedup-on-retry (mock asserts
  exactly one apprentice call across two POSTs of the same
  brief_id). HTTP layer mounts `POST /v1/shadow` returning
  200 OK with empty body per Master's brief. Workspace tests
  53/53 → 55/55.
- **2026-04-26 — AS-3 POST /v1/verdict endpoint landed
  (mock-only).** New crates inside `slm-doorman`:
  - `verdict.rs` — `VerdictVerifier` async trait with
    `SshKeygenVerifier` impl (shells out to `ssh-keygen -Y
    verify -n apprenticeship-verdict-v1` against
    `${FOUNDRY_ROOT}/identity/allowed_signers`; tokio
    `spawn_blocking` keeps the stdin write off the runtime).
    Tests inject a `MockVerifier`. `VerdictDispatcher`
    orchestrates: verify → parse → cache lookup →
    sanitised corpus tuple write → ledger event under
    `flock(2)` → promotion check → DPO pair on refine/reject.
  - `promotion_ledger.rs` — `PromotionLedger { dir }` writes
    `ledger.md` + `.stats.jsonl` + `stages.json` (atomic
    rename) under one `flock(2)`. `next_stage()` applies
    convention §2 thresholds (review→spot-check at n≥50 +
    accept-rate ≥0.85; spot-check→autonomous at n≥100 +
    accept-rate ≥0.95).
  - `brief_cache.rs` — in-process FIFO from
    `(brief_id, attempt_id)` to `(brief, attempt)`; populated
    on `/v1/brief`, read on `/v1/verdict`. Default cap 1024.
  Wire shape per design-pass Q5: JSON `{ body, signature
  (base64), senior_identity }`. Six new verdict tests +
  three brief-cache tests + four promotion-ledger tests; all
  53/53 passing. Server-side: AS-3 endpoint returns 403 on
  signature failure, 410 on cache miss, 400 on parse failure.
- **2026-04-26 — AS-2 POST /v1/brief endpoint landed
  (mock-only).** `ApprenticeshipDispatcher::dispatch_brief()`
  composes the apprentice prompt (resolved citations +
  redacted scope.files contents + brief body + acceptance
  test + required-response shape), routes through
  `Doorman::route` so audit-ledger entries are captured,
  parses YAML-frontmatter / fenced-diff response, returns
  `ApprenticeshipAttempt`. Tier-B routing on chars
  > `SLM_BRIEF_TIER_B_THRESHOLD_CHARS` (default 8000 ~
  2000 tokens). Three modules added to slm-doorman:
  `redact.rs` (Rust port of `bin/capture-edit.py`
  REDACTIONS), `citations.rs` (best-effort registry resolver
  — no `serde_yaml` dep), `apprenticeship.rs` (dispatcher).
  Five new tests; workspace 25/25 → 40/40.
- **2026-04-26 — AS-1 apprenticeship types in slm-core
  landed.** Three serde wire types per convention §3-§5:
  `ApprenticeshipBrief`, `ApprenticeshipAttempt`,
  `ApprenticeshipVerdict`. Plus `SeniorRole`,
  `VerdictOutcome`, `BriefScope`,
  `APPRENTICE_ESCALATE_THRESHOLD = 0.5`,
  `DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS = 8000`,
  `VERDICT_NAMESPACE` / `VERDICT_BATCH_NAMESPACE`. Six
  round-trip serde tests; workspace 19/19 → 25/25.
- **2026-04-26 — B4 Tier C client landed (mock-only).**
  `crates/slm-doorman/src/tier/external.rs` filled per
  `~/Foundry/conventions/llm-substrate-decision.md` and
  Master's 2026-04-26 brief Answer 3. Compile-time
  `&'static [&'static str]` allowlist (`ExternalAllowlist`);
  `FOUNDRY_DEFAULT_ALLOWLIST` carries the three labels
  documented in the substrate decision (citation-grounding,
  initial-graph-build, entity-disambiguation). `TierCProvider`
  enum (Anthropic / Gemini / Openai) with model-prefix parsing
  (`anthropic:claude-haiku-4-5` form). `TierCPricing` per-token
  rates extending `PricingConfig` semantics. `complete()` runs
  the allowlist check + provider parse BEFORE any network
  attempt, then POSTs OpenAI-compatible chat-completions with
  `X-Foundry-Module-ID`, `X-Foundry-Request-ID`, and
  `X-Foundry-Tier-C-Label` headers. `slm-doorman-server` HTTP
  layer now parses an `X-Foundry-Tier-C-Label` request header
  onto `ComputeRequest::tier_c_label`. Six wiremock-based unit
  tests covering happy path with cost computation, unallowlisted
  label denial (verifies zero network calls landed at the mock
  server), missing label denial, unknown provider prefix, model
  prefix parsing, and pricing arithmetic. Workspace tests
  12/12 → 19/19 passing; clippy + fmt clean.
- **2026-04-26 — PricingConfig in YoYoTierConfig (cost-field
  path a).** Doorman now computes `cost_usd` from
  `inference_ms × per-provider hourly rate`. New
  `SLM_YOYO_HOURLY_USD` env var on the server. Two unit tests
  cover the arithmetic + the default-zero invariant.
- **2026-04-26 — third-pass zero-container drift cleanup
  (Master Answer 1).** ARCH §5.10 SkyPilot row dropped outright;
  ARCH §2 Ring 1 Bootstrap items 3+4 rewritten to GCE
  start/stop ceremony per the convention's trade-off section.
  Cluster manifest also tracked here with Master's Doctrine
  v0.0.4 triad-schema backfill.
- **2026-04-26 — B7 outbox priority ask + housekeeping.**
  Asked Master to install Doorman as systemd unit on workspace
  VM so other clusters can feed into it; archived Master's
  10:30 reply + moved prior outbox to archive.
- **2026-04-26 — B2 Yo-Yo HTTP client landed (mock-only).**
  `crates/slm-doorman/src/tier/yoyo.rs` filled out per
  `infrastructure/slm-yoyo/CONTRACT.md`. `BearerTokenProvider`
  async trait + `StaticBearer` impl. `complete()` does POST
  `/v1/chat/completions` with the four required `X-Foundry-*`
  headers, retries once on 503 (honouring `Retry-After`,
  capped at 60s), refreshes once on 401/403, refuses 410 with
  `ContractMajorMismatch` (no retry). Captures
  `X-Foundry-Inference-Ms` and `X-Foundry-Yoyo-Version`
  response headers for the audit ledger. Four wiremock unit
  tests cover happy path, 503 retry, 401 refresh, 410 mismatch
  — all passing. `slm-doorman-server` env-var contract extended
  with `SLM_YOYO_BEARER` (static-bearer dev path; real
  deployments swap in provider-specific `BearerTokenProvider`
  impls). Workspace test count 6/6 → 10/10. No live calls, no
  `tofu apply` per operator cost guardrail.
- **2026-04-26 — second-pass zero-container drift cleanup
  (4a).** Eleven sites consolidated into one commit per
  Master's per-site replacement text. Two additional drift
  surfaces (ARCH §5.10 SkyPilot row now orphaned, ARCH §2 Cloud
  Run scale-to-zero) queued in NEXT.md Queue for third-pass
  authorisation.
- **2026-04-26 — B5 verification PASSED end-to-end.** Doorman
  release binary booted against Master's `local-slm.service`
  (Tier A backend, llama-server, OLMo 3 7B Q4 on port 8080). All
  three control endpoints returned 200; `/readyz` confirmed
  community-tier mode (`has_local:true, has_yoyo:false,
  has_external:false`). One real `POST /v1/chat/completions` with
  `X-Foundry-Module-ID` and `X-Foundry-Request-ID` headers landed
  a content string from the model in 43.9 s (CPU-only on
  e2-standard-4; expected). Audit ledger at
  `~/.service-slm/audit/2026-04-26.jsonl` carries one entry as
  specified — `tier:"local"`, `cost_usd:0`, non-zero
  `inference_ms`, `completion_status:"ok"`. B3 was delivered by
  Master in v0.0.11 (`68e7c16`); D1 done operator-side prior.
- **2026-04-25 — B1 Doorman scaffold landed.** `service-slm/` is now
  a self-contained cargo workspace with `slm-core`, `slm-doorman`
  (lib with three-tier router + JSONL audit ledger), and
  `slm-doorman-server` (axum bin with /healthz, /readyz,
  /v1/contract, POST /v1/chat/completions). 6/6 unit tests pass;
  clippy and fmt clean. Standalone-vs-nested workspace question
  closed in `ARCHITECTURE.md` §6 with precedent recorded; nested
  conversion remains mechanical if the monorepo unification
  decision later goes that way.
