---
schema: foundry-draft-v1
state: ready-for-sweep
language_protocol: GUIDE
originating_cluster: project-design
target_repo: customer/woodfine-fleet-deployment
target_path: pclp1-wiki/
target_filename: guide-wiki-design-tokens.md
audience: wiki-deployment-engineers
bcsc_class: customer-internal
bilingual: false
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z (typography + token spec).
  dist/tokens.css inspection — CSS custom property names and values.
  IBM Plex font delivery (training data: Google Fonts, npm @ibm/plex).
research_inline: true
---

# Guide: Wiki Design Tokens

Operational runbook for integrating the PointSav design token stylesheet into a Woodfine wiki deployment. Follow these steps when provisioning a new wiki instance or updating tokens after a design-system release.

---

## Prerequisites

- Access to the deployment's `static/` directory
- Access to Zola template files (`templates/`)
- The latest `tokens.css` from `vendor/pointsav-design-system/dist/tokens.css`

---

## Step 1 — Copy tokens.css to the deployment

```bash
# From the deployment root
cp vendor/pointsav-design-system/dist/tokens.css static/tokens.css
```

`tokens.css` is a generated file — do not edit it directly. All token changes go through the design-system source (`dtcg-vault/exports/tokens.full.json`) and must be promoted through the design-system release process.

---

## Step 2 — Link tokens.css in the base template

In `templates/base.html` (or the Zola theme's base template), add the stylesheet link in `<head>`:

```html
<head>
  <!-- Design tokens — must load before any component CSS -->
  <link rel="stylesheet" href="/static/tokens.css">

  <!-- Dark mode init script — must run before tokens.css renders -->
  <script>
    (function() {
      var stored = localStorage.getItem('ps-theme');
      var prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (stored === 'dark' || (!stored && prefersDark)) {
        document.documentElement.dataset.theme = 'dark';
      }
    })();
  </script>

  <!-- Other stylesheets reference token vars — load after tokens.css -->
  <link rel="stylesheet" href="/static/wiki.css">
</head>
```

**Order matters:** `tokens.css` must appear before any stylesheet that references `var(--ps-*)` custom properties. The dark-mode init script must appear before `tokens.css` to prevent flash of light theme on dark-mode page loads.

---

## Step 3 — Add IBM Plex fonts

### Option A — Self-hosted (recommended for privacy)

Download WOFF2 files from npm `@ibm/plex` or the IBM Plex GitHub repository. Place in `static/fonts/`:

```
static/
  fonts/
    IBMPlexSans-Regular.woff2
    IBMPlexSans-SemiBold.woff2
    IBMPlexSans-Bold.woff2
    IBMPlexMono-Regular.woff2
    IBMPlexMono-Medium.woff2
```

Add `@font-face` declarations to `static/fonts.css` (create if absent):

```css
@font-face {
  font-family: 'IBM Plex Sans';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('/static/fonts/IBMPlexSans-Regular.woff2') format('woff2');
}
@font-face {
  font-family: 'IBM Plex Sans';
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url('/static/fonts/IBMPlexSans-SemiBold.woff2') format('woff2');
}
@font-face {
  font-family: 'IBM Plex Sans';
  font-style: normal;
  font-weight: 700;
  font-display: swap;
  src: url('/static/fonts/IBMPlexSans-Bold.woff2') format('woff2');
}
@font-face {
  font-family: 'IBM Plex Mono';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('/static/fonts/IBMPlexMono-Regular.woff2') format('woff2');
}
@font-face {
  font-family: 'IBM Plex Mono';
  font-style: normal;
  font-weight: 500;
  font-display: swap;
  src: url('/static/fonts/IBMPlexMono-Medium.woff2') format('woff2');
}
```

Add `<link rel="stylesheet" href="/static/fonts.css">` to `base.html` before `tokens.css`.

### Option B — Google Fonts CDN

If self-hosting is not available, Google Fonts hosts IBM Plex Sans and IBM Plex Mono:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600;700&display=swap" rel="stylesheet">
```

Note: Google Fonts requests are visible to Google's servers. Use Option A for deployments with strict privacy requirements.

---

## Step 4 — Apply token variables in wiki CSS

Use the short-form aliases defined by `tokens.css` in your wiki-specific stylesheet:

```css
/* wiki.css */

body {
  font-family: var(--font-sans);          /* IBM Plex Sans chain */
  font-size:   var(--ps-wiki-font-size-base);  /* 1.0625rem / 17px */
  line-height: var(--leading-body);       /* 1.6 */
  color:       var(--color-text-primary);
  background:  var(--color-surface-page);
}

.wiki-article {
  max-width: var(--measure);  /* 65ch */
}

h1 { font-size: var(--text-h1); }  /* 2.25rem */
h2 { font-size: var(--text-h2); }  /* 1.75rem */
h3 { font-size: var(--text-h3); }  /* 1.375rem */
h4 { font-size: var(--text-h4); }  /* 1.125rem */

code, pre {
  font-family: var(--font-mono);
  background:  var(--color-surface-code);
}

a {
  color: var(--color-text-link);
}

a.redlink {
  color: var(--color-text-redlink);
}
```

---

## Step 5 — Verify dark mode

Test the dark mode by opening the wiki and:

1. Setting `localStorage.setItem('ps-theme', 'dark')` in the browser console.
2. Refreshing the page — the dark theme should load without a flash of light colours.
3. Toggling the dark-mode-toggle button — the theme should switch instantly.
4. Setting `localStorage.removeItem('ps-theme')` and refreshing with OS dark mode on — wiki should match OS preference.

---

## Updating tokens

When a new `tokens.css` is released by the design system:

1. Copy the updated file to `static/tokens.css`.
2. Check the changelog (`vendor/pointsav-design-system/CHANGELOG.md`) for any token renames or removals.
3. Search `static/wiki.css` for any removed token names and update them.
4. Commit: `git add static/tokens.css && commit-as-next.sh "tokens: update to design-system vX.X.X"`.

---

## Research trail

### Done
- tokens.css structure verified against `dist/tokens.css` (generated 2026-05-06).
- IBM Plex self-hosting confirmed as SIL OFL 1.1 — no licence restrictions on serving WOFF2.
- Google Fonts IBM Plex URL pattern confirmed (training data — verify current URL format against fonts.google.com before using in production).
- Dark-mode init script pattern confirmed correct (inline script before first paint).

### Suggested
- Add variable font instructions once exact WOFF2 sizes are confirmed against live `@ibm/plex` package.

### Open questions
1. Confirm `@ibm/plex` current version and exact WOFF2 file sizes for latin + latin-ext subsets before finalising the self-hosting step.
