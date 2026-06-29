---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-gis-whitespace-cannibalization-model
owner: project-gis
parent: project-gis-map-ux-audit
created: 2026-06-20
updated: 2026-06-20
---

# White-Space & Cannibalization Model

Sub-BRIEF of the 2026-06-20 map UX/tech audit
(`BRIEF-gis-map-ux-audit-2026-06-20.md`), promoting backlog item **F16** —
"No white-space / cannibalization analysis" — into a designed analytical
capability owned by `project-gis`. F16 was raised independently by the Costco
site-selection persona and the spatial-economics professor; the competitor
teardown (`R2-competitor-teardown.md`) ranks the same two gaps — **void /
white-space analysis** and **cannibalization / overlap quantification** — as
gaps 5 and 6 of 15, both explicitly tied to retail site-selection and investor
credibility. CARTO ships a store-cannibalization module; Placer.ai ships Void
Analysis; SiteZeus ships white-space revenue projection. These are the analytics
retail location-intelligence buyers pay $5k–$30k/yr for. Our map today answers
"where do the right retailers cluster" — it does not answer "where is a
co-location *missing but demanded*" or "how much does a new node *overlap* an
existing one." F16 closes both.

The leverage here is unusual: **the hard geometry already runs in the browser.**
The UI's `groupOverlappingClusters` Union-Find (shipped under B6,
DESIGN-RESEARCH "Bento Merged Zones," commit `21cf18df`) already detects when
proximity rings overlap and renders a merged-ring panel via
`showMergedGroupPanel`. That detection is computed and then discarded as a UX
convenience. This BRIEF turns the discarded overlap signal into a **quantified
competition metric**, and adds its mirror image — **white space** — as a
first-class surface on chain select.

---

## 1. Problem statement

The map is a supply map. Every dot — T1/T2/T3 cluster, raw retail POI — marks
where retail *is*. A site-selection or investment audience needs the two
demand-relative reads that supply maps cannot give:

- **Cannibalization / saturation.** When a chain (or a co-location node) sits
  close enough to another that their trade areas overlap, the overlap is shared,
  not additive demand. Opening or acquiring inside an existing footprint
  transfers sales rather than capturing new ones. A buyer evaluating a candidate
  needs to know *how saturated* a market already is before treating its tier as
  upside.
- **White space.** The inverse and the more valuable question: a market with
  strong co-location demand (high tier, large catchment population and spend,
  the right anchor mix) where a *given* chain is **absent**. For a Costco
  expansion director, "show me every T1/T2 grocery-plus-hardware co-location in
  the US that does not contain a Costco" is the literal job. The map cannot
  answer it today; the user rebuilds it in Excel and defaults to Esri/Placer.

Both questions are demand-vs-supply gaps. We already hold every input: the
demand side (catchment population from WorldPop, modelled spend, co-location
tier/composition in `clusters-meta.json`), and the supply side (per-chain
presence in the `service-business/*.jsonl` ingests, ~120 chains across 13+
countries NA/EU). We have never joined them into a gap metric.

This is the audit's "the moat is invisible" theme (F5) applied to the analytic
layer: the data asset is genuinely differentiated, but it is presented as a map
of dots rather than as a decision.

---

## 2. Current state — what already exists

| Asset | Where | What it gives the model |
|---|---|---|
| `groupOverlappingClusters` (Union-Find) | inline `index.html` (gateway `www/`) | Connected-components grouping of clusters whose proximity rings overlap. Already computes the *adjacency* the cannibalization metric needs. |
| `showMergedGroupPanel` | inline `index.html` | Renders the merged group as one panel — proof the overlap relation is already surfaced to the user, just not quantified. |
| Proximity rings | `layer3-catchment.pmtiles` + per-cluster ring radius `max(1.0, span_km/2 × 1.15)` | The geometry whose pairwise overlap is the raw cannibalization signal. (Radius derivation is itself a known defect — F11; see §7.) |
| `clusters-meta.json` | gateway `www/data/` (5,702 clusters; T1=1,157 / T2=4,283 / T3=262, Phase 18) | Per-cluster tier, member chains / `brand_family`, `span_km`, centroid, confidence flag. The demand + composition record. |
| Catchment demographics | `layer4-census.pmtiles`, `layer5-spend.pmtiles`, `work/catchment-data.json` | Per-cluster population (WorldPop 100m → H3 res-7) and modelled spend — the demand magnitude. |
| Per-chain presence | `service-business/<chain>.jsonl` (~120 chains) | The supply side: which chains are in which cluster. The "is chain X present" predicate for white-space. |
| MapLibre + PMTiles + positron | gateway | Render substrate; overlap shading and white-space markers slot into existing layer/sort-key conventions. |

The Union-Find already runs client-side at interaction time. The model below is
mostly a **precompute + join + surface** exercise, not new geometry.

---

## 3. Part A — Cannibalization / overlap metric (per cluster)

### 3.1 Definition

For each cluster *i* with ring area *A_i*, and each other cluster *j* whose ring
overlaps *i* (the relation the Union-Find already finds), compute the geodesic
intersection area *A_ij* (turf metric buffers — never screen-circle area; see
F29). Define a per-cluster **overlap fraction**:

```
overlap_i = clamp( Σ_j area(ring_i ∩ ring_j) / area(ring_i) , 0, 1 )
```

This is the share of cluster *i*'s modelled trade area that is contested by at
least one neighbour. `overlap_i = 0` → standalone catchment (clean upside);
`overlap_i → 1` → almost entirely inside other footprints (saturated).

Two refinements, both cheap given existing data:

- **Demand-weighted overlap.** Weight the intersection by catchment population
  (or spend) rather than bare area, so overlap over empty land scores lower than
  overlap over dense population. Uses `layer4`/`layer5` already on disk.
- **Same-chain overlap (true cannibalization).** Restrict *j* to clusters
  containing the *same chain* as a selected chain to answer "would a new Costco
  here eat an existing Costco's trade area." This is the chain-specific
  cannibalization read versus the generic market-saturation read.

### 3.2 Output

- A precomputed `overlap_i`, `overlap_pop_weighted_i`, and the contributing
  neighbour cluster IDs, written into `clusters-meta.json` (or a sibling
  `clusters-overlap.json` to keep the eager payload from growing — see F3).
- On the map: an optional **saturation shade** on the ring fill (low chroma →
  high chroma with overlap), distinct from the tier hue so the two reads do not
  collide (F18). Off by default; a layer toggle, legend entry mandatory (F1).
- In the scorecard (F5): a "Market saturation" line — e.g. *"38% of this
  catchment overlaps 2 neighbouring co-locations"* — with the neighbour list.

### 3.3 Honesty constraints (BCSC + peer-review posture)

Overlap is a **modelled** quantity derived from modelled rings. Until the ring
radius is given demand meaning (F11) and catchments move toward observed /
drive-time trade areas (F6), the metric must be labelled as **planned/indicative
overlap of straight-line distance bands**, not measured cannibalization of
observed trade areas. The Method modal must state this and carry the same
DBSCAN-sensitivity caveat the synthesis flags (F28). We do not present a modelled
ratio as a measured sales-transfer figure.

---

## 4. Part B — White space (per chain, on select)

### 4.1 Definition

White space is a **chain-conditioned** read. On selecting a chain (the existing
chain search, F17), a market qualifies as white space for that chain when:

```
qualifies(cluster c, chain X) :=
      tier(c) ∈ {T1, T2}                       # strong co-location demand
  AND catchment_population(c) ≥ P_min          # demand floor (WorldPop)
  AND catchment_spend(c) ≥ S_min               # spend floor (modelled)
  AND chain X NOT present in c                  # the absence
  AND anchor_mix(c) ⊇ chain X's category peers # the right neighbours
  AND confidence(c) = high                      # F13 — gate on the confidence flag we already compute
```

`anchor_mix ⊇ category peers` is the co-location differentiator: a grocery
white-space candidate for Costco is a cluster already containing the warehouse /
big-box / hardware mix Costco co-locates with, not merely any populated place.
This is the read no single-chain foot-traffic product produces, because it
requires the co-location composition we already model.

Thresholds `P_min`, `S_min` start as documented constants in the Method modal
(F30 honesty: explicit, not hidden) and become user sliders in a later
iteration.

### 4.2 The Costco persona (worked example)

The Costco site-selection director (audit persona COSTCO) selects "Costco" in
chain search. The map intends to:

1. Filter to Costco's ~present clusters (existing F17 fly-to/filter behaviour).
2. Overlay **white-space markers** — clusters that `qualifies(c, "Costco")` —
   in a distinct, legended symbol (hollow/halo'd; F8) so absence reads as a
   positive opportunity signal, not missing data.
3. Rank white-space candidates by catchment population × spend (the F5 score),
   surfaced as a list (which doubles as the crawlable/a11y surface, F21/F26).
4. Each candidate's scorecard shows: catchment population & households, modelled
   spend, the co-located anchors present, the saturation/overlap figure from
   Part A (is this white space *also* uncontested?), and "why this qualifies."

The same flow serves any chain in the ingest set; Costco is the demonstration
because it is the audit's reference persona and a clean big-box co-location case.

### 4.3 Output surface

- **Map layer:** white-space markers (chain-conditioned, recomputed on select),
  distinct encoding from tier dots and from saturation shading; legend entry;
  toggle.
- **Ranked list / compare tray:** ties directly into F15 (candidate comparison)
  — the white-space list *is* a shortlist; selecting candidates into the compare
  tray gives the side-by-side table (pop, spend, competitors, tier, saturation,
  score). This is the artifact that travels into an investment committee (F24).
- **Deep-link (F27):** the selected chain + white-space toggle serialize into the
  URL hash so a "here is Costco's white space in the US South" view is shareable.

---

## 5. Data inputs (consolidated)

| Input | Source on disk | Role |
|---|---|---|
| Cluster tier, composition, `span_km`, member chains | `clusters-meta.json` (gateway `www/data/`) | Demand strength + anchor mix + ring geometry seed |
| Ring geometry | `layer3-catchment.pmtiles` | Overlap intersection (Part A) |
| Catchment population | `layer4-census.pmtiles` / `work/catchment-data.json` (WorldPop 2026 100m → H3 res-7) | Demand floor + ranking + demand-weighted overlap |
| Catchment spend | `layer5-spend.pmtiles` (per-capita multipliers; 3-stage estimation — flag MAUP/error per F25) | Spend floor + ranking |
| Per-chain presence | `service-business/<chain>.jsonl` (~120 chains, NA/EU) | Absence predicate (Part B); same-chain cannibalization (Part A refinement) |
| Confidence flag | `clusters-meta.json` / `regional-markets.json` (high-conf marker) | Gate (F13) — no white-space claim on low-confidence clusters |
| Overlap adjacency | `groupOverlappingClusters` Union-Find (existing) | Free adjacency for Part A — already computed |

No new ingest is required for v1. The model is a join over assets the Phase
11–18 builds already produced.

---

## 6. Output surface summary

1. **Precompute** (build-time, `app-orchestration-gis/`): a
   `build-overlap-whitespace.py` step that writes per-cluster `overlap_*`
   fields and, optionally, a static per-chain white-space candidate index. This
   keeps interaction-time work to a filter, not a recompute, and keeps the eager
   payload disciplined (F3 — prefer a sibling JSON / PMTiles attribute to
   inflating `clusters-meta.json`).
2. **Map layers:** saturation shade (Part A) + chain-conditioned white-space
   markers (Part B), each with a mandatory legend entry (F1), distinct from tier
   encoding (F8/F18), toggleable, off by default.
3. **Scorecard integration (F5):** saturation line + white-space "why this
   qualifies" block inside the per-cluster inspector.
4. **List / compare / export (F15/F24):** the white-space ranking is a
   shortlist; compare tray and one-pager export reuse it.
5. **Method modal (F14/F28/F30):** definitions, thresholds, the
   modelled-not-measured caveat, DBSCAN + ring-radius sensitivity.

---

## 7. Dependencies, risks, sequencing

**Hard dependency on radius semantics (F11) and trade-area framing (F6).**
Overlap of `span_km/2 × 1.15` rings inherits that formula's lack of demand
meaning. The metric is *directionally* useful immediately but is not
defensible as cannibalization until rings are derived from a demand quantity
(distance-decay, drive-time isochrone, or population/spend threshold). Plan:
ship the model against current rings with explicit "indicative" labelling;
re-derive once F6/F11 land. The two BRIEFs (this one and the Delivery
Re-Architecture BRIEF, which owns the PMTiles/observed-trade-area path) should
sequence F6/F11 ahead of the v2 of this metric.

**Geodesic computation (F29).** All intersection areas computed with turf metric
buffers in equal-area space, not screen circles — a 35 km ring at 60°N is not a
35 km ring at 25°N.

**Threshold defensibility (F30).** `P_min`/`S_min` and the T1/T2 gate must be
stated, not hidden; a too-permissive floor reproduces the "Regional Market = ≥1
co-location" over-claim the professor persona rejected.

**Payload discipline (F3).** Adding overlap fields and a white-space index must
not re-inflate the eager load the delivery BRIEF is trying to shrink. Prefer
PMTiles attributes or lazily-fetched per-chain indices.

**ADR-07.** White-space *ranking* and overlap are deterministic spatial joins —
keep them out of any AI layer. A natural-language front door ("T1 grocery
co-locations without a Costco in Texas") is acceptable only as query translation
into these deterministic filters, never as data synthesis (R2 gap 10 caveat).

---

## 8. Planned phasing (forward-looking; subject to operator go/no-go)

The following are intended phases, not commitments:

- **v0 (read-only, indicative):** precompute `overlap_i` from existing rings;
  add the saturation line to the scorecard and a basic Method-modal definition.
  No new map layer yet. Lowest risk; proves the join.
- **v1 (white-space on select):** chain-conditioned white-space markers + ranked
  list on chain select, gated on confidence, wired to the compare tray (F15).
  This is the demo-able Costco-persona capability.
- **v2 (demand-true):** re-derive overlap and white-space against observed /
  drive-time trade areas once F6/F11 land; promote labelling from "indicative"
  to "modelled trade-area" where the underlying mobility data (LODES, MITMA,
  layer6) supports it.

## 9. Routing

This BRIEF is engineering work owned in `project-gis` (per the synthesis §4
routing: a new analytical capability, not a doc update). Companion editorial
follow-through:

- **TOPIC — Co-location Tiering & Scoring** (revise B1 + `gis-variable-distance-model`)
  should document the overlap/white-space definitions once stabilised. →
  project-editorial.
- **DESIGN-RESEARCH — Site Scorecard / Compare / Export Inspector** (F5/F15/F24)
  must include the saturation line and white-space block in the inspector
  spec. → project-design.
- The merged-ring UX precedent (B6, `21cf18df`) is the visual starting point for
  the saturation shading; coordinate so the two do not double-encode overlap.
