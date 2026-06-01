---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-orgcharts
target_repo: pointsav-design-system
target_path: tokens/
target_filename: theme-woodfine.css
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-TOKEN-CHANGE
authored: 2026-06-01T00:00:00Z
authored_by: totebox@project-orgcharts session-2026-06-01
authored_with: sonnet-4-6
master_cosign: PENDING-COMMAND-SESSION
research_done_count: 4
research_suggested_count: 2
open_questions_count: 2
research_provenance: sub-agent
research_inline: true
references:
  - archive-2026-06-01/pointsav-design-system/tokens/theme-woodfine.css
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_WCP_JW3.html
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_SPV2_Detailed_JW2.html
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_Organization_JW2.html
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-25_Chart_Bencal_SPV2_JW2.html
notes_for_designer: |
  Two token families are missing from theme-woodfine.css: teal and red.
  Both are used consistently across the four Bencal charts (May 2026) as hardcoded
  hex values. The charts cannot reference CSS custom properties for these colors
  because the properties don't exist yet.

  Teal (#005D5D / #9EF0F0) appears in all four charts for fund / flow-through
  entity roles and SVG connectors. Red (#A2191F / #FFB3B8) appears in
  Organization_JW2 for a specific entity class. Both are IBM Carbon values.

  Before adding: confirm whether to adopt the Carbon values directly or
  substitute Woodfine-institutional equivalents (see open questions below).
  The green drift issue is tracked separately in
  research-bencal-chart-green-value-drift.draft.md.

  MASTER CO-SIGN REQUIRED before project-design processes this draft.
---

# DESIGN-TOKEN-CHANGE — Add `--wf-teal` and `--wf-red` to theme-woodfine.css

## Proposed addition

Insert the following two token families into `theme-woodfine.css`
under the `/* WOODFINE CHART PALETTE */` block, after `--wf-grey-tint`:

```css
/* WOODFINE CHART PALETTE — additions (teal + red, 2026-06-01) */
--wf-teal:          #005D5D;   /* Fund / flow-through entity role */
--wf-teal-tint:     #9EF0F0;   /* Teal box background */
--wf-red:           #A2191F;   /* Alert / required-action entity role */
--wf-red-tint:      #FFB3B8;   /* Red box background */
```

The `--wf-magenta` / `--wf-magenta-tint` pair is NOT included here — magenta
appears in older charts but is not used in the four Bencal JW2/JW3 charts.
If magenta should be added, that is a separate token-change request.

## DTCG bundle update (project-design to complete)

project-design should also add DTCG primitive + semantic entries for these
two families in `tokens/dtcg-bundle.json`, parallel to the existing
`wf-blue`, `wf-green`, etc. entries. The naming pattern is:
`primitive.color.wf-teal`, `primitive.color.wf-teal-tint`,
`semantic.chart.wf-teal`, `semantic.chart.wf-teal-tint`.

## Downstream impact

- `components/nodes.css` — `.org-token--teal` and `.org-token--red` class
  additions needed (parallel to existing `.org-token--green`). project-design
  to assess whether nodes.css warrants a companion DESIGN-COMPONENT draft
  or can be bundled here.
- `components/connectors.css` — `.org-connector--teal` and `.org-connector--red`
  additions needed (parallel to existing `.org-connector--green`).
- `components/org-chart-venn.css`, `org-chart-matrix.css`,
  `org-chart-governance.css` — check for hardcoded teal/red values that should
  reference the new tokens.
- Four Bencal charts in `inputs/current-org-chart-html/` — once tokens are
  added, charts should be updated to reference `var(--wf-teal)` etc.
  instead of hardcoded hex values (remediation separate from this token draft).

## Notes on value selection

The values `#005D5D` (teal) and `#A2191F` (red) are IBM Carbon values.
The existing Woodfine palette uses institutional variants rather than
Carbon values directly (e.g., `--wf-blue: #164679` vs Carbon Blue 80 `#002D9C`).
See open questions below.

---

## Research trail

### Done

1. Audited `theme-woodfine.css` — confirmed `--wf-teal` and `--wf-red` are absent.
   `--wf-red` is referenced in `org-chart-printable.html` template and NEXT.md
   as planned, but never added to the theme.
2. Audited four Bencal charts — confirmed teal (`#005D5D`/`#9EF0F0`) appears in
   all four; red (`#A2191F`/`#FFB3B8`) appears in Organization_JW2 only.
3. Audited `components/nodes.css` — confirmed `.org-token--teal` and
   `.org-token--red` classes reference `var(--wf-teal)` / `var(--wf-red)` which
   don't exist yet. nodes.css was written expecting these tokens.
4. Confirmed DESIGN-TOKEN-CHANGE requires Master co-sign per
   `conventions/cluster-design-draft-pipeline.md` §3.3.

### Suggested

1. project-design to check WCAG 2.2 AA contrast for `#005D5D` on `#9EF0F0`
   background (likely passes for normal text; verify for small text at 9-12px
   used in node labels).
2. project-design to check whether Carbon-native teal (`#005D5D`) vs a
   Woodfine-institutional teal variant is the right call — consult Jennifer
   or compare against the Woodfine brand guide if one exists.

### Open questions

1. **Teal value**: `#005D5D` is IBM Carbon Teal 70. Should Woodfine adopt this
   directly, or use a custom institutional value as was done for blue
   (`#164679` vs Carbon Blue 80)? This is a brand decision that needs
   operator input before the token is ratified.
2. **Red value**: `#A2191F` is IBM Carbon Red 70. The printable template had
   `#ED1B2F` (a different red). Which is the correct Woodfine red? NEXT.md
   listed `--wf-red` as planned but gave no value. Needs operator decision.
