# RD.2 — Pre-Wiki BIM Shell Redesign: Git Archaeology

**Scope:** `app-privategit-bim/src/render/shell.rs` and `app-privategit-bim/src/assets/bim-layout.css` (+ `tokens.css`, since dark-mode color values live there), diffed between baseline commit `76bf3c6e` and `HEAD`. Earlier-era comparison at `e222418b`.

## 1. Commit timeline (render/ and assets/ only)

```
7ae34ad1  feat(app-privategit-bim): dark-mode infrastructure                          [step 1]
2ef545f5  feat(app-privategit-bim): header + utility-bar rebuild, dark-mode fixes      [step 2]
64c108a0  fix(app-privategit-bim): disambiguate duplicate 'Key Plans' sidebar label
be407bee  feat(app-privategit-bim): sidebar + card-grid restyle, dark-mode surfaces    [step 3]
bc3bb9dc  feat(app-privategit-bim): footer rebuild + Important Information band        [step 4]
472eb451  feat(app-privategit-bim): server-side search                                 [step 5]
19979562  fix(app-privategit-bim): mobile header overflow hid the dark-mode toggle
68e3406f  feat(app-privategit-bim): redo Important Information band against Command's actual spec
```

`76bf3c6e` is the last commit before `7ae34ad1` — confirmed as the correct pre-redesign baseline.

## 2. Earlier era for depth: `e222418b` → `76bf3c6e`

`e222418b` ("match home.woodfinegroup.com's brand exactly") already has the same single fixed `.bim-topbar` shape as the `76bf3c6e` baseline, but simpler:

```html
<!-- e222418b -->
<header class="bim-topbar">
  <button class="bim-topbar__toggle" ...>&#9776;</button>
  <a href="/" class="bim-topbar__brand">Woodfine</a>   <!-- plain text, no SVG -->
  <span class="bim-topbar__sep" aria-hidden="true"></span>
  <span class="bim-topbar__label">BIM Object Library</span>
  <div class="bim-header-spacer"></div>
  <span class="bim-topbar__meta">app-privategit-bim</span>   <!-- dev-debug label -->
</header>
```

By `76bf3c6e`, the brand link had gained the full inline Woodfine wordmark SVG (`.bim-topbar__logo`), and the `.bim-topbar__meta` dev-debug label (`app-privategit-bim`) is gone, plus the footer gained a fourth bullet (`AGPL-3.0-or-later` license line, source-code link) not present in `e222418b`'s three-item footer list. Otherwise structurally identical: one fixed dark-navy topbar, hamburger toggle, brand+sep+label, spacer, sidebar+main two-column shell, three-column footer, no search, no theme toggle, no utility bar, no disclosure band.

## 3. Baseline (`76bf3c6e`) vs HEAD — `shell.rs` structural diff

### Baseline `<body>` structure (single dark topbar, no search, no theme toggle)

```html
<body class="bim-body">
  <header class="bim-topbar">
    <button class="bim-topbar__toggle" aria-label="Toggle menu" aria-expanded="false" type="button">&#9776;</button>
    <a href="/" class="bim-topbar__brand" aria-label="Woodfine">
      <svg class="bim-topbar__logo" ...>...</svg>
    </a>
    <span class="bim-topbar__sep" aria-hidden="true"></span>
    <span class="bim-topbar__label">BIM Object Library</span>
    <div class="bim-header-spacer"></div>
  </header>
  <div class="bim-shell">
    <nav class="bim-side-nav" aria-label="BIM sidebar">{sidebar}</nav>
    <main id="bim-main-content" class="bim-main">{content}</main>
  </div>
  <footer class="bim-footer">
    <!-- 3 columns: "Woodfine BIM Object Library" / "Machine-readable surface" / "Platform" -->
    <!-- .bim-footer__base: single copyright line, no badges, no cities -->
  </footer>
</body>
```

### HEAD `<body>` structure (utility bar + light header + search + theme toggle + disclosure band + rebuilt footer)

```html
<body class="bim-body">
  <div class="bim-utility">
    <div class="bim-utility__inner">
      <a href="https://woodfinegroup.com" class="bim-utility__home">Woodfine Capital Projects</a>
      <nav class="bim-utility__nav" aria-label="Woodfine network">
        <a class="bim-utility__link" href="https://corporate.woodfinegroup.com" ...>Corporate</a>
        <a class="bim-utility__link" href="https://projects.woodfinegroup.com" ...>Projects</a>
        <a class="bim-utility__link" href="https://github.com/pointsav" ...>GitHub</a>
      </nav>
    </div>
  </div>
  <header class="bim-header">
    <div class="bim-header__inner">
      <button class="bim-topbar__toggle" ...>&#9776;</button>
      <a href="/" class="bim-header__brand" aria-label="Woodfine — BIM Object Library" data-path="/">
        {wordmark}  <!-- same SVG, now factored into wordmark_svg() helper, navy-on-white via currentColor -->
        <span class="bim-header__lockup">
          <span class="bim-header__word">Woodfine</span>          <!-- display:none, kept for a11y/SEO text -->
          <span class="bim-header__subtitle">BIM Object Library</span>
        </span>
      </a>
      <form class="bim-search" action="/search" method="get" role="search">
        <label class="bim-search__label" for="bim-search-input">Search BIM Objects</label>
        <div class="bim-search__form">
          <svg class="bim-search__icon" ...>...</svg>
          <input id="bim-search-input" class="bim-search__input" type="search" name="q"
                 placeholder="Search categories, entities, research&hellip;" autocomplete="off">
          <button class="bim-search__button" type="submit"><span class="bim-search__button-label">Search</span></button>
        </div>
      </form>
      {theme_toggle}  <!-- sun/moon SVG button, omitted on /edit/* routes -->
    </div>
  </header>
  <div class="bim-shell">
    <nav class="bim-side-nav" aria-label="BIM sidebar">{sidebar}</nav>
    <main id="bim-main-content" class="bim-main">{content}</main>
  </div>
  <section class="bim-disclosure" aria-label="Important information">
    <details class="bim-disclosure__details">
      <summary class="bim-disclosure__summary">Important Information</summary>
      <div class="bim-disclosure__body">
        {important_information}   <!-- read from Git-owned markdown, not hardcoded -->
        <p class="bim-disclosure__more"><a href="/disclaimers" data-path="/disclaimers">Read the full disclaimer &rarr;</a></p>
      </div>
    </details>
  </section>
  <footer class="bim-footer">
    <!-- 3 columns: "Woodfine BIM Object Library" (now also carries AGPL+source, moved from old "Platform" column)
         / "Machine-readable surface" (unchanged) / "Network" (new — replaces "Platform") -->
    <div class="bim-footer__base">
      <div class="bim-footer__base-row">
        <div class="bim-footer__cities"><span>Vancouver</span><span class="bim-footer__cities-sep">|</span><span>New York</span></div>
        <div class="bim-footer__badges">
          <span class="bim-badge">...Powered by PointSav Digital Systems...</span>
          <a class="bim-badge" href="https://www.apache.org/licenses/LICENSE-2.0" ...>BIM data licensed Apache-2.0</a>
          <a class="bim-badge bim-badge--license" href="https://creativecommons.org/licenses/by-nd/4.0/" ...>
            <span class="bim-badge__cc"><img src="/static/cc.svg">...cc-by.svg...cc-nd.svg...</span>
            Editorial content CC BY-ND 4.0
          </a>
        </div>
      </div>
      <p>Copyright ...</p>
      <p class="bim-footer__disclaimer">Provided for reference and coordination only — ... See <a href="/disclaimers">Important Information</a>.</p>
      <p class="bim-footer__trademark">...</p>
    </div>
  </footer>
</body>
```

### Structural summary table

| Element | Baseline (`76bf3c6e`) | HEAD | Status |
|---|---|---|---|
| `.bim-topbar` (dark navy, fixed, single bar) | present | **gone entirely** | removed |
| `.bim-topbar__brand`, `__logo`, `__sep`, `__label` | present | classes gone (superseded) | removed |
| `.bim-header-spacer` | present | gone | removed |
| `.bim-utility` / `.bim-utility__inner` / `__home` / `__nav` / `__link` | absent | present | **new** |
| `.bim-header` / `.bim-header__inner` / `__brand` / `__logo` / `__lockup` / `__word` / `__subtitle` | absent | present | **new** (replaces topbar) |
| `wordmark_svg()` helper fn | absent (inline SVG duplicated at call site) | present, extracted | refactor |
| `.bim-search` + `__label`/`__form`/`__icon`/`__input`/`__button`/`__button-label` | absent | present | **new** |
| `.bim-theme-toggle` + sun/moon SVGs | absent | present (suppressed on `/edit/*`) | **new** |
| Pre-paint inline `<script>` reading `localStorage['bim-theme']` | absent | present in `<head>` (skipped on `/edit/*`) | **new** |
| `html_theme_attr` (`data-theme="light"` hardcode for `/edit/*`) | absent | present | **new** |
| `.bim-topbar__toggle` (hamburger) | present | present (reused as-is under new header) | unchanged |
| `.bim-shell` / `.bim-side-nav` / `.bim-main` | present | present | unchanged |
| `.bim-disclosure` band (`<details>/<summary>`) | absent | present | **new** |
| `content::render_important_information()` call | absent | present | **new** |
| Footer 3rd column: "Platform" (AGPL/source/Powered-by/pointsav.com link) | present | **gone**, replaced by "Network" column (home/Corporate/Projects/GitHub) | removed → replaced |
| Footer 1st column gains AGPL license line + source link | absent | present (moved from old 3rd column) | moved |
| `.bim-footer__base-row`, `__cities`, `__cities-sep` | absent | present | **new** |
| `.bim-badge`, `__glyph`, `__text`, `__lead`, `__name`, `--license`, `__cc` | absent | present | **new** |
| `.bim-footer__disclaimer` (persistent one-line disclaimer) | absent | present | **new** |
| `/disclaimers` route link (`data-path`) | absent | present (footer + disclosure band) | **new** |
| `full_title`, `esc()`, footer stat line (`{tc}`/`{comp}`/`{rc}`) | present | present | unchanged |

## 4. Baseline vs HEAD — `bim-layout.css` diff

File grew from **688 lines → 1114 lines** (+426).

### Color palette
- Baseline hardcodes hex colors throughout (`#111827`, `#374151`, `#6B7280`, `#164679`, `#E6E7E8`, `#9CA3AF`) directly in component rules.
- HEAD converts nearly all of these to `var(--bim-fg, #111827)`, `var(--bim-fg-secondary, #374151)`, `var(--bim-fg-muted, #6B7280)`, `var(--bim-accent, #164679)`, `var(--bim-border, #E6E7E8)`, `var(--bim-border-strong, #9CA3AF)` — token-ized so they can flip in `:root[data-theme="dark"]`. Example (`.bim-category-card-name`):
  ```css
  /* baseline */
  .bim-category-card-name { font-size: 1rem; font-weight: 600; margin-bottom: 0.375rem; color: #164679; }
  /* HEAD */
  .bim-category-card-name { font-size: 0.9375rem; font-weight: 600; color: var(--bim-fg, #111827); }
  ```
- New dark-mode variable block added to `tokens.css` (not `bim-layout.css` itself):
  ```css
  :root[data-theme="dark"] {
    --bim-accent:        #6FA8DC;  /* lightened for AA text contrast on dark surfaces */
    --bim-accent-hover:  #8CBAE3;
    --bim-accent-active: #ABCCEA;
    --bim-accent-subtle: rgba(111, 168, 220, 0.16);
    --bim-fg:            #F3F4F6;
    --bim-fg-secondary:  #D1D5DB;
    --bim-fg-muted:      #9CA3AF;
    --bim-fg-faint:      #6B7280;
    --bim-border:        #2D333F;
    --bim-border-subtle: rgba(255, 255, 255, 0.06);
    --bim-border-strong: #4B5563;
    --bim-bg-page:       #0F1218;
    --bim-bg-sidebar:    #171B24;
    --bim-bg-subtle:     #171B24;
    --bim-bg-tertiary:   #1F2430;
  }
  ```
  Note: `--bim-topbar-bg` (`#164679`) was deliberately un-derived from `--bim-accent` (hardcoded hex instead) so the topbar/footer dark chrome doesn't shift when `--bim-accent` lightens for dark-mode text contrast — topbar/footer are "intentionally NOT overridden" per the `7ae34ad1` commit message, since they're already permanently-dark in both themes.
- New layout-only vars: `--bim-bg-page`, `--bim-utility-height` (32px), `--bim-header-height` (64px), `--bim-header-stack` (calc of the two) — replaces the old single `--bim-topbar-height` (48px) as the fixed-offset reference for `.bim-shell`/`.bim-side-nav`.

### Typography
- No font-family swap (still `var(--bim-font-sans, 'IBM Plex Sans')`, `--bim-font-display`, `--bim-font-mono` throughout) — the commit `2ef545f5` explicitly says BIM keeps its own Oswald/Nunito Sans/navy system rather than porting the wiki's Inter + Source Serif 4 pairing.
- `.bim-topbar__label` (mono, uppercase, letter-spaced) is gone; replaced by `.bim-header__subtitle` (mono, `0.6875rem`, `letter-spacing: 0.06em`, uppercase) inside the new brand lockup.
- `.bim-category-card-name` font-size shrinks `1rem` → `0.9375rem`; `.bim-category-card-desc` rule removed entirely (description line dropped from home/category cards per `be407bee`).

### New component classes (never existed in baseline)
- **Utility bar:** `.bim-utility`, `.bim-utility__inner`, `.bim-utility__home`, `.bim-utility__nav`, `.bim-utility__link`
- **Header:** `.bim-header`, `.bim-header__inner`, `.bim-header__brand`, `.bim-header__logo`, `.bim-header__lockup`, `.bim-header__word`, `.bim-header__subtitle`
- **Search:** `.bim-search`, `.bim-search__label`, `.bim-search__form`, `.bim-search__icon`, `.bim-search__input`, `.bim-search__button`, `.bim-search__button-label`
- **Theme toggle:** `.bim-theme-toggle`, `.bim-theme-toggle__sun`, `.bim-theme-toggle__moon` (+ `:root[data-theme="dark"]` visibility-swap rules)
- **Search results page:** `.bim-search-page__meta`, `.bim-search-results`, `.bim-search-result`, `.bim-search-result__kind`, `.bim-search-result__title`, `.bim-search-result__snippet`, `.bim-search-result__snippet mark`
- **Sidebar accent treatment:** `.bim-side-nav .bim-nav-link` (border-left accent instead of solid-fill hover/active), scoped specifically to sidebar so breadcrumbs/cards/research-index keep their own treatment
- **Footer/badges:** `.bim-footer__base-row`, `.bim-footer__cities`, `.bim-footer__cities-sep`, `.bim-footer__badges`, `.bim-badge`, `.bim-badge__glyph`, `.bim-badge__text`, `.bim-badge__lead`, `.bim-badge__name`, `.bim-badge--license`, `.bim-badge__cc`, `.bim-cc-icon`, `.bim-footer__disclaimer`
- **Disclosure band:** `.bim-disclosure`, `.bim-disclosure__details`, `.bim-disclosure__summary` (custom `▸`/`▾` chevron via `::before`, `list-style: none`, hides native `::-webkit-details-marker`), `.bim-disclosure__body`, `.bim-disclosure__more`
- **Mobile:** `.bim-nav-group--mobile-only` (utility-bar network links relocate into the sidebar drawer below 768px)

### Removed classes
`.bim-header-spacer`, `.bim-topbar` (block renamed/rebuilt as `.bim-header`), `.bim-topbar__brand`, `.bim-topbar__logo`, `.bim-topbar__sep`, `.bim-topbar__label`, `.bim-topbar__meta` (was already gone by `76bf3c6e`, confirmed absent), `.bim-category-card-desc`.

### Category card restyle (`be407bee`)
```css
/* baseline */
.bim-category-card { display: block; padding: 1.5rem; border: 1px solid #E6E7E8; ... }
.bim-category-card:hover { border-color: #164679; box-shadow: 0 2px 8px rgba(22, 70, 121, 0.12); }

/* HEAD — accent-left-border convention matching the wiki's .k-cat-card */
.bim-category-card {
  display: flex; flex-direction: column; gap: 0.25rem;
  padding: 1rem 1.25rem;
  border: 1px solid var(--bim-border, #E6E7E8);
  border-left: 3px solid var(--bim-accent, #164679);
  border-radius: var(--bim-radius-md, 4px);
  background: var(--bim-bg-page, #fff);
  ...
}
```

## 5. Commit messages explaining WHY (direct quotes)

**`2ef545f5` — the commit citing "matching the structural pattern of the live wiki instances":**
> "Second step of the shell redesign — replaces the solid-navy app-shell topbar with a light header + utility bar, **matching the structural pattern of the live wiki instances** (app-mediakit-knowledge-2 at corporate/projects.woodfinegroup.com) while keeping BIM's own Oswald/Nunito Sans/navy brand system (the wiki uses a different font pairing, Inter + Source Serif 4, that itself doesn't match home.woodfinegroup.com — not porting that mismatch into BIM too)."

**`7ae34ad1` — the trigger for the whole redesign:**
> "First step of a full shell redesign, prompted by a direct operator comparison against home.woodfinegroup.com and the live Woodfine/PointSav wiki instances (app-mediakit-knowledge-2) — both have genuine dark-mode support and BIM had none."
>
> Also explains the deliberate topbar/footer exclusion: "Topbar and footer are intentionally NOT overridden — they're already permanently-dark chrome in both themes. `--bim-topbar-bg` is now a hardcoded hex rather than derived from `--bim-accent`, so `--bim-accent` is free to lighten in dark mode (for text contrast) without dragging the topbar's fill along with it."

**`be407bee` — sidebar/card restyle:**
> "card.rs: home/category-index cards drop their description line, keeping name + entity count only — **matches the wiki's denser `.k-cat-card` pattern** (the description is still one click away on the category page itself)."
> "`.bim-category-card` restyled to **the wiki's accent-left-border convention** (3px solid `var(--bim-accent)` left border, hover raises border/shadow) instead of a plain uniform box border."

**`bc3bb9dc` — footer rebuild:**
> "Fourth step of the shell redesign — **matches the wiki's footer/disclosure convention** (BROWSE/THIS-SITE/NETWORK columns, cities line, badge chips, collapsible disclosure band) rather than the previous flat 3-column license dump."
> "Footer base row gains a 'Vancouver | New York' cities line (**the same real corporate fact the wiki instances use**) and two badge chips: 'Powered by PointSav Digital Systems' (inline SVG glyph, not the wiki's literal MediaKit graphic)..."

**`472eb451` — search:**
> "Fifth and final step of the shell redesign — wires up the search bar added in the header rebuild (previously a dead form pointing nowhere)."
> Notably declines to add a real search-index crate: "the whole corpus is ~150-200 entities across 20 categories plus 3 research articles ... a linear scan for search is strictly cheaper than what's already shipping, not a new architectural pattern."

**`68e3406f` — the post-round-1 correction, redoing the disclosure band against the real spec:**
> "The disclosure band built earlier this session was ad-hoc — hardcoded directly in shell.rs, without having seen inbox message command-20260702-important-information-footer-structure-a until the shutdown sweep. That message describes a real, already-researched pattern (studied home.*, Apollo, Apollo Academy, BCSC/EDGAR/SEDAR) and a working reference implementation in the live wiki instances (app-mediakit-knowledge). This redoes it against that actual spec instead of the improvised version."
> "New persistent one-line footer disclaimer, always visible regardless of the collapsible band's open/closed state ('so a collapsed band never screenshots bare' — verbatim rationale from the spec message)."
> "Real official CC BY-ND 4.0 marks (cc.svg/cc-by.svg/cc-nd.svg, copied from the reference wiki engine's own assets — same icons, not reinvented)..."

**`19979562` — mobile fix, verification-driven:**
> "Found during the post-implementation verification pass: at 390px the header row ... summed wider than the viewport, pushing the theme toggle out of the visible area entirely with no horizontal scroll available to reach it — not clipped, just gone."

## 6. Files of record

- `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/src/render/shell.rs`
- `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/src/assets/bim-layout.css`
- `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/src/assets/tokens.css`
- `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/src/assets/bim-components.css` (also touched by `be407bee`, not diffed above in detail — dark-token conversion of `.bim-tag`, `[aria-current="page"]`, `.bim-token-table`/`.bim-prop-table`, key-plan swatch colors)
- Commits: `76bf3c6e` (baseline), `e222418b` (earlier era), `7ae34ad1`, `2ef545f5`, `64c108a0`, `be407bee`, `bc3bb9dc`, `472eb451`, `19979562`, `68e3406f` (HEAD-adjacent)
