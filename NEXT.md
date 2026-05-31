# NEXT.md — project-gis

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Full TODO with all sections and sequencing: `.agent/plans/project-infrastructure-todo.md`.

Last updated: 2026-05-31

---

## VM-MediaKit — Phase 1 COMPLETE (6/6) [2026-05-29]

- [ ] **J1 §7.2 OLS regression** — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)`.
      Blocked on Phase 24B: Kontur H3 population join to `work/clusters-ols.csv`. [2026-05-28]
- [ ] **J1 permutation test** — `sim-tier-permutation.py`; 10,000 spatial shuffles, one-tailed p-value. [2026-05-28]
- [ ] **J3 §6 Results** — AEC flood + seismic build coverage metrics required. [2026-05-29]

## AEC pipeline

- [ ] **Flood build (Night 5)** — Aqueduct URL fix applied 2026-05-31 (skip-on-failure logic added).
      Re-run crontabbed 2026-06-02T05:00Z. Check `build-aec-flood.log` at session start.
- [ ] **Seismic rebuild** — URL fix in bd17a348; crontabbed 2026-06-01T05:00Z.
      Check `build-aec-seismic.log` at next session start.
- [ ] **Coverage metrics export script** — write `export-aec-coverage.py` additions for flood + seismic
      after both builds complete. Feeds J3 §6 Results and GUIDE A14.

## Stage 6 pending

- [ ] **4 commits** (39aa1b11 → d1899abb) awaiting Command Session canonical promote.
      Staging divergence documented in outbox `project-gis-20260530-staging-divergence`.
      Needs rebase/cherry-pick reconciliation before canonical push.
- [ ] **Session cleanup commit** (this session) — pending; will add to Stage 6 queue.

## Regional Markets

- [ ] **294488f gap** — re-dispatch signal sent to project-editorial
      (outbox `project-gis-20260531-rm-redispatch-294488f`). Command to verify
      content-wiki-projects after project-editorial re-commits.
- [ ] **A10/A11/A12** — hold pending revision (methodology correction applies).
      Wichita suburbs (Derby KS, Andover KS) may qualify under corrected definition; needs
      data verification from `score-regional-markets.py` output.

## Editorial backlog dispatched this session

- [x] **B13** TOPIC-regional-name-resolution — dispatched 2026-05-31
- [x] **B14** TOPIC-colocation-tier-nomenclature — dispatched 2026-05-31
- [x] **B15** TOPIC-gis-as-bim-substrate — dispatched 2026-05-31
- [x] **B16** TOPIC-uk-eu-food-retail-coverage — dispatched 2026-05-31
