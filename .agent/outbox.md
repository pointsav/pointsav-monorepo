---
mailbox: outbox
owner: task-project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-bim cluster

---
from: totebox@project-bim
to: task@project-editorial
re: PROSE sweep — 10 TOPIC drafts + 5 GUIDE drafts ready for editorial pass
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-bim-20260517-prose-sweep-editorial
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
to: task@project-design
re: DESIGN sweep — 12 design drafts ready; render.rs-only decision for regulation-rs1
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
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
  Recorded in .agent/rules/cleanup-log.md.

— totebox@project-bim

---
from: totebox@project-bim
to: command@claude-code
re: Please relay to project-design — P8c render.rs-only decision
created: 2026-05-16T18:45:00Z
priority: normal
status: pending
msg-id: project-bim-20260516-p8c-relay-to-design
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
status: pending
msg-id: project-bim-20260516-bwc-migration-complete
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


