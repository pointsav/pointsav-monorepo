# NEXT.md — service-slm

> Last updated: 2026-04-26
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- **B2 — Yo-Yo HTTP client.** §7 zero-container rewrite landed in
  the narrow Master-authorised form (file-tree subtree only, per
  brief). B2 is the natural next pickup. Fill
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

- **ARCHITECTURE.md / DEVELOPMENT.md remaining zero-container
  drift (Master sign-off needed before second-pass).** Eight
  references survived the §7 narrow rewrite, in adjacent
  sections Master told me not to touch without going back to
  outbox first: §2 Ring 1 Bootstrap "Pre-built container in
  Artifact Registry" (rewrite to "pre-built ELF binary +
  systemd unit in GCE image"); §2 memory-tier table row
  ("Container image + GCS-cached weights" → "Native binary +
  GCS-cached weights"); §4 moduleId table row ("which container
  variant to boot" → "which binary variant to boot"); §5.9
  Sigstore description; §6 `slm-compute` crate description
  ("Cloud Run driver, container mgmt" → "GCE driver, lifecycle
  mgmt"); §8 event vocabulary `BOOT_REQUEST` SkyPilot reference;
  §10 2030 headroom SkyPilot row; plus three references in
  `DEVELOPMENT.md` (§1.1 Release-build container signing; §4
  Phase 1 "Python, vLLM, SkyPilot, dbt, Dagster"; §4 Phase 2
  "container-side for remote"; §5 B2 blocker row "SkyPilot pool
  with min_replicas=1"). Coalesce into one second-pass commit
  once Master approves expanding scope.
- **B4 — Tier C client with narrow-precision allowlist.** Fill
  `crates/slm-doorman/src/tier/external.rs`. Implement per-provider
  HTTP wiring (Anthropic Claude, Google Gemini, OpenAI). Hard-code
  the allowlist of task labels permitted to use Tier C; never
  default-fallback. Confirm with Master what the initial label set
  is before extending the allowlist.
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
- Move `cognitive-bridge.sh` → `scripts/` — layout-hygiene defect
  queued in monorepo `NEXT.md`. Single `git mv`; script body uses
  positional args only, no caller audit needed.
- Triage `transient-queues/` — mirrors the `discovery-queue`
  "Not-a-project" pattern in the registry. Decide: gitignore and
  relocate live state to `service-fs/data/`, or confirm as
  deliberate fixture. Do not alter until decided.
- Reconcile `cognitive-forge` → `content-compiler` wire format —
  writer emits `.md` files (markdown bullets); reader only
  consumes `.json`. They do not interoperate today. Pick one
  format and land the contract.
- Close "MISSING CONNECTION PHYSICS" — define the concrete wire
  from `cognitive-bridge.sh` to the local SLM. With B1 in place
  the answer is now: `POST $SLM_BIND_ADDR/v1/chat/completions`
  through the Doorman, not directly to the Tier A endpoint.
  Replace the placeholder
  `RESPONSE="[UNVERIFIED STAGING OVERLAY]..."` with the real call.
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
  `ARCHITECTURE.md` — `manifest.yaml`, `container/Dockerfile`,
  `weights/registry.yaml`, `sky/*.yaml`, `keys/secret-refs.yaml`.
  Note: `conventions/zero-container-runtime.md` (ratified
  2026-04-25) prohibits `Dockerfile` in any deployment path —
  reconcile with `ARCHITECTURE.md` §7 before scaffolding this
  directory; the references to `container/Dockerfile` and
  `requirements.txt` in §7 predate the convention and need
  rewriting in a follow-up edit.
- Build out `ledger/events.csv` per `ARCHITECTURE.md` §8 once a
  consumer (Ring 1 `service-fs` proxy or SOC3 export job)
  materialises. The current B1 JSONL log at
  `~/.service-slm/audit/<date>.jsonl` is the v0.1 substrate.
- Land `cargo deny check licenses` in CI per `DEVELOPMENT.md`
  §2.2. `deny.toml` is in place; the CI driver isn't.

## Blocked

- **system-slm connection protocol.** Largely closed by B1 —
  `slm-doorman-server` exposes the OpenAI-compatible HTTP surface
  the bridge can call. The remaining decision is whether
  `cognitive-bridge.sh` calls the Doorman or a raw Tier A endpoint.
  Once the bridge is migrated to call the Doorman, this blocker
  lifts.
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
