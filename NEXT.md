# NEXT.md — project-orgcharts

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-05

---

## Stage 6 pending

- [ ] Promote all project-orgcharts commits to canonical.
      Outbox message updated (2026-06-05, msg-id: project-orgcharts-20260605-stage6-v4-charts).
      Includes: all 5 commits — outbox/GUIDE/gitignore/V3-registry/V4-charts-and-registry.
      Command Session runs `bin/promote.sh`.
      [2026-06-05 totebox@claude-code]

---

## Customer leg — awaiting Command + project-editorial

- [ ] Command Session to commit `MANIFEST.md` to `woodfine-fleet-deployment/cluster-totebox-corporate/`.
      Outbox message sent (2026-06-05, msg-id: project-orgcharts-20260605-customer-leg-manifest).
      [2026-06-05 totebox@claude-code]
- [ ] project-editorial to deliver `GUIDE-orgchart-authoring.md` from drafts-outbound.
      Draft staged: `.agent/drafts-outbound/GUIDE-orgchart-authoring.draft.md` (2026-06-05).
      [2026-06-05 totebox@claude-code]

---

## Wiki leg — milestone-gated

- [ ] `topic-corporate-chart-design-system.md` + `topic-pre-canon-vs-post-canon-drift.md`
      substance pending JW7+JW9 REVIEW milestones. [2026-05-01]

---

## archive-2026-06-01/ — deletion review 2026-07-01

- [ ] Directory gitignored (2026-06-05, commit `fe99d71b`). Contains misplaced repo
      clones — no live references. Per README: safe to `rm -rf` after 2026-07-01.
      [2026-06-04 totebox@claude-code]

---

## Registry CSV — follow-up items

- [ ] Nodes 8, 10–14 (Ireland fund service providers: Issuer, AIFM, Depositary, Administrator,
      Auditor, Transfer Agent): TOKEN_SHAPE left empty — these entities do not appear as t-node
      elements in any current chart HTML. Add when a chart is created for the ETN/ICAV structure.
- [ ] Nodes 51 (Global Management), 52 (Realty Solutions Common Shares), 53 (Holdings 1 Inc.),
      54 (Holdings 1 LP): TOKEN_SHAPE left empty — not found in any current chart. Add when charts
      are created for these entities.
- [ ] `token-olive` class (management chart) — may not yet exist in `pointsav-design-system` token
      bundle. Flag for project-design backfill.
- [x] Node 28 (Woodfine Management Corp.) — updated to `token-base token-green` in JW10 (V4 change, 2026-06-05 commit `739e15e5`). TOKEN_SHAPE in V4 CSV also updated.

---

## Resolved this session (2026-06-05, continuation)

- [x] **V4 TOKEN_SHAPE changes applied to 6 charts** — commit `739e15e5` (jwoodfine, 2026-06-05).
      Nodes 28/36/40/50 updated. New JW versions: JW10 (Woodfine-Group), JW11 (Mexico),
      JW20 (Cross-Border-2), JW30 (Transaction-1), JW19 (Transaction-2), JW16 (Transaction-3).
      New CSS added to charts: `token-green` (JW10), `token-orange-ellipse-dashed` (JW20),
      `token-purple` (JW30/JW19/JW16).
- [x] **WCP-MASTER-ENTITY-REGISTRY_V4.csv committed** — same commit `739e15e5`.
      New nodes 36, 95–104 added. Working-tree V3 accidentally reverted to old labels during
      Jennifer's V4 edit — restored from git (`git checkout -- inputs/V3.csv`); V4 built correctly
      from committed V3 CSS classes.

---

## Resolved this session (2026-06-05)

- [x] **Bencal naming conflict** — Operator confirmed: canonical is **BPC / Bencal Private Capital Inc.**
      (2026-06-05). JW2 files already correct. No BCL files found in deployment instance
      (grep confirmed). Decision recorded in memory + session-context. [2026-06-05 totebox@claude-code]
- [x] **archive-2026-06-01/ gitignored** — commit `fe99d71b` (pwoodfine, 2026-06-05).
      Deletion review 2026-07-01.
- [x] **Stage 6 outbox sent** — commit `9c422878` (jwoodfine, 2026-06-05).
- [x] **GUIDE-orgchart-authoring staged** — commit `fc7c720d` (pwoodfine, 2026-06-05).
- [x] **Customer leg MANIFEST outbox sent** — commit `fc7c720d` (pwoodfine, 2026-06-05).
