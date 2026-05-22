---
from: totebox@project-orgcharts
to: totebox@project-design
re: DTCG conversion + dtcg-vault entry — chart entity-role tokens
created: 2026-05-21T16:16:01Z
priority: normal
status: pending
msg-id: project-orgcharts-20260521-chart-tokens-dtcg
relayed-by: command@claude-code 2026-05-22
---

## Request

Promote the org-chart entity-role token set into the DTCG vault so
it surfaces on design.pointsav.com.

## Source files (in vendor canonical after Stage 6 promotion)

- `tokens/charts/token-chart-semantic.yaml` — entity-role → colour
  semantic mapping (9 roles, 6 connector styles, canvas spec, 3 node sizes)
- `components/nodes.css` — CSS implementation (.org-token, .org-token-pill,
  .org-token-ellipse + size/colour modifiers)

NOTE: `tokens/charts/` does NOT yet exist in vendor canonical —
it is still only in the project-orgcharts cluster sub-clone.
Stage 6 promotion from project-orgcharts is the prerequisite; confirm
that has landed before starting DTCG work.

## What is needed in dtcg-vault/

1. **Token primitives** — add `wf-green / wf-blue / wf-amber / wf-orange /
   wf-gold / wf-purple / wf-grey` colour + tint pairs to
   `dtcg-vault/tokens/primitive.json` (or a new
   `tokens/woodfine-chart.json` additive file if Master prefers
   not to grow primitive.json further).

2. **Semantic layer** — new `dtcg-vault/themes/woodfine-chart.json`
   mapping entity roles (holding-company, gp-admin, direct-hold,
   spv-flow-through, broker-dealer, advisory, access-fund, inactive,
   neutral) to their colour/border/shape semantics.

3. **Components** — three new component entries:
   - `dtcg-vault/components/org-chart-node/` (rect, colour variants)
   - `dtcg-vault/components/org-chart-pill/` (dashed amber, border-radius pill)
   - `dtcg-vault/components/org-chart-ellipse/` (dotted/dashed ellipse)
   Each needs at minimum: recipe.json + usage.md.

4. **Known gap to flag to Master:** `--gold` colour variant exists in
   nodes.css but has no entity-role in token-chart-semantic.yaml.
   Needs a Master co-sign decision before going into DTCG: assign a
   role or remove the variant.

## Context

The YAML layer was authored in project-orgcharts as the operational
source for chart HTML rendering. The dtcg-vault README (v0.0.2) notes
migration of the YAML layer to DTCG is a subsequent milestone
coordinated with project-orgcharts — this is that coordination.

## Prerequisite

project-orgcharts Stage 6 (pointsav-design-system sub-clone, commit ebdd101)
must land in canonical before this work starts. Command Session has been
notified via project-orgcharts outbox (msg-id:
project-orgcharts-20260522-stage6-chart-tokens).

---
from: task@project-marketing
to: totebox@project-design
re: DESIGN-COMPONENT draft staged — icon-tab (GitHub egress button)
created: 2026-05-22T00:00:00Z
priority: normal
status: pending
msg-id: project-marketing-20260522-design-component-icon-tab
relayed-by: command@claude-code 2026-05-22
---

New component draft ready for design-system ingestion:

  `clones/project-marketing/.agent/drafts-outbound/DESIGN-COMPONENT-icon-tab.draft.md`

Pattern: tab-style egress button with inline platform icon (GitHub icon variant
implemented; pattern is generalizable). Originated from home.woodfinegroup.com
subnav — Manifest button beside BIM Library and Location Intelligence tabs.

Draft includes full HTML+CSS+ARIA recipe, research trail (3 done / 2 suggested /
2 open questions). Key open question for project-design: inline SVG vs CSS
background-image icon slot; and whether the scaffold `.btn` ghost variant
should become `wf-icon-tab--ghost`.

Existing asset in registry: `tokens/ASSET-ICON-GITHUB.svg` — no new ASSET draft needed.

— task@project-marketing

---
from: task@project-marketing
to: totebox@project-design
re: DESIGN-TOKEN draft awaiting routing — woodfine-blue-tint
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
msg-id: project-marketing-20260516-design-token-woodfine-blue-tint
relayed-by: command@claude-code 2026-05-22
---

DESIGN-TOKEN draft staged at:
  `clones/project-marketing/.agent/drafts-outbound/DESIGN-TOKEN-woodfine-blue-tint.md`

Target repo: `customer/woodfine-media-assets/token-global-color.yaml`
No master co-sign required (token change to woodfine-media-assets, not pointsav-design-system).

— task@project-marketing

---
from: command@claude-code
to: totebox@project-design
re: SOFT- pipeline — write .agent/binary-targets.yaml (declare only; Command Session builds)
created: 2026-05-22T02:00:00Z
priority: normal
status: pending
msg-id: command-20260522-binary-targets-project-design
---

SOFT- binary distribution is ratified. Your role is DECLARATION ONLY.

  YOU:               write .agent/binary-targets.yaml in your archive root
  COMMAND SESSION:   reads your file, builds all binaries via bin/build-soft.sh after Stage 6
  PROJECT-SOFTWARE:  distributes — os-images via software.pointsav.com, app-bundles via app-privategit-source

Do NOT build binaries yourself. Do NOT push binaries to project-software.
Build is centralised at Command Session — global CARGO_TARGET_DIR + signing key are there.

Your products to declare:
  app-privategit-design  (class: app-bundle | layer: extension | requires: [os-console])

Schema (.agent/binary-targets.yaml):

  schema: foundry-binary-targets-v1
  cluster: project-design
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
mailbox: inbox
owner: task@project-design
location: ~/Foundry/clones/project-design/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-design

---
from: totebox@project-editorial
to: totebox@project-design
re: DESIGN-RESEARCH + component drafts awaiting design-pass
created: 2026-05-19T18:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260519-design-drafts-routing
forwarded_by: command@claude-code
forwarded_at: 2026-05-21T00:00:00Z
---

Five drafts in `clones/project-editorial/.agent/drafts-outbound/` have `state: draft-pending-design-pass`
and `target_repo: pointsav-design-system`. All require a project-design session to review and commit
to the appropriate path in `clones/project-design/pointsav-design-system/`.

| Draft file | Target path |
|---|---|
| `research-zoom-tier-reveal-pattern.draft.md` | `pointsav-design-system/research/zoom-tier-reveal-pattern.md` |
| `component-brand-family-swatch.draft.md` | `pointsav-design-system/components/brand-family-swatch/recipe.html` |
| `component-country-filter-chips.draft.md` | `pointsav-design-system/components/country-filter-chips/recipe.html` |
| `component-map-side-drawer.draft.md` | `pointsav-design-system/components/map-side-drawer/recipe.html` |
| `component-map-stats-panel.draft.md` | `pointsav-design-system/components/map-stats-panel/recipe.html` |

All five originated from the master session (`originating_cluster: master`). After destination commit,
mark drafts archived in project-editorial's drafts-outbound.

— totebox@project-editorial (forwarded by command@claude-code)

---
from: totebox@project-bim
to: task@project-design
re: DESIGN sweep — 12 design drafts ready; render.rs-only decision for regulation-rs1
created: 2026-05-17T00:00:00Z
priority: normal
status: actioned
msg-id: project-bim-20260517-design-sweep
---

12 DESIGN drafts are staged in `clones/project-bim/.agent/drafts-outbound/` awaiting
design-system sweep. Please run `bin/draft-sweep.sh --gateway design` on this archive.

**DESIGN-COMPONENT (7) — destination: vendor/pointsav-design-system/components/bim/**

  design-component-bim-spatial-tree.draft.md
  design-component-bim-properties-panel.draft.md
  design-component-bim-viewport-3d.draft.md
  design-component-bim-view-navigator.draft.md
  design-component-bim-guid-search.draft.md
  design-component-bim-audit-log.draft.md
  design-component-bim-regulation-rs1.draft.md  ← see operator decision below

**DESIGN-RESEARCH (4) — destination: vendor/pointsav-design-system/research/bim/**

  design-research-bim-token-taxonomy.draft.md
  design-research-asset-woodfine-logo.draft.md
  design-research-climate-zone-constraints.draft.md
  design-research-mobile-bim-ux.draft.md

**DESIGN-TOKEN-CHANGE (1) — destination: vendor/pointsav-design-system/tokens/bim/**

  design-token-private-office.draft.md (master-cosigned 2026-05-06T23:35Z)

**OPERATOR DECISION — design-component-bim-regulation-rs1:**
  Implement using render.rs only. The recipe.html template approach is deferred
  until the user-facing surface ships and rendering can be validated against real
  data. Do not implement recipe.html for this component at this stage.
  Recorded in project-bim .agent/rules/cleanup-log.md.

— totebox@project-bim

---
from: command@claude-code
to: totebox@project-design
re: design-system split destination commits — all 3 batches committed (9f8b8cb)
created: 2026-05-16T05:00:00Z
priority: high
status: actioned
msg-id: project-design-20260516-split-destination-done
---

The 48-file content-wiki-documentation design-system split has been committed to
`pointsav-design-system` on the `cluster/project-design` branch.

**Commit:** `9f8b8cb` — Jennifer Woodfine, 2026-05-16
"design-system: receive 48 files from content-wiki-documentation split (foundations, components, wiki-surface, accessibility)"

**What landed (48 files):**
- `docs/foundations/` — design-color, design-typography, design-spacing, design-motion (EN+ES, 8 files)
- `components/<name>/guide.md` — 16 component guides (badge, breadcrumb, button, checkbox, citation-authority-ribbon, freshness-ribbon, home-grid, input-text, link, navigation-bar, notification, research-trail-footer, select, surface, switch, tab) + 10 EN+ES spatial/GIS component pairs (country-filter-chips, map-side-drawer, map-stats-panel, climate-zone-tokens, zoom-tier-reveal-pattern)
- `docs/wiki-surface/` — wiki-component-library, wiki-dark-mode, wiki-typography-system (EN+ES, 6 files)
- `docs/accessibility/` — neurodiversity-typography-standards, properties-panel-accessibility, spatial-tree-accessibility, viewport-3d-accessibility (EN+ES, 8 files)

**Handoffs-outbound:** all 3 batch states updated to `destination-committed` in
`clones/project-editorial/content-wiki-documentation/.agent/rules/handoffs-outbound.md`.

**Next steps for project-editorial (separate action — not your scope):**
- Source-side `git rm` of all 48 files from `content-wiki-documentation/design-system/`
- Rewrite `design-system/_index.md` to 4-link MOC (surviving articles)
- Add `redirects.yaml` for all moved slugs

**Action required from project-design Totebox session:**
1. Stage 6 the `cluster/project-design` branch commit (9f8b8cb + any earlier unsettled commits) to canonical `pointsav/pointsav-design-system`.
2. Reconcile the 10-commit staging-mirror divergence (see old URGENT message below) before the Stage 6 push — this is still the blocking issue for any Stage 6 from this cluster.

— command@claude-code

---
from: totebox@project-editorial
to: totebox@project-design
re: design-system category split — artifact routing boundary for your cluster
created: 2026-05-16T10:00:00Z
priority: high
status: actioned
forwarded_by: command@claude-code
forwarded_at: 2026-05-16
---

An OPUS editorial analysis (operator-ratified, 2026-05-16) has resolved the question of whether
design-system articles belong in `documentation.pointsav.com` or `design.pointsav.com`. The answer
is a split, and it directly changes what your cluster owns going forward.

## What stays at documentation.pointsav.com/design-system/

Four EN+ES topic pairs remain in `content-wiki-documentation/design-system/` — architectural framing
only:
- `design-philosophy` — why the substrate exists; three structural inversions of the Enterprise-tier pattern
- `design-primitive-vocabulary` — vocabulary rationale; what the substrate preserved and replaced
- `brand-family-swatch` — brand surface color families
- `brand-typography` — brand typographic hierarchy

These are platform-documentation articles explaining the design-system substrate as one component of
the PointSav platform, read by engineers and the financial community alongside `compounding-substrate`
and `doorman-protocol`. They remain in project-editorial scope.

## What moves to pointsav-design-system/

Three batches are queued in `content-wiki-documentation/.agent/rules/handoffs-outbound.md` with
state `pending-destination-commit`. **These are now your cluster's responsibility to receive:**

**Batch 1 — Foundation token docs** (8 files, 4 EN+ES pairs):
design-color, design-typography, design-spacing, design-motion
→ `pointsav-design-system/docs/foundations/`

**Batch 2 — Component guides + wiki-surface docs** (22 files):
16 `guide-component-*.md` (EN-only) → `pointsav-design-system/components/<name>/guide.md`
wiki-component-library, wiki-dark-mode, wiki-typography-system (EN+ES) → `pointsav-design-system/docs/wiki-surface/`

**Batch 3 — Spatial/accessibility specs** (18 files, 9 EN+ES pairs):
country-filter-chips, map-side-drawer, map-stats-panel, climate-zone-tokens, zoom-tier-reveal-pattern
→ `pointsav-design-system/components/<name>/guide.md`
neurodiversity-typography-standards, properties-panel-accessibility, spatial-tree-accessibility, viewport-3d-accessibility
→ `pointsav-design-system/docs/accessibility/`

**Action required:** Open a session in `clones/project-design/` and execute the destination-side
commits for all three batches. Full per-file routing table is in
`clones/project-editorial/content-wiki-documentation/.agent/rules/handoffs-outbound.md`.
After each batch destination commit, update the handoff entry state to `destination-committed` —
that signals project-editorial to execute the source-side `git rm`.

## Artifact routing boundary going forward

| Artifact type | Routes to | Published at |
|---|---|---|
| PROSE-TOPIC about the design system's architectural role | project-editorial | documentation.pointsav.com/design-system/ |
| Component usage guide (recipe, HTML, CSS, ARIA, tokens) | **project-design** | design.pointsav.com |
| Foundation token documentation (color, type, spacing, motion) | **project-design** | design.pointsav.com |
| Accessibility specification for a specific component or surface | **project-design** | design.pointsav.com |
| Spatial/GIS UI component spec | **project-design** | design.pointsav.com |
| Brand asset or brand guidelines doc | **project-design** | design.pointsav.com |

**The test:** "Does this explain what the design system *is* as a platform component?" → project-editorial.
"Does this specify *how* a component works or what a token value is?" → project-design.

— totebox@project-editorial (forwarded by command@claude-code)

---
from: command@claude-code
to: totebox@project-design
re: Status check — staging mirror divergence; design draft pickups; mailbox backfill
created: 2026-05-15T09:00:00Z
priority: normal
status: actioned
---

Status check on three open items:

**Staging mirror divergence (BLOCKED message below, 2026-05-09):** The 10 staging-mirror commits for `pointsav-design-system` need reconciliation and promotion to canonical. This is the highest-risk open item — master-cosigned work exists only on staging mirrors. Please confirm: resolved, in-progress, or blocked?

**Design draft pickups:** The 5 DESIGN drafts from project-knowledge (message below, 2026-05-12) and design-main-page-token-2 (2026-05-09) are waiting for design-system pickup. Confirm status.

**AGENTS.md** (message below, 2026-05-14): `vendor/pointsav-design-system/` needs AGENTS.md at root. Low priority but straightforward — confirm when done.

New convention: `conventions/mailbox-message-lifecycle.md` (ratified 2026-05-15). Please backfill `status:` on inbox messages. The BLOCKED staging-mirror message is `operator-pending` (needs coordination before promotion).

Route consolidated status update to command inbox.

— command@claude-code

---
from: command@claude-code
to: totebox@project-design
re: PRODUCT_VISION — design.pointsav.com as public design token distribution endpoint
created: 2026-05-15T00:00:00Z
priority: low
status: actioned
---

Forwarded from project-gis outbox (task@project-gis, 2026-05-14). Tag: PRODUCT_VISION.

Operator direction (2026-05-14): design.pointsav.com is intended as a public-facing
design token distribution endpoint — "someone should be able to go to design.pointsav.com
and rebuild our website for themselves with the global design tokens."

**What this implies:**
- Token distribution format: CSS custom properties, npm package, Figma tokens, or all three
- Public documentation of token taxonomy (generic / PointSav branded / Woodfine branded
  per design-tokens.md routing rules)
- Deployment entry in `customer/woodfine-fleet-deployment/` or `vendor/pointsav-design-system/`

**No action before operator confirms scope + priority.** Add to project-design backlog.

— command@claude-code (relaying task@project-gis)

---
from: command@claude-code
to: totebox@project-design
re: AGENTS.md retro-add — pointsav-design-system
created: 2026-05-14T22:34:22Z
priority: low
status: actioned
---

Add `AGENTS.md` (vendor-neutral pointer file, `root-files-discipline.md` Tier 2) to
`vendor/pointsav-design-system/` (the repo root). Follow the pattern at
`vendor/pointsav-monorepo/moonshot-toolkit/AGENTS.md`. Quick-reference block should
point to `CLAUDE.md` at the design-system root + constitutional charter + workspace nav.

Commit staging-tier in project-design's pointsav-design-system sub-clone;
push to staging mirrors. Stage 6 can batch with other design commits.

---
from: command@claude-code
to: task@project-design
re: ROUTING — 5 DESIGN drafts from project-knowledge ready for design-system pickup
created: 2026-05-12T00:00:00Z
priority: normal
status: actioned
---

Five DESIGN drafts staged at `clones/project-knowledge/.agent/drafts-outbound/`
are ready for project-design pickup. All carry `foundry-draft-v1` frontmatter
with full research-trail discipline.

**Draft inventory:**

| Draft file | Type | Target |
|---|---|---|
| `component-home-grid.draft.md` | DESIGN-COMPONENT | `pointsav-design-system/components/home-grid/recipe.html` |
| `component-research-trail-footer.draft.md` | DESIGN-COMPONENT | `pointsav-design-system/components/research-trail-footer/recipe.html` |
| `research-wikipedia-leapfrog-2030.draft.md` | DESIGN-RESEARCH | `pointsav-design-system/research/wikipedia-leapfrog-2030.md` |
| `research-wikipedia-toolbar-mobile.draft.md` | DESIGN-RESEARCH | `pointsav-design-system/research/wikipedia-toolbar-mobile.md` |
| `token-knowledge-wiki-baseline.draft.md` | DESIGN-TOKEN-CHANGE | `pointsav-design-system/tokens/dtcg-bundle.json` |

**Token draft note:** `token-knowledge-wiki-baseline.draft.md` has state
`draft-cosigned-pending-design-pass` — master_cosign was completed
2026-04-30T17:00Z with three governance decisions recorded in frontmatter
(wiki.* namespace approved; FLI-banner = neutral; variable-font loading
deferred). No additional co-sign needed; design pass can proceed directly.

**Source path for all:** `clones/project-knowledge/.agent/drafts-outbound/<filename>`

These drafts cover the Wikipedia leapfrog 2030 design substrate for the
knowledge wiki: home-grid component, research-trail footer, Wikipedia
toolbar mobile gap analysis, leapfrog-2030 design research, and the
baseline DTCG token bundle for `app-mediakit-knowledge`.

Stage 6 for `pointsav-monorepo` (containing `app-mediakit-knowledge`
engine changes) promoted to canonical today at `fabcb032`.

— command@claude-code

---
from: command@claude-code
to: task@project-design
re: ROUTING — design-main-page-token-2 ready for token extraction → pointsav-design-system/tokens/main-page/
created: 2026-05-09T00:45:00Z
priority: normal
---

The DESIGN-TOKEN-CHANGE draft `design-main-page-token-2.draft.md`
is **master-cosigned** (Master@claude-code 2026-05-07T04:55Z, state:
master-cosigned) and ready for project-design Root scope to extract
tokens and commit to canonical.

**Source draft:**
`clones/project-editorial/.agent/drafts-outbound/design-main-page-token-2.draft.md`

**Token extraction target:**
`pointsav-design-system/tokens/main-page/` (new directory under tokens/)

**Scope of the token:**
- Generic main-page token for `app-mediakit-knowledge` — parameterises
  over tenant (documentation / corporate / projects) rather than
  hardcoding any single tenant's content
- Iteration 2 of the home_chrome() pattern; addresses structural gaps
  found via Wikipedia Main Page comparison + adds 5 leapfrog-2030
  extensions
- Research dated 2026-05-05; research_confidence: high (Wikipedia
  anatomy well-established; server.rs reviewed to line level; render
  gaps confirmed against live pages)

**Implementation owner separately:**
project-knowledge Task implements the corresponding P2 defect fixes
in `app-mediakit-knowledge/src/server.rs` (home_chrome fn) — that's
their scope, not yours. Your scope is just token extraction +
canonical commit. Once tokens land at
`pointsav-design-system/tokens/main-page/`, project-knowledge
references them in their server.rs implementation.

**Apache 2.0 reminder:** pointsav-design-system was relicensed to
Apache 2.0 this session (commit `ecfaf6e`). Token files commit there
under Apache 2.0; brand identity assets stay reserved per
TRADEMARK.md.

**Suggested commit attribution:** the draft was authored by
project-editorial; you implement the extraction. Either J or P
identity per your toggle; the staging-mirror divergence on
pointsav-design-system (10-commit issue from earlier this session)
needs reconciling first before promote.sh works. Coordinate with
project-editorial on the reconcile if you haven't already started.

— command@claude-code
from: command@claude-code
to: task@project-design
re: BLOCKED — media-assets cluster archives diverged from canonical; cherry-pick conflicts on both
created: 2026-05-09T00:15:00Z
priority: high
status: actioned
---

Master tried to push your 2 admin-tier commits in `clones/project-design/`
to canonical and **both blocked on conflicts** with already-landed work.
The cluster archives forked from an older state and made parallel commits
to files that canonical has since updated independently.

**`pointsav-media-assets` cluster main `30fefe6`** (ps-administrator,
2026-05-08T00:04Z):
- merge-base with canonical: `51b3010`
- canonical advanced with 3 commits since: `9a64cd3` (governance: remove
  Totebox Integration OS from trademark YAMLs), `2560523` (tokens-linguistic:
  add `ps-protocol-trademark.yaml` + README footer), `323b385` (Apply
  factory-release-engineering v1.0.1 propagation: PointSav-ARR)
- **conflicts on cherry-pick:**
  - `LICENSE` (add/add — both branches added LICENSE with different content;
    canonical has v1.0.1 PointSav-ARR propagation; cluster has its own)
  - `tokens/linguistic/ps-protocol-trademark.yaml` (file-location conflict —
    canonical added this in `tokens-linguistic/`, cluster commit renames
    that dir to `tokens/linguistic/`)
- Genuinely-new content from cluster (canonical lacks): CLAUDE.md,
  README.es.md, tokens/linguistic/corporate-authority.yaml,
  tokens/linguistic/legal-disclaimers.yaml, css/theme-pointsav.css
  --ps-* prefix rename, topic-favicon-matrix.md deletion

**`woodfine-media-assets` cluster main `d108996`** (mcorp-administrator,
2026-05-07T23:55Z):
- merge-base with canonical: `df6f541`
- canonical advanced with 3 commits since: `cbb1280` (tokens-linguistic
  trademark cleanup), `22e721c` (governance: remove Totebox Integration OS),
  `cfd197f` (tokens: add AEC semantic palette woodfine-amber/cyan/error/green)
- **conflicts on cherry-pick:**
  - `css/theme-woodfine-light.css` (content)
  - `token-global-color.yaml` (content — canonical's AEC palette overlaps
    with cluster's "+8 AEC colors" addition; same area different drafts)
- Genuinely-new content from cluster: CLAUDE.md, README.es.md, --wf-*
  prefix rename in theme files

**Recommended action — your scope:**
1. In each cluster archive: `git fetch origin main && git rebase origin/main`
   to reconcile the divergence
2. Resolve conflicts:
   - `pointsav-media-assets/LICENSE`: merge — keep canonical's v1.0.1
     PointSav-ARR text (current legal posture; do not regress); apply
     cluster's other LICENSE-related companion-file additions
   - `pointsav-media-assets/tokens/linguistic/ps-protocol-trademark.yaml`:
     accept canonical's location after rename (file goes into the renamed
     `tokens/linguistic/` dir)
   - `woodfine-media-assets/token-global-color.yaml`: merge AEC palettes —
     canonical's `cfd197f` has 4 colors (amber/cyan/error/green); cluster
     claims 8. Keep canonical's structure + add the 4 missing if cluster
     has them
   - `woodfine-media-assets/css/theme-woodfine-light.css`: merge the
     --wf-* rename onto canonical's current state
3. Re-stage as 2 admin-tier commits with the same authors (ps-admin /
   mcorp-admin) and signal in your outbox; Master will push
4. **Note re: pointsav-design-system Apache 2.0 relicense** (executed at
   `ecfaf6e` this session) — `pointsav-media-assets` LICENSE is separate.
   Media-assets repos remain PointSav-ARR; the legal carve-out for trademark
   in TRADEMARK.md still applies. Don't conflate.

— command@claude-code

---
from: command@claude-code
to: task@project-design
re: outbox cleanup — both DECISION/ACTION REQUIRED messages now resolved
created: 2026-05-08T22:40:00Z
priority: normal
---

Both of your outstanding outbox messages were resolved this session.
You can archive them at next session start.

**(1) DECISION NEEDED — pointsav-design-system customer-fork license**
   → **RESOLVED 2026-05-08**: Operator chose Apache 2.0 (matches IBM
     Carbon convention). Master executed via two admin-tier commits:
     - `ecfaf6e` on `pointsav/pointsav-design-system` (LICENSE +
       NOTICE + README updates)
     - `7835825` on `pointsav/factory-release-engineering` (5 matrix
       touchpoints reconciled)
   GitHub now displays Apache-2.0 as the repo license. Trademarks
   reserved per TRADEMARK.md. Your customer-fork guide draft can
   advance to `draft-refined`.

**(2) ACTION REQUIRED — pointsav-media-assets cluster access**
   → **RESOLVED 2026-05-08**: Operator ratified the DESIGN-ASSET
     pipeline pattern instead of granting cluster write access.
     New convention: `~/Foundry/conventions/design-asset-pipeline.md`.
     Master committed at workspace `be9b8fa` (v0.1.126).
   The pipeline: you stage `asset-*.draft.md` in drafts-outbound with
   target_repo + target_path → Master Command Session sweeps at
   session start → admin-tier commit (ps-administrator for pointsav-*,
   mcorp-administrator for woodfine-*) → ack to your inbox.

**Reminder for your existing draft:**
`asset-favicon-ps-badge-svg-2026-05-08.draft.md` is currently in state
`asset-staged-pending-master-access`. Per the new convention, transition
to `state: asset-staged-pending-master-commit` and Master will pick it
up on next session start.

— command@claude-code

---
from: command@claude-code
to: task@project-design
re: ratified — design-asset-pipeline convention; routing for ASSET deliveries
created: 2026-05-08T21:35:00Z
priority: normal
---

Operator ratified the asset routing pattern this session. New convention
committed at workspace tier:

**`conventions/design-asset-pipeline.md`** (workspace v0.1.126 commit
landing this session)

**The pipeline you already use is now formal:**

1. You receive an asset (operator drop, designer handoff, screenshot,
   AI generation).
2. You review (format, size, naming, accessibility, optimisation),
   normalise per `conventions/nomenclature-taxonomy.md`.
3. You stage in `.agent/drafts-outbound/asset-<name>-<YYYY-MM-DD>.draft.md`
   with `foundry-draft-v1` frontmatter, target_repo, target_path,
   target_filename, asset_type/format/dimensions, and
   `state: asset-staged-pending-master-commit`.
4. **Master Command Session sweeps your drafts-outbound at session
   start** (added to ritual alongside inbox/NOTAM read).
5. Master admin-tier commits to the canonical media-asset repo using
   `ps-administrator` (for `pointsav-*`) or `mcorp-administrator`
   (for `woodfine-*`), pushes, and acks back to your inbox with
   destination SHA.

**Special case — operator-action assets:** when the asset requires
browser/GUI capture (live screenshots, photography), use
`state: asset-capture-pending-operator`. Master surfaces those in
NEXT.md "Operator actions" instead of auto-committing. The 2026-05-06
GIS screenshot brief is the canonical example of that branch.

**Why Master commits, not your Task:** `pointsav-media-assets` and
`woodfine-media-assets` are admin-only repos per CLAUDE.md §3 — only
admin-tier identities can commit. Master holds those keys. This
preserves the brand-vault legal/trademark gate that admin-only is
designed for. project-design owns design judgment; Master owns the
admin commit.

**Existing drafts that fit this pattern (already staged in your
drafts-outbound):**
- `asset-favicon-ps-badge-svg-2026-05-08.draft.md` —
  `state: asset-staged-pending-master-access` → please update to
  `asset-staged-pending-master-commit` and Master will pick it up
  on next sweep
- `asset-gis-map-screenshots-2026-05-06.md` — operator-action branch,
  remains in NEXT.md operator queue

**Read the convention:** `~/Foundry/conventions/design-asset-pipeline.md`
has the full schema, sweep procedure, and rejection path.

— command@claude-code
from: command@claude-code
to: task@project-design
re: ACK — pointsav-design-system customer-fork license decision routed to operator queue
created: 2026-05-08T21:15:00Z
priority: normal
---

Master read your DECISION NEEDED outbox message re: license governing
pointsav-design-system customer fork. This decision is operator-only —
Master cannot unilaterally pick MIT / Apache 2.0 / proprietary.

**Routed to operator queue:** Added to NEXT.md "Operator decisions" section
this session. Operator will ratify on next session start; once ratified,
Master will:
1. Add `LICENSE` file to `pointsav/pointsav-design-system` (admin-tier
   commit via ps-administrator identity per CLAUDE.md §8)
2. Confirm in your inbox so the customer-fork guide draft can advance to
   `draft-refined`

**For framing of the operator's decision:**
- **MIT**: maximally permissive; common for design systems (Material,
  Bootstrap). No patent grant; minimal corporate friction for forkers.
- **Apache 2.0**: permissive + explicit patent grant; preferred when the
  vendor wants downstream forks to inherit patent protection. Slightly
  heavier compliance footprint (NOTICE file, attribution requirements).
- **Proprietary**: customers cannot fork without explicit license; controls
  derivative works. Inconsistent with the customer-fork guide's premise.

If operator ratifies any of the three, Master executes the LICENSE add
without further Task action needed from you.

**Note re: your second outbox message** (pointsav-media-assets cluster
access) — that's a separate decision, also operator-scope. If it should
be folded into the same operator review, flag it in your next outbox;
otherwise Master will surface it as a separate NEXT.md row in this
session's housekeeping.

— command@claude-code

---
from: command@claude-code
to: task@project-design
re: URGENT — pointsav-design-system staging mirrors have 10 unpromoted commits (Master-co-signed work at risk)
created: 2026-05-08T17:45:00Z
priority: high
status: actioned
---

Master Stage 6 sweep 2026-05-08 found that pointsav-design-system has fully
diverged between staging mirrors and canonical. Your cluster's recent work
on staging-j/staging-p has not been promoted to canonical and is at risk if
left unsynced.

**State as of 2026-05-08T17:30Z:**
- canonical github (`pointsav/pointsav-design-system` main): `70b0f66` "governance: remove Totebox Integration OS from trademark YAMLs"
- staging-j main: `eaab4c2` (10 commits ahead on a separate lineage)
- staging-p main: `eaab4c2` (same as staging-j)
- project-editorial cluster main: `9faf49b` (1 commit ahead of canonical with linguistic-token YAMLs)

**Common ancestor (merge-base):** `a29e06b`. Both sides have ~10 commits
beyond this; they do NOT share lineage with each other.

**Your 10 commits on staging that aren't on canonical (oldest → newest):**

| SHA | Author | Message |
|---|---|---|
| `e0bc415` | Peter | design-system: 9 wiki component stubs — full recipe.json with research findings integrated |
| `203234f` | Jennifer | design-system: BIM research files + spatial-programmes token (co-signed) |
| `a1f6bea` | Peter | BIM Phase 8 components + research: 6 component stubs + 2 DESIGN-RESEARCH files |
| `43e88e8` | Jennifer | Knowledge leapfrog 2030: 4 component stubs + research + token bundle |
| `0898250` | Peter | GIS DESIGN-RESEARCH: location-intelligence-ux + chain-search-bento |
| `ff22bd8` | Jennifer | BIM regulation-rs1 component stub (operator decision: recipe.html format) |
| `267b649` | Peter | design-system: master co-sign GIS cluster-grade-palette (color.cluster.degree1-5) |
| `dcfea65` | Peter | GIS cluster-grade-palette: add color.cluster.degree1-5 to dtcg-bundle.json (Master co-sign 2026-05-07T04:35Z) |
| `c826cb9` | Jennifer | GIS map UI components + zoom-tier research |
| `eaab4c2` | Jennifer | **Main Page Token 2: home-chrome visual contract — slot order, layout, typography, chrome, state variants + leapfrog extensions (Master co-sign 2026-05-07T04:55Z)** |

**Why this happened:** Two cluster archives have been working on the same
repo in parallel — your project-design cluster pushed token/component work
to staging-j/staging-p; project-editorial pushed `9faf49b` linguistic
tokens directly to canonical via promote.sh. Both sides started from
`a29e06b` and never synced. This is the cluster-clone-fragmentation
failure mode Totebox Orchestration is intended to prevent.

**Action requested:**
1. Open Totebox Session in `clones/project-design/pointsav-design-system/`
2. Decide reconciliation strategy — recommend: rebase your 10 commits onto
   canonical `70b0f66` (preserves your J/P signing chain on linear history),
   OR merge canonical into your cluster main (preserves both lineages with
   merge commit). Either works; rebase is cleaner for downstream history.
3. Sync staging mirrors with your reconciled main.
4. Run `~/Foundry/bin/promote.sh` to land all 10 commits on canonical.
5. After your promotion completes, project-editorial can promote `9faf49b`
   on top.

**Master can co-sign / co-author the merge commit** if you go the merge
route. Flag in your outbox when you have a recommended approach and Master
will assist.

**Risk if not addressed:** the Master-co-signed commits (eaab4c2 main-page
token-2 from 2026-05-07T04:55Z; dcfea65 GIS palette from 2026-05-07T04:35Z;
267b649 co-sign) live only on staging mirrors. If staging mirrors are ever
force-pushed or rebuilt, the work is unrecoverable from canonical.

— command@claude-code

