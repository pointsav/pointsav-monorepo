---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/woodfine-fleet-deployment
target_path: gateway-orchestration-gis/
target_filename: guide-gis-adding-a-country.md
audience: customer-operator
bcsc_class: current-fact
language_protocol: PROSE-GUIDE
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 10 Phase H operator decision: rename tienda-inglesa-mx
  to tienda-inglesa-uy, requiring Uruguay (UY) to be added as a new country.
  This guide documents the end-to-end pattern using that example.
research_inline: false
notes_for_editor: |
  This is a customer-side operational guide (not a TOPIC, not a DESIGN-RESEARCH).
  Commands must be copy-paste runnable. Cross-references to TOPICs that explain
  the *why* should be in See Also, not in line.
---

# Guide: Adding a Country to the GIS Pipeline

This guide documents the end-to-end procedure for extending GIS coverage to a country that is not currently in the operational footprint. The May 2026 example is Uruguay (UY), added when the chain `tienda-inglesa` was reassigned from Mexico to its actual country of operation.

The procedure has three stages: register the country bounding box, configure region-name resolution, and add the chain configurations. Each stage is mechanical; the per-country effort is roughly twenty minutes excluding ingest time.

## Stage 1 — Register the Country Bounding Box

The OpenStreetMap Overpass ingest queries chain stores within a country's bounding box. The bounding box must be registered in the ingest module before any chain in that country can be queried.

Edit `pointsav-monorepo/app-orchestration-gis/ingest-osm.py` and add an entry to the `COUNTRY_BBOX` dictionary:

```python
COUNTRY_BBOX = {
    "US":      (24.0,  -125.0,  50.0,  -65.0),
    # ...
    "UY":      (-35.0, -58.5,  -30.0,  -53.0),
}
```

The four-tuple is (lat_min, lon_min, lat_max, lon_max). Source the values from a country profile or a short Wikipedia query. Tighten the box to exclude neighbour-country territory: a too-loose box increases bbox contamination, which the country-polygon containment filter will catch but only after wasted Overpass round-trips.

## Stage 2 — Configure Region-Name Resolution

The `region_engine` resolves cluster coordinates to human-readable regional names. New countries fall through to the Natural Earth admin-1 fallback by default; this gives state or province names. If finer granularity is required (metro areas, municipalities), a country-specific boundary file must be added.

For most new countries, the default fallback is acceptable. Skip ahead to Stage 3.

For countries where finer granularity matters (typically: existing operational tier expansion or a market with strong municipal identity), follow the pattern in `region_engine.py`:

1. Download an admin-2 or admin-3 GeoJSON. GADM 4.1 (UC Davis) provides global open-data coverage at multiple admin levels: `https://geodata.ucdavis.edu/gadm/gadm4.1/json/gadm41_<ISO3>_<level>.json.zip`. ISO3 is the three-letter country code; level is 2 or 3.
2. Add the download to `download-boundaries.sh` following the existing CA Census Subdivision pattern.
3. Add a loader, format helper, and routing branch to `region_engine.py`. The loader pattern is one line; the format helper extracts `NAME_2` (admin-2) or `NAME_3` (admin-3) and applies the CamelCase splitter. The routing branch goes in `RegionEngine.resolve()` keyed on the ISO code.

GADM names often need post-processing. Re-use `_gadm_camelcase_split` from `region_engine.py`; consider whether language-specific preposition handling is needed (Spanish-speaking countries already covered by `_GADM_SPANISH_PREPS_RE`).

## Stage 3 — Add Chain Configurations

For each retail chain operating in the new country, create one YAML file in the deployment's `service-fs/service-business/` directory.

```yaml
schema: service-business-chain-v1
chain_id: tienda-inglesa-uy
country: Uruguay
country_code: UY
region: latam
category: Regional food distributor
brand_family: Food
retailer: Tienda Inglesa
canonical_name: "Tienda Inglesa S.A."
website: tiendainglesa.com.uy
wikidata_id: Q7794716
osm_overpass_tag: brand:wikidata=Q7794716
store_count_approx: 35
locations_file: tienda-inglesa-uy.jsonl
locations_status: pending
last_updated: 2026-05-08
name_query: "Tienda Inglesa"
```

The `chain_id` follows the convention `<retailer-slug>-<iso2-lowercase>`. The `wikidata_id` should be the brand entity, not the parent company. The `name_query` field is a fallback used when the Wikidata-based query returns zero results — common for chains whose OpenStreetMap brand:wikidata tag is sparse or absent.

Register each chain in `pointsav-monorepo/app-orchestration-gis/config.py`. Anchor-class chains (hypermarket, hardware, warehouse club) belong in `REGION_CONFIG[<ISO2>]`. Food-family chains belong in the `GENERIC_FOOD` set; they appear on the map but do not contribute to cluster scoring.

Register the chain's brand family in `pointsav-monorepo/app-orchestration-gis/build-tiles.py` `CHAIN_FAMILY` dictionary. Six families are available: Hypermarket, Hardware, Warehouse, Food, Furniture, Pharmacy.

## Stage 4 — Ingest and Rebuild

Run the ingest for the new chains:

```bash
cd pointsav-monorepo/app-orchestration-gis
python3 ingest-osm.py --chain <chain_id_1> <chain_id_2> ...
```

Each chain is queried via Overpass against the country's bounding box. Failures fall back to the name-based query if `name_query` is set; otherwise the chain is skipped with a console message.

After ingest, run the cluster-entities deduplication and the full pipeline:

```bash
python3 ../../service-business/cluster-entities.py
python3 build-clusters.py
python3 generate-rankings.py
python3 build-tiles.py --layer 1
python3 build-tiles.py --layer 2
```

Verify chain coverage with `check-chain-counts.py`:

```bash
python3 check-chain-counts.py
```

Look for the new chains in the output. The status should be OK (raw count within ±20% of `store_count_approx`) or UNDER (raw count below 80% — typical when OpenStreetMap coverage is sparse for the country). OVER status (raw count above 120%) indicates bbox contamination not caught by the country-polygon filter — investigate.

## Stage 5 — Verify Live

Open `gis.woodfinegroup.com`. Pan to the new country; zoom past the regime threshold. Retailer dots should appear in the chain's brand-family colour. Click a cluster: the inspector panel should show region names resolved by the engine — either fine-grained municipal names (if Stage 2 added boundary files) or fallback admin-1 state/province names.

## See Also

- [Adding a New Chain to the GIS Pipeline](guide-gis-adding-a-chain.md)
- [GIS Pipeline Rebuild](guide-gis-pipeline-rebuild.md)
- [Regional Name Resolution Architecture](topic-regional-name-resolution-architecture.md)
- [UK and European Food Retail Coverage](topic-uk-eu-food-retail-coverage.md)
