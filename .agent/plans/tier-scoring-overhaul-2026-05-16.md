# Tier Scoring Overhaul — Consolidated Plan

**Owner:** project-gis Totebox session (execution); operator (gate decisions).
**Source-of-truth:** this document. Supersedes the brief (Agent D weighted-percentile) and to-do (Agent F pure-predicate) intermediates in the same directory; those remain as historical record.
**Drafted:** 2026-05-16, consolidated from seven research agents (A–G) plus five operator decisions captured in-session.

---

## 1. Executive summary

The map's two competing tier systems collapse into one. A 4-class anchor taxonomy replaces the 3-set arrangement; the V2 weighted score is retired in favour of a pure-predicate engine that reads the Sprint 14 catchment integrals as per-ISO percentile gates plus geometric overlap. The bento inspector promotes tier-as-quality-grade to a single dominant badge and drops the sub-score breakdown. The change is additive at the data layer (`score_final` becomes audit-only), conservative at the UI layer (one bento redesign, no new layers), and unblocks European Apex coverage that is structurally impossible today.

**Headline changes:**

- New anchor taxonomy: Hypermarket / Warehouse-Club / Lifestyle / Hardware (4 classes, was 3).
- Tier engine becomes pure predicate over composition + per-ISO catchment percentiles + civic + IoU; no weights, no `score_final` arithmetic.
- Map badge unifies on the 4-tier quality grade — **Regional / District / Local / Fringe** (locked 2026-05-16, G1); composition (`tier_descriptor`) demotes to a secondary chip.
- Bento inspector redesigned to tier-first BentoBox (Layout A desktop, Layout B mobile).
- EU Apex unblocks via Mercadona-ES / Tesco-UK / Sainsbury's-UK (config-only Phase 1), then Carrefour-FR re-ingest (Phase 5).

---

## 2. Operator decisions (D1–D5, captured)

| # | Decision | Rationale |
|---|---|---|
| **D1** | Mercadona-ES promoted to ALPHA-ES (parallel to Soriana-MX). | Spain flagship hypermarket; 1,603 records already ingested; config-only change. |
| **D2** | Map badge = 4-tier quality grade (primary); 8-label composition (`tier_descriptor`) demotes to secondary chip. | Today's split — `tier_descriptor=="Prime"` n=1,145 vs `rank_v2==T3` n=28 — confuses readers. One badge resolves it. |
| **D3** | Apex requires (Costco + Hypermarket) OR (IKEA + Hypermarket). Single-anchor insufficient. | Apex is a co-location product. A lone destination anchor does not constitute one. |
| **D4** | Cutoffs are geometric / percentile, not arbitrary numeric thresholds. | Operator's words: "geometrically driven." Self-bounded; saturation guard becomes unnecessary. |
| **D5** | 4 anchor classes: Hypermarket / Warehouse-Club / Lifestyle / Hardware. IKEA exits ALPHA_ANCHORS into its own Lifestyle class; Costco stays Warehouse; "Hypermarket" reserved for Walmart/Tesco/Soriana-class general-merchandise+grocery chains. | Restores semantic precision lost when the 3-set arrangement lumped IKEA with hypermarkets. |

---

## 3. Pure-predicate tier definitions (Agent F, verbatim)

```
TIER_1 (currently "Apex")  ⇔
       ( {Costco}  ⊆ anchors(C)  ∧  {Hypermarket-family} ∩ anchors(C) ≠ ∅ )
    ∨  ( {IKEA}    ⊆ anchors(C)  ∧  {Hypermarket-family} ∩ anchors(C) ≠ ∅ )
    ∧  rank_pp_iso(C) ≤ p10
    ∧  rank_sp_iso(C) ≤ p20
    ∧  ∃ h ∈ hospitals_classified(C, "regional", tertiary_ring)
    ∧  IoU_max(C, T1∪T2_higher_in_ISO) ≤ 0.10

TIER_2 (currently "Hub")   ⇔
       ( {Hypermarket-family} ∩ anchors(C) ≠ ∅
         ∧  {AlphaHW ∪ AlphaWH} ∩ anchors(C) ≠ ∅ )
    ∧  rank_pp_iso(C) ≤ p25
    ∧  ( rank_pg_iso(C) ≤ p25 ∨ rank_ph_iso(C) ≤ p25 ∨ rank_pw_iso(C) ≤ p25 )
    ∧  ∃ h ∈ hospitals_classified(C, {"regional","district"}, tertiary_ring)
    ∧  IoU_max(C, T1) ≤ 0.25

TIER_3 (currently "Valid") ⇔
       ( {AlphaHW ∪ AlphaWH} ∩ anchors(C) ≠ ∅ )
    ∧  rank_pp_iso(C) ≤ p50
    ∧  ∃ h ∈ hospitals_classified(C, any, tertiary_ring)

TIER_4 (currently "Borderline") ⇔  none of the above
```

Notation: `rank_pp_iso(C) ≤ p10` means "C's primary-ring population rank, within its ISO, is in the top decile." Each predicate is a hard gate; all conjuncts must be satisfied.

---

## 4. Anchor taxonomy (4 classes)

| Class | Members (NA + EU) |
|---|---|
| **Hypermarket** — general-merchandise + groceries, daily-shopping format | walmart-us, walmart-ca, walmart-mx, target-us, soriana-mx, mercadona-es (new D1), tesco-uk (Phase 1), sainsburys-uk (Phase 1), bilka-dk, k-citymarket-fi, prisma-fi, hagkaup-is (migrate from ALPHA_ANCHORS["EU"]); pending re-ingest: carrefour-fr (Phase 5), auchan-fr (Phase 6), leclerc-fr (Phase 6), edeka-de E-Center/Marktkauf scope (Phase 6) |
| **Warehouse-Club** — membership-bulk | costco-us, costco-ca, costco-mx, costco-uk, costco-fr, sams-us, sams-mx, bjs-us, makro-nl |
| **Lifestyle** — destination furniture/home goods | ikea-us, ikea-ca, ikea-mx, ikea-uk, ikea-fr, ikea-de, ikea-at, ikea-nl, ikea-pt, ikea-nordics |
| **Hardware / Home-Improvement** | homedepot-us, homedepot-ca, homedepot-mx, lowes-us, lowes-ca, rona-ca, bq-uk, obi-de, obi-it, obi-pl, leroy-merlin-pt, leroy-merlin-es, leroy-merlin-fr, bauhaus-de, bauhaus-es, bauhaus-se, hagebau-de, toom-de, hornbach-de, hornbach-at, castorama-fr, castorama-pl, brico-depot-es, praxis-nl, k-rauta-fi, obs-bygg-no |

Held out (deliberate, document the rejection in V3 methodology to prevent re-litigation): lidl, aldi (discount-grocery, ~1,500 m² — semantically not Hypermarket); rewe-de (only ~120 REWE Center are true hypermarkets, too small a subset).

---

## 5. EU coverage matrix

Before any change, after Phase 1 config promotion, after Phase 5 Carrefour re-ingest.

| Country | Today | After Phase 1 | After Phase 5 | Notes |
|---|---|---|---|---|
| US, CA, MX | Apex-reachable | same | same | walmart / target / soriana cover hypermarket leg |
| ES | not Apex-reachable | **Apex-reachable** | same | mercadona-es promotion (D1) |
| UK | not Apex-reachable | **Apex-reachable** | same | tesco-uk + sainsburys-uk promotion |
| Nordic (DK, FI, IS, SE-via-NORDICS) | Apex-reachable but ALPHA_ANCHORS-routed | **Apex-reachable** via correct class | same | migrate bilka / k-citymarket / prisma / hagkaup into ALPHA_HYPERMARKET |
| FR | not Apex-reachable | not Apex-reachable | **Apex-reachable** | carrefour-fr re-ingest required (current 509 of ~5,200 stores) |
| DE | not Apex-reachable | not Apex-reachable | not Apex-reachable until Phase 6.3 | edeka-de scoped to E-Center/Marktkauf only |
| IT, PL, NL, PT, AT, NO, CZ, GR | not Apex-reachable | not Apex-reachable | Phase-6+ work | each requires its own hypermarket-class ingest |

EU Apex coverage doubles in scope at Phase 1 ship; closes the structural gap that produces T3=4 globally today.

---

## 6. BentoBox inspector layouts (Agent G)

### Layout A — Tier-first, desktop default

```
┌──────────────────────────────────────────────┐
│  Edmonton South Common                       │
│  Edmonton, AB · Canada                       │
├──────────────────────────────────────────────┤
│  ███ APEX                                    │
│  Hypermarket + Hardware + Warehouse          │
├─────────────────────┬────────────────────────┤
│  NA RANK            │  SITES IN RING         │
│  #14                │  23                    │
│  of 3,758           │  at 3 km · 8 at 1 km   │
├─────────────────────┴────────────────────────┤
│  ANCHORS                                     │
│  [ Walmart Supercentre ]  [ Costco ]         │
│  [ Home Depot ]                              │
│  + RONA · Canadian Tire                      │
│  Co-located within 150 m: Real Canadian SS   │
├──────────────────────────────────────────────┤
│  CATCHMENT                                   │
│  ~118,000 people · 35 km                     │
│  (hover for 150 km secondary + spend)        │
│  [ Show on Map ]                             │
├──────────────────────────────────────────────┤
│  CIVIC CONTEXT                               │
│  1 hospital · 2 universities within 5 km     │
└──────────────────────────────────────────────┘
```

### Layout B — Anchors-first, mobile (< 480 px)

```
┌──────────────────────────────────────────────┐
│  Edmonton South Common      [ APEX ]         │
│  Edmonton, AB · Canada                       │
├──────────────────────────────────────────────┤
│  Walmart Supercentre · Costco · Home Depot   │
│  RONA · Canadian Tire · Real Canadian SS     │
│  Hypermarket + Hardware + Warehouse          │
├─────────────────────┬────────────────────────┤
│  23 sites · 3 km    │  ~118,000 people       │
│  #14 in N. America  │  35 km catchment       │
├──────────────────────────────────────────────┤
│  1 hospital · 2 universities within 5 km     │
│                                              │
│  [ Show catchment on map ]                   │
└──────────────────────────────────────────────┘
```

The composition chip ("Hypermarket + Hardware + Warehouse") is the demoted secondary chip per D2. The merged-zones italic disclosure (`Co-located within 150 m: …`) preserves verbatim from the B6 staged DESIGN-RESEARCH draft.

---

## 7. Test matrix — 6 reference locations

| Location | Expected tier (F) | Delta vs V2 weighted | Notes |
|---|---|---|---|
| Mississauga Heartland | Tier 1 | same | Canonical Apex archetype. Walmart + Costco + Home Depot, dense primary-ring population, regional hospital. |
| Madrid Salamanca | Tier 1 | upgrade | Currently fails for lack of ES Hypermarket alpha; D1 unblocks. |
| Sherwood Park | Tier 1 | upgrade | Costco within 3.96 km satisfies composition gate at 3 km primary disk; closes outstanding-todo E2. |
| Mountain View | Tier 2 | demotion | V2 smoothed via `count_bonus`; under pure-geometric, rank_pp_iso likely misses p10 or IoU exceeds 0.10. Document explicitly in Phase 2.11 diff harness. |
| Anderlecht | Tier 2 | upgrade | If Makro qualifies as warehouse leg. Carrefour pending Phase 5 still gates Tier 1. |
| Camden | Tier 3 | 2-tier demotion | Composition gate fires hard — no Costco/IKEA + Hypermarket combo in inner-London format. Correct outcome under D3. |

---

## 8. Operator decisions (15) — locked 2026-05-16

All gates closed. Two operator overrides on the agent recommendations are flagged below. The detailed rationale per gate is preserved as historical record of the reasoning that led to each decision.

### Summary table — all 15 decisions

| # | Gate | Decision | Vs recommendation |
|---|---|---|---|
| G1 | Tier names | **Set B — Regional / District / Local / Fringe** | matches |
| G2 | Anchor/Anchored conflict | **Moot under G1** (auto-resolved) | n/a |
| G3 | Badge chip char cap | **8 chars** (auto-resolved by G1) | matches |
| G4 | Apex 3rd Hardware leg | **No — keep two-leg composition** | matches |
| G5 | Spend axes at T1 | **Absent T1, disjunctive T2** | matches |
| G6 | Percentile thresholds | **Coarse: p10 / p25 / p50** | matches |
| G7 | IoU bounds | **Single value: 0.10 Apex / 0.25 Hub** | matches |
| G8 | Tier 4 structure | **Single bucket — Fringe** | matches |
| G9 | Costco + IKEA, no Hyper | **Tier 2 / District** | matches |
| G10 | Disclosure language | **Percentile-relative** | matches |
| G11 | Hub overlap predicate | **Inclusive (standalone-strong OR shadow)** | matches |
| G12 | UI tier count | **4 fully-defined tiers** (retire NEUTRAL catchall) | matches |
| G13 | Spend integrals on bento | **Hover-only on catchment cell** | matches |
| G14 | Civic cell shape | **Ship with hospital + university tier breakdown** (regional / district per record) | **operator override — richer than recommended** |
| G15 | `score_final` retention | **Cut entirely now** — no audit retention window | **operator override — purer than recommended** |

### Operator-override impact on phase implementation

- **G14 override** affects Phase 4.5: civic cell renders tier-broken-down counts (e.g. "1 regional hospital · 1 district hospital · 1 regional university") rather than aggregate counts. Verify `build-clusters.py` exposes per-tier `hc_count_regional`, `hc_count_district`, `he_count_regional`, `he_count_small` (or equivalent) — if only aggregate `hc_count` / `he_count` are emitted today, add the per-tier emission as a small extension to Phase 1.
- **G15 override** affects Phase 2.7: remove `score_final` from the data model in Phase 2 alongside the other V2 sub-scores; no audit-retention window. The Phase 2.11 diff harness compares `rank_v2` (read once before removal) against new `tier` in-memory; no GeoJSON emission of `score_final` post-Phase-2 ship.

### Per-gate rationale (historical record)

### G1 — Tier name set

**Recommendation: Set B (Regional / District / Local / Fringe).**

ICSC reach hierarchy is the universally-understood retail-real-estate vocabulary; the Spanish cognates are direct (Regional / Distrito / Local / Periferia), shortest set (avg 6.75 chars) with margin against the 12 px badge cap; "Marginal" in Set F carries a faintly pejorative tone that "Fringe" — read as geographic, not judgemental — avoids. The Bloomberg article standard prefers neutral structural language over functional verbs, which is the gap Set F's "Destination / Anchored" trips into.

**Alternatives:** Set F (Destination / Anchored / Standard / Marginal) reads cleaner to a developer audience but creates the G2 conflict and risks the badge-chip cap; rejected names (Apex / Hub / Valid / Borderline) remain non-shippable per Agent E.

**Gates:** Phase 4 final UI render; Phase 7 TOPIC/DESIGN-RESEARCH drafts (B9, B14).

**Trigger to revisit:** if the operator prefers "function-named tiers because the map is sold as a function-discovery tool, not a hierarchy-display tool," swap to Set F and resolve G2 with Convergent.

---

### G2 — "Anchor"/"Anchored" conflict

**Recommendation: moot under G1 = Set B.** If G1 flips to Set F, use **Convergent** (10 chars, Sp: Convergente).

If "Anchored" is the Tier-2 label and "Anchor" the tenant role, the bento renders "Anchored: Walmart, Costco, Home Depot" with both meanings on screen. "Convergent" eliminates the collision and reads cleaner as "co-location density attribute" rather than "tenant present."

**Alternatives:** keep "Anchored" and disambiguate by typographic weight (rejected — accessibility-fragile); drop the tier label entirely on the bento and rely on color (rejected — fails screen-reader audit).

**Gates:** Phase 4, Phase 7.

---

### G3 — Badge chip char cap

**Recommendation: 8 chars hard cap.**

Designing to 8 chars constrains G1 to Set B regardless of preference, which is the right architectural call: a tier system that doesn't fit a 12 px chip is a tier system that breaks on mobile. Agent G's Layout B is already constrained by mobile width; 8-char headroom keeps the chip at single-line on every viewport in production. The accessibility-research convention is to target 7±2 chars for tier labels at chip scale.

**Alternatives:** 10-11 chars allows Set F at desktop, but Set F renders "Destination" at chip-overflow on mobile — solving that with abbreviation reintroduces "Apex"-class compression artifacts.

**Gates:** Phase 4.1 (TIER_COLORS palette + badge sizing).

---

### G4 — Apex composition: third leg required?

**Recommendation: NO. Keep two-leg composition; do not add Hardware-Alpha as a third conjunct.**

The hospital/civic predicate (`regional` at tertiary_ring) is the third axis. Adding Hardware-Alpha collapses Tier 1 to nearly empty — the Mississauga / Sherwood Park / Edmonton South Common archetypes all have Hardware-Alpha, but the EU Tier-1 candidates after Phase 5 (Madrid Salamanca, central Manchester) routinely do not within 3 km. A three-leg retail gate would re-create the EU-coverage problem in a new form.

**Alternatives:** require Hardware-Alpha (rejected, kills EU); require any third Alpha-class (more permissive but lets `Hyper + Lifestyle + Lifestyle` qualify, which is incoherent — IKEA is one site).

**Gates:** Phase 2.1 (`build-geometric-ranking.py` predicate emission).

---

### G5 — Spend axes at Tier 1: conjunctive or disjunctive?

**Recommendation: disjunctive within Tier 2; absent from Tier 1.**

Tier 1 already has the primary + secondary population gates (rank_pp_iso ≤ p10 AND rank_sp_iso ≤ p20) — these are the load-bearing geometric constraints. Adding spend conjuncts at Tier 1 over-constrains the predicate and creates dependency on the spend-estimate vintage (which is per-capita-multiplier-based, not direct). At Tier 2, the disjunctive form `(rank_pg ∨ rank_ph ∨ rank_pw) ≤ p25` captures the "specialist retail node" signal without forcing all three categories to clear simultaneously.

**Alternatives:** conjunctive at Tier 2 (rejected — would demote 60%+ of current Hub candidates that excel in one spend category, not all three); disjunctive at Tier 1 (rejected — population gates already do the work, spend would be cosmetic redundancy).

**Gates:** Phase 2.1.

---

### G6 — Percentile thresholds

**Recommendation: Agent F's coarse set (p10 / p25 / p50).**

Finer deciles (p10/p20/p30/p50) imply precision the underlying data does not warrant — clusters-meta has 6,815 entries unevenly distributed across 13 ISOs, several with sub-100 cluster counts. Decile boundaries on a 47-cluster ISO are noise. The coarse set keeps tier assignment stable across pipeline rebuilds, which is the disclosure property NI 51-102 cares about more than the granularity property.

**Alternatives:** finer deciles (rejected for stability); ISO-population-adjusted floors (rejected — adds parameter requiring its own justification).

**Gates:** Phase 2.1, 2.10 (V3 disclosure language).

---

### G7 — IoU bounds

**Recommendation: single value per tier (0.10 Apex, 0.25 Hub).**

Ring-dependent IoU adds a parameter that must be defended in disclosure for marginal gain. The single-value bound is already conservative enough — Tier 1 with 10% overlap permits genuinely-distinct dense urban Apex sites while excluding ring-overlap artifacts. Disclosure simplicity favours the single-value form.

**Alternatives:** ring-dependent (primary 0.05, secondary 0.15 — rejected, no empirical basis; would require an Agent-H research run); IoU on H3-cell overlap rather than disk overlap (rejected — premature optimization, disk form is sufficient at current resolution).

**Gates:** Phase 2.3 (`IoU_max` implementation).

---

### G8 — Tier 4 structure

**Recommendation: single bucket — "none of the above."**

T4a/T4b adds a sub-tier distinction the badge cannot communicate (one color, one chip). The civic-absent vs civic-present distinction is interesting analytically but belongs in the `tier_predicates_fired` audit list, not in the UI surface. Splitting Tier 4 multiplies the dimensions readers must absorb at exactly the layer where the redesign is collapsing dimensions.

**Alternatives:** T4a/T4b with paired NEUTRAL palette shades (rejected — defeats the unification logic of D2).

**Gates:** Phase 2.1, Phase 4.1.

---

### G9 — Costco + IKEA, no Hypermarket

**Recommendation: Tier 2.**

D3 explicitly requires Hypermarket-class for Tier 1. A Costco + IKEA pair without a Walmart/Tesco/Carrefour-class anchor is exactly the rare-but-legitimate "destination cluster without daily-shopping function" archetype — the cluster pulls regional traffic but doesn't serve a primary daily-shop catchment. Tier 2 captures that correctly: composition is strong, but daily-shop function is absent.

**Alternatives:** Tier 1 by relaxing D3 (rejected — operator explicitly affirmed D3 in this session); Tier 3 (rejected — the composition is genuinely above standard).

**Gates:** Phase 2.1.

---

### G10 — Percentile vs absolute disclosure

**Recommendation: percentile language ("Tier 1 = top decile by primary-ring population within country").**

Self-bounded language is the BCSC-NI-51-102 disclosure preference — it cannot become stale as data refreshes. Absolute thresholds ("≥ 1.2 M people") require the disclosure document to update on every pipeline rebuild that shifts the distribution, which compounds against the visible-operational deployment cadence. Percentile language also reads naturally as "this is the methodology, here is where this cluster falls in it," which is the BCSC reasoning chain.

**Alternatives:** absolute ("Tier 1 = catchment population ≥ 1.2 M") — stable in absolute terms but goes stale when 2027 census ingestion shifts the floor; mixed ("Tier 1 requires top decile AND ≥ X people") — combines the costs of both with the benefits of neither.

**Gates:** Phase 2.10 (SCORING-METHODOLOGY.md V3 rewrite); Phase 7.1 (TOPIC).

---

### G11 — Hub overlap requirement

**Recommendation: inclusive (Agent F's draft — Hub passes if standalone-strong OR overlaps a stronger neighbour).**

The inclusive form correctly captures both archetypes: the standalone Tier-2 ("good cluster in a small market") and the shadow-of-Apex Tier-2 ("good cluster sitting near an Apex"). Requiring both narrows Hub to shadow-only, which mis-tiers single-strong-cluster regional markets that have no Apex above them.

**Alternatives:** require both (Hub = shadow-of-Apex only — rejected, miscategorizes regional dominants); drop overlap entirely from Hub (rejected — loses the structural signal of "exists in a hierarchy").

**Gates:** Phase 2.1.

---

### G12 — Tier count: 3 or 4?

**Recommendation: 4 fully-defined tiers (add a Tier 4 color, retire NEUTRAL as catchall).**

The current 3 + NEUTRAL arrangement at `www/index.html:336-342` makes Tier 4 look like a data error ("Unknown" fallback) rather than a substantive classification. Pure-predicate F treats Tier 4 as a real assignment — "Fringe" / "Borderline" — and the UI must reflect that. Adding the fourth palette slot is a one-line change and prevents the recurring user question "why are these clusters grey, is the data broken?"

**Alternatives:** keep 3 + NEUTRAL fallback (rejected — semantically wrong under D2); 4 tiers with NEUTRAL preserved for genuine-missing-data (acceptable hybrid, ship if the data has nullable cases).

**Gates:** Phase 4.1.

---

### G13 — Spend integrals (pg / ph / pw) on bento

**Recommendation: hover-only on the catchment cell.**

The spend integrals are real signal but visually heavy at default render. Hover-only preserves them for power users without competing with the tier-first hierarchy in the redesign. Mobile (Layout B) cuts them entirely — touch interfaces have no hover surface, and the mobile bento is already constrained to five cells.

**Alternatives:** omit entirely (rejected — loses audit value); always-visible (rejected — defeats Layout A's hierarchy).

**Gates:** Phase 4.4.

---

### G14 — Civic context cell: data source

**Recommendation: ship the civic cell in Phase 4 — fields are already in clusters-meta.**

`build-clusters.py:222, 407` already emits `hc_count` (distinct hospitals within tertiary ring) and the equivalent `he_count` for universities. The bento can consume these directly. No pipeline change required to ship the Layout-A "1 hospital · 2 universities within 5 km" line. If the tier classifier requires the same data, Phase 2 already wires it; the bento read is read-only.

**Alternatives:** defer civic cell to follow-up (rejected — fields already exist, deferral is gratuitous); render hospital tier-class explicitly (regional / district) (defer — interesting but adds visual weight; revisit after Phase 4 ships).

**Gates:** Phase 4.5.

---

### G15 — `score_final` retention

**Recommendation: retain in GeoJSON for audit; cut entirely from UI.**

The pure-predicate engine makes `score_final` non-load-bearing for tier assignment, but the rebuild diff (Phase 2.11) and the V2 → V3 transition audit benefit from keeping the field one sprint longer. The UI is where the visible-operational cadence and "no breakdown panel" anti-pattern enforce its removal — the bento has no place for a 0-1000 number that does not drive any decision. Agent G's "hover-detail audit trail" recommendation is well-meaning but would re-introduce exactly the score-watching behaviour the redesign exists to retire.

**Alternatives:** cut entirely from GeoJSON (acceptable later — defer one sprint for audit); hover-detail in inspector (rejected — re-anchors readers on a deprecated number).

**Gates:** Phase 2.7, Phase 4.3.

---

## 9. Phased to-do (8 phases)

Phase numbering preserves the to-do file's structure. Items reflect the gate recommendations above; revisit if any gate flips.

### Phase 1 — Taxonomic restructure + immediate-promote (config-only, no re-ingest)

- [ ] **1.1** Split `config.py:23-116` into 4 sets: `ALPHA_HYPERMARKET`, `ALPHA_WAREHOUSE`, `ALPHA_LIFESTYLE`, `ALPHA_HARDWARE`.
- [ ] **1.2** Migrate members per D5: HYPERMARKET ← walmart-*, target-*, soriana-*, bilka-dk, k-citymarket-fi, prisma-fi, hagkaup-is (from existing ALPHA_ANCHORS["EU"]) + new mercadona-es, tesco-uk, sainsburys-uk. WAREHOUSE ← costco-*, sams-*, bjs-*, makro-*. LIFESTYLE ← ikea-*. HARDWARE ← preserves existing ALPHA_HARDWARE.
- [ ] **1.3** Update `build-clusters.py:120-140 evaluate_tier()` to consume the new 4-class taxonomy.
- [ ] **1.4** Update `build-clusters.py:323-328` anchor category derivation for the new IKEA-as-Lifestyle classification.
- [ ] **1.5** Update `tier_descriptor` composition labels in `build-clusters.py:338-353` (will demote to secondary chip in Phase 4 — language can land now).
- [ ] **1.6** Pipeline rebuild — `build-clusters.py → generate-rankings.py → build-tiles.py`.
- [ ] **1.7** Sanity-check distribution: count clusters per `tier_descriptor` per ISO. Expect FR/GB/DE Apex/Prime counts to increase from ~0 to non-trivial after Mercadona/Tesco/Sainsbury's are promoted.
- [ ] **1.7a** **Per-tier civic count emission** (added 2026-05-16 per G14 override). Verify `build-clusters.py` emits `hc_count_regional`, `hc_count_district`, `he_count_regional`, `he_count_small` per cluster (split from aggregate `hc_count` / `he_count`). If not present, add the per-tier emission consuming the existing `hospital_tier` / `university_tier` fields from `ingest-osm-civic.py:100-141`. Gates Phase 4.5 civic cell render.
- [ ] **1.8** Stage 6 promote → canonical → live tiles.

### Phase 2 — Pure-geometric ranking engine

Gated by G4–G11. Replaces V2 weighted score with pure predicates.

- [ ] **2.1** Write `build-geometric-ranking.py` per Agent F skeleton — assigns `tier` ∈ {1,2,3,4} and `tier_predicates_fired` (list of predicate strings).
- [ ] **2.2** Extend `synthesize-od-study.py:192-213` to produce per-ISO percentile ranks for all 8 catchment axes (pp/sp/pg/sg/ph/sh/pw/sw) — currently only 4 are ranked.
- [ ] **2.3** Implement `IoU_max(C, stronger_clusters)` as a single O(N²) post-pass over primary-disk polygons (closed-form lens-area for equal-radius circles).
- [ ] **2.4** Implement `hospitals_classified(C, tier_filter, ring)` predicate consuming the existing `hospital_tier` field from `ingest-osm-civic.py:100-115`.
- [ ] **2.5** Apply tier-stepping civic predicates: Tier 1 needs regional, Tier 2 accepts district, Tier 3 accepts any, Tier 4 no civic predicate (per §3).
- [ ] **2.6** Add lexicographic tiebreaker for ordinal-tied clusters: anchor-count desc → pp desc → cluster_id.
- [ ] **2.7** Remove from emitted geometry: `score_base`, `score_count_bonus`, `score_diversity_bonus`, `score_multi_anchor`, `score_civic_depth`, `score_overlap_penalty`, `rank_v2`, **and `score_final`** (per G15 operator override — no audit-retention window; the Phase 2.11 diff harness reads `rank_v2` in-memory before removal).
- [ ] **2.8** Remove saturation guard loop (`generate-rankings.py:241-251`) — top-decile-within-ISO is inherently ≤ 10%.
- [ ] **2.9** Move `generate-rankings.py` to `legacy/generate-rankings-v2.py` (rollback path); update build scripts to call `build-geometric-ranking.py`.
- [ ] **2.10** Rewrite `SCORING-METHODOLOGY.md` as V3 — percentile-language disclosure per G10.
- [ ] **2.11** Pipeline rebuild + diff harness — emit 4×4 contingency table of old `rank_v2` × new `tier` to identify promotions/demotions for review.
- [ ] **2.12** Stage 6.

### Phase 3 — Civic-tier predicate refinement (optional enrichment)

Largely folded into Phase 2.

- [ ] **3.1** Enrich `_HOSPITAL_REGIONAL_SIGNALS` (`ingest-osm-civic.py:81-88`) with non-English tokens: `hospital comarcal`, `ospedale civile`, `szpital wojewódzki`, `hôpital universitaire`, `klinikum`, etc.
- [ ] **3.2** Optional: integrate IPEDS EF2023A.zip (outstanding-todo F1) for stricter US `regional` university classification (≥ 5,000 students).
- [ ] **3.3** Re-ingest civic-osm with updated regional signals.
- [ ] **3.4** Re-run Phase 2 pipeline to absorb new civic classifications.

### Phase 4 — BentoBox inspector redesign

Gated by G1 (final names), G3 (chip cap), G12 (tier count), G13–G15.

- [ ] **4.1** Update `www/index.html:336-340` TIER_COLORS palette — 4 tier slots per G12; retire NEUTRAL fallback or downgrade to genuine-missing-data only.
- [ ] **4.2** Rebuild `showClusterDetail()` (`www/index.html:1290-1452`) for Layout A: tier badge dominant (40-50 px, full-width), composition chip (24 px, muted) beneath.
- [ ] **4.3** Cut from default render: density/km², `score_final`, sub-score breakdown (Shannon, count_bonus, civic_depth, overlap_penalty).
- [ ] **4.4** Demote to hover/long-press: 150 km secondary catchment, NA-vs-national rank duality, unique_brands count, spend integrals (pg/ph/pw per G13).
- [ ] **4.5** Keep visible: place name + region, tier badge + composition chip, rank cell (one rank), sites-in-ring cell (3 km headline + 1 km muted), anchors pill list, catchment headline, **civic context line with tier breakdown** per G14 — render as "1 regional hospital · 1 district hospital · 1 regional university · 1 small university" (not aggregate "2 hospitals · 2 universities"). Requires `build-clusters.py` to expose per-tier counts (`hc_count_regional` / `hc_count_district` / `he_count_regional` / `he_count_small`) — if only aggregates are emitted today, add per-tier emission to Phase 1 scope.
- [ ] **4.6** Preserve `merged_zones` co-location disclosure italic line (`:1435`) exactly — already correct per B6 staged draft.
- [ ] **4.7** Apply final tier names from G1 throughout `www/index.html` + styling + clusters-meta-emitting code.
- [ ] **4.8** Mobile responsive: `@media (max-width: 480px)` collapse to Layout B; hide secondary catchment, rank duality, civic context, density; replace with `[ Full detail ]` modal sheet.
- [ ] **4.9** Anti-pattern guard: do NOT add a sub-score breakdown panel. If a future user asks "why Tier 1?", answer with a one-line predicate hover sourced from the methodology TOPIC.
- [ ] **4.10** Visual regression — before/after screenshots of 8–10 reference clusters from §7.

### Phase 5 — Carrefour-FR re-ingest

Outstanding-todo A1. Unlocks French Tier 1 coverage.

- [ ] **5.1** Update Carrefour YAML: `wikidata_id: ~`, `name_query: "Carrefour"`, `name_query_partial: true` (Aldi-NL precedent).
- [ ] **5.2** Re-ingest carrefour-fr — expect ~5,200 stores vs current 509.
- [ ] **5.3** Promote carrefour-fr to `ALPHA_HYPERMARKET`.
- [ ] **5.4** Pipeline rebuild.
- [ ] **5.5** Sanity-check: France Tier 1 count should go from 0 to non-trivial.
- [ ] **5.6** Stage 6.

### Phase 6 — Subsequent EU chain coverage

- [ ] **6.1** Ingest auchan-fr — Wikidata Q758603, ~600 stores. Promote to `ALPHA_HYPERMARKET`.
- [ ] **6.2** Ingest leclerc-fr — E.Leclerc cooperative; ~720 hypermarkets. Promote to `ALPHA_HYPERMARKET`.
- [ ] **6.3** Ingest edeka-de scoped to E-Center/Marktkauf only (~600 of ~11,000 affiliates).
- [ ] **6.4** Document explicit non-ingest of rewe-de, lidl, aldi in `SCORING-METHODOLOGY.md` V3.

### Phase 7 — Documentation + Wiki

- [ ] **7.1** Stage TOPIC: pure-geometric ranking methodology (replaces V2 prose) → project-editorial.
- [ ] **7.2** Update staged TOPIC: co-location tier nomenclature (B14) with final names from G1.
- [ ] **7.3** Update staged DESIGN-RESEARCH: tier-naming-accessibility (B9) with chip-size validation from Phase 4.
- [ ] **7.4** Update staged DESIGN-RESEARCH: bento merged zones disclosure (B6) — already preserved in Phase 4.6.
- [ ] **7.5** Update `DATA-MANIFEST.md` to reflect 4-class taxonomy + pure-geometric ranking.
- [ ] **7.6** Rewrite `SCORING-METHODOLOGY.md` to V3 (done in Phase 2.10, restated here for tracking).

### Phase 8 — Backlog closure

- [ ] **8.1** D2 Fred Meyer ALPHA removal review — decide fred-meyer-us ∈ `ALPHA_HYPERMARKET` (PNW-regional ~92-132 stores).
- [ ] **8.2** E2 Sherwood Park 3 km gap — verify Tier 1 under pure-geometric post-Phase 2.
- [ ] **8.3** F1 IPEDS EF2023A.zip — feeds Phase 3.2.
- [ ] **8.4** A1 Carrefour-FR — closed by Phase 5.
- [ ] **8.5** C1 Auchan-FR — closed by Phase 6.1.
- [ ] **8.6** C2 Mercadona-ES — closed by Phase 1.2.

---

## 10. Parallelization + dependencies

```
Phase 1 (config + 4-class taxonomy)
    │
    ├──► Phase 4 (bento redesign — can start in parallel once G1–G3, G12–G15 answered)
    │
    └──► Phase 2 (pure-geometric engine — needs G4–G11)
            │
            ├──► Phase 3 (civic enrichment — optional)
            │
            └──► Phase 5 (Carrefour re-ingest — needs new ALPHA_HYPERMARKET set live)
                    │
                    └──► Phase 6 (Auchan / Leclerc / Edeka — sequential ingests)

Phase 7 (docs) interleaves with all phases as artifacts produced.
Phase 8 (backlog closure) is verification, not new work.
```

**Minimum viable ship = Phase 1 + Phase 2 + Phase 4.** One focused sprint for the project-gis Totebox. Phase 3, 5, 6, 7, 8 are follow-on.

---

## 11. Files touched

| File | Phase | Action |
|---|---|---|
| `config.py:23-116` | 1 | Split into 4 alpha-class sets; promote Mercadona / Tesco / Sainsbury's |
| `build-clusters.py:120-140, 323-328, 338-353` | 1 | New 4-class consumer; updated `_anchor_cat`; composition labels |
| `synthesize-od-study.py:192-213` | 2 | Extend per-ISO percentile ranks to all 8 axes |
| `build-geometric-ranking.py` (new) | 2 | Pure-predicate tier engine |
| `generate-rankings.py` → `legacy/generate-rankings-v2.py` | 2 | Move to legacy; rollback path |
| `ingest-osm-civic.py:81-88` | 3 | Non-English `_HOSPITAL_REGIONAL_SIGNALS` tokens |
| `www/index.html:336-340, 1290-1452` | 4 | TIER_COLORS (4 slots); Layout A + Layout B |
| `SCORING-METHODOLOGY.md` | 2, 7 | Rewrite as V3 (percentile-language disclosure) |
| `DATA-MANIFEST.md` | 7 | Reflect new taxonomy + ranking |
| Carrefour YAML (outstanding-todo A1) | 5 | Aldi-NL pattern |
| Auchan / Leclerc / Edeka YAMLs | 6 | New ingests |
| `.agent/drafts-outbound/topic-*-tier-*.draft.md` (B14) | 7 | Final names + methodology |
| `.agent/drafts-outbound/DESIGN-RESEARCH-tier-naming-*.draft.md` (B9) | 7 | Chip-size validation |
| `.agent/drafts-outbound/DESIGN-RESEARCH-bento-*.draft.md` (B6) | 4, 7 | Preserved; merged-zones italic carries through |

---

## 12. Risks / open concerns

- **Predicate fragility (Agent F §4).** One mistagged civic record flips a tier. Mitigation: lexicographic tiebreaker (Phase 2.6) + `tier_predicates_fired` audit list per cluster.
- **EU coverage gap before Phase 5/6 ship.** France/Germany Tier 1 remains thin until Carrefour re-ingest + Edeka E-Center ingest land. Phase 1 alone unlocks UK + Spain + Nordic correctness.
- **Mountain View demotion** (test matrix §7). V2 weighted yields Apex via `count_bonus` smoothing; pure-geometric demotes to Tier 2 if `rank_pp_iso` misses p10 or IoU > 0.10. Document explicitly in Phase 2.11 diff harness; operator review required before Stage 6.
- **"Why Tier 1?" question from users** (Agent G anti-pattern). Bento has no numeric breakdown; the answer lives in the methodology TOPIC plus `tier_predicates_fired` hover. If operators repeatedly request a breakdown panel, refuse — that is the failure mode this redesign exists to retire.
- **G3 ↔ G1 coupling.** If chip-cap measurement at Phase 4.1 reveals 8 chars is unachievable in the brand font at 12 px, Set F also fails and the operator must approve abbreviation. Validate with a UI measure before locking the tier-name copy in source.
- **Score_final transition.** Keeping the field one sprint for audit (G15) means readers may still see V2 numbers in raw GeoJSON exports during the transition window. Disclosure language in V3 SCORING-METHODOLOGY.md must explicitly mark the field as deprecated-pending-removal.
- **Disclosure-language stability under percentile thresholds.** Percentile language (G10) is self-bounded but means tier-X-cluster sets shift with every pipeline rebuild. This is correct behaviour but should be noted in the V3 methodology so external readers understand the field is recomputed, not absolute.

---

## 13. References

Code (verified 2026-05-16):

- `config.py:23-116, 119-123, 167, 172-186, 304-354` — anchor sets, region config, display names
- `build-clusters.py:30, 120-140, 188-246, 222, 257-342, 407` — radii, `evaluate_tier`, score computation, anchor classification, `hc_count` / `he_count` emission
- `generate-rankings.py:50-156, 128-129, 201-232, 235-265` — V2 scoring, civic placeholders, overlap penalty, tier cutoffs + saturation guard
- `synthesize-od-study.py:117-172, 192-213` — catchment integrals, per-ISO percentile ranks
- `ingest-osm-civic.py:60-141, 81-88, 100-115, 250` — hospital/university classifiers, regional-signals list, hospital_tier emission, denylist
- `www/index.html:336-342, 356, 1290-1452, 1332, 1435` — TIER_COLORS, `tierColor()`, `showClusterDetail()`, tier badge render, merged_zones italic disclosure

Operator memory (workspace `~/Foundry/.agent/memory/`):

- `feedback_bcsc_disclosure.md` — informs G10
- `feedback_language_standard.md` — informs G1, G7 naming neutrality
- `feedback_visible_operational_first.md` — informs G15 (cut from UI now, retain audit field)
- `feedback_credit_budget_ship_now_subset.md` — phase prioritization (Phase 1 + 2 + 4 = MVP)
- `feedback_no_doctrine_convention_in_public.md` — informs V3 methodology prose
- `project_gis_woodfinegroup.md` — current GIS deployment state; V2 distribution counts

Backlog:

- `.agent/outstanding-todo.md` A1 (Carrefour-FR), C1 (Auchan-FR), C2 (Mercadona-ES), D2 (Fred Meyer review), E2 (Sherwood Park 3 km gap), F1 (IPEDS EF2023A)
- `.agent/rules/artifact-registry.md` B6, B9, B14 (staged drafts touched by Phase 7)

Pipeline state at consolidation (2026-05-16): 7,434 clusters per `clusters.geojson`, 6,815 catchment-centroids, 13 countries OSM-ingested.

— consolidated 2026-05-16 from agents A–G + operator decisions D1–D5
