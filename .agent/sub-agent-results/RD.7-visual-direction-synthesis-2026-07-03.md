# RD.7 — Visual Direction Synthesis: the Decision Spec

**Date:** 2026-07-03
**Inputs:** RD.1 (live-site audit), RD.2 (pre-wiki git archaeology), RD.3 (design.pointsav.com audit), RD.4 (BB.13/BB.14 extraction), RD.5 (hyperscaler re-survey), RD.6 (AEC precedent survey).
**Status:** This is a decision, not a menu. Where BB.14 §6 (RD.4 §B.7) already froze a value and nothing since has invalidated it, that value is restated frozen. Where the July context (4-section IA, dark mode shipped, utility bar/search dropped, sibling palette drift) changes the problem, this document decides.

---

## 0. Decision summary

**bim.woodfinegroup.com becomes an instrument-panel catalog in the Adobe Spectrum docs-chrome grammar — Source Serif 4 display / Geist Sans UI / Geist Mono data, `#1A4480` drafting blue (confirmed, not adjusted), a single 48px "title-block" header (utility bar and in-header search deleted), a 272px four-section collapsible sidebar tree, and the five AEC structural markers baked into templates.** The footer — content, badges, trademark paragraph, permanently-dark treatment — does not change. Dark mode survives, re-palettized from wiki near-black to BB.13's desaturated instrument navy. Implementation is CSS + font assets + moderate `shell.rs` template surgery; no architecture change.

What this reconciles:

| Source judgment | Disposition here |
|---|---|
| RD.1 §3a wiki-borrowed chrome (utility bar, in-header search, light-header-as-wiki-pattern, accent-left-border card/nav hover convention, mobile network drawer group) | **Removed or replaced.** The left-accent bar survives in exactly one place: sidebar active-item indicator (that one is Spectrum grammar too). |
| RD.1 §3b BIM-native elements (classification chips, AEC status colors, spec-card/token-table system, key-plan swatches, footer *content*, sidebar skeleton) | **Kept, restyled** onto the new type/palette. |
| RD.1 §4 footer + badges + trademark; disclosure band | **Unchanged** (operator-validated; §7 below). |
| Theme toggle + FOUC script + full dark mode (operator-prompted per RD.2 `7ae34ad1`) | **Kept** — BB.14 predates this; the new spec supersedes BB.13's "defer dark mode" note because dark mode has since shipped at operator request. Dark palette re-derived in §6.2. |
| BB.14 §6 frozen values (type spine, `#1A4480`, 272px sidebar, 896px content, radius/spacing scales, tab-band chrome, 5 markers) | **Adopted as frozen**, with the two adaptations BB.14 never covered: 4-section sidebar (§4) and header-without-search (§3). |
| RD.5 riders (don't chase Spectrum 2 roundness; navy-collision flag; sibling baseline is now IBM Plex + `--cds-*`) | **Applied** — S1-lineage docs chrome only; color decision in §1. |
| RD.6 (iTwinUI corroboration; no displacement) | **Applied as corroboration only.** iTwinUI's soft categorical palette is queued as the future systematization path for chip families (§5, marker 2 note), not adopted now. |

---

## 1. Color decision: `#1A4480` confirmed — with an interactive-state weighting rule

**Decision: keep `#1A4480` drafting blue. No hex adjustment.** Reasoning:

1. **The collision is narrower than it looks.** The sibling's `#0e3a66` appears only as *link text and selected-text color*; its dominant interactive signals are bright Carbon blues (`#0050e6` interactive, `#0f62fe` focus) on a `#161616` dark header. BIM's thumbnail color story is not "navy vs navy" — it is *one disciplined dark-blue accent on warm off-white with no dark header band* vs *a two-blue Carbon system under a black band*. Those compositions read differently at 200×150px even where individual hexes are cousins.
2. **The semantics are the point.** `#1A4480` was derived from the US-federal/NBS drafting-documentation range (`#0A5494`–`#1F5E9B`, BB.13 §A.4/A.5) and clears ~7.8:1 on `#FAFBFC`. Any adjustment large enough to matter at thumbnail scale leaves that range — drifting either toward Carbon blue-60 territory (collides with the sibling's focus color) or toward the MEP cyan slot (collides with our own chip semantics).
3. **RD.5 already re-scored the pick at 14/15 with this flag priced in** (Color 5→4, Typography 4→5, Chrome 5). The load-bearing distinguishers are now typography and chrome, and this spec deliberately weights them: serif display headings, the `#EFEFEF`/`#E8E8E8` panel chrome, and the AEC markers do the thumbnail work; color corroborates.

**The weighting rule (normative, per RD.5's mitigation):** `#1A4480` must be the *only* interactive accent on the site — buttons, active-nav left border, active tab underline-free lift, focus rings, link text, chip accents, hero SVG stroke. **The focus ring is `#1A4480` (2px, 2px offset), never `#0f62fe`.** No second bright blue anywhere. The sibling shows navy text + bright-blue interaction; BIM shows drafting-blue everything. Single-accent discipline is itself a signature.

**Escape hatch (record, don't act):** if the operator judges the navy separation insufficient on first side-by-side render, the one-line fallback is `#205493` — brighter, still inside the researched drafting range, ~5.9:1 on `#FAFBFC` (AA all sizes). Do not pre-emptively take it; it trades authority for separation we expect not to need.

---

## 2. Chrome concept: the header is a title block

The organizing idea for the header (and the answer to "what fills the space where the utility bar and search lived"): **an architectural drawing's title block** — the strip on every drafting sheet carrying project identity and governing standards, and nothing else. No navigation to sibling properties (that lives in the footer's Network column, which already exists and stays), no search box, no utility strip.

### Header spec (single bar, replaces utility bar + 64px header — chrome height drops 96px → 48px)

| Property | Value |
|---|---|
| Element | `header.bim-header`, sticky, `height: 48px`, `z-index` above sidebar |
| Background | `--bim-bg-surface` (`#FFFFFF` light / `#242E42` dark), `border-bottom: 1px solid var(--bim-border)` |
| Contents, left → right | ① hamburger button (visible ≤1056px only, unchanged mechanism) · ② Woodfine wordmark SVG (existing `wordmark_svg()` helper, `currentColor`, height 20px; renders `#1A4480` light / `#E8EDF5` dark) · ③ 1px vertical divider (`--bim-border`, 20px tall, 12px margins) · ④ descriptor `BIM OBJECT LIBRARY` — Geist Mono 11px, 500, uppercase, `letter-spacing: 0.08em`, `--bim-fg-muted` · ⑤ flexible spacer · ⑥ standards line `IFC 4.3 · ISO 16739-1:2024 · DTCG` — Geist Mono 10.5px, `--bim-fg-faint`, static text (links nowhere), hidden ≤768px · ⑦ theme toggle (existing sun/moon button, 32×32px hit area, 4px radius, retained) |
| Deleted | `.bim-utility` block entirely (all classes, CSS, and the ≤768px `.bim-nav-group--mobile-only` "Woodfine Network" sidebar fallback — the footer Network column is the sole remaining cross-property surface, per operator instruction) · `.bim-search` form and all `bim-search__*` classes/CSS |

**Why a light header when RD.1 tags "light header" as wiki-borrowed:** the wiki-borrowed thing was the *ensemble* (light header + utility strip + in-header search). A light 48px bar with a bottom hairline is also exactly Spectrum's docs-chrome grammar — and it is the anti-sibling move: design.pointsav.com opens with a black `#161616` band; BIM opens with none. At thumbnail scale that is the strongest single chrome differentiator available, and it costs nothing.

**Search disposition:** the server-side `/search` route (RD.2 `472eb451`) stays live and functional — it is ~free, deep-linkable, and useful to agents/tests — but has **no chrome entry point**. A 24-item catalog fully visible in a sidebar tree does not need a search box; re-adding one (as a Ctrl-K affordance) is a future call gated on catalog growth, not part of this direction.

---

## 3. Layout skeleton

| Property | Value |
|---|---|
| Header | 48px sticky (above) |
| Sidebar | **272px** fixed/sticky below header, own `overflow-y: auto`, bg `--bim-bg-sidebar` (`#EFEFEF`), `border-right: 1px solid var(--bim-border)` |
| Content | `max-width: 896px`, horizontal padding 32px, `margin-left: 272px` desktop |
| Full desktop | 272 + 896 + paddings ≈ 1250px comfortable at 1280+; content flexes below |
| Breakpoint ladder | **Retain the tested ladder** 1056 / 768 / 600 / 480 / 420 (RD.1 §2), minus every rule that referenced the deleted utility bar and search. ≤1056px: sidebar becomes the existing off-canvas drawer (same content, same toggle). Do not re-derive breakpoints — the mobile pass was verification-driven (RD.2 `19979562`) and the deletions only simplify it. |
| Footer / disclosure band | Keep existing `margin-left: 272px` offset treatment (was 16rem = 256px; update the one value) |

---

## 4. The four-section sidebar (the new design problem — solved here)

BB.13/BB.14 speced a flat ~20-leaf categorical tree; the approved IA is now 4 sections + Research. The sidebar becomes a **section tree with native `<details>` disclosure** — no JS required to operate it, SSR sets state.

### Structure (top → bottom)

```
[Top cluster — ungrouped]
  Overview                    → /
  Browse All BIM Objects      → /tokens
  About                       → /about
──────────────────────────── (1px divider, 12px vertical margin)
▾ TAXONOMY              10    ← <details open><summary>
    Spatial
    Elements
    Systems
    Materials
    Assemblies
    Performance
    Identity + Codes
    Relationships
    Professional Office Subtypes
    Building Width Calculator
▾ OBJECTS                6
    Key Plans · Amenity Key Plan · Retail Select ·
    Tech Industrial · Interior · Furniture        (one per row)
▾ COMPOSITIONS           5
    Tile System · Floor Plate Standards · Floor Plate
    Assembly Rules · Building Grid · Tenant Mix    (one per row)
▾ CONTEXT                3
    Climate Zones · Landscape + Parking · Water Management
──────────────────────────── (divider)
[Bottom cluster — top-level leaves]
  Key Plan Diagrams           → /key-plans
  Furniture Library           → /furniture
  Research                    → /research
```

### Exact treatment

| Element | Spec |
|---|---|
| Section header (`<summary>`) | Geist Sans **11px / 600 / uppercase / letter-spacing 0.08em**, color `--bim-fg-muted`; padding `8px 16px`; hover bg `#E5E9EF` (dark: `#202839`); `cursor: pointer`; native marker suppressed |
| Section count | Right-aligned within the summary row, **Geist Mono 10px**, `--bim-fg-faint` — the catalog states its inventory (`TAXONOMY  10`). Rendered from the vault count server-side |
| Chevron | 10px stroke chevron at far right after the count, `rotate(0)` closed → `rotate(90deg)` open, `transition: transform 120ms` |
| Leaf link | Geist Sans **13px / 400**, color `--bim-fg-secondary`, padding `6px 16px 6px 28px` (indented under section); hover: bg `#E5E9EF`, no border movement |
| Active leaf | `border-left: 2px solid var(--bim-accent)` (padding-left drops to 26px to hold alignment), bg `--bim-accent-subtle`, color `--bim-accent`, weight 600, `aria-current="page"`. This is the **only** surviving left-accent-bar treatment on the site |
| Active-section highlight | The `<summary>` of the section containing the current page renders at `--bim-fg` (full strength, not muted) — so the owning section is identifiable even when the list is scrolled or the section is collapsed |
| Top/bottom cluster links | Same leaf style, un-indented (`padding-left: 16px`) |

### Collapse/expand behavior

- **All four sections `open` by default, server-rendered.** A catalog shows its inventory; 33 total rows ≈ 950px scrolls gracefully inside the sidebar's own scroll region (Spectrum's and Carbon's sidebars scroll; this is normal). Auto-collapsing inactive sections would make the site feel smaller than it is — the opposite of the redesign's purpose.
- Users collapse per-visit via native `<details>`; **no persistence in v1** (localStorage persistence is a ≤10-line future add inside the Option B script budget if the operator asks; not speced now).
- SSR guarantees the section containing the current route carries `open` regardless of default (idempotent today since all default open; future-proofs a collapsed-default flip).
- No-JS behavior: identical (native element).
- Mobile drawer (≤1056px): same tree, same disclosure behavior, inside the existing off-canvas panel.

---

## 5. Page anatomies + the five AEC markers (exact placement)

### Homepage
1. **Hero row** — left: headline in **Source Serif 4, 36px/400** (`--bim-text-display`; the serif at the hero *is* the typographic identity move, per BB.14 §9 open question — adopt, with the known one-line revert), lead paragraph Geist Sans 16px, stat line `24 CATEGORIES · 18 COMPONENTS · 3 RESEARCH ENTRIES` in Geist Mono 12px uppercase `--bim-fg-muted`. Right: **[MARKER 4] isometric building-mass hero SVG** — inline, compile-time embedded, ~280×240px, floor plates stacked with grid facade strokes, `stroke: var(--bim-accent)` 1.5px on a `--bim-bg-sidebar` panel, 8px radius. Replaces any count-block/flat hero.
2. **Four section panels, 2×2 grid** (1-col ≤768px): each panel = section name Geist Sans 15px/600 + count + its category links as a compact 13px list + "View section →" in `--bim-accent`. Card treatment: `1px solid var(--bim-border)`, **6px radius**, `--bim-bg-surface`, hover = border-color `--bim-accent` + `box-shadow: 0 2px 8px rgba(16,24,40,0.08)`. **The wiki accent-left-border card convention is dropped** (RD.1 §3a).

### Section landing pages (Taxonomy / Objects / Compositions / Context)
Category-card grid, 2-col desktop / 1-col ≤600px. Each card: category name 15px/600, entity count, and **[MARKER 3] its IFC anchor entity in Geist Mono 11px `--bim-fg-muted`** (`IfcSpatialElement`, `IfcBuiltElement`, …) linking to the bSDD/buildingsmart URI. Card chrome as homepage panels. (This retires the flat 24-card homepage grid; density problem BB.14 flagged in §B.9 is resolved by the sectioning itself — max 10 cards per landing.)

### Category pages (`/tokens/*`)
1. Breadcrumb `Section / Category` — Geist Sans 12px, `--bim-fg-muted`, plain `/` separators. (BB.13's `═` storey-glyph breadcrumb applies only to spatial-hierarchy surfaces, i.e. Key Plans pages — use it there, not globally.)
2. `h1` — **Source Serif 4, 30px/600**.
3. **[MARKER 2] Classification chip row**, directly under the h1 (the existing BIM-native chip row, restyled): label prefix Geist Mono 11px; `IFC <entity>` chip on `#E8EEF8`/`--bim-accent` text; `UNICLASS <code>` chip on `#ECFEFF`/`#0E7490`; standards chip `IFC 4.3 · ISO 16739-1:2024` outline-muted; `DTCG` outline-muted; constraint chips where applicable on `#FFF8ED`/`#B54708`. Radius 4px, padding 2px 8px. *Future note (RD.6): if chip families grow past three color slots, systematize on an iTwinUI-style named soft categorical palette rather than ad-hoc additions.*
4. **Tab bar** — the Spectrum instrument band and the page's signature chrome element: full-content-width strip, bg `--bim-bg-panel` (`#E8E8E8` light / `#202A3E` dark), `border-bottom: 1px solid var(--bim-border)`. Tabs: Geist Sans 13px/500, padding 10px 16px; **active tab lifts** — `--bim-bg-page` fill, 1px side/top borders, no bottom border, 4px top radius (classic panel tab, no underline). Tab set per category from its existing sections: `Specification / BIM Objects / Regulation / Token Format` (only those that exist). Mechanism: content SSR'd in full, Option B inline JS applies `role="tablist"` switching; **no-JS fallback = sections stack with anchor links** (current accordion content reused, chrome swapped).
5. Spec tables (`.bim-detail-table`, `.bim-token-table`, restyled): **0 radius on cells** (AEC data-table convention), header row bg `--bim-bg-sidebar`, labels Geist Sans 13px, values that are identifiers/codes in **Geist Mono 12px**. **[MARKER 1] IFC GUID monospace:** the Specification/Identity table on every category page carries an `Example GlobalId` row — `2O2Fr$t4X7Zf8NOew3FL_A` in Geist Mono 11px `--bim-fg-muted`. Structural (rendered by the table template), not optional.
6. Token Format panel: raw DTCG JSON in a dark code block — bg `#1A1A1A` **in both themes**, Geist Mono 13px, copy button top-right (existing Option B budget), fg at 100/80/60% opacity tiers for value/key/punctuation differentiation (no highlight library).

### Key Plans / diagram surfaces
**[MARKER 5] Dark viewport preview frame:** `/key-plans` diagrams and visual previews on spatial categories (Key Plans, Tile System, Building Grid, Floor Plate pages) render inside `data-preview-theme="dark"` frames — bg `#1C2333`, `1px solid #2E3D5A`, 8px radius, caption row carrying the plan name + a sample GUID in Geist Mono 11px. **The frame stays dark in light mode** — that inversion (dark viewport in light documentation) is the AEC authoring-tool signature (BB.13 §A.4). Existing key-plan category swatch fills are retained inside the frames. Breadcrumbs on these pages use the `═` storey separator.

---

## 6. Exact value tables

### 6.1 Typography

| Token | Value |
|---|---|
| `--bim-font-display` | `'Source Serif 4', 'Source Serif Pro', Georgia, 'Times New Roman', serif` — h1/h2 + hero only. Weights: 400 page titles/hero, 600 section headings |
| `--bim-font-sans` | `'Geist', 'Geist Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI Variable', 'Segoe UI', Helvetica, Arial, sans-serif` — everything else, 16px body |
| `--bim-font-mono` | `'Geist Mono', ui-monospace, 'SFMono-Regular', Menlo, Consolas, monospace` — GUIDs, chips labels, codes, JSON, counts, standards line |
| Scale | `--bim-text-xs: 0.6875rem` (11px — GUIDs, chip text, IFC anchors, section headers) · `--bim-text-sm: 0.8125rem` (13px — sidebar, tabs, tables, code) · `--bim-text-base: 1rem` (16px body) · `--bim-text-lg: 1.125rem` (18px lead) · `--bim-text-xl: 1.25rem` (20px h3) · `--bim-text-2xl: 1.5rem` (24px h2, serif) · `--bim-text-3xl: 1.875rem` (30px h1, serif) · `--bim-text-display: 2.25rem` (36px hero, serif) |
| Line heights | 1.25 tight (headings) / 1.55 normal (body) / 1.6 prose |
| Removed | Oswald, Nunito Sans, Roboto Slab `@font-face` sets and their fallback stacks (the corporate-home pairing was the right call against the wiki's Inter, but the catalog identity now diverges deliberately from home.woodfinegroup.com per the BB.14 bankers'-test mandate — this is the visual-direction change `tokens.css`'s own header comment says it replaced) |

Self-hosted woff2 via the existing `fonts.css` mechanism, **Latin-subset**: Geist Sans ~280KB + Geist Mono ~180KB + Source Serif 4 (400/600 only, not the full 200–900 axis) ≈ **350–450KB combined subset** — roughly payload-neutral against the three families it replaces. All OFL 1.1.

### 6.2 Color

**Light (`:root`):**

| Token | Value | Token | Value |
|---|---|---|---|
| `--bim-bg-page` | `#FAFBFC` | `--bim-fg` | `#101828` |
| `--bim-bg-surface` | `#FFFFFF` | `--bim-fg-secondary` | `#344054` |
| `--bim-bg-sidebar` | `#EFEFEF` | `--bim-fg-muted` | `#667085` |
| `--bim-bg-panel` | `#E8E8E8` | `--bim-fg-faint` | `#98A2B3` |
| `--bim-bg-code` | `#1A1A1A` (both themes) | `--bim-border` | `#D0D5DD` |
| `--bim-accent` | **`#1A4480`** (~7.8:1 on page bg) | `--bim-border-subtle` | `#E5E7EB` |
| `--bim-accent-hover` | `#133360` | `--bim-border-strong` | `#98A2B3` |
| `--bim-accent-active` | `#0F2848` | `--bim-accent-subtle` | `#E8EEF8` |
| Focus ring | `2px solid #1A4480`, offset 2px — **never `#0f62fe`** | Sidebar hover | `#E5E9EF` |

**Status colors (brand-governed set retained — RD.1 §3b):** `--bim-safe: #54924E` (kept over BB.14's `#027A48` — the existing value is operator-brand-governed via `theme-woodfine.css` and the delta is invisible; the other three already agree), `--bim-warning: #B54708` / bg `#FFF8ED`, `--bim-mep: #0E7490` / bg `#ECFEFF`, `--bim-error: #B42318` / bg `#FEF3F2`.

**Dark (`:root[data-theme="dark"]`) — desaturated instrument navy, promoted from BB.13's preview-frame palette to full page.** Rationale: dark mode is shipped and operator-prompted (RD.2 `7ae34ad1`) so it stays, but the current `#0F1218` near-black is the wiki-era palette *and* sits next to the sibling's `#161616` Carbon dark. The navy-cast instrument set is already researched ("technical instrument rather than code editor"), harmonizes with the always-dark viewport frames, and keeps the two products distinguishable in dark mode too:

| Token | Value | Token | Value |
|---|---|---|---|
| `--bim-bg-page` | `#1C2333` | `--bim-fg` | `#E8EDF5` |
| `--bim-bg-surface` | `#242E42` | `--bim-fg-secondary` | `#C3CBD9` |
| `--bim-bg-sidebar` | `#1A2030` | `--bim-fg-muted` | `#8B96A8` |
| `--bim-bg-panel` | `#202A3E` | `--bim-fg-faint` | `#64718A` |
| `--bim-accent` | `#5E9BDE` (BB.13's `#4A90D9` lightened one step; ~5:1 on `#1C2333`, AA small text) | `--bim-border` | `#2E3D5A` |
| `--bim-accent-hover` | `#7FB0E8` | `--bim-border-subtle` | `rgba(232,237,245,0.06)` |
| `--bim-accent-active` | `#A3C6EF` | `--bim-border-strong` | `#43547A` |
| `--bim-accent-subtle` | `rgba(94,155,222,0.16)` | Sidebar hover | `#202839` |

Footer and dark code blocks remain **permanently dark, un-toggled** in both themes (existing convention, kept). The FOUC-avoidance inline script and `/edit/*` light-hardcode are unchanged.

### 6.3 Spacing, radius, misc

| Scale | Values |
|---|---|
| Spacing | 4px base, 10 stops: 4 / 8 / 12 / 16 / 20 / 24 / 32 / 40 / 48 / 64px |
| Radius | **0** — table cells, property-panel rows · **4px** — chips, badges, inputs, tabs (top corners), theme toggle · **6px** — buttons, cards/panels on grids · **8px** — preview frames, hero SVG panel, modals |
| Shadows | One hover tier only: `0 2px 8px rgba(16,24,40,0.08)` (dark: `rgba(0,0,0,0.35)`). No elevation system |

---

## 7. What stays unchanged (normative)

1. **The footer, in full** (RD.1 §4): three columns (library facts + licenses / machine-readable surface / **Network** — the Network column is the operator-sanctioned footer home for `home.woodfinegroup.com` and sibling links, satisfying "at most a single unobtrusive link… in the footer" and then some, already approved), cities/badges base row, **the three badges exactly as-is** (PointSav glyph, Apache-2.0 link, CC BY-ND 4.0 with the official cc/cc-by/cc-nd SVG marks), copyright line, persistent one-line disclaimer, **the trademark paragraph verbatim**. Permanently-dark `#111827` treatment kept. The only footer edits permitted: the sidebar-offset value (256→272px) and the global body-font inheritance (Nunito Sans → Geist Sans) — content, structure, colors, and badge treatment are frozen.
2. **The Important Information disclosure band** — compliance content built against Command's ratified spec (RD.2 `68e3406f`), git-owned markdown source. Keeps its `<details>` structure; inherits new fonts; offset value updated; nothing else.
3. **Routes, SSR architecture, vault, systemd, nginx** — untouched. `/search` stays live (chrome entry removed only).
4. **Theme toggle, FOUC script, `/edit/*` light-hardcode, hamburger/drawer mechanism, breakpoint ladder.**
5. **AEC status color values** (brand-governed, §6.2).

---

## 8. Implementation cost (BB.14 §8-style delta)

**Classification: CSS + font-asset change plus moderate template surgery in one file (`shell.rs`) — not an architecture change.** Larger than BB.14's pure-CSS estimate because chrome structure changes (utility-bar deletion, sidebar sectioning, tab band), but no new routes, no new crates, no bundler, no JS beyond the existing Option B budget.

| File | Change | Rough size |
|---|---|---|
| `src/assets/fonts.css` | Swap `@font-face` set + family vars; drop Oswald/Nunito/Roboto Slab woff2, add Geist Sans/Geist Mono/Source Serif 4 subsets | ~same 80 lines; assets ±0–100KB net |
| `src/assets/tokens.css` | Full light+dark palette, type scale, spacing/radius vars per §6 | 87 → ~130 lines, rewrite |
| `src/assets/bim-layout.css` | Delete utility-bar + search + mobile-network blocks (~150 lines); header rework (~50); sidebar section tree (~80); hero + section panels (~60); tab band (~40); offset value updates | net 1114 → ~1000 lines; ~350 touched |
| `src/assets/bim-components.css` | Chip recolor/retype, table restyle (0-radius cells, mono values), GUID row class, dark preview frame | ~60-line delta |
| `src/render/shell.rs` | Remove utility bar + search form; title-block header; sidebar `<details>` section generation from the 4-section IA mapping (section → category list, counts) | ~150 lines changed |
| `src/render/` content fns (home, section landings, category pages) | Hero SVG (~35 lines inline path data, compile-time), 2×2 section panels, section-landing card grids w/ IFC anchor labels, tab-bar markup around existing accordion content, GUID table row | ~150–200 lines |
| `src/assets/bim.js` | Keep theme toggle; add `role="tablist"` switcher (~25 lines); delete any search wiring | small |
| `carbon.min.css` / `carbon.esm.js` / `carbon-overrides.css` | **Scope to `/edit/*` routes only** — public catalog pages stop shipping IBM Carbon CSS entirely (both a payload win and the removal of literal Carbon lineage from the public surface). Requires one audit pass for public-page rules leaning on Carbon resets | ~10 lines in `shell.rs` head emission + audit |

**Estimated total: ~700–900 lines touched across 7 files, one to two Totebox sessions, then the normal local-bim (9096) → operator preview → Command `push-to-prod.sh bim` path.** New font-loading cost ≈ neutral after subsetting (§6.1). No `Cargo.toml` change.

## 9. Risks / first-render checkpoints

1. **Serif at hero/h1 (Medium, known):** confined to h1/h2/hero; one-line CSS revert per heading level if rejected on first render.
2. **Navy separation (Low, priced in):** verify with the actual side-by-side thumbnail test against live 9094 after first render; escape hatch `#205493` (§1) only if it fails.
3. **Carbon-CSS scoping regression (Low-Medium):** public pages may silently depend on Carbon resets; audit pass required before removing from public head.
4. **Tab no-JS fallback (Low):** anchors + stacked sections must remain readable — test with JS disabled once.
5. **All-open sidebar height (Low):** 33 rows scroll inside the sidebar; if the operator finds it long, per-section collapsed defaults are a server-side one-liner (state mechanism already speced).
