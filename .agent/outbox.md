---
mailbox: outbox
owner: task-project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-bim cluster

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


