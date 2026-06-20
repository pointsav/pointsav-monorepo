# Session Context — project-gis

Rolling 5-session summary. Newest entry on top. Oldest entry pushed to session-context-archive.md when this file exceeds 5 entries.

---

## 2026-06-20 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Phase H8 complete: HTTP GET to Doorman `/healthz` (200 OK) via raw TCP over VirtIO-net from
  seL4 user space. Key discovery: QEMU SLiRP sends an ARP broadcast before delivering TCP SYN-ACK;
  guest must reply to ARP (52:54:00:12:34:56 for 10.0.2.15) so SLiRP learns the guest MAC.
- Written: `moonshot-sel4-vmm/src/bin/virtio_net_http.rs` — dual VirtIO queue init, ARP reply,
  raw TCP/IP (IP+TCP checksums hand-rolled, `#![no_std]`), HTTP/1.1 GET to `10.0.2.2:9080/healthz`.
- Added `syscall::send()` wrapper (was missing; `pd::notify()` called it).
- Written: `moonshot-toolkit/examples/os-console-virtio-http.toml`.
- Gate output: `[h8] ARP request received` → `[h8] SYN-ACK! server_seq=0x00000001` → `HTTP GET gate: PASSED`
- Commit `2e0b47c5` (Peter). Phase H roadmap H1–H8 now complete.
- M-17 contamination observed: `NEXT.md`, `CLAUDE.md`, `brief-discipline.md` contain project-design
  and project-editorial content. Did NOT fix — noted for Command Session sweep.

**Pending / carry-forward:**
- Stage 6: commit `2e0b47c5` (+ all prior Phase H commits) need `bin/promote.sh` from Command Session.
- NEXT.md M-17 contamination: three archives concatenated into one file; Command Session repair needed.
- CLAUDE.md M-17 contamination: file now contains project-design guide content instead of project-console.

**Operator preferences surfaced:** None new this session.

---

## 2026-06-19 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- **Shutdown only** — no new feature work.
- **M-17 repair (recurring)**: project-console session committed contaminated NEXT.md and
  session-context.md to project-gis repo overnight. Third time in 48 hours. NEXT.md and
  session-context.md both restored to correct GIS content and committed.
- **Root pattern**: project-intelligence + project-console sessions open project-gis files
  (NEXT.md, session-context.md) thinking they are in their own archive — likely because
  CLAUDE.md at project-gis/ is itself contaminated with project-intelligence content.
  Fix requires Command Session: correct the CLAUDE.md header at project-gis/.

**Carry-forward:**
- [ ] Night 6 verification — wildfire layer15 after GFWED fix
- [ ] EU seismic fallback — git clone or GSHAP raster
- [ ] FEMA US SFHA (layer12) — refresh failed Night 5
- [ ] F-series tracking — F1–F7 at project-editorial
- [ ] Command: apply nginx gzip + cache-control on foundry-prod (outbox msg sent)
- [ ] Stage 6 — now 6 commits ahead after this repair commit; outbox needs update
- [ ] push-to-prod.sh gis — after Stage 6

**Operator preferences surfaced:** None new.

---

## 2026-06-19 — Totebox@claude-sonnet-4-6 (performance audit + delivery)

**Done this session:**
- **Performance audit of gis.woodfinegroup.com** — curl timing + HTML inspection + nginx analysis:
  - Root cause 1: `gzip_types` commented out → JS/CSS/JSON served uncompressed (784 KB maplibre + 19 MB clusters-meta)
  - Root cause 2: `Cache-Control: no-cache` on all assets incl. static libs never changes
  - Root cause 3: External map style URL (`tiles.openfreemap.org`) with no preconnect hint
  - Root cause 4: No preload hints for 784 KB maplibre-gl.js
  - Domain resolves to foundry-prod (34.168.19.68) — separate VM from foundry-workspace
- **Implemented — index.html preload hints**: `preconnect` for openfreemap.org + `preload`
  for maplibre-gl.js/pmtiles.js/CSS in both deployment www/ and archive source
- **Nginx changes applied on foundry-workspace** (reference only, not prod):
  gzip_types + lib/ + data/ cache-control location blocks
- **Documented for Command**: exact nginx diffs in outbox `project-gis-20260619-perf-nginx-prod`
- **Commit**: `de977b4b` (pwoodfine) — preload hints + NEXT.md repair
- **Stage 6**: 5 commits ahead of origin; outbox updated

**Operator preferences surfaced:**
- "yes" to performance fix proposals = implement all listed changes immediately
- Audit findings delivered as structured table with estimated impact before implementing

---

## 2026-06-19 — Totebox@claude-sonnet-4-6 (Night 5 verification + AEC cleanup)

**Done this session:**
- Post-overnight build verification: PKS T1=692 ✓, T2=2,670, T3=3,709; park_ride=22,514 ✓;
  layer10 (2.1 MB) ✓, layer11 (120 MB) ✓, layer12-EU (151 KB) ✓; flood_hazard=855 ✓
- **GFWED root cause found + fixed**: NetCDF variable is `GPM.LATE.v5_FWI` not bare `FWI`;
  downloads all succeeded (128 MB each) but gdalinfo check failed on wrong var name
- EU seismic diagnosed: `maps.efehr.org` NXDOMAIN (subdomain removed upstream)
- Log file cleanup: `*.log` added to .gitignore (root + app-orchestration-gis)
- Briefs README contamination fixed; BRIEF-gis-nightly-rebuild-aec updated with Night 5 results
- Outbox cleaned (3 stale messages marked)
- **Commit**: `d7602bc7` (pwoodfine) — GFWED fix + gitignore + briefs README + Night 5 verification

**Operator preferences surfaced:**
- Plan-mode for broad audit tasks; operator approves plan before execution begins
- "check on everything" = verify + diagnose root causes + fix where possible + clean up

---

## 2026-06-19 — Totebox@claude-sonnet-4-6 (AEC overnight build fixes + run)

**Done this session:**
- 6 script bug fixes across 3 AEC build scripts:
  1. `overnight-aec-builds.sh` — path fix (ingest-osm-parking.py at monorepo path)
  2. `build-aec-seismic.sh` — EU join OR→two-if bug (Step 8)
  3. `build-aec-flood.sh` — AQUEDUCT threshold 100MB→85MB
  4. `overnight-aec-builds.sh` — removed IS (Iceland not in known-countries)
  5. `build-aec-flood.sh` — numpy 2.x: gdal_calc.py→pure GDAL Python API
  6. `build-aec-flood.sh` — `--config OGR_GEOJSON_MAX_OBJ_SIZE 0` on Step 12 EU merge
- Overnight build ran: park_ride + PKS rebuild ✓; AEC seismic (EU 0) ✓; AEC flood Night 5 ✓
- GFWED wildfire all 12 months failed → root cause found next session (wrong variable name)
- Commits: d19afca7, fce227b1, b1f2514d, b881c640

**Operator preferences surfaced:**
- "startup"/"shutdown" commands followed exactly
- Build errors prioritised and fixed inline without waiting for next session

---

## 2026-06-15 — Totebox@claude-sonnet-4-6 (Q1–Q8 diagnostic + ingest)

**Done this session:**
- Q1–Q5 AEC diagnostics completed (BRIEF A26 §5 updated)
- Q6 interceramic-mx COMPLETE: YAML wikidata fixed; 51 records written
- Q7 J1 §7.2 OLS COMPLETE: N=3,178 (US+ES); R²=0.538; T1 β=-0.040 (p<0.001)
- Q8: Command outbox sent — Stage 6 + root disk audit
- Commit: 029a4b59 (jwoodfine)

**Operator preferences surfaced:**
- "proceed" = approve and execute next queued question without re-confirmation
