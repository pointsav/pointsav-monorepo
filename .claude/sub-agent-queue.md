---
schema: foundry-cluster-queue-v1
owner: task-project-slm
created: 2026-04-28
location: ~/Foundry/clones/project-slm/.claude/sub-agent-queue.md
---

# Sub-agent Queue — Task Claude on project-slm cluster

Cluster-scope sub-agent briefs ratified by Master and waiting on
operator green-light to dispatch. Master ratifies via inbox reply;
Task dispatches on operator instruction (per `conventions/model-
tier-discipline.md` §1A.6 — operator-directed dispatches are
explicit ratification, no Task self-dispatch).

When operator says "dispatch <brief-id>", Task dispatches via
`Agent` tool with `model: "sonnet"` (or `"haiku"` if the brief
notes mechanical-only).

After a brief completes, move the entry to "Completed" with
commit reference and outcome note.

---

## Coverage briefs A/B/C — ratified v0.1.33 §D

Source: PS.6 in v0.1.42 plan; original Sonnet chunk #6 audit.

### Brief A — `slm-doorman-server::http.rs` test factory + smoke + error mapping + apprenticeship-disabled [COMPLETED 2026-04-28 commit `d9ea19d` + `35a0c64`]

- **Effort**: ~3-4 hours Sonnet
- **Acceptance**: ≥10 new passing tests; clippy + fmt clean; existing 46 tests still pass
- **Constraint**: foreground + serial (writes git index); MUST run before B/C (factory dependency)
- **Files**: new `slm-doorman-server/tests/http_test.rs`; minor edits to expose `AppState` builder if needed
- **Brief text**: see outbox `2026-04-27T19:30:00Z` sub-agent-queue proposal
- **Outcome**: 12 new tests (4 smoke + 5 error-mapping + 3 apprenticeship-disabled). Tests 55 → 67. Structural change: slm-doorman-server gained `src/lib.rs` exposing `pub mod http` + `pub mod test_helpers` so integration tests can import. Test helpers reusable by B/C: `temp_ledger`, `temp_promotion_ledger`, `app_state_no_tiers`, `app_state_with_local`, `app_state_with_external`, `app_state_with_apprenticeship`. Two minor brief deviations: `TierUnavailable` maps to 503 not 502 (code authoritative); `ExternalNotAllowlisted` covered as unit-mapping assertion (router can't auto-select Tier C via /v1/chat/completions). Used `tower::ServiceExt::oneshot` (already transitive); added `wiremock` to slm-doorman-server dev-deps.

### Brief B — `tier/local.rs` unit tests [COMPLETED 2026-04-28 commit `97f360e`]

- **Effort**: ~1-2 hours Sonnet
- **Acceptance**: ≥4 new passing tests in `tier::local::tests`
- **Constraint**: foreground + serial; independent after A
- **Files**: new `#[cfg(test)]` block in `crates/slm-doorman/src/tier/local.rs`
- **Brief text**: see outbox `2026-04-27T19:30:00Z`
- **Outcome**: 5 new tests (happy path 200; default model fallback when ComputeRequest::model is None; 5xx → Upstream; empty choices → UpstreamShape; malformed JSON → Upstream). Tests 67 → 72. No unexpected findings; local.rs is straightforward (no retry/auth/custom headers). wiremock pattern reused from yoyo.rs / external.rs.

### Brief C — `VerdictOutcome::Reject` + `DeferTierC` dispatcher tests [COMPLETED 2026-04-28 commit `5087a2c`]

- **Effort**: ~1 hour Sonnet
- **Acceptance**: 2 new passing tests in `verdict::tests`
- **Constraint**: foreground + serial; independent after A
- **Files**: addition to `crates/slm-doorman/src/verdict.rs::tests`
- **Brief text**: see outbox `2026-04-27T19:30:00Z`
- **Outcome**: 2 new tests. Tests 72 → 74. Behaviour-vs-brief: Reject DOES produce DPO pair (`produces_dpo_pair()` matches `Refine`); DeferTierC does NOT (escalation not refinement). Both write corpus tuples; both record non-accept ledger events.

---

## PS.1 follow-up briefs — ratified by Master 2026-04-28

Source: Master's v0.1.42-pending PS.1 ack reply (inbox 2026-04-28T00:21Z).

### Brief PS.1-1 — `pointsav-public:slm-yoyo` GCE image existence verification (B3) [COMPLETED 2026-04-28]

- **Effort**: ~30 minutes Sonnet
- **Acceptance**: written brief reporting whether the image family exists, what it ships (vLLM vs mistral.rs vs both), what its description metadata says, what tools/binaries/services are baked in
- **Constraint**: foreground; runs gcloud-describe + optional one-shot test-VM boot
- **Sequence**: must run FIRST in the PS.1 follow-up sequence
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (3)
- **Outcome**: Project `pointsav-public` does NOT exist in GCP; image has never been built. Surfaces D4 (Master-tier image-build pipeline) as 12th blocker upstream of all PS.1 B/W items. PS.1-5 + PS.2 + Yo-Yo-MIN are now blocked on D4. PS.1-2 + PS.1-3 + PS.1-4 still proceed. CUSTOMER-RUNBOOK.md added to PS.1-3 rename scope. nginx TLS layer absent from any artefact — needs Master-tier design pass before D4 ships. Surfaced to Master via outbox 2026-04-28T01:30Z.

### Brief PS.1-2 — Module update for B1 + B2 + W1 [DISPATCHABLE — operator option-A 2026-04-28; admin-tier procedure]

- **Resolution**: operator confirmed option (a) 2026-04-28 post-v0.1.59 sweep; pipeline boundary relaxed for PS.1-2/-3/-4 admin-tier dispatch. Workspace-repo edits commit via CLAUDE.md §8 admin-tier procedure (`ps-administrator` author identity; SSH alias `github.com-pointsav-administrator`); commits land at workspace tier.
- **Effort**: ~1-2 hours Sonnet
- **Acceptance**: `infrastructure/slm-yoyo/tofu/` patches: (a) add `variable "preemptible"` default false; use `provisioning_model = var.preemptible ? "SPOT" : "STANDARD"` and `automatic_restart = !var.preemptible`; (b) extend `null_resource.gpu_quota_request` to file `NVIDIA_A100_GPUS_per-region` (when `var.gpu_class == "a100-40gb"`) or `NVIDIA_A100_80GB_GPUS_per-region` (when `a100-80gb`); (c) update `variable "gpu_class"` description to include both on-demand and Spot prices per class; document SLA in W2 README. NO `tofu apply` — module-spec edit only.
- **Constraint**: foreground + serial; pure module edit; tests not applicable
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (1)

### Brief PS.1-3 — B4 doc update (mistral.rs → vLLM rename in CONTRACT.md + variables.tf + CUSTOMER-RUNBOOK.md) [DISPATCHABLE — operator option-A 2026-04-28]

- **Resolution**: same as PS.1-2 — workspace-repo edit via admin-tier procedure.
- **Effort**: ~30 minutes Sonnet
- **Acceptance**: rename mistral.rs → vLLM in: (1) `infrastructure/slm-yoyo/CONTRACT.md` (lines 18, 66, 100 + any others); (2) `infrastructure/slm-yoyo/tofu/variables.tf` `image_family` description; (3) `infrastructure/slm-yoyo/CUSTOMER-RUNBOOK.md` (lines 29, 194-209: `systemctl status mistralrs`, `/var/lib/mistralrs/weights/`, `mistralrs-idle.timer`). Wire format unchanged. **DO NOT pin a specific vLLM patch version** — PS.1-1 finds image doesn't exist yet; pin to "vLLM ≥0.12" floor only. Patch pin lands when D4 builds the actual image.
- **Constraint**: foreground; doc edit only
- **Sequence**: independent now (was PS.1-1 dependent; resolved with version-pin caveat)
- **Brief text**: derived from Master's v0.1.42 §"B4 — vLLM" call + PS.1-1 finding

### Brief PS.1-4 — `local-doorman.env` output snippet (W6) [DISPATCHABLE — operator option-A 2026-04-28]

- **Resolution**: workspace-repo edit via admin-tier procedure.
- **Effort**: ~30 minutes Sonnet
- **Acceptance**: extend `infrastructure/slm-yoyo/tofu/outputs.tf` with envsubst-ready Doorman config snippet keyed to selected `gpu_class` (which determines `SLM_YOYO_HOURLY_USD`); operator pastes into local-doorman.service `Environment=` block after `tofu apply`
- **Constraint**: foreground; pure outputs.tf addition
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (2)

### PS.4 multi-step plan (long-running pipeline; sequential dispatches)

PS.4 was originally framed as a single ~3-5 day brief. Breaking into discrete
chunks for the pipeline:

- **PS.4 step 1 — audit_proxy endpoint scaffold** ✅ commit `40dc18e`
- **PS.4 step 2 — audit_proxy upstream provider relay** ✅ commit `028c411`
- **PS.4 step 3 — purpose allowlist enforcement** ✅ commit `acee9f7`
- **PS.4 step 4 — audit_capture endpoint scaffold** ✅ commit `36d4fab`
- **PS.4 step 5 — integration tests + cross-cluster contract doc** ✅ commit `e4cb8a8` — **PS.4 CLOSED**
- **PS.4 step 3 — purpose allowlist enforcement** (~1-2hr Sonnet)
  Add a configurable allowlist for the `purpose` field (similar to
  `ExternalAllowlist` for Tier C labels). Reject unallowlisted purposes
  with a typed error before any upstream call. Tests + default allowlist
  matching documented audit purposes (e.g., `editorial-refinement`,
  `citation-grounding`, `entity-disambiguation`).
- **PS.4 step 4 — audit_capture endpoint scaffold** (~3-4hr Sonnet)
  Inverse direction: lets Ring 1 producers (project-data anchor-emitter,
  project-language gateway) push audit events for work done locally
  without going through the Doorman. New `POST /v1/audit/capture` endpoint;
  request shape with audit_id (caller-generated UUIDv7 — caller is the
  source of truth for its own work), module_id, purpose, status (ok |
  policy-denied | upstream-error | etc.), prose-edit / design-edit /
  graph-mutation / etc. event-type discriminator, and event-specific
  payload. Validates + appends to ledger. Tests for shape + ledger
  write.
- **PS.4 step 5 — integration tests + cross-cluster contract docs** (~2-3hr)
  End-to-end tests: audit_proxy + audit_capture exercised together;
  ledger query helpers; cross-cluster contract document at
  `service-slm/docs/audit-endpoints-contract.md` for project-language A-4
  + project-data A-5 to consume.

### Brief PS.1-5 — Kill-switch first-time-run verification (W7) [BLOCKED on D4 per PS.1-1]

- **Effort**: ~30 minutes Sonnet (mostly waiting; wall time longer)
- **Acceptance**: written verification that the kill-switch fires when budget cap breached. Procedure: apply test mode (`tofu apply -var monthly_cap_usd=1`); start the GCE VM; let one inference call accrue cost (or simulate via a Pub/Sub manual publish); confirm Cloud Function fires and stops VM; `tofu destroy` to clean up.
- **Constraint**: foreground + serial; runs `tofu apply`/`destroy` in test mode; needs operator attention or pre-authorisation for the apply
- **Sequence**: BLOCKED on D4 (image-build pipeline) — `tofu apply` fails at `data "google_compute_image" "yoyo"` lookup until image exists in `pointsav-public`
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (W7 verification)

---

## Dispatch sequence (Master-suggested, 2026-04-28)

When Yo-Yo prep work resumes:
1. PS.1-1 (image verification) — foundational fact-finding
2. PS.1-2 (module update for B1+B2+W1) — bulk of the change
3. PS.1-3 (CONTRACT.md + variables.tf vLLM rename, depends on PS.1-1)
4. PS.1-4 (local-doorman.env output)
5. PS.1-5 (kill-switch verification — independent)

Coverage briefs A/B/C are independent of all PS.1-* briefs and have
no Yo-Yo dependency. They can run in parallel with the PS.1 sequence
once operator green-lights.

---

## Pending Master ratification

### 2026-04-29 — AS-3 verdict-signing fix — Option 3 + Option 2 hybrid + doctrine MINOR amendment

Master 02:05Z diagnosed the apprenticeship-arm root cause: Doorman shadow
flow is structurally broken at the AS-3 verdict-signing step. Apprentice-
completion tuples sit in BriefCache (in-memory), evicted on Doorman
restart. The 14 "apprenticeship" corpus tuples are from a DIFFERENT path
(project-language editorial Stage-1 Pattern A); the shadow-brief-via-
Doorman path has produced ZERO corpus growth since B7.

Operator green-lit recommendation at chat surface 03:00Z; outbox message
to Master committed `7c947a7`.

**Recommendation**: Option 3 (capture-on-apprentice-completion at `review`
stage) + Option 2 (Master signs at sweep cadence in parallel) + doctrine
MINOR amendment to claim #32 (additive semantics: corpus admits captured
tuples; verdicts promote to higher quality tiers).

**Implementation scope** (cluster-Task; ~3-5hr Sonnet; dispatchable on
Master ratification):

1. `apprenticeship.rs` — extend apprentice-completion path to write tuple
   to `data/training-corpus/apprenticeship/<task-type>/<tenant>/<id>.jsonl`
   immediately at `stage_at_capture: review`, verdict fields null/pending.
2. `verdict.rs` + `VerdictDispatcher` — change semantics from "create
   tuple on verdict" to "promote existing tuple". Verdict signing updates
   stage + adds verdict block.
3. BriefCache — keep for verdict-binding session window context.
4. Tests — extend apprenticeship test suite; verify corpus tuples appear
   at `review` stage; verify verdict promotes in-place.

**Two paths surfaced** (operator favors α):
- α: Master ratifies doctrine MINOR + cluster-Task implements ~4-6hr
  end-to-end.
- β: Operator-presence ratification first; cluster-Task holds.

**HOLD until Master replies.** Implementation cannot proceed without
doctrine MINOR — the change touches the entry semantics of training
corpus per claim #32, which is workspace-tier scope.

Reference doc for substrate context: `service-slm/docs/trainer-scoping.md`
commit `562baa0` (3,200 words, 11 sections).

## Completed

### 2026-04-29 — Iter-20 trainer-scoping comprehensive doc [COMPLETED commit `562baa0`]

Operator-directed Path A — comprehensive trainer scoping post-B7-LIVE.
Apprenticeship corpus is now flowing but no trainer exists to consume it.
This doc is the cluster's input to the trainer-substrate conversation.

- **Outcome**: 1 new file (`service-slm/docs/trainer-scoping.md`); ~3,200
  words / 1,047 lines / 11 sections. Commit `562baa0` (Peter Woodfine).
- **Doc-only**: tests still 143/143; no code changes.
- **Frontmatter**: v0.1.58 Research-Trail fields with research_done_count
  14 / research_suggested_count 9 / open_questions_count 13;
  research_provenance: direct-consultation.

**Key findings from the research:**

1. **One pre-framework trainer artefact exists**: `vendor/pointsav-monorepo/
   service-slm/router-trainer/` — knowledge-distillation script from iMac
   era, Qwen2.5 Coder 1.5B via llamafile, 15 email-routing tuples.
   Schema, model family, and task type all incompatible with current
   corpus. NOT directly reusable. Establishes local-distillation
   precedent.
2. **`data/adapters/` directory is declared in two conventions** but
   doesn't exist. No adapters have ever been trained against the
   current corpus.
3. **1.07 GB Qwen weight file** lives in tracked Git at the
   `router-trainer/` path — layout-rule violation predating the rule.
   Surfaced for cleanup; not acted on (outside cluster scope).
4. **`apprenticeship/prose-edit/woodfine/` tenant shard** has 1 tuple
   on day zero — tenant-isolation rule means this must be excluded
   from Vendor-side training; flagged in research trail.
5. **llama-server LoRA support unverified** — potential Phase 1 blocker
   depending on whether the deployed binary supports runtime adapter
   loading.
6. **Doctrine claim #14 (federated marketplace)** is the explicit
   mechanism for cross-Customer adapter sharing; currently
   unimplemented.

**5 deployment options enumerated** with tradeoffs (Workspace VM CPU,
Yo-Yo GPU, Customer GPU, Hyperscaler API, Federated). Yo-Yo gated on
D4. Workspace VM CPU viable for Phase 0 only. Hyperscaler is fastest-
to-first-adapter but conflicts with sovereignty framing.

**13 explicit open questions for operator** (§9 of doc). Each
answerable in a sentence; each changes implementation shape.

**Phased recommendation**:
- **Phase 0** ($0, ~4-8hr impl + 8-24hr background CPU training):
  Python SFT script reading 87 engineering tuples; QLoRA 4-bit OLMo 3
  7B via bitsandbytes; 10-20 training steps; produces adapter at
  `data/adapters/`. **Do NOT deploy** — proof of life only.
- **Phase 1**: minimum-viable adapter; real training run on chosen
  deployment option; Doorman hot-reload + composition.
- **Phase 2**: trigger model (threshold or cron); validation gates;
  promotion criteria.
- **Phase 3**: multi-adapter composition per task-type; PS.5
  graduation operational.

### 2026-04-29 — B7 LIVE — Doorman redeployed with apprenticeship_enabled=true [Master action; workspace v0.1.68]

**MAJOR MILESTONE — Stage 2 of the flow is operational.**

Operator authorized "go" at chat surface 00:21Z; Master executed iter-19
runbook end-to-end in ~5min wall time. All 8 steps landed cleanly:

| Step | Result |
|---|---|
| 1 | Pre-built binary verified at `service-slm/target/release/slm-doorman-server` (7.9 MB) |
| 2 | `sudo install -m 0755` to `/usr/local/bin/slm-doorman-server` (root:root) |
| 3 | `sudo install -m 0640` env file to `/etc/local-doorman/local-doorman.env` (root:local-doorman) |
| 4 | `mkdir + chown` audit-ledger dir at `/var/lib/local-doorman/audit/` (750 local-doorman:local-doorman) |
| 5 | Drop-in landed at `/etc/systemd/system/local-doorman.service.d/env-file.conf` |
| 6 | `systemctl daemon-reload + restart local-doorman.service` — active(running) since 00:22:25Z |
| 7 | Smoke test 7 PASS / 1 client-side timeout (Tier A cold-path > curl default; advisory) |
| 8 | corpus-stats: **86 engineering + 14 apprenticeship tuples** |

**Doorman startup log confirms the load-bearing flag**:
- `apprenticeship_enabled=true` ✓
- `audit_dir=/var/lib/local-doorman/audit/` ✓
- `bind_addr=127.0.0.1:9080` + `has_local=true` ✓
- Lark grammar pre-validation enabled (PS.3 step 5) ✓

Every commit across all 8 active clusters now feeds the apprenticeship
arm AND engineering arm. PS.5 graduate-task-types-to-service-slm-first
becomes incrementally feasible as DPO tuples accumulate.

Small follow-up logged: smoke-test script's curl timeout is shorter than
Olmo 3 7B Q4 cold-path. Could extend `--max-time` for the chat-completions
test. Minor; not blocking; queued for future iter.

### 2026-04-28 — Iter-19 B7 deploy-readiness package [COMPLETED commit `72f4100`]

Long-running Sonnet pipeline iteration 19. Operator-directed in response to
"get the flow in place so we're not wasting any of the daily corpus
training data". Cluster-Task contribution that takes B7 from "Master
figures out how to deploy" to "Master copies binary and runs systemctl
daemon-reload + restart".

- **Outcome**: 4 new files. No code changes. Tests still 143/143. Commit
  `72f4100` (Peter Woodfine).
- **Binary verified**: `cargo build --release -p slm-doorman-server`
  produces `service-slm/target/release/slm-doorman-server` (7.5 MB
  stripped). Binary NOT committed (target/ gitignored); runbook
  documents scp transfer.
- **`service-slm/docs/deploy/local-doorman.env.example`**: 17 env vars
  documented across 5 groups (server bind, Tier A, Tier B, Tier C,
  apprenticeship + audit ledger). SLM_TIER_C_* commented-out with TODO
  for when operator wires Anthropic key. Workspace-dogfood defaults
  applied (`SLM_AUDIT_TENANT_CONCURRENCY_CAP=16`,
  `SLM_AUDIT_DIR=/var/lib/local-doorman/audit/`,
  `SLM_LARK_VALIDATION_ENABLED=true`, etc.).
- **`service-slm/docs/deploy/deploy-doorman-workspace-vm.md`**: 8-step
  runbook from prerequisites through rollback + troubleshooting.
  Frontmatter includes v0.1.58 Research-Trail fields.
- **`service-slm/scripts/smoke-test-doorman.sh`** (mode 0755): 8 endpoint
  tests; `DOORMAN_URL` configurable; advisory mode (always exits 0).
- **`service-slm/scripts/corpus-stats.sh`** (mode 0755): surveys
  `~/Foundry/data/training-corpus/engineering/<cluster>/` and
  `~/Foundry/data/training-corpus/apprenticeship/`. Reports tuple count,
  date range, schema sanity-check on most recent 5 events.
- **Surprises/discoveries**:
  - 3 env-var groups not listed in original brief: `SLM_LOCAL_MODEL`,
    `SLM_YOYO_MODEL`+`SLM_YOYO_HOURLY_USD`, full FOUNDRY_* namespace
    (FOUNDRY_ROOT / FOUNDRY_ALLOWED_SIGNERS / FOUNDRY_DOCTRINE_VERSION /
    FOUNDRY_TENANT). Agent enumerated them by grep against main.rs and
    documented all 17 in env example.
  - Existing systemd unit at `infrastructure/local-doorman/` already
    carries `SLM_APPRENTICESHIP_ENABLED=true` inline. Runbook uses
    `service.d/env-file.conf` drop-in pattern instead of editing the
    unit — simpler.
  - SLM_TIER_C_* namespace SHARED between Tier C compute routing AND
    audit_proxy (one env block enables both surfaces).
  - Corpus directory check found **84 tuples in engineering corpus**
    (2026-04-26 → 2026-04-28; ~30 added today via this pipeline).
- **Build hygiene**: cargo test 143/143; clippy + fmt clean.

### 2026-04-28 — Iter-18 ARCHITECTURE.md + DEVELOPMENT.md refresh [COMPLETED commit `93718c2`]

Long-running Sonnet pipeline iteration 18. Doc-only refresh syncing both
files with shipped reality (PS.3 + PS.4 + iter-15/16/17 hardening landed
~50 tests + 2 endpoints + new contract doc + 6 new error variants since
the docs were last touched).

- **Outcome**: doc-only commit. No code changes. Commit `93718c2` (Peter
  Woodfine).
- **ARCHITECTURE.md**: added §7 crate responsibilities (covering
  `audit_proxy.rs`, `grammar_validation.rs`, all PS.3/PS.4 modules),
  rewrote §8 endpoint table (9 routes), added §9 three-tier grammar
  policy, added §10 audit substrate (cites contract doc v0.2.0), rewrote
  §6 cargo workspace, dropped stale rows.
- **DEVELOPMENT.md**: rewrote §1 build/test commands (current workspace),
  rewrote §4 from B1-B6 blockers to landed/gated table (PS.3/PS.4/PS.6/
  PS.7 LANDED; B7/D4/PS.1/PS.2/PS.5 gated), rewrote §5 deps against actual
  Cargo.toml, added §7 apprenticeship-substrate enablement.
- **Stale items dropped**: B5-verification-pending, AS-2-implementation-
  pending, mistralrs framing, Cloud Run / SkyPilot / OCI Artifact
  references, standalone-vs-nested workspace open question, forward-
  declared workspace deps that don't exist (mistralrs, candle, apalis,
  kuzu, google-cloud-run, etc.).
- **Cites added**: `service-slm/docs/audit-endpoints-contract.md` v0.2.0
  from ARCHITECTURE.md §10.
- **Frontmatter**: both files gain v0.1.58 Research-Trail fields with
  `research_provenance: tacit`.
- **Discovery**: test-count discrepancy between state files (153) and
  actual (143). Iter-17 sub-agent claimed +22 tests; actual delta was
  +12 (131 → 143). State-file housekeeping corrected in this iter's
  follow-up commit. CLAUDE.md cluster section also has minor doc-debt
  (lists 3 crates but omits slm-doorman-server's lib target) — flagged.

### 2026-04-28 — Iter-17 PS.6 chunk #6 tail coverage gaps [COMPLETED commit `436cb4f`]

Long-running Sonnet pipeline iteration 17. Closes lower-priority coverage
gaps from the original PS.6 chunk #6 audit deferred from A/B/C briefs.

- **Outcome (corrected count)**: 12 new tests across 4 modules. Tests
  131 → 143. Commit `436cb4f` (Jennifer Woodfine). The iter-17 sub-agent's
  report claimed "+22 tests" / "153 total" but cargo test workspace count
  post-iter-18 verification was 143 — the +22 figure was over-counted (the
  agent counted 10 "additional edge-case tests within sections" that were
  partially duplicates or non-incremental). Real impact is +12 tests
  closing the four gap categories.
- **All four sections applied** — codebase had all relevant layers:
  - `tier::yoyo::tests`: BearerTokenProvider failure (provider err / empty
    token + 401 refresh path) — 2 tests.
  - `ledger::tests`: HOME unset + readonly-parent + readonly-dir-append — 3 tests.
  - `redact::tests`: gho_ / xox- / false-positive prevention — 3 tests.
  - `citations::tests`: partial blocks / empty IDs / duplicates — 4 tests.
  - Plus 10 additional tests within sections covering edge cases the agent
    found during inspection.
- **No new error variants** — coverage on existing code paths.
- **Discovery**: existing `redact.rs` covered `ghp_` (GitHub PAT) but NOT
  `gho_` (GitHub OAuth — distinct token type). Pattern was wired but
  untested. New test confirms the path works; no bug, but a real coverage
  gap that's now closed.
- **Build hygiene**: cargo test 153/153; clippy + fmt clean.

### 2026-04-28 — Iter-16 audit endpoint hardening (payload cap + per-tenant concurrency) [COMPLETED commit `6e47d27`]

Long-running Sonnet pipeline iteration 16. Production-grade DoS/abuse
hardening on the audit endpoints we shipped in PS.4.

- **Outcome**: 4 new tests in `slm-doorman-server::tests::http_test`. Tests
  127 → 131. Commit `6e47d27` (Jennifer Woodfine).
- **Payload cap**: `AUDIT_PROXY_MAX_REQUEST_BYTES = 64 * 1024` in `http.rs`.
  Fires BEFORE deserialise via `Bytes` extractor (cheaper rejection).
- **Per-tenant concurrency**: `Arc<Mutex<HashMap<ModuleId, Arc<Semaphore>>>>`
  on `AppState`. `tokio::sync::Semaphore::try_acquire_owned()` (non-blocking,
  fail-fast). RAII permit-release on handler exit. Lazy-init per tenant.
  Default cap 4; configurable via `SLM_AUDIT_TENANT_CONCURRENCY_CAP`.
- **Two new error variants**:
  - `AuditProxyPayloadTooLarge { size_bytes, max_bytes }` → 413 →
    `PolicyDenied`.
  - `AuditTenantConcurrencyExhausted { module_id, cap }` → 503 +
    `Retry-After: 5` header → `PolicyDenied`. Retryable; not permanent.
- **Threat model**: single tenant flooding either endpoint exhausting Doorman
  resources (long-running Tier C calls especially). Per-tenant counts protect
  neighbouring tenants without rejecting low-volume callers.
- **Tests**: oversized 413; just-under-boundary passes; concurrency-cap
  rejects excess (pre-saturated semaphore strategy avoids `AppState: Clone +
  Send + 'static` requirement); per-tenant independence (cap=1 per tenant
  doesn't block other tenants).
- **Build hygiene**: cargo test 131/131; clippy + fmt clean.
- **Open hardening items for future iterations**:
  - Per-tenant semaphore map grows unboundedly (one entry per ModuleId
    seen). Closed-tenant-set deployments unaffected; future hardening could
    add eviction if dynamic tenant sets become common.
  - `SLM_AUDIT_TENANT_CONCURRENCY_CAP` default 4 — high-volume tenants may
    need to tune at deploy time. Note in GUIDE-doorman when PS.8 lands.
  - Per-tenant request rate limit (requests-per-second) is a separate
    concern from in-flight count and not addressed in this iter.

### 2026-04-28 — Iter-15 entry_type discriminator on all 4 ledger entry kinds [COMPLETED commit `442e161`]

Long-running Sonnet pipeline iteration 15. Closes the future-direction note
from PS.4 step 5; cross-cluster consumers (project-language A-4 + project-data
A-5 + project-bim service-codes) can now discriminate entry kind via a single
explicit field instead of field-presence inference.

- **Outcome**: 3 new tests. Tests 124 → 127. Commit `442e161` (Jennifer
  Woodfine).
- **Canonical strings**: `chat-completion` / `audit-proxy-stub` /
  `audit-proxy` / `audit-capture`. Kebab-case, no version suffix.
- **Backwards-compat strategy**: `#[serde(default = "default_entry_type_<kind>")]`
  on each struct's new `entry_type` field. Deserialisation of old JSONL
  (missing field) defaults to the correct canonical value per struct type.
  `AuditLedger::append_*` methods clone the entry and force the canonical
  constant before serialising, so callers can't accidentally write a wrong
  tag.
- **Contract doc**: `service-slm/docs/audit-endpoints-contract.md` MINOR
  bump v0.1.0 → v0.2.0. §3.1 gains canonical-string table. §3.2 rewritten:
  explicit entry_type tag is canonical; field-presence remains
  backwards-compat fallback. §5 records version-history rationale.
- **Tests**:
  - **Added**: `entry_type_tag_discriminates_all_entry_kinds` (integration,
    builds all 4 via ledger API, reads back JSONL, asserts canonical
    strings).
  - **Added**: `audit_entry_missing_entry_type_field_deserialises_with_correct_default`
    + `all_entry_types_default_correctly_when_entry_type_field_absent` (unit
    tests covering serde-default backwards-compat for all 4 structs).
  - **Preserved**: `mixed_entry_types_in_jsonl_stream_distinguishable_by_field_presence`
    — passes unchanged (proof that the field-presence fallback still works).
- **Build hygiene**: cargo test 127/127; clippy + fmt clean.

### 2026-04-28 — Iter-13 SLM_AUDIT_DIR cluster-scope wiring [COMPLETED commit `5812501`]

- **Cluster-scope** (NOT admin-tier). `service-slm/crates/slm-doorman-server/
  src/main.rs` reads `SLM_AUDIT_DIR` env var; falls back to existing
  `$HOME/.service-slm/audit/` default when unset.
- **Pattern**: env var present → `create_dir_all` first (warn-and-fallback
  on failure) → `AuditLedger::new()`; env var unset → `default_for_user()`.
  Either path emits a single startup `info!` line confirming chosen
  directory.
- **Tests**: 124/124 still passing — `temp_ledger` test helpers call
  `AuditLedger::new()` directly; main.rs change doesn't reach them.
- Pairs with PS.8 GUIDE-doorman handoff (still parked on Master catalog
  provisioning).
- Commit `5812501` (Peter Woodfine).

### 2026-04-28 — Iter-14 mistral cleanup tail (workspace tier) [COMPLETED commit `278b4ab`]

- **Admin-tier** (workspace repo). Closes the last two `mistral`
  references that fell outside PS.1-3's three-file scope:
  `CUSTOMER-RUNBOOK.es.md` line 30 (Spanish sibling) and
  `tofu/README.md` line 208.
- **Bonus accuracy**: es.md license note updated from "MIT" (mistral.rs's
  license) to "Apache 2.0" (vLLM's actual license) — Sonnet caught the
  license mismatch and corrected.
- **Final grep**: zero `mistral` hits across entire `infrastructure/
  slm-yoyo/` subtree post-commit.
- Workspace commit `278b4ab` (`ps-administrator`).

### 2026-04-28 — PS.1-2 + PS.1-3 + PS.1-4 batch (workspace-tier, admin-tier procedure) [COMPLETED]

Long-running Sonnet pipeline iterations 10-12. First operator option-A
admin-tier dispatch trio. All three commits on workspace repo (`/srv/foundry/`),
NOT cluster clone. Author: `ps-administrator`. Signed via SSH (ED25519
`SHA256:APVrt+kKC1bgKTszRBHc+5ZXdxIFD8GdGwzjCOU1LXw` confirmed on each).

- **PS.1-3** workspace commit `d6c2af6` — `mistral.rs → vLLM` rename across
  `infrastructure/slm-yoyo/CONTRACT.md` (5 sites), `tofu/variables.tf`
  (2 sites), `CUSTOMER-RUNBOOK.md` (6 sites). Placeholder note added to
  CUSTOMER-RUNBOOK.md ("systemd unit names + paths set by D4 image-build").
  No vLLM patch pin (D4 ships the canonical version).
- **PS.1-4** workspace commit `bb85219` — `local-doorman.env` snippet output
  added to `tofu/outputs.tf`; envsubst-ready Environment= block content with
  SLM_YOYO_{ENDPOINT,BEARER_ID,HOURLY_USD,MODEL} keyed to selected
  gpu_class. Folded in PS.1-3 catch-up: outputs.tf line 33 mistralrs log
  filter renamed to vllm.
- **PS.1-2** workspace commit `a268215` — module update: B1 preemptible
  variable + provisioning_model SPOT/STANDARD; B2 conditional A100 quota
  null_resources (40GB or 80GB per gpu_class); W1 cost-math docs (both
  on-demand and Spot prices per gpu_class) + Spot SLA caveat in variable
  description + README Common Operations Spot deploy example.

Sonnet wall time across the trio: ~10 minutes total (~150k tokens combined).

**Mistral cleanup tail** (out of scope for the three above; pending separate
follow-up brief):
- `infrastructure/slm-yoyo/CUSTOMER-RUNBOOK.es.md` line 30 — Spanish sibling
  parallel rename.
- `infrastructure/slm-yoyo/tofu/README.md` line 190 — tofu README rename.

Both are quick doc-only edits via admin-tier procedure when next pipeline
iteration runs.

### 2026-04-28 — PS.4 step 5 (integration tests + cross-cluster contract doc) [COMPLETED commit `e4cb8a8`]

Long-running Sonnet pipeline iteration 9 — **FINAL slice of PS.4 multi-day
work**. PS.4 sequence now CLOSED.

- **Outcome**: 3 new integration tests in
  `slm-doorman-server/tests/audit_endpoints_integration.rs` (new file).
  Tests 121 → 124. Commit `e4cb8a8` (Peter Woodfine).
- **Contract doc**: `service-slm/docs/audit-endpoints-contract.md` (NEW).
  Five sections: POST /v1/audit/proxy, POST /v1/audit/capture, audit
  ledger format, error handling table, versioning + stability.
  v0.1.0 baseline; PATCH/MINOR/MAJOR rules per workspace `CLAUDE.md` §7.
  Frontmatter includes the v0.1.58 Research-Trail fields with
  `research_provenance: tacit` (honest declaration — contract derives
  directly from implementation, not external research).
- **Integration tests**:
  - `audit_capture_then_audit_proxy_round_trip` — prose-edit capture +
    citation-grounding proxy; verifies 3 entries (1 capture + 1 proxy
    stub + 1 proxy final) land in same JSONL file, distinct audit_ids,
    correct field presence.
  - `audit_proxy_failure_records_stub_only_then_capture_succeeds_independently` —
    upstream 500 produces stub + final-with-error; subsequent
    anchor-event capture still writes correctly.
  - `mixed_entry_types_in_jsonl_stream_distinguishable_by_field_presence` —
    executable form of contract-doc §3.2; synthesises all four entry
    types directly via ledger API and verifies field-presence
    discrimination correctly tags each.
- **`entry_type` discriminator NOT added** (deferred per brief; named
  as future direction in contract doc §3.2). Adding it later would be
  a MINOR contract bump.
- **Build hygiene**: cargo test 124/124; clippy + fmt clean.

### 2026-04-28 — PS.4 step 4 (audit_capture endpoint scaffold) [COMPLETED commit `36d4fab`]

Long-running Sonnet pipeline iteration 8. The inverse direction of audit_proxy:
Ring 1/2/3 producers push audit events for work done locally without going
through the Doorman.

- **Outcome**: 6 new tests in `slm-doorman-server::tests::http_test`. Tests
  115 → 121. Commit `36d4fab` (Peter Woodfine).
- **New endpoint**: `POST /v1/audit/capture`. Accepts `AuditCaptureRequest`
  with caller-generated `audit_id` (UUID, any version), `module_id`,
  `event_type`, `source`, `status`, `event_at` (caller's clock), `payload`
  (untyped JSON object), optional `caller_request_id`. Writes
  `AuditCaptureEntry` to the same daily JSONL ledger; returns 200 with
  `AuditCaptureResponse { audit_id, caller_request_id, status: "captured" }`.
- **Cross-cluster types**: `AuditCaptureRequest` / `AuditCaptureResponse`
  in `slm-core/src/lib.rs` for project-language A-4 + project-data A-5
  import.
- **Five accepted event types**: `prose-edit`, `design-edit`, `graph-
  mutation`, `anchor-event`, `verdict-issued`. Validated against
  `AUDIT_CAPTURE_VALID_EVENT_TYPES` const slice in `http.rs`.
- **Payload cap**: `AUDIT_CAPTURE_MAX_PAYLOAD_BYTES = 16 * 1024` (16 KiB).
  DoS-prevention floor.
- **Single-entry design** (vs audit_proxy's stub + final pair): work
  already happened upstream of capture; no two-phase commit needed.
  `AuditCaptureEntry` carries `event_type`, `source`, `event_at`
  (caller clock), `captured_at` (Doorman clock), `payload`. No
  token/cost/provider fields (those are proxy-specific).
- **Three new error variants**:
  - `AuditCaptureUnknownEventType { event_type }` → 400 BAD_REQUEST
  - `AuditCapturePayloadTooLarge { size_bytes, max_bytes }` → 413
    PAYLOAD_TOO_LARGE
  - `AuditCaptureInvalidTimestamp { value }` → 400 BAD_REQUEST
- **Build hygiene**: cargo test 121/121; clippy + fmt clean.
- **For step 5**: no explicit `entry_type` discriminator field exists in
  the JSONL stream today — consumers distinguish by field presence (e.g.,
  `event_type` indicates capture; `provider` indicates proxy). Worth
  documenting in the cross-cluster contract doc; hardening (adding an
  explicit type tag) is optional for step 5 or later.

### 2026-04-28 — PS.4 step 3 (audit_proxy purpose allowlist) [COMPLETED commit `acee9f7`]

Long-running Sonnet pipeline iteration 7. Enforces a configurable allowlist
on the `purpose` field of `AuditProxyRequest`.

- **Outcome**: 4 new tests in `slm-doorman-server::tests::http_test`. Tests
  111 → 115. Commit `acee9f7` (Peter Woodfine).
- **Allowlist type**: `AuditProxyPurposeAllowlist` in
  `crates/slm-doorman/src/audit_proxy.rs`. `&'static [&'static str]` with
  `EMPTY` const and `from_static` constructor — mirrors
  `tier::external::ExternalAllowlist` pattern exactly.
- **Default purposes** (in `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST`):
  `editorial-refinement`, `citation-grounding`, `entity-disambiguation`,
  `initial-graph-build`. Sourced from `conventions/llm-substrate-decision.md`.
- **Empty-allowlist semantic**: fail-closed — all purposes denied.
  Documented in `EMPTY` doc comment + handler logic.
- **Ordering**: allowlist check runs BEFORE audit_id generation /
  stub-ledger write. Rationale: policy-denied requests should not pollute
  the audit trail. Tests verify zero JSONL entries on rejection.
- **New error variant**: `AuditProxyPurposeNotAllowlisted { purpose: String }`.
  HTTP 403 FORBIDDEN. `CompletionStatus::PolicyDenied`. Same classification
  as `ExternalNotAllowlisted`.
- **AppState change**: `audit_proxy_purpose_allowlist:
  AuditProxyPurposeAllowlist` added as a separate field from
  `audit_proxy_client`. The handler uses `state.audit_proxy_purpose_allowlist`
  directly; the allowlist field on `AuditProxyConfig` is kept for client-side
  symmetry but not consulted by the handler. Deliberate separation enables
  per-deployment allowlist configuration independent of relay-client config.
- **Existing test fixture rename**: `valid_audit_proxy_body()` /
  `valid_audit_proxy_relay_body()` switched from `"editorial-grammar-check"`
  to `"editorial-refinement"` (which IS on the default allowlist).
- **Build hygiene**: cargo test 115/115; clippy + fmt clean.

### 2026-04-28 — PS.4 step 2 (audit_proxy upstream provider relay) [COMPLETED commit `028c411`]

Long-running Sonnet pipeline iteration 6. Mock-only per standing B4 operator
guardrail (no live API calls; wiremock; no provider-SDK installs).

- **Outcome**: 9 new tests across slm-doorman + slm-doorman-server. Tests
  102 → 111. Commit `028c411` (Peter Woodfine).
- **Resumption note**: First dispatch hit API 500 at ~7.5min/49 tool uses,
  with substantial uncommitted work in tree. Resumed via SendMessage to
  the same agent for completion (3 compile errors in tests file:
  ledger_dir binding-name mismatch in two tests; missing
  `audit_proxy_client: None` in one AppState literal). Resume took ~3.5min.
- **New module**: `crates/slm-doorman/src/audit_proxy.rs` — `AuditProxyClient`
  parallel to `ExternalTierClient`. Same env-var contract (`SLM_TIER_C_*`
  for Anthropic / Gemini / OpenAI endpoints + keys); same `TierCPricing`
  arithmetic; raw `reqwest` (mockable). Per-provider request body +
  authentication header construction (Anthropic `x-api-key` +
  `anthropic-version`; Gemini `x-goog-api-key`; OpenAI `Authorization:
  Bearer`).
- **Two-entry ledger design**: handler writes a `AuditProxyStubEntry` on
  validation success (preserved from step 1); after relay completes (ok or
  upstream-error), writes a SECOND entry with prompt_tokens,
  completion_tokens, cost_usd, latency_ms, status. Audit trail of attempted
  call survives even if upstream fails.
- **New error variant**: `DoormanError::AuditProxyProviderUnavailable
  { provider: String }`. HTTP 503 SERVICE_UNAVAILABLE.
  `CompletionStatus::UpstreamError` (server-side config gap, not caller-side
  rule violation).
- **AppState change**: added `audit_proxy_client: Option<AuditProxyClient>`.
  `slm-doorman-server::main` reads `SLM_TIER_C_*` env vars per provider;
  builds the client only if at least one provider has both endpoint + key.
  None means audit_proxy returns 503 with "unconfigured" message.
- **Test helpers**: new `app_state_with_audit_proxy(...)` helper takes
  wiremock URL + provider variant.
- **Build hygiene**: cargo test 111/111; clippy + fmt clean.
- **Operator guardrail observed**: no live API calls; no provider-SDK
  installs.

### 2026-04-28 — PS.4 step 1 (audit_proxy endpoint scaffold) [COMPLETED commit `40dc18e`]

Long-running Sonnet pipeline iteration 5. First slice of multi-day PS.4 work.

- **Outcome**: 5 new tests in `slm-doorman-server::tests::http_test`. Tests
  97 → 102. Commit `40dc18e` (Peter Woodfine).
- **New endpoint**: `POST /v1/audit/proxy` — accepts an `AuditProxyRequest`
  with `module_id`, `purpose`, `provider` ("anthropic" | "gemini" | "openai"),
  `model`, `messages`, optional sampling params, and `caller_request_id`.
  Validates shape; on success writes a stub audit-ledger entry and returns
  503 SERVICE_UNAVAILABLE with `{audit_id, caller_request_id, error: "audit_proxy
  upstream relay pending PS.4 step 2"}`.
- **Request/response types**: defined in `slm-core/src/lib.rs` so other clusters
  (project-language, project-data) can import as typed clients.
- **Audit-ledger stub**: new `AuditProxyStubEntry` struct (not a variant of
  `AuditEntry`) written to the same daily JSONL file via new
  `AuditLedger::append_proxy_stub()` method. Status field carries
  "scaffold-stub-no-relay-yet" until step 2 upgrades to "ok" or
  "upstream-error".
- **New error variant**: `DoormanError::AuditProxyInvalidProvider { provider:
  String }`. HTTP 400, `PolicyDenied` classification.
- **HTTP status choice**: 503 not 501 — the endpoint IS functional (it
  validates, writes the audit_id, echoes back); only the upstream relay is
  pending.
- **Build hygiene**: cargo test 102/102; clippy + fmt clean.

### 2026-04-28 — PS.3 step 5 (llguidance Doorman-side Lark validation) [COMPLETED commit `978ab79`]

Long-running Sonnet pipeline iteration 4. Optional fail-fast layer on top of
the three-tier grammar policy.

- **Outcome**: 7 new tests (4 in `grammar_validation::tests`, 3 in
  `slm-doorman-server` http_test wiremock + status mapping). Tests 90 → 97.
  Commit `978ab79` (Peter Woodfine).
- **llguidance version pinned**: `1.7` (locks to 1.7.4). No extra feature
  flags; default features include the Lark compiler and toktrie. Binary
  size delta +1.4 MB (6.3 → 7.7 MB stripped release).
- **Validation entry point**: `LarkValidator::new()` wraps
  `Arc<ParserFactory>` initialised with `ApproximateTokEnv::single_byte_env()`
  (the standalone-validation pattern from `llguidance/sample_parser/src/
  minimal.rs` — no real LLM tokenizer required). `validate(&str)` returns
  `Result<(), String>` with line/column-annotated error message on
  malformed Lark.
- **Wired into router**: `DoormanConfig.lark_validator: Option<LarkValidator>`;
  pre-validation guard before Tier B dispatch; new variant
  `DoormanError::MalformedLarkGrammar { reason }` → 400 BAD_REQUEST →
  `CompletionStatus::PolicyDenied`. `slm-doorman-server::main` enables by
  default with `SLM_LARK_VALIDATION_ENABLED=false` opt-out; non-fatal on
  init failure.
- **Latency**: ~1 ms/call in release mode; factory shared across requests
  via `Arc`.
- **Tests of note**: `lark_validation_runs_before_tier_b_dispatch` is the
  critical wiremock proof — malformed Lark + Tier B routing produces zero
  network requests.
- **Build hygiene**: cargo test 97/97; clippy + fmt clean.

### 2026-04-28 — PS.3 step 4 (Tier C rejects all grammar variants) [COMPLETED commit `fdee78f`]

Long-running Sonnet pipeline iteration 3. Smallest chunk in PS.3 sequence.

- **Outcome**: 3 new tests in `tier::external::tests` (Lark / GBNF / JsonSchema
  rejection, all asserting zero network requests). Tests 87 → 90. Commit
  `fdee78f` (Peter Woodfine).
- **New error variant**: `DoormanError::TierCGrammarUnsupported { dialect:
  &'static str, advice: &'static str }`. Mirrors `TierAGrammarUnsupported`
  pattern exactly: `CompletionStatus::PolicyDenied` (router) → 400 BAD_REQUEST
  (HTTP).
- **Ordering decision**: grammar check runs AFTER allowlist check (step 2 of
  6 in `complete()`). Allowlist is the more fundamental gate — an
  unallowlisted request must be refused regardless of grammar.
- **Files touched** (5, all cluster-scope): `crates/slm-doorman/src/error.rs`,
  `tier/external.rs`, `router.rs` (classify_error arm), `slm-doorman-server/
  src/http.rs` (From<DoormanError> arm), `tests/http_test.rs` (mirror match).
- **Build hygiene**: cargo test 90/90; clippy + fmt clean.
- No surprises; no layer-scope concerns; no deviations from brief.

### 2026-04-28 — PS.3 step 3 (Tier A rejects Lark, passes GBNF/JsonSchema) [COMPLETED commit `9f9f37b`]

Long-running Sonnet pipeline iteration 2.

- **Outcome**: 4 new tests in `tier::local::tests` (None / GBNF / JsonSchema /
  Lark→error). Tests 83 → 87. Commit `9f9f37b` (Peter Woodfine).
- **New error variant**: `DoormanError::TierAGrammarUnsupported { dialect:
  &'static str, advice: &'static str }`. HTTP mapping: 400 BAD_REQUEST.
  Classified as `CompletionStatus::PolicyDenied` in router.
- **Wire fields**: GBNF serialises to top-level `grammar` (NOT inside extra_body
  — that's vLLM-specific; llama-server uses native field). JsonSchema serialises
  to top-level `json_schema`. Lark rejected before any network call (test asserts
  zero requests received).
- **Build hygiene**: cargo test 87/87; clippy + fmt clean.
- **Files touched** (all cluster-scope): `crates/slm-doorman/src/error.rs`,
  `tier/local.rs`, `router.rs` (exhaustive match update), `slm-doorman-server/
  src/http.rs`, `tests/http_test.rs` (mirror match).
- **Surprises**: router.rs `classify_error()` and `tests/http_test.rs`
  `doorman_error_to_status` mirror match both required updates beyond the brief
  scope — exhaustive matches on `DoormanError` enum. Sonnet handled cleanly.

### 2026-04-28 — PS.3 step 2 (Yo-Yo client grammar serialisation) [COMPLETED commit `266fa4d`]

Long-running Sonnet pipeline iteration 1. Operator-directed; explicit ratification
per `conventions/model-tier-discipline.md` §1A.6.

- **Outcome**: 4 new tests in `tier::yoyo::tests` (Lark / GBNF / JsonSchema / None).
  Tests 79 → 83. Commit `266fa4d` (Peter Woodfine).
- **Implementation**: Added `extra_body: Option<serde_json::Value>` to private
  `OpenAiChatRequest` struct with `skip_serializing_if = "Option::is_none"`. The
  `complete()` method maps `ComputeRequest.grammar` to vLLM ≥0.12 envelope:
  - `GrammarConstraint::Lark(s)` → `extra_body.structured_outputs.grammar = s`
  - `GrammarConstraint::Gbnf(s)` → `extra_body.structured_outputs.grammar = s`
    (vLLM's llguidance backend auto-detects Lark vs GBNF; no separate format hint
    field needed per the agent's research)
  - `GrammarConstraint::JsonSchema(v)` → `extra_body.structured_outputs.json_schema = v`
  - `None` → no `extra_body.structured_outputs` field at all (no empty objects emitted)
- **Build hygiene**: clippy + fmt clean post-commit.
- **No layer-scope concerns**: edits confined to
  `service-slm/crates/slm-doorman/src/tier/yoyo.rs`.

### 2026-04-28 — Coverage briefs A/B/C (PS.6) — all three landed cleanly

Operator green-light "set it up to do all the recommendations" 2026-04-28. Three foreground-serial Sonnet sub-agent dispatches; each agent committed via `bin/commit-as-next.sh`. Workspace tests 55 → 74 (+19) across the batch. PS.6 (task #14 in local list) closed.

- A: `d9ea19d` + `35a0c64`. 12 tests; structural addition of `src/lib.rs` to slm-doorman-server.
- B: `97f360e`. 5 tests in `tier::local::tests`.
- C: `5087a2c`. 2 tests in `verdict::tests` covering Reject + DeferTierC.

Recurring SSH-perm issue: agents had to chmod 600 the staging-tier keys *three times* across this batch (back to 0640 between commits — some workspace process is touching them). Flagged to Master in outbox 2026-04-28T03:30Z (separate from PS.1-1 / layer-scope outboxes earlier today).

### 2026-04-28 — PS.1-1 image verification

- Dispatched 2026-04-28 by Task Claude (operator green-light "yes")
- Sonnet sub-agent foreground; ~30 min wall time; ~70k tokens
- Outcome: GCP project `pointsav-public` does NOT exist; image has never been built; D4 image-build pipeline never dispatched. Surfaced to Master via outbox 2026-04-28T01:30Z. See sub-agent transcript in session log.
- Knock-on effects: PS.1-3 scope expanded (CUSTOMER-RUNBOOK.md added); PS.1-5 + PS.2 + Yo-Yo-MIN blocked on D4; nginx TLS layer absent from any artefact (Master-tier design pass needed before D4).
