# Tonight Build — Approval Checklist

> Ready for operator sign-off · 2026-05-23
> Start window: after 22:00 PDT (05:00 UTC 2026-05-24)

---

## What runs tonight

Two scripts run sequentially (PID 2507282 confirmed running):

```
bash nightly-rebuild.sh >> nightly-rebuild.log 2>&1
bash phase19-rebuild.sh >> phase19-rebuild.log 2>&1
```

**Script 1 — nightly-rebuild.sh (~6 min)**

| Step | Action | Output |
|------|--------|--------|
| 1 | Pre-flight: disk check + taxonomy.py line count | guard |
| 2 | `build-clusters.py` | `work/clusters.geojson` |
| 3 | `build-tiles.py --layer 2` | `layer2-clusters.pmtiles` + `clusters-meta.json` |

Applies London fix + geometric split + costco-uk data.

**Script 2 — phase19-rebuild.sh (~45 min, rate-limited by OSM Overpass)**

| Step | Action |
|------|--------|
| 1 | Ingest Decathlon EU×12 countries (FR/DE/GB/ES/IT/NL/PL/PT/SE/DK/NO/FI) |
| 2 | Ingest Decathlon CA |
| 3 | Ingest REI-US, Bass Pro Shops-US, Cabela's-US |
| 4 | `build-clusters.py` (with sport chains in JSONL) |
| 5 | `build-tiles.py --layer 2` (final rebuild incorporating all sport data) |

**Total estimated: ~50 min.** Layer 2 rebuilt twice — first pass applies geometric
fixes; second pass is the authoritative Phase 19 output with sport chains.

Four changes are applied across both builds.

---

## What the build applies

### London splitter fix (commit 29f4f23a)

Cluster `co_gb_n5150596_w011882` — 487 members spanning 22 km across central
London — will be partitioned into geographically compact retail-park clusters.

**Before:** 1 monster cluster, 22 km span, effectively useless for site-selection.
**After:** Estimated 20–40 distinct clusters at correct retail-park scale.

`split_greedy_tight()` now dissolves any tight atom whose diameter exceeds
`TAU_LOOSE_KM` (3 km) into individual stores before the greedy pass runs.
Normal small-city atoms (< 3 km) are unaffected.

### costco-uk re-ingest (2026-05-23)

Re-ingested with `name_query: "Costco"` fallback and `format_exclude_names` for
tyre centres and petrol stations. 32 mixed records → 30 clean warehouse records.
With 30 properly tagged stores across GB, several UK retail parks that previously
had no price_club presence may now qualify for T1 or T2 uplift.

### Geometric T2→T3 split (commit added 2026-05-23)

T2 clusters with `span < 1.25km AND member_count ≤ 2` are reclassified T3.
These are bare two-anchor pairs (one hypermarket + one hardware store, close
together) that don't merit "retail park" status.

**Before:** T1=1,157 / T2=4,283 / T3=262 (~20% / 75% / 5%)
**After (predicted):** T1=1,157 / T2=2,889 / T3=1,656 (~20% / 51% / 29%)

T1 is completely unchanged. The split only moves the weakest T2 clusters down.

### UI filter (already live in index.html)

Cluster mode now renders only `tier ∈ {1, 2, 3}` (`tier >= 1` filter).
Any node with `tier = 0` (data anomaly fallback) is suppressed. This change
is already deployed — no rebuild needed for this.

---

## Pre-flight (already verified)

- [x] Disk free: 39 GB (threshold: 5 GB)
- [x] `taxonomy.py`: 498 lines (threshold: 400)
- [x] `nightly-rebuild.sh --dry-run`: PASS
- [x] `tippecanoe v2.79.0`: present on PATH
- [x] `build-clusters.py` syntax: no import errors
- [x] `costco-uk` re-ingested: 30 clean records (was 32 mixed; 2 tyre centres removed)

---

## How to start

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
bash nightly-rebuild.sh
```

Log streams to terminal and appends to `nightly-rebuild.log`.

Or schedule for 05:00 UTC:
```bash
echo "cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis && bash nightly-rebuild.sh" | at 05:00
```

---

## After the build — what to check

1. **Tier counts** — logged by build-clusters.py. Expected: T1 ~1,157 (unchanged),
   T2 ~2,889 (down from 4,283), T3 ~1,656 (up from 262). Large deviation = anomaly.
2. **London fix** — `co_gb_n5150596_w011882` (22 km) should not appear in new output.
   New GB T1 spans should all be < 8 km. GB cluster count will increase (20–40 clusters
   replacing 1).
3. **cluster-meta.json size** — expect similar to Phase 18 (~11 MB). Large change = anomaly.
4. **Map check** — reload gis.woodfinegroup.com, zoom to London. Should see multiple
   distinct cluster dots instead of one giant pin. Zoom out to any major market — more
   T3 dots visible across the map (the reclassified two-store pairs).

---

## Not included in tonight — Phase 19 (when ready)

- Sport category (Decathlon / REI / Bass Pro): `BRIEF-ADD-SPORT-CATEGORY-2026-05-23.md`
- DE lifestyle chains (XXXLutz / Höffner / Segmüller): deferred Phase 20
- Meijer US / Bodega Aurrera MX: deferred Phase 19 or 20

---

## Approval

Operator approval to start: **[x] Approved 2026-05-23 by operator**

Notes:
