---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-orgcharts
target_repo: pointsav-design-system
target_path: research/
target_filename: research-orgchart-green-value-decision.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-06-01T00:00:00Z
authored_by: totebox@project-orgcharts session-2026-06-01
authored_with: sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: sub-agent
research_inline: true
references:
  - archive-2026-06-01/pointsav-design-system/tokens/theme-woodfine.css
  - archive-2026-06-01/pointsav-design-system/themes/MEMO-Woodfine-Color-Matrix.md
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_WCP_JW3.html
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_SPV2_Detailed_JW2.html
notes_for_designer: |
  Two of the four Bencal charts (WCP_JW3 and SPV2_Detailed_JW2) use
  #198038 / #DEFBE6 for the green token role. The design system
  has --wf-green: #54924E / --wf-green-tint: #EEF6EC.

  These are materially different greens — #54924E is a muted, warm
  institutional green; #198038 is IBM Carbon Green 70, a saturated
  commercial green. They cannot both be correct for the same role.

  This needs a decision, not a patch. Document the decision in the
  research file so future chart authoring uses one consistent value.
---

# DESIGN-RESEARCH — Bencal chart green value drift vs design-system `--wf-green`

## The conflict

| Source | Value | Description |
|---|---|---|
| `theme-woodfine.css` `--wf-green` | `#54924E` | Woodfine institutional green — muted, warm |
| `theme-woodfine.css` `--wf-green-tint` | `#EEF6EC` | Corresponding tint |
| Bencal WCP_JW3, SPV2_Detailed_JW2 (hardcoded) | `#198038` | IBM Carbon Green 70 — saturated |
| Bencal charts (tint, hardcoded) | `#DEFBE6` | IBM Carbon Green 10 |

The two Bencal charts that use green (WCP_JW3 for WCP / operational entity
role; SPV2_Detailed_JW2 for a similar flow-through indicator) were authored
against the Carbon value, not the Woodfine CSS custom property. The other
two Bencal charts (Organization_JW2, SPV2_JW2) do not use the green role,
so this drift doesn't affect them.

## Why this matters

`components/nodes.css` `.org-token--green` already references
`var(--wf-green)`, which resolves to `#54924E`. If a chart uses the
`.org-token--green` class, it renders with `#54924E`. If it uses the
hardcoded `#198038` (as the Bencal charts do via inline `.token-green`
classes), it renders differently. Charts that mix both approaches will
show inconsistent greens on the same canvas.

The `MEMO-Woodfine-Color-Matrix.md` describes `--wf-green` (`#54924E`) as
"Accounting / finance / operations domain accent." The Bencal WCP chart uses
green for WCP operational structure — consistent with that domain assignment.
The right color for that role is `--wf-green: #54924E`, not Carbon `#198038`.

## Recommendation

**Adopt `--wf-green: #54924E` as the canonical green.** The Bencal charts
should be updated to reference `var(--wf-green)` / `var(--wf-green-tint)`
via the design-system class `.org-token--green` rather than inline hardcoded
values. No token change needed — the token is correct; the charts are drifted.

Chart remediation (two files):
- `inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_WCP_JW3.html`
- `inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_SPV2_Detailed_JW2.html`

Both should replace `.token-green { background: #DEFBE6; border-color: #198038; }`
with a reference to `--wf-green` / `--wf-green-tint`, and the SVG
`arrow-green` marker (`#198038`) with `var(--wf-green)` or the hex `#54924E`.

## If Carbon green is preferred instead

If operator or Creative Designer prefers `#198038` (Carbon Green 70) as more
visually distinct or better suited for the WCP operational role, then the
correct path is a DESIGN-TOKEN-CHANGE to update `--wf-green` in
`theme-woodfine.css` — NOT silently leaving the charts with hardcoded values
that diverge from the token. That token-change would require Master co-sign.

---

## Research trail

### Done

1. Confirmed `--wf-green: #54924E` in `theme-woodfine.css` — this is the
   current canonical value.
2. Confirmed `MEMO-Woodfine-Color-Matrix.md` assigns green to
   "Accounting / finance / operations" — consistent with WCP usage.
3. Confirmed `nodes.css` `.org-token--green` references `var(--wf-green)`,
   not a hardcoded value — the component is correct; the charts drifted.

### Suggested

1. project-design (or operator) to confirm whether `#54924E` or `#198038`
   is the preferred Woodfine green for operational/fund-flow roles, and
   record the decision in `MEMO-Woodfine-Color-Matrix.md`.

### Open questions

1. **Operator preference**: Jennifer — does the WCP chart green look right
   to you at `#54924E` (warmer, institutional), or do you prefer
   `#198038` (brighter, Carbon-native)? The answer determines whether
   we patch the charts to match the token, or update the token to match
   the charts.
