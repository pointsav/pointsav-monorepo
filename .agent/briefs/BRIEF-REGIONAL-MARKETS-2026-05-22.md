# BRIEF — Regional Markets: structure, wiki, Top 400, BentoBox

> Research brief · 2026-05-22 · feeds a 4-agent study (GIS · coding · UX · UI).
> Companions: `BRIEF-VARIABLE-DISTANCE-2026-05-21.md`, `BRIEF-CATEGORY-TAXONOMY-2026-05-22.md`.

---

## 1. Purpose

Define the **Regional Market ↔ co-location** structure and how it surfaces in
three products: the **wiki** (projects.woodfinegroup.com), the **Top 400** lists,
and the **BentoBox** on gis.woodfinegroup.com.

## 2. What already exists

`regional-markets.json` (gateway `www/data/`, built 2026-05-15) — **3,011
Regional Markets**, each:
```json
{ "market": "Sherwood Park", "iso": "CA",
  "region": "Strathcona County, Edmonton", "mkt_conf": "high",
  "cluster_count": 4, "cluster_ids": ["c_costco_ca_…", "c_home_depot_ca_…", …] }
```
- A Regional Market is a **named geographic container** that holds **1..N
  co-locations** by `cluster_ids`.
- It is built on the **OLD anchor-centric cluster IDs** — must be rebuilt on the
  new DBSCAN co-locations (Sherwood Park currently lists 4 old clusters; the new
  two-pass DBSCAN finds **3** co-locations there — nodes A/B/C).
- `index.html` does **not** use `regional-markets.json` today — the Regional
  Market concept exists in data but is absent from the UI.

## 3. The model

- **Regional Market** = the named market (e.g. "Sherwood Park", Strathcona
  County). It *contains* the co-locations geometrically inside it.
- **Co-location** = the DBSCAN cluster (the bubble/ring). Sherwood Park RM
  contains **3** co-locations.

### 3a. Wiki — projects.woodfinegroup.com
**One TOPIC per Regional Market.** "Sherwood Park" is one article; the article
shows the **three co-locations within it**. The Regional Market is the editorial
unit, not the co-location.

### 3b. Top 400 — North America & Europe
**TOP 400 North America** and **TOP 400 Europe** (Europe = UK + Nordics +
Continental). Each is a **list of co-locations**. Every row carries a column
showing **which Regional Market the co-location is in**.

### 3c. BentoBox — gis.woodfinegroup.com
The BentoBox must reflect the Regional Market ↔ co-location relationship.

## 4. The ranking question (operator — unresolved)

Unsure whether to show the **Top 400 ranking** on gis.woodfinegroup.com. Candidate
rankings to surface (pick the minimal, cleanest set):
- Top 400 NA / Top 400 Europe (a cutoff list rank)
- geometric distance-rank for NA / Europe (continent percentile)
- tier rank — within the tier
- geometric distance-rank — within the country

**Constraint: clean product/service design — do NOT put much ranking text in
the BentoBox.**

## 5. Research questions

**GIS + coding — the Regions structure:**
1. How is a Regional Market defined geometrically? How are co-locations assigned
   to one (the `region` field looks like admin/place resolution)?
2. Rebuild `regional-markets.json` on the new DBSCAN co-locations — does Sherwood
   Park resolve to one RM with the 3 new co-locations?
3. Schema: how each co-location carries its Regional Market reference; how the
   Top 400 list + the RM column are generated; how one-TOPIC-per-RM is produced.

**UX + UI — the BentoBox:**
4. How should the BentoBox present Regional Market vs co-location — what the user
   sees zoomed out, on a bubble click, and drilled into a ring.
5. Which ranking(s) to surface, and how to show them with minimal text (§4).

---

## 6. Findings — 4-agent study (GIS · coding · UX · UI), 2026-05-22

### Regional Market structure
- An RM is a **settlement-level place** resolved by `RegionEngine.resolve_market()`
  — offline point-in-polygon against boundary files (GADM / TIGER / GISCO LAU).
  `mkt_conf` (high/medium/low) is **geocoding precision, not market quality** —
  it must never be shown as a ranking.
- An RM **contains 1..N co-locations**; a co-location belongs to the RM its
  **centroid** resolves to (non-overlapping settlement polygons → exactly one).
- **`regional-markets.json` has no committed build script** — a lost one-off. A
  committed **`build-regional-markets.py`** stage must be written.
- The RM↔co-location link is a **slug** `regional_market` (e.g.
  `rm_ca_sherwood-park`), derived from `(iso, settlement)` — geometric,
  ID-independent, survives the DBSCAN cluster-ID change.
- **Sherwood Park rebuilds to 1 RM / 3 co-locations** — *conditional on
  re-keying the CA Nominatim override* (currently keyed by old cluster IDs;
  without the fix Sherwood Park will not resolve to "Sherwood Park").

### Top 400
- `generate-top400.py` is a stale parallel pipeline (per-anchor-store, retired
  `linear_score`) — rewrite as a sort + slice + project over the new
  co-locations.
- NA = US/CA/MX; Europe = UK + Nordics + Continental. Sort key
  `(tier, dist_rank_in_tier, demand_rank_in_tier)`, slice `[:400]`. Drop the
  Apex/Hub/Core/Valid star bands (they collide with T1–T3).
- Each row carries a **Regional Market column**.

### Wiki
- New **`generate-rm-topics.py`** — **one TOPIC per Regional Market**; the TOPIC
  = the RM as the article + a section per co-location → drafts-outbound →
  project-editorial.
- Grain issue: dense metros (Chicago / Madrid) collapse to one giant RM — needs
  a sub-division rule before the wiki leg.

### BentoBox (UX + UI converged)
- **Bug:** the panel currently mislabels a single co-location "Regional Market."
- **Two views:** `showMarketDetail()` — RM parent, hero block + co-location list,
  **conditional** (shown only for 2+ co-location markets; singletons skip
  straight to the co-location) — plus the existing co-location view, corrected.
  3-level breadcrumb: Overview → Regional Market → Co-location.
- **One rank, shown minimally:** the **Top 400 continental cutoff position**, as
  a single chip (gold ◆ + number); unranked → show nothing. Delete the current
  4-slot placeholder rank grid; drop tier-rank, country percentile, and
  continent percentile from the headline. Compactness stays as a *property* line.
- No new map layer — the RM is a panel / breadcrumb concept, not 3,011 polygons.

### Open operator decisions
1. **Dense-metro RM granularity** — settlement is too coarse for Chicago/Madrid
   (one giant TOPIC); sub-divide by a finer admin tier, or cap co-locations per TOPIC.
2. **Rural / no-settlement co-locations** (`mkt_conf: low`) — fall back to the
   nearest RM, or exclude from the wiki.
3. **Top 400 sort** — tier-primary (recommended) vs pure `dist_rank`.

### Implementation — S0 / S2
- **S0:** add `regional_market` / `market_name` / `mkt_conf` to
  `simulate-dbscan-ab.py` output (resolve on each co-location centroid);
  prototype `build-regional-markets.py`, the rewritten `generate-top400.py`, and
  `generate-rm-topics.py` for the Sherwood Park RM — on Alberta.
- **S2:** production rebuild — committed `build-regional-markets.py`, full
  Top 400 (NA + Europe), 3,011 TOPIC drafts staged to project-editorial.

---

## 7. Metro Market vs Regional Market (operator, 2026-05-22) — research re-opened

The product unit is the **Regional Market** (settlement-level — Sherwood Park).
A **Metro Market** is a coarser container — a major metropolitan area. The risk:
fringe Regional Markets get wrongly **absorbed into the Metro Market** and lose
their identity (Sherwood Park mislabelled "Edmonton").

**Principles:**
- The platform is **not looking for Metro Markets** — the product is Regional
  Markets. Metro Market is context / labelling only.
- **Keep all co-location rings on the map** — nothing is removed.
- The **Metro Market label must not appear in the co-location / cluster zoom**
  (bubble / ring views) — it is a higher-level label only.
- **Metro Markets are defined by published lists** — CBRE and Oxford Economics
  publish metro-market lists. A city on those lists = a Metro Market.
- **The hard question: where does the Metro Market end and the Regional Market
  begin** at fringe sites? Sherwood Park (Strathcona County — a *separate
  municipality*) is clearly its own Regional Market, not Edmonton. But a
  co-location south of Edmonton — South Edmonton Common (inside the City of
  Edmonton) vs Leduc / Beaumont / Nisku (separate cities)?

**Worked data:** 21 co-locations in the Edmonton CMA — `work/edmonton-area-colocations.md`
— including the IKEA co-location at South Edmonton Common [53.45, −113.49]
(inside Edmonton) and a Leduc co-location [53.27, −113.57] (a separate city).

**Research questions:**
1. **GIS** — how does settlement resolution distinguish a fringe municipality
   (Strathcona County / Sherwood Park, Leduc, Beaumont) from the metro core
   (City of Edmonton)? What admin boundary draws the line? Does `resolve_market`
   get Sherwood Park right — not swallowed into Edmonton — and resolve each
   Edmonton-area co-location to the correct municipality?
2. **Data science** — the co-location distribution: metro-core vs
   fringe-settlement; the Edmonton CMA cases specifically; quantify the
   fringe / ambiguous co-locations platform-wide.
3. **Urban planning / CRE** — what *is* a Metro Market, per CBRE and Oxford
   Economics published lists? Where does a metro end and a fringe regional
   market begin — MSA/CMA boundary vs municipal boundary vs commuter shed? How
   should the platform encode and use Metro Market (a label above the Regional
   Market, never shown in the cluster zoom)?

**Deliverable:** a clear Metro Market vs Regional Market rule, the
metro-vs-fringe boundary definition, and how Metro Market is used.

---

## 8. Operator corrections — 2026-05-22

### 8a. Rural co-locations → resolve to their municipality
A rural co-location is **not** "no market." It resolves to its **containing
municipality** (county / rural municipality / township). Every co-location on
land sits in some admin unit, and the resolver always returns one — the
municipality *is* the Regional Market. This closes the §7 open question: no
"exclude", no "nearest-settlement" fallback — resolve to the admin unit.

### 8b. The Top 400 is NOT in the geometric layer
The **Top 400 sort is a demand-layer product** (Stage 2 — see
`BRIEF-VARIABLE-DISTANCE`), computed later. It is **not part of this geometric
layer**.
- In the geometric / co-location layer, ranking is **all geometric** — a
  co-location's distance-rank vs **all sites in its country**, or **within its
  tier** (`dist_rank_in_tier`, the variable-distance Stage-1 rank).
- **Correction to §6 (BentoBox):** the co-location / cluster zoom shows the
  **geometric distance-rank**, *not* a Top-400 chip. The UX/UI agents' "Top 400
  cutoff chip" recommendation is **superseded** — show the geometric rank.
- `generate-top400.py` still produces the Top 400 list (a list of co-locations
  with the Regional Market column), but as a **demand-layer** artifact run after
  the demand stage — out of scope for the geometric build.

---

## 9. Metro vs Regional — findings, 3-agent study (GIS · data-science · urban-planning), 2026-05-22

### The boundary rule — settled
- **Regional Market = the incorporated municipal polygon** (CA GADM admin-3 CSD ·
  US TIGER place · MX municipio · EU GISCO LAU). The product unit.
- **Metro Market = the MSA/CBSA (US) / CMA (CA) polygon, filtered to a published
  list** — CBRE Econometric Advisors (~61 US metros) + Oxford Economics Global
  City Reports (382 US + 6 CA). Context only · nullable.
- A Regional Market **nests inside at most one Metro Market and is never
  dissolved into it.** Reject the MSA/CMA boundary, the built-up area, and the
  commuter shed as the RM rule — all three swallow Sherwood Park.
- Resolve by **point-in-polygon on the co-location centroid, not lat/lon cutoffs.**

### The resolver is mostly right — one real bug
GIS ran all 21 Edmonton co-locations through `resolve_market()`:
- ✅ IKEA / South Edmonton Common → **Edmonton**; Leduc → **Leduc**; Fort
  Saskatchewan / St. Albert / Spruce Grove → their own municipality. The
  municipal-polygon rule already keeps fringe cities distinct.
- ❌ **Sherwood Park (3 co-locations) mislabels as "Strathcona County"** — it is
  a hamlet *inside* the Strathcona County polygon (no CSD of its own), and its
  correct name comes from a Nominatim override **keyed by dead anchor-centric
  cluster IDs** the new DBSCAN co-locations cannot match.

### Fixes — priority order
1. **Re-key the CA Nominatim override to geometry** (settlement polygons or
   centroid + radius), resolved by point-in-polygon — independent of cluster ID.
   Without this Sherwood Park vanishes into "Strathcona County".
2. `build-regional-markets.py` / the DBSCAN rewrite must call `resolve_market()`
   on each co-location centroid (the sim currently does no region resolution).
3. Add **`resolve_metro()`** — a separate function, gated on the published
   CBRE/Oxford metro list, writing a standalone `metro_market` field.
4. Each co-location carries **four independent fields**: `regional_market` (slug
   — product unit), `market_name` (display), `metro_market` (context, "" if
   none), `mkt_conf` (geocode precision — never a ranking).

### Metro Market usage
- Build a committed **`metro-markets.json`** catalogue (~60–390 metros from the
  published lists).
- Metro Market appears **only**: a wiki Metro index page above the Regional
  Market TOPICs; optionally a Top-400 grouping column; at most one muted context
  line in the Regional Market panel. **Never in the co-location/ring zoom,
  never a breadcrumb level.**

### Distribution & count
- Platform-wide estimate: ~45–50 % metro-core · ~15–20 % metro-fringe · ~30–40 %
  independent settlement → **~50–55 % of co-locations sit outside major-metro
  principal cities.**
- "Fringe" is a real recurring class (the tight Walmart + Canadian Tire
  power-centre pair) but it is a **property, not a tier** — it cuts across T1–T3.
- The ~3,011 Regional Market count **holds — flat-to-slightly-up** (~3,000–3,300).
  Correct municipal resolution *prevents* the only shrink mechanism (fringe
  absorption into the metro).

---

## 10. The override question — a system, not one-off fixes (operator, 2026-05-22)

**Operator principle:** there must be **no one-off Region fixes**. The same
resolution logic must apply **uniformly to all data**. If a consistent,
systematic rule yields "Strathcona County" for Sherwood Park, **that is
acceptable** — a uniform rule beats a hand-curated exception list. Per-place
correctness is *not* worth a maintenance liability.

This puts the §9 fix-#1 ("re-key the Nominatim override") in question. The
override `ca_places_nominatim.json` (~12 hand-curated county→settlement entries)
is exactly the one-off-fix pattern the operator rejects — re-keying it to
geometry makes it survive, but it is still a curated exception list.

**Question for the dispatched GIS agent:** is the override a sound design, or
should sub-municipal settlement resolution be a **proper, complete geographic
layer** that *every* co-location is cross-checked against systematically? Three
candidate outcomes:
1. Replace the override with an authoritative, complete **populated-places
   layer** (e.g. OSM `place=*`, GeoNames, WhosOnFirst, national gazetteers) —
   one dataset, one rule, all 17 countries, no exceptions.
2. **Drop sub-municipal resolution entirely** — accept the municipal/county
   answer everywhere ("Strathcona County" for Sherwood Park), consistently.
3. Keep a curated override only if no systematic layer can cover all markets.

The deciding test: a system applied uniformly, not a list of rescued places.

---

## 11. Override question — VERDICT: drop it (GIS agent, 2026-05-22)

**Decision: Outcome 2 — drop sub-municipal settlement resolution; retire
`ca_places_nominatim.json`.** The Regional Market = the incorporated municipal /
CSD polygon the centroid falls in — uniformly, all 17 countries, no exceptions.

- **No systematic global sub-municipal layer exists.** Every candidate — OSM
  `place=*`, GeoNames, Who's On First, Natural Earth — is point geometry
  (forcing a fuzzy nearest-place radius rule that does not compose with the
  municipal point-in-polygon resolver) or has admittedly incomplete coverage.
  The city/town/village/hamlet tier is per-mapper judgement, not a population
  rule — Sherwood Park is OSM `place=hamlet` at 72,017 people. **Outcome 1 is
  not achievable.**
- **The override fails the operator's principle** — open-ended (grows with
  every ingest), a non-reproducible one-time Nominatim snapshot, Canada-only.
- **"Strathcona County" for Sherwood Park is a correct geographic statement** —
  Sherwood Park is an *unincorporated hamlet*, the urban service area of
  Strathcona County, with no municipal polygon of its own. The county name is
  true, consistent with Leduc County etc., and coherent as a wiki TOPIC title.
- **This supersedes §9 fix-#1** ("re-key the override"): do not re-key —
  **delete it.** Removing the override is what makes resolution geometry-derived
  and cluster-ID-independent — the goal §9 wanted, achieved by deletion.
- **Implementation = mostly deletion:** remove the Nominatim override branch +
  the `cluster_id` parameter from `resolve_market()`, the override-file load,
  and `build_ca_nominatim()`. The CA branch collapses to: CSD point-in-polygon
  → `high` confidence if no county keyword, `medium` if a county keyword. US /
  MX / EU unchanged. **One rule, 17 countries, zero curated exceptions.**

---

## 12. Dense-metro granularity — VERDICT: geometric self-clustering (2 agents, 2026-05-22)

The §6 open item — a big principal-city municipality (City of Chicago, Madrid)
is one oversized Regional Market = one oversized wiki TOPIC. The operator asked
whether the CBRE / Oxford Economics framework provides a global solution.

**It does not.** CBRE EA submarkets are CoStar-licensed, US-only for retail,
broker-drawn judgement geography — not a real boundary dataset, not global.
Oxford Economics has no sub-metro geography at all. And no global sub-city
*administrative* boundary layer exists either (OSM `admin_level` 9/10 is
per-mapper and absent for most US/CA cities; the EU has no sub-LAU tier) — the
§11 conclusion repeats one tier up. CBRE/Oxford give the Metro Market *label*
(§9, correct); they give nothing *below* the municipality.

**Verdict — geometric self-clustering.** A Regional Market TOPIC is published
whole *unless* it holds more than ~8 co-locations, in which case its
co-locations are partitioned into **District sections** by single-linkage
agglomerative clustering of their own centroids (~8 km cut). Global by
construction, no external data, reproducible — the same rule everywhere.
It is a **wiki-presentation rule** in `generate-rm-topics.py` only — no schema
change, no map layer, no breadcrumb level, `region_engine.py` untouched.
Districts named geometrically (ordinal + quadrant), never curated. The Regional
Market and Metro Market models (§9) are unchanged. See `BRIEF-BUILD-SPEC` §6.4.
