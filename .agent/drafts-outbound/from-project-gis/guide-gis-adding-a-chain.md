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
  Covers two bugs discovered and fixed: YAML boolean parsing of country codes,
  and OSM name suffix pattern requiring prefix-regex query.
research_inline: false
notes_for_editor: |
  Technical guide. No Bloomberg-register polish needed — keep imperative tone.
  Paths anchor at GitHub repo root (pointsav-monorepo/...).
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
- `wikidata_id` and `osm_overpass_tag` can be `~` (null) if the chain has sparse brand tag coverage in OSM.
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

**ALPHA threshold:** promote a chain to `ALPHA_HARDWARE` or `ALPHA_WAREHOUSE` in `config.py` only when it has ≥ 20 records. Below that threshold, keep as GENERIC — the chain is present but too sparse to reliably signal co-location quality.

---

## Step 3 — Classify the chain in config.py

**File:** `pointsav-monorepo/app-orchestration-gis/config.py`

Add the chain to the appropriate set:

- `ALPHA_HARDWARE["EU"]` / `ALPHA_HARDWARE["NA"]` — primary large-format HW anchor for a market
- `GENERIC_HARDWARE["EU"]` — present but not a primary market anchor
- `ALPHA_WAREHOUSE["EU"]` etc. — cash-and-carry / warehouse club
- `ALPHA_ANCHORS["EU"]` — large-format hypermarket or destination anchor

Add the chain to `REGION_CONFIG` under the appropriate country key, in the correct role list (`anchor`, `hardware`, or `warehouse`).

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

## Step 7 — Commit

```bash
git add pointsav-monorepo/app-orchestration-gis/config.py \
        pointsav-monorepo/app-orchestration-gis/www/index.html
~/Foundry/bin/commit-as-next.sh "GIS: add <chain-id> — <N> records, <outcome>"
```

Stage the YAML separately if it is tracked in the cluster repo. The JSONL and cleansed-clusters.jsonl are in the Totebox deployment (not tracked in Git).
