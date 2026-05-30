---
schema: foundry-draft-v1
artifact_type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
audience: design-team
bcsc_class: no-disclosure-implication
version: "1.0"
date: 2026-05-30
title: "Regional Market TOPIC — Page Template Design"
research_done_count: 4
research_suggested_count: 3
open_questions_count: 5
research_provenance: project-gis archive — derived from existing wiki_chrome() server function patterns, score-regional-markets.py output schema, and Phase 22/23 cluster data shape
research_inline: false
target_repo: pointsav-design-system
routes_to: project-design
---

# Regional Market TOPIC — Page Template Design

A Regional Market (RM) TOPIC article is a new wiki page archetype that
profiles a single human settlement which contains one or more co-location
clusters. It joins five data surfaces — the cluster registry, the AEC
overlays, the composite scoring output, civic-infrastructure rosters, and
Wikipedia summary content — into a single reader-facing article on the
knowledge wiki.

This document specifies the visual layout, the named CSS classes the
template will rely on, and the HTML skeleton the wiki platform developer
implements. It does not specify token values; tokens are chosen from the
existing design-system palette per the routing rules in
`.agent/rules/design-tokens.md`.

---

## 1. Layout overview

The page is a two-column layout sitting inside the existing
`wiki_chrome()` shell. The right column carries an infobox card, the
left column carries the article body. Both columns share the wiki TOC
sidebar that is already part of the chrome — the infobox sits to the
right of the article body, not in place of the TOC.

```
┌─────────────────────────────────────────────────┬──────────────────────┐
│  # City Name — Regional Market                  │ ┌──────────────────┐ │
│                                                 │ │   INFOBOX        │ │
│  > Lead paragraph (Wikipedia extract + context) │ │  Rank: 16 / 400  │ │
│                                                 │ │  Tier: ████ T1   │ │
│  ## Overview                                    │ │  Clusters: 4     │ │
│  [Wikipedia-sourced geographic context]         │ │  Score: 48.0     │ │
│                                                 │ │  Country: US     │ │
│  ## Co-location Profile                         │ │  Nearest metro:  │ │
│  | Cluster | Tier | Composition | Span |        │ │  OKC 250 km      │ │
│  | ...     | T1   | Hyper+Hard  | 2.1  |        │ └──────────────────┘ │
│                                                 │                      │
│  ## Civic Infrastructure                        │ [Wikipedia thumbnail │
│  Medical: [list]                                │  if available]       │
│  Academic: [list]                               │                      │
│                                                 │                      │
│  ## AEC Data                                    │                      │
│  | Layer | Value |                              │                      │
│                                                 │                      │
│  ## Composite Score                             │                      │
│  | Metric | Value | Weight |                    │                      │
│                                                 │                      │
│  ## Wikipedia References                        │                      │
└─────────────────────────────────────────────────┴──────────────────────┘
```

**Column ratio.** Left column ~ 65–70% of the article content width,
right column ~ 30–35%. The infobox column has `position: sticky; top: var(--rm-infobox-sticky-top)`
so the card stays visible as the reader scrolls the long body.

**Vertical rhythm.** Section headings (`h2`) inherit the existing wiki
heading scale; do not introduce a Regional-Market-specific heading
treatment. The infobox card uses its own internal type scale.

---

## 2. Infobox card design

The infobox is the single most-scanned element on the page. Readers
who land from a search result want rank, tier, and country answered
within one second of paint.

### Fields (in display order)

| Field | Source | Treatment |
|---|---|---|
| **Rank** | `score-regional-markets.py` `rank` field | "16 / 400" — current position over total ranked markets in the same region (NA or EU) |
| **Best Tier** | derived from cluster `tier` values | Badge: T1 = gold, T2 = silver, T3 = bronze; if mixed, shows the highest-tier present |
| **Cluster Count** | length of `cluster_ids` | Integer, e.g. "4 clusters" |
| **Composite Score** | `score` field | One decimal place, e.g. "48.0" |
| **ISO Country** | `country` field | ISO 3166-1 alpha-2 code, e.g. "US" |
| **Nearest Major Centre** | derived nearest metro + km | e.g. "Oklahoma City, 250 km SE" |
| **Centroid** | `centroid_lat`, `centroid_lon` | Decimal degrees, four places, e.g. "37.6872, −97.3301" |

### Optional Wikipedia thumbnail

If the Wikipedia summary API returned a `thumbnail.source` URL for the
city article, render a thumbnail below the field block. Cap width at
the infobox width; do not crop. Caption with the page title and a link
to the live Wikipedia article.

### CSS class names

| Class | Applies to |
|---|---|
| `.rm-infobox` | Outer card container (right column wrapper) |
| `.rm-infobox-fields` | The keyed field list inside the card |
| `.rm-infobox-field` | One label/value pair |
| `.rm-infobox-field-label` | Field label (e.g. "Rank") |
| `.rm-infobox-field-value` | Field value (e.g. "16 / 400") |
| `.rm-rank-badge` | Rank display when emphasised |
| `.rm-tier-badge` | Tier display chip — modifier classes `.rm-tier-badge--t1`, `--t2`, `--t3` carry the gold/silver/bronze treatment |
| `.rm-wiki-thumb` | Wikipedia thumbnail wrapper |
| `.rm-wiki-thumb-caption` | Caption beneath thumbnail |

### Responsive behaviour

Below the existing wiki mobile breakpoint (the chrome already defines
this; reuse the same media query), the infobox flows **above** the
article body, full-width. The sticky positioning is dropped at the
mobile breakpoint — the card scrolls with the page. Order on mobile:

1. Page title (`h1`)
2. Infobox (full width, no sticky)
3. Lead paragraph
4. Rest of article body

The TOC sidebar collapses per the existing wiki chrome rules; this
template inherits that behaviour and does not override it.

---

## 3. Co-location table

This is the article's data centrepiece. It enumerates every cluster
the Regional Market contains, with enough columns that a reader can
form a profile of the market without scrolling to the AEC or score
sections.

### Columns

| Column | Source | Notes |
|---|---|---|
| **Cluster** | Cluster descriptive name (built from anchor composition + locality) | Links to the cluster page on the GIS map at `https://gis.woodfinegroup.com/?cluster=<id>` |
| **Tier** | `tier` value, badged | T1 / T2 / T3 chip |
| **Anchor Composition** | derived from anchor brand_family set | e.g. "Hypermarket + Hardware" |
| **Span** | `span_km` | One decimal place |
| **Civic** | derived: any civic infrastructure within cluster? | "Yes" / "No" badge — "Yes" shaded |

### Row colouring

Row backgrounds carry a soft tier-coloured tint. The table is
the only place in the page where tier colour is conveyed as a row
fill (the infobox uses a single tier badge).

| Tier | Row background |
|---|---|
| T1 | Light gold — `#FFF8DC` (Cornsilk equivalent; final value pulled from design-system tokens, not hardcoded) |
| T2 | Light blue — `#EEF4FF` (token-sourced) |
| T3 | Light grey — `#F5F5F5` (token-sourced) |

Hex values shown above are reference targets; the actual rendered
colours come from the design-system token bundle. The template requests
tier tints, not literal hex.

### CSS class names

| Class | Applies to |
|---|---|
| `.rm-cluster-table` | The `<table>` element |
| `.rm-cluster-table th` | Header cells (standard table header treatment) |
| `.rm-cluster-table tbody tr.tier-t1` | T1 row (gold tint) |
| `.rm-cluster-table tbody tr.tier-t2` | T2 row (blue tint) |
| `.rm-cluster-table tbody tr.tier-t3` | T3 row (grey tint) |
| `.rm-civic-yes` | Civic-Yes badge cell |
| `.rm-civic-no` | Civic-No badge cell |

### Accessibility

Row colour is decorative — tier is also conveyed by the tier badge in
the second column, so colour-blind readers do not lose information.
The civic Yes/No is a text label, not a tick/cross glyph.

---

## 4. AEC data grid

A compact 2-column key/value grid showing the climate, ecological,
and seismic context of the Regional Market centroid.

### Fields

| Field | Source | Inclusion rule |
|---|---|---|
| ASHRAE 169 Zone | DATA-aec-clusters.csv `ashrae_zone` | US markets only |
| EU Climate Zone | DATA-aec-clusters.csv `eu_climate_zone` | EU markets only |
| Köppen-Geiger Class | DATA-aec-clusters.csv `koppen` | All markets |
| WWF Ecoregion | DATA-aec-clusters.csv `wwf_ecoregion` | All markets |
| WWF Biome | DATA-aec-clusters.csv `wwf_biome` | All markets |
| Seismic PGA (2% / 50yr) | DATA-aec-clusters.csv `seismic_pga` | All markets, if populated |
| Flood return period (100yr) | DATA-aec-clusters.csv `flood_100yr` | All markets, if populated |

**Empty fields are omitted from the grid.** The template must not
render rows labelled "N/A" or "—". A reader scanning the grid sees
only the data that exists for this market.

### CSS class names

| Class | Applies to |
|---|---|
| `.rm-aec-grid` | The grid container (CSS Grid, `grid-template-columns: max-content 1fr`) |
| `.rm-aec-grid-label` | Field label |
| `.rm-aec-grid-value` | Field value |

The grid is intentionally a `<dl>` (description list) under the hood,
which gives screen readers proper term/definition pairing.

---

## 5. Score breakdown

The composite score block answers "why does this market rank where it
does?" It has two parts: a small table with the six component values,
and an optional pure-CSS horizontal bar visualisation showing each
component's relative weight.

### Table

| Column | Notes |
|---|---|
| Metric | The component name (e.g. "Cluster count", "Best-tier weight", "Civic infrastructure", "AEC coverage", "Mobility coverage", "Span penalty") |
| Value | The component's contribution after weighting |
| Weight | The configured weight (from `score-regional-markets.py`) |

The bottom row shows the **Total** in bold, matching the value in
the infobox.

### Optional bar visualisation

Pure CSS only — no JavaScript. Each component renders as a horizontal
bar whose width is proportional to its weighted contribution. Bar fill
uses a single neutral colour from the design-system token bundle (not
tier colours — the score breakdown is orthogonal to tier).

### CSS class names

| Class | Applies to |
|---|---|
| `.rm-score-table` | The score breakdown `<table>` |
| `.rm-score-total` | The bold total row |
| `.rm-score-bars` | The optional bar visualisation container |
| `.rm-score-bar` | One bar row |
| `.rm-score-bar-fill` | The filled portion of one bar |
| `.rm-score-bar-label` | Component label |
| `.rm-score-bar-value` | Component value |

The bar visualisation may be hidden via a `prefers-reduced-motion: reduce`
fallback that drops to table-only on user preference. (Bars are not
animated; the fallback is for visual-density reduction.)

---

## 6. Wikipedia attribution footer

Mandatory on any Regional Market TOPIC that embeds Wikipedia summary
text in the Overview section, or that uses a Wikipedia thumbnail in the
infobox. The footer sits at the bottom of the article body, above the
existing wiki article footer.

```html
<footer class="rm-wiki-attribution">
  Wikipedia content reproduced under
  <a href="https://creativecommons.org/licenses/by-sa/4.0/" rel="license">CC BY-SA 4.0</a>.
  Accessed <time datetime="YYYY-MM-DD">YYYY-MM-DD</time>.
  Original article(s):
  <a href="https://en.wikipedia.org/wiki/...">City Name</a>,
  <a href="https://en.wikipedia.org/wiki/...">County Name</a>.
</footer>
```

### CSS class names

| Class | Applies to |
|---|---|
| `.rm-wiki-attribution` | The attribution footer block |
| `.rm-wiki-attribution a` | Embedded links (license + source articles) |

Visual treatment: smaller type (one step below body text), muted
foreground colour from the design-system neutral scale, top border to
separate from article body.

---

## 7. Template HTML skeleton

The skeleton below shows the structural markup the template emits.
It assumes the existing `wiki_chrome()` server function wraps it with
the standard wiki shell (topnav, TOC, search, dark-mode toggle, footer).
The skeleton is what gets injected into the `wiki_chrome()` body slot.

```html
<article class="rm-article">

  <!-- Left + right two-column wrapper -->
  <div class="rm-layout">

    <!-- LEFT COLUMN: article body -->
    <div class="rm-body">

      <h1 class="article__title">Wichita, Kansas — Regional Market</h1>

      <p class="rm-lead">
        Wichita is a Tier-1 Regional Market containing four
        co-location clusters. It is the largest market in central
        Kansas, 250 km southeast of Oklahoma City.
      </p>

      <section class="rm-overview">
        <h2 id="overview">Overview</h2>
        <p>
          <!-- Wikipedia summary extract, attributed -->
          Per Wikipedia, accessed 2026-05-30, Wichita is the largest
          city in Kansas …
        </p>
      </section>

      <section class="rm-colocation">
        <h2 id="co-location-profile">Co-location Profile</h2>
        <table class="rm-cluster-table">
          <thead>
            <tr>
              <th scope="col">Cluster</th>
              <th scope="col">Tier</th>
              <th scope="col">Anchor Composition</th>
              <th scope="col">Span (km)</th>
              <th scope="col">Civic</th>
            </tr>
          </thead>
          <tbody>
            <tr class="tier-t1">
              <td>
                <a href="https://gis.woodfinegroup.com/?cluster=NA-12345">
                  East Kellogg Hypermarket Belt
                </a>
              </td>
              <td><span class="rm-tier-badge rm-tier-badge--t1">T1</span></td>
              <td>Hypermarket + Hardware</td>
              <td>2.1</td>
              <td><span class="rm-civic-yes">Yes</span></td>
            </tr>
            <!-- additional rows … -->
          </tbody>
        </table>
      </section>

      <section class="rm-civic">
        <h2 id="civic-infrastructure">Civic Infrastructure</h2>
        <h3>Medical</h3>
        <ul>
          <li>Ascension Via Christi St Francis</li>
          <li>Wesley Medical Center</li>
        </ul>
        <h3>Academic</h3>
        <ul>
          <li>Wichita State University</li>
        </ul>
      </section>

      <section class="rm-aec">
        <h2 id="aec-data">AEC Data</h2>
        <dl class="rm-aec-grid">
          <dt class="rm-aec-grid-label">ASHRAE 169 Zone</dt>
          <dd class="rm-aec-grid-value">4A — Mixed-Humid</dd>
          <dt class="rm-aec-grid-label">Köppen-Geiger Class</dt>
          <dd class="rm-aec-grid-value">Cfa — Humid subtropical</dd>
          <dt class="rm-aec-grid-label">WWF Ecoregion</dt>
          <dd class="rm-aec-grid-value">Central forest-grasslands transition</dd>
          <dt class="rm-aec-grid-label">WWF Biome</dt>
          <dd class="rm-aec-grid-value">Temperate Grasslands, Savannas &amp; Shrublands</dd>
        </dl>
      </section>

      <section class="rm-score">
        <h2 id="composite-score">Composite Score</h2>
        <table class="rm-score-table">
          <thead>
            <tr>
              <th scope="col">Metric</th>
              <th scope="col">Value</th>
              <th scope="col">Weight</th>
            </tr>
          </thead>
          <tbody>
            <tr><td>Cluster count</td><td>12.0</td><td>3.0</td></tr>
            <tr><td>Best-tier weight</td><td>16.0</td><td>4.0</td></tr>
            <tr><td>Civic infrastructure</td><td>8.0</td><td>2.0</td></tr>
            <tr><td>AEC coverage</td><td>6.0</td><td>1.5</td></tr>
            <tr><td>Mobility coverage</td><td>4.0</td><td>1.0</td></tr>
            <tr><td>Span penalty</td><td>2.0</td><td>0.5</td></tr>
            <tr class="rm-score-total">
              <td>Total</td><td>48.0</td><td></td>
            </tr>
          </tbody>
        </table>

        <!-- Optional bar visualisation -->
        <div class="rm-score-bars" aria-hidden="true">
          <div class="rm-score-bar">
            <span class="rm-score-bar-label">Best-tier weight</span>
            <span class="rm-score-bar-fill" style="--rm-bar-pct: 33%"></span>
            <span class="rm-score-bar-value">16.0</span>
          </div>
          <!-- additional bars … -->
        </div>
      </section>

      <section class="rm-references">
        <h2 id="wikipedia-references">Wikipedia References</h2>
        <ul>
          <li><a href="https://en.wikipedia.org/wiki/Wichita,_Kansas">Wichita, Kansas</a></li>
          <li><a href="https://en.wikipedia.org/wiki/Sedgwick_County,_Kansas">Sedgwick County, Kansas</a></li>
          <li><a href="https://en.wikipedia.org/wiki/Wichita_State_University">Wichita State University</a></li>
        </ul>
      </section>

      <footer class="rm-wiki-attribution">
        Wikipedia content reproduced under
        <a href="https://creativecommons.org/licenses/by-sa/4.0/" rel="license">CC BY-SA 4.0</a>.
        Accessed <time datetime="2026-05-30">2026-05-30</time>.
        Original article(s):
        <a href="https://en.wikipedia.org/wiki/Wichita,_Kansas">Wichita, Kansas</a>,
        <a href="https://en.wikipedia.org/wiki/Sedgwick_County,_Kansas">Sedgwick County, Kansas</a>.
      </footer>

    </div>

    <!-- RIGHT COLUMN: infobox -->
    <aside class="rm-infobox" aria-labelledby="rm-infobox-heading">
      <h2 id="rm-infobox-heading" class="visually-hidden">Market Summary</h2>

      <dl class="rm-infobox-fields">
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Rank</dt>
          <dd class="rm-infobox-field-value">
            <span class="rm-rank-badge">16 / 400</span>
          </dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Best Tier</dt>
          <dd class="rm-infobox-field-value">
            <span class="rm-tier-badge rm-tier-badge--t1">T1</span>
          </dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Clusters</dt>
          <dd class="rm-infobox-field-value">4</dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Composite Score</dt>
          <dd class="rm-infobox-field-value">48.0</dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Country</dt>
          <dd class="rm-infobox-field-value">US</dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Nearest major centre</dt>
          <dd class="rm-infobox-field-value">Oklahoma City, 250 km SE</dd>
        </div>
        <div class="rm-infobox-field">
          <dt class="rm-infobox-field-label">Centroid</dt>
          <dd class="rm-infobox-field-value">37.6872, −97.3301</dd>
        </div>
      </dl>

      <!-- Wikipedia thumbnail (if available) -->
      <figure class="rm-wiki-thumb">
        <img src="https://upload.wikimedia.org/.../wichita-skyline.jpg"
             alt="Wichita skyline at dusk"
             loading="lazy">
        <figcaption class="rm-wiki-thumb-caption">
          <a href="https://en.wikipedia.org/wiki/Wichita,_Kansas">Wichita, Kansas (Wikipedia)</a>
        </figcaption>
      </figure>
    </aside>

  </div>
</article>
```

---

## Open questions for project-design

These items are flagged for resolution during component implementation.

1. **Tier badge tokens.** Are gold / silver / bronze already declared in
   `pointsav-design-system` tokens? If yes, the badge component reuses
   them; if no, they should be added before this template ships.
2. **Sticky offset.** The `--rm-infobox-sticky-top` value depends on the
   `wiki_chrome()` topnav height plus any existing sticky breadcrumb. The
   value should come from a design-system token, not be hardcoded in this
   template.
3. **Mobile breakpoint reuse.** The wiki chrome already has a mobile
   breakpoint (768 px per `app-mediakit-knowledge/static/style.css`).
   Confirm this template uses the same breakpoint variable rather than
   declaring its own.
4. **Civic badge style.** The "Yes / No" civic indicator could either
   be a plain text label or a small shaded chip. The template currently
   asks for a chip; project-design should confirm against the existing
   chip / pill component patterns.
5. **Bar visualisation polish.** The pure-CSS horizontal bars are
   specified as optional. If the design-system has an existing
   data-bar / sparkline pattern, use that instead of inventing a new one.

---

## Routing

This is a generic article-template specification with no PointSav or
Woodfine branding. Per `.agent/rules/design-tokens.md` rule 1, it
routes to **`pointsav-design-system`** for implementation:

- Component recipe → `components/regional-market-topic/guide.md`
- Research file → `dtcg-vault/research/regional-market-topic.md`

No token changes are requested; this template composes existing
tokens (heading scale, neutral colours, chip pattern, table treatment).
If a tier-tint token is missing, project-design opens a separate
DESIGN-TOKEN-CHANGE draft with Master co-sign before adding it.
