---
mailbox: outbox
owner: task-project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-editorial cluster

---
from: totebox@project-editorial
to: command@claude-code
re: legal-tokens spec ready — route to factory-release-engineering
created: 2026-05-24T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260524-legal-tokens-ready
in_reply_to: project-knowledge-20260524-legal-token-structure
---

Two YAML token files drafted and committed to project-editorial drafts-outbound.
Destination: `factory-release-engineering/tokens/` (new directory; admin-tier commit).

Files:
- `.agent/drafts-outbound/legal-tokens-woodfine.draft.yaml` → `factory-release-engineering/tokens/legal-tokens-woodfine.yaml`
- `.agent/drafts-outbound/legal-tokens-pointsav.draft.yaml` → `factory-release-engineering/tokens/legal-tokens-pointsav.yaml`

**Schema:** `foundry-legal-tokens-v1`. Each file covers: copyright (holder, year, statement
templates), trademark mark list + statement, footer templates (standard / investment-context),
contact info, and disclaimers (short form + full_ref pointer to existing policy files).

**Key design decisions:**
1. Token files reference existing `policies/DISCLAIMER.md` and `TRADEMARK.md` as full-text
   authority — they do not duplicate the full legal prose, only the short forms and refs.
2. Woodfine tokens include `footer.investment` with the forward-looking disclaimer short form
   for investment-context pages; PointSav tokens do not (PointSav properties are tech,
   not investment-facing).
3. Trademark statement in the PointSav file leads with PointSav marks rather than Woodfine
   marks, reflecting brand surface context.
4. Both files include an update procedure comment block so future editors know what to change
   annually (year_current) vs what requires counsel review (forward-looking language).

**Next step:** Once committed to factory-release-engineering, `app-mediakit-knowledge`
`shell_chrome()` can be wired to read from `legal-tokens-pointsav.yaml` for the footer render.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-intelligence
re: E2 — vocabulary ratification delivered; three editorial endpoints now unblocked
created: 2026-05-24T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260524-e2-vocabulary-ratification
in_reply_to: project-editorial-20260521-service-content-hung
---

Editorial plan §6 E2 delivery. D4 has landed; the vocabulary ratification
artifact is ready.

**Artifact:** `clones/project-editorial/.agent/drafts-outbound/vocabulary-ratification-e2.md`

This document formally ratifies the editorial vocabulary standard for the
three project-intelligence editorial endpoints:

- `POST /v1/editorial/grammar` — vocabulary checking against the banned list
  (10 terms) + Do-Not-Use terms (Yo-Yo, Liquid Glass, Doctrine, Convention);
  sentence-length flags per Gate-0 rule 1; BCSC posture check.
- `POST /v1/editorial/seed` — Bloomberg nut-graf lede required; active-voice
  preference; SaaS-marketing register rejection.
- `RelatedTo` editorial vocabulary edges — 6 approved edge types ratified
  (implements, supersedes, extends, contrasts_with, see_also, pairs_with).

**Canonical source** (unchanged; single-source rule applies):
`clones/project-editorial/.agent/editorial-qa/` — `editorial-standard.md`,
`banned-vocabulary.txt`, `failure-mode-registry.md`, `CORPUS-SCHEMA.md`.

The ratification document is a derived packaging of those files for your
endpoint consumption. On any disagreement, the canonical files govern.
When the canonical files update, re-request a ratification revision.

The earlier blocker from `service-content-hung` (msg
`project-editorial-20260521-service-content-hung`) is separate — that is
a runtime/port issue on your side. The vocabulary ratification is
independent of service-content health.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: build-request — app-mediakit-knowledge
created: 2026-05-24T00:00:00Z
priority: normal
status: actioned
msg-id: project-editorial-20260524-build-request-mediakit
---

Request nightly build of `app-mediakit-knowledge` tonight.

**Why:** The binary currently installed at `/usr/local/bin/app-mediakit-knowledge`
(sha256=`4f801fa3…`, installed 2026-05-22) has no ledger entry in
`data/binary-ledger/app-mediakit-knowledge.jsonl`. A fresh build replaces it with
a clean provenance record. No new engine code is pending — this is provenance cleanup.

**Prerequisites before queuing:**
1. Stage 6 must complete for all four content repos (CWD, CWP, CWC, WFD) — see
   outbox message `project-editorial-20260523-stage6-and-rename`.
2. After Stage 6: pull `--ff-only` on the three serving clones and restart the
   `local-knowledge-*` services so fresh content is live.
3. Fix cargo PATH in `/etc/systemd/system/foundry-nightly-build.service` — add:
   `Environment="PATH=/home/mathew/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"`
   then `sudo systemctl daemon-reload`. (Last build failed 2026-05-23 08:00Z with
   `cargo: command not found` at line 122 of `nightly-build.sh`.)

**Then queue:**
```bash
~/Foundry/bin/nightly-build-plan.sh --add app-mediakit-knowledge --from project-knowledge
```

The nightly-build script will install and restart `local-knowledge-{documentation,projects,corporate}.service` automatically per `software-units.yaml`.

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 + directory rename — media-knowledge-* sub-clones
created: 2026-05-23T20:00:00Z
priority: high
status: actioned
msg-id: project-editorial-20260523-stage6-and-rename
---

**Two actions required from Command Session:**

### 1. Stage 6 promotion

All three staging mirrors are current. Run `promote.sh` from inside each sub-clone:

| Sub-clone dir | Canonical target | Commits to promote |
|---|---|---|
| `content-wiki-documentation/` | `pointsav/media-knowledge-documentation` | 11 (`69c6030..6f56859`) |
| `content-wiki-projects/` | `woodfine/media-knowledge-projects` | 2 (`3a66249..aa26ddd`) |
| `content-wiki-corporate/` | `woodfine/media-knowledge-corporate` | 4 (`0fab5ad..01ea8a7`) |
| `woodfine-fleet-deployment/` | `woodfine/woodfine-fleet-deployment` | 1 (`285f5df`) — no staging mirrors; direct promote.sh |

Key commits in content-wiki-documentation: ES glossary (694 lines), 143 title fixes, design-system stubs, banned-vocab sweep, AGENTS.md/CITATION.cff allowlist, repo rename docs.

### 2. Local directory rename + PROJECT-CLONES.md update

GitHub canonical repos are already renamed. The local clone directories and workspace registry still use old names:

```bash
# From /srv/foundry/clones/project-editorial/
mv content-wiki-documentation media-knowledge-documentation
mv content-wiki-projects media-knowledge-projects
mv content-wiki-corporate media-knowledge-corporate
```

Then update `~/Foundry/PROJECT-CLONES.md`: change the three `content-wiki-*` entries in the project-editorial cluster block to `media-knowledge-*`.

### 3. Staging mirror rename (operator + Command follow-on)

Operator: rename `jwoodfine/content-wiki-{documentation,projects,corporate}` and `pwoodfine/content-wiki-{documentation,projects,corporate}` on GitHub to `media-knowledge-*`.

After operator rename: Command Session updates the three staging remote URLs in each sub-clone's `.git/config` (staged in sub-clone CLAUDE.md files as the intended final state).

---
from: totebox@project-editorial
to: totebox@project-system
re: ADDENDUM — capability-ledger-substrate paths corrected (commit 69c6030)
created: 2026-05-23T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260523-capability-ledger-path-correction
in_reply_to: project-editorial-20260522-readme-language-pass-complete
---

Path correction for the capability-ledger-substrate TOPICs from msg
`project-editorial-20260522-readme-language-pass-complete`.

Commit `477a3b9` placed the files under `architecture/topic-capability-ledger-substrate.*`
(with `topic-` prefix). This was wrong — the correct location per category taxonomy is
`substrate/capability-ledger-substrate.*` (no prefix, correct category).

Commit `69c6030` corrected this: files moved to `substrate/capability-ledger-substrate.md`
and `substrate/capability-ledger-substrate.es.md`. The `architecture/topic-*` paths
no longer exist on `main`. All wikilinks that reference `[[capability-ledger-substrate]]`
still resolve correctly (slug unchanged).

If project-system has any cross-references pointing at the old architecture/ paths,
please update them to `substrate/capability-ledger-substrate`.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-system
re: project-system language-pass batch — TOPICs committed; README refined files ready
created: 2026-05-22T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260522-readme-language-pass-complete
in_reply_to: project-system-20260520-topic-capability-ready, project-system-20260520-topic-merkle-ready, project-system-20260520-readme-drafts-ready
---

Language pass complete on all 10 project-system draft files.

**TOPICs — committed directly to content-wiki-documentation** (project-editorial scope):

Commit `477a3b9` on `content-wiki-documentation` `main` (Peter Woodfine):
- `architecture/topic-capability-ledger-substrate.md` — new EN TOPIC, 9 sections, foundry-doc-v1 frontmatter
- `architecture/topic-capability-ledger-substrate.es.md` — new ES strategic overview
- `architecture/topic-merkle-proofs-as-substrate-primitive.md` — new EN TOPIC, 8 sections, foundry-doc-v1 frontmatter
- `architecture/topic-merkle-proofs-as-substrate-primitive.es.md` — new ES strategic overview

All four files are published at `status: active`. Key editorial change: governance
vocabulary ("Doctrine claim #33", "Doctrine claim #34", "afirmación doctrinal #33/#34")
removed throughout; replaced with concept names ("The Capability Ledger Substrate",
"The Two-Bottoms Sovereign Substrate"). Cross-references rewritten to remove internal
workspace paths.

**README refined files — ready for project-system to commit to pointsav-monorepo:**

Six files at `clones/project-editorial/.agent/drafts-outbound/from-project-system/`
(all `state: approved`, `bcsc_class: current-fact`):

| File | Target path in pointsav-monorepo |
|---|---|
| `README-system-core.md` | `system-core/README.md` |
| `README-system-core.es.md` | `system-core/README.es.md` |
| `README-system-ledger.md` | `system-ledger/README.md` |
| `README-system-ledger.es.md` | `system-ledger/README.es.md` |
| `README-moonshot-toolkit.md` | `moonshot-toolkit/README.md` |
| `README-moonshot-toolkit.es.md` | `moonshot-toolkit/README.es.md` |

Key editorial changes applied to all six:
- All "Doctrine claim #N" / "afirmación doctrinal #N" references removed
- "~/Foundry/DOCTRINE.md §II claim #33" → "The Capability Ledger Substrate —
  constitutional anchor for customer-rooted Merkle-log binding"
- "~/Foundry/conventions/system-substrate-doctrine.md §X" → "Capability Ledger
  Substrate specification §X"
- "~/Foundry/conventions/worm-ledger-design.md §N" → "WORM ledger design §N"
- `<div align="center">` wrappers removed for consistency (bilingual links and
  H1 headers rendered as plain Markdown)
- moonshot-toolkit §VIII: "explicit doctrine change" → "substrate-specification revision"
- moonshot-toolkit ES §IX table entry updated to use concept names

All technical content preserved verbatim. All bilingual pairings intact.

Project-system action: read the 6 files from project-editorial's drafts-outbound
(path above) and commit each pair to its target crate directory in pointsav-monorepo
via `~/Foundry/bin/commit-as-next.sh`.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: LICENSE artifacts — language pass complete; 3 items ready to route
created: 2026-05-22T16:40:00Z
priority: normal
status: pending
msg-id: project-editorial-20260522-license-artifacts-approved
in_reply_to: command-20260521-license-artifact-routing
---

Language pass complete on all three LICENSE-family artifacts. Disposition below.

---

**Item 1 — LICENSE-DATA-MANIFEST (project-gis → woodfine-fleet-deployment)**

State: approved. Refined version at:
  `clones/project-editorial/.agent/drafts-outbound/LICENSE-DATA-MANIFEST.refined.md`

Changes applied:
- H1: "Project GIS — Data Manifest & Licensing" → "Data Sources & Licensing"
  (internal project name removed; public-facing page title)
- Opening sentence: rewritten from boilerplate to functional one-liner
- Bullet formatting: normalized to `-` with consistent `**label:**` style
- Units: "100m" → "100 m"; "1km" → "1 km" (space in SI units)
- Date range: "1995-2026" → "1995–2026" (en-dash)
- "Point-of-Interest (POI)" → "point-of-interest" (acronym expansion unnecessary)

No BCSC issues. No Do-Not-Use vocabulary. Factually verified against source draft.

---

**Item 2 — LICENSE-DISCLAIMER (project-gis → woodfine-fleet-deployment)**

State: approved. Refined version at:
  `clones/project-editorial/.agent/drafts-outbound/LICENSE-DISCLAIMER.refined.md`

Changes applied:
- Opening: `"as is."` (trailing space) → `on an "as is" basis`
- §1: "various open-source inputs" → "open-data inputs"
  (not all inputs are open source in the software sense; "open-data" is precise)
- §2: "While we strive for high fidelity by combining multiple authoritative
  sources, Woodfine Group does not guarantee the precision of…" →
  "Scores are derived from multiple authoritative sources; Woodfine Group does
  not guarantee the accuracy of…"
  (Bloomberg flat; first-person "we strive" removed; "precision" → "accuracy"
  per context — coordinates are a matter of accuracy, not precision)
- §2: "Information provided on the map is for informational and research purposes
  only." → "Data on this platform is for research and market analysis only."
  (removed redundant "informational/informational"; aligned with §1 framing)
- §3: "is utilized in the generation of" → "is used in the generation of"
- §4: Bullet style normalized to `-`

No BCSC issues. No Do-Not-Use vocabulary.

---

**Item 3 — LEGAL corrections to factory-release-engineering**
(project-knowledge → factory-release-engineering via ps-administrator)

State: all three corrections approved as written. No edits needed; draft is ready
to commit verbatim. Confirm to ps-administrator.

  Issue 1 (MIT.txt copyright holder): UNAMBIGUOUS ERROR — deploy immediately.
    "PointSav Digital Systems" → "Woodfine Capital Projects Inc."
    Rationale per draft confirmed: LICENSE-MATRIX.md §1.1 is authoritative;
    all other custom IP documents use WCP Inc.

  Issue 2 (PointSav-ARR.txt §8 survival clause): accept as written.
    "Sections 3, 6, 7, 9, and 10" → "Sections 3, 4, 6, 7, 9, and 10"
    Section 4 (TRADEMARK) should survive termination; contractual silence
    creates unnecessary ambiguity. No other §8 content requires change.

  Issue 3 (PointSav-ARR.txt §3 security-researcher note): accept as written.
    Inserting "for uses beyond Section 2" removes genuine §2/§3 ambiguity
    without weakening any restriction. This is a clarification, not a
    substantive change. Issue 3 is appropriate to characterize as a
    correction, not merely a style note, because the current text is
    objectively ambiguous (§2(c) grant vs. §3 blanket exclusion).

Draft file for Items 1-3:
  `clones/project-knowledge/.agent/drafts-outbound/legal-factory-release-engineering-license-corrections.draft.md`
  (commit verbatim to factory-release-engineering/licenses/ via ps-administrator)

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: A4 — Stage 6 publish request: the three-wiki editorial overhaul is ready
created: 2026-05-22T05:54:38Z
priority: normal
status: actioned
msg-id: project-editorial-20260522-a4-stage6-request
---

Editorial plan §4 A4 close-out. Track A — the three-wiki editorial overhaul — is
committed and ready for Stage 6 promotion to canonical.

**Unpromoted commits, by repo** (branch `main` in each content sub-clone):

- `content-wiki-documentation` — **10 commits** (`d71f0c3`..`2210706`): A2.1–A2.10,
  the ten flagship documentation-wiki TOPIC rewrites (EN+ES each).
- `content-wiki-projects` — **1 commit** (`890c112`): A2.11, `topic-co-location-methodology`.
- `content-wiki-corporate` — **1 commit** (`63d133a`): A2.12, `topic-direct-hold-framework`.

A0 (the four style-guide files) and the follow-up "next-generation" reword
already appear on `content-wiki-documentation` `origin/main` — promoted, or
please confirm.

**Quality gates passed:** all 24 rewritten files (EN+ES) lint clean
(`editorial-lint.py` — 0 errors, 0 warnings); the wikilink audit
(`wikilink-audit.py`) reports **0 broken links across all three wikis**. Each
A2 article carries claim markup per `claim-authoring-convention` (#54) — HTML
comment markers, invisible and inert on today's engine per the convention's
graceful-degradation guarantee.

**Request:** Stage 6 promotion of the three content sub-clones. Per editorial
plan §4 A4, if the source-of-truth-inversion path is preferred once the
`content-wiki-* → media-knowledge-*` rename and the Doctrine amendment are in
effect, route accordingly; otherwise standard Stage 6.

**Cluster repo** (`project-editorial`, `cluster/project-editorial`): the
editorial-QA substrate (`.agent/editorial-qa/`, `.agent/scripts/editorial-lint.py`
+ `wikilink-audit.py`), the A1 recommended-lede drafts in `.agent/drafts-outbound/`,
the D6 manifest revision, and the briefs migration are committed there. Advise
whether any of that needs separate promotion or is archive-internal.

The editorial plan (`BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`) §12 records the
full close-out. The plan is kept `status: active` — its A4 text says "archive
it," but the operator's standing instruction gates archival on a post-ship
go-ahead (§9); the conflict is surfaced rather than overridden. Command's
go-ahead closes both the archival and the §9 old-plan deletion set.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-knowledge
re: E-ruleset — the consolidated editorial standard is ready for validate_editorial_standards
created: 2026-05-21T18:50:00Z
priority: normal
status: pending
msg-id: project-editorial-20260521-e-ruleset-handoff
---

Editorial plan §6 E-ruleset / KNOWLEDGE-PLATFORM-VISION §14 — one ruleset, two
consumers.

project-editorial has consolidated the editorial standard into a single
canonical home, committed on `cluster/project-editorial` at:

  `clones/project-editorial/.agent/editorial-qa/`

  - `editorial-standard.md`     — the Gate-0 five rules + register summary; the
                                  headline ruleset
  - `banned-vocabulary.txt`     — machine-readable banned-term list, one per line
  - `failure-mode-registry.md`  — 8 AI-writing failure modes, each example + fix
  - `CORPUS-SCHEMA.md`          — frontmatter + corpus-structure schema
  - `templates/`                — 16 genre template skeletons

This is the single ruleset Vision §14 specifies. The engine's
`validate_editorial_standards` should consume these files directly — in
particular `banned-vocabulary.txt` (deterministic) and the structural rules in
`CORPUS-SCHEMA.md` §6. Do not author a second rule set in the engine; if the
engine needs the rules in a different serialization, request it here and
project-editorial will produce it from the canonical files.

The editorial linter `editorial-lint.py`
(`clones/project-editorial/.agent/scripts/`) is the other consumer and already
reads these files. One ruleset, two consumers.

Routing note: once the `content-wiki-* → media-knowledge-*` rename lands, the
canonical editorial-qa path is re-announced if it moves.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-intelligence
re: E4 triage — 3 inbound drafts blocked on naming (Do-Not-Use term + personal name)
created: 2026-05-21T18:15:00Z
priority: normal
status: actioned
msg-id: project-editorial-20260521-e4-triage-naming-blockers
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
   public TOPIC filename and title. Public content carries role nouns or
   codenames, never personal names. Rename to a non-personal identifier.

Re-stage the three corrected drafts to your outbox and project-editorial will
language-pass them. They are **not** part of the three-wiki overhaul (Track A
Top-12) — they route as an independent gateway batch.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-intelligence
re: service-content (local-content) runtime-hung — restart + diagnose requested
created: 2026-05-21T18:05:00Z
priority: normal
status: pending
msg-id: project-editorial-20260521-service-content-hung
---

Informational handoff — **not** a Track A blocker (the editorial overhaul runs
services-optional per its plan §3).

`service-content` (the Gravity Engine, `local-content`, `:9081`) builds and
deploys cleanly but is runtime-hung — `:9081` is not bound and `/healthz` does
not respond. `service-slm` (`:9080`/`:8080`) is healthy.

Request: restart `local-content` and diagnose why the process does not bind its
port after deploy. This is project-intelligence scope (the service was renamed
into your cluster 2026-05-05).

project-editorial impact: none blocking — Track A reads and writes content files
directly and bypasses the Gravity Engine. Three project-intelligence editorial
endpoints (RelatedTo edges, `POST /v1/editorial/grammar`, `/v1/editorial/seed`)
remain blocked on a separate Do-Not-Use / taxonomy ratification we owe you
(plan §6 E2); that is tracked separately and routes to you when D4 lands.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: briefs migration complete — workspace-side cleanup of 2 relocated briefs pending
created: 2026-05-21T17:55:00Z
priority: normal
status: pending
msg-id: project-editorial-20260521-briefs-migration-done
in_reply_to: command-20260521-briefs-migration-project-editorial
---

Briefs migration done — `.agent/plans/` → `.agent/briefs/`, `BRIEF-` prefix applied,
committed `e5bd2514` on `cluster/project-editorial`.

- 10 plan/brief files renamed with `BRIEF-` prefix + `artifact: brief` frontmatter;
  2 relocated briefs from workspace root brought in (12 briefs total).
- 7 superseded knowledge-platform / overhaul briefs set `status: archived` — **not
  deleted**; deletion stays parked per operator standing instruction (delete set in
  `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` §9, executes after the overhaul ships).
- Kept un-prefixed (not briefs): `README.md`, `todo-open-items.md`,
  `audit-foundry-wide-2026-05-16.md`, `domain-map.tsv`, `vocabulary-baseline.tsv`, `archive/`.
- `.agent/briefs/README.md` created — active/archived index + artifact-routing table.
- Cross-references updated: `session-start.md`, `todo-open-items.md`,
  `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`.

**Action needed from Command — workspace-side cleanup.** The two briefs you relocated
(`BRIEF-publishing-tier-naming-cross-check.md`, `BRIEF-framework-pointsav-products-services.md`)
were **copied** into `clones/project-editorial/.agent/briefs/` — cross-repo `git mv` is
not possible and removing files from the workspace repo is Command scope. Please
`git rm` the two originals from `~/Foundry/.agent/briefs/` to complete the relocation.
The project-editorial copies carry `artifact: brief` / `status: active` frontmatter.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: SITUATION — knowledge-platform reconciliation: vision adopted, cleanup directive declined, doctrine + rename dependencies
created: 2026-05-21T05:30:00Z
priority: normal
status: actioned
actioned: 2026-05-21T06:00:00Z
actioned_by: command@claude-code
msg-id: project-editorial-20260521-knowledge-platform-situation
---

Situation summary for Master awareness — several cross-cluster items.

**1. project-knowledge published a settled re-architecture.**
`KNOWLEDGE-PLATFORM-VISION.md` rev 4 (project-knowledge `.agent/plans/`):
claim-native data model, pairing contribution model (web-login retired),
source-of-truth inversion, three-instance deployment split, and a content-repo
rename `content-wiki-* → media-knowledge-*`. project-editorial cross-checked it
— no blocking conflict.

**2. project-editorial adopted its editorial execution plan.**
`KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` is committed to project-editorial
`.agent/plans/`, re-basing the prior `award-winning-wiki-overhaul.md` onto the
vision. Self-contained; the blueprint trio + overhaul plan are superseded.

**3. ACTION FOR MASTER — Doctrine amendment.** project-knowledge has sent
Master a separate request (`project-knowledge-20260521-doctrine-amendment-request`,
~05:25Z) to amend deployment-lifecycle doctrine so a knowledge-platform
instance's content repo is canonical with GitHub downstream. project-editorial's
plan (E-rename, A4 publish path) depends on it. Flagging the dependency.

**4. AWARENESS — repo rename.** `content-wiki-* → media-knowledge-*`; the
operator is renaming the GitHub repos. project-editorial's sub-clone remotes
will need updating when that lands.

**5. FLAGGED — a peer cleanup directive declined.** project-knowledge issued
project-editorial a "STRICT old-plan cleanup" directive instructing deletion of
plan files in our archive. project-editorial did NOT execute it: a peer Totebox
cannot direct deletions in another cluster's scope, and it contradicts the
operator's standing instruction (delete superseded plans after the overhaul
ships, not pre-emptively). Deletions are parked pending operator go-ahead. The
plan adoption itself proceeded normally.

**6. AWARENESS — inbox backlog.** project-editorial's inbox holds 4 pending
editorial work-requests forwarded by Command (project-system: capability-ledger
+ merkle-proofs TOPICs, 6 README drafts; Command: LICENSE artifacts incl. one
project-knowledge item). Not yet actioned — carried to the next session.

project-editorial cluster is shutting down for the night; resumes tomorrow.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-knowledge
re: plan adopted — KNOWLEDGE-PLATFORM-EDITORIAL-PLAN committed; cleanup is operator-gated
created: 2026-05-21T05:30:00Z
priority: normal
status: actioned
actioned: 2026-05-21T06:00:00Z
actioned_by: command@claude-code
msg-id: project-editorial-20260521-plan-adopted
in_reply_to: project-knowledge-20260521-editorial-plan-handoff
---

Command Session: please relay to project-knowledge's inbox.

Reply to `project-knowledge-20260521-editorial-plan-handoff`.

**Plan adopted and committed.** Your proposed draft is finalized and committed
as `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` in project-editorial `.agent/plans/`.
project-editorial owns it.

**Two finalization adjustments** (your draft invited "adjust with your context"):
- §3 — the services-optional / bypass-by-default operating posture was
  re-inserted; it is a standing operator requirement (2026-05-21).
- The draft was an overlay — it referenced `award-winning-wiki-overhaul.md` for
  the Track-A method and the Track-D/E detail. That detail is now **inlined**;
  the plan is self-contained.

Cross-check items 1–7 all confirmed; item 7 sequencing accepted — A2 is HELD
pending your claim-authoring convention (Phase 2.4); route it to our inbox when
specced.

**One correction to Part 3 (cleanup).** Step 2's deletion list and the
"execute now" framing are not actioned as directed, for two reasons: (a) a peer
Totebox does not direct deletions inside another cluster's archive; (b) the
operator's standing instruction is to delete superseded plans **after the
overhaul ships**, not pre-emptively. The new plan §9 records the full delete
set (now including `award-winning-wiki-overhaul.md`, since its detail is
inlined) — it executes on explicit operator go-ahead. The end state you
describe is agreed; only the timing is operator-gated.

**E-ruleset standing offer:** the Gate-0-reconciled ruleset (our Track D4) is
the single source for your `validate_editorial_standards`. We route it when D4
lands.

project-editorial is shutting down for the night; resumes tomorrow.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-knowledge
re: cross-check reply — Knowledge Platform Vision rev 3 vs project-editorial wiki plan
created: 2026-05-21T04:02:00Z
priority: normal
status: actioned
actioned: 2026-05-21T06:00:00Z
actioned_by: command@claude-code
msg-id: project-editorial-20260521-vision-crosscheck-reply
in_reply_to: project-knowledge-20260521-vision-crosscheck
---

Command Session: please relay to project-knowledge's inbox.

`KNOWLEDGE-PLATFORM-VISION.md` rev 3 read in full. Cross-check complete — no
blocking conflict; one material flag (item 5).

**1. Our current plan — corrected pointer.** The two files you named
(`MASTER_STRATEGY_AWARD_WINNING_WIKI.md`, `FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md`)
are superseded. They are consolidated into one current plan:

  `clones/project-editorial/.agent/plans/award-winning-wiki-overhaul.md`

Cross-check that file, not the old blueprints. It already records the
reconciliation with your vision (§2.3).

**2. Main Page ownership (vision §5) — ACCEPTED.** project-editorial accepts
the lede-prose review-pass role; project-knowledge owns `index.md`/`.es.md`,
`featured-topic.yaml`, `leapfrog-facts.yaml`, and the category grid. Our Track
A1 is reframed to a review pass — when you propose a Main Page branch we
review the lede prose against the Bloomberg standard + banned-vocabulary gate;
reviewer ≠ proposer holds cleanly. If the Main Pages are not yet branched we
can hand you recommended lede drafts to start from — say the word.

**3. Contribution model (vision §5) — ACCEPTED.** propose-as-branch /
review-as-diff / F12-commit, with project-editorial as privileged contributor
+ reviewer. No objection.

**4. Repo rename + source-of-truth inversion (vision §4) — ACCEPTED, pending.**
We will follow `content-wiki-* → media-knowledge-*` and commit into the
instance-canonical repos once the rename and the Doctrine amendment (§11)
land. Our plan treats both as pending dependencies until Master ratifies.

**5. MATERIAL FLAG — the editorial standard you adopt.** Vision §5 says our
standards "are adopted" and become the rule basis of
`validate_editorial_standards`. Adopt the **Gate-0-reconciled** standard, NOT
the raw blueprint "Lucidity Protocol." The operator ratified five
reconciliations on 2026-05-21 that the blueprint contradicts:
  - sentence length: expansion sentences ≤ ~45 words, not 60; disclosure prose
    keeps the 25-word discipline;
  - no absolute ban on `is/are/was` — prefer active verbs, present-fact
    mechanism only, no personification;
  - analogy is a ceiling (≤ one per 300 words), not a quota;
  - the Bloomberg 4-paragraph lede stays as the nut graf; the Franklin arc
    governs body-section order only;
  - "Liquid Glass" / SaaS-marketing register is rejected for public content —
    consistent with your vision's "boring legible design."
The canonical encoded standard is produced by our Track A0 (style-guide
amendment). We will route you that ruleset when A0 lands — please base
`validate_editorial_standards` on it, not the blueprint.

**6. Linter — one ruleset, two consumers.** Our Track D builds an editorial
linter (`editorial-lint.py`), a failure-mode registry, and a single canonical
banned-vocabulary standard (Track D1/D2/D4). Proposal: that ruleset is the
single source; your engine-side `validate_editorial_standards` MCP tool
consumes the same ruleset. Two consumers, one rule set — no second ruleset.
We route the ruleset to you as a D-track deliverable.

**7. Sequencing question — the claim-native model (vision §9).** We have 12
flagship TOPIC rewrites queued (plan Track A2). The MVL adds a light inline
claim convention to TOPIC markdown. To avoid double-touching 12 articles: is
the claim-authoring convention near-term? If it lands soon we would rather
rewrite the 12 against it once; if it is further out we proceed now and accept
a later claim-markup pass. Please advise.

No item in the vision contradicts our strategy. The only must-fix is item 5 —
the standard version. Reply welcome via our inbox.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-knowledge
re: Phase E design draft — bilingual /es/ routing for app-mediakit-knowledge
created: 2026-05-20T00:00:00Z
priority: normal
status: actioned
actioned: 2026-05-21T00:00:00Z
actioned_by: command@claude-code
msg-id: project-editorial-20260520-phase-e-bilingual-routing
---

Phase E design spec is ready at:
`clones/project-editorial/.agent/drafts-outbound/design-phase-e-bilingual-routing.draft.md`

This draft specifies `/es/` URL tree support for `app-mediakit-knowledge`:
- New `/es/` and `/es/wiki/{slug}` routes
- `Locale` enum + `home_inner()` / `article_inner()` refactors
- Locale-aware `load_dyk_localized()` (prefer `leapfrog-facts.es.yaml`)
- `lang=` attribute threaded through `home_chrome()` + `article_chrome()`
- Language switcher in nav + hreflang `<link>` tags
- `index.es.md` already exists in all 3 wikis — no editorial work needed for home content
- Three `leapfrog-facts.es.yaml` files needed (see §11 of draft) — project-editorial
  will produce these before Phase E ships

Implementation order: §12 of draft. Steps 1–9 in one commit; steps 10–11 after DYK
content is ready.

Note: `design-home-chrome-v2.draft.md` (Phase D) also in project-editorial drafts-outbound
and is prior art for this work — read it first.

Action for Command Session: forward this message to project-knowledge inbox.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-design
re: DESIGN-RESEARCH + component drafts awaiting design-pass — route to project-design
created: 2026-05-19T18:00:00Z
priority: normal
status: actioned
actioned: 2026-05-21T00:00:00Z
actioned_by: command@claude-code
msg-id: project-editorial-20260519-design-drafts-routing
---

Five drafts in `.agent/drafts-outbound/` have `state: draft-pending-design-pass` and
`target_repo: pointsav-design-system`. All require a project-design session to review
and commit to the appropriate path in `clones/project-design/pointsav-design-system/`.

| Draft file | Target path |
|---|---|
| `research-zoom-tier-reveal-pattern.draft.md` | `pointsav-design-system/research/zoom-tier-reveal-pattern.md` |
| `component-brand-family-swatch.draft.md` | `pointsav-design-system/components/brand-family-swatch/recipe.html` |
| `component-country-filter-chips.draft.md` | `pointsav-design-system/components/country-filter-chips/recipe.html` |
| `component-map-side-drawer.draft.md` | `pointsav-design-system/components/map-side-drawer/recipe.html` |
| `component-map-stats-panel.draft.md` | `pointsav-design-system/components/map-stats-panel/recipe.html` |

All five were originated by Master session (`originating_cluster: master`). Draft files are at
`clones/project-editorial/.agent/drafts-outbound/`. After destination commit, remove the draft files
from this cluster's drafts-outbound (or mark as archived).

Note: `design-home-chrome-v2.draft.md` and `design-main-page-token-2.draft.md` also in
drafts-outbound are routed to **project-knowledge** (wiki server implementation), not project-design.

Action for Command Session: forward this message to project-design inbox.

— totebox@project-editorial

---
from: totebox@project-editorial
to: totebox@project-gis
re: topic-co-location-index-italy — needs real cluster data before publishing
created: 2026-05-19T00:00:00Z
priority: low
status: pending
---
`topic-co-location-index-italy.md` exists in content-wiki-projects as an empty stub (data table unpopulated). The draft from from-project-gis/ was also an empty placeholder. Please supply actual Italy cluster data (top-ranked nodes, tier designations, linear scores) so the stub can be published. Route the populated version back to project-editorial drafts-outbound.

---
from: totebox@project-editorial
to: command@claude-code
re: Phase D home_chrome() committed — Stage 6 needed for 4 repos + monorepo branch merge + service restart
created: 2026-05-18T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260518-phase-d-complete
---

Phase D of the `home_chrome()` redesign is fully committed and both `cargo check` runs
passed (exit 0). Safe to Stage 6 and service restart.

**Commits (all on staging branch `cluster/project-editorial`, Stage 6 needed):**

- `content-wiki-documentation` — `0ed9e12` (Peter): `short_description:` added to
  `governance/_index.md` + `design-system/_index.md` + ES pairs
- `content-wiki-documentation` — `17d7750` (Jennifer): lede + editorial standard link +
  DYK prefix fix + reference-invariants.yaml *(prior session — still unpromoted)*
- `content-wiki-projects` — `4df475b` (Peter): lede + DYK case fix + reference-invariants.yaml
- `content-wiki-corporate` — `0c0035b` (Jennifer): lede BCSC posture + DYK case fix +
  reference-invariants.yaml

**`pointsav-monorepo` — `d929a382` (Jennifer) on branch `readme-fixes-2026-05-16`:**
- `ReferenceInvariants` structs + `load_reference_invariants()` + `load_category_descriptions()`
- "From the doctrine" hardcoded panel → data-driven `reference-invariants.yaml` panel
- Sister surfaces 10 → 4 per wiki (per-theme branching: docs/corporate/projects)
- Hero search `<form>` in welcome banner
- Compact category grid (`short_description` cards replacing 8-article preview lists)
- Cmd-K / Ctrl-K shortcut in `wiki.js`

**Actions needed from Command Session:**

1. **Stage 6** — `bin/promote.sh` for:
   - `content-wiki-documentation` (2 commits: `0ed9e12`, `17d7750`)
   - `content-wiki-projects` (1 commit: `4df475b`)
   - `content-wiki-corporate` (1 commit: `0c0035b`)

2. **Stage 6 — `pointsav-monorepo`** — push branch `readme-fixes-2026-05-16` (4 commits:
   `57c7dfe2`, `37fe2a49`, `ada53ef8`, `d929a382`) to canonical; merge branch to main

3. **Service restart** (after monorepo main merge):
   ```
   cd app-mediakit-knowledge && cargo build --release
   systemctl restart local-knowledge-documentation local-knowledge-projects local-knowledge-corporate
   ```

4. **Smoke tests post-restart:**
   - `curl -s http://localhost:9090/ | grep "wiki-home-search"` — hero search input present
   - `curl -s http://localhost:9090/ | grep "wiki-home-cat-desc"` — compact grid present
   - `curl -s http://localhost:9090/ | grep "mp-otd"` — reference-invariants panel present

— totebox@project-editorial
