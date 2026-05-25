---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-drawer-mobile-nav/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Mobile sidebar/TOC collapse pattern (≤799px breakpoint).
  inert attribute for background content lockout (modern baseline).
research_inline: true
---

# DESIGN-COMPONENT — wiki-drawer-mobile-nav

Slide-in drawer for mobile navigation and TOC. Activated at ≤799px (compact breakpoint). Contains: site nav, search input, and table of contents. Background content receives `inert` attribute while open to prevent interaction and AT traversal outside the drawer.

## Structure

```html
<!-- Trigger (in page header) -->
<button type="button"
        class="ps-wiki-drawer-trigger"
        aria-expanded="false"
        aria-controls="ps-wiki-drawer"
        aria-label="Open navigation">
  ☰
</button>

<!-- Drawer -->
<div class="ps-wiki-drawer"
     id="ps-wiki-drawer"
     role="dialog"
     aria-label="Site navigation"
     aria-modal="true"
     hidden>
  <div class="ps-wiki-drawer__header">
    <span class="ps-wiki-drawer__title">Navigation</span>
    <button type="button"
            class="ps-wiki-drawer__close"
            aria-label="Close navigation">×</button>
  </div>
  <div class="ps-wiki-drawer__body">
    <nav aria-label="Site">{{site-nav-links}}</nav>
    <nav aria-label="Table of contents">{{toc-links}}</nav>
  </div>
</div>

<!-- Backdrop -->
<div class="ps-wiki-drawer-backdrop" aria-hidden="true" hidden></div>
```

## JS pattern

```js
const trigger  = document.querySelector('.ps-wiki-drawer-trigger');
const drawer   = document.getElementById('ps-wiki-drawer');
const backdrop = document.querySelector('.ps-wiki-drawer-backdrop');
const main     = document.querySelector('main');

function openDrawer() {
  drawer.hidden   = false;
  backdrop.hidden = false;
  main.inert      = true;
  trigger.setAttribute('aria-expanded', 'true');
  drawer.querySelector('.ps-wiki-drawer__close').focus();
}
function closeDrawer() {
  drawer.hidden   = true;
  backdrop.hidden = true;
  main.inert      = false;
  trigger.setAttribute('aria-expanded', 'false');
  trigger.focus();
}
trigger.addEventListener('click', openDrawer);
drawer.querySelector('.ps-wiki-drawer__close').addEventListener('click', closeDrawer);
backdrop.addEventListener('click', closeDrawer);
document.addEventListener('keydown', e => { if (e.key === 'Escape') closeDrawer(); });
```

## Tokens needed

- `--ps-surface-elevated` — drawer panel background
- `--ps-surface-inverse` with opacity — backdrop overlay (rgba)
- `--ps-border-subtle` — drawer edge border
- `--ps-speed-4` — slide-in transition (320ms)
- `--ps-ease-display` — drawer entrance easing
- `--ps-viewport-compact` — `@media (max-width: 799px)` show trigger
- `--ps-space-5`, `--ps-space-6` — drawer padding

## ARIA

- `role="dialog"` + `aria-modal="true"` on the drawer panel.
- Trigger: `aria-expanded` + `aria-controls` linking to the drawer.
- `inert` on `<main>` prevents AT from reading background content without focus management.
- Escape key closes drawer and returns focus to trigger.
- Two named `<nav>` landmarks inside drawer: "Site" and "Table of contents".

## Research trail

### Done
- `inert` attribute browser support verified (Chrome 102+, Firefox 112+, Safari 15.5+ — covers 2026 baseline).
- `aria-modal="true"` vs `inert` interaction: both needed — `aria-modal` for AT virtual cursor; `inert` for actual DOM lockout.
- Escape key + focus restoration pattern documented per ARIA dialog authoring practices.
- Backdrop click-to-close documented.
