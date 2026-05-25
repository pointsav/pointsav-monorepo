# Visual Design System — Tiers, Rings & Retailer-First Map

**Owner:** project-gis Totebox session (research); operator (gate decisions).
**Drafted:** 2026-05-20 from four parallel Opus research agents.
**Status:** Research complete. No implementation started.
**Relationship to prior plans:**
- `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md` — upstream cluster model
- `RING-HIERARCHY-DESIGN-2026-05-20.md` — ring architecture (locked palette)
- `tier-scoring-overhaul-2026-05-16.md` — tier names (G1) and BentoBox layouts

---

## CRITICAL: Live Palette Conflicts (must fix before any visual release)

The current live `index.html` dot and delta colors **violate the locked BLUE=containment / ORANGE=reach / SLATE=limit rule**:

| # | Element | Current live value | Conflict | Severity |
|---|---|---|---|---|
| C1 | Anchor/hypermarket dot | `#0A3070` | **Identical to T1 co-location ring** — anchor is invisible inside its own ring | High |
| C2 | Hardware dot | `#D97706` | Burnt amber — same hue family as catchment orange `#f97316` — reads as catchment artifact | High |
| C3 | Delta "Unchanged" | `#2563eb` | Blue — on Alberta sim, delta ring is indistinguishable from co-location ring | High |
| C4 | Tier badge "Local" | `#64748B` | Same slate as the locked 150km data-horizon ring — muddles SLATE=limit semantic in panel | Medium |
| C5 | Warehouse dot | `#0891B2` | Hue-safe, but fails WCAG 3:1 against `#0b1220` (too dark for small dot) | Low |

---

## 1. Member Store Dot Colors

The dot layer is **qualitative** (four nominal roles, no order). It must occupy hue space that the locked ring palette leaves free: green/teal/cyan/gold/magenta/violet. Never blue or orange ramps.

**Figure-ground rationale:** navy ring recedes (boundary/context), gold anchor advances (subject). This is the Mapbox Studio dark-showcase pattern: structural geometry in low-chroma cool tones, points-of-interest in high-chroma warm tones.

| Role | Fill hex | Stroke | Radius z10→z14 |
|---|---|---|---|
| **Anchor / hypermarket** | `#FACC15` (amber-gold) | `#0b1220` 1.5px dark halo | 5px → 9px |
| **Hardware** | `#2DD4BF` (teal) | `#0b1220` 1.25px halo | 4px → 7px |
| **Warehouse club** | `#22D3EE` (bright cyan) | `#0b1220` 1.25px halo | 4px → 7px |
| **Lifestyle** | `#E879F9` (magenta-orchid) | `#0b1220` 1.25px halo | 3.5px → 6px |

**Hierarchy mechanics — required beyond color (color-blind safety):**

1. **Radius ramp:** anchor 9px, secondaries 7px, lifestyle 6px at z14. Size is fully color-blind-safe.
2. **Shape for the teal/cyan at-risk pair:** hardware = rounded-square; warehouse = circle. (Teal/cyan collapse under deuteranopia. Esri/CBRE/Placer use shape+color for retail formats precisely for this reason.) Alternative: give warehouse a double-stroke ring instead of shape change.
3. **Dark halo on every dot** (`#0b1220` stroke): prevents dots bleeding into basemap shadow at z10. Without it, gold is the only role that survives small sizes.

**Do NOT encode cluster tier in dot size.** Tier is carried by the ring (1km solid vs. 3km dashed) and the centroid. A Walmart is the same physical store in T1 and T3 — dot encodes only zoom + store role.

**Dot sizing interpolation (all roles, then multiply by role factor):**

```
"circle-radius": ["interpolate", ["linear"], ["zoom"],
   8, 2.5,
  10, 4.0,
  12, 6.0,
  14, 8.0
]
```
Anchor = ×1.0 of above. Secondary (hardware/warehouse) = ×0.72. Lifestyle = ×0.75.

---

## 2. Hull Fill Colors

The hull (convex hull of member store positions) sits inside the co-location ring, under member dots.

**One hue for all tiers — opacity carries the tier distinction (sequential cartographic principle):**

| Property | Value |
|---|---|
| `fill-color` | `#1E40AF` (the T2 indigo) for **all** tiers |
| `fill-opacity` | T1: 0.14 / T2: 0.10 / T3: 0.07 |
| `fill-outline-color` | none (or `#1E40AF` at 0.0) — "one strong stroke + one soft fill" rule |
| blend | normal (not multiply — multiply kills readability on dark basemaps) |

**Why not match the per-tier ring hue?** `#0A3070` hull inside `#0A3070` ring = near-zero separation. Using `#1E40AF` for all creates a readable tint that stays "blue = containment" while separating from the ring stroke.

**Hull geometry rules:**
- N = 1 (singleton): no hull
- N = 2 or collinear: 2px-buffered capsule around segment (not convex hull, which degenerates to a line)
- N ≥ 3 non-collinear: true convex hull polygon, fill-only

**Hull `minzoom`:** match the co-location ring (z9 for T1, z7 for T2/T3). Below minzoom collapse to centroid-only.

**At z10–11:** hull adds value even at small size — the outline of the cluster footprint (tight patch vs. elongated sliver) is pre-attentively readable and the ring+dot alone cannot show it.

---

## 3. Centroid Marker

The centroid is understated — it is a click target, not the visual hero.

**Recommendation: thin `+` cross glyph (not hollow ring, not filled circle, not suppressed).**

- Cartographically unambiguous: `+` is the universal center-of-mass symbol (used in DBSCAN/k-means plots, survey centroids, Esri centroid layers)
- Cannot be mistaken for a store dot
- Mostly negative space = inherently understated

| Property | Value |
|---|---|
| Glyph | `+` symbol, ~1.25px stroke weight |
| Size | z10: 9px → z14: 14px bounding box |
| Color | Tier blue: `#0A3070` (T1) / `#1E40AF` (T2) / `#3B6FB5` (T3) |
| Opacity | 0.85 unselected → 1.0 selected |
| Hit area | 24px invisible padding (large hit target, small visible glyph) |

**Rejected alternatives:**
- Hollow ring: reads as a store dot to many users; competes with member dots
- Suppress / hull-as-click-target: contradicts requirement for a definite click affordance
- Low-opacity filled circle: reads as "a store we're not sure about"

---

## 4. Tier Badge / Tier Indicator Colors

Context: BentoBox panel (`#0f172a`). Tier is ordinal → single-hue sequential ramp (correct for ordinal data). Keep blue family (tier = co-location strength = containment story).

**Kill the 42px tier badge billboard.** Promote the mobile chip spec to default.

| Tier | Badge background | Text | Notes |
|---|---|---|---|
| **Regional (T1)** | `#1D4ED8` | `#FFFFFF` | Brightened from `#0A3070` (too dark against panel — ~1.4:1). `#1D4ED8` → white text ≈ 5.9:1. WCAG AA pass. |
| **District (T2)** | `#3B82F6` | `#0b1220` | Dark text on mid-blue ≈ 6.3:1. Clear step down. |
| **Local (T3)** | `#7DD3FC` (light sky) | `#0b1220` | **Moved off `#64748B` slate** (conflict C4). Still blue family. Dark text ≈ 11:1. |
| **Fringe (T4)** | transparent + `#334155` 1px border | `#94A3B8` | Ghost chip — no fill. Fringe is not a real tier; it should attract less attention than Regional, not more. **Current `#e2e8f0` light-gray is a bright white block — backwards priority.** |

**Badge spec (chip/pill):**
- `border-radius: 6px` (not 12px — 12px reads as consumer app; CBRE/Bloomberg chips are 4–6px)
- `font-size: 11px`, `font-weight: 700`, `letter-spacing: 0.04em`, `text-transform: uppercase`
- `padding: 4px 10px`
- Preferred style: tinted background at low opacity + 1px border at tier hue (e.g., Regional: bg `rgba(10,48,112,0.25)`, border `#1E40AF`, text `#93c5fd`) — more institutional than a solid fill block

---

## 5. Delta Visualization Colors (Alberta Simulation Overlay)

| Status | Recommended | vs. Proposed | Change reason |
|---|---|---|---|
| **Upgraded** | `#22C55E` (brighter green) | `#16a34a` → brighter | Higher luminance for dark-basemap pop |
| **Unchanged** | `#94A3B8` (neutral slate-gray) | **replace `#2563eb`** | "Unchanged" = zero delta = neutral. Gray is the honest encoding; vacates blue channel so it cannot be confused with co-location rings (conflict C3) |
| **Downgraded** | `#F43F5E` (rose-red) | `#dc2626` → shift | Brighter on dark; shifted toward magenta so green/red pair separates better under deuteranopia |
| **New** | `#A855F7` (violet) | `#9333ea` → brighter | Keep violet (correct: not on the up/down scale); brighten for dark basemap |

**Color-blind reinforcement (required — green/red is the worst pair for deuteranopia):**
- Upgraded: green + `▲` glyph at north arc point
- Downgraded: rose + `▼` glyph
- New: violet + dashed stroke (no old geometry = provisional/new)
- Unchanged: gray + solid, no glyph (quiet default)

---

## 6. Selection and Hover State System

**Dim-the-field model** (professional standard — Placer.ai, CBRE): contrast comes from *both* directions (selected = lifted; non-selected = dimmed).

| Element | Unselected (base) | Selected |
|---|---|---|
| **Co-location ring `line-opacity`** | 0.35 (dimmed, not hidden) | 0.90 (T1) / 0.70 (T2/T3) per ring plan §2 |
| **Co-location ring `line-width`** | base width | base + 0.5px |
| **Hull `fill-opacity`** | 0.07–0.14 per tier | 0.14–0.22 (+50%) |
| **Member dots `circle-opacity`** | 0.55 | 1.0 |
| **Member dots `circle-radius`** | base | base × 1.4 |
| **Centroid marker** | 0.85 | 1.0 |
| **Other clusters' dots/hull** | — (when one cluster selected) | dim to 0.20 |
| **Other clusters' rings** | — (when one cluster selected) | dim to 0.35 |

**Key rules:**
- **Dim unselected rings, do not hide them.** 0.35 keeps spatial-density context ("there are neighbors") without distraction.
- **No color change on selection** — only opacity, width, radius. Hue stays reserved for meaning.
- **No dot size change on hover** — reserve size change for selection only (hover ≠ selection is a professional-standard distinction).

**Hover state:**
- Pointer cursor (mandatory)
- Whole-cluster hull/ring brighten slightly (hull 0.10 → 0.14, ring +0.10 opacity)
- 2-line tooltip: cluster name (line 1), `"[Tier] · N stores"` (line 2)
- Tooltip: `#0b1220` bg, white text, ~11px, ~150ms open delay, no rich data (BentoBox only on click)

**BentoBox cross-highlight (store row ↔ map dot):**
- Mechanism: `setFeatureState({source:'member-stores', id:storeFeatureId}, {bentoHover:true})`
- Visual: white 1.5px stroke on dot + ×1.35 radius
- **No pulse animation** — pulsing reads as alert/error/GPS and strobe-flickers during fast list scanning
- Bidirectional optional (hover dot → highlight BentoBox row): low cost, high polish

---

## 7. Transition Model — Two-Speed

**Reject uniform "no animations."** The professional standard (CoStar, JLL, Placer) is:

- **Camera/zoom-driven → instant hard cut.** Crossing a zoom threshold (S0→S1, S6a) must not fade — fading reads as loading, not state change. ✓ Correct as in ring plan.
- **Selection/user-intent → ~150ms ease.** A click is a causal act; a brief transition communicates "your click did this." Hard cut on selection feels like a bug.

**S3 data-layer reveal — stagger, do not slam.** All four layers (catchment fill+ring, horizon ring, vignette, heatmap) appearing simultaneously in one frame is visually violent. Sequence:

| Delay | Layer |
|---|---|
| 0ms | `catchment-inner-fill`, `catchment-inner-line` (35km) |
| 80ms | `catchment-outer-line` (150km ring) |
| 120ms | `data-horizon-mask` (vignette) |
| 150ms | active heatmap sublayer |

Total: under 250ms. Reads as "reach → then limit → then data." Matches color semantics.

**State transition summary:**

| Transition | Type | Duration |
|---|---|---|
| S0→S1 (zoom in, rings appear) | Camera threshold | Instant |
| S1→S2 (select cluster, others dim) | User intent | 150ms ease |
| S2→S3 (data layer ON) | User intent | Staggered, ~250ms total |
| S4/S5 (switch cluster) | User intent | 150–200ms re-center ease |
| S6a (zoom out, data OFF) | Camera threshold | Instant |
| S6b (zoom out, data ON) | Nothing appears/disappears | No transition |

---

## 8. Ring Weight Hierarchy (fourth differentiating axis)

Three rings in three colors *can* work but require a weight hierarchy or they compete visually. Add `line-width` and `line-opacity` as a fourth axis:

| Ring | Size | Color | Dash | **Line width** | **Line opacity** |
|---|---|---|---|---|---|
| Co-location 1/3km | Innermost | Deep blue | solid/short-dash | **2px** | **1.0** |
| 35km catchment | Middle | Orange | long-dash | **1.5px** | **0.85** |
| 150km horizon | Outermost | Slate | long-dash | **1px** | **0.5** |

Rationale: the eye is drawn inward (heaviest = innermost = subject), matching analytical priority. The catchment ring's orange *fill* carries its weight so the line can be lighter. The horizon ring is a caveat/disclaimer — it should whisper.

---

## 9. Data Horizon Vignette — Revised Spec

**Risk:** 0.45 opacity near-black over-reads as consumer/theatrical (Apple Maps "look around," Spotlight search). Real-estate and site-intelligence audiences read heavy vignetting as marketing demo, not analysis tool.

**Professional standard:** a subtle out-of-coverage mask — geography outside the horizon remains legible (city names, coastlines); it is simply visibly "not in scope."

| Property | Original plan | Revised recommendation |
|---|---|---|
| `fill-color` | `#0b1220` | `#0b1220` (unchanged) |
| `fill-opacity` | 0.45 | **0.20–0.22** — halved |
| Naming | "vignette mask" / "lit stage" | **"out-of-coverage mask"** — factual, BCSC-defensible |
| Token name | — | `mask-data-extent` |

**Rationale:** the 150km **ring** (crisp slate dashed stroke) does the "here is the limit" communication. The mask only needs to gently *support* the ring, not dominate it. The ring is the primary limit signal; the mask is the secondary confirmation.

Validate at 0.22 on Alberta sim. Operator may prefer 0.28; 0.35+ risks the consumer read.

---

## 10. Map Typography & Labels

**Critical migration note:** current `index.html` runs a **light theme** with `text-halo-color: #ffffff`. On dark tiles, white halos create the "glowing sticker" effect — the single most common amateur-dark-map tell. **All halos must invert to `#0b1220` for the dark migration.**

### Map label specifications (dark basemap)

| Label type | First zoom | text-size | text-color | Halo |
|---|---|---|---|---|
| Regional market name | z6 | `z6:11 → z10:15` | `#94a3b8` (slate-400) | `#0b1220` w1.5 blur0.5 |
| Cluster tier label | z9 | `z9:11 → z13:14` | `#e2e8f0` (slate-200) | `#0b1220` w1.75 |
| Store chain label | z13 | `z13:10 → z16:12` | `#cbd5e1` (slate-300) | `#0b1220` w1.25 |
| Arc label "Data horizon — 150 km" | Active only | fixed 11 | `#64748b` (slate-500) | `#0b1220` w1 blur1 |

### Font recommendation

- **Safe default (no custom pipeline):** `["Noto Sans Bold"]` / `["Noto Sans Medium"]` / `["Noto Sans Italic"]` — ships in every Maptiler font stack, clean, professional.
- **Avoid:** "DIN Offc Pro" (Monotype licensed, legal exposure); "Open Sans" (generic/dated).
- **Best case:** if generating custom SDF glyphs, use Inter throughout — unifies map + BentoBox panel, excellent tabular figures.

### Arc label design ("Data horizon — 150 km")

- `symbol-placement: "point"`, north point of ring, `text-offset: [0, -1.2]` — do NOT rotate along arc
- `font-style: italic` — cartographic convention for non-physical/analytical boundaries
- `letter-spacing: 0.04em` — annotation register
- `text-opacity: 0.85` — quiet; never competes with cluster labels
- Em-dash format retained (`—`): on-brand with Bloomberg editorial standard

### Cluster naming

- **Do not put tier code inside the display name.** "Edmonton — T1 Regional (South Common)" conflates three orthogonal facts.
- Map label: two-line — submarket (`South Common`), metro (`Edmonton`) below at lower weight/opacity.
- Tier belongs in the BentoBox badge only.
- Use `ANCHOR_DISPLAY_NAMES` from `config.py` as curated override; nearest-place derivation as fallback only.

---

## 11. BentoBox Typography System

Panel: `#0f172a` (slate-900). Type scale (1.20 minor-third, anchored at 13px):

| Role | Size / line-height | Weight | Color |
|---|---|---|---|
| Primary heading (cluster name) | 20px / 26px | 700 | `#f8fafc` (slate-50), `letter-spacing: -0.01em` |
| Section eyebrow ("Trade Area", "Member Stores") | 10px / 14px | 700 | `#64748b` (slate-500), `text-transform: uppercase`, `letter-spacing: 0.08em` |
| Data values (population, spend) | 18px / 22px | 700 | `#f1f5f9` (slate-100), **`font-variant-numeric: tabular-nums`** |
| Secondary text (addresses) | 11px / 16px | 400 | `#94a3b8` (slate-400) |
| Disclaimer / data-horizon copy | 10px / 15px | 400 | `#64748b` (slate-500), italic |

**Numeric formatting:**
- Headline stats → abbreviated: `2.8M`, `847K` (2 significant figures)
- Tables / comparison values → full separators: `2,847,000` (locale-aware via `Intl.NumberFormat`)
- **No green/red for raw magnitudes** — a large population is not "good." Reserve color for genuine delta/comparison values, always paired with `▲`/`▼` and `+`/`−` sign (never color alone).
- Right-align numeric columns; left-align labels.

**Layout discipline:**
- 16px panel padding, 12–16px between sections
- 1px `#1e293b` hairline dividers (not nested boxes/cards)
- Flatten the `.index-box` / `.civic-box` nesting pattern — a hairline + eyebrow label does the same job with less ink

---

## 12. Zoom-Graduated Visibility (Retailer-First)

The three cluster-geometry elements carry different information at different zooms; they should appear at different levels:

| Zoom | Co-location ring | Hull | Member dots | Centroid |
|---|---|---|---|---|
| z8–z10 | ✓ (ring only) | hidden | hidden (sub-px noise) | hidden |
| z11–z12 | ✓ | fades in | fades in | fades in |
| z13–z14 | ✓ | fades back (hairline only) | dominant | dominant |

At z14 (street level): stores are the subject; ring is context only. Reduce hull fill-opacity at z14 (not disappear, just recede).

---

## 13. State-Bound Legend (missing feature)

Professional tools always show a legend that is *bound to the active overlay state*. The current plan has no legend.

**When data layer is OFF:** no legend (nothing to explain).
**When data layer is ON:** a compact fixed legend (bottom-left, 8px grid):
- Orange: "35 km primary catchment"
- Slate: "150 km data horizon"
- Heatmap bins: labeled discrete steps (not a continuous ramp bar), 4–5 bins labeled with value ranges

The legend disappears when the data layer toggles OFF. Never a floating decoration.

---

## 14. Professional Polish — Minimum Viable (Priority Order)

Achievable in MapLibre GL JS + CSS without a design-system rebuild:

1. **Tabular numerics.** `font-variant-numeric: tabular-nums` on all numeric values in the panel. Single biggest amateur→professional jump, near-zero cost.
2. **State-bound legend.** Compact fixed position, visible only when data layer is ON, labeled discrete bins.
3. **Stagger the S3 data-layer reveal** (§7). Eliminates the most jarring moment in the design. Pure `transition` paint property.
4. **Ring weight stratification** (§8): 2px/1.0, 1.5px/0.85, 1px/0.5. Turns three competing rings into an inward hierarchy.
5. **Halve the vignette opacity** to ~0.22 (§9). Removes consumer-product risk.
6. **Synchronized hover.** On dot or cluster hover, brighten map feature AND highlight BentoBox row simultaneously.
7. **8px grid audit.** BentoBox width 360, padding 16/24, rows 32/40, badges/swatches multiples of 4. Invisible individually, decisive in aggregate.

---

## 15. Design Token Format

**Recommended:** Style Dictionary-compatible DTCG JSON, three-tier:

1. **Primitive tokens** — raw values, no semantics. `color-blue-900: #0A3070`. Routes to `pointsav-design-system`.
2. **Semantic tokens** — meaning, no raw values. `ring-containment-t1: {color-blue-900}`. Encodes BLUE/ORANGE/SLATE rule as enforceable token names.
3. **Component tokens** — `bentobox-bg`, `tier-badge-t1-bg`. Branded ones route to `woodfine-media-assets`.

Per `.agent/rules/design-tokens.md` routing protocol:
- Generic tokens → `pointsav-design-system`
- PointSav branded → `pointsav-media-assets`
- Woodfine branded → `woodfine-media-assets`

**Required fields per token:**

| Field | Description |
|---|---|
| `name` | Canonical token name, lowercase-hyphen, tier-prefixed |
| `value` | Raw value (primitive) or `{reference}` (semantic/component) |
| `type` | DTCG type: `color` / `dimension` / `opacity` / `duration` |
| `description` | One-line semantic intent — Bloomberg-precise |
| `category` | primitive / semantic / component |
| `route` | `pointsav-design-system` / `woodfine-media-assets` etc. |
| `state-binding` | Which S0–S6 states the token is active in |
| `semantic-class` | `containment` / `reach` / `limit` / `neutral` — makes no-crossover rule machine-checkable |
| `deprecated` / `replaces` | Migration trail |
| `cites` | Citation IDs per workspace §16 |

---

## 16. Open Decisions (not yet locked)

| # | Decision | Options | Note |
|---|---|---|---|
| OD-V1 | Vignette opacity final value | 0.20 / 0.22 / 0.25 | Validate on Alberta sim first |
| OD-V2 | Anchor dot color | `#FACC15` (amber-gold) vs. `#FCD34D` (lighter gold) | Both correct in principle; test against dark basemap |
| OD-V3 | Centroid glyph implementation | MapLibre `+` sprite vs. SVG icon vs. circle-layer workaround | CSS circle-layer can fake a `+` with `::before`/`::after` but MapLibre layers need a sprite for symbol-layer `+` |
| OD-V4 | Hull type | Convex hull vs. alpha shape (concave) | Convex recommended for 3–8 points; alpha only needed if cluster has distant outlier member |
| OD-V5 | Hover transition duration | Instant vs. 80ms ease vs. 150ms ease | Dots on hover: instant (color-state flicker is expected behavior); BentoBox cross-highlight: 80ms |
| OD-V6 | Font stack | Noto Sans (safe/free) vs. Inter SDF (unified panel+map) | Committing to Inter requires a glyph-generation pipeline step |
| OD-V7 | S3 stagger durations | Proposed: 0/80/120/150ms | Validate against actual FPS; slower if heatmap tile-load is slow |

---

## 17. Phased Implementation Order (after Alberta sim sign-off)

This design plan does not alter the implementation order from `RING-HIERARCHY-DESIGN-2026-05-20.md` §12. It adds visual-spec refinements to each phase:

1. **Alberta sim validation** — fix `members_detail` bug in `simulate-dbscan-ab.py`; add `sim-ring-src` + 3 ring layers; validate ring weight hierarchy and vignette opacity at 0.22.
2. **Palette conflict fixes** — update live `index.html` dot colors (C1/C2/C3/C4 above); update delta colors.
3. **Ring semantics + weight** — `updateProximityRings()` tier-derived radius; ring weight stratification table above.
4. **150km ring repaint** — remove `line-blur`; add dash; `#64748B`; weight 1px/0.5 opacity.
5. **Vignette mask** — `data-horizon-mask` at 0.22 opacity; token `mask-data-extent`.
6. **Member dot layer** — convex hull + centroid glyph + member dots; `showClusterDetail()`.
7. **BentoBox upgrade** — tabular numerics; state-bound legend; type system per §11.
8. **Transition model** — stagger S3; 150ms selection transitions.
9. **Data horizon label** — italic annotation at north point.
10. **Design tokens** — extract to DTCG JSON; route per §15.

---

## 18. References

**Benchmarks surveyed:**
- CBRE Market Analytics / Dimension
- JLL Tetris / JLL Research
- CoStar Pro / Suite
- Placer.ai Analytics
- Esri ArcGIS Insights / Urban Observatory / Business Analyst
- SafeGraph Patterns / Dewey

**Standards referenced:**
- ColorBrewer2 — qualitative/sequential/diverging scheme discipline
- IBM Carbon Design — dark theme state-via-opacity, not hue; categorical palette guidance
- DTCG (Design Tokens Community Group) — token format specification
- Polychrome-36 — extended qualitative palette
- WCAG 2.1 AA — contrast requirements

**Prior plans:**
- `.agent/plans/DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md`
- `.agent/plans/RING-HIERARCHY-DESIGN-2026-05-20.md`
- `.agent/plans/tier-scoring-overhaul-2026-05-16.md`

— drafted 2026-05-20 from four Opus research agents (color palette, symbol design, typography, integrated design system)
