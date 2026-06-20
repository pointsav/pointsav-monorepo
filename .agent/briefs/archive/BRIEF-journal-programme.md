---
artifact: brief
brief-id: BRIEF-journal-programme
title: "JOURNAL Programme — J1–J7"
owner: totebox@project-data
status: active
schema: foundry-brief-v1
archive: project-data
created: 2026-06-12
updated: 2026-06-12
---

# BRIEF — JOURNAL Programme

## Mission

Advance J1–J7 academic manuscripts from draft to submission-ready. Each paper documents
a distinct research contribution from the monorepo's architecture, data pipeline,
or system design. Papers route to project-editorial for language-review gating before
external submission. All manuscripts live at `JOURNAL/` in the monorepo root.

## Paper inventory

| ID | File | State | Venue | forbidden_terms_cleared | Blocker |
|----|------|-------|-------|------------------------|---------|
| J1 | JOURNAL-retail-colocation-v0.1.draft.md | draft | Economic Geography (Wiley, IF 7.2) | true (v0.3) | §7.2 pending Phase 24B data |
| J2 | JOURNAL-trustworthy-systems-v0.1.draft.md | draft | ASPLOS (ACM, 19.4% AR) | true | Bench #9 re-run pending |
| J3 | JOURNAL-aec-data-layers-v0.1.draft.md | draft | Automation in Construction (Elsevier, IF 12.0) | true (v0.2) | §6 Results pending coverage metrics (project-gis scope) |
| J4 | JOURNAL-private-network-v0.3.draft.md | draft | IEEE TIFS (IF 9.65) | true (v0.3) | §4–§5 lang-pass at project-editorial |
| J5 | JOURNAL-totebox-orchestration-v0.1.stub.md | draft | MLSys (ACM, 22% AR) | **true (v0.3, 2026-06-12)** | §4 Implementation + §5 Evaluation + §8 Conclusion pending deployment evidence |
| J6 | JOURNAL-desktop-environment-v0.1.stub.md | draft | ACM TOCHI | true (v0.2) | §6 Results pending user study |
| J7 | (renaming note) | — | — | — | J5 was previously called J7 in session context; canonical file is JOURNAL-totebox-orchestration |

> Note: the `artifact-registry.md` lists J1–J7 with slightly different numbering (J5 is
> totebox-orchestration in that file). Use the file names as the authoritative identifiers.

## Project-data scope

Work actionable in this archive:
- J5 (JOURNAL-totebox-orchestration): §4 Implementation evidence comes from os-totebox
  build-out. Update §4 when the first deployment is live.
- J5: §5 Evaluation + §6 Discussion + §8 Conclusion after benchmark harness.
- J5: ORCID IDs required before submission (operator action).

Work in other archives' scope:
- J1 §7.2 Phase 24B data — project-gis
- J3 §6 coverage metrics — project-gis
- J4 lang-pass — project-editorial

## Routing

When a paper is cleared for submission:
1. Confirm `forbidden_terms_cleared: true` and `state: draft` → change to `under-review`.
2. Stage to `.agent/drafts-outbound/` with `foundry-draft-v1` frontmatter.
3. Route to project-editorial for final language pass + submission gating.
4. Operator submits to journal; update `submission_status: submitted`.

## Cross-references

- `BRIEF-os-totebox-ppn-build-out` — provides §4 evidence for J5
- `.agent/rules/journal-artifact-discipline.md` — full schema + forbidden-vocabulary list
- `JOURNAL/` directory — canonical manuscript location
