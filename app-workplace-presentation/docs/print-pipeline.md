# Print Pipeline — technical reference

> Reference for the developer implementing Phase 7 of Workplace✦Presentation.

---

## What it does

File → Print (or `Ctrl+P`) produces a PDF handout: one slide per landscape
page. The user triggers the OS print dialog and chooses "Save as PDF" as the
printer destination. No PDF library is shipped — the browser's print-to-PDF is
the production path.

Uses Paged.js, the same pagination library memo uses for document printing.
Already vendored in `src/js/vendor/paged.polyfill.js` via `scripts/download-deps.sh`.

---

## Print stylesheet contract

```css
@page {
  size: letter landscape;       /* US Letter: 11" × 8.5" */
  margin: 0;                    /* 1:1 with canvas — no margins needed */
}

.slide {
  page-break-after: always;
  width: 100%;
  height: 100%;
  /* Slides print at native 1:1 — canvas is already US Letter landscape */
}

.slide:last-child {
  page-break-after: auto;       /* Prevent a trailing blank page */
}
```

Editor chrome (menubar, navigator, code pane, status bar) hidden with
`@media print { .editor-chrome { display: none; } }`.

**Why no aspect transformation:** Canvas is US Letter landscape. Print page is
US Letter landscape. They're the same geometry. No reflow, no scaling, no
surprises — the printed page is a pixel-exact render of the canvas.
See ADR-PR-09 for the aspect ratio rationale.

---

## Trigger sequence

1. User selects File → Print or presses `Ctrl+P`.
2. `print.js` builds a print view: clone the current deck's slides into a
   standalone fragment. Apply the print stylesheet.
3. Invoke Paged.js to lay out the fragment into paged form (`PagedPolyfill.preview()`
   on the fragment).
4. After Paged.js emits its `rendered` event, call `window.print()`.
5. The browser opens its native print dialog. User selects destination
   (physical printer or "Save as PDF").
6. On print dialog close, tear down the print view and restore the editor.

---

## What does not happen

- **No new font embedding for print.** Fonts are already in the document via
  `@font-face` rules — the browser uses them directly.
- **No PDF library.** The browser's native print-to-PDF is the production path.
  Adding a PDF library (e.g. pdf-lib, jsPDF) would bloat the app, add a
  dependency to audit, and produce worse output than the browser engine.
- **No custom handouts with multiple slides per page.** Phase 7 ships one slide
  per page only. Multi-slide handouts (2×2, 3×3 layouts with notes) are
  deferred — see `CLEANUP_LOG.md`.
- **No speaker notes in the print output.** Deferred with the speaker notes
  feature.

---

## Margin behaviour

Full-bleed (`margin: 0`) is the default because a slide is visually complete —
it has its own padding, background, and typography. Adding page margins would
create an unwanted white border around each slide.

Users who want margins can select them in the OS print dialog. This is standard
browser behaviour and does not require editor support.

---

## Testing a print build

```
1. Open a 5-slide deck in the editor.
2. File → Print.
3. In the OS print dialog, select "Save as PDF" as the printer.
4. Save to desktop.
5. Open the PDF. Verify:
   - 5 landscape pages
   - One slide per page
   - Fonts render correctly (embedded)
   - No editor chrome visible
   - Content matches canvas render
```

If the PDF shows editor chrome, a `@media print` rule is missing or overridden.
If fonts render as fallbacks, the `@font-face` rules didn't reach the print
view — the print fragment clone is probably losing the `<style>` block.
