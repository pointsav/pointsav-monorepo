---
schema: foundry-draft-v1
state: asset-staged-pending-master-access
language_protocol: ASSET
originating_cluster: project-design
target_repo: pointsav/pointsav-media-assets
target_path: icons/
target_filename: ps-badge-favicon.svg
audience: vendor-internal
bcsc_class: no-disclosure-implication
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: tacit
research_inline: false
notes_for_master: |
  This draft is a staging record for the PS badge SVG file. Write the SVG
  content below to pointsav-media-assets/icons/ps-badge-favicon.svg once
  the cluster write access decision from the outbox message (2026-05-08)
  is resolved.

  Companion design decision record:
  research-ps-badge-favicon-design-2026-05-08.draft.md

  No language pass required — binary/vector artefact.

  Commit message suggestion:
  "media: add ps-badge-favicon.svg — Design System inline SVG favicon asset"
---

# Asset — PS Badge Favicon SVG

## File to write

**Destination:** `pointsav-media-assets/icons/ps-badge-favicon.svg`

**Content:**

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <rect width="100" height="100" rx="12" fill="#234ed8"/>
  <text
    x="50"
    y="50"
    dominant-baseline="central"
    text-anchor="middle"
    fill="#ffffff"
    font-family="Arial,sans-serif"
    font-weight="700"
    font-size="40">PS</text>
</svg>
```

## Usage reference

**As a data URI in HTML (inline favicon):**

Encode `#` as `%23` and wrap with the data URI scheme:

```html
<link rel="icon" type="image/svg+xml"
  href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><rect width='100' height='100' rx='12' fill='%23234ed8'/><text x='50' y='50' dominant-baseline='central' text-anchor='middle' fill='%23ffffff' font-family='Arial,sans-serif' font-weight='700' font-size='40'>PS</text></svg>">
```

This is the form currently live in `app-privategit-design/src/render.rs` line 30.

**As a static file served by nginx:**

```nginx
location = /favicon.svg {
    root /srv/foundry/deployments/vault-privategit-design-1/assets/;
}
```

With a corresponding `<link rel="icon" type="image/svg+xml" href="/favicon.svg">`.

## Token correspondence

| SVG attribute | Value | Token |
|---|---|---|
| `rect fill` | `#234ed8` | `color.primary-60` in `vault/tokens/primitive.json` |
| `text fill` | `#ffffff` | white (no token; absolute) |

If `color.primary-60` changes, this SVG file must be updated to match. The data
URI form in `render.rs` must also be updated manually — CSS custom properties
are not resolvable inside data URIs.
