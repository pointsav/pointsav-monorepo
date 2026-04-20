# Fonts — reference

> SIL Open Font Licence families used by Workplace✦Presentation.
> Downloaded by `scripts/download-deps.sh`, base64-embedded by `scripts/embed-fonts.sh`.

---

## The set

Eight families, all OFL. Same set as Workplace✦Memo — ensures cross-app
consistency when users move content between document, spreadsheet, and
presentation.

| Family | Style | OFL source | Recommended use |
|---|---|---|---|
| Source Sans 3 | Humanist sans | Adobe Originals | Body text default |
| Source Serif 4 | Transitional serif | Adobe Originals | Classical body, headlines |
| Source Code Pro | Monospace | Adobe Originals | Code blocks, code-view pane |
| JetBrains Mono | Monospace | JetBrains | Code-view pane alternative |
| Inter | Neo-grotesque sans | Rasmus Andersson | Modern UI, large display |
| IBM Plex Sans | Neo-grotesque sans | IBM | Corporate modern |
| Playfair Display | Transitional display | Claus Eggers Sørensen | Editorial titles |
| Fira Sans | Humanist sans | Mozilla | Friendly body text |

All eight are permissively OFL-licensed. Redistribution and embedding is
explicitly allowed. No royalty, no runtime licence check, no phone-home.

---

## Embedding strategy

Fonts are embedded into saved `.html` files as base64 strings inside `@font-face`
rules in the inlined `<style>` block. This is a standard CSS pattern:

```css
@font-face {
  font-family: 'Source Sans 3';
  font-weight: 400;
  font-style: normal;
  src: url(data:font/woff2;base64,d09GMgABAAAAAC...) format('woff2');
}
```

**Only fonts actually used by the deck are embedded.** At export time, `export.js`
inspects every text element in every slide, collects the set of
(family, weight, style) tuples actually referenced, and embeds only that set.

This keeps saved file size manageable. A deck using 2 families × 2 weights is
~200 KB of font data. A deck referencing all 8 families × 4 weights each would
be ~2–3 MB — still portable, but worth avoiding if not needed.

---

## Build-time vs runtime embedding

`scripts/embed-fonts.sh` runs at setup time and generates `src/js/font-data.js`
containing all available fonts as base64 constants. This is used by two callers:

1. **The editor** — loads fonts at runtime via `fonts.js` for the font panel
   and live preview.
2. **`export.js`** — when saving a file, reads the base64 constants from
   `font-data.js` and inlines the needed subset into the saved file's `<style>`
   block.

The file `src/js/font-data.js` is gitignored. It is regenerated on every fresh
clone by `make setup`.

---

## Why WOFF2 specifically

All sources are WOFF2 (Web Open Font Format 2). WOFF2 has better compression
than WOFF or raw TTF, is universally supported by every browser since 2017,
and can be embedded as base64 without compatibility concerns.

Do not substitute TTF or OTF sources. They inflate saved file size 30–50%.

---

## Licence compliance in saved files

Every saved `.html` file must carry an OFL attribution comment. Recommended
placement: a single comment block above the `@font-face` rules:

```css
/*
  Fonts embedded in this file are licensed under the SIL Open Font Licence
  version 1.1. See https://scripts.sil.org/OFL for the full licence text.
  Embedded families: Source Sans 3 (Adobe), JetBrains Mono (JetBrains),
  ...
*/
```

`export.js` generates this comment dynamically based on which fonts are
actually embedded in the file.

---

## Adding a new family

Do not add without commission. If a user needs a specific face:

1. Confirm OFL (or similarly permissive: Apache 2.0, MIT). Other licences need
   legal review — do not accept CC-BY-ND, SIL-ONL, or proprietary EULAs.
2. Add to `scripts/download-deps.sh` with a verified checksum.
3. Add to the table above in this file.
4. Re-run `make setup` to re-embed.
