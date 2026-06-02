---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: draft
title: "Knowledge Wiki Deployment — Fonts, Content Mounts, and Mobile-First Notes"
slug: guide-knowledge-wiki-deployment
target_repo: woodfine-fleet-deployment
target_path: gateway-knowledge-documentation-1/guide-knowledge-wiki-deployment.md
bilingual_required: false
author: totebox@project-editorial
created: 2026-06-01
version: "0.1"
editorial_notes: >
  §F placeholder GUIDE — font loading is current fact (Inter + Source Serif 4 chosen);
  content mounts (knowledge.toml) is planned/intended language throughout (Phase 6).
  Bloomberg pass applied. No ES sibling required.
---

# Knowledge Wiki Deployment — Fonts, Content Mounts, and Mobile-First Notes

This guide covers three operational topics for `app-mediakit-knowledge` deployments:
self-hosting the Inter and Source Serif 4 typefaces, the intended `knowledge.toml` mount
manifest format (Phase 6, planned), and mobile-first deployment considerations.

---

## 1. Self-hosting Inter and Source Serif 4

The wiki engine loads Inter (UI and headings) and Source Serif 4 (body reading prose)
from the `/static/fonts/` directory of the deployment's static-asset bundle. No external
font CDN request is made at read time — this protects reader privacy and ensures
the wiki renders correctly in air-gapped environments.

### Required font files

Place the following files in the deployment's `/static/fonts/` directory:

| File | Purpose | Source |
|---|---|---|
| `inter-var.woff2` | Inter variable font (Latin subset) | fonts.google.com/specimen/Inter or fontsource.org |
| `inter-var-ext.woff2` | Inter variable font (latin-ext, required for Spanish) | same |
| `SourceSerif4Variable-Roman.woff2` | Source Serif 4 variable (Latin subset) | fonts.google.com/specimen/Source+Serif+4 |
| `SourceSerif4Variable-Roman-ext.woff2` | Source Serif 4 variable (latin-ext) | same |

Variable font files cover the full weight axis (Inter 100–900; Source Serif 4 200–900).
Each file replaces multiple static-weight files and produces a smaller combined download.

### CSS bundle (`dist/tokens.css`)

The engine's CSS bundle declares the `@font-face` rules. Verify the following declarations
are present in the built CSS:

```css
@font-face {
  font-family: 'Inter';
  src: url('/static/fonts/inter-var.woff2') format('woff2');
  font-weight: 100 900;
  font-style: normal;
  font-display: swap;
  unicode-range: U+0000-00FF, U+0131, …; /* Latin */
}

@font-face {
  font-family: 'Source Serif 4';
  src: url('/static/fonts/SourceSerif4Variable-Roman.woff2') format('woff2');
  font-weight: 200 900;
  font-style: normal;
  font-display: swap;
  unicode-range: U+0000-00FF, U+0131, …; /* Latin */
}
```

`font-display: swap` prevents invisible text during font load: the fallback system font
renders immediately; Inter and Source Serif 4 swap in when their downloads complete.

### Verification

After deployment, open the wiki in a browser with DevTools Network panel. Filter by
`font` type. Confirm four WOFF2 requests complete with status 200 from the wiki's own
origin (not fonts.googleapis.com or any external domain).

---

## 2. Content mounts (`knowledge.toml`) — Phase 6, planned

> **Note.** The `knowledge.toml` mount manifest is a planned capability for Phase 6 of
> `app-mediakit-knowledge`. The specification below describes the intended design.
> The single-repository model is the current deployed form.

The intended mount manifest file `knowledge.toml` at the content directory root is planned
to declare which git repositories the engine fetches content from, and which blueprint each
mounted repository uses.

### Intended `knowledge.toml` structure

```toml
# knowledge.toml — content mount manifest (Phase 6 planned)

[instance]
name = "documentation.pointsav.com"

[[mount]]
source_repo  = "https://github.com/pointsav/media-knowledge-documentation"
mount_path   = "."
blueprint    = "topic"
edit_url     = "https://github.com/pointsav/media-knowledge-documentation/edit/main/{path}"

[[mount]]
source_repo  = "https://github.com/pointsav/media-knowledge-projects"
mount_path   = "projects/"
blueprint    = "topic"
edit_url     = "https://github.com/pointsav/media-knowledge-projects/edit/main/{path}"
```

### Intended blueprint behaviour

| Blueprint | URL pattern | Index inclusion | Notes |
|---|---|---|---|
| `topic` | `/:category/:slug` | Primary article index | Standard wiki article |
| `guide` | `/guides/:slug` | Guide catalog only | Operational doc; distinct chrome |
| `regional-market` | `/markets/:slug` | Markets index | Domain-specific pluggable blueprint (planned) |

The `edit_url` template receives the article's `source_path` as `{path}`. The engine is
intended to render an "edit this page" link that routes editors to the source repository
rather than accepting local writes — preserving the source-of-truth inversion across a
federated surface.

---

## 3. Mobile-first notes

No server configuration is required for mobile support. The engine ships mobile-first CSS:
the layout, navigation, and table of contents render correctly at 375 px before progressive
enhancement for wider viewports.

Verify the `<meta>` viewport tag is present in the rendered HTML `<head>`:

```html
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="color-scheme" content="light dark">
```

The `color-scheme` meta prevents the browser from drawing a white flash before the
dark-mode initialisation script reads `localStorage`. Both tags are present in the engine's
default templates.

Touch targets across all interactive elements — TOC entries, edit pencils, navigation links,
toggle buttons — meet the 44 px × 44 px minimum (WCAG 2.2 SC 2.5.8). Safe-area padding
(`env(safe-area-inset-*)`) is applied to the shell chrome for notched displays. No
operator action is required; these properties are built into the engine's default CSS.

---

## See also

- `app-mediakit-knowledge` — the Rust wiki engine this guide covers
- `patterns/federation-via-content-mounts.md` — architecture of the content-federation
  pattern this guide operationalises
