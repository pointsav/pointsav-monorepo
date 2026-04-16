# Font System — Workplace✦Memo

Technical reference for how fonts are managed, embedded, and delivered.

---

## Design principle

Fonts are a first-class feature. Every font used in a document is embedded
in the exported HTML file as a base64 data URI. The document renders
identically offline on any machine, in any browser, now or in twenty years.

No font is ever loaded from a CDN or external server at runtime.
The application makes zero outbound network connections after installation.

---

## Three-stage font lifecycle

```
Stage 1: Source
  .woff2 files in fonts/
  (not committed — downloaded once by download-deps.sh)

Stage 2: Build
  embed-fonts.sh reads .woff2 files
  → writes src/js/font-data.js (base64 strings, .gitignored)

Stage 3: Runtime
  font-data.js loads into window.WORKPLACE_FONT_DATA
  fonts.js injects @font-face rules into document head
  Fonts are available to the editor canvas and all templates
  On export: @font-face rules with base64 data written into the HTML file
```

---

## font-data.js structure

```js
window.WORKPLACE_FONT_DATA = {
  'EB Garamond': {
    '400': {
      'normal': 'AAEAAAARAQAABAAQR...',   // base64 WOFF2
      'italic': 'AAEAAAARAQAABAAQR...',
    },
    '600': {
      'normal': 'AAEAAAARAQAABAAQR...',
    },
  },
  'DM Sans': {
    '400': { 'normal': '...' },
    '500': { 'normal': '...' },
    '700': { 'normal': '...' },
  },
  // ...
};
```

This file is **generated at build time** and is `.gitignore`d.
It must be regenerated when font files change.

---

## @font-face injection

`fonts.js` calls `injectBuiltinFontFaces()` on `DOMContentLoaded`.
This creates a single `<style id="workplace-font-faces">` block in
`document.head` containing all `@font-face` declarations using
`data:font/woff2;base64,…` URIs.

The fonts are available immediately — no async loading, no FOUT
(Flash of Unstyled Text), no network round-trip.

---

## Export — font embedding in HTML

When `WorkplaceExport.assembleHTML()` is called, it calls
`buildFontFaceCSS()` which reads from `window.WORKPLACE_FONT_DATA`
and writes `@font-face` rules directly into the `<style>` block
of the exported HTML file.

The exported file is therefore completely self-contained:
the font data travels with the document.

---

## File size considerations

| Scenario | Approximate additional size |
|---|---|
| 1 font family, 1 weight, 1 style | ~30–50 KB |
| 2 families, 3 weights each | ~200–400 KB |
| Full 8-family set, all weights | ~1.5–2 MB |

A standard document using one body font + one heading font (the common
case for all built-in templates) adds ~200–350 KB to the exported file.
This is acceptable for a permanent, archived document.

---

## Adding downloaded fonts (Fonts panel — Phase 2)

Users will be able to download additional OFL fonts from the Fonts panel.
Downloaded fonts are:
- Stored in the OS app data directory: `$APPDATA/workplace-memo/fonts/`
- Read via the `read_font_file` Tauri IPC command (returns base64)
- Registered as additional `@font-face` rules at runtime
- Included in the embedded export

The `read_font_file` Rust command is already implemented. The UI panel
is Phase 2.

---

## Licence compliance

All built-in fonts are SIL Open Font Licence 1.1.

The OFL explicitly permits:
> "The fonts and derivatives, however, cannot be sold by themselves"
> "The fonts, along with any derivative works, can be bundled, embedded,
> redistributed and/or sold with any software..."

Embedding fonts in exported HTML files and distributing those files
is fully permitted under the OFL. Embedding fonts in the application
binary (`font-data.js` is compiled into the Tauri bundle) is also
fully permitted.

Full OFL text: https://scripts.sil.org/OFL
