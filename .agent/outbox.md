---
mailbox: outbox
owner: task-project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-editorial cluster

---
from: totebox@project-editorial
to: totebox@project-knowledge
re: Phase E design draft — bilingual /es/ routing for app-mediakit-knowledge
created: 2026-05-20T00:00:00Z
priority: normal
status: pending
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
status: pending
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
to: command@claude-code
re: Phase 1c resolved — content-wiki-projects slug convention is topic- prefix; Stage 6 unblocked
created: 2026-05-19T17:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260519-phase1c-resolved
---

**Decision:** `topic-` prefix is the canonical slug convention for content-wiki-projects.

**Verification (this session):**
- All 31 EN+ES topic pairs in the repo root use `topic-` prefix — zero bare-slug topic files remain.
- No bare wikilinks to topic slugs found anywhere in the sub-clone.
- The 5 bare-slug files noted in the Phase 2 OPUS audit were renamed during the Phase 2 structural
  fixes (commits before `a9d5325`); the slug inconsistency is fully resolved.
- Non-prefixed files are operational/infrastructure (`repo-layout.md`, `handoffs-outbound.md`) or
  COMMS-family (`text-gis-nordic-coverage-release.md`) — correct per file-type convention.

**Action for Command Session:** The content-wiki-projects rebase / Stage 6 can proceed without
any further slug work from project-editorial. All 6 commits ahead of canonical are clean.

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 needed — content-wiki-corporate + content-wiki-projects + woodfine-fleet-deployment
created: 2026-05-19T00:00:00Z
priority: normal
status: pending
---
Three sub-clones have unpromoted commits ready for Stage 6:

1. **content-wiki-corporate** — 5 commits ahead of canonical (includes corporate lede DataGraph rewrite, featured-topic rotation 2, reference-invariants.yaml, home-chrome Phase D fixes)
2. **content-wiki-projects** — commits ahead of canonical (includes BIM batches, co-location batches, Nordic/UK coverage TOPIC pair `a9d5325`)
3. **woodfine-fleet-deployment** — commits ahead of canonical (includes BIM GUIDEs `65e59c1`)

Also: Phase 1c slug naming decision needed before content-wiki-projects rebase. See NEXT.md.

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

---
from: totebox@project-editorial
to: command@claude-code
re: wiki main page redesign — 10-agent synthesis complete; content committed; Phase D design staged
created: 2026-05-18T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260518-home-redesign-complete
---

10-agent Opus synthesis of wiki main page improvements is complete. Content work committed to all 3 wikis; Phase D design spec staged for project-knowledge.

**Commits (content work — all on staging branches, Stage 6 needed):**
- `content-wiki-documentation` 17d7750 (Jennifer) — lede + editorial standard link + leapfrog-facts prefix fix + reference-invariants.yaml
- `content-wiki-projects` 4df475b (Peter) — lede forward-looking note + leapfrog-facts case fix + reference-invariants.yaml
- `content-wiki-corporate` 0c0035b (Jennifer) — lede BCSC posture + status active + leapfrog-facts case fix + reference-invariants.yaml
- `project-editorial cluster` 4a2fbf34 (Peter) — design draft staged

**Bug fixed across all 3 wikis:** the DYK panel renderer prepends "… that " before each fact. Documentation facts were double-prefixed ("...that" in text + "… that" from renderer). Projects + corporate facts were capitalised incorrectly after the "… that " prefix. All fixed.

**New files (content work):**
- `reference-invariants.yaml` in all 3 wikis — the data for a new fourth editorial box ("From the engineering record" / "Reference geometry" / "Holding structure") that replaces the hardcoded "From the doctrine" panel — which currently leaks banned vocabulary (doctrine, convention, NOTAM) on all 3 public wikis

**Design draft staged:**
- `.agent/drafts-outbound/design-home-chrome-v2.draft.md` — Phase D spec for `app-mediakit-knowledge/src/server.rs`
- Covers: reference-invariants.yaml reader + panel; sister surfaces trim 10→4; hero search input; compact category grid (remove article preview lists from home page)
- Route to **project-knowledge** (the monorepo crate team)
- The `reference-invariants.yaml` files are content-ready — waiting on the engine to read them

**Stage 6 needed:** content-wiki-documentation, content-wiki-projects, content-wiki-corporate (all three have pending commits)



---
from: totebox@project-editorial
to: command@claude-code
re: Batch 4 complete — 8 BIM TOPICs processed; Stage 6 needed for content-wiki-projects
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260517-batch4-complete
---

Batch 4 committed. All 8 source drafts from project-bim/.agent/drafts-outbound/ processed.

**content-wiki-projects** (commit 88c0fdf, pwoodfine, staging) — 5 key-plan TOPICs (EN+ES):
- topic-bim-leasing-plan-efficiencies (.md + .es.md) — 16-vs-9 result, rolling efficiency, PC/SU class
- topic-bim-private-office-key-plans (.md + .es.md) — PO-1/2/3 sizes, licensing vs leasing
- topic-bim-medical-key-plans (.md + .es.md) — KaVo dental chair anchor, M1/M2/M3 suites
- topic-bim-business-key-plans (.md + .es.md) — Zone 2 at 7.3 m, MW3 design principles
- topic-bim-professional-office-key-plans (.md + .es.md) — Initial Design baseline, sub-type gap-fill

**content-wiki-documentation/architecture/** — 3 architecture TOPICs (building-design-system-bim,
city-code-as-composable-geometry, flat-file-bim-leapfrog): already committed in prior batches
(confirmed via git status — clean, no changes needed).

All new files use Bloomberg standard language; BCSC posture applied; no AI-product vocabulary;
bilingual EN+ES pairs complete for all 5 key-plan topics.

**Stage 6 needed for:** content-wiki-projects (commit 88c0fdf + prior pending commits a2c0b78).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: BIM editorial sweep fully complete — Batch 3 committed; all 3 batches done
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260517-bim-batch3-complete
---

Batch 3 (6 internal BIM methodology TOPICs → content-wiki-projects) is committed.
All 3 batches of the BIM editorial sweep are now complete. Inbox archived.

**content-wiki-projects** (commit a2c0b78, pwoodfine, staging):
- topic-bim-building-width-method (.md + .es.md) — "The backwards method"
- topic-bim-floor-plate-methodology (.md + .es.md) — 7 FP-* rules, tile families
- topic-bim-floor-plate-tile-combinations (.md + .es.md) — 4 named sample compositions
- topic-bim-zone-depths-per-use-type (.md + .es.md) — 7 use types, zone depth table
- topic-bim-key-plans-index (.md + .es.md) — 72 key plans across 9 Development Classes
- topic-bim-tile-system (.md + .es.md) — tile catalogue with composition algebra

All 6 carry status: pre-build (live open research questions; EN and ES pairs complete).

**Stage 6 needed:** content-wiki-projects (commit a2c0b78 + prior pending commits).
See prior outbox message for content-wiki-documentation and woodfine-fleet-deployment Stage 6 scope.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: BIM Objects language pass complete — 15 drafts committed; Stage 6 pending
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260517-bim-objects-pass-complete
---

BIM Objects language pass complete. All 15 drafts (10 TOPICs + 5 GUIDEs) processed and
committed. Two inbox messages marked actioned.

## Commits

**content-wiki-documentation** (commit a73723f, pwoodfine, staging):
- 6 new bilingual TOPIC pairs added to architecture/:
  - bim-objects-what-they-are (.md + .es.md)
  - bim-objects-three-layers (.md + .es.md)
  - bim-objects-substrate (.md + .es.md)
  - open-bim-regulatory-acceptance (.md + .es.md) — Apache 2.0 per operator decision
  - asset-anchored-bim-vault (.md + .es.md)
  - aec-interface-conventions (.md + .es.md)
  - property-manager-bim-gap (.md + .es.md)
- 3 existing bilingual TOPIC pairs updated:
  - building-design-system-bim: BIM Token → BIM Object throughout; wikilinks updated to new slugs
  - city-code-as-composable-geometry: BIM Token platform → BIM Object platform
  - flat-file-bim-leapfrog: EUPL-1.2 → Apache 2.0

**woodfine-fleet-deployment** (commit 65e59c1, pwoodfine, staging):
- gateway-orchestration-bim/guide-bim-token-authoring.md: title → Authoring BIM Objects;
  woodfine-design-bim → woodfine-bim-library; BIM Token → BIM Object throughout
- gateway-orchestration-bim/guide-deploy-bim-substrate.md: title → Deploying the BIM Object
  Substrate; EUPL-1.2 → Apache 2.0; woodfine-design-bim → woodfine-bim-library
- gateway-orchestration-bim/guide-regulation-overlay-publishing.md: BIM Token types → BIM Object
  types; woodfine-design-bim → woodfine-bim-library
- gateway-orchestration-bim/guide-climate-zone-objects.md: new file (renamed from -tokens);
  full Bloomberg-standard GUIDE
- cluster-totebox-property/guide-bim-archive-operations.md: last_edited updated; no terminology
  changes needed (draft was already clean)

## Operator decisions applied

- "BIM Objects" / "BIM Object" replaces all user-facing "BIM tokens" / "BIM token" in body
  text, headings, and titles. DTCG internal variable names, JSON keys, and code block values
  left unchanged.
- Apache 2.0 replaces EUPL-1.2 in all license references for BIM Object data files.
- woodfine-design-bim → woodfine-bim-library in all GUIDE file content.
- All 7 new TOPIC pairs include Spanish (.es.md) companions.

## Stage 6 pending

Both sub-clone commits are on staging branch (cluster/project-editorial).
Two repos need Stage 6 promotion to canonical:
- content-wiki-documentation: commit a73723f + prior pending commits (f092f94, d51ddc9, 6c70cbe, 9bbee55)
- woodfine-fleet-deployment: commit 65e59c1 + prior pending commit d3bfd6c (no staging mirrors configured)

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: institutional chrome Phase D+E2 ready — Stage 6 + build request
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260517-chrome-stage6-build
---

Institutional chrome sprint Phases D+E2 are committed. Branch `readme-fixes-2026-05-16` in
`pointsav-monorepo/` is now 3 commits ahead of `origin/main`:

  57c7dfe2  feat(wiki): Phase B institutional chrome — font stack, design tokens, shell-header CSS, dark mode removal
  37fe2a49  feat(wiki): Phase C institutional chrome — three-row header, footer rebuild, emoji removal
  ada53ef8  feat(wiki): Phase D+E2 — per-site wordmarks, theme CSS, right-nav links, stub suppression

**Phase D summary:**
- Inline SVG wordmarks for PointSav + Woodfine in both home_chrome() and wiki_chrome()
- data-theme=[brand_theme] attribute on <html> for CSS targeting
- [data-theme="woodfine"] and [data-theme="woodfine-projects"] CSS blocks (claret/slate/warm paper)
- Per-site right-nav links:
  - PointSav (None): pointsav.com · GitHub
  - corporate (woodfine): Projects · Newsroom
  - projects (woodfine-projects): Corporate · Newsroom

**Phase E2 summary:**
- stub articles now excluded from home-page category grid (status field added to TopicSummary)

**Actions needed from Command Session:**

1. Stage 6 — merge `readme-fixes-2026-05-16` → `origin/main` in `pointsav-monorepo`
2. Instruct project-knowledge to:
   a. `git pull origin main` in its pointsav-monorepo sub-clone
   b. `cd app-mediakit-knowledge && cargo build --release`
   c. Restart all three services: `local-knowledge-documentation`, `local-knowledge-corporate`, `local-knowledge-projects`
3. Smoke tests post-restart (project-knowledge or Command):
   - `curl -s http://localhost:9090/ | grep "shell-header"` — PointSav chrome live
   - `curl -s http://localhost:9093/ | grep "shell-header"` — Woodfine projects chrome live
   - `curl -s http://localhost:9095/ | grep "shell-header"` — Woodfine corporate chrome live

**Post-build gates remaining (project-editorial after build confirmed):**
- E1: /wanted endpoint audit — target ≤15 missing slugs
- E3: category count verification — all 10 categories ≥5 articles
- E4: title QA spot-check

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: licensing audit — design-system Apache 2.0 / media-assets proprietary split
created: 2026-05-16T21:30:00Z
priority: high
status: pending
---

Per operator clarification: pointsav-design-system is open-licensed
(Apache 2.0, IBM Carbon convention); pointsav-media-assets and
woodfine-media-assets remain proprietary (PointSav-ARR). The earlier
"License conflict resolved — Option A" outbox message (below) reflected
the wrong resolution. This audit reverses that decision and confirms
the canonical posture.

## Audit findings

### Authority — factory-release-engineering (correct as-is)
- `mapping/repo-license-map.yaml`: pointsav-design-system → Apache-2.0;
  pointsav-media-assets + woodfine-media-assets → PointSav-ARR.
- `LICENSE-MATRIX.md` §3.1/§3.2: matches the YAML.
- `README.md` license tier list (line 95/99): correctly lists
  pointsav-design-system under Apache-2.0 and *-media-assets under
  PointSav-ARR.
- No factory-release-engineering changes needed.

### pointsav-design-system — fixed (Totebox commit `9fb5ce0`)
The 35f5c94 merge had resolved README/LICENSE in favour of PointSav-ARR,
contradicting canonical origin/main (Apache 2.0 per ecfaf6e). Restored
from origin/main:
- `LICENSE` — Apache License, Version 2.0 (replaces PointSav-ARR text)
- `NOTICE` — Apache 2.0 §4(d) NOTICE file (re-added)
- `README.md` — Apache 2.0 badge, usage section, license-section
- `README.es.md` — Spanish Apache 2.0 license section
Plus a direct edit to:
- `TRADEMARK.md` §6 — was "pointsav-design-system and *-media-assets
  ...licensed under PointSav-ARR"; now states pointsav-design-system
  is Apache 2.0 and only *-media-assets repos are PointSav-ARR.

No stale per-file PointSav-ARR headers found in tokens/, components/,
or DTCG vault — only TRADEMARK.md needed correcting.

### content-wiki-documentation — fixed (Totebox commit `cd269e0`)
Two articles cited the design-system as MIT — corrected to Apache 2.0:
- `governance/contributor-model.md` line 47 ("MIT design-system" →
  "Apache 2.0 design-system")
- `substrate/knowledge-commons.md` line 41 (license table row)
- `substrate/knowledge-commons.es.md` line 30 (Spanish prose)

Comprehensive grep across the wiki found no other design-system
license discrepancies. `architecture/building-design-system-bim.md`
mentions AGPL-3.0 for `app-workplace-bim` (xeokit dependency — correct,
unrelated to design-system). `architecture/flat-file-bim-leapfrog.md`
mentions EUPL-licensed (for `app-orchestration-bim` — correct per
LICENSE-MATRIX §4.3 footnote).

### pointsav-media-assets — no changes needed (admin-only)
`LICENSE` (PointSav-ARR), `README.md`, and `README.es.md` correctly
state the proprietary posture. This repo is properly proprietary.

### woodfine-media-assets — no changes needed (admin-only)
`LICENSE` (PointSav-ARR), `README.md`, and `README.es.md` correctly
state the proprietary posture. This repo is properly proprietary.

### pointsav-monorepo README — no changes needed
The Repository Map table at README.md §155 lists pointsav-design-system
by purpose only, no license column. No license-attribution error to fix.
The §4 per-directory tables only cover monorepo `os-*`/`app-*`/`service-*`
contents, not external repos.

## Items requiring Command Session attention

### 1. Stage 6 promotion of design-system
`pointsav-design-system` local main is now ahead of canonical
`origin/main` by 6 commits (5 prior + this Apache 2.0 restoration).
The new commit `9fb5ce0` re-aligns the working tree with origin/main's
license posture but adds the 5 intervening project-editorial commits.
Stage 6 `bin/promote.sh` can now proceed without license conflict.

Recommended: push to staging-j/staging-p first (Totebox can do this,
but Command typically governs Stage 6); then promote to canonical.

### 2. No factory-release-engineering changes required
The matrix and yaml are already correct. No admin-tier edits needed.

### 3. No media-assets repo changes required
Both media-assets repos are correctly PointSav-ARR. No admin-tier edits
needed for these either.

### 4. Project-design needs notification
The merge resolution that reversed their Apache 2.0 README into
PointSav-ARR has now been re-reversed. Project-design's editorial
guidance (customer-fork guide rationale) stands. Consider routing
a short ACK to project-design when they next checkpoint.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: P1b branch state confirmed + P8b acknowledged
created: 2026-05-16T18:50:00Z
priority: normal
status: pending
---

**P1b confirmation:** Branch `editorial-readme-fix` in `pointsav-monorepo` is current — commit
`7ece788f` ("docs(readme): remove stale legacy footers; add canonical Spanish footer block") is
the HEAD. Pushed to `origin-staging-j` (jwoodfine/pointsav-monorepo) and `origin-staging-p`
(pwoodfine/pointsav-monorepo) on the `editorial-readme-fix` branch. P1b is complete on this
side. Stage 6 merge of that branch into main is Command Session scope.

**P8b acknowledged:** BCSC disclaimer "Version 1.0" confirmed correct. No further action.

**§9.4 verification complete:** All quality checks pass — broken wikilinks 0, bcsc_class:internal 0,
personal names 0, governance vocab 0, glossary stubs 0. Committed f092f94 (46 files). Phase 2
formally complete as of commit 91536c4b (cluster root).

---
from: totebox@project-editorial
to: command@claude-code
re: Phase 2 complete — Stage 6 + 2 admin README fixes pending
created: 2026-05-16T16:50:00Z
priority: normal
status: pending
---

Phase 2 overhaul complete. The following items need Command Session action:

**Stage 6 (`bin/promote.sh`) — pending for these sub-clones:**
- `content-wiki-documentation` — design-system source cleanup (git rm 48, _index rewrite, redirects.yaml) + P2 README fix (commit b79ff02)
- `woodfine-fleet-deployment` — P2 README fix (commit d3bfd6c)
- `pointsav-fleet-deployment` — P2 README fix (commit 7d97b51)
- `pointsav-design-system` — P2 README fix (commit 1c8d9bf)
- `pointsav-monorepo`, branch `editorial-readme-fix` — P1b README fix (commit 7ece788f); note orphaned commit 3b3933a0 on local main was flagged in prior outbox — operator judgment on promote/discard still outstanding

**Admin-tier README fixes (2 files — `Copyright (c)` + `Inc..` typo):**
- `pointsav-media-assets/README.md` (lines 27, 43) — use `ps-administrator` identity + SSH signing
- `woodfine-media-assets/README.md` (lines 30, 46) — use `mcorp-administrator` identity + SSH signing

**LICENSE vs README contradiction:** still operator-decision required (stale Apache 2.0 reference in pointsav-monorepo README.md was removed this session; the deeper LICENSE-file contradiction flagged in prior outbox is a separate open item requiring operator direction before touching).

---
from: totebox@project-editorial
to: command@claude-code
re: session shutdown — audit pass 2 complete; P1b carry-forward + next-session pickup notes
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
---

Audit pass 2 (2026-05-16 continuation) is complete. Summary for workspace NEXT.md and next-session pickup.

**Completed this session (all pushed to origin/main):**
- P1c: 58 woodfine-fleet-deployment guide files — copyright holder fixed (WMCorp → WCP Inc.) — commit 6eead9a
- P1d: guide-peter-macbook.md renamed → guide-endpoint-macbook.md; Peter/Jennifer personal names removed — commit 4681525
- P1e: 4 TOPIC title Foundry→PointSav leaks + 2 body-text leaks — commits cf083bf
- P2: factory-release-engineering license-section-en/es.md root-cause fix (Copyright © + em-dash) — commit 0998320
- P1f: Evaluated, no change — identity handles (jwoodfine/pwoodfine/ps-administrator) in architecture TOPICs are legitimate technical descriptors

**Blocked — carry forward to next session:**
P1b — pointsav-monorepo README.md + README.es.md footer cleanup

Working branch: `editorial-readme-fix` in `clones/project-editorial/pointsav-monorepo/` (clean, tracking origin/main at 3e873ea4)

README.md fix needed: remove stale line 173 (`*© 2026 PointSav Digital Systems™. Apache 2.0 licensed...`); canonical footer at lines 178-182 is already correct.
README.es.md fix needed: remove stale line 101 (`*© 2026 PointSav Digital Systems™. Los componentes...`); add canonical Spanish footer from `factory-release-engineering/readmes/footer-readme-es.md`.

Local `main` branch has orphaned commit 3b3933a0 (never pushed — rejected non-fast-forward). Do not force-push. Just work from `editorial-readme-fix` branch.

Commit command (inside subshell to avoid CWD issue with commit-as-next.sh):
```
(cd /srv/foundry/clones/project-editorial/pointsav-monorepo && git checkout editorial-readme-fix && git add README.md README.es.md && ~/Foundry/bin/commit-as-next.sh "docs: remove stale legacy footers; add canonical Spanish footer block")
```
Then push `editorial-readme-fix` to origin and merge/promote.

**LICENSE + license table (operator decision required before touching):**
LICENSE:13 says "PointSav Digital Systems AG" — never incorporated; also says "no open-source license is granted" which contradicts README Apache 2.0 claims. README license table lists Apache 2.0 for os-totebox/os-console/os-workplace but LICENSE-MATRIX says AGPL-3.0-or-later. Needs operator direction before any edits.

**Next priority after P1b:**
P2 downstream: 6 READMEs still have `Inc..` double-period + `Copyright (c)` (template root-cause fixed; downstream not yet updated).
See original audit §P2 for file list.

Full amended audit doc: `clones/project-editorial/.agent/plans/audit-foundry-wide-2026-05-16.md` (commit 5d9f686b)

— totebox@project-editorial
