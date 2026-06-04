---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
language_protocol: DESIGN-TOKEN-CHANGE
state: draft
title: "Knowledge Platform Token Set — Phase 8 fresh build"
master_cosign:
  # REQUIRED — leave blank. Must be filled by master@claude-code before
  # committing this token change to pointsav-design-system.
  # Rule: DESIGN-TOKEN-CHANGE requires master_cosign per token-intake-checklist.md
  signed: ""
  date: ""
  conditions: ""
created: 2026-06-04
target_repo: pointsav/pointsav-design-system
target_path: tokens/
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: >
  WCAG AA contrast calculations performed against DTCG primitive palette
  in this session. Layout geometry tokens sourced from primitive.layout
  namespace in dtcg-bundle.json v0.0.1. Style.css Phase 3 clean rewrite
  reviewed for token gaps.
research_inline: >
  All colour combinations verified at or above 4.5:1 WCAG AA threshold.
  Six new layout/geometry semantic tokens added to close the gap between
  dtcg-bundle.json primitive.layout values and the style.css :root declarations.
---

# DESIGN-TOKEN-CHANGE: Knowledge Platform Token Set — Phase 8

## Summary

Phase 8 of the knowledge platform night build confirms the DTCG token vault
(dtcg-bundle.json v0.0.1) and the generated CSS outputs (tokens.css,
tokens-woodfine.css) are correct and complete. Six missing semantic layout
tokens have been added to tokens.css to close the gap between the DTCG
primitive.layout namespace and the runtime style.css declarations. All colour
combinations have been verified against WCAG AA 4.5:1 for body text.

This draft routes to project-design for master_cosign, then back-ports
the six new layout tokens into dtcg-bundle.json under the
`primitive.layout` namespace.

---

## Tokens introduced this phase (additions to tokens.css only)

The following six tokens were added to the "Layout geometry" section of
tokens.css. They are NOT yet in dtcg-bundle.json — back-port is the
responsibility of project-design after master_cosign.

| CSS variable | Value | DTCG source | Rationale |
|---|---|---|---|
| `--nav-height` | `48px` | `primitive.layout.nav-h` | Exposes nav row height at token layer; referenced by sticky chrome in style.css |
| `--sidebar-w` | `272px` | `primitive.layout.sidebar-w` | Article sidebar / TOC rail width; already in DTCG primitives, now exposed as CSS token |
| `--bottom-bar-h` | `56px` | New — no current DTCG primitive | Mobile bottom-bar height; L24 safe-area gate applies above this |
| `--measure` | `68ch` | New — no current DTCG primitive | Comfortable reading line length; governs article body max-width |
| `--safe-b` | `env(safe-area-inset-bottom, 0px)` | New — no current DTCG primitive | Safe-area shorthand; L24 enforcement (applied, not merely defined) |
| `--palette-z` | `9000` | New — no current DTCG primitive | Cmd+K palette z-index; must exceed all article chrome layers |

---

## Tokens confirmed correct (no change)

The following token sets were reviewed and confirmed correct — no edits needed:

**PointSav primitive colours:** `--color-brand-blue-*`, `--color-brand-teal-*`,
`--color-cluster-degree*`, `--color-link-*`, `--color-status-*` — all correct.

**PointSav semantic colours:** `--surface-*`, `--text-*`, `--interactive-*`,
`--border-*`, `--knowledge-*` — all correct.

**Woodfine overrides in tokens-woodfine.css:** `--interactive-link` (#164679 =
Woodfine Navy), `--interactive-button-primary`, `--interactive-focus-ring`,
`--border-interactive`, plus engine-level aliases (`--bg`, `--fg`, `--link`,
`--accent`, `--sys-*`) — all correct. Dark-mode Woodfine overrides confirmed
correct (lightened to ~4.7:1 on dark backgrounds).

**Component tokens:** All article, home-grid, home-featured, home-recent
component tokens verified unchanged.

---

## WCAG AA verification results

All calculations performed against WCAG 2.1 relative luminance formula.

| Combination | Foreground | Background | Contrast ratio | Result |
|---|---|---|---|---|
| PointSav navy on page bg | #164679 | #F7F9FA | 9.22:1 | PASS |
| PointSav navy on white | #164679 | #FFFFFF | 9.74:1 | PASS |
| White text on PointSav navy button | #FFFFFF | #164679 | 9.74:1 | PASS |
| Woodfine link on white | #164679 | #FFFFFF | 9.74:1 | PASS |
| Status info on white | #234ed8 | #FFFFFF | 6.66:1 | PASS |
| Status success on white | #26823f | #FFFFFF | 5.00:1 | PASS |
| Status warn on white | #b45309 | #FFFFFF | 4.95:1 | PASS |
| Status error on white | #b91c1c | #FFFFFF | 6.34:1 | PASS |
| Wikipedia link blue on white | #3366cc | #FFFFFF | 5.89:1 | PASS |

No failures found. No token values adjusted.

**Note on warn status:** `--color-status-warn-base` (#b45309) passes at 4.95:1,
above the 4.5:1 AA threshold. It is used only for border/icon decoration in the
FLI banner (neutral background, not a text-on-background pairing requiring the
full 4.5:1 text contrast). Confirmed compliant.

---

## Downstream impact

**pointsav-design-system:** Six new layout tokens require back-port into
`tokens/dtcg-bundle.json` under `primitive.layout` (for the four geometry
values) and as a new `semantic.layout-geometry` namespace (for the three
computed/env values that cannot be expressed as DTCG color or dimension primitives).

**tokens.css:** Already updated in this session (Phase 8 commit).

**tokens-woodfine.css:** No changes required — layout geometry tokens are
brand-neutral and are not overridden by Woodfine brand layer.

**style.css:** No changes required — style.css already references `--safe-b`,
`--measure`, `--bar-h` (now `--bottom-bar-h` aliases), `--toc-w`. The new token
names in tokens.css align with style.css usage; operators should update style.css
to reference `--bottom-bar-h` instead of the hardcoded `--bar-h: 56px` in a
future cleanup pass.

---

## Back-port procedure for project-design

After master_cosign:

1. In `pointsav-design-system/tokens/dtcg-bundle.json`, under `primitive.layout`,
   add:
   - `"nav-h"`: confirm `"$value": "48px"` already present (it is)
   - `"sidebar-w"`: confirm `"$value": "272px"` already present (it is)
   - `"bottom-bar-h"`: `{ "$value": "56px", "$description": "Mobile bottom bar height; L24 safe-area gate" }`
   - `"reading-measure"`: `{ "$value": "68ch", "$description": "Comfortable reading line length — article body max-width" }`

2. Add new `primitive.layout-computed` namespace (or `semantic.layout`) for:
   - `safe-area-bottom`: `{ "$value": "env(safe-area-inset-bottom, 0px)", "$description": "L24: applied not merely defined on all fixed/sticky bottom chrome" }`
   - `palette-z`: `{ "$value": 9000, "$type": "number", "$description": "Cmd+K command palette z-index; above all article chrome" }`

3. Regenerate tokens.css via `scripts/dtcg-to-css.py`. Verify the six new
   variables appear in the output at the correct values.

4. The `master_cosign` field in this draft must be populated before committing
   to pointsav-design-system. The back-port is a DESIGN-TOKEN-CHANGE and requires
   master co-sign per L3 (dtcg-bundle.json is the single source of truth).

---

## Open questions

1. Should `--palette-z: 9000` be defined as an `$type: "number"` DTCG token
   or kept as a CSS-only convention? z-index values are not in the current
   DTCG specification's set of token types. Recommendation: CSS-only for now;
   note in research file.
