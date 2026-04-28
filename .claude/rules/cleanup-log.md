# Cleanup Log — pointsav-monorepo

Living record of in-flight cleanup work, open questions, and decisions made during active development. This file is read at session start and updated at session end when meaningful cleanup occurs. Maintained in-repo so the history travels with the code.

---

## How this file is maintained

- **Read at session start.** Claude Code reads this file at the start of every session (per the instruction in `CLAUDE.md`). The tables below reflect the current state of in-flight work. Apply the guidance before touching any related files.
- **Update at session end.** When a session includes meaningful cleanup — renames across multiple files, deprecated code removal, resolving an open question, surfacing a new one — append a dated entry to the top of the **Session entries** section at the bottom of this file.
- **Do not log trivial edits.** Single-file typo fixes, comment tweaks, or routine formatting changes do not belong here. This log is a record of decisions, not of every keystroke.
- **Commit each update with the code changes it describes.** The log and the work it documents travel together through git history.

---

## Interpreting build signals during cleanup

Until the workspace `Cargo.toml` is unified (see Layer 1 audit findings), `cargo build --workspace` and `cargo check` at the repo root only exercise the 8 declared members. The other ~70 crates are not covered by workspace-level commands. When making changes to any crate outside the declared members, run `cargo check` inside that crate's directory specifically. Do not rely on workspace-root build signals to confirm correctness across the full repo. This caveat lifts when the workspace is unified.

---

## Active legacy-to-canonical renames

These substitutions are known and in progress. Canonical names are from the Nomenclature Matrix. When the last occurrence of a legacy name is removed from the repo, move the row to the **Completed migrations** section with the date of completion.

| Legacy | Canonical | Status | Notes |
|---|---|---|---|
| `service-parser` | `service-extraction` | In flight — both names present | Working name still used in portions of the active codebase. Canonical name is the long-term target. Consolidation planned for a future release. |
| `service-llm` | `service-slm` | Documentation-only inconsistency | Code references are correct. Legacy appearances in docs should be read as `service-slm`. |
| `cluster-totebox-real-property` | `cluster-totebox-property` | In flight | Appears in older deployment manifests and doc references. |
| `os-interface`, `os-integration` | `os-orchestration` | In flight | Legacy names predate the current three-layer stack nomenclature. |
| `RealPropertyArchive` | `PropertyArchive` | In flight | Appears in older archive-type documentation and possibly in legacy code comments. |

---

## Deprecations — flag and remove

Names no longer in use. Any occurrence in the repo should be flagged and removed. If a removal blocks something active, surface it — do not leave the legacy name in place silently.

| Name | Status | Notes |
|---|---|---|
| `fleet-command-authority` | Deprecated — remove | Node no longer in use. Should not appear in any current deployment manifest, build script, or documentation. |

---

## Intentional exceptions — do not migrate

Items that may look like candidates for cleanup but are intentionally preserved as-is. Do not "fix" these without confirmation.

| Item | Rationale |
|---|---|
| `cluster-totebox-personnel-1` and other numbered personnel instances | Exist locally but intentionally absent from GitHub and the MEMO. Not a naming error. Do not flag as legacy. |
| Two ConsoleOS operating patterns (multi-service `node-console-operator` and single-service nodes) | Both patterns are valid. The MEMO documents `node-console-operator` only, by design, to keep official documentation clean. Do not flag the single-service pattern as an inconsistency. |

---

## Open questions

Pending confirmations that affect how Claude should describe or reason about parts of the system. Do not invent values for these. If a task requires an answer, stop and surface the question.

| Question | Current handling |
|---|---|
| Verification Surveyor daily throttle number | Under operational review. Do not cite a specific number. Refer to it as "a system-enforced daily limit" until confirmed in a future MEMO version. |
| User Guide language on Sovereign Data Foundation | The User Guide contains language treating the Foundation as a current equity holder and active auditor. Requires a language review pass before any User Guide content is reused in public-facing materials. Flag any passage that describes the Foundation as current or active. |
| `service-search` inclusion in the next MEMO | Confirmed for inclusion in the next MEMO version. Treat as canonical in code; note the doc catch-up is pending. |
| Is the per-crate independent workspace pattern intentional (some crates meant to be extractable and published separately) or accidental drift? | Pending decision — do not act on related findings until answered. |
| Are `app-console-*` and `app-network-*` directories without `Cargo.toml` intentional scaffolding for planned work, or abandoned attempts? | Pending decision — do not act on related findings until answered. |
| Should the doubly-nested `service-email-egress-{ews,imap}` structure be flattened, or does the nesting reflect a real protocol-implementation hierarchy? | Pending decision — do not act on related findings until answered. |
| What is `discovery-queue` — runtime data that should be gitignored, reference data that belongs elsewhere, or a misplaced crate? | Pending decision — do not act on related findings until answered. |
| Does `vendors-maxmind` (containing a GeoLite2 database, not code) belong as a `vendor-*` crate at all, or should it move to a non-workspace data directory? | Pending decision — do not act on related findings until answered. |

---

## Completed migrations

Migrations fully resolved in the repo. Moved here from **Active legacy-to-canonical renames** when the last occurrence of the legacy name is removed. Empty for now.

*(none yet — move rows here as migrations close out)*

---

## Session entries

Newest on top. Append a dated block when a session includes meaningful cleanup work. Format:

```
## YYYY-MM-DD
- What changed (files touched, counts, rationale)
- What was left pending and why
- New open questions surfaced
```

---

## 2026-04-28 — Long-running Sonnet pipeline activated (operator-directed); iteration 1 = PS.3 step 2

Operator green-light "set up a long running pipeline for Sonnet to run on auto"
2026-04-28. Goal per operator: drive service-slm toward usable-for-coding-and-
writing state and feed apprenticeship corpus via commit cadence; reduce pressure
on Claude usage; service-slm trains pointsav-llm in parallel. Explicit ratification
per `conventions/model-tier-discipline.md` §1A.6 (operator-directed dispatches).

Self-paced via `/loop` skill in dynamic mode (no fixed interval). Each iteration:
read inbox → check git/tests clean → dispatch one cluster-scope Sonnet brief
(foreground+serial) → verify → update state files → commit → ScheduleWakeup for
next iteration.

**In-scope queue (cluster-scope, no Master gate):**
- PS.3 step 2 — Tier B (Yo-Yo) grammar serialisation **(this iteration)**
- PS.3 step 3 — Tier A reject Lark, pass GBNF/JsonSchema
- PS.3 step 4 — Tier C reject all grammar variants
- PS.3 step 5 — `llguidance` Doorman-side Lark validation
- PS.4 step 1..N — A-1 audit_proxy + audit_capture endpoints (multi-step;
  cross-cluster gate for project-language A-4 + project-data A-5)

**Deliberately skipped (layer-scope pending Master clarification):**
- PS.8, PS.1-2/-3/-4 — workspace-repo files; outbox 2026-04-28T02:30Z still
  awaiting reply.

**Workspace-tier blocked (Master scope, can't unblock from cluster):**
- D4 image-build pipeline (gates Yo-Yo MIN + PS.2 + PS.1-5)
- B7 Doorman redeploy with `SLM_APPRENTICESHIP_ENABLED=true`

### Iteration 1 outcome — PS.3 step 2 — Yo-Yo client grammar serialisation

- **Commit**: `266fa4d` (Peter Woodfine)
- **Tests**: 79 → 83. Four new wiremock tests in `tier::yoyo::tests`:
  Lark / GBNF / JsonSchema / None.
- **Wire envelope**: vLLM ≥0.12 `extra_body.structured_outputs.{grammar, json_schema}`
  per v0.1.33 Q2 ratification. Lark and GBNF both serialise to the same `grammar`
  field; vLLM's llguidance backend auto-detects format. JsonSchema lands on the
  `json_schema` sibling. `None` omits the envelope entirely (no empty objects).
- **Build hygiene**: cargo test clean; clippy `-D warnings` clean; fmt clean.
- **No layer-scope concerns** raised by the Sonnet agent.
- Wall time: ~3.5 minutes; ~100k Sonnet tokens.

### Master message archived during iteration

Master inbox message at 2026-04-28T17:09Z: workspace v0.1.57 ratified
`conventions/cluster-design-draft-pipeline.md` (COMPONENT-* draft pipeline). No
immediate action — message explicitly notes "clusters with no UI surface skip
cleanly" and project-slm has no UI work in flight. Acknowledgment added to
outbox 2026-04-28T17:30Z; original archived to `inbox-archive.md`. The five
forward-looking UI surfaces in the message (tier-routing dashboard, audit-ledger
viewer, adapter chain inspector, API-key rotation panels, cost-tier chips) are
recorded for whenever a future cluster milestone introduces a UI surface; most
plausible first candidate is the audit-ledger viewer once PS.4 lands and
project-language consumes the proxy.

### Pipeline continues

Next iteration: PS.3 step 3 (Tier A reject Lark, pass GBNF/JsonSchema natively).
Self-pacing via ScheduleWakeup; will resume once this iteration's commit is
landed.

### Iteration 2 outcome — PS.3 step 3 — Tier A grammar handling

- **Commit**: `9f9f37b` (Peter Woodfine)
- **Tests**: 83 → 87. Four new tests in `tier::local::tests`:
  None / GBNF / JsonSchema / Lark→error.
- **Wire fields**: GBNF serialises to top-level `grammar` (llama-server's
  native field). JsonSchema serialises to top-level `json_schema`. Lark
  rejected at the Doorman boundary BEFORE any network call (test asserts
  wiremock server received zero requests).
- **New error variant**: `DoormanError::TierAGrammarUnsupported { dialect,
  advice }`. HTTP mapping: 400 BAD_REQUEST. `CompletionStatus::PolicyDenied`.
  Per v0.1.33 Q1 ratification: Tier A grammar asymmetry accepted — apprentice
  on Tier A produces unconstrained output OR uses GBNF/JsonSchema only;
  Lark enforced only when escalated to Tier B (Yo-Yo via llguidance).
- **Build hygiene**: cargo test 87/87; clippy `-D warnings` clean; fmt clean.
- **No layer-scope concerns**.
- **Wall time**: ~4 minutes; ~95k Sonnet tokens.
- **Surprise**: router.rs `classify_error()` and the slm-doorman-server
  `tests/http_test.rs::doorman_error_to_status` mirror match both required
  updates (exhaustive matches on the `DoormanError` enum). Sonnet caught and
  fixed both cleanly. The mirror-match pattern in test code is worth
  documenting — it's a maintenance burden when adding new error variants.

### Pipeline continues — iteration 3

Next: PS.3 step 4 (Tier C reject all grammar variants — smallest chunk;
~30 min Sonnet).

---

## 2026-04-28 — Sonnet batch wrap-up (PS.7 + A/B/C + layer-scope flag) — 5 commits, +19 tests

Operator green-light "set it up to do all the recommendations"
2026-04-28. Five foreground-serial Sonnet sub-agent dispatches
in sequence (each blocking the next per §1A.2 git-index race
discipline). Workspace tests 55 → 74 (+19).

Order landed:

1. **PS.7** `472e44a` — 8 zero-container drift edits in
   service-slm/ARCH+DEV.md. Doc-only.
2. **Layer-scope flag** `962c329` — pre-dispatch sweep on
   PS.1-2/-3/-4 found workspace-repo file paths. Per
   CLAUDE.md §11 action matrix, infrastructure/ is
   Master-tier. Three queue entries marked LAYER-SCOPE
   PENDING; surfaced via outbox 2026-04-28T02:30Z.
3. **Brief A** `d9ea19d` + `35a0c64` — http.rs test factory
   + 12 tests (4 smoke + 5 error-mapping + 3
   apprenticeship-disabled). Structural change: slm-doorman-
   server gained `src/lib.rs` with `pub mod http` +
   `pub mod test_helpers` for integration-test imports.
4. **Brief B** `97f360e` — tier/local.rs unit tests (5)
   modeled on yoyo wiremock pattern.
5. **Brief C** `5087a2c` — VerdictDispatcher Reject +
   DeferTierC tests (2). Reject DOES produce DPO pair
   (matches Refine); DeferTierC does NOT (escalation not
   refinement).

### Layer-scope status

PS.1-2 / PS.1-3 / PS.1-4 NOT executed — workspace-repo
files. Master clarification pending. The recommendation
list I gave the operator included these without checking
file paths; queueing-discipline error to flag.

### SSH-perm regression — third occurrence

Three sub-agent runs found keys reverted from 0600 to 0640
between commits. Both Jennifer + Peter regress
simultaneously. Sub-agents applied chmod 600 as workaround.
Surfaced to Master via outbox 2026-04-28T03:30Z with four
recommendations (audit jennifer-user processes; umask 077;
perm assertion in commit-as-next.sh; document chmod-600
floor in CLAUDE.md §3).

### PS.6 (task #14) closed; #13 (PS.7) closed

All three coverage briefs landed. Cluster sub-agent-queue
entries A/B/C marked COMPLETED with commit refs + outcomes.
Queue still carries PS.1-2/-3/-4 (LAYER-SCOPE PENDING) and
PS.1-5 (BLOCKED on D4). Coverage-track is done; PS.3 / PS.4
/ PS.8 remain as the major dispatchable cluster work.

### AS-5 trajectory captures

Each commit fired AS-5 shadow brief dispatch — apprenticeship
corpus capturing live. No commit-failure interruptions today
(workaround held).

---

## 2026-04-28 — PS.7 4th+5th-pass zero-container drift (8 sites, 2 files)

- Applied 8 prose edits to `service-slm/ARCHITECTURE.md` and
  `service-slm/DEVELOPMENT.md` per Master v0.1.33 §C bundle
  authorisation + v0.1.36 framing correction + operator green-light.
- **4th-pass (3 sites):**
  - ARCH §3 line 132: "Cloud Run," → "GCE Yo-Yo instances,"
  - ARCH §5.2 line 197: `hyper` crate role "Cloud Run" → "Yo-Yo GCE endpoints"
  - DEV §4 Phase 2 step 5: "Port the Cloud Run driver" → "Port the GCE compute driver … per `infrastructure/slm-yoyo/tofu/`"
- **5th-pass (5 sites):**
  - ARCH §2 Ring 3b table storage cell: "OCI Artifacts" → "GCS-archived (signed, SLSA-attested)"
  - ARCH §3b para: "stored as an OCI Artifact (Sigstore-signed, SLSA-attested)" → "stored as a GCS object (Sigstore-signed via the sigstore crate, SLSA-attested)"
  - DEV §2.2: "(Ring 3b, OCI Artifacts)" → "(Ring 3b, GCS-stored adapters)"
  - DEV §6 build-time risks table: dropped `cargo-chef` Docker layer caching mention; kept `sccache`
  - DEV §7 workspace deps: `google-cloud-run = "*"` → `google-cloud-compute = "*"` (GCE start/stop ceremony crate)
- `cargo check --workspace` clean post-edit (Finished, no errors/warnings).
- No open questions surfaced. Sonnet's fifth-pass judgment ("substantially clean") stands.

---

## 2026-04-28 — PS.1-1 image verification dispatched + 12th blocker D4 surfaced

Operator green-lit dispatch of PS.1-1 (image verification)
2026-04-28 post-Tetrad-housekeeping. Sonnet sub-agent
foreground; ~30 min wall, ~70k tokens. Major finding:

### Headline: pointsav-public project does not exist

Two independent gcloud probes (`compute images list
--project=pointsav-public`,
`compute images describe-from-family slm-yoyo
--project=pointsav-public`) return `The resource
'projects/pointsav-public' was not found` — not
permissions; the project has never been created.
Workspace SA confirmed active with cloud-platform scope
on `woodfine-node-gcp-free` (the one project visible).

The slm-yoyo `tofu/README.md` "PointSav GCE image
versions" table corroborates: `slm-yoyo | First seen:
pending — first build via Task D4`. Task D4 has not been
dispatched. Image-build pipeline source is not in the
workspace (`find` for `*.pkr.hcl` / `packer.json` /
`build-image*` returned nothing).

### D4 surfaces as 12th blocker upstream of all PS.1 items

`tofu apply` fails immediately at the
`data "google_compute_image" "yoyo"` lookup regardless
of what comes after. D4 is workspace-tier scope per
CLAUDE.md §11 (Master executes; Task flags). Master
needs to: (1) create `pointsav-public` GCP project; (2)
author / restore / locate the image-build pipeline; (3)
build image with vLLM ≥0.12 + nginx TLS terminator +
Let's Encrypt + idle-shutdown timer + systemd unit + CUDA
+ Ubuntu 24.04; (4) publish to slm-yoyo family; (5) IAM
binding for customer image-read.

### Cluster sub-agent-queue.md updated

- PS.1-1 marked **COMPLETED** with outcome ref.
- PS.1-3 scope **EXPANDED** to also cover
  `CUSTOMER-RUNBOOK.md` lines 29 + 194-209 (`systemctl
  status mistralrs`, `/var/lib/mistralrs/weights/`,
  `mistralrs-idle.timer`); version-pin caveat added (do
  not pin patch; "vLLM ≥0.12" floor only).
- PS.1-5 marked **BLOCKED on D4** (kill-switch
  verification needs `tofu apply` working).
- Coverage A/B/C unaffected; PS.1-2 / PS.1-3 / PS.1-4
  still dispatchable.

### Adjacent finding — nginx absent from spec

Master's v0.1.42 §W4 ack assumed image ships nginx with
Let's Encrypt cert. PS.1-1 finds **no nginx mention in
any current slm-yoyo artefact** (variables.tf,
CUSTOMER-RUNBOOK.md, CONTRACT.md, tofu/README.md). Only
nginx in workspace is `local-proofreader` /
`local-knowledge`. nginx layer needs design pass before
D4's image build.

### Adjacent finding — broader rename scope than B4

Master's §B4 ack named CONTRACT.md + variables.tf for the
mistral.rs → vLLM rename. PS.1-1 found mistral.rs naming
also in CUSTOMER-RUNBOOK.md (3 sites). Folded into PS.1-3
expanded scope. systemd unit names + weight paths are
config-set-by-image-builder (D4) — no current files to
rename for those.

### State after dispatch

- Tasks: #18 (PS.2) marked blocked-on-D4. New #23 (D4
  Master-tier flagging) added. PS.1-1 finding doesn't
  warrant own task — outcome captured in
  `sub-agent-queue.md` Completed section + this log.
- Outbox: PS.1-1 finding outbox message + Tetrad
  confirmation = 9 messages awaiting Master pickup.
- Inbox: empty.
- No code changes; tests still 46/46.

### What dispatchable next under operator green-light

- **PS.1-2** (B1+B2+W1 module update; no image dependency)
- **PS.1-3** (mistral.rs → vLLM doc rename, expanded scope,
  no patch pin)
- **PS.1-4** (local-doorman.env output snippet)
- **A** (http.rs test factory) → then **B** + **C** in
  parallel (coverage briefs; cluster-internal; no Yo-Yo
  dependency)
- **PS.3** (AS-2 wire-format adapter; ~1-2 weeks Sonnet;
  Doorman side; mock-tested)
- **PS.4** (A-1 audit endpoints; ~3-5 days Sonnet; Doorman
  side)
- **PS.8** (GUIDE-doorman cross-repo handoff; ~1 hour
  Opus + Sonnet; bounded)

The operationalization plan's critical sequence shifts:
without the Yo-Yo deploy path, the parallel paths
(Doorman-side AS-2/A-1 + cluster tests + GUIDE handoff)
become higher-leverage until Master ships D4.

---

## 2026-04-28 — Tetrad upgrade + PS.1 ack housekeeping

Single-pass housekeeping for the two Master inbox messages
arriving overnight: the Tetrad Discipline upgrade (Doctrine
v0.0.10 / claim #37) and the PS.1 readiness review ack.

### Tetrad upgrade — required actions all completed

- **Read** `conventions/project-tetrad-discipline.md` — ratified
  2026-04-28 under doctrine v0.0.10 — fourth structural leg
  (wiki TOPIC contribution to `vendor/content-wiki-documentation`)
  added to the existing vendor + customer + deployment Triad.
- **Manifest amended** at `.claude/manifest.md`: rename
  `triad:` → `tetrad:`; new `wiki:` leg block declares
  drafts_via path, project-language gateway, three planned
  TOPIC priorities, status `leg-pending` (substance lands
  as cluster milestones progress).
- **Three TOPIC skeletons + Spanish pairs staged** in
  `.claude/drafts-outbound/` (six files total):
  - `topic-doorman-protocol.md` + `.es.md` — Doorman as
    security boundary + three-tier compute routing pattern.
  - `topic-apprenticeship-substrate.md` + `.es.md` — Doctrine
    claim #32 originated this cluster; cited as workspace-wide
    precedent for sub-agent-as-tier-discipline at v0.1.30.
  - `topic-zero-container-inference.md` + `.es.md` — SMB GPU
    economics + idle-shutdown pattern; BCSC class
    forward-looking until Yo-Yo MIN deploys.
- All six skeletons carry `foundry-draft-v1` frontmatter
  per convention; section headings + `(draft-pending —
  substance follows in milestone N+1)` markers.
- Outbox confirmation message to Master sent (optional but
  encouraged per brief).

### PS.1 ack — 4 blockers + 7 warnings called by Master

All called; sub-agent dispatch pre-authorised under operator
green-light:

- **B1 preemptible**: add `variable "preemptible" { default
  = false }`; use `provisioning_model = SPOT/STANDARD`;
  flip `automatic_restart = !var.preemptible`.
- **B2 A100 quota**: extend `null_resource.gpu_quota_request`
  for `NVIDIA_A100_GPUS_per-region` (40GB) or
  `NVIDIA_A100_80GB_GPUS_per-region` (80GB) per gpu_class.
- **B3 image existence**: own sub-agent verification brief
  BEFORE B4. Quick `gcloud compute images list`.
- **B4 vLLM (authoritative call)**: per v0.1.33 Q2; mistral.rs
  framing in CONTRACT.md + variables.tf is stale. Update
  CONTRACT.md + variables.tf to name vLLM.
- **W1 cost-math**: bundle into B1 with both on-demand + Spot
  prices per gpu_class.
- **W2 gcloud beta GA**: test on workspace VM at brief
  landing time; fall back to GA path if beta drops.
- **W3 idle-shutdown wins**: drop "30-min daily window"
  framing; idle-shutdown is the correct shape.
- **W4 nginx in image**: GCE image must terminate TLS via
  nginx + Let's Encrypt cert keyed to static IP reverse-DNS;
  endpoint URL stays HTTPS. **Never HTTP-on-the-wire across
  a public network** — structural rule.
- **W5 firewall default**: module default 0.0.0.0/0 stays for
  SMB; tighten via deployment-instance vars for workspace
  dogfood. Document in CUSTOMER-RUNBOOK §"Hardening for
  static-IP operators".
- **W6 local-doorman.env output snippet**: yes; sub-agent
  brief candidate.
- **W7 kill-switch first-run verification**: standalone
  sub-agent brief OR PS.2 prefix.

### Sub-agent-queue.md created at cluster level

Per Master's instruction "write them into your cluster's
.claude/sub-agent-queue.md and dispatch when operator
green-lights". Eight ratified briefs entered:

- **A/B/C** — three coverage briefs (PS.6 in v0.1.42 plan):
  http.rs test factory + smoke (~3-4hr); tier/local.rs unit
  tests (~1-2hr); VerdictDispatcher Reject/DeferTierC
  (~1hr). A first (factory dependency); B/C independent.
- **PS.1-1..5** — five PS.1 follow-ups: image verification
  (must-first); module update for B1+B2+W1; B4 doc rename
  mistral.rs→vLLM; local-doorman.env output snippet;
  kill-switch first-run verification.

Suggested dispatch sequence per Master: PS.1-1 → PS.1-2 →
PS.1-3 → PS.1-4 → PS.1-5; coverage A/B/C parallel-able.

Yo-Yo MIN deploy itself stays gated per operator direction
("wait on launching the Yo-Yo until we have more of the
coding in place"). All sub-agent dispatches above are prep
work, not deployment work.

### State after housekeeping

- Inbox: empty (placeholder reset).
- Outbox: 7 messages from prior session + new Tetrad
  confirmation = 8 messages awaiting Master pickup.
- Tasks: #15 + #17 closed; new tasks for Tetrad upgrade
  and queue creation (will be added to local task list);
  20 total (15 active).
- New file: `.claude/drafts-outbound/` with 6 TOPIC skeletons.
- New file: `.claude/sub-agent-queue.md` with 8 briefs.
- No code changes; tests still 46/46.
- Working tree clean post-commit.

---

## 2026-04-27 — PS.1 Yo-Yo deploy readiness review (Opus judgment, ~30 min)

Read every file in `infrastructure/slm-yoyo/tofu/`
end-to-end + CONTRACT.md + tofu/README.md. Module authored
2026-04-25; no post-authoring commits. Surfaced 4 blockers
and 7 warnings to Master via outbox 2026-04-27T23:30Z.

### Blockers (apply will fail or produce wrong shape)

- **B1**: `preemptible = false` hard-coded in compute.tf
  line 40; PS.1 brief specifies preemptible MIN. Cost
  diverges 5× (target $7-8/mo → actual ~$50/mo).
- **B2**: `quota.tf` requests only `GPUS-ALL-REGIONS-per-project`;
  A100 deploy needs `NVIDIA_A100_GPUS_per-region` (40GB) or
  `NVIDIA_A100_80GB_GPUS_per-region` separately.
- **B3**: `pointsav-public:slm-yoyo` GCE image existence
  unverified; lookup fails apply if image not published.
- **B4**: vLLM vs mistral.rs runtime mismatch — CONTRACT.md
  + variables.tf describe mistral.rs; PS.2 brief specifies
  vLLM flags. Resolve before PS.2.

### Warnings (deploy succeeds but operational concerns)

- **W1**: variables.tf cost-math drift (on-demand prices in
  doc vs preemptible prices in PS.1 brief).
- **W2**: `gcloud beta quotas` may have moved to GA; test
  before relying on auto-request.
- **W3**: PS.1 brief "30-min daily window" semantics
  mismatch with module's idle-shutdown-on-inactivity
  pattern; pick one.
- **W4**: `https://${IP}:${PORT}` in outputs.tf, but
  mistral.rs/vLLM don't terminate TLS by default; either
  image has nginx (undocumented) or URL should be `http://`.
- **W5**: `doorman_ip_cidrs = ["0.0.0.0/0"]` open-internet
  default; tighten to `/32` for workspace VM dogfood.
- **W6**: Operator hand-stitches Doorman config from
  outputs; a `local-doorman.env` output snippet would close
  the deploy → Doorman-config gap.
- **W7**: Kill-switch Cloud Function source is dynamic
  archive; first-time end-to-end run worth a separate
  verification brief.

### Structurally sound

Versions pinned and current; IAM minimum-viable; budget cap
+ kill-switch defense-in-depth solid; static external IP for
endpoint stability; `desired_status = TERMINATED` +
`lifecycle.ignore_changes` correctly models on-demand
pattern; secrets pre-provisioned in Secret Manager;
service-account scopes match CLAUDE.md §3 GCP identity model.

### Sub-agent brief candidates surfaced

- (1) Module update for B1+B2 (Sonnet ~1-2hr; bounded; no
  apply).
- (2) `local-doorman.env` output snippet (Sonnet ~30 min).
- (3) B4 runtime-resolution research (Sonnet ~30 min).

### Recommended sequence

1. Resolve B4 first (PS.2 target is undefined without).
2. Resolve B3 in parallel (image existence).
3. Resolve B1+B2 as one module update.
4. Address W3+W4 (Master ratification calls).
5. Test apply with `monthly_cap_usd=10` to prove
   kill-switch before MIN.

### State after PS.1

Task #17 closed. Outbox carries PS.1 readiness review as
high-priority message. Yo-Yo MIN deploy gated on Master
+ operator answering B1-B4 + W3-W4. PS.2 (multi-LoRA +
structured-outputs verification) gated specifically on B4.

---

## 2026-04-27 — Master ratification cascade (v0.1.31 / v0.1.33 / v0.1.36 / v0.1.42)

Single-pass housekeeping: archived 5 inbox messages from
Master in chronological order; reset placeholder; updated
existing tasks #1/#4/#13/#14/#15/#16; added new tasks #17-#20
for SLM operationalization plan items PS.1/PS.2/PS.4/PS.5.

### v0.1.31 (18:55Z) — Reverse-Funnel Editorial Pattern

Doctrine claim #35 ratified. Cluster Tasks no longer
self-refine wiki content; ship bulk drafts forward to
project-language (editorial gateway). New input port at
`~/Foundry/clones/project-slm/.claude/drafts-outbound/`.
Frontmatter contract: `foundry-draft-v1`. project-language
enforces register / banned-vocab / BCSC / bilingual /
citation-ID resolution; cluster authors author bulk content
only. Apprenticeship corpus emits JSONL `draft-created`
event; project-language emits `draft-refined`; originating
cluster emits `creative-edited` on Creative Contributor edit.
Tasks have explicit write permission to
`~/Foundry/data/training-corpus/apprenticeship/prose-edit/<tenant>/<draft-id>.jsonl`
per CLAUDE.md §11 v0.1.31 amendment.

### v0.1.31 (19:00Z) — AS-2 second consumer

service-language editorial gateway is the second primary
AS-2 consumer (alongside service-proofreader). Volume:
70-100 drafts/week × 7 clusters × 5 sessions/week = dominant
Doorman-mediated load once project-language Task starts
sweeping. Per-request grammar passing accommodates both
consumers trivially; no design change anticipated.

### v0.1.33 (19:55Z) — BIG ACK — four tracks ratified

(A) **AS-2 scope correction RATIFIED.** Sonnet finding
right; corrected scope (wire-format adapter, not crate
integration) right; 1-2 weeks realistic. Q1: accept Tier A
grammar asymmetry — apprentice on Tier A unconstrained;
Lark grammars are EDITORIAL floor on Tier B (per Doctrine
claim #35). Q2: pin to vLLM ≥0.12 envelope
(`extra_body.structured_outputs.grammar`); CONTRACT.md
MINOR bump 0.0.1 → 0.1.0.

(B) **GUIDE-doorman Q1-Q4 answered.** Q1: catalog name
`local-doorman/` (matches existing
`infrastructure/local-doorman/` + running
`local-doorman.service` unit; symmetric with `local-fs/`,
`local-proofreader/`, `local-knowledge/` precedents). Q2:
wire `SLM_AUDIT_DIR` in slm-doorman-server::main.rs (~10
lines; default `/var/lib/slm-doorman/audit/` per unit;
multi-instance override-friendly). Q3: GUIDE shows both
tenant defaults with operator-picks-per-deployment note.
Q4: same deployment as `local-doorman.service` — unit name
throughout GUIDE is `local-doorman.service`. Refined draft
go-ahead: apply Q1-Q4 answers; cross-repo handoff via
outbox mechanism per CLAUDE.md §11 to
`customer/woodfine-fleet-deployment/local-doorman/GUIDE-doorman-deployment.md`.

(C) **5th-pass drift bundle authorized** — initially framed
Master-scope (corrected in v0.1.36 to cluster-scope).

(D) **Three sub-agent briefs A/B/C RATIFIED.** Pass §1A
confidence gate. Cluster-scope so not in workspace queue.
Dispatch authority: operator green-light to this Task
session via Agent tool with `model:"sonnet"`. A first
(factory dependency); B+C independent after; foreground+
serial per §1A rule 2.

### v0.1.36 (20:35Z) — CLUSTER scope correction

Correction to v0.1.33 §C framing. The 8 zero-container drift
sites (3 from 4th-pass + 5 from 5th-pass) live in
`service-slm/ARCHITECTURE.md` + `DEVELOPMENT.md` — files
inside this cluster's clone. Master editing them at
workspace tier crosses layer scope per CLAUDE.md §11 action
matrix. Bundle stays pre-authorized; cluster Task dispatches
the prose-edit when operator green-lights, with the per-site
replacement text from earlier outbox messages.

### v0.1.42 (22:50Z) — SLM OPERATIONALIZATION PLAN

`conventions/service-slm-operationalization-plan.md` ratified.
This cluster on critical path. Healing-effect framing: once
service-slm contributes alongside Claude, errors heal via
verdict signing → corpus → continued LoRA training loop.
Sonnet output today is acceptable because the loop heals it
tomorrow. **Prioritize Sonnet over Opus on bulk work.** Opus
stays for architectural decisions.

Eight items (PS.1..PS.8) prioritized:
- PS.1 (Opus, ~30 min, GATE) — Yo-Yo deploy readiness
- PS.2 (Sonnet, ~2hr) — Multi-LoRA + structured-outputs
  verification on Yo-Yo (resolves Risk 1)
- PS.3 (Sonnet, ~1-2 weeks) — AS-2 wire-format adapter
- PS.4 (Sonnet, ~3-5 days) — A-1 Doorman audit_proxy +
  audit_capture endpoints (parallel with PS.3)
- PS.5 (Sonnet, ~1 week) — AS-6/AS-7 P1 production routing
  on version-bump-manifest task type
- PS.6 (Sonnet × 3, ~9-12hr total) — three coverage briefs
- PS.7 (Sonnet, ~30 min) — 4th+5th-pass prose-edit
- PS.8 (Opus + Sonnet, ~1 hour) — GUIDE-doorman handoff

Critical sequence: PS.1 → Yo-Yo MIN deploy → PS.2 → PS.4
parallel → PS.3 → PS.5. Yo-Yo MIN: A100 80GB preemptible
(~$0.50-0.70/hr); 30-min daily window initially → ~$7-8/month;
fixed UTC hour (e.g., 02:00 UTC off-peak); quality gate
project-language verdict accept-rate ≥0.6 over rolling 50 →
continue, below → abort.

### Cross-cluster dependencies (recorded)

- A-4 (project-language adapter) depends on PS.4
- A-5 (project-data anchor-emitter audit-ledger module-id)
  depends on PS.4
- service-language refinement at scale waits on Tier B
  (Yo-Yo) + AS-2 to scale beyond hand-refinement

### Task list state

Updated #1 (PS.3), #4 (PS.8), #13 (PS.7), #14 (PS.6), #16
(folded into PS.3 step 6). Closed #15 (GUIDE refinement Q1-Q4
answered). Added #17 (PS.1), #18 (PS.2), #19 (PS.4),
#20 (PS.5). Outbox has 6 messages from prior session, all
acked by Master in v0.1.33/v0.1.36; migration to
outbox-archive deferred to next housekeeping pass.

### NEXT.md rewritten

Right-now section captures the v0.1.42 plan; Critical sequence
explicit; Cross-cluster dependencies listed. Pre-authorized
for operator green-light: PS.7 (fastest), PS.6 (highest test-
coverage value). PS.1 startable in this Opus session; not
sub-agent-deferrable.

### No code changes

Tests still 46/46. Working tree clean post-commit.

---

## 2026-04-27 — v0.1.30 codifies sub-agent-as-tier-discipline (this cluster cited as operational precedent)

- **Master message archived (workspace v0.1.30, 2026-04-27T17:00:00Z).**
  Informational; no action required. Inbox placeholder reset.
- **Behavioural change for future sessions in this cluster.** The
  exit+re-enter pattern in `conventions/model-tier-discipline.md`
  §1 is now deprecated for tier-discipline purposes
  (operator-elective only — e.g., operator wants to converse with
  a different model directly). Sub-agent dispatch via the Agent
  tool is THE tier-discipline mechanism going forward. Six rules
  at `conventions/model-tier-discipline.md` §1A:
  1. Bounded brief (one task, one result, file paths, capped
     response length).
  2. Foreground + serial when writing (`.git/index` race);
     read-only sub-agents MAY parallelise.
  3. ≥80% confidence gate. Pass: mechanical edits, well-specified
     implementations, read-only research. Fail: architectural
     decisions, doctrine drafting, cross-layer coordination.
  4. Layer scope preserved — Task sub-agents stay in Task scope.
     Cross-layer asks travel via mailbox.
  5. Anti-slop — must contribute to a real next step.
  6. One brief → one result → parent reviews → commit OR queue
     next. Parent never delegates the commit decision.
- **Self-dispatch now requires Master ratification.** When this
  Task is waiting on Master / operator / cross-cluster and wants
  to propose more sub-agent work, the proposal goes via outbox.md
  for Master to ratify into `~/Foundry/.claude/sub-agent-queue.md`.
  Operator-directed dispatches (e.g., the operator says "launch
  chunk #N") remain fine — that's explicit ratification.
- **Operational precedent recorded.** Master's brief explicitly
  cites this cluster as the operational origin of the codified
  pattern: *"`project-slm` Task has been operating this pattern
  organically since 2026-04-26 — see their cluster cleanup-log for
  examples (three-parallel research-only Sonnet pass on 2026-04-27
  closed chunks #6 + #7 + #8 without writes; AS-2 scope correction
  on 2026-04-27 saved 3-4 weeks of misdirected implementation).
  v0.1.30 codifies that practice as workspace-wide convention."*
  This cluster's cleanup-log entries from 2026-04-27 are now
  doctrinal precedent.
- **No code changes.** Tests still 46/46.

---

## 2026-04-27 — Three-parallel Sonnet research pass (chunks #6 + #7 + #8)

Three foreground research-only Sonnet sub-agents, launched
in parallel (no `.git/index` race — none did writes). All
durable knowledge for future Task work; chunk #7 surfaced
to Master via outbox.

### Chunk #6 — slm-doorman test coverage gaps

46 tests across 10 modules (healthy baseline). Three
priority gaps:

1. **`slm-doorman-server/src/http.rs` has ZERO automated
   tests.** Every `DoormanError` → HTTP status mapping,
   the `SLM_APPRENTICESHIP_ENABLED=false` 404 path,
   malformed-header 400 paths — all unverified by
   automated tests. Highest operational impact (silent
   regression risk). Effort: moderate (needs `AppState`
   factory; cases easy after that).
2. **`tier/local.rs` has no unit tests at all.** Only
   indirect coverage via AS-2 dispatcher. `empty choices →
   UpstreamShape` and `error_for_status` paths dark.
   Effort: easy — wiremock + factory pattern reusable
   from yoyo/external tests.
3. **`VerdictOutcome::Reject` + `DeferTierC` not
   exercised through `VerdictDispatcher::dispatch`.**
   `Reject` is in promotion stats tests but not full
   dispatch; `DeferTierC` not tested anywhere at
   dispatcher level. Effort: easy — same shape as
   existing `refine_verdict_writes_dpo_pair`.

Lower-priority gaps in same audit: BearerToken provider
failures, audit-ledger error paths (HOME unset, dir not
writable), redaction patterns `gho_` / `xox-`,
citations-resolver edge cases. Tracked as task #14.

### Chunk #7 — GUIDE-doorman-deployment.md refinement

Audited the staged `/srv/foundry/GUIDE-doorman-deployment.md`
(workspace-root draft from B7 prep, commit `6937a95`)
against current ARCH/DEV.md / systemd unit + bootstrap.sh
/ conventions. Significant drift:

- Wrong catalog path (`vendor/` should be `customer/`).
- Audit ledger path mismatch — unit declares
  `SLM_AUDIT_DIR` but server code uses
  `$HOME/.service-slm/audit/`; env var is
  declared-but-unused.
- Tier B section names "GCP Cloud Run" — ruled out by
  zero-container-runtime convention.
- References nonexistent `infrastructure/slm-doorman/`
  bootstrap path.
- Missing `SLM_BRIEF_TIER_B_THRESHOLD_CHARS` env var,
  missing `flock(2)` + BriefCache process-restart
  caveat, missing GCE cold-start in troubleshooting.
- Tone/scope drift toward architectural prose
  (`What is the Doorman`, `Integration with Totebox`).
- Apprenticeship Substrate framed as v0.1.x+ future; in
  reality endpoints exist now (404 when disabled).

Refined ~400-line draft surfaced inline in outbox to
Master 2026-04-27 with four open questions: (Q1) catalog
subfolder name; (Q2) audit ledger path policy
(accept code path or wire `SLM_AUDIT_DIR`); (Q3) tenant
default in unit file; (Q4) relationship to existing
`infrastructure/local-doorman/`. Tracked as task #15.

### Chunk #8 — CONTRACT.md MINOR-bump prep

Researched `infrastructure/slm-yoyo/CONTRACT.md` (current
v0.0.1) versioning rules + field-placement conventions
ahead of future AS-2 wire-format addition. Findings:

- MINOR semantics are header-centric ("new optional
  headers or endpoints") — does not explicitly address
  body fields.
- Principle 1 says "metadata in headers, never body" —
  direct tension with vLLM `extra_body.structured_outputs.
  grammar` placement. But `extra_body` is vLLM's
  inference-engine extension slot, arguably not "PointSav
  metadata", so Principle 1 may not bind.
- 410 MAJOR-mismatch is contract-level (line 149), not
  Doorman invention.
- No `cites:` frontmatter; doesn't claim convention
  status.

Recommendation when AS-2 scope ack lands: MINOR bump
0.0.1 → 0.1.0 with three changes: (1) optional grammar
field at `extra_body.structured_outputs.grammar` with
default null + min-vLLM annotation; (2) add
`supports_structured_outputs: bool` to `/v1/contract`
discovery (matches existing `supports_lora` /
`supports_streaming` pattern); (3) one-line addition to
versioning section acknowledging optional body extensions
as MINOR category. Tracked as task #16.

### Cumulative session state

Six commits this session, no code changes. Tests still
46/46 in slm-doorman. Outbox: five messages awaiting
Master pickup (AS-2 scope, fifth-pass drift, fourth-pass
drift, NEXT.md sweep, GUIDE refinement). Inbox: empty.
All Sonnet research findings landed as durable cleanup-log
entries.

---

## 2026-04-27 — Fifth-pass zero-container drift + §11 verification (Sonnet sub-agents chunks #2 + #3)

- **Chunk #2 — five new drift sites caught.** Foreground
  Sonnet research agent audited
  `service-slm/ARCHITECTURE.md` + `DEVELOPMENT.md` against
  `conventions/zero-container-runtime.md`. Five sites
  beyond fourth-pass:
  - ARCH §2 line 59 Ring 3b memory table — "OCI Artifacts"
    (structural).
  - ARCH §3b line 118 — "stored as an OCI Artifact"
    (structural; couples with previous).
  - DEV §2.2 line 122-124 — "OCI Artifacts" signing
    description (prose; couples with previous two).
  - DEV §6 line 237 — `cargo-chef` for Docker layer
    caching (prose).
  - DEV §7 line 289 — declared workspace dep
    `google-cloud-run = "*"` (structural — would pull
    Cloud Run client bindings at compile time).
  Eight sites total bundled with fourth-pass in outbox
  for single Master-authorised prose-edit commit.
- **Chunk #3 — §11 cross-references VERIFIED CLEAN.**
  Foreground Sonnet research agent verified every file
  path, type name, enum variant, constant, env var, HTTP
  endpoint, ledger path, corpus path, and promotion
  threshold cited in `ARCHITECTURE.md` §11 (the
  apprenticeship section added in AS-7) against the
  current code state under `service-slm/crates/`. **All
  OK.** No stale references, no mismatches. One
  observation: `VERDICT_BATCH_NAMESPACE` is exported but
  never used in the verify path — §11 does not claim it
  is wired, so not §11 drift; surfaced here as a future
  follow-up if batch verification becomes desirable. The
  doc terminology shorthand `<ulid>` vs the code's
  `UUIDv7` is consistent across both surfaces.
  §11 is reliable as a spec reference for the current
  code state.
- **No code changes** in either audit. Tests still 46/46
  in slm-doorman.

---

## 2026-04-27 — AS-2 scope correction surfaced to Master (Sonnet sub-agent chunk #1)

- **Model-tier-discipline applied.** Per
  `conventions/model-tier-discipline.md`, ran a research-only
  Sonnet sub-agent (foreground; same Opus session, no
  `.git/index` race) to verify the `llguidance` crate API
  surface before scaffolding the AS-2 integration. Cost:
  one foreground Agent invocation (~3 minutes wall, ~58k
  Sonnet tokens). Saved: committing to a 3-4 week
  implementation against a wrong design.
- **Finding.** `llguidance` is real (v1.7.4, MIT, pure
  Rust, actively maintained) but is decode-time
  infrastructure that needs to be in the LLM sampler loop.
  Our Doorman is HTTP-only — no integration point on
  Tier A or Tier B for the Rust crate itself. The
  decision-rationale committed in `9c99af5` is sound for
  the *protocol* choice (vLLM does support llguidance
  natively as a sampling backend), but the "Rust-native"
  benefit accrues to the vLLM server, not to Doorman code.
- **Per-tier reality:**
  - Tier A llama-server HTTP API: only `grammar` (GBNF) +
    `json_schema` fields. Lark NOT accepted on the wire.
  - Tier B vLLM HTTP API: Lark via
    `extra_body.structured_outputs.grammar` (vLLM ≥0.12)
    or legacy `extra_body.guided_grammar`. vLLM internally
    applies llguidance.
  - Tier C: no arbitrary grammar support (vendor-specific
    JSON-mode at best).
- **Outbox to Master.** Surfaced the correction with two
  questions: (Q1) is Tier A grammar asymmetry acceptable
  — apprentice on Tier A produces unconstrained output,
  Lark only enforced when escalated to Tier B? (Q2) what's
  the vLLM version target for the Doorman wire layer? Hold
  on all AS-2 code work until Master ack.
- **NEXT.md AS-2 entry rewritten** with corrected scope
  and the HOLD-pending-Master-ack note.
- **Task #1 description updated** with corrected scope.
- **No code changes.** Tests still 46/46 passing in
  slm-doorman.

---

## 2026-04-27 — NEXT.md Queue refresh + fourth-pass zero-container drift surfaced

- **NEXT.md sweep against committed reality.** Session-start
  read flagged that `service-slm/NEXT.md` Queue still listed
  six items already closed in commit history. Moved them to
  "Recently done" with commit refs:
  - `cognitive-bridge.sh → scripts/` (`badd447`, 2026-04-26)
  - `cargo deny check licenses` in CI (`d97a994`, 2026-04-26)
  - `MISSING CONNECTION PHYSICS` in `cognitive-bridge.sh`
    (`3c0c8e5`, 2026-04-26) — also lifted the corresponding
    `system-slm connection protocol` entry from the Blocked
    section since the bridge now calls the Doorman.
  - `cognitive-forge ↔ content-compiler` wire format
    reconciliation (`5da4676`, 2026-04-26)
  - B4 Tier C client mock-only (`d8ef1ec` + server-side
    env-var wiring `fab047e`, 2026-04-26) — was already in
    "Recently done" but also still in Queue.
  - ARCH §5.10 + §2 zero-container third-pass cleanup
    (`8c3212e`, 2026-04-26) — Queue text mis-implied the
    third-pass was still pending Master sign-off.
- **Fourth-pass zero-container drift sites surfaced to
  Master via outbox.** Verifying the §5.10 / §2 third-pass
  against the live file turned up three new sites the
  third-pass scope did not cover:
  - ARCHITECTURE.md §3 line 132: "External calls (Cloud Run,
    Mooncake sidecar, Claude API, ...)"
  - ARCHITECTURE.md §5.2 line 197: `hyper` crate role
    "(Cloud Run, Claude API, LMCache master)"
  - DEVELOPMENT.md §4 Phase 2 step 5: "Port the Cloud Run
    driver (`crates/slm-compute`, ...)"
  Per the established third-pass pattern (do not act without
  Master authorisation), these are surfaced via outbox with
  per-site replacement-text recommendations and queued in
  NEXT.md as a fourth-pass Queue item. No prose edits in
  this commit — drift-flagging only.
- **AS-2 inbox ack.** Master 2026-04-27 v0.1.26 message
  acknowledging the AS-2 library decision (`llguidance`)
  archived to inbox-archive.md per the mailbox protocol.
  AS-2 grammar implementation queued as a multi-week Queue
  item in NEXT.md; develops independently of project-language
  Phase 1B per Master's brief.
- **NEXT.md `Last updated` bumped to 2026-04-27.**
- **No code changes; tests still 46/46 passing in
  slm-doorman.**

---

## 2026-04-28 — Brief A: http.rs test factory + 12 integration tests (PS.6 sub-brief #1)

- **slm-doorman-server gains a library target** (`src/lib.rs`). Required
  because `http.rs` is private to the binary; integration tests under
  `tests/` cannot import from a binary crate's `src/` directly. The `[lib]`
  target exposes `pub mod http` (containing `AppState` and `router`) and
  `pub mod test_helpers` (factory helpers for tests). `main.rs` updated to
  `use slm_doorman_server::http` instead of the inline `mod http`.
- **`slm-doorman-server/tests/http_test.rs` created.** 12 new tests
  covering three categories:
  - Smoke (4): `smoke_healthz_returns_200_ok`,
    `smoke_readyz_returns_200_with_tier_flags`,
    `smoke_contract_returns_200_with_version_fields`,
    `smoke_chat_completions_happy_path_returns_200_with_content`
  - Error-mapping (5): `error_tier_unavailable_returns_503`,
    `error_brief_cache_miss_returns_410`,
    `error_verify_signature_returns_403`,
    `error_external_not_allowlisted_maps_to_403`,
    `error_malformed_module_id_header_returns_400`
  - Apprenticeship-disabled 404 (3): `apprenticeship_disabled_brief_returns_404`,
    `apprenticeship_disabled_verdict_returns_404`,
    `apprenticeship_disabled_shadow_returns_404`
- **Workspace tests 55/55 → 67/67.** All existing 55 pass; 12 new pass.
  Clippy clean; fmt clean. Committed `d9ea19d` (Peter-authored).
- **Deviation from brief noted**: `TierUnavailable` maps to 503
  `SERVICE_UNAVAILABLE` (not 502 `BAD_GATEWAY` as the brief listed). Tested
  against the actual code mapping. `ExternalNotAllowlisted` cannot be
  triggered through the HTTP handler (tier_hint is hardcoded None); covered
  via a `From<DoormanError>` mapping assertion rather than a full HTTP
  round-trip.
- **Dev-deps added** to `slm-doorman-server/Cargo.toml`: tower, wiremock,
  tokio, serde_json, async-trait, base64, chrono (all dev-only).
- **Test helpers in lib.rs reusable by Briefs B and C** (next two PS.6
  coverage briefs): `temp_ledger`, `temp_promotion_ledger`,
  `app_state_no_tiers`, `app_state_with_local`, `app_state_with_external`,
  `app_state_with_apprenticeship`.

---

## 2026-04-26 — B4 Tier C client (mock-only per operator guardrail) + PricingConfig

- **B4 Tier C client implemented end-to-end as code + tests, zero
  live network.** Per Master's 2026-04-26 10:30 inbox brief
  Answer 3 and the operator's relayed cost guardrail.
  `crates/slm-doorman/src/tier/external.rs` rewrite:
  - `ExternalAllowlist` switched from runtime `HashSet<String>`
    to compile-time `&'static [&'static str]` per the brief.
    `EMPTY` const default; `from_static` const constructor;
    `FOUNDRY_DEFAULT_ALLOWLIST` carries the three labels
    documented in `llm-substrate-decision.md` §"Three compute
    tiers" (citation-grounding, initial-graph-build,
    entity-disambiguation).
  - `TierCProvider` enum (Anthropic / Gemini / Openai). Model
    identifier carries a `provider:` prefix; `parse_model_id`
    splits and matches.
  - `TierCPricing` struct holds per-provider per-mtok input/
    output rates. `cost_usd(provider, prompt_toks,
    completion_toks)` does the per-call computation.
  - `ExternalTierConfig` grows `provider_endpoints:
    HashMap<TierCProvider, String>`, `provider_api_keys`, and
    `pricing`.
  - `ExternalTierClient::complete()` enforces invariants in
    order: (1) allowlist check, (2) provider parsing, (3)
    endpoint+key lookup, (4) network call, (5) cost computation
    from response usage. Failure at steps 1-3 returns BEFORE any
    network attempt — verified by tests asserting
    `server.received_requests()` length 0.
- **`slm-core::ComputeRequest` extended with `tier_c_label:
  Option<String>`** (serde default; backward-compatible). Server
  HTTP layer parses `X-Foundry-Tier-C-Label` request header onto
  this field.
- **Six wiremock-based unit tests covering all wire paths**:
  happy_path_allowlist_match_returns_content_and_cost,
  unallowlisted_label_refuses_before_any_network_call,
  missing_label_refuses_before_any_network_call,
  unknown_provider_prefix_surfaces_upstream_shape,
  provider_parses_known_prefixes,
  foundry_default_allowlist_contains_documented_labels,
  tier_c_pricing_arithmetic. Total workspace tests 12/12 → 19/19.
- **Server wiring not in this commit.** `slm-doorman-server`
  still passes `external: None` to `DoormanConfig`. The
  ExternalTierClient is buildable from env vars (per-provider
  endpoint / key / pricing) but the env-var parsing surface is
  follow-up work — not specifically named in Master's brief.
  Surfaced in NEXT.md.
- **PricingConfig (Master Answer 2) landed in same session** as
  a prior commit (`8c2418d`); see that commit's cleanup-log
  entry for the Yo-Yo cost-field arithmetic.
- **Operator guardrail observed:** no live API calls to
  Anthropic / Gemini / OpenAI; no provider-SDK installs (used
  raw `reqwest` so the endpoint is mockable); no auto-promotion
  of any request to Tier C without the explicit allowlist
  label. v0.0.10 hard rule #4 preserved end-to-end.

---

## 2026-04-26 — PricingConfig in YoYoTierConfig (Master Answer 2)

(Brief entry — full detail in commit `8c2418d` body.)

- Added `PricingConfig` struct to
  `crates/slm-doorman/src/tier/yoyo.rs`. `yoyo_hourly_usd: f64`
  default zero. Method `yoyo_cost_usd(inference_ms)` computes
  `(hourly_usd / 3_600_000) × inference_ms`.
- `YoYoTierConfig` grows `pricing` field.
- `YoYoTierClient::complete()` sets `cost_usd:
  self.config.pricing.yoyo_cost_usd(inference_ms)`.
- Server reads `SLM_YOYO_HOURLY_USD` env var (default 0.0).
- Two unit tests: arithmetic verification + default-zero
  invariant.

---

## 2026-04-26 — third-pass zero-container cleanup (Master-authorised)

- **Two surviving drift sites resolved per Master's 2026-04-26
  10:30 inbox brief Answer 1.**
  - `ARCHITECTURE.md` §5.10 "Not-Rust components" SkyPilot row
    dropped outright (orphaned after the §10 SkyPilot drop;
    table now enumerates only LMCache+Mooncake and vLLM, both
    actively in the architecture).
  - `ARCHITECTURE.md` §2 Ring 1 Bootstrap items 3+4 rewritten
    to GCE start/stop ceremony per the convention's "What is
    used instead" + "Cold-start: the only honest concern"
    sections. Item 3: "Cloud Run GPU scale-to-zero" → "GCE GPU
    instance with `idle_shutdown_minutes=N` per
    `infrastructure/slm-yoyo/tofu/`". Item 4: "Warm pool opt-in
    via min-instances=1" → "Warm-VM mode opt-in: hold the GCE
    instance running between requests within a configurable
    window".
  - Closing line "Bill-per-second for request processing; zero
    idle cost outside explicitly-opened warm windows" updated
    to "zero idle cost once the `idle_shutdown_minutes=N` timer
    fires and the instance stops" — same economics, GCE
    nomenclature.
- **Cluster manifest also updated by Master in parallel
  (Doctrine v0.0.4 triad schema)** — committed in same commit per
  the cluster-manifest-tracking pattern Master confirmed in B5
  reply (4d). Manifest's customer-tier "leg-pending" item names
  `GUIDE-doorman-deployment.md` as Task work to draft —
  surfacing for follow-up; not in this commit's scope.

---

## 2026-04-26 — B2 Yo-Yo HTTP client (mock-only per operator guardrail)

- **B2 implemented end-to-end as code + tests, zero live
  network.** Per Master's 2026-04-26 07:50 inbox brief and the
  operator's relayed cost posture
  ("There is no reason to run a Yo-Yo yet and it should not be
  costing us any money for now"), the implementation is purely a
  code/mock exercise:
  - `BearerTokenProvider` async trait + `StaticBearer` impl in
    `crates/slm-doorman/src/tier/yoyo.rs`. Real provider impls
    (GCP Workload Identity, RunPod / Modal Secret Manager,
    customer mTLS) implement the trait but are NOT wired in this
    commit — they are future work the trait keeps open.
  - `YoYoTierClient::complete()` does POST `/v1/chat/completions`
    with `Authorization: Bearer <token>` plus four
    `X-Foundry-*` headers (`Request-ID`, `Module-ID`,
    `Contract-Version`, `Complexity`) per
    `infrastructure/slm-yoyo/CONTRACT.md`.
  - Retry policy:
    - 503 + `Retry-After`: sleep `min(retry_after, 60)` seconds
      then retry once
    - 401 / 403: refresh token, retry once with fresh token
    - 410: surface `DoormanError::ContractMajorMismatch`, no
      retry (CONTRACT.md MAJOR-version mismatch is loud-fail)
    - other non-2xx: surface `UpstreamShape` with body preview
  - Response metadata: capture `X-Foundry-Inference-Ms` (else
    fall back to wall-clock) and `X-Foundry-Yoyo-Version` for
    the audit ledger.
- **Cost field deferred.** CONTRACT.md does not carry a cost
  field on the wire. Doorman computes Tier B cost from
  `inference_ms × per-provider hourly rate`; that
  `PricingConfig` lands in a follow-up. For B2 the audit-ledger
  `cost_usd` is 0 — accurate as "unknown" rather than
  mis-attributed.
- **Two error variants added to `DoormanError`:**
  `ContractMajorMismatch { remote_status, doorman_version }` and
  `BearerToken(String)`. Both classify as `UpstreamError` in the
  audit ledger and `BAD_GATEWAY` in the inbound HTTP layer.
- **Tests.** Four `wiremock`-based async tests covering happy
  path 200, 503 retry, 401 auth refresh, 410 mismatch. Workspace
  total 6/6 → 10/10 unit tests passing. `cargo clippy
  --all-targets -- -D warnings` clean; `cargo fmt --all --
  --check` clean.
- **Server wiring.** `slm-doorman-server` env-var contract
  extended with `SLM_YOYO_BEARER` (static-bearer dev path).
  `SLM_YOYO_ENDPOINT` empty → community-tier mode unchanged
  (B5 pattern preserved).
- **Operator guardrail observed:** no `tofu apply`, no live
  HTTP calls against any deployed Yo-Yo, no real bearer-token
  consumption against any provider, no CUDA / GPU runtime
  installs. v0.0.10 hard rule #4 preserved end-to-end.

---

## 2026-04-26 — second-pass: eleven zero-container drift sites (Master-authorised)

- Per Master's 2026-04-26 07:50 inbox brief (4a "GO AHEAD") and
  the per-site replacement text Master supplied, applied eleven
  prose edits across `service-slm/ARCHITECTURE.md` and
  `service-slm/DEVELOPMENT.md` in a single commit:
  - ARCH §2 memory-tier table row 1 storage column (line 56)
    "Container image + GCS-cached weights" → "systemd-unit
    `ReadWritePaths` + GCS-cached weights"
  - ARCH §2 Ring 1 Bootstrap item 1 (line 67-68) "Pre-built
    container in Artifact Registry" → "Pre-built native binary
    in the `pointsav-public` GCE image family per
    `infrastructure/slm-yoyo/tofu/` precedent" with citation of
    `conventions/zero-container-runtime.md`
  - ARCH §4 moduleId table row 1 (line 145) "which container
    variant to boot" → "which `systemd` unit `ExecStart` per
    `moduleId`"
  - ARCH §5.9 Sigstore (line 252) "container images and OCI
    artefacts" → "native binaries and unit files; SSH commit
    signing per workspace `CLAUDE.md` §3 is the primary
    commit-time authority, with `sigstore` reserved for
    release-artefact signing"
  - ARCH §6 `slm-compute` crate (line 285) "Cloud Run driver,
    container mgmt" → "GCE driver, systemd lifecycle"
  - ARCH §8 event vocabulary (line 427) "BOOT_REQUEST —
    SkyPilot asked to spin up" → "BOOT_REQUEST — OpenTofu
    provisioning kicked off via `tofu apply`"
  - ARCH §10 2030 headroom — dropped the "Distributed KV across
    clouds (SkyPilot 0.11 + Mooncake)" row entirely
  - DEV §1 release-build (line 116) "release-build container
    signing" → "release-build SSH commit + tag signing on top
    of `sigstore` binary signing; no container images produced"
  - DEV §4 Phase 1 (line 159) "Python, vLLM, SkyPilot, dbt,
    Dagster" → "Python, vLLM (multi-LoRA), OpenTofu, dbt,
    Dagster" with `conventions/adapter-composition.md` citation
    for the vLLM-stays decision
  - DEV §4 Phase 2 (line 176-178) "container-side for remote"
    → "remote-side native binary delivered via the
    `pointsav-public` GCE image"
  - DEV §5 B2 row "SkyPilot pool with `min_replicas=1`" →
    "OpenTofu module with `idle_shutdown_minutes=N` per
    `infrastructure/slm-yoyo/tofu/`"
- **Additional drift surfaced — NOT touched in this commit.**
  `service-slm/ARCHITECTURE.md` §5.10 "Not-Rust components,
  behind network protocols" table contains a row
  `| SkyPilot (if used) | Python | Multi-cloud abstraction,
  overkill for Phase 1 single-cloud | External driver, not
  linked |`. With §10's SkyPilot row dropped, this §5.10 row
  reads as orphaned ("if used" but no remaining call-site).
  Master did not list §5.10 in the eleven-site brief; per the
  "stop and surface if structurally larger" caveat, leaving it
  for next-pass authorisation. Recommendation: drop the row.
- **Cloud Run reference at §2 Ring 1 Bootstrap item 3** ("Cloud
  Run GPU scale-to-zero with drivers pre-installed") and the
  surrounding paragraphs about "warm pool opt-in" and
  "Bill-per-second for request processing" also reference
  Cloud Run — a containerised runtime per the convention's
  "What this rules out" list. Master did not list these; same
  caveat applies. Suggest dropping the Cloud Run mention in
  favour of GCE start/stop ceremony per the convention's
  trade-off section. Surface for next-pass authorisation.

---

## 2026-04-26 — ARCHITECTURE.md §7 zero-container rewrite (Master-authorised)

- **Scope of this commit (narrow, per brief).** Rewrote §7 file
  tree only: `compute/container/{Dockerfile,requirements.txt,
  build.sh}` → `compute/systemd/{local-slm.service,deploy.sh}`;
  `compute/sky/{ingest,warmpool,teardown}.yaml` →
  `compute/tofu/{main,variables,outputs}.tf` plus
  `tofu/killswitch/`. Added preface paragraph that names the two
  reference implementations the layout dogfoods
  (`infrastructure/local-slm/` for Tier A, B5-verified today;
  `infrastructure/slm-yoyo/tofu/` for Tier B). Added trailing
  paragraph clarifying the in-tree subtrees are per-deployment
  overrides composed against upstream defaults. Cited
  `conventions/zero-container-runtime.md` as the structural
  authority. `memory/adapters/train/` annotated as Python via
  `pyproject.toml + uv` per the `router-trainer/` precedent
  (Master's brief).
- **Adjacent drift NOT touched in this commit (surfaced to
  Master via outbox + NEXT.md):** eight more container /
  SkyPilot references remain in `service-slm/ARCHITECTURE.md`
  (§2 Ring 1 Bootstrap "Pre-built container in Artifact
  Registry"; §2 memory-tier table; §4 moduleId table; §5.9
  Sigstore "container images"; §6 `slm-compute` crate
  description "Cloud Run driver, container mgmt"; §8 event
  vocabulary `BOOT_REQUEST — SkyPilot asked to spin up`; §10
  2030 headroom "SkyPilot 0.11"; plus three more in
  `service-slm/DEVELOPMENT.md` §1.1, §4 Phase 1, §4 Phase 2,
  §5 B2 row). Per Master's "stop and surface if structurally
  larger than expected" caveat in the brief, I did NOT expand
  the rewrite to cover them; the §7 commit is the narrow
  Master-authorised change. A second-pass session needs an
  explicit go-ahead to consolidate the rest.

---

## 2026-04-26 — B5 verification end-to-end (Tier A live)

- **B5 PASSED.** Doorman release binary booted against Master's
  `local-slm.service` (delivered B3 in workspace v0.0.11
  `68e7c16`; D1 done operator-side prior). Verification trail
  captured in `service-slm/NEXT.md` Recently-done and in the
  archived inbox message. One audit-ledger entry at
  `~/.service-slm/audit/2026-04-26.jsonl` for request_id
  `b2e10115-c747-4fc8-b571-80484db7276e`:
  `tier:"local"`, `model:"Olmo-3-1125-7B-Think-Q4_K_M.gguf"`,
  `inference_ms:43914`, `cost_usd:0.0`,
  `completion_status:"ok"`.
- **No code change in this commit** — the binary was built from
  `78031c4` (B1 scaffold). The release binary at
  `service-slm/target/release/slm-doorman-server` is gitignored
  per `service-slm/.gitignore`.
- **Doctrine v0.0.2 deltas read but not absorbed into code.**
  Per Master's inbox brief, no behavioural change for B5.
  §IV.c cluster manifest now lives at `.claude/manifest.md`
  (backfilled by Master); §XV trajectory-substrate hooks are
  workspace-tier responsibility (Master's L1 capture, not Task).
  Adapter Composition Algebra (§XIV) note: the Doorman is
  structurally aligned but the composition logic is not in B1
  scope — pickup once L3 constitutional adapter exists.
- **Three follow-ups from prior session closed by Master:**
  standalone-vs-nested workspace decision confirmed (no edit
  needed); deny.toml/rust-toolchain.toml repo-layout question
  deferred to next Root Claude in `pointsav-monorepo`;
  `ARCHITECTURE.md` §7 zero-container rewrite explicitly
  authorised as Task scope (queued as next Right-now item in
  `service-slm/NEXT.md`, separate atomic commit).

---

## 2026-04-25 — B1 Doorman scaffold (Phase B, inbox v0.0.7)

- **service-slm scaffolded as standalone cargo workspace.** New
  `service-slm/Cargo.toml` (workspace), `deny.toml` (per
  `service-slm/DEVELOPMENT.md` §2.1), `rust-toolchain.toml`
  (stable), `.gitignore`. Three workspace members under
  `crates/`: `slm-core` (shared types + moduleId discipline),
  `slm-doorman` (lib: three-tier router + JSONL audit ledger),
  `slm-doorman-server` (axum bin: `/healthz`, `/readyz`,
  `/v1/contract`, `POST /v1/chat/completions`). Existing
  `cognitive-forge/` subcrate remains untouched, listed under
  workspace `exclude`. `cargo check`, `cargo test`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo fmt`
  all clean; 6/6 unit tests pass.
- **Standalone-vs-nested workspace question closed** in
  `service-slm/ARCHITECTURE.md` §6. Standalone chosen because it
  touches no code outside `service-slm/` and leaves the monorepo
  unification cleanup (2026-04-18 audit, 8 of ~70+ crates declared)
  to be settled separately. Conversion to nested later is
  mechanical (move members up; drop nested `Cargo.toml`).
- **B5 verification path covered structurally.** The
  `slm-doorman-server` env-var contract (omit `SLM_YOYO_ENDPOINT`)
  realises the "Doorman boots without Yo-Yo" requirement per
  Optional Intelligence (`conventions/three-ring-architecture.md`).
  End-to-end smoke against a live Tier A endpoint is queued in
  `service-slm/NEXT.md` Right-now and depends on Master's B3
  systemd unit landing on the workspace VM.
- **Tier B (B2) and Tier C (B4) deferred per inbox brief** —
  client interfaces and request-shape stubs are in
  `tier/yoyo.rs` and `tier/external.rs`; `complete()` returns
  `DoormanError::NotImplemented { filled_in_by: "B2" | "B4" }`
  so the router exercises the fallback path without confusion.
- **Layout-rule question to surface to Root Claude.** Two
  files at `service-slm/` project root are not in
  `.claude/rules/repo-layout.md`'s project-root allowed list but
  are mandated by `service-slm/DEVELOPMENT.md` §2.1 / standard
  cargo conventions: `deny.toml`, `rust-toolchain.toml`. Either
  the rule's project-root allowed-files list extends to admit
  these two filenames for crates that are themselves cargo
  workspaces, or a different home is named. Recommendation
  (Task scope, not action): admit both at the project root,
  scoped to projects that are workspaces.
- **Convention-drift item surfaced into NEXT.md.**
  `service-slm/ARCHITECTURE.md` §7 references
  `compute/container/Dockerfile` and `requirements.txt` — both
  predate `conventions/zero-container-runtime.md` (ratified
  2026-04-25). Architecture text needs rewriting before
  scaffolding the `compute/` directory; queued as a NEXT.md
  item, not closed here.

---

## 2026-04-23 — service-slm activation (framework §8)

- **`service-slm` activated via framework §8.** First-live
  cluster occupation on `cluster/service-slm` (Task Claude in
  `~/Foundry/clones/service-slm/`). Added per-project
  `CLAUDE.md`, `NEXT.md`, `ARCHITECTURE.md`, `DEVELOPMENT.md`.
  Registry row flipped Scaffold-coded → Active; summary count
  4 → 5. Commit `32e51e4`, Peter-authored, held locally
  (no push) per workspace `CLAUDE.md` §7 Stage-6 hold.
- **Four defects newly surfaced at service-slm project level** —
  added to `service-slm/NEXT.md` Queue, not yet closed:
  - `cognitive-bridge.sh` at project root (layout defect;
    already flagged in monorepo `NEXT.md` layout-hygiene list).
  - `transient-queues/` holds runtime payload state in Git,
    mirroring the `discovery-queue` "Not-a-project" pattern.
    Triage pending.
  - `cognitive-forge/` subcrate carries the Do-Not-Use term
    "Cognitive Forge." Inherits the rename concern queued
    against sibling `tool-cognitive-forge`; pair both in one
    decision.
  - `cognitive-forge → content-compiler` wire format
    inconsistent — writer emits `.md`, reader parses `.json`.
    Not interoperating today.
- **Open architectural question surfaced — standalone vs nested
  cargo workspace.** SLM-STACK.md lays `service-slm` out as its
  own cargo workspace with `crates/`. The monorepo
  workspace-under-declaration (2026-04-18 audit finding) has a
  pending unification decision. Which wins — standalone per
  SLM-STACK, or nested member of a unified monorepo workspace?
  Recorded in `service-slm/ARCHITECTURE.md` §6 "Open question";
  scaffolding waits for resolution.
- **Workspace-root → sibling/cluster handoff pattern first
  applied.** New workspace `CLAUDE.md` §9 "Workspace-root
  source files" subsection is the tracking mechanism for
  `SLM-STACK.md` / `YOYO-COMPUTE.md` rehoming. The Task-scope
  half landed in commit `32e51e4`; the Root-scope half (wiki
  `topic-*.md` files) remains open for a Root Claude session
  in `content-wiki-documentation/`. Workspace-root originals
  stay in place until every destination has committed.

---

## 2026-04-23

- **Repo-layout rule introduced.** Added
  `.claude/rules/repo-layout.md` codifying the allowed file set at
  the monorepo root and at each project directory root, and naming
  the sibling repos where cross-cutting content belongs (user guides,
  ADRs, design-system material). Anchor for the file-relocation work
  queued behind it (see `NEXT.md`).
- **Defects surfaced at root by this rule** — staged for separate
  commits, not moved in this session:
  - ~~`force_build.sh` (tracked, at repo root) → queued move to
    `vendor-sel4-kernel/scripts/`~~ **Closed 2026-04-23** — moved
    via `git mv` in a follow-up commit within this session. Zero
    runtime callers; script body uses absolute paths so no content
    edits required.
  - `GUIDE-OPERATIONS.md` (tracked, at repo root) → queued move to
    `content-wiki-documentation/`.
  - `USER_GUIDE_2026-03-30_V2.md` (tracked, at repo root) → queued
    move to `content-wiki-documentation/` with `_V2` dropped, per
    CLAUDE.md §6 edit-in-place rule.
  - `app-console-content/src/{pointsav-surveyor.sh,surveyor.py}` →
    queued move to `app-console-content/scripts/`. `surveyor.py` is
    the Verification Surveyor operational tool, misfiled alongside
    Rust crate source.
  - ~~`os-infrastructure/build_iso/forge_iso.sh` → queued rename to
    `os-infrastructure/build_iso/compile_binary.sh`~~ **Closed
    2026-04-23** — renamed via `git mv`; in-file header comment
    updated to reflect the new name and record the rename
    rationale. Zero external callers.
- **Project-root scripts flagged (not yet moved):** ~15 scripts sit
  at project root instead of under `scripts/` across `service-vpn`
  (5 generator scripts), `service-email` (`spool-daemon.sh`),
  `service-slm` (`cognitive-bridge.sh`), `service-content`
  (`forge-seeds.sh`), `os-network-admin` (2 scripts),
  `os-totebox` (1), `tool-cognitive-forge` (1),
  `vendor-phi3-mini` (2), `app-mediakit-telemetry` (5 generic
  scaffold scripts). Each project is a separate closure task.
- **Open question surfaced.** `surveyor.py` hard-codes
  `MAX_DAILY_VERIFICATIONS = 10`. The existing cleanup-log open
  question — "Verification Surveyor daily throttle number — Under
  operational review. Do not cite a specific number" — must
  reconcile: either the code is authoritative (close the question,
  value is 10) or the doc is authoritative (the code is out of step
  and needs updating). Do not cite the number externally until
  resolved.
- **Second open question surfaced (os-infrastructure build
  pipeline).** The two scripts `os-infrastructure/forge_iso.sh`
  (ISO assembly) and `os-infrastructure/build_iso/compile_binary.sh`
  (binary compile, renamed this session) are sequential build
  stages but are not wired together — the assembly script does not
  invoke the compile script, and there is no Makefile or top-level
  driver. Operator must run them manually in order. Is this
  intentional (operator-gated two-step) or drift (should become a
  single driver script)? Pending decision before next pipeline
  refactor.

---

## 2026-04-22

- **Project framework bootstrap.** Added `.claude/rules/project-registry.md`
  with 100-row inventory of every top-level directory, classified by
  state per `~/Foundry/CLAUDE.md` §8 (Reserved-folder /
  Scaffold-coded / Active / Defect / Not-a-project). Framework docs,
  templates, and activation procedure live workspace-level. This
  cleanup-log was also introduced onto `main` today (previously
  present only on feature branches — drift closed).
- **Taxonomy expanded to seven domains.** Added `app-orchestration-*`
  to the in-force `app-[os]-*` list in
  `~/Foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md` §3. Triggered by
  `app-orchestration-bim` appearing during the session — would have
  been an unmatched-prefix defect under the original six-domain
  rule. Now conformant; `os-orchestration` already exists as a
  Systemic Wordmark (§2).
- **Four BIM-research directories registered.** `app-console-bim`,
  `app-orchestration-bim`, `app-workplace-bim`, `service-bim` — each
  with a single `RESEARCH.md`. Classified as Reserved-folder pending
  decision to activate.
- **Audit cleanup.** Removed 2 `__MACOSX/` directories and 16
  tracked `.DS_Store` / AppleDouble files from extraction-artefact
  scaffolding in the egress crates. Added `.DS_Store` to
  `.gitignore`.

---

## 2026-04-18 — Layer 1 structural audit — findings

- **Headline finding:** Workspace `Cargo.toml` declares only 8 of ~70+ crates as members. Everything else is treated as standalone workspaces, which explains the 23 stray `Cargo.lock` files scattered through the repo. `cargo build --workspace` will skip almost everything; profile/edition inheritance is not reaching most crates.
- **Severity counts:** 1 Critical, 1 High, 4 Medium, 1 Low.
  - Critical: workspace under-declaration (8 of ~70+ crates).
  - High: 23 stray `Cargo.lock` files inside member crates.
  - Medium: prefix violations (2); dir-name vs `Cargo.toml` name mismatches (13); doubly-nested `service-email-egress-{ews,imap}` scaffolding; many `app-console-*` / `app-network-*` directories without `Cargo.toml`.
  - Low: `discovery-queue` orphan data directory at root.
- **Good news on prefix adherence:** across ~85 directories, adherence to the seven canonical prefixes is approximately 97.6%. Only two violations found: `pointsav-pty-bridge` (no recognized prefix) and `vendors-maxmind` (plural form instead of canonical `vendor-`).
- **Nested redundancy:** `service-email-egress-ews` and `service-email-egress-imap` both contain a redundant intermediate directory of the same name — a doubly-nested copy-paste scaffolding pattern producing depth-3 crates. All 13 directory-name / `Cargo.toml`-name mismatches are concentrated in these nested egress areas (short dir names like `egress-ingress` aliasing qualified crate names like `service-email-batch-ingress`).
- **No modifications were made in this session — audit only.**
- **Next:** Open Questions section of this log to be updated separately with five new questions raised by the audit.

---

## 2026-04-18

- Initialized this cleanup log. Seeded active renames, deprecations, intentional exceptions, and open questions from Section 13 of the PointSav Project Instructions.
- Established the session-start / session-end read-and-update pattern in CLAUDE.md.
- No code changes in this session. Next session should confirm the active renames table against a fresh grep of the repo to establish a baseline count of remaining occurrences per legacy term.
- Open question surfaced: whether the `service-parser` / `service-extraction` consolidation is scoped for a specific MEMO version or tracked informally. Answer will determine how we prioritize closing that migration.
