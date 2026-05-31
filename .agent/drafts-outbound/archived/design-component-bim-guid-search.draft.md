---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-guid-search-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "bim-token-taxonomy.md component catalog (console-unique #1); sub-agent A report 2026-04-28; IFC 4.3 IfcRoot.GlobalId (22-character base64 compressed GUID)"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-guid-search/
---

# bim-guid-search — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-guid-search` |
| IFC anchor | `IfcRoot.GlobalId` |
| Uniclass 2015 | FI_60 |
| Surface scope | Console-unique (read-only operations) |
| Mode | Read-only — no mode prop needed |
| Container element | `<search class="bim-guid-search">` |
| ARIA role | `<search>` landmark with `aria-label="Find element by GUID"` |

## Purpose

Accepts an IFC GUID (22-character base64-compressed string, or
full UUID hyphenated format) and looks up the element in the vault.
Returns: IFC class name, Name, Uniclass classification, Pset summary,
and a link to the full properties panel. This is the primary
diagnostic instrument for operations staff who receive a GUID from a
contractor, a BCF issue, or a system alert and need to locate the
physical element it refers to.

## GUID formats supported

| Format | Example |
|---|---|
| IFC compressed (22 char) | `2N1NMOV9z7$8Ww2DVqvxPB` |
| UUID hyphenated | `b94af52e-1a6c-4b3c-8d41-9f7c2e0a3d12` |
| Partial prefix (8+ chars) | `2N1NMOV9` → autocomplete candidates |

## Visual anatomy

```
.bim-guid-search (search, aria-label="Find element by GUID")
  .bim-guid-search__input-row
    input.bim-guid-search__input
      (type="search", placeholder="Enter IFC GUID…", aria-label="IFC GUID",
       pattern="[0-9A-Za-z_$]{22}|[0-9a-f-]{36}", spellcheck="false",
       autocomplete="off", aria-autocomplete="list")
    button.bim-guid-search__submit (aria-label="Search")

  .bim-guid-search__result (aria-live="polite", role="status")
    .bim-guid-search__result-header
      code.bim-guid-search__guid "2N1NMOV9z7$8Ww2DVqvxPB"
      .bim-guid-search__class-chip "IfcWall"
    .bim-guid-search__result-name "WAL-EXT-001 — External wall (EF_25_10_30)"
    .bim-guid-search__result-location "Ground Floor → Zone A → Grid B/2-3"
    .bim-guid-search__result-psets
      "Pset_WallCommon · Qto_WallBaseQuantities · Pset_WallCommon_FireRating"
    a.bim-guid-search__result-link href="/elements/{guid}"
      "View full properties →"

  .bim-guid-search__error (hidden unless error, role="alert")
    "Element not found: [guid]"
```

## Interaction model

1. User types 8+ characters → client-side autocomplete from loaded element index
   (IFC GUIDs are globally unique; no need for fuzzy matching — prefix match only)
2. User presses Enter or clicks Search button → POST to `/api/elements?guid=...`
3. Result panel renders with live region announcement for screen readers
4. If not found: `role="alert"` error message; input retains focus

## ARIA contract

- Container: `<search>` (HTML5 landmark) with `aria-label`
- Input: `aria-label="IFC GUID"`, `aria-autocomplete="list"`, `aria-controls="[results-id]"`
- Result area: `aria-live="polite"`, `role="status"` — announces result on update
- Error: `role="alert"` (live region, assertive) — announces immediately
- GUID display: `<code>` with `aria-label="Element GUID"`

## CSS token dependencies

- `--bim-font-mono` — GUID display (monospace, 22-char fits single line at 13px)
- `--bim-accent` — input focus ring
- `--bim-bg-surface` — panel background
- `--bim-text-sm` — result metadata lines

## Service dependency

Calls `service-buildings` → `GET /elements/{guid}` (scaffolded at port 9102).
Falls back to client-side search in loaded IFC model when vault endpoint is
unavailable (offline-first posture per CLAUDE.md §6).
