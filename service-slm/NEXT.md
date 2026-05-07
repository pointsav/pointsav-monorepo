# NEXT.md — service-slm

> Last updated: 2026-05-07 (D4 image pipeline + nightly drain infrastructure)
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now — OPERATOR-PRESENCE REQUIRED (all code tasks complete)

All software-layer tasks from the Leapfrog 2030 architecture are now complete.
Reference: `service-slm/docs/topic-leapfrog-architecture.md`

**Software Configuration (ALL DONE — commits `6bbbe49` → `5a6d3f0`):**
- [x] **Multi-Yo-Yo Support:** `main.rs` supports `SLM_YOYO_TRAINER_ENDPOINT` +
  `SLM_YOYO_GRAPH_ENDPOINT`; `HashMap<String, YoYoTierClient>` routes by label.
- [x] **Grammar Constraints:** `service-content` passes entity JSON Schema as `grammar`
  field + `X-Foundry-Yoyo-Label: graph` header; Doorman deserializes and routes to Yo-Yo #2.
- [x] **Seed Alignment:** `Archetypes.json`, `Domains.json`, `Themes.json` present in
  `service-content/seeds/` (3 domains, 4 themes, 5 archetypes — verified 2026-05-05).
- [x] **Tier C Drafting Pipeline:** `service-content POST /v1/draft/generate` wired;
  queries LadybugDB graph → ≤2K-token prompt → Doorman `/v1/audit/proxy` → Claude Sonnet.
- [x] **Phase 3 threshold watcher:** `service-slm/scripts/corpus-threshold.py` + systemd
  timer `training-trigger.timer` (Sunday 02:00 UTC); marker-only mode pre-D4.
- [x] **DataGraph proxy endpoints (2026-05-06, commit `5a6d3f0`):** `POST /v1/graph/query`
  + `POST /v1/graph/mutate` in slm-doorman-server; proxy to service-content; audit-log
  as `graph-query` / `graph-mutation`; require `X-Foundry-Module-ID`; 167/167 tests.

**Infrastructure Provisioning (Operator-gated — D4 code complete as of 2026-05-07):**

D4 pipeline landed in canonical at `0140176`. Runbook: `docs/deploy/deploy-yoyo-tier-b.md`.

- [ ] **Create GCP Project:** Physically create the `pointsav-public` GCP project.
- [ ] **L4 quota:** Request `NVIDIA_L4_GPUS` quota in `us-west1` via GCP console.
- [ ] **Build image:** `cd service-slm/compute/packer && packer build yoyo-image.pkr.hcl`
  → publishes to `slm-yoyo` family in `pointsav-public`.
- [ ] **Provision infra:** `cd service-slm/compute/opentofu && tofu apply` → VM + 100 GB SSD +
  Instance Schedule (02:00 UTC nightly start) + firewall + IAM.
- [ ] **Upload weights:** `gcloud compute scp olmo-3-32b-think-q4.gguf yoyo-tier-b-1:/data/weights/`
- [ ] **Wire Doorman env vars:** Add 7 Tier B vars to `/etc/local-doorman/local-doorman.env`
  (see `docs/deploy/local-doorman.env.example` and runbook Step 5).
- [ ] **Restart Doorman + verify:** `sudo systemctl restart local-doorman` → `/readyz` must show
  `has_yoyo: true`; circuit closes within 30 s of vLLM reporting healthy.
- [ ] **Smoke test nightly drain:** Run `scripts/start-yoyo.sh`, push one shadow brief, confirm
  drain + idle-shutdown fires after 30 min (runbook Step 7).
- [ ] **Re-enable apprenticeship:** Set `SLM_APPRENTICESHIP_ENABLED=true` in Doorman env.
- [ ] **Tier C Auth:** Add Anthropic API key to `local-doorman.env` — enables `audit_proxy`
  and `service-content POST /v1/draft/generate`. Requires operator API key.
- [ ] **cmake + C++ compiler:** `apt install cmake build-essential` on workspace VM
  (required for `lbug = "0.16"` at `cargo build` time in service-content).
- [ ] **Deploy Yo-Yo #2 (Extractor):** `a3-highgpu-1g` Dedicated; deploy when ready to
  process `cluster-totebox-jennifer`.
- [ ] **Batch Ingestion:** Feed 1,600+ deployment files into Yo-Yo #2; monitor LadybugDB growth.

---

## Previous Phase — Phase 2 COMPLETE (2026-05-01):
- **Brief C** `f2e158f` — `service-content/scripts/forge-seeds.sh` path generalization
- **Brief D** `6f664f9` — LadybugDB graph engine + HTTP server on port 9081
  (`GraphStore` trait + `LbugGraphStore`; `/v1/graph/context` + `/v1/graph/mutate`)
- **Brief E** `624828d` — Doorman `GraphContextClient` wired into `router.rs`
  (Ring 2 → Ring 3 graph grounding; non-fatal; `SERVICE_CONTENT_ENDPOINT` env var)
- **Tests: 157/157** (14 slm-core + 92 slm-doorman + 5 audit_endpoints + 4 queue + 42 http)
- Doctrine claim #44 (Knowledge-Graph-Grounded Apprenticeship) operational at code layer

**Phase 3 — awaiting operator green-light:**
- Training threshold detection: 50-tuple trigger per adapter corpus bucket
- Sunday 02:00 UTC fallback cron
- First adapter: `engineering-pointsav`
- Quality gate: ≥60% validation acceptance rate

**Operator-presence carries (urgent):**
- Yo-Yo idle-shutdown timer (runbook step 8) — 5 min; closes $520/mo → $130/mo
- Stage-6 promote authorization (cluster now 12 commits ahead of origin/main)
- `cmake` + C++ compiler on workspace VM — required for `lbug = "0.16"` to compile
  (LadybugDB uses a C++ build; `cargo check` passes but full `cargo build` of
  service-content needs cmake present)

**Resume on next session:**
1. Read inbox
2. Phase 3 on operator go-ahead; operator-presence carries above first

---

**Iter-24 LANDED** — operator-directed deep-research scoping doc at
`service-slm/docs/yoyo-training-substrate-and-service-content-integration.md`
(commit `8ce4fce`). 10,837 words; 11 sections; 22 external sources;
14 operator open questions. Three urgent findings: service-content
bypasses Doorman (Phase 1 fix); KuzuDB acquired by Apple Oct 2025
(Phase 2 graph DB decision needed); OLMo 3 32B Think has NO commercial
API (validates Yo-Yo investment). Three Doctrine claim candidates
proposed (#43 Single-Boundary Compute Discipline; #44 Knowledge-Graph-
Grounded Apprenticeship; #45 TUI-as-Corpus-Producer). Phased Phase 0-6
roadmap. 6 Master-ratification proposals queued in outbox followup.

---

**Iter-22 + Iter-23 LANDED** — Brief Queue Substrate cluster-Task scope
COMPLETE per apprenticeship-substrate.md §7C (doctrine v0.0.14).

- iter-22 `03b0b78` — queue.rs module (~870 lines) + drain worker + 5
  §7C-required tests (147 → 152). flock(2) + atomic rename; idempotent
  enqueue; reaper sweeps long-leased briefs.
- iter-23 `66790b8` — shadow_handler async-202 (no more 300s capture-edit
  timeout); worker drains queue + dispatches to apprentice + writes
  corpus tuple on completion (preserves v0.0.13 capture-on-completion
  semantics from iter-21). Tests 152 → 154.

**Cluster posture**: 154/154 verified. This is the structural moment per
Master 04:05Z: "service-SLM crosses from 'configured but not training' to
'actually training continuously.'"

**AS-3 fix LANDED** — cluster commit `a161992` 2026-04-29T~03:30Z.
Master ratified Path α at workspace tier (doctrine v0.0.13;
convention §7B; AS-3/4/5 marked Live). Tests **147/147** (+5).
`apprenticeship.rs::dispatch_shadow` now writes corpus tuple at
`stage_at_capture: "review"` immediately on apprentice completion.
`verdict.rs::dispatch` changed to promote-existing-tuple semantics.
New `OrphanVerdictNoCorpusTuple` → 410 error variant. BriefCache
retained for session-window verdict-binding context. Awaiting Master
Stage-6 promote + binary rebuild + systemctl restart sequence per
03:13Z reply.

---

**B7 LIVE 2026-04-29T00:22:25Z — workspace v0.1.68 closes B7.**
Doorman is up and healthy. Engineering corpus capture works
(87+ tuples). Apprenticeship arm via Doorman shadow flow is the
broken layer; AS-3 fix above addresses it.

Master executed the iter-19 runbook end-to-end in ~5min wall time
(operator chat-authorized 00:21Z; deploy 00:22Z; LIVE confirmation 00:25Z).
Doorman startup log confirms `apprenticeship_enabled=true`. Smoke test
7/8 PASS (1 advisory timeout on Tier A cold-path). corpus-stats:
**86 engineering + 14 apprenticeship tuples** at flow-online moment.

Every commit across all 8 active clusters (project-slm + project-data +
project-orgcharts + project-language + project-proofreader +
project-system + project-knowledge + project-bim) now feeds both arms of
the corpus:
- Engineering arm: `capture-edit` hook → engineering corpus JSONL
- Apprenticeship arm: shadow brief → Doorman `/v1/shadow` → apprenticeship corpus

PS.5 graduate-task-types-to-service-slm-first becomes incrementally
feasible as DPO tuples accumulate. PointSav-LLM continued-pretraining +
`apprenticeship-pointsav` / `apprenticeship-woodfine` LoRA training data
starts compounding from this moment.

**Iter-19 outcome — B7 deploy-readiness package** (cluster `72f4100`):
- 4 new files: env example + runbook + smoke-test + corpus-stats.
- Binary built + verified (7.5 MB stripped). NOT committed.
- 17 env vars documented; SLM_TIER_C_* commented-out for Anthropic-key
  TODO; workspace-dogfood defaults applied.
- Runbook used systemd `service.d/env-file.conf` drop-in pattern (avoided
  editing existing workspace-tier unit at `infrastructure/local-doorman/`).

---

**Iter-15 + Iter-16 + Iter-17 + Iter-18 outcomes** (post-iter-14 hardening sweep):
- **Iter-15** `442e161` — entry_type discriminator on all 4 ledger entry
  kinds. Contract v0.1.0 → v0.2.0 MINOR. Tests 124 → 127.
- **Iter-16** `6e47d27` — audit endpoint hardening: 64 KiB payload cap on
  /v1/audit/proxy + per-tenant concurrency cap on both endpoints
  (default 4 via `SLM_AUDIT_TENANT_CONCURRENCY_CAP`). Two new error
  variants. Tests 127 → 131.
- **Iter-17** `436cb4f` — PS.6 chunk #6 tail coverage: BearerToken
  failures + ledger error paths + redaction patterns (gho_/xox-) +
  citations-resolver edge cases. Tests 131 → 143 (+12; original
  sub-agent report over-counted to 153 — corrected in iter-18).
- **Iter-18** `93718c2` — ARCHITECTURE.md + DEVELOPMENT.md refresh
  (doc-only). Both files synced with shipped reality; v0.1.58 Research-
  Trail frontmatter added; stale framings dropped (B5-pending,
  AS-2-pending, mistralrs, Cloud Run, SkyPilot, OCI Artifact, etc.);
  cites contract doc v0.2.0.

**Verified test count**: 143/143 (slm-core 14 + slm-doorman 85 +
audit_endpoints_integration 4 + http_test 40).

**Iter-13 + Iter-14 outcomes:**
- Iter-13 cluster `5812501` — SLM_AUDIT_DIR env var wired in
  slm-doorman-server::main.rs (env present → create_dir_all → ledger;
  failure → warn + fallback to default; startup info! line).
- Iter-14 workspace `278b4ab` — mistral cleanup tail (CUSTOMER-RUNBOOK.es.md
  + tofu/README.md). Zero `mistral` hits remain in slm-yoyo subtree.

**Combined session totals (2026-04-28)**: 14 iterations, 26 commits
(20 cluster + 6 workspace admin-tier ps-administrator-signed), +50 tests
(74 → 124), 19+ AS-5 corpus events. Cumulative Master sweep absorbed:
v0.1.57 COMPONENT-pipeline ack, v0.1.58 Research-Trail ack, v0.1.59
ratification + option-A action items.

**Tests:** 124/124 passing. Last code commit `e4cb8a8` (PS.4 step 5 —
integration tests + cross-cluster contract doc).

**Pipeline summary** (operator-directed, 2026-04-28):
- 9 iterations / 19 commits / +50 tests (74 → 124)
- PS.3 sequence (iter 1-4): three-tier grammar policy + Doorman-side
  Lark fail-fast validation. 5 commits, +23 tests.
- PS.4 sequence (iter 5-9): cross-cluster audit substrate (audit_proxy
  + audit_capture endpoints + contract doc). 5 commits, +27 tests.
- 9 state-file commits per iteration.
- AS-5 shadow brief fired every commit → apprenticeship corpus
  accumulated 19 JSONL events.

**Cross-cluster shipping surface ready**:
- `service-slm/docs/audit-endpoints-contract.md` v0.1.0 — wire contract
  for project-language A-4 + project-data A-5.
- `slm-core` exports: `AuditProxyRequest/Response/Usage`,
  `AuditCaptureRequest/Response`, `ChatMessage`, `GrammarConstraint`.
- Endpoints live: `POST /v1/chat/completions`, `POST /v1/audit/proxy`,
  `POST /v1/audit/capture`. All with mock-only test coverage per B4
  operator guardrail.

**audit_proxy now functional (against mocks)**: handler validates →
writes stub ledger entry → calls AuditProxyClient.relay() →
Anthropic/Gemini/OpenAI HTTP via raw `reqwest` (mockable) → writes
final ledger entry with token counts + cost + latency + status →
returns 200 with `AuditProxyResponse`. Two-entry ledger preserves
audit trail of attempted calls even on upstream failure. New
`AuditProxyProviderUnavailable` → 503 when provider unconfigured at
startup.

**Provider env-var contract** reuses existing `SLM_TIER_C_*` namespace
(per B4 / `ExternalTierClient`); main.rs builds the client only when
at least one provider has both endpoint + key.

**Pipeline operator-directed dispatch sequence:**
1. **PS.3 step 2** — Yo-Yo grammar serialisation ✅ `266fa4d`
2. **PS.3 step 3** — Tier A reject Lark, pass GBNF/JsonSchema ✅ `9f9f37b`
3. **PS.3 step 4** — Tier C reject all grammar variants ✅ `fdee78f`
4. **PS.3 step 5** — llguidance Doorman-side Lark validation ✅ `978ab79`
5. **PS.4 step 1** — audit_proxy endpoint scaffold ✅ `40dc18e`
6. **PS.4 step 2** — audit_proxy upstream provider relay ✅ `028c411`
7. **PS.4 step 3** — purpose allowlist enforcement ✅ `acee9f7`
8. **PS.4 step 4** — audit_capture endpoint scaffold ✅ `36d4fab`
9. **PS.4 step 5** — integration tests + cross-cluster contract doc ✅ `e4cb8a8`

**PS.4 CLOSED.** PS.3 + PS.4 both complete. Pipeline session ends.

## Next pipeline restart points (require operator + Master input)

**Master-blocked** (cluster cannot dispatch):
- **D4** image-build pipeline → unblocks PS.1-5, PS.2, Yo-Yo MIN deploy
- **B7** Doorman redeploy with `SLM_APPRENTICESHIP_ENABLED=true` →
  apprenticeship corpus starts feeding from production traffic
- **Layer-scope clarification** (outbox 2026-04-28T02:30Z) → unblocks
  PS.1-2/-3/-4 + PS.8 (workspace-repo edits via admin-tier procedure)

**Threshold-blocked** (depends on accumulation):
- **PS.5** (graduate task-types to service-slm-first production
  routing) — depends on B7 + apprenticeship corpus reaching tuning
  threshold

**Cluster-Task dispatchable but lower-leverage** (if no Master input
arrives):
- Add explicit `entry_type` discriminator to ledger entries (deferred
  from PS.4 step 5; MINOR contract bump). Hardening sweep.
- Substance authoring on the three pre-v0.1.58 TOPIC skeletons
  (probably premature; substance should follow cluster milestones).

**Doorman grammar substrate — full picture post-PS.3:**
- Tier A (local llama-server): GBNF + JsonSchema native; Lark → 400.
- Tier B (Yo-Yo / vLLM): all three via `extra_body.structured_outputs`
  envelope (vLLM ≥0.12).
- Tier C (external API): all three rejected → 400.
- Doorman-side fail-fast: malformed Lark caught upstream of Tier B
  relay via llguidance compile (`MalformedLarkGrammar { reason }` →
  400 with line/col diagnostics; ~1ms/call; opt-out via
  `SLM_LARK_VALIDATION_ENABLED=false`).

Three new typed error variants this round (`TierAGrammarUnsupported`,
`TierCGrammarUnsupported`, `MalformedLarkGrammar`) — all → 400 +
`PolicyDenied`. Symmetric pattern.

**Master messages archived during iterations 1+4** (no blocking action):
- v0.1.57 COMPONENT-* draft pipeline (no UI work in flight)
- v0.1.58 Research-Trail Substrate (no backfill of pre-v0.1.58 drafts)

Both acked via outbox.

**Deliberately skipped (layer-scope pending Master):**
- PS.8 (guide-doorman cross-repo handoff)
- PS.1-2/-3/-4 (Yo-Yo module + doc updates)

All four sit at workspace-repo / infrastructure path; outbox
2026-04-28T02:30Z still awaiting Master reply. Pipeline excludes them.

**Awaiting Master input (10 messages in outbox):**

- **PS.1-1 finding** (high priority) — D4 image-build pipeline
  is upstream blocker for all Yo-Yo deploy work; nginx TLS layer
  absent from spec; CUSTOMER-RUNBOOK.md added to PS.1-3 rename
  scope
- **PS.1-2/-3/-4 layer-scope** — workspace-repo files; three
  resolution paths proposed (delegate, take as Master, or
  hybrid); cluster lean: delegate
- **SSH-perm regression** — third occurrence today; staging-tier
  keys keep reverting from 0600 to 0640; four recommendations
  (audit jennifer-user processes; umask 077; perm assertion in
  commit-as-next.sh; document chmod-600 floor in CLAUDE.md §3)

**Workspace-tier blocked (Master scope):**

- **D4** image-build pipeline — gates Yo-Yo MIN deploy + PS.2 +
  PS.1-5
- **B7** Doorman redeploy with `SLM_APPRENTICESHIP_ENABLED=true`
  on workspace VM (AS-5 helpers may have landed — observed by
  shadow-brief firing on every commit this session)
- **PS.2** Multi-LoRA + structured-outputs verification — needs
  Yo-Yo running

## Critical sequence (revised post-PS.1-1)

1. **D4** (Master) → create `pointsav-public` GCP project + build
   first slm-yoyo image (vLLM ≥0.12 + nginx TLS + CUDA + Ubuntu 24.04)
2. **Yo-Yo MIN deploy** (operator-gate; Master orchestrates) —
   only after D4
3. **PS.2** (Sonnet test, ~2hr) — verify multi-LoRA +
   structured-outputs combo on the live Yo-Yo
4. **PS.1-5** (Sonnet, ~30 min) — kill-switch first-time-run
   verification
5. Yo-Yo path opens; PS.5 (P1 production routing) eligible once
   corpus accumulates threshold

**In parallel with the Yo-Yo path** (no Yo-Yo dependency):
- PS.3 steps 2-5 (Doorman tier-client grammar wiring)
- PS.4 (A-1 audit endpoints; gates project-language A-4 +
  project-data A-5)
- PS.8 (guide-doorman cross-repo handoff; bounded ~1hr)

## Cross-cluster dependencies

- A-4 (project-language adapter) DEPENDS ON PS.4
- A-5 (project-data anchor-emitter audit-ledger module-id)
  DEPENDS ON PS.4
- service-language refinement at scale (project-language
  editorial gateway, dominant Doorman load 70-100 drafts/wk
  × 7 clusters × 5 sessions/wk per Doctrine claim #35) waits
  on Tier B (Yo-Yo) + AS-2 to scale beyond hand-refinement

## v0.1.31 / v0.1.33 / v0.1.36 / v0.1.42 ratifications captured

- AS-2 scope correction RATIFIED (Q1: Tier A grammar
  asymmetry accepted; Q2: vLLM ≥0.12 envelope; CONTRACT.md
  MINOR bump 0.0.1→0.1.0). Two consumers per v0.1.31:
  service-proofreader + service-language editorial gateway.
- guide-doorman Q1-Q4 answered (Q1: catalog `local-doorman/`;
  Q2: wire SLM_AUDIT_DIR; Q3: both tenants with operator-
  picks note; Q4: same deployment as local-doorman.service).
- 8-site zero-container drift bundle authorized (4th+5th-pass
  combined; cluster-scope per v0.1.36 correction).
- Three coverage briefs A/B/C ratified (cluster-scope, not
  workspace queue; operator green-lights dispatch).
- Reverse-Funnel Editorial Pattern (Doctrine claim #35);
  drafts-outbound input port at
  `~/Foundry/clones/project-slm/.agent/drafts-outbound/`.

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

- **AS-2 implementation — scope correction pending Master
  ack.** Sonnet research (sub-agent chunk #1, 2026-04-27)
  found the `llguidance` Rust crate is decode-time only and
  has no integration point in our HTTP-relay Doorman shape.
  Corrected scope: thin wire-format adapter — Tier B sends
  Lark grammars in `extra_body.structured_outputs.grammar`
  (vLLM ≥0.12); Tier A accepts only GBNF / JSON Schema (not
  Lark) per llama-server HTTP API; Tier C no grammar; optional
  llguidance dep for Doorman-side Lark validation only.
  Surfaced to Master via outbox 2026-04-27 with two questions
  (Tier A grammar asymmetry; vLLM version target). HOLD all
  code work until Master ack. Note: authoring of the actual
  Lark grammar file (`service-content/schemas/banned-vocab.lark`)
  is project-language Phase 1B scope per Master's 2026-04-27
  v0.1.26 brief — NOT this cluster's work.
- **ARCH/DEVELOPMENT.md zero-container drift FOURTH +
  FIFTH-pass — Master sign-off needed.** Third-pass cleared
  by commit `8c3212e` (2026-04-26); fourth-pass surfaced
  three sites (ARCH §3 line 132 "External calls (Cloud Run,
  ...)", ARCH §5.2 line 197 hyper-crate role "(Cloud Run,
  ...)", DEV §4 Phase 2 step 5 "Port the Cloud Run
  driver"). Sonnet sub-agent audit 2026-04-27 surfaced five
  more sites (fifth-pass): ARCH §2 line 59 Ring 3b memory
  table "OCI Artifacts" (structural; couples with §3b line
  118 "stored as an OCI Artifact" and DEV §2.2 line 122-124
  "OCI Artifacts" in signing description — three coupled
  references to same Ring 3b adapter-storage decision); DEV
  §6 line 237 `cargo-chef` for Docker layer caching
  (prose); DEV §7 line 289 declared workspace dep
  `google-cloud-run = "*"` (structural — would compile-time
  pull Cloud Run client bindings). Eight sites total bundled
  in outbox 2026-04-27 for one Master-authorised prose-edit
  commit (same pattern as 4a eleven-site / third-pass two-
  site bundles). Do not act without authorisation.
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
