---
schema: foundry-draft-v1
state: draft
language_protocol: GUIDE
originating_cluster: project-gis
target_repo: woodfine/woodfine-fleet-deployment
target_path: gateway-orchestration-gis-1/
target_filename: guide-gis-adding-a-chain.md
audience: internal-engineering
bcsc_class: internal
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from obs-bygg-no ingest debugging session 2026-05-06.
  Updated 2026-05-06: added brand_wikidata flat field (B2), Overture taxonomy
  migration notice (B1), and ALPHA/GENERIC criteria expansion.
research_inline: false
notes_for_editor: |
  Technical guide. No Bloomberg-register polish needed — keep imperative tone.
  Paths anchor at GitHub repo root (pointsav-monorepo/...).
  Updated this session with brand_wikidata, ALPHA criteria, and Overture note.
---

# Guide: Adding a New Chain to the GIS Pipeline

This guide covers the end-to-end process for adding a new retail chain to the co-location intelligence platform. All commands run from the `pointsav-monorepo/app-orchestration-gis/` directory on the workspace VM.

---

## Step 1 — Create or verify the chain YAML

Each chain requires a YAML file at:

```
/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business/<chain-id>.yaml
```

**Critical YAML rules:**

- `country_code` must be a quoted string: `country_code: "NO"` — not `country_code: NO`. PyYAML parses unquoted two-letter codes that match boolean keywords (`NO`, `YES`, `ON`, `OFF`) as booleans. This causes `iso_country_code: false` in every ingested record, silently breaking cluster formation.
- `wikidata_id` is the Wikidata QID for the chain (e.g. `Q13556979` for IKEA). It drives the `brand:wikidata` Overpass query — the most reliable OSM filter for international chains. Set to `~` only when OSM tag coverage is genuinely sparse; a missing QID forces a slower, less precise name-match fallback. The `wikidata_id` value is also written as a flat `brand_wikidata` field in every ingested record, enabling reliable chain deduplication and future parent-child detection.
- `osm_overpass_tag` can be `~` (null) if the chain has sparse brand tag coverage in OSM.
- If the chain uses location-suffixed names in OSM (e.g. "Brand CityName" rather than "Brand"), set both `name_query` and `name_query_partial: true`. The partial flag triggers a prefix-regex Overpass query (`^Brand`) that captures all suffix variants.

**Example — chain with sparse wikidata tags:**

```yaml
chain_id: obs-bygg-no
country_code: "NO"          # quoted — prevents PyYAML boolean parse
wikidata_id: ~
osm_overpass_tag: ~
name_query: "Obs Bygg"
name_query_partial: true    # OSM names are "Obs Bygg Slitu", "Obs Bygg Tiller", etc.
locations_status: active
```

---

## Step 2 — Ingest from OSM

```bash
python3 ingest-osm.py --chain <chain-id>
```

The script tries `brand:wikidata=<id>` first; if that returns 0 elements, falls back to `name_query`. With `name_query_partial: true`, the query uses a regex prefix match.

Check record count:

```bash
wc -l /srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business/<chain-id>.jsonl
```

If 0 records: check the YAML for the boolean country_code bug, and try broadening the name query. If still 0, the chain may not be tagged in OSM for that country.

**ALPHA vs GENERIC criteria:**

| Classification | Condition | Effect |
|---|---|---|
| `ALPHA_HARDWARE` / `ALPHA_WAREHOUSE` | ≥ 20 ingested records; chain is a primary large-format anchor for its market | Counts toward T3/T2/T1 scoring; cluster rank depends on co-presence with ALPHA chains |
| `GENERIC_HARDWARE` / `GENERIC_WAREHOUSE` | Present in OSM but fewer than 20 records, or a secondary brand format not representative of a market anchor | Visible on the All Locations layer; does not affect cluster quality score |
| Not listed in config.py | Chain ingested but not yet classified | Included in layer 1 tiles only if chain YAML exists; invisible to cluster algorithm |

Promote to ALPHA only after confirming store count represents genuine national-scale presence. Regional chains with 6–19 locations stay GENERIC until coverage improves.

---

## Step 3 — Classify the chain in config.py

**File:** `pointsav-monorepo/app-orchestration-gis/config.py`

Add the chain to the appropriate set:

- `ALPHA_HARDWARE["EU"]` / `ALPHA_HARDWARE["NA"]` — primary large-format HW anchor for a market
- `GENERIC_HARDWARE["EU"]` — present but not a primary market anchor
- `ALPHA_WAREHOUSE["EU"]` etc. — cash-and-carry / warehouse club
- `ALPHA_ANCHORS["EU"]` — large-format hypermarket or destination anchor

Add the chain to `REGION_CONFIG` under the appropriate country key, in the correct role list (`anchor`, `hardware`, or `warehouse`).

## Step 3b — Classify the chain in CHAIN_FAMILY (build-tiles.py)

**File:** `pointsav-monorepo/app-orchestration-gis/build-tiles.py`

Every chain visible on the All Locations layer must have a `brand_family` entry in the `CHAIN_FAMILY` dict near the top of the file. Without this entry the chain displays on the map without a category badge and its category label reads as "Hardware" (the fallback default).

Add the chain_id under the correct family:

| Family | Use for |
|---|---|
| `"Hypermarket"` | Large-format general merchandise / hypermarkets (80k+ sq ft) |
| `"Hardware"` | Home improvement and building materials |
| `"Warehouse"` | Membership / cash-and-carry club format |
| `"Food"` | Conventional grocery and specialty food retail |
| `"Furniture"` | Home furnishings and décor specialists |
| `"Pharmacy"` | Pharmacy-led health retail with broad product range |
| `"Department"` | Traditional department store (non-hypermarket format) |

Example:

```python
"save-on-foods-ca": "Food",
```

Chains classified as `ALPHA_HARDWARE`, `ALPHA_WAREHOUSE`, or `ALPHA_ANCHORS` in `config.py` should receive the matching family here. Food-family chains are data-layer only — they appear on the map but do not affect cluster scores.

---

## Step 4 — Rebuild cleansed-clusters.jsonl

The build pipeline reads from a merged file, not directly from individual JSONLs. After any new ingest, run:

```bash
python3 /srv/foundry/clones/project-gis/pointsav-monorepo/service-business/cluster-entities.py
```

This reads all `*.jsonl` files in `service-fs/service-business/`, deduplicates records within 100 m of the same chain, and writes `service-business/cleansed-clusters.jsonl`. Skipping this step causes the new chain's records to be invisible to build-clusters.py.

---

## Step 5 — Rebuild the pipeline

Run in order from `pointsav-monorepo/app-orchestration-gis/`:

```bash
python3 build-clusters.py
python3 generate-rankings.py
python3 build-tiles.py
```

`build-radius.py` is optional unless cluster centroids shifted significantly (adds ~5 min; rebuilds 75km catchment polygons).

---

## Step 6 — Update REGION_SUMMARY in index.html

After rebuild, compute new totals:

```bash
python3 -c "
import json
from collections import Counter
with open('work/clusters.geojson') as f:
    d = json.load(f)
na_isos = {'US','CA','MX'}
c1, c3 = Counter(), Counter()
for feat in d['features']:
    p = feat['properties']
    iso = p.get('iso','?')
    if p.get('rank_1km',0) > 0: c1['na' if iso in na_isos else 'eu'] += 1
    if p.get('rank_3km',0) > 0: c3['na' if iso in na_isos else 'eu'] += 1
print(f'NA 1km={c1[\"na\"]} 3km={c3[\"na\"]}')
print(f'EU 1km={c1[\"eu\"]} 3km={c3[\"eu\"]}')
"
```

Update the `REGION_SUMMARY` const in `www/index.html` to match. Deploy:

```bash
cp www/index.html /srv/foundry/deployments/gateway-orchestration-gis-1/www/index.html
```

---

---

## Appendix: Overture taxonomy field migration

The Overture Maps Foundation changed the category field schema in the 2025-11 release and removed the old `categories` field entirely in the 2026-06 release. If the workspace is running an Overture ingest against a release dated 2025-11 or later, the DuckDB query in `ingest-overture.py` must use `taxonomy.primary` instead of `categories.primary`. The script in this repo was updated to `taxonomy.primary` on 2026-05-06; no further action needed unless the script is cloned from an older branch.

This change affects only the civic-places ingest (hospital, university, airport). Hardware and warehouse chain ingest uses the OSM Overpass API and is unaffected.

---

## Step 7 — Commit

```bash
git add pointsav-monorepo/app-orchestration-gis/config.py \
        pointsav-monorepo/app-orchestration-gis/www/index.html
~/Foundry/bin/commit-as-next.sh "GIS: add <chain-id> — <N> records, <outcome>"
```

Stage the YAML separately if it is tracked in the cluster repo. The JSONL and cleansed-clusters.jsonl are in the Totebox deployment (not tracked in Git).

---

## Appendix: Sprint 9–11 additions (May 2026)

Three changes to the chain-onboarding pattern landed in the May 2026 sprint quarter and are now standard.

**Food family is data-only.** A chain registered with `brand_family: Food` appears on the live map as a coloured retailer dot but does not contribute to cluster formation or scoring. The food family is the right home for grocery chains (Tesco, Sainsbury's, Lidl, Aldi, Soriana, Chedraui) where the customer wants on-map context but not anchor-tier scoring. Register such chains in `config.py` `GENERIC_FOOD` set; do not add them to `REGION_CONFIG[<ISO2>]` anchor / hardware / warehouse tier lists. The brand-family taxonomy convention is documented in `topic-retail-brand-family-taxonomy.md`.

**`name_query` fallback is now the standard for chains with split Wikidata identifiers.** Aldi operates as Aldi Süd (Q41171) and Aldi Nord (Q125054) on a country-by-country split. OpenStreetMap brand:wikidata tags on individual store records are inconsistent. To capture all stores of such chains, register one Wikidata identifier in the YAML AND a `name_query` field with the brand's display name. The ingest tries the Wikidata query first; if it returns zero records the name_query fires. For chains where the Wikidata query returns a small but non-zero number of records (e.g. Aldi Netherlands), the name_query does NOT fire — to force the fallback, set `wikidata_id: ~` (null) so the wikidata pass returns zero and the name_query takes over.

**The country-polygon containment filter (Sprint 11) catches bbox contamination.** When a chain's bounding box overlaps a neighbour country (most commonly the United States and Canada border), Overpass returns cross-border records that would otherwise be mis-attributed. The filter, added to `ingest-osm.py` in May 2026, drops records that fall outside the chain's declared country polygon. Telemetry: `polygon-filter: dropped N cross-border records`. No action is required when adding a new chain — the filter operates on every chain automatically. The country polygon is loaded from `fallback_ne_admin1.geojson` (Natural Earth admin-1, unioned by ISO).

**See also:** `guide-gis-adding-a-country.md` for the procedure to extend the operational footprint to a new country (Sprint 10 added Uruguay).

---

## Appendix: Sprint 13 — name filter and Canadian spelling (May 2026)

**`SKIP_NAME_SUBSTRINGS` audit.** `ingest-osm.py` maintains a list of lowercase name substrings that identify sub-facility OSM elements — pharmacy windows, gas stations, vision centres, and similar amenities that share a brand:wikidata tag with the parent store. Any record whose OSM `name` contains one of these substrings is silently dropped at ingest time (before the JSONL is written). The current list is:

```python
SKIP_NAME_SUBSTRINGS = [
    "gasoline", " gas", "fuel", "petrol",
    "pharmacy", "vision center", "garden center", "photo center",
    "tire & lube", "tire and lube",
    "food court", "optical", "hearing aid", "moneycenter"
]
```

If a chain's ingest returns unexpectedly few records, check whether any of its OSM element names contain a substring from this list. A common failure mode: chains using Canadian or British English spellings (e.g. "Centre", "Supercentre") that partially match filter substrings.

**The "supercentre" gotcha.** Prior to May 2026 the list included `"supercentre"`. This silently dropped all Canadian Walmart stores tagged as "Walmart Supercentre" in OSM (Canadian English spelling). The US-English spelling "Supercenter" was not matched (different string). The filter was removed in Sprint 13. If you are adding a chain in a Canadian or UK market and record counts seem low, verify that no filter substring matches the chain's name format by running:

```bash
python3 -c "
import json
SKIP = ['gasoline', ' gas', 'fuel', 'petrol', 'pharmacy', 'vision center',
        'garden center', 'photo center', 'tire & lube', 'tire and lube',
        'food court', 'optical', 'hearing aid', 'moneycenter']
path = '/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business/<chain-id>.jsonl'
hits = []
with open(path) as f:
    for line in f:
        r = json.loads(line)
        name = (r.get('location_name') or '').lower()
        matches = [s for s in SKIP if s in name]
        if matches:
            hits.append((name, matches))
print(f'{len(hits)} records matched filter substrings (these were NOT dropped — they are in the JSONL).')
print('If counts seem low, the dropped records are the ones NOT in the JSONL.')
"
```

Records that matched the filter were already dropped during ingest and are invisible in the JSONL. To check for dropped records, compare `wc -l <chain>.jsonl` against the `OSM elements returned: N` line in the ingest console output. A large gap between elements returned and final record count indicates filter activity.
