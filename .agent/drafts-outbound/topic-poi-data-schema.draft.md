---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-TOPIC
originating_cluster: project-gis
target_repo: pointsav/content-wiki-documentation
target_path: architecture/
target_filename: topic-poi-data-schema.md
bilingual_pair: topic-poi-data-schema.es.md
audience: technical
bcsc_class: public
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Research agents dispatched 2026-05-06 covering:
  1. SafeGraph parent-child POI model (parent_placekey, enclosed, polygon_class)
  2. Overture Maps schema (taxonomy field migration Dec 2025 / June 2026 removal)
  3. Placekey standard (What@Where identifier; shared Where suffix for same address)
  4. Rust DuckDB integration (duckdb-rs crate pattern)
  5. Retail chain coverage gaps (cross-referenced against config.py actuals)
  Doctrine claim #39 (research-trail discipline) satisfied.
research_inline: false
cites: []
notes_for_editor: |
  Technical TOPIC. Bloomberg register. No marketing language.
  Requires bilingual pair (es.md) — do not publish without Spanish version.
  BCSC class public — no forward-looking claims without planned/intended language.
---

# POI Data Schema

The co-location intelligence platform ingests point-of-interest (POI) data from two sources — the OpenStreetMap Overpass API for retail chain locations, and the Overture Maps Foundation dataset for institutional anchors — and normalises both into a unified schema before cluster analysis.

---

## Record types

The platform operates two record classes within its location data layer.

**Service-business records** represent individual retail chain locations: hardware stores, warehouse clubs, hypermarkets, and food anchors. Each record is identified by a `chain_id` key that links it to a chain configuration file, and by a `brand_wikidata` field holding the Wikidata QID for the retail brand. The Wikidata QID is the most reliable cross-source chain identifier because it is brand-level rather than name-level — two stores spelled differently but sharing the same QID belong to the same chain.

**Service-places records** represent institutional anchors: hospitals, universities, and airports. These are ingested from Overture Maps using the `taxonomy.primary` category field, which replaced the deprecated `categories.primary` field in the Overture 2025-11 release. Service-places records use a `category_id` key (`hospital`, `university`, `airport`) in place of `chain_id`.

---

## Core fields

Both record classes share the following fields:

| Field | Type | Notes |
|---|---|---|
| `location_name` | string | Display name; COALESCE of brand name and category fallback |
| `brand_wikidata` | string or null | Wikidata QID (e.g. `Q13556979`); null for civic places with no brand identity |
| `street_address` | string or null | Freeform address from OSM `addr:housenumber` + `addr:street` or Overture addresses |
| `city` | string or null | Locality from `addr:city`, `addr:town`, or `addr:municipality` |
| `region` | string or null | Province, state, or NUTS-3 region |
| `iso_country_code` | string | ISO 3166-1 alpha-2 country code |
| `latitude` | float | WGS 84, 7 decimal places |
| `longitude` | float | WGS 84, 7 decimal places |
| `naics_code` | string | NAICS industry classification |
| `top_category` | string | NAICS top-level category description |
| `sub_category` | string | NAICS sub-category description |
| `source` | string | `osm` or `overture` |
| `confidence` | float | Confidence score (OSM: fixed 0.85; Overture: from dataset) |

---

## Chain identification and the Wikidata QID

The `brand_wikidata` field holds the Wikidata QID for the retail brand. Wikidata QIDs are persistent, language-independent, and maintained by a global community — making them the preferred chain identifier across commercial POI datasets.

The OpenStreetMap community tags retail locations with `brand:wikidata=<QID>`, and the ingest pipeline uses this tag as its primary query filter. A location tagged with the correct QID will be captured regardless of local name spelling variations.

The Overture Maps Foundation exposes brand identity via the `brand.wikidata` field in its Places schema. The platform extracts this field at ingest for service-places records.

---

## Overture taxonomy schema

Overture Maps deprecated the `categories` struct in November 2025 and removed it in the June 2026 release. The replacement is the `taxonomy` struct, which exposes:

- `taxonomy.primary` — the primary category identifier (equivalent to the old `categories.primary`)
- `taxonomy.alternate` — an array of secondary category associations with optional attribute structs

Category identifiers are unchanged across this migration. Queries that previously read `categories.primary = 'hospital'` become `taxonomy.primary = 'hospital'` without any change to the filter values.

---

## Spatial deduplication

OSM data for large-format retailers sometimes includes both node and way elements for the same physical location (the building footprint as a way, and the entrance as a node). The pipeline deduplicates records within a 100-metre spatial cluster per chain, retaining the record with the most complete address fields.

A second deduplication pass runs at 25 metres across different `chain_id` values sharing the same `brand_wikidata` QID. This identifies sub-format or co-branded stores — for example, a fuel station operating under a different `chain_id` but sharing the parent retailer's QID — which are candidates for the parent-child sub-location model described below.

---

## Parent-child sub-location model

Large-format retailers frequently operate ancillary services at the same address: pharmacies, fuel stations, optical centres, and garden centres. In raw OSM data these appear as separate POI elements, each with a distinct name and sometimes a distinct `chain_id`.

The intended model (pending operator approval) treats the primary store as the parent location and collapses ancillary services into a `sub_entities` list within the parent record. On the map, one bubble represents the parent; the bento box detail panel lists sub-services. This matches the SafeGraph parent-child pattern, which uses a `parent_placekey` field plus `enclosed: true` and `polygon_class: 'BUILDING'` to identify physically contained sub-locations.

The Placekey standard — a globally unique identifier with a `What@Where` structure — expresses this relationship via a shared `Where` component: two POIs at the same address share their `Where` suffix (the geocell part), while their `What` prefix (the brand hash) differs. This shared-suffix pattern is planned as the primary mechanism for identifying co-located sub-businesses once Placekey integration is added to the pipeline.

---

## Address completeness

Address coverage in the current dataset varies by country. OSM coverage of `addr:housenumber` and `addr:street` is strong in Western Europe and Canada, moderate in the United States, and sparse in some Nordic and Southern European markets.

A planned enhancement will spatial-join POI records against the Overture Addresses theme (≤15 metre radius) to back-fill missing street-level addresses. The Overture Addresses theme provides structured address records for over two billion global addresses derived from authoritative national registries.

---

## Data update cadence

Service-business records are re-ingested per chain on demand, typically when a new chain is added or when quarterly OSM coverage audits flag anomalies. Service-places records (hospital, university, airport) are re-ingested against new Overture quarterly releases. The Overture S3 path in the ingest script must be updated to reference each new release.
