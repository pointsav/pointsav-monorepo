---
plan: PHASE16-RESUME
created: 2026-05-19
session: project-gis Totebox
status: SUSPENDED — safe to resume next session
---

# Phase 16 — Resume State

Session suspended 2026-05-19. Pick up from here at next session start.

---

## What was completed this session (Phase 15 + UI fixes)

All Phase 15 pipeline steps completed:
- `build-tiles.py --layer 2` → layer2-clusters.pmtiles (76.7MB, 13,657 clusters)
- `build-geometric-ranking.py` → tier fields patched (T1=435, T2=1,602, T3=3,080, T4=8,540)
- `build-mobility-tiles.py` → layer6+layer7 rebuilt
- `synthesize-od-study.py` → catchment ranks + clusters-meta.json updated (13,657 entries)
- `build-catchment-polygons.py` → new catchment-polygons.geojson (27,314 features, 45MB)

UI fixes shipped in index.html:
- 4-tier color system (T1=navy/#0A3070 → T4=slate/#64748B; T1=best)
- BentoBox tier counts now dynamic from clusterMeta (was stale hardcoded)
- T4 Fringe badge added
- Retailers-disappearing-on-zoom bug fixed (circle-opacity reset in drillIntoCluster)
- TOTAL_CLUSTERS updated to 13,657
- Catchment polygons rebuild kicked off (was only 6,815 clusters — now covers all 13,657)

Data fixes:
- artifact-registry.md updated (Phase 15 section, clusters-meta entry updated)
- DATA-MANIFEST.md anchor taxonomy updated (Wegmans, WinCo, Sprouts, ASDA, Morrisons, H-E-B)
- outbox.md Phase 15 completion message prepended
- ikea-mx.yaml: added `name_query: IKEA` + updated notes re OSM coverage gap

---

## Immediate next step: layer3-catchment.pmtiles rebuild

**Problem:** tippecanoe ran overnight with `--no-feature-limit --maximum-zoom 12` and produced a 1.7GB PMTiles file. Tiles are too large (tile 2/1/1 = 500KB+ even at minimum detail). This file is live but too heavy.

**Fix needed:** Rebuild with corrected parameters:
```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis

tippecanoe \
  --output /srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles/layer3-catchment.pmtiles \
  --force \
  --layer catchment \
  --minimum-zoom 2 \
  --maximum-zoom 8 \
  --drop-densest-as-needed \
  --simplification 8 \
  work/catchment-polygons.geojson
```

Key changes vs current bad build:
- `--maximum-zoom 8` (was 12) — catchment rings don't need tile-level detail
- `--drop-densest-as-needed` (replaces `--no-feature-limit`) — allows thinning at low zoom
- `--simplification 8` (was 4) — more aggressive polygon simplification

The source GeoJSON is ready at `work/catchment-polygons.geojson` (45MB, 27,314 features).
Expected output: ~100-200MB (vs current 1.7GB).

After rebuild, confirm in UI: click a cluster → catchment ring appears.

---

## IKEA North America — investigation result

**ikea-us.jsonl**: 51 records — matches ~55 US stores, OK.
**ikea-ca.jsonl**: 13 records — matches ~14-15 Canadian stores, OK.
**ikea-mx.jsonl**: 2 records — CDMX (Coyoacán) + Monterrey. OSM gap: Guadalajara + Puebla stores not yet mapped in OSM.

**Action:** No re-ingest needed. OSM coverage is the bottleneck. Re-ingest when OSM adds Guadalajara/Puebla. `name_query: IKEA` fallback now documented in ikea-mx.yaml.

---

## Outstanding commit needed

`ikea-mx.yaml` updated this session (name_query added, notes updated) — this file lives in the deployment instance, NOT in the git repo. No commit needed for this file.

`outbox.md` has one unstaged change (Phase 15 message prepended). Commit when ready:
```bash
cd /srv/foundry/clones/project-gis
git add .agent/outbox.md
~/Foundry/bin/commit-as-next.sh "gis: Phase 15 outbox + ikea-mx name_query fallback"
```

---

## Outstanding operator decisions (unchanged from Phase 15)

| Item | Decision needed |
|------|----------------|
| Path C composition (HW∧HM as T1-qualifying) | ~+199 US T1; methodologically significant |
| UK/FR/DE OD manual downloads | ONS ODWP01EW, INSEE FD_MOBPRO21, BA Pendler |
| US LODES full ingest | 5 states done; `ingest-lodes.py` auto-download; ~2-4h |
| layer3-catchment.pmtiles rebuild | See above — immediate next step |

## Outstanding data retrieval queue

| Data source | Status | Notes |
|---|---|---|
| US LODES (remaining 46 states) | Queued | `ingest-lodes.py` auto-download; ~2-4h |
| UK ONS ODWP01EW | Manual download needed | nomisweb.co.uk; ~77MB; MSOA-level |
| France INSEE FD_MOBPRO21 | Manual download needed | insee.fr; commune-level |
| Germany BA Pendler | Manual download needed | XLSX; kreis-level |
| Kontur Population (global) | Queued | HDX; ~2.3GB; replaces WorldPop |
| EU chain coverage (IT/GR/AT/PT/NL) | Research needed | IKEA-only anchor regions; Conad, Albert Heijn, Continente candidates |

---

## Session state checklist

- [x] Phase 15 pipeline complete
- [x] UI tier system migrated (4-tier, T1=best)
- [x] BentoBox counts dynamic
- [x] T4 badge added
- [x] Retailer zoom bug fixed
- [x] catchment-polygons.geojson rebuilt (27,314 features)
- [ ] layer3-catchment.pmtiles rebuild with correct params — **DO THIS FIRST**
- [ ] Outbox commit
- [ ] US LODES full ingest (operator decision needed)
- [ ] UK/FR/DE OD downloads (manual; operator action)
