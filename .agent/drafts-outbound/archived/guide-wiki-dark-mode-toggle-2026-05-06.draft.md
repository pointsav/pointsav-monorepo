---
schema: foundry-draft-v1
state: ready-for-sweep
language_protocol: GUIDE
originating_cluster: project-design
target_repo: customer/woodfine-fleet-deployment
target_path: pclp1-wiki/
target_filename: guide-wiki-dark-mode-toggle.md
audience: wiki-deployment-engineers
bcsc_class: customer-internal
bilingual: false
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  wiki-dark-mode-toggle component spec (component-wiki-dark-mode-toggle-2026-05-06.draft.md).
  MDN: data-* attributes, localStorage, prefers-color-scheme.
  WCAG audit 2026-05-06 — all pairs pass AAA.
research_inline: true
---

# Guide: Wiki Dark Mode Toggle

Operational runbook for wiring the PointSav dark mode toggle into a Woodfine wiki deployment.

---

## Prerequisites

- `tokens.css` linked in `base.html` (see guide-wiki-design-tokens.md)
- The dark-mode init script installed in `<head>` (see guide-wiki-design-tokens.md Step 2)

---

## Step 1 — Add the toggle button to the site header template

In `templates/partials/header.html` (or equivalent), add:

```html
<button class="ps-wiki-dark-toggle"
        type="button"
        aria-pressed="false"
        aria-label="Switch to dark mode">
  <span class="ps-wiki-dark-toggle__icon" aria-hidden="true">🌙</span>
  <span class="ps-wiki-dark-toggle__label">Dark</span>
</button>
```

The button state (aria-pressed, aria-label, icon) will be set correctly by the JavaScript in Step 3.

---

## Step 2 — Add toggle styles to wiki.css

```css
.ps-wiki-dark-toggle {
  display: inline-flex;
  align-items: center;
  gap: var(--ps-space-2);
  padding: var(--ps-space-2) var(--ps-space-3);
  background: var(--ps-interactive-ghost);
  color: var(--ps-ink-secondary);
  border: none;
  border-radius: var(--ps-corner-2);
  cursor: pointer;
  font-family: var(--font-sans);
  font-size: 0.875rem;
  transition: background var(--ps-speed-2) var(--ps-ease-utility);
}

.ps-wiki-dark-toggle:hover {
  background: var(--ps-interactive-ghost-hover);
}

.ps-wiki-dark-toggle:focus-visible {
  outline: var(--ps-focus-ring-width) solid var(--ps-focus-ring);
  outline-offset: var(--ps-focus-ring-offset);
}
```

---

## Step 3 — Add the toggle JavaScript

Add to `static/wiki-theme.js` (create if absent):

```javascript
(function () {
  var btn = document.querySelector('.ps-wiki-dark-toggle');
  if (!btn) return;

  function isDark() {
    return document.documentElement.dataset.theme === 'dark';
  }

  function syncButton() {
    var dark = isDark();
    btn.setAttribute('aria-pressed', String(dark));
    btn.setAttribute('aria-label', dark ? 'Switch to light mode' : 'Switch to dark mode');
    btn.querySelector('.ps-wiki-dark-toggle__icon').textContent = dark ? '☀' : '🌙';
    btn.querySelector('.ps-wiki-dark-toggle__label').textContent = dark ? 'Light' : 'Dark';
  }

  // Sync button to current theme on load
  syncButton();

  btn.addEventListener('click', function () {
    var dark = isDark();
    document.documentElement.dataset.theme = dark ? '' : 'dark';
    localStorage.setItem('ps-theme', dark ? 'light' : 'dark');
    syncButton();
  });
})();
```

Add to `base.html` before `</body>`:

```html
<script src="/static/wiki-theme.js" defer></script>
```

---

## Step 4 — Handle server-side rendering (optional)

If the Zola template can read a cookie to pre-render the correct aria-pressed state, add:

```html
<button class="ps-wiki-dark-toggle"
        type="button"
        aria-pressed="{{ theme_cookie | default(value='light') == 'dark' | lower }}"
        aria-label="{{ theme_cookie == 'dark' | ternary(true='Switch to light mode', false='Switch to dark mode') }}">
```

This requires the Zola template to have access to a cookie or header — generally not available in static generation. For static deployments, leave `aria-pressed="false"` as the HTML default and let JavaScript correct it on load (no flash occurs because the init script in `<head>` sets the correct visual theme before render).

---

## Step 5 — Verify

Open the wiki. Check:

1. Toggle button is present in the site header.
2. Clicking toggles the colour scheme instantly (no page reload).
3. Reload the page — the chosen theme is preserved.
4. Set OS dark mode preference on/off — when no explicit choice has been stored, the wiki follows the OS preference.
5. `aria-pressed` attribute on the button matches current state (inspect with browser DevTools → Accessibility panel).

---

## Research trail

### Done
- Toggle button ARIA pattern confirmed: aria-pressed (true/false), aria-label describes the action.
- localStorage key confirmed: `ps-theme` (values: `'dark'`/`'light'`).
- FOUT-theme prevention: inline script in `<head>` before stylesheet confirmed correct.
- WCAG audit: all dark mode colour pairs pass AAA — no accessibility blocker.
