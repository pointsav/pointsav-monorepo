# Print & PDF Pipeline — Workplace✦Memo

Technical reference for how documents are printed and exported as PDF.

---

## Overview

Workplace✦Memo uses the OS print subsystem for PDF export — the same
mechanism a user would use to print from a browser or word processor.
No third-party PDF library is required. No server is involved.

The key insight is that **the document is already in the correct format
for printing**: it is HTML with embedded fonts and CSS `@media print`
rules that mirror the canvas geometry exactly.

---

## What the user sees

1. User clicks **⎙ Print** in the toolbar, or uses **File → Print / Save as PDF**
2. A temporary print window opens containing the fully assembled document HTML
3. The OS print dialogue appears (native macOS/Linux/Windows print sheet)
4. The user selects **Save as PDF** (or a physical printer)
5. The print window closes automatically after the dialogue is dismissed

The printed document is identical to the canvas. There are no surprises.

---

## Why we open a new window for printing (not `window.print()` directly)

Calling `window.print()` on the main application window would print the
entire application chrome — toolbar, ruler, status bar, grey desktop background.
We prevent this by:

1. Assembling a clean, standalone HTML document (`WorkplaceExport.assembleHTML()`)
2. Opening it in a new window that contains **only** the document content
3. Triggering `window.print()` on that clean window

The new window is invisible to the user except for the OS print dialogue
that appears over it.

---

## The assembled HTML document

`WorkplaceExport.assembleHTML()` produces a single self-contained HTML file
containing:

| Section | Content |
|---|---|
| `@font-face` rules | All fonts base64-encoded as `data:font/woff2;base64,…` |
| `@page` rules | Paper size, margins, page-break helpers |
| Template CSS | Full CSS from the active template |
| `@media print` block | Ensures correct rendering in the print engine |
| Document body | The raw `innerHTML` of `#document-canvas` |

The file has **no external dependencies**. It renders correctly offline.

---

## Font embedding — why base64 is mandatory

The print engine (whether the OS print dialogue or a headless renderer)
loads fonts synchronously from the HTML before rendering. If a font is
referenced by URL:

- `https://fonts.googleapis.com/…` — **fails** if the app is offline,
  if the print engine has a different origin policy, or if Google changes
  the URL (common for versioned fonts)
- `file:///…` — **fails** in Tauri's WebView due to CSP and origin restrictions
- `data:font/woff2;base64,…` — **always works** regardless of network,
  origin, or time

All fonts in Workplace✦Memo are embedded as base64. The overhead is
~200–400 KB per document for two font families at three weights each —
acceptable for a permanent archived document.

---

## CSS `@page` rules

The `@page` block in the exported HTML controls paper size and margins:

```css
@page {
  size: 210mm 297mm;      /* A4. Letter: 216mm 279mm */
  margin: 25mm;           /* Set by user via ruler or Document menu */
}
@page :first {
  margin-top: 33mm;       /* Title page gets extra breathing room */
}
```

### macOS 10.13 (High Sierra) — known limitation

WKWebView on macOS 10.13 has **partial** `@page` support. Specifically:
- `size` and `margin` are supported ✅
- `@page :first` is supported ✅
- `@top-center`, `@bottom-center` margin box at-rules are **not supported** ✗

This means page headers and footers (Phase 2) will not render correctly
in PDF output from macOS 10.13. They will work correctly on:
- Linux via WebKitGTK 2.40+
- macOS 10.15+ (Catalina and later) via WKWebView
- Any Chromium-based browser's print dialogue

**Workaround for development validation:** Open the exported HTML file in
Firefox or Chrome on any platform and use that browser's print dialogue.
The result will be more accurate than WKWebView on 10.13.

---

## Page break control

The following CSS classes and inline styles control pagination:

| Method | Effect |
|---|---|
| `break-before: always` | Force a new page before this element |
| `break-inside: avoid` | Keep this element on a single page |
| `break-after: avoid` | Do not insert a page break after this element |
| `.page-break` class | Explicit page break (inserted via Insert menu) |

Applied automatically by Workplace✦Memo to common elements:
- `h1, h2, h3, h4` → `break-after: avoid` (prevents orphaned headings)
- `table, figure, pre` → `break-inside: avoid` (keeps them intact)

---

## Paged.js and the live canvas

The editor canvas uses **Paged.js** to render live page breaks as you type.
Paged.js is a W3C Paged Media specification polyfill that:

1. Intercepts the document content in `#document-canvas`
2. Computes page breaks based on the `@page` geometry
3. Renders each page as a discrete `<div class="pagedjs_page">` element

This gives the true Word-like page-on-grey-desktop appearance in real time.

**Important:** Paged.js renders a *preview*. The actual print/PDF output
is produced by the OS print engine from the assembled HTML, not from
the Paged.js preview. In practice they should be identical, but the
OS print engine is the authoritative output.

---

## Silent PDF export (Phase 2)

Phase 1 always shows the OS print dialogue. Phase 2 will add silent PDF
export (no dialogue, immediate file save) using platform-specific APIs:

| Platform | API | Status |
|---|---|---|
| Linux | WebKitGTK headless print API | Phase 2 |
| macOS | WKWebView `_printOperationWithPrintInfo` | Phase 2 |
| Windows | WebView2 `PrintToPdfAsync` | Phase 2 |

Silent export is useful for batch export and automation. The Phase 1
dialogue approach is preferred for end users — it gives them control
over paper size, destination, and copies.

---

## Print CSS checklist

Before a release, verify these items produce correct output:

- [ ] Fonts render correctly (not falling back to system fonts)
- [ ] Margins match the ruler settings exactly
- [ ] Headers (`h1`, `h2`) do not appear alone at the bottom of a page
- [ ] Tables do not split mid-row across pages
- [ ] Code blocks do not split across pages
- [ ] Background colours on `th`, `blockquote`, `pre` print correctly
  (requires `print-color-adjust: exact` — included in export CSS)
- [ ] First page has correct top margin
- [ ] All four built-in templates produce correct output
