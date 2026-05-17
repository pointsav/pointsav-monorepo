# Agent 1 — Building Width Calculator deck — report

**Date:** 2026-05-17
**Agent:** totebox@project-bim (claude-opus-4-7)
**Target:** `preview/building-width-calculator.html`

## What changed in the HTML

### Structural correction — the formula was wrong

Prior HTML formula: `Building Width = 2 × (Habitat + Magazine + Corridor)`
(doubled the corridor).

Corrected formula: `Building Width = 2 × (Habitat + Magazine) + Corridor`
(corridor is a single centreline; Habitat and Magazine are mirrored).

Verified against two independent V3 sources:

- V12 (Jan 2025) Professional Office: 6 + 3 + 3 + 3 + 6 = 21 m total
  (Z3 corridor appears once, not twice)
- Business Notes V3 Option A/A: 2 × 5.51 + 2 × 9.26 + 2.75 = 32.29 m
  (the document tabulates this total explicitly)

Fixes applied:
- `totalWidth()` JS helper rewritten
- Slide 3 formula text updated
- Slide 3 table "Half" column relabelled to "Half (Z1+Z2)"
- Slide 2 bar chart rewritten to render the full mirror `[Z1|Z2|Z3|Z2|Z1]`
  instead of one half scaled

### BIM_TOKENS data values updated

| Use type | Before | After | Source |
|---|---|---|---|
| professional-office | 6.0 / 3.8 / 2.0 | **6.0 / 3.0 / 3.0** | V12 baseline (TBD per V12; prior values had no source) |
| business | 6.0 / 7.3 / 2.7 | **5.51 / 9.26 / 2.75** | Notes V3 Option A/A (widest enumerated symmetric option) |
| medical | 7.2 / 4.87 / 2.89 | **7.2819 / 4.877 / 2.892** | M3/M1/M2 sketches (286 5/8" + 192" + 113 7/8") |
| private-office | 5.9944 / 1.3716 / 0 | unchanged | confirmed against PO sketches (19'8" + 4'6") |
| laboratory | 6.7818 / 4.8006 / 3.048 | unchanged | confirmed against Summary V2 |
| academic | 4.7 / 3.0 / 0 | unchanged | confirmed against Summary V2 |
| civic | 6.0 / 7.23 / 3.6 | unchanged | no DISCOVERY sketch — synthesised |

New computed building widths (slide 2 + slide 3 will display these):

- Private Office: **14.73 m**
- Academic: **15.40 m**
- Professional Office: **21.00 m**
- Laboratory: **26.21 m**
- Medical: **27.20 m**
- Business: **29.30 m → 32.29 m** (was 32.0 m; new value uses Option A/A)
- Civic: **30.06 m**

Widest now = Business (32.29 m). Was Civic (33.66 m) under prior
double-corridor formula.

### Prose updates

- Slide 1 lede: anchored language to V12 — "Façade Frontage",
  "Desks/Workstations", "single centreline Zone 3" — and clarified
  that Habitat default is 6 m but use types diverge
- Slide 1 zone definitions: rewrote Zone 1, 2, 3 descriptions to
  reflect V12 wording ("balance out the dimensions"; "overdesign
  the corridors by 20%"; "in and out" traffic) and noted that two
  use types have no Zone 3
- Slide 1 callout: unchanged (Summary V2 confirms 0.7 m verbatim)
- Slide 2 lede: corrected "1.4 m wider" (was wrong arithmetic; the
  actual spread was 18.93 m under old formula) to "roughly 17.6 m
  wider" under corrected formula
- Slide 3 formula: rewritten as `2 × (Habitat + Magazine) + Corridor`
  with strip layout `Z1 + Z2 | Z3 | Z2 + Z1`
- Slide 3 eyebrow: "Mirror equation · both halves equal" → "Habitat
  + Magazine mirrored · corridor single"

## Source documents and how they drove changes

| Document | What it provided | What it changed |
|---|---|---|
| V12 PDF (Jan 7 2025) | Canonical mirror cross-section; 21 m baseline; "TBD" markers | Triggered restoration of Professional Office to 6/3/3; confirmed Z3 single |
| Medical Notes (Summary V2, May 2025) | Z-value table per use type; Medical/Laboratory/Academic anchored | Confirmed Medical Z1 = 23'10" (sketch precision 7.2819 m); confirmed Lab/Academic values; confirmed 0.7 m supplement wording |
| Medical sketches M1/M2/M3 | Dimensioned depths (286 5/8" / 192" / 113 7/8"); SF labels | Refined Medical Z1 from 7.2 to 7.2819 |
| Business Notes V3 | 21 enumerated width options; Option A/A as widest symmetric | Replaced un-sourced 6/7.3/2.7 with sourced 5.51/9.26/2.75 |
| Private Office Notes | PO-1/PO-2/PO-3 dimensioned plans | Confirmed Z1 = 19'8" and Z2 = 4'6" already in token |
| building-width-calculator.pdf (master file) | Identical to current HTML rendered as PDF | Confirmed — used as reference for "before state" |

## TOPIC drafts created

- `/srv/foundry/clones/project-bim/.agent/drafts-outbound/topic-bim-building-width-method.draft.md`
  — the inversion (Furniture → Zone → Width), the mirror cross-section,
  the formula, and the rationale per zone
- `/srv/foundry/clones/project-bim/.agent/drafts-outbound/topic-bim-zone-depths-per-use-type.draft.md`
  — seven-row canonical table; per-use-type sections with source
  evidence and rationale; sketch-evidence appendix; living-document
  scaffolding ("Future research", "Open inconsistencies")

Both carry `target_repo: vendor/content-wiki-projects` per operator
direction (project.woodfinegroup.com surface).

## Open questions / things I couldn't resolve

1. **Academic Small key plan area conflict** — building-width-calculator.dtcg.json
   records 105 m² / 1,131 SF; professional-office-subtypes.dtcg.json
   records 87.7 m² / 944 SF. Architecture plan inconsistency #1. Pending
   operator decision on canonical value.
2. **Civic zone depths unverified** — no completed DISCOVERY sketch.
   Token values 6.0 / 7.23 / 3.6 are synthesised; left unchanged.
3. **Professional Office Z2/Z3** — V12 marks both as "TBD" at 3 m;
   final values await the Professional Office Key Plan completion.
   The HTML preview now reflects the V12 placeholder; if the operator
   prefers to keep the prior 3.8/2.0 interpretation, revert
   BIM_TOKENS.zones["professional-office"].
4. **Business option selection** — Notes V3 enumerates 21 options.
   I chose Option A/A (widest symmetric, 32.29 m) because it matches
   the previous HTML's positioning of Business as "widest Magazine"
   and because it's the upper envelope. The operator may prefer
   a balanced option like C/C (27.27 m).
5. **PDF could not be regenerated** — `build-pdf.mjs` requires
   Playwright from `/home/jennifer/sandbox/working/ps-talking-points/node_modules`,
   which is outside the Totebox archive and is sandbox-blocked from
   my Bash tool. The operator can run the regen command directly:
   `cd /srv/foundry/clones/project-bim/preview && NODE_PATH=/home/jennifer/sandbox/working/ps-talking-points/node_modules node build-pdf.mjs building-width-calculator.html`

## Source inconsistencies (V12 vs later docs)

- V12 (Jan 2025) shows Professional Office Z2=3 m TBD, Z3=3 m TBD
- Summary V2 (May 2025) leaves Professional Office Z2/Z3 blank
- The prior HTML used 3.8 m / 2.0 m for Professional Office, untraced
  to any source — likely an intermediate working value never recorded
  in a published doc. Restored to V12 baseline pending operator input.
- Medical Z1 has three published values: 7.2 m (Summary V2 rounded),
  7.282 m (sketch 286 5/8"), and "23'10"" (Summary V2 ft display
  which corresponds to ~7.27 m, closer to sketch than to 7.2). The
  HTML now uses 7.2819 m to reconcile.
- Business 13.30 m Zone 02 annotation on Sketch 2-4 is the **total
  bilateral Z2** (6.63 m + 6.63 m), not per-side. The Building Width
  Options table normalises to per-side. The token value is per-side
  (Z2 = 9.26 m means 18.52 m of Magazine across the mirror).

## What was deliberately not changed

- Slide structure (six slides)
- Visual design (palette, typography, layout)
- Print CSS (`@media print` block — explicitly preserved per task)
- `build-pdf.mjs` (PDF generator unchanged)
- Floor-plate composition tile labels (Composition A/B/C) — task
  scope limited to zone depths and source-cited values; tile-color
  vs narrative mismatch on Composition C is a future cleanup
