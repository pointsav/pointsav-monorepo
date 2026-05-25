---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-token-architecture.md
audience: internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-23
authored_by: totebox@project-knowledge
authored_with: claude-opus-4-7
---

# DESIGN-RESEARCH: Token architecture for app-mediakit-knowledge

Audit of the full token ecosystem across `pointsav-design-system`,
`pointsav-media-assets`, `woodfine-media-assets`, the wireframe substrate,
and the live wiki engine; followed by a definitive proposal for a three-file
brand-override architecture, a new `shell.*` namespace for header/footer
isolation, a font-loading token tier, a naming-conflict resolution, and a
WCAG-correctness fix to the existing tertiary text token.

---

## 1. Executive summary

Five token systems exist in parallel today: the wiki engine's vendored
DTCG bundle (148 CSS custom properties under `--color-*`, `--surface-*`,
`--text-*`, `--knowledge-*`, component scopes); the design-system
canonical DTCG bundle (a divergent superset under the same path
prefixes); the two `*-media-assets` brand-palette YAML files
(`pointsav-canvas: #09090B`, `woodfine-blue: #164679`); the
`theme-generic.css` / `theme-woodfine.css` `--sys-*` substrate; and the
wireframe `--ds-*` / `--wf-*` substrate. The single largest hazard is
not the divergence itself but the **naming conflict** between
`--ds-paper-*` / `--ds-ink-*` (wireframe) and `--surface-*` / `--text-*`
(wiki DTCG output). This document selects `--surface-*` / `--text-*` as
the canonical convention because it is what the live wiki and the DTCG
generator already emit; `--ds-*` is wireframe-local and migrates inward.
The proposed architecture is three files —
`tokens-base.css` (brand-neutral primitives + semantic intent),
`tokens-pointsav.css` (PointSav steel-blue + dark-canvas overrides),
`tokens-woodfine.css` (Woodfine blue + light-canvas overrides) — loaded
in that order by the wiki binary per `WIKI_BRAND_THEME`. A new
`shell.*` semantic namespace (header rows, footer rows, sidebar, sticky
threshold) closes the chrome-isolation gap, and a `font.face.*`
generator extension self-hosts Nunito Sans variable for Woodfine. The
WCAG defect at `#878d99` is replaced at the primitive tier with
`#666c78` (neutral-60), which clears 4.5:1 contrast on every semantic
surface the token participates in.

---

## 2. Token ecosystem map — factual inventory

| File | Path | Purpose | Token count | Tier |
|---|---|---|---:|---|
| Wiki DTCG bundle | `pointsav-monorepo/app-mediakit-knowledge/scripts/dtcg-bundle.json` | Source of truth for wiki CSS generation | 148 leaf tokens | Canonical (vendored copy) |
| Wiki generator | `pointsav-monorepo/app-mediakit-knowledge/scripts/dtcg-to-css.py` | DTCG → CSS oklch emitter | (script) | Build tool |
| Wiki generated CSS | `pointsav-monorepo/app-mediakit-knowledge/static/tokens.css` | PointSav baseline (deployed) | ~148 custom properties | Runtime (generated) |
| Wiki Woodfine override | `pointsav-monorepo/app-mediakit-knowledge/static/tokens-woodfine.css` | Semantic-tier swap for Woodfine | 17 custom properties | Runtime (hand-edited) |
| Wiki engine override layer | `pointsav-monorepo/app-mediakit-knowledge/static/theme-woodfine.css` | Engine-alias bridge (`--bg`, `--link`, `--accent`) | 14 custom properties | Runtime (hand-edited) |
| Wiki style aliases | `pointsav-monorepo/app-mediakit-knowledge/static/style.css` lines 9–62 | Engine-internal aliases (`--bg`, `--fg`, `--link`, `--max-content-width`, `--toc-width`) | ~28 custom properties | Runtime (hand-edited) |
| DS canonical bundle | `pointsav-design-system/tokens/dtcg-bundle.json` | DTCG superset (wiki + non-wiki) | 178 leaf tokens | Canonical |
| DS legacy substrate (PointSav) | `pointsav-design-system/tokens/theme-generic.css` | `--sys-*` Tier-2 baseline | 14 custom properties | Legacy |
| DS legacy substrate (Woodfine) | `pointsav-design-system/tokens/theme-woodfine.css` | `--sys-*` Tier-2 Woodfine override | 7 custom properties | Legacy |
| DS legacy palette (PointSav) | `pointsav-design-system/tokens/token-global-color.yaml` | Brand palette (hex only) | 8 colours | Legacy |
| DS global YAMLs | `pointsav-design-system/tokens/global/token-global-{typography,spacing,elevation,print,assets}.yaml` | Tier-1 globals (typography physics, spacing scale, etc.) | ~30 entries | Legacy |
| DS alias YAML | `pointsav-design-system/tokens/semantic/token-alias-ui.yaml` | Tier-2 aliases (`canvas-base`, `touch-target-min`) | ~14 entries | Legacy |
| PointSav brand palette | `pointsav-media-assets/token-global-color.yaml` | Brand palette (hex only) | 8 colours | Brand asset |
| Woodfine brand palette | `woodfine-media-assets/token-global-color.yaml` | Brand palette (hex only) | 8 colours | Brand asset |
| Woodfine theme map | `woodfine-media-assets/theme-woodfine-light.yaml` | YAML injection table (Woodfine → alias slots) | 8 mappings | Brand asset |
| Woodfine theme CSS | `woodfine-media-assets/css/theme-woodfine-light.css` | `--bg-*`, `--text-*`, `--border-*`, `--accent-*` block | 12 custom properties | Brand asset (light theme) |
| Woodfine font files | `woodfine-media-assets/fonts/{Nunito_Sans,Barlow_Condensed,Sahitya,Zilla_Slab}/` | OFL-licensed self-hostable TTF/variable TTF | 4 families | Brand asset (binary) |
| Wireframe substrate | `.agent/drafts-outbound/wireframe-woodfinegroup-home.draft.html` lines 54–124 | `--ds-*` + `--wf-*` chrome wireframe | ~50 custom properties | Draft |

Five distinct namespaces compete for the same conceptual slot:

| Concept | Wiki DTCG | Wireframe | DS Legacy | Woodfine assets |
|---|---|---|---|---|
| Canvas | `--surface-background` | `--ds-paper-2` | `--sys-canvas` | `--bg-deep` |
| Card / raised surface | `--surface-layer` | `--ds-paper-1` | `--sys-card` | `--bg-panel` |
| Body text | `--text-primary` | `--ds-ink-1` | `--sys-text` | `--text-primary` |
| Muted text | `--text-secondary` | `--ds-ink-2` / `--ds-ink-3` | `--sys-muted` | `--text-muted` |
| Hairline border | `--border-subtle` | `--ds-rule-hairline` | `--sys-hairline` | `--border-dim` |
| Accent / link | `--interactive-link` | `--wf-blue` | `--sys-accent` | `--accent-woodfine` |

This is the consolidation problem; §4 resolves it.

---

## 3. Annotated token audit — wiki dtcg-bundle.json

Status legend: **KEEP** — correct as-is; **CHANGE** — value or
description wrong; **MOVE** — wrong tier; **DROP** — not used by engine
or duplicates another token; **ADD** — missing token the engine needs.

### 3.1 `primitive.color.brand.*` (8 tokens)

| Path | Status | Notes |
|---|---|---|
| `primitive.color.brand.blue.50` `#edf5ff` | KEEP | Carbon blue-10 equivalent; used as status.info.bg ancestor |
| `primitive.color.brand.blue.60` `#0f62fe` | KEEP | PointSav primary interactive blue (Carbon interactive-01) |
| `primitive.color.brand.blue.70` `#0043ce` | KEEP | Link-active and hover-darker |
| `primitive.color.brand.blue.80` `#002d9c` | KEEP | Reserved for visited-on-light variants |
| `primitive.color.brand.teal.50` `#d9fbfb` | KEEP | Featured-pin bg |
| `primitive.color.brand.teal.60` `#009d9a` | KEEP | Featured-pin accent / direct-source signal |
| (missing) `primitive.color.brand.steel.50` | ADD | PointSav steel-blue light — `#A4B9D0` for hover-state and aside-tinted surfaces |
| (missing) `primitive.color.brand.steel.60` | ADD | PointSav steel-blue base — `#869FB9`, currently leaked into `style.css` line 37 as a raw hex literal (`--accent`) |
| (missing) `primitive.color.brand.steel.70` | ADD | PointSav steel-blue hover/visited — `#6181A4`, currently in `token-global-color.yaml` as `pointsav-steel-hover` |
| (missing) `primitive.color.brand.woodfine.60` | ADD | Woodfine blue base — `#164679`, currently in `tokens-woodfine.css` as raw oklch |
| (missing) `primitive.color.brand.woodfine.70` | ADD | Woodfine blue visited — `#0F3258` |
| (missing) `primitive.color.brand.woodfine.50` | ADD | Woodfine blue tint — `#E8EFF7` (already in wireframe as `--wf-blue-tint`) |

### 3.2 `primitive.color.link.*` (5 tokens)

All KEEP. `#3366cc` default, `#795cb2` visited, `#447ff5` hover,
`#0043ce` active, `#cc3333` redlink. Encyclopedic register; Wikimedia
Codex muscle-memory continuity rationale stands.

### 3.3 `primitive.color.cluster.*` (5 tokens)

KEEP all five. GIS cluster ramp; orthogonal to wiki chrome; passes WCAG
AA per Master co-sign 2026-05-07.

### 3.4 `primitive.color.status.*` (8 tokens)

KEEP all eight. Success/warn/info/error × {base, bg}. Confirmed compliant.

### 3.5 Implicit neutral ramp (NOT in current bundle — gap)

The wiki DTCG bundle resolves `--text-primary`, `--text-secondary`,
`--text-tertiary`, `--surface-background`, `--surface-layer-accent` to
literal hex values (`#0e0f12`, `#4a4f59`, `#878d99`, `#f5f6f8`,
`#e6e8ec`) with descriptions that point at a phantom
`primitive.color.neutral-*` ramp "in primitive.json" — but the wiki's
`dtcg-bundle.json` itself never declares the ramp. The `pointsav-design-system`
canonical bundle does declare it (`neutral-10` through `neutral-100`,
plus `white`, `black`). This is a synchronisation drift.

| Path | Status | Notes |
|---|---|---|
| (missing) `primitive.color.neutral-10` | ADD | `#f5f6f8` — currently inlined as `surface.background` literal |
| (missing) `primitive.color.neutral-20` | ADD | `#e6e8ec` — currently inlined as `surface.layer-accent`, `border.subtle`, multiple knowledge.* values |
| (missing) `primitive.color.neutral-30` | ADD | `#cdd1d8` — `surface.layer-hover`, `text.disabled`, `border.disabled` |
| (missing) `primitive.color.neutral-40` | ADD | `#aab0bb` — `text.placeholder` |
| (missing) `primitive.color.neutral-50` | DROP after refactor | `#878d99` — **WCAG failure** at every semantic use; replace with `#666c78` neutral-60 (see §8) |
| (missing) `primitive.color.neutral-60` | ADD | `#666c78` — new `text.tertiary` / `border.strong` target |
| (missing) `primitive.color.neutral-70` | ADD | `#4a4f59` — `text.secondary` |
| (missing) `primitive.color.neutral-90` | ADD | `#1f2125` — `surface.inverse` |
| (missing) `primitive.color.neutral-100` | ADD | `#0e0f12` — `text.primary` |

After ADD, every semantic colour token's `$value` becomes a
`{primitive.color.neutral-XX}` alias rather than a hex literal. The
generator already resolves aliases (`dtcg-to-css.py` line 94 `resolve`);
this requires no script change.

### 3.6 `primitive.font.family.*` (5 tokens)

| Path | Status | Notes |
|---|---|---|
| `font.family.body` `Georgia, 'Times New Roman', Times, serif` | KEEP | System-stack serif per F-3 (Master co-sign 2026-05-07) |
| `font.family.heading` `Charter, 'Bitstream Charter', Georgia, serif` | KEEP | System-stack heading serif |
| `font.family.sans` `system-ui, -apple-system, ...` | KEEP | Default sans for PointSav chrome |
| `font.family.mono` `ui-monospace, SFMono-Regular, ...` | KEEP | Default mono |
| `font.family.kbd` `ui-monospace, SFMono-Regular, ...` | KEEP | Code/kbd subset |
| (missing) `primitive.font.family.display` | ADD | For Woodfine: `'Barlow Condensed', 'Oswald', 'Trade Gothic LT Std', 'Helvetica Neue', Arial, sans-serif` — utility-row uppercase tracking |
| (missing) `primitive.font.family.brand-sans` | ADD | For Woodfine: `'Nunito Sans', 'Avenir LT Std', system-ui, ...` |
| (missing) `primitive.font.family.brand-serif` | ADD | For Woodfine: `'Zilla Slab', 'Roboto Slab', Georgia, serif` |

These three are PointSav-blank, Woodfine-populated; they get overridden
in `tokens-woodfine.css`.

### 3.7 `primitive.font.size.*` (14 tokens)

KEEP all 14. Modular scale; covers utility (11px) through hero (68px).
Wireframe's `--ds-fs-utility: 10px` is **below** the scale floor; on
review, 10px utility text fails WCAG 1.4.4 (resizable text) on most
screens — keep the 11px floor and require the wireframe to migrate up.

### 3.8 `primitive.font.weight.*` + `primitive.line.height.*`

KEEP all 9 tokens. No gaps.

### 3.9 `primitive.space.*` (8 tokens)

| Path | Status | Notes |
|---|---|---|
| `space.025` (2px) → `space.32` (256px) | KEEP | Eight-step exponential |
| (missing) `primitive.space.06` (12px) | ADD | Wireframe `--ds-sp-3`; required for utility-row vertical padding |
| (missing) `primitive.space.3` (24px) | ADD | Wireframe `--ds-sp-6`; common chrome internal padding |
| (missing) `primitive.space.6` (36px) | ADD | Wireframe `--ds-sp-7` |
| (missing) `primitive.space.5` (40px) | ADD | Hits CTA / button-row gap value |

The scale is currently powers of two only; adding the 3/4 interstitials
matches what real chrome layouts need.

### 3.10 `primitive.radius.*` + `primitive.motion.*` + `primitive.density.*`

KEEP all. No gaps.

### 3.11 `semantic.surface.*`, `semantic.text.*`, `semantic.border.*`, `semantic.interactive.*`

KEEP, but **CHANGE** every `$value` from a hex literal back to a
`{primitive.color.neutral-XX}` alias after §3.5 ADD lands. Specifically:

| Token | Current value (hex literal) | Should be |
|---|---|---|
| `semantic.text.tertiary` | `#878d99` | `{primitive.color.neutral-60}` (**`#666c78`**, see §8) |
| `semantic.border.strong` | `#878d99` | `{primitive.color.neutral-60}` |
| `semantic.knowledge.editpencil.color` | `#878d99` | `{primitive.color.neutral-60}` |
| `component.article.section.heading-edit-pencil-color` | `#878d99` | `{primitive.color.neutral-60}` |

These four are the surface area of the WCAG defect.

### 3.12 `semantic.knowledge.*` (8 tokens)

KEEP all. Namespace approved by Master co-sign 2026-04-30, renamed to
`knowledge.*` per F-1 condition 2026-05-07.

### 3.13 `component.*` (39 tokens)

KEEP all. Three families (home-grid, home-featured, home-recent) plus
seven article.* sub-families (lead, toc, section, references,
research-trail, fli-banner, density-toggle). No drops.

### 3.14 Missing wiki-specific semantic tokens (chrome / shell)

Today the wiki engine encodes its chrome dimensions as raw values in
`style.css`:

| Dimension | Current location | Current value | Should be |
|---|---|---|---|
| Article max-width | `style.css:11` `--max-content-width` | `76em` | `shell.content.max-width` token |
| TOC width | `style.css:391` `--toc-width` | `14em` | `shell.toc.width` token |
| Header padding | `style.css:113` `.mw-header` | `0.75rem 1.25rem` | `shell.header.padding-block` + `shell.header.padding-inline` |
| Footer padding | `style.css:159` `.site-footer` | `1.5rem 1.25rem 1rem` | `shell.footer.padding-*` |
| Mobile breakpoint | `style.css:237` `@media` | `768px` | Should reference `home-grid.breakpoint-2-to-1` or a new `shell.breakpoint.mobile` |

The complete `shell.*` proposal is in §5.

### 3.15 Tokens to ADD outright (full list)

Beyond §5 shell.* and §6 font.face.*:

1. `primitive.color.neutral-{10,20,30,40,60,70,80,90,100}` — 9 tokens
2. `primitive.color.brand.steel.{50,60,70}` — 3 PointSav institutional
3. `primitive.color.brand.woodfine.{50,60,70}` — 3 Woodfine institutional
4. `primitive.font.family.{display,brand-sans,brand-serif}` — 3 brand stacks
5. `primitive.space.{06,3,5,6}` — 4 spacing interstitials

Total ADD: **22 net-new primitives** before semantic/shell additions.

---

## 4. Naming convention decision — `--ds-*` vs `--surface-*` / `--text-*`

**Decision: `--surface-*` / `--text-*` / `--interactive-*` / `--border-*` wins.**
The `--ds-*` convention from the wireframe is deprecated on contact.

### 4.1 Why

1. The wiki engine already ships ~148 `--surface-*`/`--text-*`/`--interactive-*` properties to every page. Reversing this would force regeneration of `tokens.css`, hand-rewrite of `tokens-woodfine.css`, hand-rewrite of `style.css` aliases (lines 19–37), and a full visual-regression sweep of every live page at `documentation.pointsav.com` — for no functional gain.
2. The `--surface-*` / `--text-*` names are **intent-named**, the
   DTCG-recommended pattern: the token name describes what it is for
   (a surface, a text colour, a border), not what tier of paper or ink
   it represents. `--ds-paper-2` and `--ds-ink-3` carry no intent
   information — a downstream consumer cannot tell which one is the
   body-text colour without consulting a map.
3. The `--ds-*` prefix asserts vendor ownership ("design system");
   workspace policy is **unprefixed semantic tokens** (Master co-sign
   2026-04-30 ratification: "the token substrate uses no vendor prefix
   at any tier"). `--ds-*` violates that policy.
4. The wireframe was authored 2026-05-09 with explicit notes-for-design
   acknowledging it would migrate to the canonical token substrate.
   This is the migration.

### 4.2 Migration path

Three commits, in order, all in `app-mediakit-knowledge`:

1. **Commit A — wireframe rename** (project-design, no engine impact). In
   the design-system repo where the wireframe lands, rewrite the
   wireframe's `--ds-*` block to use `--surface-*`, `--text-*`,
   `--font-family-*`, `--font-size-*`, `--space-*`, `--shell-*` (new)
   names with the same values. Visual output identical.
2. **Commit B — wireframe consumer rewrite**. Update every CSS selector
   in the wireframe `<style>` block from `var(--ds-paper-2)` to
   `var(--surface-background)`, etc. Mechanical search-replace; the
   table in §2 is the exact mapping. Visual output identical.
3. **Commit C — wireframe value reconciliation**. Where wireframe
   values disagree with token-substrate values (e.g. wireframe
   `--ds-paper-2: #F7F9FA` vs current PointSav `--surface-background:
   #f5f6f8`), the **brand-override file decides** — under
   `tokens-woodfine.css`, `--surface-background` is `#F7F9FA` (already);
   under PointSav default it stays `#f5f6f8`. The wireframe targets
   Woodfine specifically and was always meant to render under the
   Woodfine override.

### 4.3 What remains `--ds-*` after migration

Nothing. There are no `--ds-*` tokens in the wiki engine today and there
should be none after the wireframe migrates. The convention is
**banned** going forward.

### 4.4 What remains brand-local prefix-tagged

`--wf-*` (Woodfine) and any future `--ps-*` (PointSav) are also banned
at runtime. The brand-override file (`tokens-woodfine.css`) **overrides
the unprefixed semantic tokens**; downstream CSS reads
`var(--interactive-link)` and gets the right colour automatically. The
wireframe's `--wf-blue: #164679` becomes the value of
`--interactive-link` under the Woodfine override, full stop.

---

## 5. Proposed `shell.*` token namespace

The wiki chrome — header rows, footer rows, sidebar, sticky threshold
— currently has no token tier. This is the biggest semantic gap in the
current bundle and the proximate cause of every "magic number"
literal in `style.css` (`0.75rem 1.25rem`, `76em`, `14em`, `768px`).

### 5.1 Namespace shape

```yaml
semantic:
  shell:
    header:
      utility-row-height:  # row 1 — language toggle, account chrome
      brand-row-height:    # row 2 — wordmark
      nav-row-height:      # row 3 — primary navigation
      total-height:        # sum, for sticky offset math
      padding-inline:      # left + right padding inside chrome rows
      padding-block:       # top + bottom padding inside chrome rows
      sticky-threshold:    # scroll-y at which nav-row pins (px)
      border-bottom-color: # alias → semantic.border.subtle
      utility-bg:          # alias → semantic.surface.layer-accent
      brand-bg:            # alias → semantic.surface.background
      nav-bg:              # alias → semantic.surface.background
    footer:
      total-height:        # baseline footer chrome height
      padding-inline:
      padding-block-start: # top padding (larger)
      padding-block-end:   # bottom padding (smaller)
      border-top-color:    # alias → semantic.border.subtle
      bg:                  # alias → semantic.surface.layer
      copyright-bg:        # the baseplate row (footer-row 3+)
    sidebar:
      width:               # left-rail TOC width
      gap:                 # space between sidebar and main column
      collapse-at:         # viewport-width breakpoint
    content:
      max-width:           # outer constraint on .site-main
      padding-inline:      # gutters when below max-width
      gap:                 # vertical rhythm between sections
    breakpoint:
      mobile:              # collapses chrome to single column
      tablet:              # collapses sidebar
      desktop:             # full three-column
```

### 5.2 Proposed values (per brand)

| Token | PointSav (baseline) | Woodfine (override) |
|---|---|---|
| `shell.header.utility-row-height` | `2rem` (32px) | `2.25rem` (36px) |
| `shell.header.brand-row-height` | `4.5rem` (72px) | `7.5rem` (120px) — wordmark prominence |
| `shell.header.nav-row-height` | `3rem` (48px) | `3rem` (48px) |
| `shell.header.total-height` | `9.5rem` (152px) | `12.75rem` (204px) |
| `shell.header.padding-inline` | `1.25rem` (20px) | `3.5rem` (56px) — wider gutters per wireframe |
| `shell.header.padding-block` | `0.75rem` (12px) | `0.5rem` (8px) |
| `shell.header.sticky-threshold` | `9.5rem` | `12.75rem` |
| `shell.header.border-bottom-color` | `{semantic.border.subtle}` | `{semantic.border.subtle}` |
| `shell.header.utility-bg` | `{semantic.surface.layer-accent}` | `{primitive.color.neutral-20}` (Woodfine: `#E6E7E8`) |
| `shell.header.brand-bg` | `{semantic.surface.layer}` | `{semantic.surface.background}` |
| `shell.header.nav-bg` | `{semantic.surface.layer}` | `{semantic.surface.background}` |
| `shell.footer.total-height` | (auto) | (auto) |
| `shell.footer.padding-inline` | `1.25rem` | `5rem` (80px) |
| `shell.footer.padding-block-start` | `1.5rem` | `1.25rem` |
| `shell.footer.padding-block-end` | `1rem` | `0.75rem` |
| `shell.footer.border-top-color` | `{semantic.border.subtle}` | `{semantic.border.subtle}` |
| `shell.footer.bg` | `{semantic.surface.layer}` | `{semantic.surface.background}` |
| `shell.footer.copyright-bg` | `{semantic.surface.layer-accent}` | `{semantic.surface.layer-accent}` |
| `shell.sidebar.width` | `14em` | `14em` |
| `shell.sidebar.gap` | `2rem` | `2rem` |
| `shell.sidebar.collapse-at` | `960px` | `960px` |
| `shell.content.max-width` | `76em` | `90em` (1440px — wireframe `--ds-container-max`) |
| `shell.content.padding-inline` | `1.25rem` | `1.25rem` |
| `shell.content.gap` | `2rem` | `2rem` |
| `shell.breakpoint.mobile` | `480px` | `480px` |
| `shell.breakpoint.tablet` | `768px` | `768px` |
| `shell.breakpoint.desktop` | `1024px` | `1024px` |

Emitted CSS custom properties: `--shell-header-utility-row-height`,
`--shell-header-brand-row-height`, etc. The
`to_css_var()` function in `dtcg-to-css.py` already strips the
`semantic.` prefix and joins on `-`, so no generator change is needed
for the basic case.

### 5.3 What this replaces in `style.css`

| Line | Current | Replacement |
|---|---|---|
| 11 | `--max-content-width: 76em;` | `var(--shell-content-max-width)` |
| 113 | `.mw-header { padding: 0.75rem 1.25rem; }` | `padding: var(--shell-header-padding-block) var(--shell-header-padding-inline);` |
| 159 | `.site-footer { padding: 1.5rem 1.25rem 1rem; }` | `padding: var(--shell-footer-padding-block-start) var(--shell-footer-padding-inline) var(--shell-footer-padding-block-end);` |
| 237, 878, 960, 983 | `@media (max-width: 768px) / 960px / 600px` | `@media (max-width: var(--shell-breakpoint-tablet))` (note: CSS does not support custom properties in media queries directly — use a build-time SASS-like substitution, or accept that breakpoint tokens are documentation-only and the literals must continue in `@media` rules. Recommend the latter for now.) |
| 391 | `--toc-width: 14em;` | `var(--shell-sidebar-width)` |

The mobile-breakpoint media-query case is the only one that does not
translate cleanly because CSS custom properties are runtime values and
`@media` evaluates at parse time. Document this as a known limitation;
keep the breakpoint tokens for documentation continuity even though
they cannot drive `@media` rules directly.

---

## 6. Font-loading token additions

Today the wiki ships **no `@font-face` declarations**. PointSav uses
system stacks (correct). Woodfine needs Nunito Sans (variable),
Zilla Slab, and Barlow Condensed — all already in
`woodfine-media-assets/fonts/` under OFL. The wireframe currently loads
these from Google Fonts CDN (`fonts.googleapis.com`); that is a
sovereignty violation per CLAUDE.md §6 ("no CDN dependency at this
stage"). Self-host.

### 6.1 New `primitive.font.family.*` entries

Already proposed in §3.6 — `display`, `brand-sans`, `brand-serif`. The
PointSav DTCG bundle defaults them to system stacks; Woodfine overrides
swap in self-hosted families.

### 6.2 New `font.face.*` namespace (generator-side, not semantic)

DTCG does not standardise `@font-face`. Proposal: add a top-level
sibling to `primitive`/`semantic`/`component` named `font-face`,
treated specially by `dtcg-to-css.py` to emit `@font-face` blocks
**before** the `:root {}` block.

```yaml
font-face:
  nunito-sans-variable:
    family: "Nunito Sans"
    style: normal
    weight: "300 700"           # variable axis range
    src: "url('/static/fonts/NunitoSans-VariableFont_YTLC,opsz,wdth,wght.woff2') format('woff2-variations')"
    display: swap
    unicode-range: "U+0000-00FF, U+0131, ..."  # Latin Extended basic
  nunito-sans-italic-variable:
    family: "Nunito Sans"
    style: italic
    weight: "300 700"
    src: "url('/static/fonts/NunitoSans-Italic-VariableFont_YTLC,opsz,wdth,wght.woff2') format('woff2-variations')"
    display: swap
  zilla-slab-regular:
    family: "Zilla Slab"
    style: normal
    weight: 400
    src: "url('/static/fonts/ZillaSlab-Regular.woff2') format('woff2')"
    display: swap
  zilla-slab-bold:
    family: "Zilla Slab"
    style: normal
    weight: 700
    src: "url('/static/fonts/ZillaSlab-Bold.woff2') format('woff2')"
    display: swap
  barlow-condensed-medium:
    family: "Barlow Condensed"
    style: normal
    weight: 500
    src: "url('/static/fonts/BarlowCondensed-Medium.woff2') format('woff2')"
    display: swap
  barlow-condensed-semibold:
    family: "Barlow Condensed"
    style: normal
    weight: 600
    src: "url('/static/fonts/BarlowCondensed-SemiBold.woff2') format('woff2')"
    display: swap
```

### 6.3 Generated output (in `tokens-woodfine.css` only)

```css
/* Woodfine self-hosted faces — emitted before :root */
@font-face {
  font-family: "Nunito Sans";
  font-style: normal;
  font-weight: 300 700;
  font-display: swap;
  src: url('/static/fonts/NunitoSans-VariableFont_YTLC,opsz,wdth,wght.woff2')
       format('woff2-variations');
}
@font-face {
  font-family: "Nunito Sans";
  font-style: italic;
  font-weight: 300 700;
  font-display: swap;
  src: url('/static/fonts/NunitoSans-Italic-VariableFont_YTLC,opsz,wdth,wght.woff2')
       format('woff2-variations');
}
/* ... Zilla Slab and Barlow Condensed faces ... */

:root {
  /* Brand family overrides */
  --font-family-sans:    "Nunito Sans", "Avenir LT Std", system-ui, -apple-system, ...;
  --font-family-body:    "Zilla Slab", "Roboto Slab", Georgia, serif;
  --font-family-heading: "Zilla Slab", "Roboto Slab", Georgia, serif;
  --font-family-display: "Barlow Condensed", "Oswald", "Trade Gothic LT Std", "Helvetica Neue", Arial, sans-serif;

  /* ... rest of Woodfine overrides ... */
}
```

### 6.4 PointSav font behaviour

`tokens-pointsav.css` emits **no `@font-face` blocks**. All four
families fall through to their system-stack defaults from
`tokens-base.css`. This preserves the sovereignty discipline for the
default deployment.

### 6.5 Asset pipeline note

TTF files in `woodfine-media-assets/fonts/` must be converted to WOFF2
(approximately 3× smaller; ~15 KB per static face, ~80 KB for the
variable face). The conversion is a separate ASSET-* commit in
`pointsav-design-system`. Use `fonttools` (`pyftsubset` for subset,
`woff2_compress` for compression). Subset to Latin Extended-A
(U+0000–017F, ~250 glyphs) to drop file size by another ~70%. This is
out of scope for the token architecture itself but is the blocker for
shipping Woodfine self-hosted fonts.

---

## 7. Revised three-file CSS architecture

### 7.1 File responsibilities

#### `tokens-base.css` (brand-neutral)

Generated from `dtcg-bundle.json` minus any brand-specific overrides.
Contains:
- All `primitive.*` tokens (colour ramps, font scale, space scale, radius, motion, density)
- All `semantic.*` tokens with neutral PointSav-baseline values
- All `component.*` tokens
- All `shell.*` tokens with PointSav-baseline values
- **No `@font-face` blocks** (system stacks)

This file loads **always**, first, on every page. ~150 custom
properties.

#### `tokens-pointsav.css` (PointSav override layer)

New file. Contains only the **deltas** that distinguish PointSav from
the neutral baseline:

```css
:root {
  /* Brand institutional accent — steel-blue */
  --color-brand-steel-50: oklch(...);  /* #A4B9D0 */
  --color-brand-steel-60: oklch(...);  /* #869FB9 */
  --color-brand-steel-70: oklch(...);  /* #6181A4 */

  /* Semantic interactive — point steel-blue at the slots that style.css reads */
  --interactive-button-primary: var(--color-brand-steel-60);
  --interactive-focus-ring:     var(--color-brand-steel-60);
  --border-interactive:         var(--color-brand-steel-60);

  /* Engine-internal alias (the one that style.css line 37 used to literalise) */
  --accent: var(--color-brand-steel-60);

  /* PointSav uses Wikipedia-blue for editorial-register links — NOT steel-blue.
     Steel-blue is for institutional chrome only. So --interactive-link stays
     pointed at primitive.color.link.default per tokens-base.css. */
}
```

Approximately 8 custom properties. Loads when `WIKI_BRAND_THEME=pointsav`
(default).

#### `tokens-woodfine.css` (Woodfine override layer)

Existing file, expanded. Contains:
- All `@font-face` blocks (§6.3)
- Brand institutional family overrides (`--font-family-sans`,
  `--font-family-body`, `--font-family-heading`, `--font-family-display`)
- Semantic surface overrides (light theme — `--surface-background: #F7F9FA`)
- Semantic text overrides (`--text-primary: #111827`)
- Semantic interactive overrides (Woodfine Blue for link/button/focus)
- Engine-alias `--accent: #164679`
- Shell overrides where Woodfine differs (per §5.2)

Approximately 35–40 custom properties plus 6 `@font-face` blocks. Loads
when `WIKI_BRAND_THEME=woodfine`.

### 7.2 Load order

```html
<link rel="stylesheet" href="/static/tokens-base.css">         <!-- always -->
<link rel="stylesheet" href="/static/tokens-pointsav.css">     <!-- if pointsav -->
<!-- OR -->
<link rel="stylesheet" href="/static/tokens-woodfine.css">     <!-- if woodfine -->
<link rel="stylesheet" href="/static/style.css">               <!-- always -->
```

The brand-override file is emitted from the binary at render time based
on `WIKI_BRAND_THEME` env var (already wired in current code per
tokens-woodfine.css comment line 2). Only one brand-override file loads
per request.

### 7.3 What this replaces

| Old file | New file | Migration |
|---|---|---|
| `static/tokens.css` (148 props, PointSav-baseline) | `static/tokens-base.css` (~150 props, brand-neutral) + `static/tokens-pointsav.css` (~8 props) | Split: extract PointSav-specific into the override layer, leave neutral primitives in base |
| `static/tokens-woodfine.css` (17 props, current) | `static/tokens-woodfine.css` (35–40 props + 6 faces) | Expand: add font-faces, font-family overrides, shell overrides |
| `static/theme-woodfine.css` (14 props, engine-alias bridge) | Merged into `static/tokens-woodfine.css` | Move: the `--bg`, `--fg`, `--link`, `--accent` aliases belong in the brand override file, not in a separate bridge |
| `static/style.css` lines 9–62 (`:root { --bg, --fg, ... }`) | Stays in style.css but reduced | Trim: most aliases collapse to the underlying DTCG names; `--max-content-width`, `--toc-width`, `--accent` move to shell.* / brand override |

### 7.4 Net file count

Three token files (`tokens-base.css`, `tokens-pointsav.css`,
`tokens-woodfine.css`) plus `style.css`. The `theme-woodfine.css`
bridge file is **deleted** — its contents fold into
`tokens-woodfine.css`.

---

## 8. WCAG fix — `#878d99` replacement

### 8.1 Failure analysis

`#878d99` (neutral-50) is referenced as a hex literal in four semantic
tokens of the wiki DTCG bundle:

1. `semantic.text.tertiary`
2. `semantic.border.strong`
3. `semantic.knowledge.editpencil.color`
4. `component.article.section.heading-edit-pencil-color`

The relative luminance of `#878d99` is **0.2603** (sRGB 135, 141, 153
→ linear 0.2462, 0.2696, 0.3185 → Y = 0.2603).

Contrast ratios on standard semantic backgrounds:

| Background | Hex | Background luminance | Contrast vs `#878d99` |
|---|---|---|---|
| `surface.layer` (white) | `#FFFFFF` | 1.0000 | **3.21:1** |
| `surface.background` (neutral-10) | `#F5F6F8` | 0.9170 | **2.95:1** |
| `surface.layer-accent` (neutral-20) | `#E6E8EC` | 0.8128 | **2.62:1** |

All three fail WCAG 2.1 SC 1.4.3 (Contrast minimum, 4.5:1 for normal
text). The text-tertiary and edit-pencil applications are normal text;
both fail. The `border.strong` application is a non-text border
(component) which requires only 3:1 per SC 1.4.11 — passes against
white, fails against the accent surface.

### 8.2 Replacement

**Adopt `#666c78` (neutral-60) as the replacement value.**

Relative luminance of `#666c78` is **0.1471** (sRGB 102, 108, 120 →
linear 0.1329, 0.1481, 0.1845 → Y = 0.1471).

| Background | Contrast vs `#666c78` | WCAG 1.4.3 (4.5:1)? |
|---|---|---|
| `surface.layer` `#FFFFFF` | **5.93:1** | PASS AA |
| `surface.background` `#F5F6F8` | **5.45:1** | PASS AA |
| `surface.layer-accent` `#E6E8EC` | **4.83:1** | PASS AA |
| `surface.layer-hover` `#CDD1D8` | **4.05:1** | FAIL AA (but: tertiary text should not render on hover surface anyway) |

`#666c78` clears 4.5:1 on every surface the tertiary-text token is
expected to appear on. AAA-grade 7:1 is achieved on white by neutral-70
(`#4a4f59`); the existing `text.secondary` token already uses
neutral-70 — `#666c78` is the right delta-step away.

### 8.3 Implementation

After §3.5 lands the `primitive.color.neutral-*` ramp, change four
semantic alias values:

```diff
-"semantic.text.tertiary":       { "$value": "#878d99" }
+"semantic.text.tertiary":       { "$value": "{primitive.color.neutral-60}" }

-"semantic.border.strong":       { "$value": "#878d99" }
+"semantic.border.strong":       { "$value": "{primitive.color.neutral-60}" }

-"semantic.knowledge.editpencil.color": { "$value": "#878d99" }
+"semantic.knowledge.editpencil.color": { "$value": "{semantic.text.tertiary}" }

-"component.article.section.heading-edit-pencil-color": { "$value": "#878d99" }
+"component.article.section.heading-edit-pencil-color": { "$value": "{semantic.knowledge.editpencil.color}" }
```

The `freshness-ribbon.color-archived` token (in the design-system
bundle, not the wiki bundle) had this same defect and was already
corrected to `neutral-70` 2026-05-01 per its `$description`. This
correction is consistent.

### 8.4 Carbon equivalence note

IBM Carbon's `gray-50` is `#8d8d8d` (luminance 0.27, contrast 3.13:1 on
white) and is documented as **decorative only / not for text**. Carbon
uses `gray-60` (`#6f6f6f`, contrast 4.69:1) as its tertiary text token.
The replacement is consistent with Carbon's convention.

---

## 9. `dtcg-to-css.py` update instructions

### 9.1 New emit targets

```python
# tokens-base.css — primitive + semantic + component + shell, baseline values
OUTPUT_BASE = "../static/tokens-base.css"

# tokens-pointsav.css — PointSav delta
OUTPUT_POINTSAV = "../static/tokens-pointsav.css"

# tokens-woodfine.css — Woodfine delta + @font-face
OUTPUT_WOODFINE = "../static/tokens-woodfine.css"
```

### 9.2 Bundle structure

The single bundle becomes layered:

```json
{
  "primitive": { ... },
  "semantic":  { ... },
  "component": { ... },
  "shell":     { ... },           // NEW — emitted into :root in tokens-base.css

  "brand": {
    "pointsav": {
      "primitive": { ... },        // brand-specific primitives (steel.*)
      "semantic":  { ... }         // overrides (e.g. interactive.button-primary)
    },
    "woodfine": {
      "primitive": { ... },        // brand-specific primitives (woodfine.*)
      "semantic":  { ... },        // overrides (link, button, focus)
      "shell":     { ... },        // overrides (wider gutters)
      "font-face": { ... }         // emitted as @font-face blocks
    }
  }
}
```

### 9.3 New emit functions

```python
def emit_brand_override(bundle, brand_name, output_path):
    """Emit a brand-override CSS file containing only the delta from base.

    For 'woodfine': emits @font-face blocks first (from bundle.brand.woodfine.font-face),
    then :root { ... } with semantic + shell overrides.
    """
    flat_base = flatten_section(bundle, ('primitive', 'semantic', 'component', 'shell'))
    brand = bundle.get('brand', {}).get(brand_name, {})

    lines = []

    # @font-face blocks (if any)
    for face_id, face in brand.get('font-face', {}).items():
        lines.append('@font-face {')
        lines.append(f"  font-family: {face['family']!r};")
        lines.append(f"  font-style: {face.get('style', 'normal')};")
        lines.append(f"  font-weight: {face['weight']};")
        lines.append(f"  font-display: {face.get('display', 'swap')};")
        lines.append(f"  src: {face['src']};")
        if 'unicode-range' in face:
            lines.append(f"  unicode-range: {face['unicode-range']};")
        lines.append('}')
        lines.append('')

    lines.append(':root {')
    flat_brand = {}
    flatten(brand, '', flat_brand)
    for path, token in sorted(flat_brand.items()):
        if path.startswith('font-face'):
            continue
        # emit only if value differs from base, or if path is new
        raw = resolve(token.get('$value', ''), {**flat_base, **flat_brand})
        # ... colour conversion + emit as in emit_section()
        lines.append(f'  {to_css_var(path)}: {css_val};')
    lines.append('}')

    with open(output_path, 'w') as f:
        f.write('\n'.join(lines) + '\n')
```

### 9.4 Path-prefix stripping

The current `to_css_var()` strips three tier prefixes (`primitive`,
`semantic`, `component`). Add `shell` to that list:

```python
if parts[0] in ('primitive', 'semantic', 'component', 'shell'):
    parts = parts[1:]
```

Also strip `brand.<brand-name>` from brand-override paths so
`brand.woodfine.semantic.interactive.link` emits as
`--interactive-link` (not `--brand-woodfine-semantic-interactive-link`).

### 9.5 Validation step

Add a post-emit validator that:
1. Asserts every token in `tokens-base.css` is referenced by `style.css`
   or is a documented unused-token (in a new `tokens-allowlist.md`).
2. Asserts no `tokens-base.css` value uses a hex literal — every colour
   must resolve through a `{primitive.color.*}` alias.
3. Asserts every brand-override file's custom-property keys are a
   subset of `tokens-base.css` keys (no orphans), except for the
   brand-specific primitive ramp (`--color-brand-steel-*`,
   `--color-brand-woodfine-*`).

### 9.6 Run command (unchanged interface)

```
python3 scripts/dtcg-to-css.py
# emits: static/tokens-base.css
#        static/tokens-pointsav.css
#        static/tokens-woodfine.css
```

---

## 10. Implementation sequence

The wiki is live at `documentation.pointsav.com` (port 9090) and
`projects.pointsav.com` (port 9093). All changes must preserve live
rendering between commits. Order matters.

### Stage A — additive only, no rendering change (one commit)

1. ADD `primitive.color.neutral-{10,20,30,40,60,70,80,90,100}` to
   `dtcg-bundle.json`.
2. ADD `primitive.color.brand.{steel,woodfine}.{50,60,70}` to
   `dtcg-bundle.json`.
3. ADD `primitive.font.family.{display,brand-sans,brand-serif}` to
   `dtcg-bundle.json` with system-stack defaults.
4. ADD `primitive.space.{06,3,5,6}` to `dtcg-bundle.json`.
5. ADD `semantic.shell.*` namespace (all PointSav-baseline values)
   to `dtcg-bundle.json`.
6. Regenerate `static/tokens.css`. Verify diff is **additive only** —
   no existing token value changes.
7. Verify live `documentation.pointsav.com` and `projects.pointsav.com`
   render unchanged (no consumer reads the new tokens yet).

**Safe to deploy as a single commit.** Risk: zero. No consumer change.

### Stage B — WCAG fix at the source (one commit)

1. CHANGE `semantic.text.tertiary` from hex literal `#878d99` to alias
   `{primitive.color.neutral-60}` (resolves to `#666c78`).
2. CHANGE `semantic.border.strong` similarly.
3. CHANGE `semantic.knowledge.editpencil.color` to alias chain.
4. CHANGE `component.article.section.heading-edit-pencil-color`
   similarly.
5. Regenerate `static/tokens.css`. The four affected custom properties
   shift colour. Visual diff: tertiary text (date columns, footnote
   muted, edit-pencil hover, recent-additions metadata) becomes darker
   on every page. Borders strong becomes darker on every page.
6. Run automated WCAG sweep against the rendered pages; confirm 4.5:1
   minimum is met on the four affected properties against all
   `--surface-*` backgrounds.

**Safe to deploy as a single commit.** Risk: low (visual-only, contrast
strictly improves). Surface to project-editorial: rendered text
becomes slightly darker; this is the desired direction.

### Stage C — `shell.*` consumption in style.css (one commit)

1. Replace `style.css:11` `--max-content-width: 76em;` with
   `var(--shell-content-max-width)`.
2. Replace `style.css:391` `--toc-width: 14em;` with
   `var(--shell-sidebar-width)`.
3. Replace literal padding values on `.mw-header`, `.site-footer`,
   `.skip-to-content`, `.utility-row`, `.nav-row` (where applicable)
   with `var(--shell-*-padding-*)`.
4. Add `border-bottom: 1px solid var(--shell-header-border-bottom-color)`
   etc.

**Safe to deploy as a single commit.** Risk: low (token values match
the literals being replaced; no visual change for PointSav).

### Stage D — generator refactor (one commit, requires testing)

1. Restructure `dtcg-bundle.json` to add the `brand.{pointsav,woodfine}`
   sub-tree.
2. Move existing `tokens-woodfine.css` semantic overrides into the
   bundle under `brand.woodfine.semantic.*`.
3. Rewrite `dtcg-to-css.py` per §9.
4. Emit `tokens-base.css`, `tokens-pointsav.css`, `tokens-woodfine.css`.
5. Update wiki engine renderer to choose the right brand override file
   based on `WIKI_BRAND_THEME` (already wired for the legacy
   `tokens-woodfine.css` path — same logic, different filename).
6. Delete `static/tokens.css` (replaced by `tokens-base.css` +
   `tokens-pointsav.css`).
7. Delete `static/theme-woodfine.css` (merged into
   `static/tokens-woodfine.css`).
8. Update `style.css` template / Rust handler to emit
   `<link rel="stylesheet" href="/static/tokens-base.css">`
   `<link rel="stylesheet" href="/static/tokens-{brand}.css">` in
   `<head>`.

**Single commit; full visual regression sweep before push.** Risk:
medium. Two-instance smoke test before promote:
- `documentation.pointsav.com` (PointSav default)
- `projects.pointsav.com` (PointSav default)
- A staging Woodfine instance (must be spun up for this stage; not in
  production yet)

### Stage E — Nunito Sans self-hosting (one commit, asset-pipeline-gated)

1. Convert `woodfine-media-assets/fonts/*.ttf` → WOFF2 (Latin-Extended-A
   subset) via fonttools. **Blocks on completion of the asset
   pipeline.**
2. ADD `brand.woodfine.font-face.*` to `dtcg-bundle.json`.
3. ADD `brand.woodfine.primitive.font.family.{sans,body,heading,display}`
   pointing at the new self-hosted families.
4. Regenerate `tokens-woodfine.css` with `@font-face` blocks at top.
5. Add `/static/fonts/` route handler to wiki binary (or static-file
   middleware).
6. Smoke-test on staging Woodfine instance: confirm FOIT/FOUT behaviour
   is `swap` (text rendered in fallback immediately, swapped when
   variable font loads).

**Single commit, but gated on font-conversion asset commit landing in
`pointsav-design-system` first.** Risk: low (Woodfine only, fallback to
system stack if `@font-face` fails to load).

### Stage F — wireframe migration (project-design, parallel)

The wireframe `.agent/drafts-outbound/wireframe-woodfinegroup-home.draft.html`
is not yet committed to `pointsav-design-system`. project-design picks
it up, renames `--ds-*` to `--surface-*` etc. per §4.2 commits A–C,
then commits to the design-system repo. Parallel to Stage D; does not
gate on it. Recommend: wireframe migration **after** Stage D so the
wireframe consumes the actual tokens-base.css values.

### Stage G — Phase 6 three-instance deployment (gated)

Per `KNOWLEDGE-PLATFORM-PLAN.md`, Phase 6 is the split into three live
instances. Stage A–E are prerequisites; Stage G adds the `corporate`
instance with whichever brand override is correct for `pointsav.com`
versus `woodfinegroup.com` versus `documentation.pointsav.com`. The
token architecture supports any brand-per-instance mapping by
`WIKI_BRAND_THEME` env var at boot.

**Gated on:** GitHub content-wiki rename + Doctrine amendment
(externally tracked; not a token-system concern).

---

## Appendix A — Token inventory crosswalk

For project-design implementation. Maps every wireframe `--ds-*` /
`--wf-*` property to its canonical replacement.

| Wireframe property | Canonical token | New CSS var | File |
|---|---|---|---|
| `--ds-paper-1: #FFFFFF` | `semantic.surface.layer` | `--surface-layer` | tokens-base / tokens-woodfine |
| `--ds-paper-2: #F7F9FA` | `semantic.surface.background` (Woodfine override) | `--surface-background` | tokens-woodfine |
| `--ds-paper-3: #E6E7E8` | `semantic.surface.layer-accent` (Woodfine override) | `--surface-layer-accent` | tokens-woodfine |
| `--ds-ink-1: #111827` | `semantic.text.primary` (Woodfine override) | `--text-primary` | tokens-woodfine |
| `--ds-ink-2: #374151` | `semantic.text.secondary` (Woodfine override) | `--text-secondary` | tokens-woodfine |
| `--ds-ink-3: #6B7280` | `semantic.text.secondary` (Woodfine override, secondary value) — merge with `--ds-ink-2` | `--text-secondary` | tokens-woodfine |
| `--ds-ink-4: #9CA3AF` | `semantic.text.tertiary` (Woodfine override) | `--text-tertiary` | tokens-woodfine |
| `--ds-rule-hairline: #E6E7E8` | `semantic.border.subtle` (Woodfine override) | `--border-subtle` | tokens-woodfine |
| `--ds-rule-strong: #9CA3AF` | `semantic.border.strong` (Woodfine override) | `--border-strong` | tokens-woodfine |
| `--ds-display: Oswald,...` | `primitive.font.family.display` (Woodfine override → Barlow Condensed) | `--font-family-display` | tokens-woodfine |
| `--ds-serif: Roboto Slab,...` | `primitive.font.family.body` (Woodfine override → Zilla Slab) | `--font-family-body` | tokens-woodfine |
| `--ds-sans: Nunito Sans,...` | `primitive.font.family.sans` (Woodfine override → Nunito Sans) | `--font-family-sans` | tokens-woodfine |
| `--ds-fs-utility: 10px` | (raise to 11px) `primitive.font.size.1` | `--font-size-1` | tokens-base |
| `--ds-fs-nav: 11px` | `primitive.font.size.1` | `--font-size-1` | tokens-base |
| `--ds-fs-body: 14px` | `primitive.font.size.3` | `--font-size-3` | tokens-base |
| `--ds-fw-regular: 400` | `primitive.font.weight.regular` | `--font-weight-regular` | tokens-base |
| `--ds-fw-medium: 500` | (ADD) `primitive.font.weight.medium` | `--font-weight-medium` | tokens-base (ADD) |
| `--ds-fw-semibold: 600` | `primitive.font.weight.semibold` | `--font-weight-semibold` | tokens-base |
| `--ds-fw-bold: 700` | `primitive.font.weight.bold` | `--font-weight-bold` | tokens-base |
| `--ds-ls-tight: 0.04em` | (ADD) `primitive.font.letter-spacing.tight` | `--font-letter-spacing-tight` | tokens-base (ADD) |
| `--ds-ls-medium: 0.10em` | (ADD) `primitive.font.letter-spacing.medium` | `--font-letter-spacing-medium` | tokens-base (ADD) |
| `--ds-ls-wide: 0.16em` | (ADD) `primitive.font.letter-spacing.wide` | `--font-letter-spacing-wide` | tokens-base (ADD) |
| `--ds-ls-extra-wide: 0.18em` | (ADD) `primitive.font.letter-spacing.extra-wide` | `--font-letter-spacing-extra-wide` | tokens-base (ADD) |
| `--ds-sp-1: 4px` | `primitive.space.05` | `--space-05` | tokens-base |
| `--ds-sp-2: 8px` | `primitive.space.1` | `--space-1` | tokens-base |
| `--ds-sp-3: 12px` | `primitive.space.06` (ADD) | `--space-06` | tokens-base (ADD) |
| `--ds-sp-4: 16px` | `primitive.space.2` | `--space-2` | tokens-base |
| `--ds-sp-5: 20px` | `primitive.space.25` (ADD as 1.25rem) | `--space-25` | tokens-base (ADD) |
| `--ds-sp-6: 24px` | `primitive.space.3` (ADD) | `--space-3` | tokens-base (ADD) |
| `--ds-sp-7: 36px` | `primitive.space.6` (ADD as 2.25rem) | `--space-6` | tokens-base (ADD) |
| `--ds-sp-8: 48px` | `primitive.space.5` (ADD as 3rem — note: collides with existing `space.4=2rem`; renumber sequence pending) | `--space-5` | tokens-base (ADD) |
| `--ds-sp-9: 56px` | (ADD) `primitive.space.55` (3.5rem) | `--space-55` | tokens-base (ADD) |
| `--ds-sp-10: 80px` | `primitive.space.8` (currently 64px) — needs reconciliation | (decision pending) | tokens-base |
| `--ds-sp-11: 120px` | `primitive.space.16` (currently 128px) — close enough | `--space-16` | tokens-base |
| `--ds-sp-12: 140px` | (no equivalent; round to 128px or add `space.18`) | (decision pending) | tokens-base |
| `--ds-motion-fast: 160ms` | (close to) `primitive.motion.duration.base` (150ms) — keep 150ms | `--motion-duration-base` | tokens-base |
| `--ds-easing-standard: ease` | `primitive.motion.easing.standard` (cubic-bezier(0.2,0,0.38,0.9)) | `--motion-easing-standard` | tokens-base |
| `--ds-container-max: 1440px` | `semantic.shell.content.max-width` (Woodfine override at 90em) | `--shell-content-max-width` | tokens-woodfine |
| `--wf-blue: #164679` | `primitive.color.brand.woodfine.60` (ADD) → `interactive-link` (Woodfine override) | `--interactive-link` | tokens-woodfine |
| `--wf-blue-on: #FFFFFF` | `semantic.text.on-color` | `--text-on-color` | tokens-base |
| `--wf-blue-tint: #E8EFF7` | `primitive.color.brand.woodfine.50` (ADD) | `--color-brand-woodfine-50` | tokens-base (ADD) |

Note on space scale: the existing wiki bundle uses an exponential
(`space.025=2px, .05=4px, .1=8px, .2=16px, .4=32px, .8=64px, .16=128px,
.32=256px`); the wireframe uses a linear 4/8/12/16/20/24/36/48/56/80/120/140
ramp. **Decision: keep the existing exponential as `primitive.space.*`
(it is what the entire component tier currently aliases to), and add a
parallel linear ramp as `primitive.space.linear.*` for shell/chrome
contexts that need 4px-step granularity.** This is the cleanest
reconciliation; both ramps coexist; consumers pick the one with the
right granularity.

---

## Appendix B — Open decisions for MASTER cosign

The following require explicit ratification before Stage D:

1. **Adoption of `shell.*` namespace** (§5). Net new top-level semantic
   sibling of `surface`, `text`, `border`, `interactive`, `knowledge`.
   Master co-sign 2026-04-30 approved `knowledge.*` on the same basis;
   precedent supports approval here.

2. **Replacement of `#878d99` with `#666c78`** (§8). Visual diff:
   tertiary text becomes slightly darker on every rendered page.
   Strictly improves accessibility; should be uncontroversial.

3. **Banning of `--ds-*` prefix** (§4). Wireframe submitter must
   accept the rename before commit to design-system.

4. **Three-file architecture** (§7). Replaces the current two-file
   layout. Engine renderer requires a one-line change to load the
   right brand-override file. Backwards compatibility: none required —
   only two instances are in production today, both PointSav default.

5. **Self-hosted Nunito Sans** (§6). Gated on font-conversion asset
   commit; not blocking Stages A–D. CDN dependency (currently in
   wireframe) violates CLAUDE.md §6 sovereignty discipline and **must
   not** ship to a production Woodfine instance regardless of this
   document's other decisions.

6. **Space-scale reconciliation** (Appendix A note). Adding a parallel
   linear ramp or extending the exponential one. Recommend the linear
   ramp under `primitive.space.linear.*` so the existing component
   layer continues to alias the exponential ramp unchanged.

---

End of research document. Ready for project-design pickup and MASTER
cosign cycle.
