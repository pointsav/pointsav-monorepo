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
