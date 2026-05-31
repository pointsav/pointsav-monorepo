---
artifact: design-spec
status: draft
type: DESIGN-WIKI-REDESIGN-SPEC
created: 2026-05-25
round: 2
---

# Wiki Design Redesign Spec — Round 2

## Executive summary

**Winner: proto-platform-document, taken as the base, with surgical grafts from proto-institutional-register.** The platform-document direction is the only one that reads as a serious institutional engineering product — sidebar + content + TOC three-pane shell, restrained navy, Oswald display for hierarchical structure, Roboto Slab for lede/blockquote weight, no novelty chrome. It is also the closest to production: the CSS is structured around design tokens already aligned with our `--navy / --fg / --bg` vocabulary, the article view has a working TOC with active-state styling, and the home page composes hero → featured → category grid → recents in the order an engineer expects to read. It directly solves every C+ defect: real Google Fonts links, a proper "PointSav" wordmark (no hyphenation), category-driven sidebar navigation, a strong typographic hero, slab-serif blockquote/lede.

The graft from proto-institutional-register is narrow: the **structural three-row header** (utility → brand → category nav-row), the **metric ledger stripe** between hero and category grid, and the **article meta strip rendered as a `<dl>`** instead of flex pills. The institutional-register prototype as a whole is overdressed (Vol. XIV No. 248, ISSN numbers, "Of Record" stamps, double-rule masthead borders) and trades authority for theatre; we keep its skeleton and discard its costume. The editorial-standard prototype is rejected outright — newspaper-of-record italics, drop-caps with small-caps first-line, "PointSav & Co.", "From the Library" with red italic emphasis — it reads as a literary magazine, not an engineering wiki, and inverts every reading expectation of a technical reader. Cream paper background is also off-brand against our `#F7F9FA` paper-2 token.

## Scoring table

| Dimension | platform-document | institutional-register | editorial-standard |
|---|---|---|---|
| 1. Institutional authority | 9 | 7 | 5 |
| 2. Typography quality | 9 | 8 | 8 |
| 3. Navigation clarity | 8 | 9 | 6 |
| 4. Home page impact | 8 | 7 | 6 |
| 5. Article reading experience | 9 | 8 | 8 |
| 6. Dark mode quality | 9 | 7 | 6 |
| 7. Mobile usability | 8 | 6 | 6 |
| 8. Brand parameterisation | 8 | 5 | 4 |
| 9. Differentiation | 7 | 8 | 7 |
| 10. Implement-ability | 9 | 6 | 5 |
| **Total** | **84** | **71** | **61** |

## Per-prototype narrative

**proto-platform-document (winner).** Reads as the engineering library Stripe / Linear / Vercel would ship if they were institutional rather than consumer-facing. Brand mark is a navy 30px rounded square `PS` glyph, wordmark is "PointSav" + "Documentation Wiki" stacked subtitle in Oswald — exactly the wordmark shape we need. Sidebar lists all 12 categories with article counts in tabular-num — solves the "navigation only has Disclaimer/Contact" defect in one stroke. Hero combines eyebrow + 52px Oswald display title + Nunito Sans lede + metric strip. Featured callout uses a 3px navy gradient stripe + accent-gold (`#C7A961`) dot — restrained, not festive. Article view has crumb → 44px title → slab-serif lede → border-top/border-bottom meta strip → prose with slab-serif blockquote left-bordered in accent-gold → numbered sticky TOC. Dark mode is properly designed: navy stays brand, paper inverts to `#0B1220` with `#7AAEEA` links — not a CSS filter. *What it gets wrong:* only a two-row header (brand+search+nav is one row), so utility-row separation is missing — graft from institutional-register fixes this. Single instance branding is hard-coded (no `data-instance` attribute) — fixable via token overrides. Brand mark uses `PS` glyph that's fine for PointSav but needs to switch to `W` for Woodfine instances.

**proto-institutional-register.** Strongest structural header in the field: real three-row split (utility bar on dark navy with theme toggle, masthead with centred wordmark, category index bar with 12-cell horizontal nav + search). The four-column ledger stripe (`248 / 12 / 3 / Today`) is the strongest single home-page element across all three protos — keep it. Article meta as a `<dl>` with vertical rules between cells is institutional and elegant. *What it gets wrong:* the whole "newspaper of record" framing — "Established MMXXIV", "Vol. XIV No. 248", "ISSN 3041-0042", "Of Record" diamond stamps, "Section A · Architecture", "Article Reading View" double-rule dividers — is theatre that an engineering reader will read as posturing. The masthead `font-size: 56px` Oswald centred wordmark on dark navy is striking but locks us into one brand presentation and breaks at 480px. Drop-cap `::first-letter` on the lede is a category error for technical writing. Mobile collapses badly (horizontal category bar wraps to 4 rows). Dark mode is colour-inverted, not designed.

**proto-editorial-standard (rejected).** Source Serif 4 italic masthead title "PointSav & Co." with red ampersand. Cream `#F5F1E8` paper background. Drop-cap with small-caps first-line on every article. "From the *Library*", "The *Twelve* Categories", "Long *Reads*" with red italic emphasis word. § flourish horizontal rules (`§ § §`). It is well-executed for what it is, but what it is is wrong: this is the New Yorker, not an engineering wiki. Cream background also violates our `#F7F9FA` paper-2 token. The article-page reading body at 19px Source Serif is genuinely pleasant, but the cost — every other surface reading as essayistic rather than referential — is too high. The `.toc li::before` decimal-leading-zero counter pattern is worth stealing, and we will: proto-platform-document already has the same pattern, so this is a non-event.

## Design decision: winner or hybrid

Base: **proto-platform-document** in full — its token system, layout shell, sidebar, hero, featured, category grid, recent list, article-wrap with TOC, footer.

Grafts from **proto-institutional-register**:

1. **Three-row header structure.** Replace platform-document's single-row `.site-header` with three stacked rows:
   - `.utility-row` (top, height 32px, `--bg-subtle` background, separates lang toggle + appearance toggle + auth state on the right; Disclaimer/Contact links on the left)
   - `.brand-row` (height 72px, wordmark left, search centred, GitHub + pointsav.com right)
   - `.nav-row` (height 48px, category links left-aligned in Oswald uppercase, sticky)
   Keep the platform-document visual treatment (no double-rule borders, no centred masthead theatre).

2. **Ledger stripe.** Insert `.ledger-stripe` from institutional-register between hero and category grid. Four facts: Articles, Categories, Live Instances, Last Updated. Use platform-document's `--navy` solid background (NOT the navy-900 masthead), white text, Oswald display nums, tabular-num. The numbers are factual content, not theatre.

3. **Article meta as `<dl>`.** Replace platform-document's flex `.article__meta` row with a 4-column `<dl>` with vertical rules between cells, using institutional-register's `.article-meta` pattern but with platform-document's restrained typography (no "Signed: jwoodfine Ed25519" theatrics — keep the practical fields: Category, Status, Last reviewed, Cites, Language).

Reject from institutional-register: masthead datelines, "Establishment MMXXIV", ISSN, "Of Record" markers, "Volume / No.", "Air-gap Compatible" badge in the register strip, double-rule dividers, drop-cap on lede, article section `§ 1` Roman/section markers, the entire `.article-divider` "Section A · Article of Record" strip.

Reject from editorial-standard entirely.

## Implementation spec

### Google Fonts `<link>` block

Place inside the existing `head!` Maud block in `server.rs`, immediately before the existing `link rel="stylesheet"` for the local stylesheet:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Oswald:wght@400;500;600;700&family=Nunito+Sans:wght@300;400;600;700&family=Roboto+Slab:wght@400;500&display=swap" rel="stylesheet">
```

Maud syntax:
```rust
link rel="preconnect" href="https://fonts.googleapis.com";
link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Oswald:wght@400;500;600;700&family=Nunito+Sans:wght@300;400;600;700&family=Roboto+Slab:wght@400;500&display=swap";
```

### Token updates (dtcg-bundle.json additions/changes)

```json
{
  "color": {
    "navy":      { "$value": "#164679", "$type": "color" },
    "navy-deep": { "$value": "#0F3258", "$type": "color" },
    "navy-soft": { "$value": "#2E63A0", "$type": "color" },
    "accent-gold": { "$value": "#C7A961", "$type": "color", "$description": "Used for featured-callout dot, blockquote left border, dark-mode lede first-letter. Not for primary actions." },
    "paper-1":   { "$value": "#FFFFFF", "$type": "color" },
    "paper-2":   { "$value": "#F7F9FA", "$type": "color" },
    "paper-3":   { "$value": "#EEF1F4", "$type": "color" },
    "paper-hover": { "$value": "#E7ECF1", "$type": "color" },
    "ink-1":     { "$value": "#111827", "$type": "color" },
    "ink-2":     { "$value": "#4B5563", "$type": "color" },
    "ink-3":     { "$value": "#6B7280", "$type": "color" },
    "ink-4":     { "$value": "#9CA3AF", "$type": "color" },
    "rule":      { "$value": "#E3E7EB", "$type": "color" },
    "rule-strong": { "$value": "#CDD3DA", "$type": "color" }
  },
  "color-dark": {
    "paper-1":   { "$value": "#111A2C" },
    "paper-2":   { "$value": "#0B1220" },
    "paper-3":   { "$value": "#16213A" },
    "paper-hover": { "$value": "#1C2A45" },
    "ink-1":     { "$value": "#F3F6FA" },
    "ink-2":     { "$value": "#C9D1DC" },
    "ink-3":     { "$value": "#94A3B8" },
    "ink-4":     { "$value": "#64748B" },
    "rule":      { "$value": "#1F2C44" },
    "rule-strong": { "$value": "#2E3E5C" },
    "link":      { "$value": "#7AAEEA" }
  },
  "font": {
    "display": { "$value": "'Oswald', ui-sans-serif, system-ui, sans-serif" },
    "body":    { "$value": "'Nunito Sans', ui-sans-serif, system-ui, sans-serif" },
    "serif":   { "$value": "'Roboto Slab', ui-serif, Georgia, serif" },
    "mono":    { "$value": "ui-monospace, 'SF Mono', Menlo, Consolas, monospace" }
  },
  "layout": {
    "sidebar-w":   { "$value": "272px" },
    "toc-w":       { "$value": "248px" },
    "utility-h":   { "$value": "32px" },
    "brand-h":     { "$value": "72px" },
    "nav-h":       { "$value": "48px" },
    "header-h":    { "$value": "152px", "$description": "Sum of utility-h + brand-h + nav-h. Used by sticky offsets." },
    "content-max": { "$value": "1440px" },
    "reading-max": { "$value": "760px" },
    "radius-sm":   { "$value": "6px" },
    "radius":      { "$value": "10px" },
    "radius-lg":   { "$value": "16px" }
  }
}
```

Instance overrides (parameterisation): add a top-level `instance` block keyed by `data-instance` attribute value:

```json
"instance": {
  "pointsav": {
    "wordmark-primary": "PointSav",
    "wordmark-sub": "Documentation Wiki",
    "brand-mark-glyph": "PS",
    "external-link-label": "pointsav.com",
    "external-link-href": "https://pointsav.com"
  },
  "woodfine-projects": {
    "wordmark-primary": "Woodfine",
    "wordmark-sub": "Projects Register",
    "brand-mark-glyph": "WP",
    "external-link-label": "woodfinegroup.com",
    "external-link-href": "https://woodfinegroup.com"
  },
  "woodfine-corporate": {
    "wordmark-primary": "Woodfine",
    "wordmark-sub": "Corporate Record",
    "brand-mark-glyph": "WC",
    "external-link-label": "woodfinegroup.com",
    "external-link-href": "https://woodfinegroup.com"
  }
}
```

### style.css changes

Treat the current `style.css` as superseded — the platform-document `<style>` block (lines 13–869 of proto-platform-document.html) becomes the new base. Apply these deltas:

1. **Token block (`:root`).** Adopt the proto-platform-document token set verbatim with the additions above (`--header-h: 152px`, `--utility-h`, `--brand-h`, `--nav-h`). Drop `--navy` aliases that shadow the dtcg names; emit CSS custom properties directly from `dtcg-bundle.json` token names.

2. **`.site-header` → three rows.** Remove `height: var(--header-h)` and `grid-template-columns: auto 1fr auto` from `.site-header__inner`. Replace with three stacked children:
   - `.utility-row` — `height: var(--utility-h)`; `background: var(--bg-subtle)`; `border-bottom: 1px solid var(--border)`; flex row, justify-content: space-between. Left: language toggle (`EN | ES`), Disclaimer, Contact. Right: appearance toggle (existing `.theme-toggle`), auth state (`Sign in` for anon, `<username> ▾` for authed).
   - `.brand-row` — `height: var(--brand-h)`; grid `auto 1fr auto`. Left: `.brand` (mark + wordmark, same as platform-document). Centre: `.search`. Right: GitHub icon + `<a class="brand-row__site">pointsav.com</a>`.
   - `.nav-row` — `height: var(--nav-h)`; `border-top: 1px solid var(--border)`; sticky top:0. Inside: `.nav-row__list` (the 12 category links in Oswald 12.5px uppercase letter-spaced 0.14em, `color: var(--fg-2)`, `padding: 0 14px`, active state `color: var(--navy)` with 2px navy underline `box-shadow: inset 0 -2px 0 var(--navy)`).
   The whole header stays sticky as a unit until the `.nav-row` reaches the top of viewport; on scroll past the brand row, only `.nav-row` remains sticky (apply `position: sticky; top: 0` only to `.nav-row`, NOT to `.site-header`).

3. **`.brand__wordmark`.** Render the wordmark as two stacked spans, never as one all-caps run. Set `text-transform: none` explicitly so no inherited rule can convert "PointSav" to "POINTSAV". Set `font-family: var(--font-display)`, `font-weight: 600`, `font-size: 18px`, `letter-spacing: 0.01em`. Sub-line `.brand__sub` is "Documentation Wiki" in Nunito Sans 11px uppercase letter-spaced 0.14em — uppercase is applied only to the sub-line, never to the primary wordmark.

4. **`.nav-row__list` items.** Drop the 12-item horizontal flex onto a single row on desktop with `overflow-x: auto` and a `mask-image` fade on the right edge for items 9+. At <900px collapse to a `<details>` accordion (Maud renders this as `details summary` block — see server.rs section).

5. **`.ledger-stripe` (new).** Add the institutional-register's stripe between hero and category grid:
   ```
   .ledger-stripe { background: var(--navy); color: #fff; padding: 22px 0; margin: 0 -28px 56px; }
   .ledger-stripe__inner { display: grid; grid-template-columns: repeat(4, 1fr); gap: 24px; max-width: var(--content-max); margin: 0 auto; padding: 0 28px; }
   .ledger-fact .num { font-family: var(--font-display); font-weight: 600; font-size: 32px; line-height: 1; font-variant-numeric: tabular-nums; }
   .ledger-fact .label { font-family: var(--font-display); font-size: 10px; letter-spacing: 0.22em; text-transform: uppercase; color: rgba(255,255,255,0.7); margin-top: 8px; }
   .ledger-fact + .ledger-fact { border-left: 1px solid rgba(255,255,255,0.18); padding-left: 24px; }
   ```
   At <760px collapse to 2×2 grid; remove the left border on column-1 items.

6. **`.article__meta` rewrite.** Convert from flex pills to `<dl>` with grid:
   ```
   .article__meta { display: grid; grid-template-columns: repeat(5, auto); gap: 0; border-top: 1px solid var(--rule); border-bottom: 1px solid var(--rule); padding: 16px 0; margin-bottom: 36px; }
   .article__meta > div { padding: 0 22px; border-right: 1px solid var(--rule); }
   .article__meta > div:first-child { padding-left: 0; }
   .article__meta > div:last-child { border-right: 0; padding-right: 0; }
   .article__meta dt { font-family: var(--font-display); font-size: 10px; letter-spacing: 0.18em; text-transform: uppercase; color: var(--fg-3); margin-bottom: 4px; }
   .article__meta dd { margin: 0; font-family: var(--font-body); font-weight: 600; font-size: 13.5px; color: var(--fg-1); font-variant-numeric: tabular-nums; }
   ```
   At <760px: `grid-template-columns: 1fr 1fr; gap: 12px 0;` and remove right borders, replace with bottom hairlines.

7. **`.prose strong` and Markdown bold.** Add an explicit rule `.prose strong { font-weight: 700; color: var(--fg-1); }`. The C+ defect "asterisks show as literal characters" is a renderer bug, not a CSS bug — confirm in server.rs that the Markdown pipeline emits `<strong>` (not `**...**` literal). See "Mechanical defect list" below.

8. **Talk tab / edit pencil — anon gating.** Add CSS rule `[data-auth="anon"] .article__tab--talk, [data-auth="anon"] .article__edit-pencil { display: none !important; }`. The `data-auth` attribute is set on `<html>` by server.rs based on session state; default to `anon` when no session cookie is present.

9. **Sidebar — instance switcher promoted.** The existing `.sidebar` "Instances" group from platform-document is correct; promote it to the top of the sidebar above `Browse`, and render the current instance with `is-active` state so a reader can see which of the three platforms they are on.

10. **Dark mode.** Adopt platform-document's `html[data-theme="dark"]` block verbatim. Verify pair contrast: `--link: #7AAEEA` on `--bg: #0B1220` should compute to ≥ 7:1 (large), ≥ 4.5:1 (body); `--fg-2: #C9D1DC` on `--bg-elevated: #111A2C` should compute to ≥ 7:1. Both pairs already passed in the prototype.

11. **Reduce featured-callout drop-cap risk.** Do NOT adopt any `::first-letter` drop-cap pattern from institutional-register or editorial-standard. The lede stays as a normal paragraph in Roboto Slab 17px.

12. **Remove Wikipedia-skin holdovers.** Audit current `style.css` for `.wiki-*`, `.mw-*`, `.vector-*` class prefixes — delete. The new shell uses BEM with `.site-header__inner`, `.brand__mark`, `.article__title`, etc.

### server.rs changes

The Maud templates need restructuring around the three-row header and the new data-attribute scheme.

1. **`<html>` element.** Add two data attributes set from request context:
   ```rust
   html lang=(lang_code) data-theme=(theme) data-instance=(instance) data-auth=(auth_state) {
       // ...
   }
   ```
   - `lang_code`: `"en"` or `"es"`.
   - `theme`: `"light"` (default) or `"dark"` (read from `Cookie: pointsav-theme=dark`).
   - `instance`: one of `"pointsav"`, `"woodfine-projects"`, `"woodfine-corporate"` (read from `Host:` header or compile-time config for the current binary).
   - `auth_state`: `"anon"` (no valid session cookie) or `"user"` (valid session).

2. **`<head>`.** Add the three Google Fonts `<link>` tags (preconnect + stylesheet) before the local stylesheet link. Add `<meta name="viewport" content="width=device-width, initial-scale=1">` if absent.

3. **Header — three rows.** Replace whatever single-row header markup currently renders with:
   ```rust
   header.site-header {
       div.utility-row {
           div.utility-row__inner {
               div.utility-row__left {
                   a href=(lang_alt_url) { (lang_alt_label) }  // "ES" if current is EN, "EN" if current is ES
                   a href="/disclaimer" { "Disclaimer" }
                   a href="/contact" { "Contact" }
               }
               div.utility-row__right {
                   button.theme-toggle id="themeToggle" type="button" aria-label="Toggle dark mode" {
                       // existing sun/moon SVG block
                   }
                   @if auth_state == "anon" {
                       a.utility-row__signin href="/auth/login" { "Sign in" }
                   } @else {
                       span.utility-row__user { (username) " ▾" }
                   }
               }
           }
       }
       div.brand-row {
           div.brand-row__inner {
               a.brand href="/" aria-label=(brand_aria) {
                   span.brand__mark aria-hidden="true" { (instance_glyph) }       // "PS" / "WP" / "WC"
                   span {
                       span.brand__wordmark { (instance_wordmark) }                 // "PointSav" / "Woodfine"
                       span.brand__sub { (instance_subline) }                       // "Documentation Wiki"
                   }
               }
               form.search role="search" action="/search" method="get" {
                   // search input + kbd hint
               }
               div.brand-row__external {
                   a href=(instance_external_href) { (instance_external_label) }   // "pointsav.com"
                   a href=(github_org_url) aria-label="GitHub" { // GitHub SVG }
               }
           }
       }
       nav.nav-row aria-label="Primary" {
           ul.nav-row__list {
               @for cat in &categories {
                   li {
                       a href=(cat.href) class=(if cat.is_active { "is-active" } else { "" }) { (cat.label) }
                   }
               }
           }
       }
   }
   ```

4. **Sidebar (article pages only).** Keep the platform-document `.sidebar` structure. Add the Instances group at top, then Browse, then Categories (12 items with counts), then Reference. Anon readers see all sidebar links unchanged.

5. **Article meta render.** Wrap the meta fields in a `<dl>`:
   ```rust
   dl.article__meta {
       div { dt { "Category" } dd { (article.category) } }
       div { dt { "Status" } dd { (article.status) } }
       div { dt { "Last reviewed" } dd { (article.last_reviewed) } }
       div { dt { "Cites" } dd { (article.cites_summary) } }
       div { dt { "Language" } dd { (article.lang_pair) } }   // "EN · ES"
   }
   ```

6. **Talk tab — auth gate at template level, not just CSS.** Even with the `[data-auth="anon"]` CSS rule, do not emit the Talk tab DOM at all when `auth_state == "anon"`:
   ```rust
   @if auth_state != "anon" {
       a.article__tab--talk href=(talk_url) { "Discussion" }
   }
   ```
   Same treatment for `.article__edit-pencil` on section headings. Belt and braces — CSS hide is the fallback, server-side omit is the primary defence.

7. **Ledger stripe.** New partial between hero and category grid:
   ```rust
   section.ledger-stripe aria-label="Library statistics" {
       div.ledger-stripe__inner {
           div.ledger-fact { div.num { (article_count) } div.label { "Articles" } }
           div.ledger-fact { div.num { (category_count) } div.label { "Categories" } }
           div.ledger-fact { div.num { (instance_count) } div.label { "Live Instances" } }
           div.ledger-fact { div.num { (last_updated_relative) } div.label { "Last Updated" } }
       }
   }
   ```
   Numbers come from a `WikiStats` struct populated at request time (or cached for 60s).

8. **Footer.** Replace whatever footer template currently exists with the verbatim canonical text. Use the platform-document footer structure (two-row: cities + links on row-1, legal paragraph on row-2). The trademark paragraph must match the canonical string character-for-character.

9. **Theme-toggle script.** Adopt the platform-document inline `<script>` (lines 1300–1330). It is self-contained, no framework, persists to `localStorage['pointsav-theme']`, falls back to `prefers-color-scheme`. Embed via Maud `script { (PreEscaped(THEME_TOGGLE_JS)) }` with `const THEME_TOGGLE_JS: &str` defined at module top.

### Wordmark fix

The C+ defect "POINT-SAV DIGITAL SYSTEMS" stems from one of three causes — verify which and fix all three:

1. **CSS `text-transform: uppercase` applied too broadly.** Search current `style.css` for `text-transform: uppercase` on any selector that targets `.brand`, `.wordmark`, or the document `<h1>`. Remove from the brand wordmark; keep on the `.brand__sub` subline only.

2. **Manual hyphenation in markup.** If the current `server.rs` renders the wordmark as `"POINTSAV DIGITAL SYSTEMS"` or `"POINT-SAV DIGITAL SYSTEMS"` literally, replace with the two-part wordmark structure shown above. The primary word is `"PointSav"` — title case, single token, no space, no hyphen.

3. **Browser hyphenation.** Add `hyphens: none; word-break: keep-all;` to `.brand__wordmark` as belt-and-braces.

The wordmark approach is **text, not SVG.** A text wordmark scales with the font system, respects `data-instance` parameterisation, and degrades to a system font if Oswald fails to load. SVG would freeze the wordmark to one weight and complicate the three-instance switching.

For visual lockup parity with the prototype, the brand block is:
```
[PS]  PointSav
      DOCUMENTATION WIKI
```
where `[PS]` is the rounded-rect mark (30×30, linear-gradient navy-deep → navy, white text in Oswald 14px 600 letter-spaced 0.04em), `PointSav` is the wordmark span (Oswald 18px 600 letter-spaced 0.01em, ink-1), and `DOCUMENTATION WIKI` is the subline (Nunito Sans 11px 600 letter-spaced 0.14em uppercase, ink-3).

### Navigation fix

The C+ defect is that nav-row contained only Disclaimer / Contact / pointsav.com / GitHub — utility links, not content routes. Fix:

- **Utility-row** (top, 32px) carries: language toggle, Disclaimer, Contact (left); theme toggle, Sign in / user menu (right). These are the items previously in the nav.
- **Brand-row** (middle, 72px) carries: brand, search, pointsav.com link, GitHub icon (right side).
- **Nav-row** (bottom, 48px, sticky) carries the **12 category routes**:
  Applications, Architecture, Design System, Governance, Infrastructure, Patterns, Reference, Services, Substrate, Systems, Guides, Company.

Each category link routes to `/<category-slug>/` (e.g. `/architecture/`) which renders the category landing page (list of all articles in that category). Category landing pages reuse the article-list template from the home page's "Recently updated" block but scoped to one category.

The active-state is computed server-side: a request to `/architecture/economic-model` resolves the category from the article's frontmatter and emits `class="is-active"` on the Architecture nav-row item. On the home page, no item is active.

Sidebar (visible on article pages only) is the secondary nav and lists categories with article counts plus the Browse and Reference subgroups. Sidebar and nav-row both link to the same category routes — sidebar is "where can I go", nav-row is "where am I and where else can I jump".

### Mechanical defect list

Defects from the C+ review that this redesign must fix in the same pass:

1. **Webfonts not loading.** Add the three Google Fonts `<link>` tags as specified above. Verify in DevTools Network panel that all three font families load with HTTP 200. Verify `getComputedStyle(document.querySelector('.brand__wordmark')).fontFamily` includes `Oswald` not the fallback.

2. **Wordmark hyphenation.** Fixed by the wordmark approach above. Test: at all viewport widths (320, 480, 768, 1024, 1440), the wordmark renders as "PointSav" in title case, never hyphenated, never all-caps.

3. **Bold Markdown broken.** Open the Markdown rendering function in `server.rs` (likely a `pulldown_cmark::Parser` pipeline or a custom one). Confirm `Tag::Strong` is mapped to `<strong>` and `Tag::Emphasis` to `<em>`. If the pipeline is custom and missing those cases, add them. Verify with a fixture: `**bold**` → `<strong>bold</strong>`, never literal asterisks. Add a `cargo test` case for this.

4. **Talk/Discussion tab visible to anon.** Server-side: do not render the Talk tab markup when `auth_state == "anon"`. CSS fallback: `[data-auth="anon"] .article__tab--talk { display: none !important; }`. Test: visit the article view with no session cookie — Talk tab must not appear in DOM (right-click → Inspect, search for "Discussion" → no match).

5. **Edit pencil on section headings visible to anon.** Same treatment — omit the markup server-side when anon, hide via CSS as fallback.

6. **Hero H1 weak.** New hero uses Oswald 400-weight at clamp(34px, 4.2vw, 52px) line-height 1.08 with a 12px uppercase eyebrow above. The H1 copy is the strong-statement form: "The engineering library behind every disclosure." (instance: pointsav) — replace with `data-instance`-driven copy for the other two instances ("The project register of Woodfine Management Corporation." / "The corporate record of Woodfine Capital Projects.").

7. **Wikipedia-skin reading.** The new shell is sidebar + content + TOC three-pane with restrained navy, Oswald display, Roboto Slab lede — closer to Stripe Docs or Linear's changelog than to MediaWiki Vector. Verify with a side-by-side comparison screenshot at the verification step.

## Verification checklist

1. `cargo build` — clean.
2. `cargo test` — all tests pass, including the new Markdown bold-rendering fixture.
3. `cargo clippy -- -D warnings` — no warnings.
4. `cargo fmt --check` — clean.
5. Run the debug server: `~/Foundry/clones/project-knowledge/app-mediakit-knowledge/target/debug/app-mediakit-knowledge --port 8090 --host 127.0.0.1` (or the project's canonical debug command — confirm against the archive's `bin/` or `justfile`).
6. `curl -s http://127.0.0.1:8090/ | grep -c 'fonts.googleapis.com'` — must return ≥ 1.
7. `curl -s http://127.0.0.1:8090/ | grep -c 'PointSav'` — must return ≥ 1; `grep -c 'POINT-SAV'` must return 0.
8. `curl -s http://127.0.0.1:8090/architecture/economic-model | grep -ci 'discussion'` — with no auth cookie, must return 0.
9. Open `http://127.0.0.1:8090/` in a browser:
   - Three-row header renders: utility (32px), brand (72px), nav (48px).
   - Wordmark reads "PointSav" + "DOCUMENTATION WIKI" subline; no hyphen, no all-caps on the primary wordmark.
   - All 12 categories visible in nav-row.
   - Hero renders with eyebrow + 52px Oswald H1 + slab-serif lede + metric strip.
   - Ledger stripe (navy bg, 4 facts) sits between hero and category grid.
   - Featured callout has navy left-stripe + gold dot.
   - Category grid renders 12 cards in a 3- or 4-column layout (responsive).
   - Recently updated list renders with category pill + tabular date.
   - Footer renders canonical text verbatim, cities line at top, legal paragraph below.
10. Toggle theme — page enters dark mode; navy stays brand-recognisable, links become `#7AAEEA`, paper inverts to `#0B1220`. Reload — preference persists.
11. Visit `/architecture/economic-model`:
    - Three-pane layout (sidebar / article / TOC).
    - Article meta renders as 5-column `<dl>` strip with vertical rules.
    - Lede is Roboto Slab 19px.
    - Bold Markdown renders as `<strong>` not literal `**`.
    - No Talk tab in DOM (anon).
    - No edit pencil on h2 headings (anon).
    - TOC is numbered (01–06), sticky, with active-section highlight.
12. Set `data-auth="user"` via DevTools — Talk tab appears (or fail if server-omitted, then test by logging in via the dev session endpoint), edit pencils appear on `:hover` of h2.
13. Resize to 480px:
    - Header collapses: utility row stays, brand row stacks (or compresses), nav-row becomes scrollable horizontal strip or `<details>` accordion.
    - Sidebar hidden.
    - TOC hidden.
    - Article body reads comfortably at 17–18px body size.
    - Footer cities line wraps, legal paragraph stays readable.
14. WCAG AA contrast pairs to verify (using DevTools or `npx wcag-contrast-checker`):
    - `#111827` on `#F7F9FA` → 16.4:1 (AAA). PASS.
    - `#6B7280` on `#F7F9FA` → 4.8:1 (AA body). PASS.
    - `#164679` on `#F7F9FA` → 9.7:1 (AAA). PASS.
    - `#FFFFFF` on `#164679` (ledger stripe) → 8.4:1 (AAA). PASS.
    - `#C7A961` on `#F7F9FA` (accent-gold, ornament use only) → 2.1:1 — FAIL for text; verify it is only used on decorative dots/borders, never on text. The featured-callout eyebrow uses `--accent` (#C7A961) at 11px — this is borderline; downgrade to `--navy` (#164679) for the eyebrow text and keep gold for the dot only.
    - Dark mode: `#7AAEEA` on `#0B1220` → 8.9:1 (AAA). PASS.
    - Dark mode: `#C9D1DC` on `#111A2C` → 11.2:1 (AAA). PASS.
15. Run the existing axe-core or pa11y test suite if present; otherwise add a baseline.
16. Take three screenshots — light home, light article, dark article — and attach to the inbox message back to project-design.
