# NEXT.md — project-editorial

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-05-28

---

## JOURNAL programme — data-gated blockers

- [ ] **J1 §7.2 primary spec** — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)` — blocked on Phase 24B (Kontur population join to clusters-ols.csv + O-D data join). project-gis owns. Outbox sent `project-editorial-20260528-j1-j3-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J1 permutation test** — `sim-tier-permutation.py` needs writing (10,000 shuffles, one-tailed p-value, cluster coords at project-gis `work/clusters-ols.csv`). project-gis owns. [2026-05-28 totebox@project-editorial]
- [ ] **J2 Bench #9 re-run** — `verify_inclusion_proof` 1024-leaf; ±11% CI → <5% CI; quiet GCP n2 host. project-system owns. Outbox sent `project-editorial-20260528-j2-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J2 citation placeholders** — 9 `[external: ...]` stubs need stable IDs in `citations.yaml`. project-system owns. [2026-05-28 totebox@project-editorial]
- [ ] **J3 §6 Results** — AEC nightly build coverage metrics (4 scripts × N countries). project-gis owns. Outbox sent `project-editorial-20260528-j3-coverage-metrics` + `project-editorial-20260528-j1-j3-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J4 §4–§5 benchmarks** — WireGuard tunnel establishment, rekey latency, policy-change propagation, failure-mode behaviour. project-infrastructure owns. Outbox sent `project-editorial-20260528-j4-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J4 two unresolved citations** — Cameron CA audit-log incident study + ZTA latency comparison. project-infrastructure to find or substitute. [2026-05-28 totebox@project-editorial]
- [ ] **J5 full writing pass** — HOLD until J2 submitted. project-orchestration owns. Outbox sent `project-editorial-20260528-j5-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J6 §6 Results** — user study execution (§5 protocol in JOURNAL file). project-bim owns. Outbox sent `project-editorial-20260528-j6-return`. [2026-05-28 totebox@project-editorial]

## JOURNAL programme — operator actions (all papers)

- [ ] **ORCID IDs** for Jennifer M. Woodfine, Peter M. Woodfine, Mathew Woodfine — required before any submission. [2026-05-28 totebox@project-editorial]
- [ ] **J1 bilingual ES sibling** — Spanish translation of J1 required before Economic Geography submission (per JoEG policy). [2026-05-28 totebox@project-editorial]

## Inbox — pending pickup

- [x] `command-20260526-dev-phase3-drafts-relay` — ACTIONED 2026-05-28: TOPIC committed to `media-knowledge-documentation/applications/app-privategit-workbench.md` + ES stub; GUIDE staged + routed to Command via outbox `project-editorial-20260528-guide-workbench-routing`.

## Backlog drift — registry items needing source-project action

- [ ] **B13 Regional Name Resolution TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B14 Co-location Tier Nomenclature TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B15 GIS as BIM Substrate TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B16 UK/EU Food Retail Coverage TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]

## Convention layer — pending (from earlier session)

- [ ] `conventions/artifact-classification.yaml` — add JOURNAL entry (schema, gateway, destinations, bilingual_pair: false). [2026-05-27 totebox@project-editorial]
- [ ] `conventions/journal-artifact-discipline.md` — new file; copy/adapt from `.agent/rules/journal-artifact-discipline.md`. Command Session scope. [2026-05-27 totebox@project-editorial]
- [ ] `conventions/artifact-registry.md` — add JOURNAL section row. Command Session scope. [2026-05-27 totebox@project-editorial]
- [ ] Foundry `NEXT.md` — add JOURNAL programme tracking checkbox. Command Session scope. [2026-05-27 totebox@project-editorial]

---

## Completed this session (2026-05-28)

- [x] A6 PROSE-RESEARCH editorial pass + committed to wiki — `a77e1bb` (reference/geometric-site-selection-national-tenancy.md + ES stub; §7.2 DATA PENDING annotation)
- [x] B5/B11/B12 TEXT artifacts dispatched to project-gis for coverage verification + language pass
- [x] Convention layer outbox message sent to command@claude-code (4 items: artifact-classification.yaml, journal-artifact-discipline.md, artifact-registry.md, Foundry NEXT.md)
- [x] B13–B16 registry drift surfaced; project-gis notified via NEXT.md
- [x] J3 full writing pass + language pass — `02117825`
- [x] J6 §1–§5 writing pass + language pass — `da4925a4`
- [x] J4 §1–§3 + §6–§7 writing pass + language pass — `67eb9a37`
- [x] J1 §7.0 preliminary OLS (Model A + B) + F6 partial + OLS script — `37523014`
- [x] Project-gis messages archived + JOURNAL brief updated to 2026-05-28 — `a34825b6`
- [x] 5 JOURNAL return outbox messages sent to source projects — `25023ce9`
