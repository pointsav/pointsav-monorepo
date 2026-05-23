# BRIEF — Add Sport as 7th Category (Phase 19)

> Implementation brief · 2026-05-23 · do this after London bug fix is stable.
> Research rationale: `BRIEF-SPORT-REBALANCE-LEAPFROG-2026-05-23.md` §2 Decision 1.

---

## Decision (operator-confirmed)

Add `sport` as 5th retail anchor category. Revised T1 rule:

```
T1 = has_hyper ∧ has_hw ∧ (has_pc ∨ has_life ∨ has_sport)
T2 = has_hyper ∧ n_retail ≥ 2   (unchanged)
T3 = n_retail ≥ 2 ∧ ¬has_hyper  (unchanged)
```

Category is same across NA and EU; only BRAND_FILL differs per country.

---

## Step 1 — taxonomy.py changes

### CATEGORIES dict — add between `lifestyle` and `medical`:
```python
"sport": {
    "label": "Sport / Outdoor Anchor",
    "naics": "451110",
    "description": "Destination sporting-goods anchor (Decathlon-class, ≥3,000 sqm).",
},
```

### _RETAIL_CATS — add sport:
```python
_RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "sport"}
```

### tier_of() — add has_sport:
```python
has_sport = "sport" in retail
if has_hyper and has_hw and (has_pc or has_life or has_sport):
    return 1
```

### BRAND_FILL["sport"] — new block:
```python
"sport": {
    "FR": ["decathlon-fr"],
    "DE": ["decathlon-de"],
    "GB": ["decathlon-gb"],
    "ES": ["decathlon-es"],
    "IT": ["decathlon-it"],
    "NL": ["decathlon-nl"],
    "BE": ["decathlon-be"],
    "PL": ["decathlon-pl"],
    "PT": ["decathlon-pt"],
    "SE": ["decathlon-se"],
    "DK": ["decathlon-dk"],
    "NO": ["decathlon-no"],
    "FI": ["decathlon-fi"],
    "US": ["rei-us", "bass-pro-shops-us", "cabelas-us"],
    "CA": ["decathlon-ca"],   # ~15 CA stores; verify OSM coverage first
},
```

### DISPLAY_NAMES entries:
```python
"decathlon-fr": "Décathlon", "decathlon-de": "Decathlon",
"decathlon-gb": "Decathlon", "decathlon-es": "Decathlon",
"decathlon-it": "Decathlon", "decathlon-nl": "Decathlon",
"decathlon-be": "Décathlon", "decathlon-pl": "Decathlon",
"decathlon-pt": "Decathlon", "decathlon-se": "Decathlon",
"decathlon-dk": "Decathlon", "decathlon-no": "Decathlon",
"decathlon-fi": "Decathlon", "decathlon-ca": "Decathlon",
"rei-us": "REI",
"bass-pro-shops-us": "Bass Pro Shops",
"cabelas-us": "Cabela's",
```

---

## Step 2 — YAML files (per chain)

**Decathlon** — Wikidata Q509349; OSM `brand:wikidata=Q509349`.
Create one YAML per country with appropriate bbox or polygon filter.
`name_query: "Decathlon"` fallback (consistent across all countries).

**REI** — Wikidata Q860698. US only. ~180 stores. No name_query needed —
OSM coverage is good for REI.

**Bass Pro Shops** — Wikidata Q4866375. US only. ~100 stores.

**Cabela's** — Wikidata Q606290. US only. ~100 stores.
Note: many Cabela's are now adjacent to Bass Pro Shops post-merger —
these will naturally co-locate in the same cluster, which is correct.

---

## Step 3 — Ingest

```bash
cd pointsav-monorepo/app-orchestration-gis
python3 ingest-osm.py --chain decathlon-fr
python3 ingest-osm.py --chain decathlon-de
# ... repeat per country
python3 ingest-osm.py --chain rei-us
python3 ingest-osm.py --chain bass-pro-shops-us
python3 ingest-osm.py --chain cabelas-us
```

Verify record counts against known store counts before rebuild.
Decathlon EU: expect 100–250 records per large market (FR ~300, DE ~100, GB ~50).
REI: expect ~180. Bass Pro: ~100. Cabela's: ~100.

---

## Step 4 — Rebuild and deploy

```bash
python3 build-clusters.py
python3 build-tiles.py
```

Expected outcomes:
- New T1 clusters where Decathlon + hypermarket + hardware co-locate in EU retail parks
- New T3 clusters: `{hypermarket, sport}` and `{hardware, sport}` combos
- T2 share should drop from 75% toward 65–68%
- T3 share should rise from 5% toward 12–15%

Update artifact-registry.md and commit via commit-as-next.sh.

---

## Notes

- Verify Decathlon CA OSM coverage before ingesting — OSM is sparse in some CA markets.
- Bass Pro + Cabela's post-merger stores may have inconsistent OSM brand tags;
  check both wikidata IDs when ingesting.
- DE Decathlon stores are often in the same retail park as Bauhaus + Kaufland —
  high T1 yield expected for DE.
