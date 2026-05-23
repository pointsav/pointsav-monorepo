# Tonight Build — Approval Checklist

> Ready for operator sign-off · 2026-05-23
> Start window: after 22:00 PDT (05:00 UTC 2026-05-24)

---

## What runs tonight

A **Phase 2 cluster + tile rebuild** only. No new chain ingests, no catchment rebuild.

| Step | Script | Output | Est. |
|------|--------|--------|------|
| 1 | `build-clusters.py` | `work/clusters.geojson` | ~4 min |
| 2 | `build-tiles.py --layer 2` | `layer2-clusters.pmtiles` + `clusters-meta.json` | ~2 min |

**Total: ~6 minutes.**

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

### UI filter (already live in index.html)

Cluster mode now renders only `tier ∈ {1, 2, 3}` (`tier >= 1` filter).
Any node with `tier = 0` (data anomaly fallback) is suppressed. This change
is already deployed — no rebuild needed for this.

---

## Pre-flight (already verified)

- [x] Disk free: 39 GB (threshold: 5 GB)
- [x] `taxonomy.py`: 488 lines (threshold: 400)
- [x] `nightly-rebuild.sh --dry-run`: PASS
- [x] `tippecanoe v2.79.0`: present on PATH
- [x] `build-clusters.py` syntax: no import errors

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

1. **Tier counts** — logged by build-clusters.py. London fix should reduce GB T1 by ~1
   (the monster) and produce new smaller T2/T3 clusters in GB. Net GB total may increase.
2. **Largest span** — `co_gb_n5150596_w011882` (22 km) should not appear in new output.
   New GB T1 spans should all be < 8 km.
3. **cluster-meta.json size** — expect similar to Phase 18 (~11 MB). Large change = anomaly.
4. **Map check** — reload gis.woodfinegroup.com, zoom to London. Should see multiple
   distinct cluster dots instead of one giant pin.

---

## Not included in tonight — Phase 19 (when ready)

- Sport category (Decathlon / REI / Bass Pro): `BRIEF-ADD-SPORT-CATEGORY-2026-05-23.md`
- DE lifestyle chains (XXXLutz / Höffner / Segmüller): deferred Phase 20
- Meijer US / Bodega Aurrera MX: deferred Phase 19 or 20

---

## Approval

Operator approval to start: **[ ]**

Notes:
