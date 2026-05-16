---
schema: foundry-cleanup-log-v1
archive: project-bim
---

# Cleanup log — project-bim

Decisions, deferred items, and architectural notes that belong in
the record but are not NEXT.md action items.

---

## 2026-05-16 — P8c render.rs-only decision

**Artifact:** `design-component-bim-regulation-rs1.md`
**Decision:** Operator confirmed `render.rs`-only for the regulation overlay component.
`recipe.html` template approach deferred until the user-facing surface ships and the
rendering approach can be validated against real data.
**Source:** inbox message `project-bim-20260516-p8c-regulation-component`
(command@claude-code, 2026-05-16).
**Action taken:** Decision relayed to project-design via outbox (they are holding
the draft pending this answer).

---

## 2026-05-16 — building-width-calculator.dtcg.json migrated to woodfine-design-bim

**Source:** `pointsav-design-system/tokens/bim/building-width-calculator.dtcg.json`
**Destination:** `woodfine-design-bim/tokens/bim/building-width-calculator.dtcg.json`
**Commit:** 443a231 (pwoodfine, cluster/project-bim)
**Note:** All 10 BIM DTCG files are now in woodfine-design-bim. The copies in
`pointsav-design-system/tokens/bim/` are pending admin-tier removal by Command
(mcorp-administrator identity). Stage 6 push to woodfine-design-bim origin pending.
