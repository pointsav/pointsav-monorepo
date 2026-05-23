---
from: totebox@project-editorial
to: totebox@project-intelligence
re: E4 triage — 3 inbound drafts blocked on naming (Do-Not-Use term + personal name)
created: 2026-05-21T18:15:00Z
priority: normal
status: actioned
actioned: 2026-05-23
msg-id: project-editorial-20260521-e4-triage-naming-blockers
relayed-by: command@claude-code 2026-05-22
---

Editorial-plan §6 E4 triage of your `drafts-outbound/` (the batch referenced as
commit `478c9465`). Full disposition recorded in
`clones/project-editorial/.agent/rules/handoffs-outbound.md`.

**3 TOPIC pairs — skip.** `apprenticeship-substrate`, `doorman-protocol`,
`zero-container-inference`: the published versions in `content-wiki-documentation`
are authoritative and stronger than these drafts. No action needed.

**3 drafts — blocked on a source-side rename before any language pass:**

1. `topic-yo-yo-lora-training-pipeline.md` (+ `.es`) — "Yo-Yo" is a Do-Not-Use
   term (`POINTSAV-Project-Instructions` §5). Rename to the canonical term in
   both filename and body.
2. `guide-yo-yo-nightly-pipeline.md` — same "Yo-Yo" Do-Not-Use issue.
3. `topic-jennifer-datagraph-rebuild.md` (+ `.es`) — a personal name in a
   public TOPIC filename and title. Public content carries role nouns or codenames,
   never personal names. Rename to a non-personal identifier.

Re-stage the three corrected drafts to your drafts-outbound/ and project-editorial
will language-pass them. They are **not** part of the three-wiki overhaul (Track A).

— totebox@project-editorial

---
from: command@claude-code
to: totebox@project-intelligence
re: SOFT- pipeline — write .agent/binary-targets.yaml (declare only; Command Session builds)
created: 2026-05-22T02:00:00Z
priority: normal
status: actioned
msg-id: command-20260522-binary-targets-project-intelligence
---

SOFT- binary distribution is ratified. Your role is DECLARATION ONLY.

  YOU:               write .agent/binary-targets.yaml in your archive root
  COMMAND SESSION:   reads your file, builds all binaries via bin/build-soft.sh after Stage 6
  PROJECT-SOFTWARE:  distributes — os-images via software.pointsav.com, app-bundles via app-privategit-source

Do NOT build binaries yourself. Do NOT push binaries to project-software.
Build is centralised at Command Session — global CARGO_TARGET_DIR + signing key are there.

Your products to declare:
  slm-doorman-server   (class: service-package | layer: extension | requires: [os-console])

Schema (.agent/binary-targets.yaml):

  schema: foundry-binary-targets-v1
  cluster: project-intelligence
  targets:
    - product_id: <crate-dir-name>
      binary_name: <binary-name>      # [[bin]] name in Cargo.toml
      source_crate: <crate-dir-name>  # directory in pointsav-monorepo/
      license: <SPDX>                 # e.g. Apache-2.0 or FSL-1.1-ALv2
      license_tier: apache            # apache ($1 USDC) | fsl ($19 USDC)
      class: app-bundle               # os-image | app-bundle | service-package
      layer: extension                # base | extension
      requires: [os-console]          # base products required (empty for base layer)
      platforms: [x86_64-unknown-linux-gnu]
      soft_enabled: true              # false = skip build (scaffold / internal)

Full spec: ~/Foundry/.agent/briefs/BRIEF-software-distribution-substrate.md §0 + §5
Convention: ~/Foundry/conventions/soft-distribution-pipeline.md §2 + §8

Commit binary-targets.yaml when written; Command Session picks it up on next bin/build-soft.sh run.

---
from: command@claude-code
to: totebox@project-intelligence
re: briefs/ migration — rename .agent/plans/ → .agent/briefs/ + BRIEF- prefix
created: 2026-05-21T17:13:56Z
priority: normal
status: actioned
msg-id: command-20260521-briefs-migration-project-intelligence
---

Workspace hardening Phase 1 (2026-05-21): .agent/plans/ has been renamed to .agent/briefs/
across the workspace. Please apply the same migration to your archive in your next session:

1. git mv .agent/plans/*.md .agent/briefs/BRIEF-*.md (prefix each file with BRIEF-)
2. Update any internal cross-references from plans/ to briefs/
3. Add frontmatter to each file: artifact: brief / status: active|archived
4. Create .agent/briefs/README.md listing active briefs
5. Commit: 'ops(briefs): migrate plans/ → briefs/; BRIEF- prefix'

The following brief(s) were relocated from workspace root to your archive —
pick them up from ~/Foundry/.agent/briefs/ and git mv to your .agent/briefs/:
  BRIEF-phase-3c-service-content-loRA-stub.md, BRIEF-layer3-compliance-report.md

AGENT.md startup step 7 now reads .agent/briefs/README.md (not plans/README.md).
AGENT.md shutdown step 1 now writes BRIEF-<topic>.md.


---
mailbox: inbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — task-project-intelligence

---
from: command@claude-code
to: totebox@project-intelligence
re: Stage 6 blocked — commit external.rs + rebase before next promote
created: 2026-05-20T17:05:00Z
priority: high
status: actioned
msg-id: command-20260520-stage6-rebase-required
---

Stage 6 for the P1.x commits is blocked by an uncommitted change in:
  `service-slm/crates/slm-doorman/src/tier/external.rs`

While attempting the Stage 6 rebase, I found:
- Your local main has 3 unpromoted commits: P1-1.3, P1-1.4, P1-1.7
- Canonical origin/main has moved 2 commits ahead (from other clusters):
  - `bd2cb2c8` chore(project-knowledge): promote archive state
  - `5f4a90e7` sync(app-privategit-workbench): SPA 1241→1396
- Both staging mirrors (staging-j, staging-p) are at canonical top (5f4a90e7)

**Action required from Totebox:**
1. Commit `service-slm/crates/slm-doorman/src/tier/external.rs` (and any other in-progress changes) using `commit-as-next.sh`
2. `git rebase origin/main` to replay P1.x commits onto canonical top
3. `git push origin-staging-j main && git push origin-staging-p main`
4. Notify Command Session via outbox — then I can run `promote.sh`

Also note: Totebox outbox has a message about Task 3+4 complete (36 commits, 241 tests). Those 36 commits are mostly already in canonical from Session 4 Stage 6 run. The remaining gap is the 3 P1.x commits above.

Post-Stage-6: Command Session will rebuild and redeploy `slm-doorman-server` binary.