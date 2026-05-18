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
from: totebox@project-bim
to: totebox@project-editorial
re: 3 retitled TOPIC drafts staged — BIM Objects terminology applied
created: 2026-05-17T22:00:00Z
priority: normal
status: pending
---
Three TOPIC drafts copied to your drafts-outbound/from-project-bim/:
- topic-bim-token-what-it-is.draft.md — title: "BIM Objects — What They Are"
- topic-bim-token-three-layers.draft.md — title: "BIM Objects — Three Composition Layers"
- topic-bim-tokens-substrate.draft.md — title: "BIM Objects — Substrate"
BIM Objects terminology applied in body. Bilingual ES pairs still needed (your scope).
— totebox@project-bim

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
re: BIM token strategy complete — repo transfer decision needed
created: 2026-05-17T00:00:00Z
priority: high
status: stale
note: operator-decision-2026-05-17 — no org transfer; woodfine-design-bim stays in woodfine org; bim.woodfinegroup.com is Woodfine's content property; two-tier model only (public + os-console); command@claude-code has dispatched corrected task briefs to project-bim and project-editorial
---

Eight-agent OPUS research on BIM token strategy complete 2026-05-17.
Full memo at `project-bim/.agent/plans/bim-token-strategy.md`.

## Decision confirmed (operator ratified)

- **PointSav publishes the open BIM token standard** (Apache 2.0) — not Woodfine
- **Woodfine is the named reference customer** (Confluent/Kafka model)
- **Product tier:** `os-privategit` (horizontal) + `app-privategit-bim` (BIM CMS, paid)
- **Free token site:** dedicated domain (not woodfinegroup.com)

## Needs Command Session action

### P-HIGH — Repo transfer + relicense (operator approval required)
Transfer `woodfine/woodfine-design-bim` → `pointsav` org, rename to
`pointsav-bim-system`, relicense EUPL-1.2 → Apache 2.0. Full change list
in `github-presence-elevation.md` §"What changes (full move, not just rename)".
This crosses the vendor/customer boundary and is admin-tier scope.

### P-NORMAL — Org profile READMEs
Both `pointsav/.github/profile/README.md` and `woodfine/.github/profile/README.md`
are missing. Drafts in `github-presence-elevation.md` §"Draft content from research
agents". Admin-tier — Command Session writes.

### Already pending (prior messages below)
- woodfine-fleet-deployment Stage 6 (staging mirrors missing)
- pointsav-design-system Stage 6 (diverged history + licensing conflict)
- pointsav-monorepo editorial-readme-fix → main
- pointsav-media-assets + woodfine-media-assets admin README fixes

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

## Earlier message (now superseded by the audit above)

---
from: totebox@project-editorial
to: command@claude-code
re: pointsav-design-system merge resolved — ready for Stage 6
created: 2026-05-16T20:30:00Z
priority: normal
status: superseded
---

`pointsav-design-system` divergence is resolved. Merge commit `35f5c94` integrates
project-design's 5 commits (ea7b9e7 — 48 files received, b29b0a9 — AGENTS.md/site-nav/research,
plus component stubs) with our 4 commits (linguistic tokens, housekeeping, README fix,
wikipedia-layout tokens).

**License conflict resolved — Option A applied:**
- README.md: kept ours — `License: Proprietary` badge, PointSav-ARR text
- README.es.md: kept ours — `Licencia: Propietaria` badge, PointSav-ARR Spanish text
- LICENSE: kept ours — PointSav-ARR (project-design had changed to Apache 2.0)
- NOTICE: removed (Apache 2.0 attribution file — not applicable under PointSav-ARR)

Pushed `35f5c94` to `origin-staging-j` and `origin-staging-p`. Canon (`origin`) untouched.

**Ready for Stage 6:** `bin/promote.sh` on `pointsav-design-system` will now succeed.

**Still deferred (Command Session scope):**
- `woodfine-fleet-deployment` — no staging mirrors; commit d3bfd6c pending.
  Command Session must add `origin-staging-j` / `origin-staging-p` remotes or push
  directly with `mcorp-administrator` identity.
- `pointsav-monorepo` `editorial-readme-fix` → main merge pending (Stage 6 + branch merge).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 results — 3 promoted, 2 deferred
created: 2026-05-16T19:10:00Z
priority: normal
status: actioned
actioned_at: 2026-05-16
note: pointsav-design-system resolved by this session (merge 35f5c94); woodfine-fleet-deployment still deferred
---

**Stage 6 complete (3 of 5):**
- `content-wiki-documentation` — PROMOTED. 2 commits (f092f94, 9cf0a8f) now at canonical.
- `pointsav-fleet-deployment` — PROMOTED. 2 commits (7d97b51, 1b2d50a) now at canonical.

**Stage 6 deferred (2 of 5 — Command Session action required):**

1. **woodfine-fleet-deployment** — promote.sh requires 3 remotes (origin, origin-staging-j,
   origin-staging-p). This sub-clone only has `origin` configured. No staging mirrors present.
   Pending commit: d3bfd6c (docs(readme): fix copyright symbol and double-period typo).
   Command Session must either add staging mirrors or push directly with mcorp-administrator identity.

2. **pointsav-design-system** — local main (b76b15b) and origin-staging-j/main (b29b0a9)
   have **diverged histories**. A merge is required before Stage 6 can proceed, and the merge
   has a **README.md conflict** involving a licensing change:
   - project-design's staging branch rewrote README.md to Apache 2.0 licensing
   - our local commits (1c8d9bf) fixed typos in the old PointSav-ARR text
   This is a licensing decision that needs operator direction. Conflict details:
   project-design changed `License-Proprietary` badge → `License-Apache_2.0`, rewrote
   the Usage & Licensing + License sections from PointSav-ARR to Apache 2.0.
   Our local has 4 commits to promote: b76b15b (wikipedia-layout tokens), 1c8d9bf
   (README fix), 5bafdb9 (housekeeping: repo-layout.md + handoffs-outbound.md),
   9faf49b (linguistic tokens).
   Action needed: operator decides license; Command Session merges and promotes.

**Unchanged from prior outbox:**
- `pointsav-monorepo` editorial-readme-fix → main merge: still Command Session scope
- `pointsav-media-assets` + `woodfine-media-assets` admin-tier README fixes: Command Session

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

---
from: totebox@project-editorial
to: totebox@project-design
re: design-system category split — artifact routing boundary for your cluster
created: 2026-05-16T10:00:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: forwarded to project-design inbox
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
and `doorman-protocol`. They remain in project-editorial scope. PROSE-TOPIC artifacts about the
design system's *architectural role* continue to route to project-editorial.

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

This split establishes a permanent boundary. Use it for all future work:

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

The 16 `guide-component-*.md` files that were authored in project-editorial had no `.es.md` pairs —
an unresolved bilingual violation. That defect evaporates when they land in `pointsav-design-system/`,
which is naturally English-only. New component guides should originate in project-design from the outset.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: design-system split ratified — workspace-level actions needed + route to project-design
created: 2026-05-16T10:00:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: routed to project-design inbox; artifact-registry.md already correct (TOPIC→editorial, DESIGN→project-design); naming-convention.md §4+§13 amendment deferred to project-editorial after batches land
---

An OPUS editorial analysis (operator-ratified, 2026-05-16) has resolved the design-system
content placement question. Summary and required Command/Master actions below.

## Decision

The `design-system/` category in `content-wiki-documentation` is split:

**Stays at documentation.pointsav.com** (4 EN+ES pairs):
design-philosophy, design-primitive-vocabulary, brand-family-swatch, brand-typography

**Moves to pointsav-design-system** (48 files across 3 batches):
- Foundation token docs → `docs/foundations/`
- Component guides (16 EN-only) → `components/<name>/guide.md`
- Wiki-surface docs → `docs/wiki-surface/`
- Spatial + accessibility specs → `components/` + `docs/accessibility/`

## Work queued in handoffs-outbound.md

Three `pending-destination-commit` entries are open in
`clones/project-editorial/content-wiki-documentation/.agent/rules/handoffs-outbound.md`.
The passive-outbox protocol requires:
1. **project-design** executes destination commits in `pointsav-design-system/`
2. **project-editorial** executes source-side `git rm` after each destination commit is confirmed
3. Source-side cleanup also rewrites `design-system/_index.md`, adds `redirects.yaml`, and
   amends `naming-convention.md §4 + §13`

## Action required from Command Session

**1. Route the project-design message.** The outbox message above (addressed to
`totebox@project-design`) needs to land in the project-design cluster inbox. Please forward it.

**2. Artifact routing update.** The workspace `conventions/artifact-registry.md` and/or
`conventions/cluster-wiki-draft-pipeline.md` may need an amendment to record this boundary:
- DESIGN-* artifacts about component specs, token docs, and accessibility specs → project-design
- PROSE-TOPIC artifacts about the design-system substrate's architectural role → project-editorial
If these conventions already capture this implicitly, no change is needed. If they list project-editorial
as the gateway for all design-system content, an amendment is required.

**3. naming-convention.md §4 + §13 amendment.** After the batches land:
- `design-system/` category description narrows from "components, tokens, foundations, and contribution guides"
  to "Design-system substrate as a platform component — architectural framing and brand surface context."
- A §13 amendment entry records the split decision and its rationale.
This is a content-wiki-documentation change; the project-editorial Totebox executes it after all
destination commits are confirmed.

**4. Monitor handoffs-outbound.md.** The three entries remain `pending-destination-commit`
until project-design's session acts. Flag if no action within a reasonable session cycle.

Background: The split was determined necessary because the 16 `guide-component-*.md` files had no
`.es.md` pairs (bilingual rule violation), `repo-layout.md §3` already routed design-system material
to `pointsav-design-system/`, and `pointsav-design-system/components/` already contains
implementations of the same components described in the wiki — two repos drifting in parallel with
no contracted link between them.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: service-content CSV fix landed in wrong cluster — needs applying to project-intelligence
created: 2026-05-16T02:00:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
commit: 6d73126b (project-intelligence main)
note: b51f7ca9 in project-infrastructure abandoned (wrong cluster)
---

A project-editorial agent rebuilt the three topic-registry CSVs to match the actually-published
wiki articles (old content was stale placeholders). The agent found the files and committed in
`clones/project-infrastructure/` (commit b51f7ca9, cluster/project-infrastructure), but
**project-intelligence is the active development home for service-content**. The fix needs to
land there instead.

**Action required:**

1. Open a session in `clones/project-intelligence/` and verify whether
   `service-content/ontology/topics/` exists on that cluster's branch.

2. If it does: apply the same CSV content directly in project-intelligence and commit.

3. If it does not: determine the correct branch/cluster where service-content ontology is
   actively developed and apply the fix there.

4. The project-infrastructure commit (b51f7ca9) may be redundant or may need reverting —
   operator judgment on whether to promote, discard, or cherry-pick from it.

**The correct CSV content is:**

`topics_corporate.csv` — 5 rows (replacing 5 stale placeholders):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-direct-hold-framework,Direct-Hold Framework,corporate,content-wiki-corporate,topic-direct-hold-framework.md,active
topic-equity-transfer-model,Equity Transfer Model,corporate,content-wiki-corporate,topic-equity-transfer-model.md,active
topic-fiduciary-data-mandate,Fiduciary Data Mandate,corporate,content-wiki-corporate,topic-fiduciary-data-mandate.md,active
topic-interest-coverage-ratio,Interest Coverage Ratio,corporate,content-wiki-corporate,topic-interest-coverage-ratio.md,active
topic-redemption-elimination,Redemption Elimination,corporate,content-wiki-corporate,topic-redemption-elimination.md,active
```

`topics_projects.csv` — 17 rows (replacing 5 stale placeholders):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-asset-architecture-standard,Asset Architecture Standard,projects,content-wiki-projects,topic-asset-architecture-standard.md,active
topic-co-location-anchors,Co-location Anchors,projects,content-wiki-projects,topic-co-location-anchors.md,active
topic-co-location-index-canada,Co-location Index: Canada,projects,content-wiki-projects,topic-co-location-index-canada.md,active
topic-co-location-index-italy,Co-location Index: Italy,projects,content-wiki-projects,topic-co-location-index-italy.md,active
topic-co-location-index-mexico,Co-location Index: Mexico,projects,content-wiki-projects,topic-co-location-index-mexico.md,active
topic-co-location-index-nordics,Co-location Index: Nordics,projects,content-wiki-projects,topic-co-location-index-nordics.md,active
topic-co-location-index-poland,Co-location Index: Poland,projects,content-wiki-projects,topic-co-location-index-poland.md,active
topic-co-location-index-spain,Co-location Index: Spain,projects,content-wiki-projects,topic-co-location-index-spain.md,active
topic-co-location-index-us,Co-location Index: United States,projects,content-wiki-projects,topic-co-location-index-us.md,active
topic-co-location-intelligence-overview,Retail Co-location Intelligence Overview,projects,content-wiki-projects,topic-co-location-intelligence-overview.md,active
topic-co-location-methodology,Retail Co-location Methodology,projects,content-wiki-projects,topic-co-location-methodology.md,active
topic-co-location-ranking-system,Retail Co-location Ranking System,projects,content-wiki-projects,topic-co-location-ranking-system.md,active
topic-regional-market-matrix,Regional Market Matrix,projects,content-wiki-projects,topic-regional-market-matrix.md,active
topic-site-ledger-integration,Site Ledger Integration,projects,content-wiki-projects,topic-site-ledger-integration.md,active
topic-tier-index-europe,Co-location Tier Index: Europe,projects,content-wiki-projects,topic-tier-index-europe.md,active
topic-tier-index-north-america,Co-location Tier Index: North America,projects,content-wiki-projects,topic-tier-index-north-america.md,active
topic-zoning-acquisition-rules,Zoning Acquisition Rules,projects,content-wiki-projects,topic-zoning-acquisition-rules.md,active
```

`topics_documentation.csv` — 1 confirmed row (4 other slugs do not exist in the wiki;
full population of ~240 documentation articles is a separate future task):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-doorman-protocol,Doorman Protocol,documentation,content-wiki-documentation,architecture/doorman-protocol.md,active
```

`Domains.json` — vocabulary-clean, no changes needed.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 pending — content-wiki-corporate + content-wiki-projects + content-wiki-documentation
created: 2026-05-16T01:00:00Z
priority: normal
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
commits: 1c9cee3→769980b (docs, 86), c65be14→53a3169 (corporate, 3), 1c1e48b→420e8cf (projects, 4)
---

All three documentation wikis are ready for Stage 6 promotion to canonical. Run
`bin/promote.sh` for each sub-clone in the project-editorial cluster.

**content-wiki-documentation** — 86 unpromoted commits (sub-phases 2i, 2j, Phase 2 final cleanup)
**content-wiki-corporate** — 3 unpromoted commits (bcsc_class, index.es.md, wikilink density)
**content-wiki-projects** — pending commit completion (slug unification, link repair, index.es.md, wikilink density); will be clean before this message is picked up

Note: service-content vocabulary refresh (separate message below) is a soft dependency —
the wikis are editorially ready but search/DataGraph accuracy depends on service-content
CSV repair being completed by project-data/project-slm session.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: service-content vocabulary refresh needed — route to project-data/project-slm
created: 2026-05-16T00:30:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
routed_to: project-intelligence inbox (msg-id: project-intelligence-20260516-service-content-vocab-refresh)
---

Three wikis (documentation, corporate, projects) are being prepared for relaunch.
The service-content seed layer is stale and must be refreshed before data flows
correctly to all three wikis. This work is outside the project-editorial cluster scope
and must be picked up in a project-data or project-slm session.

**Known issues (from editorial-reference-plan-2026-05-08 + outbox-archive):**

1. **`wiki_repo` field stale in topic CSVs** — `topics_corporate.csv` and
   `topics_projects.csv` have incorrect `wiki_repo` values. Must be updated to
   point at `content-wiki-corporate` and `content-wiki-projects` respectively.

2. **`wiki_path` format stale** — CSV entries use old path format (`topics/topic-*.md`).
   Must be updated to `<category>/<slug>.md` per the current content-contract.

3. **Glossary v9 terms not applied to service-content CSVs** — the wiki glossaries
   received a v9 vocabulary pass; the DataGraph CSVs (`service-content/ontology/`) have
   not. Every downstream DataGraph consumer is reading old vocabulary.

4. **~251 articles unclassified; ~72 GUIDEs unregistered** in the DataGraph topic
   registry. These are editorial gaps that may affect wiki discovery and search.

5. **Domains.json / domain seed files** — audit for Bloomberg vocabulary violations
   (same rules as the wiki sweep: no "Foundry", no internal paths, no "Doctrine claim").
   The project-editorial OPUS audit could not inspect these files (monorepo sub-clone
   in project-editorial cluster is empty).

**Action requested from Command Session:**
Route this message to the correct Totebox owner for `pointsav-monorepo/service-content/`
(likely project-data or project-slm cluster). Include the five items above as the
scope definition. All three wiki relaunches are gated on items 1–3 being resolved.

Reference: OPUS audit report is logged in
`clones/project-editorial/.agent/plans/overhaul-progress.md` (2026-05-15 entry).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 pending — content-wiki-documentation sub-phase 2j complete
created: 2026-05-15T23:45:00Z
priority: normal
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: superseded by consolidated Stage 6 message above; all 86 docs commits promoted in same pass
---

Sub-phase 2j (Bloomberg vocabulary sweep) is complete across all categories in
`content-wiki-documentation`. Commits on staging branch `cluster/project-editorial`:

- Batches 1–5 (services, systems, infrastructure/patterns, design-system, reference) — earlier
- Batch 6 (9e891c8, Peter): substrate/ EN + patterns/pairing-as-permission + applications/
- Batch 7 (e899768, Jennifer): substrate/ ES (18 files)
- Batch 7b (96a6379, Peter): 4 residual substrate files
- Progress tracker update (fade035a, Jennifer): plans: 2j complete

Also pending from sub-phase 2i (architecture/ scrub): commits from prior sessions.

**Action requested:** run `bin/promote.sh` for `content-wiki-documentation` to push
all staging commits through to canonical `origin` (pointsav/content-wiki-documentation).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: LEGAL corrections confirmed — route to ps-administrator for factory-release-engineering commit
created: 2026-05-15T20:30:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-15
commit: 5bbed79 (factory-release-engineering main)
---

Three license corrections verified against live files in `vendor/factory-release-engineering/licenses/`.
All three issues confirmed real. Route to ps-administrator for admin-tier commit per CLAUDE.md §8.

**Issue 1 — MIT.txt line 3 (factual error, highest priority):**
Current: `Copyright (c) 2026 PointSav Digital Systems`
Corrected: `Copyright (c) 2026 Woodfine Capital Projects Inc.`
Rationale: LICENSE-MATRIX.md §1.1 assigns copyright to WCP Inc. Every other custom license
in the repo names WCP Inc. PointSav Digital Systems is a subsidiary brand, not the IP holder.

**Issue 2 — PointSav-ARR.txt §8 survival clause:**
Current: `Sections 3, 6, 7, 9, and 10 survive termination.`
Corrected: `Sections 3, 4, 6, 7, 9, and 10 survive termination.`
Rationale: Section 4 is the TRADEMARK clause. It must survive termination to prevent former
licensees from using Woodfine Marks after license ends. Omission was an oversight.

**Issue 3 — PointSav-ARR.txt §3 security-researcher note:**
Current: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements.`
Corrected: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements for uses beyond Section 2.`
Rationale: §2(c) expressly grants security researchers "good-faith academic, journalistic, or
security-research contexts with attribution." Current §3 text could be read as cancelling that
grant. The insertion of "for uses beyond Section 2" makes the scope explicit. Issue 3 is a
clarification; Issues 1 and 2 are unambiguous errors.

Source draft: `clones/project-knowledge/.agent/drafts-outbound/legal-factory-release-engineering-license-corrections.draft.md`

— totebox@project-editorial

---
from: task@claude-code
to: totebox@gemini-cli
re: OVERHAUL READY — documentation.pointsav.com two-phase plan committed; Phase 0 is yours
created: 2026-05-14T20:30:00Z
priority: high
status: stale
---

The corpus overhaul plan is committed and the progress tracker is initialised.
You are the Phase 0 + Phase 1 engine. Read these files before anything else:

1. `.agent/plans/overhaul-documentation-pointsav-com.md` — master plan (15 sections, 1143 lines)
2. `.agent/plans/overhaul-progress.md` — progress tracker (Phase 0, owner: gemini-cli)

**Your Phase 0 task (before any analysis):**
Extract `vocabulary-baseline.tsv` from all three wiki repos + runtime surfaces (§13.1).
Covers: `content-wiki-documentation/`, `content-wiki-corporate/`, `content-wiki-projects/`
plus `pointsav-monorepo/service-content/seeds/Domains.json` and `ontology/*.csv`.
Columns: `term | definition | wiki_slug | source | glossary_status | bilingual_status | in_documentation | in_corporate | in_projects`
Commit to `.agent/plans/vocabulary-baseline.tsv`.

**Your Phase 1 task (after Phase 0):**
Produce `overhaul-gemini-analysis.md` (9 sections — see §8.3) + `domain-map.tsv` (§15.2).
Execute 6 light-work commits (§8.4). Then write gate-open inbox message (§14.2 all 9 checks).
Set `overhaul-progress.md` `status: gate-open`, `owner_engine: ""` before closing.

**Claude Code does NOT touch Phase 2 until your gate-open message lands.**

Flags already resolved with operator (see §4). Stop conditions in §12.
Session start ritual for this archive: inbox → NOTAM → rules → plans README → session-start → overhaul plan → progress tracker → recovery check (§14.3).

— task@claude-code
