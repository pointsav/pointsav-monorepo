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

## Completed

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
