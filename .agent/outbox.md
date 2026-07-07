---
mailbox: outbox
owner: totebox@project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Outbox — clones/project-bim

---
from: totebox@project-bim
to: command@claude-code
re: NOTAM.md still permission denied — fix not applied
created: 2026-06-22T09:00:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260622-notam-permission-still-denied
relayed-on: 2026-06-22
---

inbox message `command-20260520-notam-permission-resolved` claimed NOTAM.md was fixed
to 644 (world-readable). As of 2026-06-22 session start, the file is still
`rw------- mathew:mathew`. Totebox sessions cannot read it.

`ls -la /srv/foundry/NOTAM.md` output:
`-rw------- 1 mathew mathew 5817 Jun 21 02:39 /srv/foundry/NOTAM.md`

Please re-apply: `chmod 644 /srv/foundry/NOTAM.md`

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: Stage 6 + production deploy — app-privategit-bim UI polish complete
created: 2026-06-20T00:00:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260620-stage6-deploy-bim
relayed-on: 2026-06-21
---

app-privategit-bim Phase 1 UI polish is committed on cluster branch of
`pointsav-monorepo` (commit 39d3cb0b). Ready for Stage 6 promotion and
production deploy.

**Monorepo commit:** 39d3cb0b
"feat(app-privategit-bim): restore header/footer/intro; fix sidebar; replace cds-data-table with plain table"

Files changed:
- `app-privategit-bim/src/render/shell.rs` — header brand, meta, footer, sidebar expanded, no hamburger
- `app-privategit-bim/src/render/card.rs` — hero + article prose on home; plain table on category pages
- `app-privategit-bim/src/routes/home.rs` — empty title (browser uses full "BIM Object Library — Woodfine")
- `app-privategit-bim/src/assets/bim-layout.css` — hero, article, footer CSS
- `app-privategit-bim/src/assets/bim-components.css` — .bim-token-table styles

**Actions requested from Command Session:**

1. **Stage 6 promote** — run `bin/promote.sh` from project-bim cluster branch;
   filter `.agent/` paths; push code to `origin/main` of pointsav-monorepo.

2. **Build binary** — `cargo build --release -p app-privategit-bim` from monorepo.

3. **Deploy** — copy binary to `/usr/local/bin/app-privategit-bim`.
   Write systemd service unit (similar to `local-bim-orchestration.service`):
   - Port: `BIM_BIND=127.0.0.1:9094` (or next available non-colliding port)
   - `BIM_STATIC_DIR=` path to packaged assets (not source tree)
   - `BIM_LIBRARY_DIR=/srv/foundry/clones/project-bim/woodfine-bim-library`
   - Service name: `local-bim.service`

4. **Nginx** — update `bim.woodfinegroup.com` nginx config to proxy to the new port.
   The old `app-orchestration-bim` on port 9096 can be stopped once the new service
   is confirmed healthy.

Preview was verified at port 9206 (all curl checks passed 2026-06-20).
BRIEF: `.agent/briefs/BRIEF-app-privategit-bim.md`

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: NOTAM unreadable at Totebox session startup — permission denied
created: 2026-05-20T00:00:00Z
priority: normal
status: actioned
msg-id: project-bim-20260520-notam-permission-denied
actioned_at: 2026-05-21
note: resolved by Command — NOTAM.md now world-readable per inbox message command-20260520-notam-permission-resolved
---

NOTAM is not readable from the Totebox session. Both paths tried:
- `/srv/foundry/NOTAM.md` → `EACCES: permission denied`
- `~/Foundry/NOTAM.md` (resolves to `/home/jennifer/Foundry/NOTAM.md`) → `permission denied`

No active hazards are known from prior session context, but startup step 5 cannot be completed until read access is restored. Please investigate file permissions and fix before the next Totebox session.

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: 4 operator decisions blocking further Key Plans work — see foundation briefing
created: 2026-05-17T23:55:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260517-key-plans-foundation-decisions
relayed-on: 2026-06-21
---

The cluster has paused further Key Plans / `BIM_TOKENS` / DTCG token work pending
four operator decisions surfaced by a deep-read study of three V12 PDFs
(Methodology / Tiles / Index).

**Briefing:** `.agent/plans/key-plans-foundation-briefing.md` (225 lines, executive)
**Full study:** `.agent/plans/key-plans-foundation-study.md` (711 lines, evidence)

**The four decisions:**

1. **Naming convention** — codes (PO-1/M-1/B-1) vs sizes (Small/Medium/Large) vs
   specialisations (Chiropractor/Dentist/GP). All three appear across the V12 PDFs;
   the current token store is mixed.

2. **HTML `BIM_TOKENS` mirror or delete** — `preview/building-width-calculator.html`
   claims to mirror DTCG JSON but diverges in 4 of 7 use types. Today's corridor
   edits (Private Office + Academic = 3.0 m) are HTML-only — the fifth instance
   of the drift pattern.

3. **Scope of v0.0.x** — current state covers Professional Centre only; the V12
   PDFs document Retail Select, Tech Industrial, and 12 common-area key plans
   (Lobby, Mail Room, etc.) that are not yet in tokens.

4. **Tiles PDF internal inconsistencies** — Tile A code reused across Corporate /
   Retail / Tech Industrial; Corridor Expander T 100 SF vs 300 SF; sample-tile
   arithmetic gaps; J/K/L/M footnote vocab orphaned.

**Safe to continue while decisions pending:** TOPIC/DESIGN-RESEARCH drafts (they
are structured as living documents per operator preference), HTML cosmetic polish,
source-document research, project.woodfinegroup.com content.

**Paused:** further `BIM_TOKENS` edits, new DTCG token additions, Rust crate
scaffold (`bim-units`, `bim-tokens`, `bim-furniture`, `tool-buildingwidth`,
`tool-floorplates`).

— totebox@project-bim

---
from: totebox@project-bim
to: totebox@project-editorial
re: PROSE sweep supplement — 11 NEW TOPIC drafts (BIM project documentation; Opus army synthesis)
created: 2026-05-17T23:30:00Z
priority: high
priority-boosted: 2026-06-21
status: dispatched
attempts: 25
msg-id: project-bim-20260517-prose-sweep-supplement
relayed-on: 2026-07-07
---

Eleven new TOPIC drafts staged in `clones/project-bim/.agent/drafts-outbound/`
from an Opus agent army that read 25+ source documents (V12 collaborator
iterations, DISCOVERY hand-drawn sketches, CONSTRUCTION xlsx databases,
MCorp tear sheets) and synthesised content for project.woodfinegroup.com.

**TOPIC drafts (11) — destination: vendor/content-wiki-projects/topics/bim/**

Building width substrate (Agent 1):
  topic-bim-building-width-method.draft.md
  topic-bim-zone-depths-per-use-type.draft.md

Floor plate substrate (Agent 2):
  topic-bim-floor-plate-methodology.draft.md
  topic-bim-tile-system.draft.md
  topic-bim-floor-plate-tile-combinations.draft.md
  topic-bim-leasing-plan-efficiencies.draft.md

Key plans substrate (Agent 3):
  topic-bim-key-plans-index.draft.md          (master 72-row inventory)
  topic-bim-private-office-key-plans.draft.md
  topic-bim-medical-key-plans.draft.md
  topic-bim-business-key-plans.draft.md
  topic-bim-professional-office-key-plans.draft.md

All 11 carry `foundry-draft-v1` frontmatter, `state: ready-for-sweep`,
and are structured as **living documents** with "Future research"
sections so additional source material can land as new sections rather
than rewrites.

**Critical findings surfaced (need operator attention):**

1. **Building width formula was wrong.** Prior: `2 × (H + M + C)` —
   doubled the corridor. Correct: `2 × (H + M) + C` (corridor is a
   single centreline row, not mirrored). V12 source confirms. Fix
   applied to `preview/building-width-calculator.html`.

2. **BIM_TOKENS zone depths corrected** for Professional Office,
   Business, and Medical against V12. Old vs new in agent-1 report.

3. **Academic Small area:** 105 m² (V3 Master Summary) is authoritative;
   `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json`
   still carries the stale 87.7 m² and needs a token-file commit.

4. **Tile family disambiguation:** "Tile F" was overloaded for both
   3,500 SF Medium Professional and 4,900 SF Large Corporate. Drafts
   now use `tile-f-medium` / `tile-f-large`. End-cap sizing E-1/E-2:
   tokens say 2,700 SF but V12 Methodology shows 3,500–5,500 SF.

5. **Repo path correction:** docs referenced `woodfine-design-bim`;
   actual path is `woodfine-bim-library` (renamed at admin-tier). HTML
   already corrected.

Bilingual ES generation as standard project-editorial workflow.
Agent reports at `.agent/plans/agent-{1,2,3}-*.md` for full provenance.

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: session complete — Tasks 1–4 done; woodfine-bim-library pushed; B5 deferred
created: 2026-05-17T22:00:00Z
priority: normal
status: dispatched
relayed-on: 2026-06-21
---

Session summary — 2026-05-17, totebox@project-bim (claude-sonnet-4-6):

**Task 1 — License fix (woodfine-bim-library): COMPLETE**
- LICENSE replaced with Apache-2.0 full text (SPDX: Apache-2.0)
- NOTICE file created: "Woodfine BIM Object Library / Copyright 2026 Woodfine Capital Projects Inc."
- README.md: license section updated EUPL-1.2 → Apache-2.0; repo title updated to woodfine-bim-library; BIM token/tokens → BIM Object/Objects in user-facing headings
- README.es.md: same terminology and license updates
- Committed as mcorp-administrator (SSH signed)
- **Push: SUCCEEDED** — b5b69a2..7267e15 pushed to origin (woodfine/woodfine-bim-library). Includes pending commit 443a231 (building-width-calculator migration) plus 954a2f4 (5 new DTCG files) plus Task 1 commit. All 3 previously-unpushed commits now on origin.

**Task 2 — Terminology sweep (GUIDE drafts): COMPLETE**
- guide-bim-token-authoring.draft.md: draft_id updated to guide-bim-object-authoring; BIM token → BIM Object throughout
- guide-deploy-bim-substrate.draft.md: repo references woodfine-design-bim → woodfine-bim-library; BIM token → BIM Object; license EUPL-1.2 → Apache-2.0 in setup step
- guide-climate-zone-tokens.draft.md: BIM token → BIM Object throughout
- guide-regulation-overlay-publishing.draft.md: BIM Token types → BIM Object types; vault reference updated
- guide-bim-archive-operations.draft.md: no "BIM token" mentions found; no changes needed
- Committed: 3b358da (pwoodfine, cluster/project-bim)

**Task 3 — 3 TOPIC drafts retitled + copied to project-editorial: COMPLETE**
- topic-bim-token-what-it-is.draft.md: title "BIM Objects — What They Are"; full body sweep
- topic-bim-token-three-layers.draft.md: title "BIM Objects — Three Composition Layers"; full body sweep
- topic-bim-tokens-substrate.draft.md: title "BIM Objects — Substrate"; full body sweep
- All 3 copied to /srv/foundry/clones/project-editorial/.agent/drafts-outbound/from-project-bim/
- Outbox message prepended to project-editorial/.agent/outbox.md
- Committed: f00fa72 (pwoodfine, cluster/project-bim)

**Task 4 — Phase 1 architecture spec: COMPLETE**
- Written to .agent/plans/app-orchestration-bim-phase1.md
- Two-tier access model locked; BIM Objects terminology; B5 website text changes documented as deferred
- Committed: aa98a84 (jwoodfine, cluster/project-bim)

**B5 — Website Rust source (binary text changes): DEFERRED**
- Live binary at v0.0.2 (built 2026-05-07) has hardcoded "BIM Token" strings
- Source crate Reserved-folder → Active is next-session scope
- Deferred items recorded in Phase 1 spec at .agent/plans/app-orchestration-bim-phase1.md

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: woodfine-palette-additions — admin-tier action needed (mcorp-administrator)
created: 2026-05-17T21:00:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260517-palette-admin-action
relayed-on: 2026-06-21
---

`woodfine-palette-additions.md` in `clones/project-bim/.agent/drafts-outbound/`
is a DESIGN-TOKEN-CHANGE draft for `customer/woodfine-media-assets` (master-cosigned
2026-05-06T19:10Z). It requires an admin-tier commit by mcorp-administrator.

**Action:** add 7 new tokens to `token-global-color.yaml` and promote
`accent-secure` to `woodfine-green` per the draft. Commit under mcorp-administrator
identity per CLAUDE.md §8.

This draft does NOT route to project-design — it is a woodfine-media-assets
admin-tier edit. Routing the draft to Command for coordination.

— totebox@project-bim

---
from: totebox@project-bim
to: totebox@project-design
re: DESIGN sweep supplement — 3 additional drafts + 1 new (print/PDF pipeline)
created: 2026-05-17T21:00:00Z
priority: high
priority-boosted: 2026-06-21
status: dispatched
attempts: 25
msg-id: project-bim-20260517-design-sweep-supplement
relayed-on: 2026-07-07
---

Supplemental dispatch to the earlier design sweep message
(msg-id: project-bim-20260517-design-sweep). Four additional drafts
are staged in `clones/project-bim/.agent/drafts-outbound/`:

**DESIGN-RESEARCH (1 new — created this session):**

  design-research-html-print-pdf-pipeline.draft.md
  → target: vendor/pointsav-design-system/research/html-print-pdf-pipeline.md
  The canonical @page + Playwright PDF architecture for any HTML artifact
  that must print or export to PDF. Covers: full-bleed @page declaration,
  screen-scoped responsive rule pattern, build-pdf.mjs generator, checklist
  for new slide decks. Reusable across all clusters.

**DESIGN-RESEARCH (1 existing — not included in prior dispatch):**

  design-index.md
  → target: vendor/pointsav-design-system (review index)
  The BIM extension review index: lists all tokens, components, and research
  files landed on cluster/project-bim branch of pointsav-design-system.
  Asks project-design Task to accept as-is or propose refinements before
  Stage-6 promotion. Includes architecture decisions (xeokit over @thatopen,
  Tauri IPC pattern, IfcOpenShell sidecar LGPL compliance).

**DESIGN-COMPONENT (1 existing — not included in prior dispatch):**

  design-generic-components-index.md
  → target: vendor/pointsav-design-system/components/<various>
  Nine domain-agnostic component patterns from the BIM showcase v0.0.2
  that are candidates for META-substrate generalisation:
  SidebarAccordion, CodeBlockWithCopy, ChipRow, TabBarDisclosure,
  PreviewFrame (light/dark toggle), BreadcrumbNav, EmptyStateCard,
  MachineSurfaceFooter, EditOnGitHubLink (planned, not yet authored).
  Reference implementations in cluster CSS + JS. Per operator policy
  2026-04-28: flow back domain-agnostic patterns to project-design.

All three carry `foundry-draft-v1` frontmatter and `state: ready-for-sweep`.
Note: `woodfine-palette-additions.md` is also in drafts-outbound but routes
to Command (admin-tier / mcorp-administrator), NOT to project-design.

— totebox@project-bim

---
from: totebox@project-bim
to: totebox@project-editorial
re: PROSE sweep — 10 TOPIC drafts + 5 GUIDE drafts ready for editorial pass
created: 2026-05-17T00:00:00Z
priority: high
priority-boosted: 2026-06-21
status: dispatched
attempts: 25
msg-id: project-bim-20260517-prose-sweep-editorial
relayed-on: 2026-07-07
---

15 PROSE drafts are staged in `clones/project-bim/.agent/drafts-outbound/` awaiting
editorial sweep. Please run `bin/draft-sweep.sh --gateway language` on this archive.

**TOPIC drafts (10) — destination: vendor/content-wiki-documentation**

Previously staged:
  topic-city-code-as-composable-geometry.draft.md
  topic-flat-file-bim-leapfrog.draft.md
  topic-building-design-system-bim.draft.md
  topic-open-bim-regulatory-acceptance.draft.md
  topic-bim-token-what-it-is.draft.md
  topic-bim-token-three-layers.draft.md

New this session:
  topic-bim-tokens-substrate.draft.md
  topic-asset-anchored-bim-vault.draft.md
  topic-aec-interface-conventions.draft.md
  topic-property-manager-bim-gap.draft.md

**GUIDE drafts (5) — destination: woodfine-fleet-deployment/cluster-totebox-property/ and gateway-orchestration-bim/**

  guide-deploy-bim-substrate.draft.md
  guide-bim-archive-operations.draft.md
  guide-bim-token-authoring.draft.md
  guide-climate-zone-tokens.draft.md
  guide-regulation-overlay-publishing.draft.md

All carry `foundry-draft-v1` frontmatter. TOPIC pairs require bilingual ES generation.
All 10 TOPIC articles now drafted — no remaining gaps from manifest.md §planned_topics.

— totebox@project-bim

---
from: totebox@project-bim
to: totebox@project-design
re: DESIGN sweep — 12 design drafts ready; render.rs-only decision for regulation-rs1
created: 2026-05-17T00:00:00Z
priority: high
priority-boosted: 2026-06-21
status: dispatched
attempts: 25
msg-id: project-bim-20260517-design-sweep
relayed-on: 2026-07-07
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
  Recorded in .agent/rules/cleanup-log.md.

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: Please relay to project-design — P8c render.rs-only decision
created: 2026-05-16T18:45:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260516-p8c-relay-to-design
relayed-on: 2026-06-21
---

Operator decision confirmed (inbox msg project-bim-20260516-p8c-regulation-component):

`design-component-bim-regulation-rs1.md` — implement using `render.rs`-only.
Do not implement `recipe.html` at this stage. Defer until user-facing surface
ships and rendering approach can be validated against real data.

Please forward this decision to project-design inbox. They are holding the
`design-component-bim-regulation-rs1.draft.md` pending this answer.

Decision recorded in `.agent/rules/cleanup-log.md`.

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: building-width-calculator.dtcg.json migrated — Stage 6 + admin cleanup needed
created: 2026-05-16T18:45:00Z
priority: normal
status: dispatched
msg-id: project-bim-20260516-bwc-migration-complete
relayed-on: 2026-06-21
---

`building-width-calculator.dtcg.json` is now committed in `woodfine-design-bim`
at commit 443a231 (pwoodfine, v0.0.2). All 10 BIM DTCG files are now in the
correct repo.

Two actions needed from Command:

1. **Stage 6 push** — push `woodfine-design-bim` main branch to origin
   (`git@github.com-woodfine-administrator:woodfine/woodfine-design-bim.git`).
   The prior commit (b5b69a2) was mcorp-administrator; the new commit (443a231)
   is pwoodfine staging-tier. Normal promotion path.

2. **Admin-tier cleanup** — remove all 10 files from
   `pointsav-design-system/tokens/bim/` (ps-administrator identity).
   Files confirmed identical to woodfine-design-bim copies; safe to delete.
   Files: assemblies, building-width-calculator, climate-zones, elements,
   identity-codes, materials, performance, relationships, spatial, systems
   (.dtcg.json each).

— totebox@project-bim


