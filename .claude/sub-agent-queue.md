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

### Brief A — `slm-doorman-server::http.rs` test factory + smoke + error mapping + apprenticeship-disabled

- **Effort**: ~3-4 hours Sonnet
- **Acceptance**: ≥10 new passing tests; clippy + fmt clean; existing 46 tests still pass
- **Constraint**: foreground + serial (writes git index); MUST run before B/C (factory dependency)
- **Files**: new `slm-doorman-server/tests/http_test.rs`; minor edits to expose `AppState` builder if needed
- **Brief text**: see outbox `2026-04-27T19:30:00Z` sub-agent-queue proposal

### Brief B — `tier/local.rs` unit tests

- **Effort**: ~1-2 hours Sonnet
- **Acceptance**: ≥4 new passing tests in `tier::local::tests`
- **Constraint**: foreground + serial; independent after A
- **Files**: new `#[cfg(test)]` block in `crates/slm-doorman/src/tier/local.rs`
- **Brief text**: see outbox `2026-04-27T19:30:00Z`

### Brief C — `VerdictOutcome::Reject` + `DeferTierC` dispatcher tests

- **Effort**: ~1 hour Sonnet
- **Acceptance**: 2 new passing tests in `verdict::tests`
- **Constraint**: foreground + serial; independent after A
- **Files**: addition to `crates/slm-doorman/src/verdict.rs::tests`
- **Brief text**: see outbox `2026-04-27T19:30:00Z`

---

## PS.1 follow-up briefs — ratified by Master 2026-04-28

Source: Master's v0.1.42-pending PS.1 ack reply (inbox 2026-04-28T00:21Z).

### Brief PS.1-1 — `pointsav-public:slm-yoyo` GCE image existence verification (B3)

- **Effort**: ~30 minutes Sonnet
- **Acceptance**: written brief reporting whether the image family exists, what it ships (vLLM vs mistral.rs vs both), what its description metadata says, what tools/binaries/services are baked in
- **Constraint**: foreground; runs gcloud-describe + optional one-shot test-VM boot
- **Sequence**: must run FIRST in the PS.1 follow-up sequence
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (3)

### Brief PS.1-2 — Module update for B1 + B2 + W1

- **Effort**: ~1-2 hours Sonnet
- **Acceptance**: `infrastructure/slm-yoyo/tofu/` patches: (a) add `variable "preemptible"` default false; use `provisioning_model = var.preemptible ? "SPOT" : "STANDARD"` and `automatic_restart = !var.preemptible`; (b) extend `null_resource.gpu_quota_request` to file `NVIDIA_A100_GPUS_per-region` (when `var.gpu_class == "a100-40gb"`) or `NVIDIA_A100_80GB_GPUS_per-region` (when `a100-80gb`); (c) update `variable "gpu_class"` description to include both on-demand and Spot prices per class; document SLA in W2 README. NO `tofu apply` — module-spec edit only.
- **Constraint**: foreground + serial; pure module edit; tests not applicable
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (1)

### Brief PS.1-3 — B4 doc update (mistral.rs → vLLM rename in CONTRACT.md + variables.tf)

- **Effort**: ~30 minutes Sonnet
- **Acceptance**: `infrastructure/slm-yoyo/CONTRACT.md` + `tofu/variables.tf` `image_family` description updated to name vLLM as the runtime; `X-Foundry-Yoyo-Version` example updated from `mistralrs:0.8` → `vllm:0.12.0` (or current version per Brief PS.1-1 image inspection); wire format unchanged (vLLM's OpenAI-compatible endpoint serves the same shape)
- **Constraint**: foreground; doc edit only
- **Sequence**: depends on PS.1-1 outcome (image inspection confirms exact vLLM version to pin)
- **Brief text**: derived from Master's v0.1.42 §"B4 — vLLM" call

### Brief PS.1-4 — `local-doorman.env` output snippet (W6)

- **Effort**: ~30 minutes Sonnet
- **Acceptance**: extend `infrastructure/slm-yoyo/tofu/outputs.tf` with envsubst-ready Doorman config snippet keyed to selected `gpu_class` (which determines `SLM_YOYO_HOURLY_USD`); operator pastes into local-doorman.service `Environment=` block after `tofu apply`
- **Constraint**: foreground; pure outputs.tf addition
- **Brief text**: see outbox `2026-04-27T23:30:00Z` candidate (2)

### Brief PS.1-5 — Kill-switch first-time-run verification (W7)

- **Effort**: ~30 minutes Sonnet (mostly waiting; wall time longer)
- **Acceptance**: written verification that the kill-switch fires when budget cap breached. Procedure: apply test mode (`tofu apply -var monthly_cap_usd=1`); start the GCE VM; let one inference call accrue cost (or simulate via a Pub/Sub manual publish); confirm Cloud Function fires and stops VM; `tofu destroy` to clean up.
- **Constraint**: foreground + serial; runs `tofu apply`/`destroy` in test mode; needs operator attention or pre-authorisation for the apply
- **Sequence**: independent of PS.1-1..PS.1-4; can run any time before MIN deploy
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

*(none yet — queue created 2026-04-28; first dispatches pending operator green-light)*
