---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-marketing
target_repo: pointsav-design-system
target_path: tokens/
target_filename: token-alias-ui.yaml (addition) + theme-pointsav.css (addition)
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-TOKEN-CHANGE
authored: 2026-05-22T00:00:00Z
authored_by: totebox@project-marketing (claude-code)
authored_with: claude-sonnet-4-6
master_cosign: null
component_metadata:
  component_name: icon-tab (PointSav steel variant)
  component_kind: navigation
  carbon_baseline: button (icon variant)
  accessibility_targets: [wcag-2-2-aa, focus-visible, contrast-4-5-1]
  brand_voice_alignment: [precise, institutional, open-source-signalling]
references:
  - tokens/ASSET-ICON-GITHUB.svg
  - drafts-outbound/DESIGN-COMPONENT-icon-tab.draft.md
  - tokens/theme-pointsav.css
  - tokens/token-alias-ui.yaml
notes_for_designer: |
  This is the PointSav-brand instance of the icon-tab component
  (see DESIGN-COMPONENT-icon-tab.draft.md from the same session).

  Woodfine variant (already shipped, home.woodfinegroup.com):
    background: #164679 (wf-blue / --accent)   colour: #ffffff

  PointSav variant (this draft, home.pointsav.com):
    background: #B4C5D5 (ps-steel / --accent)  colour: #164679 (wf-blue)

  The PointSav brand uses steel as the accent fill with WF-blue text —
  inverted contrast vs Woodfine. The GitHub icon inherits the text
  colour via currentColor so it is #164679 on both themes.

  Token additions needed:
  1. `--ps-icon-tab-bg`    → resolves to `var(--ps-steel)` in PointSav theme,
                              `var(--wf-blue)` in Woodfine theme
  2. `--ps-icon-tab-ink`   → resolves to `#164679` in PointSav theme,
                              `#ffffff` in Woodfine theme
  3. `--ps-icon-tab-focus` → resolves to `var(--ps-steel)` in PointSav theme,
                              `var(--wf-blue)` in Woodfine theme

  These three tokens let the icon-tab component be written once
  (`wf-icon-tab { background: var(--ps-icon-tab-bg); color: var(--ps-icon-tab-ink) }`)
  and resolve correctly under both brand themes. This is the only
  reason for a separate DESIGN-TOKEN-* draft rather than folding the
  variant into the DESIGN-COMPONENT draft.

  Contrast check (WCAG 1.4.3, Level AA — 4.5:1 minimum for text):
  - PointSav: #164679 on #B4C5D5 → ~4.8:1 (passes AA)
  - Woodfine: #ffffff on #164679 → ~9.4:1 (passes AAA)

  Both variants have sufficient contrast. Document in aria.md.

  No master_cosign required: these are semantic alias additions
  (no change to primitive colour values) and both colour primitives
  (`ps-steel`, `wf-blue`) are already ratified.

research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: direct-consultation
research_inline: true
---

# DESIGN-TOKEN-POINTSAV — icon-tab steel variant

Token additions to support the `wf-icon-tab` component (see
`DESIGN-COMPONENT-icon-tab.draft.md`) rendering correctly under
both the PointSav steel theme and the Woodfine blue theme.

---

## Proposed token additions — `token-alias-ui.yaml`

```yaml
# icon-tab semantic aliases
# Resolves to the brand-correct fill and ink for the icon-tab component.
# Override in per-brand theme files (theme-pointsav.css, theme-woodfine.css).
ps-icon-tab-bg:
  $type: color
  $value: "{ps.steel}"
  $description: "Background fill for the icon-tab component. Steel in PointSav theme; WF-blue in Woodfine theme."

ps-icon-tab-ink:
  $type: color
  $value: "{wf.blue}"
  $description: "Text and icon colour for the icon-tab component. WF-blue on steel (PointSav); white on WF-blue (Woodfine)."

ps-icon-tab-focus:
  $type: color
  $value: "{ps.steel}"
  $description: "Focus ring colour for the icon-tab component."
```

## Theme overrides — `theme-pointsav.css`

```css
/* icon-tab — PointSav steel theme */
:root {
  --ps-icon-tab-bg:    var(--ps-steel, #B4C5D5);
  --ps-icon-tab-ink:   #164679;
  --ps-icon-tab-focus: var(--ps-steel, #B4C5D5);
}
```

## Theme overrides — `theme-woodfine.css` (for completeness)

```css
/* icon-tab — Woodfine blue theme */
:root {
  --ps-icon-tab-bg:    var(--wf-blue, #164679);
  --ps-icon-tab-ink:   #ffffff;
  --ps-icon-tab-focus: var(--wf-blue, #164679);
}
```

## Updated component CSS (replaces hardcoded colours in icon-tab recipe)

```css
.wf-icon-tab {
  background: var(--ps-icon-tab-bg, #B4C5D5);
  color:      var(--ps-icon-tab-ink, #164679);
}
.wf-icon-tab:focus-visible {
  outline-color: var(--ps-icon-tab-focus, #B4C5D5);
}
```

---

## Research trail

### Done
1. Verified WCAG 1.4.3 contrast ratios:
   - PointSav: #164679 on #B4C5D5 ≈ 4.8:1 (AA pass)
   - Woodfine: #ffffff on #164679 ≈ 9.4:1 (AAA pass)
2. Confirmed `ps-steel` (#B4C5D5) and `wf-blue` (#164679) are both
   ratified primitive tokens in `token-global-color.yaml`. No new
   primitives required — these additions are alias-layer only.
3. Confirmed no master_cosign required: DOCTRINE §IV alias additions
   without primitive changes are standard design-session scope.

### Suggested
1. Once tokens land, update `a.monorepo-btn` in
   `media-marketing-landing-2/content/index.html` and
   `a.manifest-btn` in `media-marketing-landing-1/content/index.html`
   to reference the canonical token names (`var(--ps-icon-tab-bg)`,
   `var(--ps-icon-tab-ink)`) so live pages stay in sync with the
   substrate.

### Open questions
1. Should `--ps-icon-tab-bg` default to the PointSav steel value
   or use a neutral (`--paper-3`) so the component is usable on
   pages without either brand theme loaded? Neutral fallback is
   safer for third-party integrators; steel fallback better for
   internal use. project-design to decide.
