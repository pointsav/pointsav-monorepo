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

## 2026-07-08 — v2 redesign confirmed live; shared push-to-prod.sh `--delete` gap fixed by Command

**Verification:** operator asked for a browser-in-the-loop check that Command pushed the correct v2
redesign build to `bim.woodfinegroup.com`. Ran headless Playwright/Chromium against the live URL,
screenshotted, and traced the rendered footer/trademark text back to
`pointsav-monorepo/app-privategit-bim/src/render/shell.rs` in the local clone — `git log` on that file
confirmed HEAD `3461856d` (the exact SHA Command cited as the cherry-pick tail). Sent a detailed
confirmation to Command (`command-20260708-verified-bim-woodfinegroup-com-is-the-co`); Command
independently re-verified the same result.
**Side finding (not project-bim's action item):** while auditing the shared `push-to-prod.sh` for
project-design, Command found `target_design`'s vault/templates/static rsync calls were missing
`--delete` — deleted source content never got removed from foundry-prod's disk. Fixed, and
preemptively added `--delete` to `target_bim`'s vault sync too (same root-cause gap, hadn't visibly
manifested for bim yet). Verified via dry-run; `bim.woodfinegroup.com` unaffected either way. No
action needed here — noting for the record in case a future session sees foundry-prod vault content
actually get pruned on next push where it previously wouldn't have.

---

## 2026-05-16 — building-width-calculator.dtcg.json migrated to woodfine-bim-library

**Source:** `pointsav-design-system/tokens/bim/building-width-calculator.dtcg.json`
**Destination:** `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json`
**Commit:** 443a231 (pwoodfine, cluster/project-bim)
**Note:** All 10 BIM DTCG files are now in woodfine-bim-library (repo renamed from
woodfine-design-bim per command-20260517-bim-rename-complete, 2026-05-17). The copies
in `pointsav-design-system/tokens/bim/` are pending admin-tier removal by Command
(mcorp-administrator identity). Stage 6 push to woodfine-bim-library origin pending.
