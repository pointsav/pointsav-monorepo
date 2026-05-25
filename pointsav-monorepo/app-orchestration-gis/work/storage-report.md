# Storage Report — project-gis Phase 11

Generated: 2026-05-17

---

## Disk state

| Filesystem | Size | Used | Avail | Use% | Mounted |
|---|---|---|---|---|---|
| /dev/root (root) | 77G | 50G | 28G | 65% | / |
| /dev/sdb (secondary) | 40G | 30G | 7.9G | 79% | /srv/foundry/deployments/cluster-totebox-personnel-1/service-fs |

**Status:** Root at 65% — acceptable. Secondary at 79% — monitor; avoid large additions.

---

## Large files on root filesystem

### work/ directory (source repo, root disk)

| File | Size | Age | Removable? |
|---|---|---|---|
| `work/spend-catchment.geojson` | 581M | May 15 | No (tile source) |
| `work/census-catchment.geojson` | 464M | May 15 | No (tile source) |
| `work/layer1-locations.geojson` | 180M | May 17 | No (tile source) |
| `work/clusters.geojson.pre-sprint9` | 17M | May 8 | **YES — stale backup** |
| `work/radius.geojson` | 15M | May 7 | Possible (regenerable) |
| `work/clusters.geojson.new` | 11M | May 5 | **YES — stale backup** |
| `work/clusters.geojson.bak` | 6.7M | May 5 | **YES — stale backup** |

**Total stale backups removable: ~35M**

### gateway deployment tiles (root disk)

| Directory | Size |
|---|---|
| `/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles/` | 1.7G |

The gateway tiles are live-serving files. Not moveable.

---

## Recommendation

### Immediate (safe to execute now, operator approval not required for stale files)

Delete stale GeoJSON backups — these are from sprints 9–10 and are superseded by
current `clusters.geojson`:

```bash
rm /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work/clusters.geojson.pre-sprint9
rm /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work/clusters.geojson.new
rm /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work/clusters.geojson.bak
```

Estimated savings: ~35M on root filesystem.

### Deferred (operator approval required)

Moving `spend-catchment.geojson` (581M) and `census-catchment.geojson` (464M) to the
secondary disk and symlinking would save ~1G on root but push secondary from 79%→82%.
Not recommended until secondary disk is expanded or tiles are pruned.

Staged commands (do NOT execute without approval):
```bash
SVC=/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/gis-work
mkdir -p "$SVC"
mv work/spend-catchment.geojson "$SVC/"
ln -s "$SVC/spend-catchment.geojson" work/spend-catchment.geojson
mv work/census-catchment.geojson "$SVC/"
ln -s "$SVC/census-catchment.geojson" work/census-catchment.geojson
```

Net result: root saves ~1G (65%→63%); secondary grows to ~82%. Not urgent.

---

## Priority action

Delete the three stale backup files (~35M). Nothing else needs operator action today.
