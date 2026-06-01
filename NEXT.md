# NEXT.md — project-data

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-01

---

## State files — repaired 2026-06-01

- [x] **cleanup-log.md contamination** — 9 entries from other archives removed. Committed `59373c45`.
- [x] **Briefs sweep** — 10 contaminated briefs archived (project-intelligence x6, project-knowledge x2,
      project-console x1, project-infrastructure x1). README rewritten.
- [x] **CLAUDE.md title** — corrected to project-data.
- [x] **session-start.md** — replaced with project-data orientation.
- [x] **manifest.md** — replaced with project-data GIS/JOURNAL tetrad.
- [x] **session-context.md** — rewritten with correct project-data context.
- [x] **NEXT.md** — this file; contaminated project-gis items removed.

---

## JOURNAL programme — active blockers

- [ ] **J1 §7.2 OLS regression** — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)`.
      Blocked on Phase 24B: Kontur H3 population join to `work/clusters-ols.csv`. [2026-05-28]
- [ ] **J1 permutation test** — `sim-tier-permutation.py`; 10,000 spatial shuffles, one-tailed p-value. [2026-05-28]
- [ ] **J3 §6 Results** — AEC flood + seismic build coverage metrics required. [2026-05-29]
      AEC build results checked 2026-06-01:
      - **Seismic (cron 2026-06-01T05:00Z): partial — EXIT 0, but 3 layers skipped:**
        USGS NSHM CONUS (curl empty reply from ScienceBase), ESHM20 EU (GeoJSON not produced),
        GWL_FCS30 wetland (download failed). EU + wetland skip → coverage incomplete for J3.
        Ran twice (05:00Z + 05:12Z); second run also stuck at [1/9]. Needs fix in project-gis.
      - **Flood (cron 2026-06-01T05:18Z): FAILED at step [15/17]** — Python `$META_PATH`
        env var not expanded (`Path('$META_PATH').read_text()` literal, not `os.environ`).
        Needs fix in project-gis `build-aec-flood.sh`. Outbox flagged to Command.
      Coverage metrics still blocked. Will unblock after fixes + re-runs.
- [ ] **J3 coverage metrics export** — write `export-aec-coverage.py` additions after both builds complete and produce valid data.
- [ ] **J4 §4–§5 language pass** — at project-editorial (outbox ref: 952b2b09). [2026-05-29]

---

## Stage 6

- **Archive-local commits only — no canonical promotion needed.**
  All 3 commits (59373c45, 005cc299, 161efbd1) are `.agent/`-only changes
  (cleanup-log, outbox, briefs, state files). The canonical monorepo gitignores `.agent/`.
  Outbox message `project-data-20260601-stage6-request` updated to `operator-pending`
  with this finding by the operator.

---

## Command actions pending

- [ ] **Brief redistribution** — 6 project-intelligence briefs archived in this archive need
      Command to copy to `clones/project-intelligence/.agent/briefs/`:
      BRIEF-slm-substrate-master, BRIEF-slm-learning-loop, BRIEF-project-intelligence-active-work,
      AI-AUDIT-baseline-2026-05-31, BRIEF-substrate-phd-thesis-2026-05-27, BRIEF-vm-hardening-and-consolidation.
      Outbox message `project-data-20260601-brief-redistribution` queued.

---

## Regional Markets (editorial — dispatched to project-editorial)

- [ ] **294488f gap** — re-dispatch signal sent to project-editorial
      (outbox `project-gis-20260531-rm-redispatch-294488f`). Command to verify
      content-wiki-projects after project-editorial re-commits.
- [ ] **A10/A11/A12** — hold pending revision (methodology correction applies; project-gis archive
      handles the data verification; this archive tracks JOURNAL dependency only).

---

## Operator-blocked (no Totebox action until input received)

- J1/J2/J3/J4/J6 — ORCID IDs required for submission; operator action.
- J1 Phase 24B — Kontur H3 population join; operator must initiate data acquisition.
- CBRE/JLL leasing data — Year 2 research; operator action.
