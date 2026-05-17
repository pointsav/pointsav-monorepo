# Co-Tenancy Tier Methodology

**Document type:** Disclosure specification  
**Version:** V3 (2026-05-16)  
**Applies to:** `build-geometric-ranking.py` output in `clusters.geojson`  
**Regulatory posture:** NI 51-102 continuous-disclosure / OSC SN 51-721  

---

## 1. Purpose

This document specifies the algorithm used to assign tiers to every co-location
cluster on the Woodfine GIS platform (gis.woodfinegroup.com). Tiers are published
as the `tier` feature property in `clusters.geojson` and rendered in the platform UI.
The methodology is reproducible from raw coordinates and publicly disclosed here.

**V3 replaces the V2 score-based system.** The composite `score_final` field and its
sub-components (`score_base`, `score_count_bonus`, `score_diversity_bonus`,
`score_multi_anchor`, `score_civic_depth`, `score_overlap_penalty`, `rank_v2`) have
been removed from all emitted geometry (operator override G15, 2026-05-16). The V2
scripts are retained at `legacy/generate-rankings-v2.py` as a rollback path.

---

## 2. Scope of Use

The tier system characterises **spatial proximity and brand diversity** of retail
co-location clusters. It is:

- Not a projection of financial return, revenue, foot traffic, or market share.
- Not a recommendation to acquire, develop, or lease any specific site.
- Not a predictor of future retail performance.

Forward-looking terms in this document ("planned," "intended," "may," "target")
reflect development intentions and are subject to change.

---

## 3. Anchor Taxonomy

Clusters are initiated by stores classified as one of four alpha anchor classes.
The initiating store's class determines the cluster's primary anchor category.

| Class | Chains (representative) |
|---|---|
| **ALPHA_HYPERMARKET** | Walmart (US/CA/MX), Target (US), Fred Meyer (US), Soriana (MX), Mercadona (ES), Tesco (UK), Sainsbury's (UK), Bilka (DK), K-Citymarket (FI), Prisma (FI), Obs Coop (NO), Hagkaup (IS), Carrefour (FR), Auchan (FR), E.Leclerc (FR), E center (DE), Marktkauf (DE), Kaufland (DE) |
| **ALPHA_LIFESTYLE** | IKEA (all regions) |
| **ALPHA_HARDWARE** | Home Depot (US/CA/MX), Lowe's (US/CA), Leroy Merlin (EU), Brico Dépôt (FR/ES), Bauhaus (EU) |
| **ALPHA_WAREHOUSE** | Costco (all regions), Sam's Club (US/MX), BJ's (US), Makro (ES/NL/PL) |

**Not ingested by design:** Rewe (DE), Lidl, Aldi — these chains operate as
neighbourhood grocery formats; their density (thousands of small stores per
country) would produce thousands of false-positive clusters below any useful
district threshold. Their absence is a deliberate semantic decision, not a
data gap.

**ALPHA_HYPERMARKET vs. cluster anchor:** Membership in ALPHA_HYPERMARKET
contributes a chain's stores to the `hyper_list` field of nearby clusters
and satisfies the composition predicate. It does not by itself initiate cluster
locations — only chains listed in `REGION_CONFIG[country]["anchor"]` seed new
cluster centroids. Carrefour (FR), Auchan (FR), E.Leclerc (FR), E center (DE),
and Marktkauf (DE) are ALPHA_HYPERMARKET members but are not currently
REGION_CONFIG anchors; their stores appear in `hyper_list` only when co-located
within 3 km of an existing IKEA or Costco anchor cluster.

---

## 4. Tier Definitions

Four tiers are assigned to each cluster.

| Tier | Name | Colloquial description |
|---|---|---|
| 1 | **Regional** | Major trade-area anchor; top decile by primary catchment population within country |
| 2 | **District** | Significant multi-format node; top quartile by primary catchment within country |
| 3 | **Local** | Hardware or wholesale hub with civic support |
| 4 | **Fringe** | Below threshold on one or more required gates |

Tier names follow the ICSC retail property hierarchy: Regional → District → Local.
Spanish cognates: Regional / Distrital / Local / Marginal.

---

## 5. Predicate Gates

Each tier requires all listed predicates to pass.

### 5.1 Tier 1 — Regional

| Gate | Predicate |
|---|---|
| **Composition** | Cluster contains (Warehouse ∧ Hypermarket) OR (Lifestyle ∧ Hypermarket) |
| **Primary population** | `rank_pp_iso` ≤ 0.10 — cluster is in the top 10% by primary catchment population within its ISO country |
| **Secondary population** | `rank_sp_iso` ≤ 0.20 — cluster is in the top 20% by secondary catchment population within its ISO country |
| **Civic — regional hospital** | `hc_count_regional` ≥ 1 — at least one regionally classified hospital within the tertiary ring |
| **IoU — non-overlap** | IoU with any stronger cluster in the same ISO ≤ 0.10 — cluster is not dominated by a higher-population peer within the 3 km disk radius |

### 5.2 Tier 2 — District

| Gate | Predicate |
|---|---|
| **Composition** | Cluster contains Hypermarket ∧ (Hardware OR Warehouse) |
| **Primary population** | `rank_pp_iso` ≤ 0.25 — cluster is in the top quartile by primary catchment population within its ISO country |
| **Spend rank** | `rank_pg_iso` ≤ 0.25 OR `rank_ph_iso` ≤ 0.25 OR `rank_pw_iso` ≤ 0.25 — cluster is in the top quartile by at least one spend axis within its ISO country |
| **Civic — any hospital** | `hc_count_regional` + `hc_count_district` ≥ 1 — at least one regional or district hospital within the tertiary ring |
| **IoU — non-overlap** | IoU with any Tier 1 cluster ≤ 0.25 |

### 5.3 Tier 3 — Local

| Gate | Predicate |
|---|---|
| **Composition** | Cluster contains Hardware OR Warehouse |
| **Primary population** | `rank_pp_iso` ≤ 0.50 — cluster is in the top half by primary catchment population within its ISO country |
| **Civic — any hospital** | `hc_count` ≥ 1 — at least one hospital of any classification within the tertiary ring |

### 5.4 Tier 4 — Fringe

All clusters that do not pass Tier 1, 2, or 3 predicates.

---

## 6. Percentile Rank Calculation

Percentile ranks (`rank_pp_iso`, `rank_sp_iso`, etc.) are computed within each ISO
country by `synthesize-od-study.py`. The value is `rank / n` where rank 1 = highest
value in the country and n = total clusters in the country. A value of 0.10 means the
cluster is in the top 10% of its country by that axis.

Eight axes are ranked:
- `rank_pp_iso` — primary catchment population (≤ 35 km crow-flies)
- `rank_sp_iso` — secondary catchment population (35–150 km)
- `rank_pg_iso` — primary grocery spend
- `rank_sg_iso` — secondary grocery spend
- `rank_ph_iso` — primary hardware spend
- `rank_sh_iso` — secondary hardware spend
- `rank_pw_iso` — primary wholesale spend
- `rank_sw_iso` — secondary wholesale spend

These thresholds are intentionally coarse (G6 operator decision). The intent is
to distinguish nationally significant clusters from local nodes, not to produce
a precise score. Threshold refinement is deferred to a future sprint.

---

## 7. IoU Calculation

Overlap between clusters is measured as the Intersection over Union (IoU) of two
equal-radius disks at the `SECONDARY_RADIUS_KM` radius (3.0 km). The closed-form
lens-area formula is used:

```
lens_area = 2r² · arccos(d/2r) − (d/2) · √(4r² − d²)
IoU = lens_area / (2 · π · r² − lens_area)
```

where d is the haversine distance between cluster centroids and r = 3.0 km.

---

## 8. Tiebreaker

Within each tier and ISO, clusters are ordered by:
1. Store count within 3 km (descending)
2. Primary catchment population `pp` (descending)
3. Cluster ID (ascending, deterministic)

---

## 9. Civic Classification Source

Hospital and university tier classifications are emitted by `ingest-osm-civic.py`
from OpenStreetMap data. Classifications:

| Type | Values |
|---|---|
| `hospital_tier` | `regional`, `district`, `clinic` |
| `university_tier` | `regional`, `small`, `excluded` |

Only `regional` and `district` hospitals count toward Tier 1 and Tier 2 civic gates.
Clinics do not pass these gates. All hospital types contribute to the Tier 3 gate.

---

## 10. Field Inventory

| Field | Emitted by | Description |
|---|---|---|
| `tier` | `build-geometric-ranking.py` | Integer 1–4 |
| `tier_predicates_fired` | `build-geometric-ranking.py` | JSON list of gate strings |
| `rank_pp_iso` … `rank_sw_iso` | `synthesize-od-study.py` | Per-ISO percentile ranks, 8 axes |
| `hc_count_regional` | `build-clusters.py` | Regional hospitals in tertiary ring |
| `hc_count_district` | `build-clusters.py` | District hospitals in tertiary ring |
| `he_count_regional` | `build-clusters.py` | Regional universities in tertiary ring |
| `he_count_small` | `build-clusters.py` | Small universities in tertiary ring |
| `national_rank` | `legacy/generate-rankings-v2.py` | Rank within ISO by V2 score_final (legacy, retained) |
| `iso_market_rank` | `legacy/generate-rankings-v2.py` | Sub-national market rank (US/CA/MX state; EU country) |
| `tier_descriptor` | `build-clusters.py` | Composition label (e.g., "Hypermarket + Hardware + Warehouse") |
| `hyper_list` | `build-clusters.py` | JSON list of ALPHA_HYPERMARKET chain IDs co-located within 3 km (excluding self) |
| `ls_list` | `build-clusters.py` | JSON list of ALPHA_LIFESTYLE chain IDs co-located within 3 km (excluding self) |
| `hw_list` | `build-clusters.py` | JSON list of ALPHA_HARDWARE chain IDs co-located within 3 km (excluding self) |
| `wh_list` | `build-clusters.py` | JSON list of ALPHA_WAREHOUSE chain IDs co-located within 3 km (excluding self) |

**Removed fields (G15 override):** `score_final`, `score_base`, `score_count_bonus`,
`score_diversity_bonus`, `score_multi_anchor`, `score_civic_depth`, `score_overlap_penalty`,
`rank_v2`. These fields are no longer emitted by the V3 pipeline.

---

## 11. Reproducibility

The full pipeline is reproducible from the raw ingested source data:

```bash
python3 synthesize-od-study.py    # per-ISO percentile ranks
python3 build-clusters.py                       # cluster geometry + civic counts
python3 legacy/generate-rankings-v2.py          # national_rank + iso_market_rank
python3 build-geometric-ranking.py              # tier assignment
python3 build-tiles.py            # PMTiles output
```

All source data is documented in `DATA-MANIFEST.md` in the project root.

---

## 12. Region Summary

Tier counts as of 2026-05-17 (Phase 9 complete: IKEA 14-chain ingest audit; sub-format
filter + multi_country fix; 10,213 clusters after deduplication).

| ISO | T1 Regional | T2 District | T3 Local | T4 Fringe |
|-----|-------------|-------------|----------|-----------|
| US  | 105         | —           | —        | —         |
| MX  | 15          | —           | —        | —         |
| ES  | 15          | —           | —        | —         |
| DE  | 9           | —           | —        | —         |
| CA  | 7           | —           | —        | —         |
| FR  | 3           | —           | —        | —         |
| GB  | 3           | —           | —        | —         |
| **Total** | **157** | **1,462** | **2,081** | **6,513** |

FR has 3 T1 clusters — enabled by Carrefour-FR, Auchan-FR, and E.Leclerc-FR
in `REGION_CONFIG["FR"]["anchor"]` (Phase 6, 2026-05-16).

DE has 9 T1 clusters — enabled by Kaufland-DE (797 stores, Q685967) added to
`ALPHA_HYPERMARKET["EU"]` and `REGION_CONFIG["DE"]["anchor"]` (Phase 7,
2026-05-17). Kaufland co-locates with IKEA-DE at 9 sites satisfying the
Lifestyle∧Hyper composition predicate. E center (38 records) and Marktkauf
(126 records) have insufficient OSM coverage for IKEA co-location.

Fred Meyer (126 stores, Q5495932) added to `ALPHA_HYPERMARKET["NA"]` and
`REGION_CONFIG["US"]["anchor"]` (Phase 8, 2026-05-17). PNW-regional Kroger
subsidiary; seeds 63 new US clusters but none qualify T1 (no IKEA or Costco
co-location within 3 km in Pacific Northwest).

IKEA 14-chain ingest audit (Phase 9, 2026-05-17): sub-format inflation corrected
across all 14 IKEA YAMLs via `format_reject_nodes: true` and `format_exclude_names`
list. ikea-mx and ikea-nl use `format_reject_nodes: false` (stores are OSM nodes).
ikea-nordics uses `multi_country: true` (spans SE/NO/DK/FI; non-ISO country_code
caused silent record drop). GB T1 dropped from 5 → 3; ES from 20 → 15; US
increased from 102 → 105 (cleaner IKEA location data improved composition matching).
