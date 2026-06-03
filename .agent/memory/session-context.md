---
schema: foundry-session-context-v1
archive: project-data
---

# Session context — rolling 3-session summary

---

## 2026-06-03 (later) | Totebox | claude-code (GIS — archetype model rework)

**Done this session (large; all DEPLOYED live + committed `aec2187e`):**
- **Simulation harness** `tools/sim_spread.py` — clusters once, evaluates qualify/tier rules
  instantly. Used to tune both archetypes via Opus sub-agent sims.
- **Commuter (PKS) redefined → geometric airport-led park-and-ride.** Candidate = sized regional
  airport (park-and-fly, ≤600 km from a metro) OR outer commuter-rail-belt station (15–110 km
  ring, connected toward core, ≤4 stops from line end). **Airports lead to spread the bubbles
  across the map** — rail-only covered 96 NA map cells; airport-led covers 957 (≈10×). 5,977 live.
- **Urban Fringe (VWH) → Retail-density.** `qualify_vwh()` admits ≥2-cat co-locations OR any lone
  STRONG/BROAD trade store; composition-score `tier_vwh(cats,n)`. 7,028 live. Both ≈ Retail (~6,500).
- **Mobile BentoBox footbar hardening** (visualViewport detent heights + resize re-snap,
  `overscroll-behavior: contain`, modal `dvh`) verified via `tools/shoot.mjs` browser-in-the-loop;
  **cache-busting `?v=` token** on archetype data URLs (fixed a stale-cache "not updating" report).
- **June 4 overnight ingest SCHEDULED** (crontab 05:00 UTC): `run-overnight-ingests.sh` — parking
  layer + parcel depots + 20 new VWH brand chains (builders'/self-storage/trade-counter). New
  scripts `ingest-osm-parking.py`, `ingest-osm-parcel-depot.py`. Brand YAMLs in local-only data dir.
- **All artifacts updated** (this entry): BRIEF, NEXT.md, artifact-registry, DATA-MANIFEST, memory.

**Pending / carry-forward:** June 4 overnight ingest fires (review `overnight-ingests.log`); then
wire parkade GREENFIELD filter into `build-pks-clusters.py` + new categories into `VWH_CHAINS`;
bump `?v=` token on every data redeploy; Stage 6 promotion (Command Session); AEC builds still failing.

**Operator preferences surfaced:** Wants both non-retail archetype maps at ~Retail bubble density
and **geographically spread across the whole map** (airports are the spread lever, not rail). Tier
counts shown are per-region (NA/EU tab), not global — clarify when numbers look low. Iterates fast
("more bubbles" → "as many as Retail" → "spread them out"); prefers diagnosis-then-fix with verified
numbers. Browser-in-the-loop verification valued. Cache-busting is mandatory on data redeploys.

---

## 2026-06-03 (earlier) | Totebox | claude-code (GIS — gateway-orchestration-gis)

**Done this session (large):**
- **Urban Fringe (VWH) + Commuter (PKS) made independent** of retail clustering — new
  `build-vwh-clusters.py` + `build-pks-clusters.py` (strict ≥2-distinct-category co-location, no
  metro gate, no hypermarket disqualifier — mirrors Retail). VWH **3,520**; PKS **5,596**
  (sized standalone airports as park-and-fly + standalone rail ≤60 km commuter-belt; 60 km is the
  data-derived daily-commute ceiling).
- **Chain expansion:** coord-based ISO fix (Würth etc.); CA/MX (RONA, Home Hardware, Fastenal,
  United Rentals, PartSource, Truper, Enterprise, Hertz); ES/IT (Norauto, Feu Vert, AKI 108,
  Brico io 72, Rexel/Sonepar, Loxam/Kiloutou). Spain 6→69, Italy 11→44, CA 32→170, MX→72.
- **Map UX:** Retail/Urban Fringe/Commuter radio group; mode-aware T1/T2/T3 BentoBox counts;
  click-to-drill on archetype dots; no centre dot in rings (GLOBAL-only dots); archetype
  BentoBox detail (auto-fill on drill-in, strongest-co-location-in-ring on click).
- **Research pages mobile pass** (Wikipedia-style collapsible sections, cross-page nav, print-doc
  reflow for Summary + Regional Markets, self-similar) + **SEO** (robots.txt, sitemap.xml, meta
  descriptions, OG, GIS titles) — site is live (HTTP 200) but needs Google Search Console submit.
- **Commuter/metro rail ingest RUNNING OVERNIGHT** (detached nohup `run-commuter-build.sh`,
  PID 958517) — self-completes ingest→build-pks→gateway sync. Fills PKS T2.

**Pending / carry-forward:** commuter build finishing tonight (monitor armed, flip NEXT.md to [x]
when DONE); `bricomart-es` 0 in OSM; AEC seismic/flood builds still failing (do NOT auto-run);
Stage 6 promotion of this session's commits (Command Session); Google Search Console submit (operator).

**Operator preferences surfaced:** Wants the map to look full/dense for visitors; strict
co-location but with sensible exceptions (airports park-and-fly, sized; rail in commuter belt).
Mobile polish matters — pages must be self-similar. Prefers concise diagnosis-then-fix. Wants to
leave heavy work running unattended overnight (`at` unavailable → nohup detached wrapper pattern).

---

## 2026-05-22–23 | Totebox | claude-code

**Done this session:** Investigated chart token coverage across design system layers. Confirmed `token-chart-semantic.yaml` + chart component CSS (nodes, connectors, panels, governance, tiers, matrix, venn) already committed in `pointsav-design-system` sub-clone at `ebdd101` (v0.2.0, 2026-05-21). Identified that `tokens/charts/` does not yet exist in vendor canonical — Stage 6 pending. Investigated `design.pointsav.com` pipeline: served by `app-privategit-design` reading from `dtcg-vault/` (DTCG JSON format), separate from the YAML-canonical layer. Wrote two outbox messages: (1) to `totebox@project-design` requesting DTCG conversion + dtcg-vault component entries; (2) to `command@claude-code` requesting outbox sweep + Stage 6. Command Session actioned both on 2026-05-22 (commit `537f15e`): project-design message relayed; Stage 6 marked in-progress.

**Pending / carry-forward:**
- Stage 6 of `ebdd101` to canonical `pointsav/pointsav-design-system` — in-progress per Command Session
- project-design to execute DTCG conversion + dtcg-vault entries for org-chart-node, org-chart-pill, org-chart-ellipse components
- 87 unstaged modified files in `pointsav-design-system` sub-clone working tree flagged to Command Session — root cause unknown; may be drift from canonical
- `--gold` CSS variant in `nodes.css` has no entity-role in `token-chart-semantic.yaml` — needs Master co-sign decision before DTCG entry
- Tetrad customer leg and wiki leg: leg-pending (unchanged)

**Operator preferences surfaced:** Operator asks direct questions about token coverage and pipeline state; prefers concise answers with a clear statement of what exists vs. what is still needed. Comfortable with session-layer explanations (Command vs. Totebox).
