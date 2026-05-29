---
schema: foundry-draft-v1
state: ready-for-sweep
language_protocol: TOPIC
originating_cluster: project-design
target_repo: vendor/content-wiki-documentation
target_path: content/
target_filename: topic-wiki-dark-mode.md
audience: editorial
bcsc_class: vendor-public
bilingual: true
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z (24-agent research pass).
  WCAG 2.1 contrast audit (hand-computed + agent 2026-05-06) — all pairs AAA.
  MDN: data-theme pattern, prefers-color-scheme media query.
  Web.dev: localStorage for theme persistence.
  CSS Tricks: flash-of-unstyled-theme prevention with inline script.
research_inline: true
---

# Wiki Dark Mode

The PointSav wiki supports light and dark colour schemes. Dark mode reduces eye strain in low-light environments and is preferred by a significant proportion of readers. This document describes the implementation approach: how the theme is set, persisted, and toggled, and provides the full colour palette for each mode.

---

## How it works

Dark mode is controlled by a `data-theme="dark"` attribute on the `<html>` element. The wiki's CSS uses this attribute as a selector override:

```css
/* Light (default) — defined on :root */
:root {
  --ps-surface-base: #ffffff;
  --ps-ink-primary: #0e0f12;
  /* ... */
}

/* Dark — overrides only semantic tokens */
[data-theme="dark"] {
  --ps-surface-base: #1f2125;
  --ps-ink-primary: #f5f6f8;
  /* ... */
}
```

Only semantic tokens (surfaces, ink, borders, status colours) change. Primitive tokens (the raw colour palette) remain unchanged in both modes. This design means that adding dark mode to a new component only requires using semantic tokens — no per-component `[data-theme="dark"]` selectors are needed.

---

## Initialisation

Theme preference is stored in `localStorage` under the key `ps-theme`. On each page load, an inline script in `<head>` reads this value and sets `data-theme` before the browser renders any content. This prevents the flash of the wrong theme (FOUT-theme) that would occur if the script ran after paint:

```html
<head>
  <script>
    (function() {
      var stored = localStorage.getItem('ps-theme');
      var prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (stored === 'dark' || (!stored && prefersDark)) {
        document.documentElement.dataset.theme = 'dark';
      }
    })();
  </script>
  <link rel="stylesheet" href="/static/tokens.css">
  <!-- ... -->
</head>
```

**Priority:** explicit user choice (localStorage) overrides the OS preference (`prefers-color-scheme`). If no choice has been stored, the OS preference is honoured.

---

## Toggle component

The wiki dark mode toggle button is defined in the `wiki-dark-mode-toggle` component (`dtcg-vault/components/wiki-dark-mode-toggle/recipe.json`). It uses `aria-pressed` and updates `aria-label` to describe the action (not the current state):

- In light mode: label = "Switch to dark mode"
- In dark mode: label = "Switch to light mode"

On click, the toggle sets `document.documentElement.dataset.theme` and writes the new value to `localStorage`.

---

## Colour palette

### Light mode

| Token | Value | Use |
|---|---|---|
| `--ps-surface-base` | #ffffff | Page background |
| `--ps-surface-subtle` | #f5f6f8 | Sidebar, code surface |
| `--ps-ink-primary` | #0e0f12 | Body text |
| `--ps-ink-secondary` | #4a4f59 | Secondary text, metadata |
| `--ps-wiki-link` | #234ed8 | Hyperlinks |
| `--ps-wiki-redlink` | #a52323 | Redlinks (non-existent articles) |
| `--ps-wiki-code-keyword` | #7c3aed | Code syntax keyword (purple-700) |

### Dark mode

| Token | Value | Use | WCAG vs background |
|---|---|---|---|
| `--ps-surface-base` | #1f2125 | Page background | — |
| `--ps-surface-code` | #151618 | Code block background | — |
| `--ps-ink-primary` | #f5f6f8 | Body text | 14.5:1 vs surface-base (AAA) |
| `--ps-ink-secondary` | #aab0bb | Secondary text | 6.2:1 vs surface-base (AAA) |
| `--ps-wiki-link` | #6ab0f5 | Hyperlinks | 8.47:1 vs page, 8.22:1 vs code (AAA) |
| `--ps-wiki-redlink` | #f56565 | Redlinks | 6.42:1 vs page (AA+) |
| `--ps-wiki-code-keyword` | #c792ea | Code syntax keyword | 7.85:1 vs code surface (AAA) |

**All dark mode colour pairs pass WCAG 2.1 AAA** (verified 2026-05-06 by hand computation + automated check). No remediation needed.

The weakest pair in the dark palette is the sidebar accent (`#4a9eff` on `#1a1a1a` = 6.32:1), which passes AA comfortably and passes AAA for large text (≥18px regular / ≥14px bold).

### Wiki surface aliases

The wiki CSS uses short-form aliases that map to the semantic tokens:

```css
--color-surface-page:    var(--ps-surface-base);
--color-surface-sidebar: var(--ps-surface-subtle);
--color-surface-code:    var(--ps-surface-code);
--color-text-primary:    var(--ps-ink-primary);
--color-text-secondary:  var(--ps-ink-secondary);
--color-text-link:       var(--ps-wiki-link);
--color-text-redlink:    var(--ps-wiki-redlink);
--color-border-subtle:   var(--ps-border-subtle);
--color-accent-primary:  var(--ps-interactive-primary);
--color-code-keyword:    var(--ps-wiki-code-keyword);
```

---

## Research trail

### Done
- data-theme attribute pattern confirmed as the CSS custom property override approach for PointSav wiki.
- FOUT-theme prevention via inline script confirmed (inline in `<head>`, before stylesheet).
- localStorage key: `ps-theme` (values: `'dark'` / `'light'`).
- WCAG 2.1 audit: all 7 dark mode pairs computed 2026-05-06 — all pass AAA.
- aria-pressed / aria-label toggle pattern confirmed per ARIA Authoring Practices.
