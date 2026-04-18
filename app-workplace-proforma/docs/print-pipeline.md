# Print Pipeline — Workplace\*Proforma

How the application generates a PDF from the in-memory grid, without any third-party PDF library.

---

## Approach

The application uses the **operating system's native print dialogue** to produce PDFs. It does not bundle a PDF library. It does not perform its own page layout calculation. It relies on the fact that every modern OS has a mature, well-tested, free PDF generator built into its print subsystem: macOS's Quartz, Linux's CUPS, Windows's Microsoft Print to PDF.

This is the same approach used by `app-workplace-memo`. It is the right approach because:

1. **Zero additional dependency.** No PDF library in the bundle. No CVE surface. No upstream licensing to reconcile.
2. **OS-native fidelity.** The PDF looks exactly as it prints. Users recognise the output as legitimate — it is produced by the same subsystem that produces their invoices and shipping labels.
3. **Accessibility inherited.** The OS print subsystem handles accessible PDF tags on macOS and Linux.
4. **Future-proof.** When the OS updates its PDF generator, the application's PDF output updates automatically with no code change.

---

## Trigger paths

The user reaches the OS print dialogue through three paths:

- **Menu bar:** File → Print / Save as PDF…
- **Toolbar:** the `⎙ Print` button
- **Keyboard:** `Cmd+P` (macOS) / `Ctrl+P` (Linux, Windows)

Each path calls `WorkplaceExport.print()`, which calls `window.print()`.

---

## CSS pipeline

The critical piece is the `@media print` block in `src/styles/app.css`. When the OS enters print preview, the WebView re-renders the page with `@media print` rules applied. The application chrome disappears; the grid flattens onto the print surface; the OS rasterises the result to PDF.

```css
@media print {
  #menubar,
  #toolbar,
  #formulabar,
  #sheet-tabs,
  #statusbar,
  #dropdown-overlay,
  .dropdown { display: none !important; }

  #workbook-desktop {
    padding: 0 !important;
    background: white !important;
    overflow: visible !important;
  }

  #grid-container { box-shadow: none !important; }
  #grid-scroll    { overflow: visible !important; }

  #proforma-grid td.cell.selected {
    outline: none !important;
    background: transparent !important;
  }

  body {
    background: white !important;
    color: black !important;
    overflow: visible !important;
  }
}
```

This does five things:

1. **Hide chrome.** Menu bar, toolbar, formula bar, sheet tabs, status bar, and dropdowns all disappear.
2. **Remove screen-only visual affordances.** The selected-cell outline, the hover states, the panel shadows all reset to print-appropriate values.
3. **Flatten overflow.** `#grid-scroll` normally clips the grid to the visible viewport. For print, it must let the grid flow across pages.
4. **White background.** The dark chrome palette is a screen affordance. Print is black-ink-on-white.
5. **Release the body.** `overflow: hidden` on `html, body` is appropriate for a desktop application but would prevent multi-page grids printing correctly.

---

## Page breaks

The browser's default page-break behaviour handles most grids correctly: a long table will break at row boundaries rather than mid-row. For proformas with section headers, two additional rules can be added in Phase 2 to control break placement:

```css
@media print {
  tr.section-header { break-before: page; }
  tr.row-total      { break-after: avoid-page; }
}
```

These are not enabled by default because the first pass treatment — letting the browser decide — produces acceptable output for typical proforma lengths.

---

## Per-platform notes

### macOS

macOS's print dialogue offers "Save as PDF" directly. The resulting PDF:
- Embeds the system monospace font used by the grid
- Preserves tabular number alignment exactly
- Includes accessibility tags if the source HTML was tagged

Tauri v1 on macOS 10.13 uses WKWebView, which routes `window.print()` through Quartz. The dialogue appears natively; the user picks page size, orientation, and destination.

### Linux

Linux uses CUPS via the WebKitGTK print integration. "Print to File" with format "PDF" is always available. "Save as PDF" may also appear depending on distribution.

Note: some Linux desktops delegate to GTK's print dialogue rather than a custom one. The functionality is the same; the visual presentation differs between GNOME, KDE, and Cinnamon.

### Windows

Windows 10 and 11 include "Microsoft Print to PDF" as a default virtual printer. Users select it from the printer list and save to a file. WebView2's `window.print()` routes through the standard Windows print subsystem.

---

## Landscape vs portrait

Proformas with many year columns (10 years × 108px ≈ 1080px) are typically wider than they are tall. The OS print dialogue lets the user pick landscape orientation. Phase 2 may add a `@page { size: A4 landscape }` hint in the CSS to make landscape the default; Phase 1 lets the user choose.

---

## Paper sizes

No paper size is hard-coded. The OS dialogue exposes A4, Letter, A3, Tabloid, Legal, and custom sizes; the user picks. Phase 2 may expose a "Page Setup" dialog in the View menu that writes a preferred size into `document.presentation.page_size` for deterministic re-renders across machines.

---

## Colour and ink

The `@media print` CSS flips the page to white-background / black-text. Numeric cells use `color: #1a1a1a` rather than pure black to reduce harshness under studio lighting — visible in both screen and print.

Negative numbers are rendered in a muted red (`#a03028`) inherited from institutional finance conventions. On a monochrome printer, the red appears as mid-grey, which is acceptable. Phase 2 may add a "Print in black only" option that flattens all colour before print.

---

## Print exhaust vs canonical file

A PDF is an **exhaust** produced from the `.json` canonical file. It is:

- Not editable in the application
- Not reconstructable back to the canonical file (PDF is lossy relative to the schema)
- Cryptographically signed at generation time via the OS print subsystem (where supported)
- Suitable for distribution to parties who should not alter the model

The canonical `.json` file is always the authoritative record. The PDF is a point-in-time snapshot for distribution. This is the same role PDF plays in `app-workplace-memo`.

---

## What gets printed

The current sheet only. Phase 1 does not print all sheets at once. To print all three sheets (Assumptions, Proforma, Returns) as a single PDF, the user prints each sheet separately and merges — or waits for Phase 2's "Print Workbook" command which iterates over every sheet, rendering each on its own page.

---

## See also

- `src/styles/app.css` — the `@media print` block
- `src/js/export.js` — `WorkplaceExport.print()`
- [ARCHITECTURE.md](../ARCHITECTURE.md) ADR-006 — IPC surface minimalism
- [../README.md](../README.md) § "The file is the product" — canonical vs exhaust
