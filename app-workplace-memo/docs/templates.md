# Template Format — Workplace✦Memo

Templates are the mechanism by which Workplace✦Memo separates document
content from visual presentation. Switching a template reflows the
entire document instantly — no reformatting, no copy-paste.

---

## What a template is

A template is a JavaScript object in `src/js/templates.js` containing:

```js
{
  label:    'Human-readable name shown in the toolbar',
  fonts:    ['Primary Font', 'Secondary Font'],   // families used
  pageSize: 'A4',          // 'A4' | 'Letter'
  marginsMm: 25,           // default margin in mm
  css:      `...`,         // the full CSS string for this template
}
```

The `css` string is injected into the document as a `<style>` block,
scoped to `body, #document-canvas`. It is the complete typographic
and layout specification for that template.

---

## Built-in templates

| Key | Label | Fonts | Intended use |
|---|---|---|---|
| `corporate` | Corporate Memo | EB Garamond + DM Sans | Formal internal memos, briefs |
| `technical` | Technical Spec | IBM Plex Sans + Source Code Pro | Specifications, API docs |
| `minimal` | Minimal Report | Lora + DM Sans | Editorial reports, long-form prose |
| `confidential` | Confidential Brief | Playfair Display + DM Sans | Sensitive documents, executive briefs |

---

## CSS conventions for template authors

### Required selectors

Every template must style these elements:

```
body / #document-canvas  — Base font, size, colour, line-height
h1                        — Primary title
h2                        — Section heading
h3                        — Sub-heading
h4, h5, h6               — Minor headings
p                         — Body paragraph
ul, ol, li               — Lists
code                      — Inline code
pre                       — Code blocks
blockquote               — Pull quotes
hr                        — Horizontal rule
a                         — Links
strong                    — Bold text
table, th, td            — Data tables
```

### Font family declarations

Reference fonts by name only — do not include fallbacks in the template
CSS. The `@font-face` rules in `font-data.js` embed the actual font
data. If the font is unavailable (e.g. during development before
`embed-fonts.sh` has been run), the browser will fall back to system
serif/sans automatically.

```css
/* Correct */
h1 { font-family: 'EB Garamond', serif; }

/* Not needed — Workplace*Memo handles @font-face injection */
/* Do NOT add @font-face rules to the template CSS */
```

### Units

Use `pt` for font sizes (they map correctly to print at 72pt = 1 inch).
Use `mm` for margins if setting them via `@page` (print only).
Use `px` only for UI decorations (borders, shadows) that do not print.

### Print behaviour

Template CSS applies both on-screen and in print output. Use
`@media print` sparingly and only for print-specific overrides.

The `@page` block is managed by `export.js` based on document settings —
do not include `@page` rules in template CSS.

---

## Adding a new template

1. Add an entry to the `TEMPLATES` object in `src/js/templates.js`
2. Add a `<option>` to the `#template-select` dropdown in `src/index.html`
3. Ensure all fonts referenced in the CSS are in the `fonts/` directory
4. Run `./scripts/embed-fonts.sh` if you added new font files
5. Test all four print paths: on-screen, print dialogue, HTML export,
   and opened-in-browser

---

## Exporting and sharing templates

Templates are self-contained CSS files. To share a template:

1. Extract the `css` string from `templates.js`
2. Save it as `<template-name>.css`
3. A receiving user places it in their `templates/` directory and
   registers it in their `templates.js`

A future release will formalise this into a template import/export UI.

---

## Template file format (planned)

Phase 2 will introduce a standalone `.wmtpl` template file format:

```json
{
  "version": 1,
  "name":    "Corporate Memo",
  "author":  "PointSav Digital Systems",
  "licence": "EUPL-1.2",
  "fonts":   ["EB Garamond", "DM Sans"],
  "pageSize": "A4",
  "marginsMm": 25,
  "css":     "... full CSS string ..."
}
```

Templates will be importable from the Document → Manage Templates panel.
