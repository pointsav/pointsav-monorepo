---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-dark-mode-toggle/
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
  prefers-color-scheme media query + localStorage persistence pattern.
  ps-preview-frame [data-theme] attribute pattern (committed this session) adapted here.
research_inline: true
---

# DESIGN-COMPONENT — wiki-dark-mode-toggle

Icon button that toggles `data-theme="dark"` on `<html>`. Persists preference to `localStorage`. Respects `prefers-color-scheme` as the initial default when no stored preference exists.

## Structure

```html
<button type="button"
        class="ps-wiki-theme-toggle"
        id="ps-theme-toggle"
        aria-label="Switch to dark mode"
        aria-pressed="false">
  <svg class="ps-wiki-theme-toggle__icon" aria-hidden="true" focusable="false">
    <!-- sun icon (light mode active) / moon icon (dark mode active) — swap via JS -->
  </svg>
</button>
```

## JS pattern

```js
const toggle = document.getElementById('ps-theme-toggle');
const root   = document.documentElement;

function applyTheme(dark) {
  root.dataset.theme = dark ? 'dark' : 'light';
  toggle.setAttribute('aria-pressed', String(dark));
  toggle.setAttribute('aria-label', dark ? 'Switch to light mode' : 'Switch to dark mode');
  localStorage.setItem('ps-theme', dark ? 'dark' : 'light');
}

// Initial: stored preference → system preference → light default
const stored = localStorage.getItem('ps-theme');
const system = window.matchMedia('(prefers-color-scheme: dark)').matches;
applyTheme(stored ? stored === 'dark' : system);

toggle.addEventListener('click', () => {
  applyTheme(root.dataset.theme !== 'dark');
});

// Follow system changes when no stored preference
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
  if (!localStorage.getItem('ps-theme')) applyTheme(e.matches);
});
```

## Tokens needed

- `--ps-interactive-ghost-hover` — toggle hover background
- `--ps-ink-secondary` — icon fill
- `--ps-ink-primary` — icon fill (active state)
- `--ps-focus-ring` — focus ring
- `--ps-speed-2` — icon swap transition

## ARIA

- `aria-pressed` reflects current state (true = dark is active).
- `aria-label` updated by JS to describe the RESULT of pressing ("Switch to dark mode" when light is current — follows the toggle button naming convention in ARIA Authoring Practices).
- Icon is `aria-hidden="true" focusable="false"` — label on the button is the accessible name.

## `prefers-reduced-motion`

Icon swap should use `transition: none` when `prefers-reduced-motion: reduce` is active.

## Research trail

### Done
- `localStorage` + `prefers-color-scheme` dual-priority pattern documented.
- `aria-pressed` + dynamic `aria-label` verified per ARIA 1.2 toggle button authoring.
- `data-theme` on `<html>` confirmed as the correct hook for CSS custom property overrides in `dist/tokens.css`.
- System preference change listener (when no stored preference) documented.
