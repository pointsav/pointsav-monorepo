---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-marketing
target_repo: pointsav-design-system
target_path: components/icon-tab/
target_filename: recipe.html + recipe.css + aria.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-05-22T00:00:00Z
authored_by: totebox@project-marketing (claude-code)
authored_with: claude-sonnet-4-6
component_metadata:
  component_name: icon-tab
  component_kind: navigation
  carbon_baseline: button (icon variant) / tab
  accessibility_targets: [wcag-2-2-aa, focus-visible, aria-label-external]
  brand_voice_alignment: [confident, direct, institutional]
  preview_html: |
    <a class="wf-icon-tab" href="https://github.com/woodfine/woodfine-fleet-deployment"
       target="_blank" rel="noopener"
       aria-label="Fleet Manifest (opens in new tab)">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor">
        <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"/>
      </svg>
      <span>MANIFEST</span>
    </a>
references:
  - tokens/ASSET-ICON-GITHUB.svg
  - components/tab/recipe.json
  - templates/html/woodfine-index-scaffold.html
  - template-agnostic-ui.html
notes_for_designer: |
  Originated from home.woodfinegroup.com subnav: user requested the
  design-system GitHub-logo button (from template-agnostic-ui.html
  `.btn` pattern) rendered in the same filled, uppercase, tab-sized
  style as the existing `.subnav .tab` buttons (BIM Library,
  Location Intelligence).

  The implementation uses the Woodfine-brand accent fill (#164679 via
  var(--accent)), white text, Oswald/display font at 11px/0.16em
  letter-spacing, 130×40px minimum size — exactly matching the
  sibling `.tab` buttons. GitHub icon is 14×14px, white via
  currentColor.

  Key design decisions to preserve or refine:
  1. Icon inherits text colour via fill="currentColor" — no separate
     icon colour token needed.
  2. The component is an <a> not <button> — it always navigates
     externally (target="_blank"). ARIA label must name the
     destination.
  3. This pattern is generalizable: any platform icon (GitHub,
     Figma, Notion, etc.) + uppercase label + filled tab style.
     Consider a generic `icon-tab` with an `--icon` slot and a
     `wf-icon-tab--github` modifier, so the icon can vary.
  4. Carbon baseline: this is closer to Button (icon-leading variant)
     than to Tab (which is link-underline). The filled style is
     a brand extension on top of Carbon's primary button.
  5. Hover: current implementation uses opacity: 0.85. Designer may
     prefer a tone-step on --accent (darken 10%) to match how
     .btn-primary hover works in template-agnostic-ui.html.

research_done_count: 3
research_suggested_count: 2
open_questions_count: 2
research_provenance: direct-consultation
research_inline: true
---

# DESIGN-COMPONENT — icon-tab

A tab-style egress button that combines a platform icon with an
uppercase label. Used in subnav bars to surface external links
(GitHub, manifests, registries) at the same visual weight as
first-class navigation tabs.

---

## Recipe — HTML

```html
<a class="wf-icon-tab wf-icon-tab--github"
   href="{{HREF}}"
   target="_blank"
   rel="noopener"
   aria-label="{{ARIA_LABEL}} (opens in new tab)">
  <!-- slot: icon — replace SVG path for other platforms -->
  <svg class="wf-icon-tab__icon"
       xmlns="http://www.w3.org/2000/svg"
       viewBox="0 0 16 16"
       fill="currentColor"
       aria-hidden="true">
    <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"/>
  </svg>
  <span class="wf-icon-tab__label">{{LABEL}}</span>
</a>
```

---

## Recipe — CSS

```css
.wf-icon-tab {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  gap: 6px;
  background: var(--ps-interactive-primary, #164679);
  color: var(--ps-ink-on-interactive, #fff);
  font-family: var(--ps-font-display, "Oswald", "Barlow Condensed", Arial, sans-serif);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.16em;
  line-height: 1.25;
  padding: 8px 14px;
  min-width: 130px;
  min-height: 40px;
  text-transform: uppercase;
  text-decoration: none;
  cursor: pointer;
  box-sizing: border-box;
  white-space: nowrap;
}

.wf-icon-tab:hover {
  opacity: 0.85;
}

.wf-icon-tab:focus-visible {
  outline: 2px solid var(--ps-focus-ring, #234ed8);
  outline-offset: 2px;
}

.wf-icon-tab__icon {
  display: block;
  flex-shrink: 0;
  width: 14px;
  height: 14px;
}

.wf-icon-tab__label {
  display: block;
  line-height: 1;
}
```

---

## Recipe — ARIA

- Component is a native `<a>` element linking to an external URL.
  No ARIA role needed — `<a href>` carries the implicit link role.
- `aria-label` MUST name the destination and include "(opens in
  new tab)" suffix (WCAG 2.4.4 Link Purpose, Level AA).
- `aria-hidden="true"` on the inline SVG prevents screen readers
  from announcing the icon path — the aria-label on the `<a>`
  carries the full label.
- Do not use `role="button"` — this is navigation, not an action.
- `rel="noopener"` is required alongside `target="_blank"` to
  prevent the opened tab from accessing the opener's window object.

---

## Research trail

### Done
1. Reviewed `tokens/ASSET-ICON-GITHUB.svg` — confirmed 16×16
   viewBox, single path, fill="currentColor". Icon is already in
   the design-system asset registry.
2. Reviewed `components/tab/recipe.json` and `components/button/
   recipe.json` — confirmed no existing icon-tab or egress-tab
   component. This is a gap.
3. Reviewed `template-agnostic-ui.html` `.btn` pattern — the
   design-system scaffold already used the GitHub icon + MANIFEST
   label pattern with a bordered light-background button. This draft
   proposes the filled Woodfine-brand variant as the canonical form,
   with the scaffold's `.btn` as a secondary ghost-style variant for
   project-design to consider.

### Suggested
1. Define a DTCG token for `--ps-font-display` (display/condensed
   typeface alias). Currently hardcoded in the Woodfine theme as
   `var(--display)`. The icon-tab component should resolve this from
   a token so non-Woodfine tenants can substitute.
2. Audit whether hover opacity (0.85) is sufficient contrast for
   WCAG 1.4.3 against the canvas colour. A tone-step (`#0f3560`
   derived from #164679) may be more robust.

### Open questions
1. Should the component expose an icon slot as a CSS custom property
   (background-image on `::before`) or keep the SVG inline? Inline
   SVG allows `currentColor` inheritance and is the current
   implementation; background-image approach would remove the HTML
   dependency on icon content but loses colour inheritance.
2. Should `template-agnostic-ui.html` `.btn` (bordered, light
   background) be registered as `wf-icon-tab--ghost` variant, or
   treated as a separate component (`icon-btn`)?
