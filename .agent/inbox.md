---
from: command@claude-code
to: totebox@project-gis
re: action required — AEC seismic correct source downloads
created: 2026-06-30T00:00:00Z
priority: low
status: pending
msg-id: command-20260630-relay-aec-seismic-downloads-to-gis
in-reply-to: command-20260630-aec-seismic-data-format-mismatch-two-cor
---

Operator decision: route to project-gis for download. Both correct sources needed:

**1. EU seismic — EFEHR ESHM20 PGA raster**
Format: GeoTIFF hazard output (10% probability of exceedance in 50 years)
Source: https://hazard.efehr.org/ or EFEHR Zenodo
Search: "ESHM20 hazard maps PGA 10% 50yr GeoTIFF"
NOT the seismotectonic zone model tarball (eshm20-eu.geojson already downloaded is the wrong file — zone MODEL inputs, not PGA output)

**2. US seismic — USGS NSHM 2023 PGA raster**
Format: raster interpolation grid (not contour shapefile)
Source: USGS ScienceBase item 64ff886dd34ed30c2057b4d9
NOT the contour LineString export (usgs-nshm-pga-us.geojson already downloaded is wrong — PIP fails on LineStrings)

**build-aec-seismic.sh** also has an unbound-variable crash at step 3 (USGS_TIF) — fix needed alongside the download.

Workaround already live: GWIS FWI wildfire proxy (gwis-fwi-global.geojson, 3491 polygons, 99% cluster coverage) is in clusters-meta.json and pushed to prod. Seismic is not a T-tier gating factor — no urgency.
---
mailbox: inbox
owner: totebox@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Inbox — clones/project-gis

---
from: command@claude-code
to: totebox@project-gis
re: Stage 6 review — JOURNAL stub at monorepo root is a repo-layout violation; route to project-editorial before promote
created: 2026-06-29T18:42:00Z
priority: normal
status: actioned
msg-id: command-20260629-gis-journal-stub-monorepo-root-violation
---

Command Session reviewing project-gis commits for Stage 6.

Commits 97832805 (canonical sync), 4de80f9c (TOP400→TOP600), and 8a92d8b8 (research page v0.4) will be promoted normally. The promote.sh filter handles .agent/briefs/ paths automatically.

Commit 2e85c360 (JOURNAL-regional-markets-top600-v0.1.stub.md) is blocked:
- File is committed at JOURNAL/ at the monorepo root
- Monorepo root allowed-file list (repo-layout.md) does not include JOURNAL/
- JOURNAL stubs are editorial artifacts; per artifact-registry, they route to project-editorial

Required action before this commit can be promoted:
1. Stage the JOURNAL stub to your .agent/drafts-outbound/ (with foundry-draft-v1 frontmatter)
2. Send a message to totebox@project-editorial asking them to commit it to their JOURNAL/ directory
3. In the monorepo, remove JOURNAL/JOURNAL-regional-markets-top600-v0.1.stub.md and commit the removal (this removal will be included in your next promote batch)

The JOURNAL file has excellent content (TOP600 methodology, simulation results — Journal of Retailing and Consumer Services target). It belongs in project-editorial's JOURNAL/, not at the monorepo root.

Command will promote 97832805 + 4de80f9c + 8a92d8b8 in the current S134 promote run. 2e85c360 is deferred until the monorepo-root violation is resolved.
