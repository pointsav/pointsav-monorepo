---
schema: foundry-draft-v1
artifact: DESIGN-TOKEN-CHANGE
language_protocol: DESIGN-TOKEN-CHANGE
state: draft-pending-master-cosign
originating_cluster: project-proforma
created: 2026-06-14
to: project-design
master_cosign: ""
research_trail:
  source_files:
    - tool-proforma-engine/src/report/forecast_statements.rs
    - tool-proforma-engine/src/report/pclp1_proforma.rs
    - tool-proforma-engine/src/html.rs
    - .agent/drafts-outbound/DESIGN-COMPONENT-financial-report-layout.draft.md
    - inputs/tokens-woodfine.css
  brief: project-proforma-financial-forecast
  downstream_impact: >
    New brand-neutral financial-statement token bundle (wcp.finance.*) extracted from the
    CSS currently hardcoded in the engine's report renderers. FEEDS the existing
    financial-report-layout component (does not supersede it). First consumer is the
    forecast-statements report family (5 entities × jurisdictions). Tokenization is a pure
    value-extraction refactor: at the default (Canada) theme the rendered PDF must be
    byte-for-byte unchanged. ES/MX jurisdiction themes are additive (A4 geometry,
    currency/number/date locale, Spanish static labels). No existing committed consumer to
    migrate; the engine carries a compiled-in mirror of the default :root for offline,
    self-contained HTML.
---

# DESIGN-TOKEN-CHANGE — wcp-finance-bundle — 2026-06-14

**Route:** project-design → `pointsav-design-system`
- DTCG JSON: `tokens/finance.tokens.json` (new top-level `wcp.finance` group)
- Generated CSS themes: `tokens/theme-finance-base.css`, `tokens/theme-finance-es.css`,
  `tokens/theme-finance-mx.css`
- Research file: `dtcg-vault/research/finance-report-bundle.md`
- Companion edits to the already-staged DESIGN-COMPONENT-financial-report-layout
  (`components/financial-report-layout/guide.md`): tokenize the verbatim CSS values.

**Blocked on:** `master_cosign` from Command Session (mandatory per `token-intake-checklist.md`
before committing to `pointsav-design-system` — this is a generic/shared design-system change,
not a Woodfine brand-only patch).

---

## Why

The financial-report CSS is currently hardcoded and duplicated (in `tool-proforma-engine/src/
html.rs` and inside each report renderer). The forecast-statements package must reproduce across
five entities and at least three jurisdiction locales (CA/US English, ES SOCIMI, MX FIBRA).
Tokenizing the values lets the same generated HTML re-theme per jurisdiction with no structural
change, and removes the CSS drift between renderers.

## Two themes — `statement` (classic) and `dashboard` (modern)

Research (2026-06-14 OPUS panels; see `[[BRIEF-financial-forecast]]`) established that a
compliance/offering **financial statement** must use the **classic audited look** (serif, pure
black-and-white, figures in thousands, accounting rules, landscape statements + portrait notes),
while **investor/analytical exhibits** (sensitivity, tear-sheets, MD&A) legitimately use a modern
**dashboard** look. So the bundle ships two themes under one namespace:

- **`statement` (classic compliance — PRIMARY).** Implemented in
  `tool-proforma-engine/src/report/forecast_statements.rs` (2026-06-14). Attributes to tokenize:
  serif family stack; `#000` on `#fff`, **no fills/tints**; accounting rules
  (`row.subtotal.border-top` single 0.75pt; `row.total.border-top` 1pt + `border-bottom` 3pt
  double); `number.scale = thousands` (comma groups; parentheses negatives; en-dash nil;
  tabular-nums); section captions bold+underline; `table.layout = fixed` with `col.label = 22%` +
  10 × `7.8%`; `Projected N / $` column heads; per-statement header block (entity / title /
  forecast / "Expressed in thousands of … dollars"); `page.statement = letter landscape`,
  `page.notes = letter portrait` (WeasyPrint named `@page`); `@bottom-right` page counter.
- **`dashboard` (modern exhibits — COMPANION).** The earlier sans-serif + tinted-row + `$M`
  abbreviation token set, scoped to `direct_hold_sensitivity` / `tearsheet_alt_re_v2` and IR
  outputs only. Never used for the audited statements.

`number.scale` is a token (`thousands` default; `full` to exactly match the legacy sample's
full-number presentation). The assurance/practitioner's-report block is NOT rendered in the
statement theme (management-prepared forecast); the token group is retained for future use.

## Relationship to the existing component

The bundle **feeds** `DESIGN-COMPONENT-financial-report-layout` — it does not replace it. The
component keeps structure/mechanism (HTML scaffold, fixed-layout doctrine, page-break utilities,
ARIA). The bundle owns the literal values that CSS hardcodes. NB: the classic `statement` theme
drops the line-number gutter and all row fills the component originally documented — those belong
to the `dashboard` theme.

## Token taxonomy — `wcp.finance.*` (DTCG dotted path → `--wcpf-*` CSS custom properties)

Representative tokens (not exhaustive):

- **color** — `wcp.finance.color.ink` (#111), `.ink.secondary`, `.rule.hairline` (#ccc),
  `.rule.strong` (#888), `.header.bg` (#f5f5f5), `.accent` (finance domain; aliases brand green
  on the Woodfine side only).
- **font / text** — `wcp.finance.font.family.base` (system-ui), `.family.mono`,
  `wcp.finance.text.size.{base,table,note,footer,gutter}` (screen/print pairs),
  `wcp.finance.text.weight.{total,subtotal,header}`.
- **space** — `wcp.finance.space.cell.{x,y}` (6px/3px), `.body.margin`, `.section.gap`.
- **table** — `wcp.finance.table.label.width` (24–25%), `.layout` (fixed), `.header.align`,
  `.cell.align` (right).
- **page/print** — `wcp.finance.page.size` (letter→A4 for ES/MX), `.orientation` (landscape),
  `.margin` (asymmetric bind edge → symmetric for ES/MX), `.break.{before,after}`,
  `.print.color-adjust` (exact).
- **row (financial semantics)** — `wcp.finance.row.{total,subtotal,banner}.{bg,border,weight}`
  (#eef2f7 / #f5f7fa / #e3edf7 + #1a2a44 banner ink).
- **number (financial semantics)** — `wcp.finance.number.negative.style` (parenthetical),
  `.negative.wrap` ( "( )" vs "-" — locale-switchable), `.zero.glyph` (em-dash),
  `.per-unit.{weight,style}`, `.align` (right), `.font-variant` (tabular-nums).
- **gutter** — `wcp.finance.gutter.{width,font,size,ink,border}` (line-number column).
- **note** — `wcp.finance.note.{size,style,ink}`.
- **assurance** — `wcp.finance.assurance.{bg,border,ink,title.weight,size}` (the boxed
  Independent Practitioner's Assurance Report block).
- **footer** — `wcp.finance.footer.{size,ink,rule,title.weight}` (BCSC forward-looking notice).
- **locale (i18n)** — `wcp.finance.locale.currency.{symbol,position}`,
  `.number.{decimal,group}`, `.date.format`, `.lang`, plus a `wcp.finance.label.<key>` static-
  label set (en/es) for fixed strings ("DRAFT", section banners, the forward-looking notice).

## Engine consumption mechanism

1. Extract the duplicated CSS into one `REPORT_CSS` const = base `:root{--wcpf-*}` defaults +
   tokenized rules (rules reference `var(--wcpf-*)`).
2. Per-jurisdiction theme = a `:root{…}` override block injected ahead of the base (later
   declarations win), exactly like `theme-woodfine-wcp.css`. `THEME_CA_BCSC` (default),
   `THEME_ES`, `THEME_MX`.
3. WeasyPrint supports `var()` — pure CSS, no JS dependency; works in both the Chromium
   (line-numbered) and WeasyPrint (CI/no-JS) paths.
4. The engine ships a compiled-in mirror of the default `:root` (self-contained offline HTML
   requirement); canonical token definitions live in `pointsav-design-system/tokens/`. A future
   `xtask` codegen step can regenerate the Rust mirror from `finance.tokens.json` to prevent drift.

## Verify

- Golden-file zero-diff at the default (Canada) theme after tokenization (HTML `<body>` unchanged;
  only the `<style>` `:root` + `var()` references added). Rasterize PDFs (`pdftoppm` + image diff)
  — default-theme diff must be empty.
- ES/MX theme-swap smoke test: A4 geometry, `€`/comma-decimal rendering, Spanish static labels,
  `table.wide` column alignment still holds.

## Split surfaced to project-design

Keep the generic bundle brand-neutral (structural finance semantics + neutral defaults). Any
genuine Woodfine-brand binding (e.g. `wcp.finance.color.accent → --wf-green`) aliases on the
`woodfine-media-assets` side per the three-repo routing rule, so the bundle stays reusable for
non-Woodfine entities and jurisdictions.
