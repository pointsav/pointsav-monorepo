---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-proforma
artifact_type: DESIGN-COMPONENT
status: draft
language_protocol: DESIGN-COMPONENT
audience: vendor-internal
bcsc_class: no-disclosure-implication
target_repo: pointsav/pointsav-design-system
destination_path: components/financial-report-layout/
routes_to: project-design
authored: 2026-06-10T00:00:00Z
authored_by: jmwoodfine@gmail.com (claude-code)
authored_with: claude-opus-4-8
paired_with: not-required-internal
component_metadata:
  component_name: financial-report-layout
  component_kind: document-template
  carbon_baseline: >
    no Carbon baseline. Carbon's DataTable is an interactive, screen-first
    component (sortable headers, row selection, pagination, toolbar) bound to
    a React model. This component is the inverse: a static, print-first
    compliance document layout where multiple independent <table> elements
    must align column-for-column across page breaks, line numbers are gutter
    artifacts not data, and the primary render target is a letter-landscape
    PDF. No Carbon component models cross-table column alignment, print @page
    sizing, or compliance line-number gutters, so there is no analog to extend.
    The component is authored as raw HTML + CSS (no framework dependency) so it
    can be emitted by codegen agents into a single self-contained .html file.
  accessibility_targets: [wcag-2-2-aa, table-semantics-scope, print-contrast-exact, no-color-only-meaning]
  brand_voice_alignment: [precise, institutional, compliance-grade]
research_done_count: 6
research_suggested_count: 2
open_questions_count: 2
research_provenance: primary-source-extraction
research_inline: true
references:
  - tool-proforma/ (WCP V2 proforma working source)
  - outputs/ (delivered compliance financial report HTML)
notes_for_designer: |
  Extracted verbatim from the delivered WCP V2 compliance financial report
  proforma, polished over two sessions. The CSS, the line-number injection JS,
  and the HTML structural patterns are load-bearing and tested in print.
  Do not refactor stylistically on intake. Two artifacts ship together:
    1. components/financial-report-layout/guide.md      (the recipe — for humans)
    2. dtcg-vault/research/component-financial-report-layout.md  (the rationale — for codegen agents)
  The research file is what a generation agent reads when asked to produce
  a new proforma / income statement / book valuation. Keep the "why" in
  research.md and the "how" in guide.md; do not merge them.
---

# DESIGN-COMPONENT — Financial report layout

## Context

A WCP V2 compliance financial report proforma was delivered as a single
self-contained HTML file. The layout — letter-landscape print, 13-column wide
tables with cross-table column alignment, a line-number gutter, semantic
total/subtotal/section-banner rows, and a BCSC compliance footer — is reusable
for any compliance financial document: proformas, income statements, book
valuations, summary sheets. This artifact extracts that pattern so future
reports begin from the tested baseline.

Two files are committed into `pointsav-design-system`:

- `components/financial-report-layout/guide.md` — the recipe a developer uses.
- `dtcg-vault/research/component-financial-report-layout.md` — the rationale a
  codegen agent reads to understand what to emit and why.

Both files' full content follows.

---

## component: guide.md

> Destination: `components/financial-report-layout/guide.md`

# Financial Report Layout

A print-first, self-contained HTML + CSS recipe for compliance financial
documents: proformas, income statements, book valuations, summary sheets.
No framework, no build step. One `.html` file, one inline `<style>` block,
one inline `<script>` for line numbers. Emits a letter-landscape PDF.

## When to use

Use this layout when **all** of the following hold:

- The document is a financial statement or projection intended for a
  compliance audience (BCSC or equivalent) and will be printed or PDF'd.
- The data is wide — multiple year/period columns (typically 11 data columns
  plus a label column) that must line up across several separate tables.
- Rows carry structural meaning (totals, subtotals, section banners) that
  must read the same on screen and in print.
- Output is a single static file. No interactivity, sorting, or pagination.

Do **not** use this for interactive on-screen dashboards (use Carbon DataTable),
for narrow single-table summaries that fit portrait, or for any document where
column counts differ table-to-table.

## Complete CSS

Paste verbatim into a single `<style>` block in `<head>`.

```css
/* --- Document frame ----------------------------------------------------- */
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1400px}
/* system-ui: no web-font dependency (file may open offline / inside PDF pipeline).
   max-width:1400px caps line length on wide monitors; removed in print. */

h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
/* Three heading tiers. h2 hairline rule separates statement sections. */

p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
/* Prose smaller than base — narrative is secondary to the tables.
   p.note: inline caveats / basis-of-preparation remarks. */

/* --- Tables: base ------------------------------------------------------- */
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.76rem}
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
/* Numbers right-aligned, never wrap. Headers centre, body cells right. */

td.lbl,th.lbl{text-align:left;min-width:230px}
/* The label column is the one left-aligned column. */

/* --- Tables: wide (multi-year) ----------------------------------------- */
table.wide{width:100%;table-layout:fixed}
table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}
/* table-layout:fixed makes every table.wide compute the SAME column widths
   from the SAME rule set — separate <table> elements align column-for-column
   down the page. Label pinned to 25%; labels may wrap because a fixed 25%
   can't always fit a long label on one line. */

/* --- Line-number gutter ------------------------------------------------- */
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
/* Injected by JS — never authored. !important overrides inherited tr.total /
   tr.subtotal / th backgrounds so the gutter stays white and un-bolded even
   on emphasised rows. It must read as a margin rule, not part of the figure. */

/* --- Semantic rows ------------------------------------------------------ */
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
tr.section-banner td{background:#e3edf7;font-weight:700;font-size:0.74rem;text-transform:uppercase;letter-spacing:.3px;color:#1a2a44;text-align:left}
/* section-banner: full-width divider naming a block of rows.
   subtotal:       running sum within a section (1px top rule, light fill).
   total:          final sum (2px top rule, heavier fill, bold).
   Apply the class to <tr>; rules cascade to its <td>. */

/* --- Footer ------------------------------------------------------------- */
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}

/* --- Print -------------------------------------------------------------- */
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{
  body{margin:0;font-size:11px;max-width:none}
  table{break-inside:avoid;page-break-inside:avoid}
  h2,h3{break-after:avoid;page-break-after:avoid}
  td.lnum,th.lnum{-webkit-print-color-adjust:exact;print-color-adjust:exact;color:#bbb!important;border-right-color:#ccc!important}
  table.wide{table-layout:fixed;font-size:10px}
  table.wide td,table.wide th{padding:3px 6px}
  table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}
}
/* table.wide rules RESTATED in @media print: some print engines re-resolve
   table layout at print time and would revert to auto, breaking alignment.
   print-color-adjust:exact forces the faint gutter and fills to survive print. */
```

## Class reference

| Selector | Element | Purpose | When to use |
|---|---|---|---|
| `table.wide` | `<table>` | Fixed-layout multi-period table | Any table with year/period columns that must align with others |
| `td.lbl` / `th.lbl` | cell | Left-aligned label (row name) column | First content cell of every row; pinned to 25% in wide tables |
| `td.lnum` / `th.lnum` | cell | Line-number gutter | Never author — injected by the script |
| `tr.section-banner` | row | Full-width uppercase section divider | Names a block of rows; use `colspan` to span all columns |
| `tr.subtotal` | row | Running subtotal within a section | 1px top rule, light fill |
| `tr.total` | row | Final / grand total | 2px top rule, heavier fill, bold |
| `p.note` | `<p>` | Italic inline caveat | Basis-of-preparation notes inline with a table |
| `.footer` | `<p>` | Compliance notice block | Forward-looking / BCSC notice at document end |

## HTML structural recipe

### Document scaffolding

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>ENTITY — Document Title</title>
  <style>/* paste the Complete CSS block here */</style>
</head>
<body>
  <h1>Entity Name — Document Type</h1>
  <p>DRAFT — YYYY-MM-DD — Version<br>
  All amounts [CURRENCY] — Prepared under [STANDARD] — Forward-looking projections; [DISCLOSURE POSTURE]</p>

  <!-- sections: h2 / tables / notes -->

  <p class="footer"><strong>Forward-Looking Information.</strong> Notice body.</p>
  <script>/* paste the line-number injection script here */</script>
</body>
</html>
```

### Narrow table (inputs, summary metrics)

```html
<table>
  <tr><th class="lbl">Parameter</th><th>Value</th><th>Ref</th></tr>
  <tr><td class="lbl">Discount rate</td><td>8.0%</td><td>§3.1</td></tr>
</table>
```

### Wide table (multi-period, 11 data columns)

Twelve authored content columns (1 label + 11 periods). Banner/spanning rows
use `colspan="12"`. The gutter is inserted by JS and is not in your colspan.

```html
<table class="wide">
  <tr>
    <th class="lbl">Line</th>
    <th>Y0</th><th>Y1</th><th>Y2</th><th>Y3</th><th>Y4</th><th>Y5</th>
    <th>Y6</th><th>Y7</th><th>Y8</th><th>Y9</th><th>Y10</th>
  </tr>
  <tr class="section-banner"><td colspan="12">OPERATING REVENUE</td></tr>
  <tr>
    <td class="lbl">Gross rental income</td>
    <td>—</td><td>$1.20M</td><td>$1.24M</td><td>$1.28M</td><td>$1.32M</td>
    <td>$1.36M</td><td>$1.40M</td><td>$1.44M</td><td>$1.48M</td><td>$1.53M</td><td>$1.58M</td>
  </tr>
  <tr class="subtotal">
    <td class="lbl">Net operating income</td>
    <td>—</td><td>$0.96M</td><td>$0.99M</td><td>$1.02M</td><td>$1.06M</td>
    <td>$1.09M</td><td>$1.12M</td><td>$1.15M</td><td>$1.18M</td><td>$1.22M</td><td>$1.26M</td>
  </tr>
  <tr class="total">
    <td class="lbl">Total return to fund</td>
    <td>—</td><td>$0.80M</td><td>$0.83M</td><td>$0.85M</td><td>$0.88M</td>
    <td>$0.91M</td><td>$0.94M</td><td>$0.97M</td><td>$1.00M</td><td>$1.03M</td><td>$1.06M</td>
  </tr>
</table>
```

Keep the column count and label/data split identical across every `table.wide`
in the document — alignment depends on structural sameness.

## Line-number injection

Append once at the end of `<body>`. Numbers rows sequentially across all tables
in document order. Include for numbered compliance statements; omit for informal
summaries. Do not reset per table.

```javascript
(function(){
  var n=1;
  document.querySelectorAll('table').forEach(function(tbl){
    tbl.querySelectorAll('tr').forEach(function(row){
      var allTh=Array.from(row.children).every(function(c){return c.tagName==='TH';});
      var cell=document.createElement(allTh?'th':'td');
      cell.className='lnum';
      cell.textContent=n++;
      row.insertBefore(cell,row.firstChild);
    });
  });
})();
```

## Print layout guidance

Test before sending to print / PDF:

1. **Print preview, not screen.** Render to PDF and confirm the gutter,
   banner, total, and subtotal fills all appear (requires `print-color-adjust:exact`).
2. **Cross-table alignment in print.** Stack two wide tables and confirm
   columns line up in the PDF (the `@media print` restatement is why).
3. **No table splits mid-row.** `break-inside:avoid` keeps tables whole;
   split tables taller than one page at section boundaries.
4. **Headings stay with their rows.** `break-after:avoid` on h2/h3.
5. **Type fits.** Print steps table to 10px. With 13 rendered columns on
   letter landscape this is tight — abbreviate ($1.20M not $1,200,000)
   or reduce period columns if figures clip.

## ARIA / accessibility notes

- Add `scope="col"` on column headers and `scope="row"` on `td.lbl` cells.
- `tr.total`, `tr.subtotal`, and `tr.section-banner` each carry a non-colour
  signal (border weight, font-weight, uppercase) — do not differentiate by
  fill alone.
- `td.lnum` is decorative. See open question in research file on deep-linkable
  line numbers.
- `print-color-adjust:exact` fills all clear WCAG contrast against their text.

---

## component: research.md

> Destination: `dtcg-vault/research/component-financial-report-layout.md`
> Audience: codegen agents. Read this before generating a financial report.

# Research — Financial Report Layout

This file records why each decision in the financial-report-layout component
was made. A generation agent producing a proforma, income statement, book
valuation, or summary sheet should read this, then emit HTML/CSS that honours
the constraints.

## 1. Why `table-layout:fixed` + explicit label `width:25%`

**Problem:** A compliance financial report is several separate `<table>`
elements (Revenue, Costs, Capital, Returns), each with its own section heading
and page-break behaviour. With default `table-layout:auto`, each table sizes
columns to its own content, so "Y3" in the Revenue table lands at a different
x-position than "Y3" in the Costs table. The period alignment — the entire
point of a multi-year statement — is lost.

`table-layout:fixed` makes column widths a function of CSS rules, not cell
contents. Every `table.wide` resolves the same widths: label at 25%, the
remaining 75% divided across 11 data columns. Because the rule set is shared,
every wide table gets identical geometry and columns align without per-table
tuning.

The rule is **restated inside `@media print`** because some print engines
re-resolve table layout at print time and would otherwise revert to auto,
breaking alignment in the PDF.

**Codegen rule:** keep column count and label/data split identical across every
`table.wide` in a document.

## 2. Why `tr.total` / `tr.subtotal` / `tr.section-banner` as semantic classes

These are **roles**, not styles. Encoding them as classes (not inline):

1. Single source of visual truth — one edit changes every total.
2. Machine-readable — a tool can find every total with `tr.total`.
3. Non-colour differentiation — roles escalate by border weight and
   font-weight, not just fill, surviving greyscale print and colour vision
   deficiency.

**Codegen rule:** apply exactly one role class per emphasised row, on the
`<tr>`. Banner rows use `colspan="12"` over the 12 authored content columns.

## 3. Why `td.lnum` is injected by JS, not server-rendered

Line numbers are a property of the *rendered document*, not the *data*. Baking
them into source rows requires recomputation every time a row is added, removed,
or reordered. Client-side injection after layout means the number is always
correct for the document as rendered, continuous across all tables.

The `!important` flags on `.lnum` exist solely to win against the inherited
`tr.total` / `tr.subtotal` / `th` backgrounds and weights — the gutter must
read as a margin rule, not as a figure.

**Codegen rule:** emit data rows only. Append the injection script once at the
end of `<body>`. Do not author `.lnum` cells. Do not count the gutter in your
`colspan` values.

## 4. Why letter-landscape `@page`

A label plus 11 periods does not fit portrait at a legible size. Landscape is
the compliance print standard for multi-year statements. **Letter** (not A4)
because the audience is a North American securities context (BCSC); the
regulator and filers print on letter stock.

Margins `1.5cm 2cm 1.5cm 1.5cm` — the wider value on the bound/punch edge
leaves room for holes without eating the gutter or first data column.

**Codegen rule:** do not change `@page` for a compliance financial report.

## 5. Colour tokens and semantic meaning

| Value | Used by | Meaning |
|---|---|---|
| `#111` | body text | Primary ink |
| `#555` | `p`, `p.note` | Secondary ink — narrative subordinate to tables |
| `#333` | `h3` | Sub-heading ink |
| `#ccc` | borders, `h2` rule | Hairline grid |
| `#f5f5f5` | `th` background | Header band |
| `#aaa` / `#bbb` | `td.lnum` ink (screen / print) | Gutter numerals — faint, recedes |
| `#d0d0d0` / `#ccc` | gutter right border | Boundary: gutter / content |
| `#eef2f7` | `tr.total` fill | Heaviest emphasis. Bottom line |
| `#f5f7fa` | `tr.subtotal` fill | Lighter emphasis. Intermediate sum |
| `#e3edf7` | `tr.section-banner` fill | Most saturated. Names a block |
| `#1a2a44` | `tr.section-banner` ink | Dark navy — the only coloured text |
| `#888` | `tr.total` top border | Heavy rule above bottom line |
| `#aaa` | `tr.subtotal` top border | Lighter rule above intermediate sum |
| `#666` / `#ddd` | `.footer` ink / rule | Compliance notice, quietest block |

The three emphasis fills are one cool-blue family at three saturations — the
hierarchy reads as one coherent system. All three clear WCAG contrast against
`#111` / `#1a2a44` in print.

**Codegen rule:** do not recolour a role. If a brand theme overrides, keep the
three-saturation relationship intact.

## 6. Typography scale

| Element | Size | Rationale |
|---|---|---|
| `body` | 13px (11px print) | Base; steps down in print to fit landscape |
| `h1` | 1.25rem | Document title; one per document |
| `h2` | 1rem + hairline rule | Statement section |
| `h3` | 0.9rem | Sub-section, no rule |
| `p` | 0.82rem | Narrative — smaller than base, subordinate to tables |
| `p.note` | 0.78rem italic | Inline caveat |
| `table` | 0.76rem (10px print wide) | Smallest legible; maximises columns per page |
| `tr.section-banner` | 0.74rem uppercase | Header read at low height via uppercase + letter-spacing |
| `td.lnum` | 9px monospace | Below data size; reads as metadata |
| `.footer` | 0.72rem | Required but visually deprioritised |

`system-ui` throughout (no web-font dependency); `'Courier New'` monospace
on the gutter reads as a ruled margin.

**Codegen rule:** do not enlarge `p`; do not shrink table type below 0.76rem
screen / 10px print — figures clip below that with 13 columns on letter landscape.

## Research trail

### Done (6)
- Extracted CSS, line-number JS, and HTML patterns verbatim from the delivered
  WCP V2 proforma (primary source; polished over two sessions).
- Verified cross-table alignment depends on `table-layout:fixed` + shared 25%
  label width, and that the rule must be restated in `@media print`.
- Confirmed `!important` on `.lnum` is required to override inherited
  total/subtotal/header backgrounds.
- Confirmed the three emphasis fills are one blue family at three saturations
  and clear WCAG contrast against their text in print.
- Confirmed `print-color-adjust:exact` is required for gutter and fills to
  survive print.
- Confirmed colspan accounting: 12 authored content columns; gutter inserted
  by script outside the authored colspan.

### Suggested (2)
- Validate the letter-landscape margin asymmetry against a real binding/
  hole-punch sample.
- Test in at least one non-Chromium print engine to confirm fixed-layout
  alignment holds.

### Open questions (2)
- Should `td.lnum` be semantically addressable (e.g. an `id` per line) so a
  reviewer's "line 42" can deep-link, or does it remain purely decorative?
- For tables taller than one printed page: should we introduce repeating
  `<thead>` so the period header reprints on each page? Repeating headers
  would interact with the line-number injector (header rows would re-number).
