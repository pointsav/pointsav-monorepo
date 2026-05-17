# Agent 3 — Key Plans Index — Report

**Date:** 2026-05-17
**Author:** totebox@project-bim (Opus 4.7 / 1M context)
**Scope:** Reconcile the Key Plans Index slide deck against source-of-truth
documents; produce TOPIC drafts; surface inconsistencies.

---

## HTML changes (5 edits to `preview/key-plans-index.html`)

1. **Title slide subtitle** — rewrote to describe the deck as Private
   Office + five Professional Office sub-types (Index entries 1–3 and
   9–37 out of 72 total), and corrected the token-file path from the
   non-existent `woodfine-design-bim` to the actual
   `woodfine-bim-library/tokens/bim/`.
2. **Slide 1 lede** — now cites the V3 Master Summary (2026-01-06)
   as the reconciliation source, not just the DTCG file names.
3. **Slide 2 lede** — clarified that FFE codes (M/B/L/A/C-1/2/3 =
   1,100 / 1,400 / 800 SF) are 1/4-floor tile sub-allocations, **not**
   the same as the Small/Medium/Large key plan footprints.
4. **Slide 3 lede + source note** — explicit Academic Small
   reconciliation: 105 m² / 1,131 SF is the V3 authoritative value,
   superseding 87.7 m² (Samples_V2 "smaller" iteration) and 110.5 m²
   (V2 February Summary).
5. **Slide 4 PO callout** — clarified that the 5.9944 m Habitat rounds
   to 6.0 m / 19'8" in the V3 table (not "marginally under" without
   context).

No values changed in the BIM_TOKENS object — they already matched
the V3 master table. Comments added to `privateOfficeZones` to
document the V3-table-display rounding vs underlying-spec precision.

---

## Academic Small 105 vs 87.7 m² resolution

Three values appear across source documents:

| Value | Source | Status |
|---|---|---|
| 87.7 m² / 944 SF | Samples_V2.pdf p.5 ("Consider removing one office?" variant) | Earliest iteration; superseded |
| 110.5 m² / 1,189 SF | V2 Summary (Feb 2025); Samples_V2.pdf p.5 full option | Intermediate iteration |
| **105 m² / 1,131 SF** | **V3 Master Summary (2026-01-06) page 1** | **Authoritative** |

**Token-file action required (operator):** The
`professional-office-subtypes.dtcg.json` file currently carries the
87.7 m² value with a note "Source: AEC_Floor Plates_Key Plans_Samples_V2.pdf."
That value should be updated to 105 m² / 1,131 SF to match
`building-width-calculator.dtcg.json` and the V3 Master Summary.

---

## Per-use-type key plan inventory (V3, 2026-01-06)

| Use type | S (m²/SF) | M (m²/SF) | L (m²/SF) | Z1 / Z2 / Z3 |
|---|---|---|---|---|
| Private Office | 30 / 325 | 43 / 465 | 64 / 685 | 6.0 / 1.4 / — |
| Laboratory | 195 / 2,099 | 316 / 3,401 | 401 / 4,313 | 6.8 / 4.8 / 3.0 |
| Academic | 105 / 1,131 | 240 / 2,583 | 378 / 4,070 | 4.7 / 3.0 / — |
| Business | 311 / 3,350 | 400 / 4,302 | 669 / 7,524 | 6.0 / 7.3 / 2.7 |
| Medical | 223 / 2,402 | 331 / 3,568 | 486 / 5,231 | 7.2 / 4.9 / 2.9 |
| Civic | 270 / 2,912 | 577 / 6,215 | 822 / 8,850 | 6.0 / 7.2 / 3.6 |
| Professional Office (baseline) | 130 / 1,400 | (none) | (none) | 6.0 / 3.8 / 2.0 |

Master inventory: **72 key plans** across 9 Development Classes —
General (25), Professional Centre (13), Retail Select (3), Suburban
Office (14), Tech Industrial (3), Landscaping (4), Parking (10).

---

## TOPIC drafts created (5 files, `.agent/drafts-outbound/`)

- `topic-bim-key-plans-index.draft.md` — master 72-row inventory
  + reconciliation table + naming conventions + FFE-tile-code clarification
- `topic-bim-private-office-key-plans.draft.md` — PO-1 / PO-2 / PO-3
  with furniture lists, occupancy figures, the non-modularity
  principle, licensing-vs-lease note
- `topic-bim-medical-key-plans.draft.md` — M3/M1/M2 (sized
  Small/Medium/Large) with KaVo uniQa anchor, cappelletti sestito
  hand-annotated revisions, AART-superseded design lineage,
  reception line-of-sight requirement
- `topic-bim-business-key-plans.draft.md` — B-1/B-2/B-3 with
  Building Width Options A/B/C/D, MW3 design-principle commentary
  verbatim, Tenant-Washroom-in-Magazine pattern
- `topic-bim-professional-office-key-plans.draft.md` — the
  superordinate baseline (Z1 6.0 / Z2 3.8 / Z3 2.0, Small 130 m²),
  why no Medium / Large exist, the 21 m vs 21.6 m demising-structural
  delta

All drafts carry full `foundry-draft-v1` frontmatter with research
provenance, route to `vendor/content-wiki-projects` at
`topics/bim/<slug>.md`, EN-only.

---

## Cross-document inconsistencies surfaced

1. **Academic Small triple-value** (resolved — see above).
2. **Medical FFE code numbering vs size** — cappelletti sestito
   sketches show M3 = Small, M1 = Medium, M2 = Large (authoring order,
   not size order). The dtcg `ffe_sizes_sf` happens to be correctly
   labelled (M3 = 800 SF, M1 = 1100 SF, M2 = 1400 SF) but anyone
   sorting by code alphabetically would mis-order. Recommend
   downstream code prefer size labels.
3. **Z2 Magazine — Business** — V3 Summary table says 7.3 m, but the
   Sketch 2 / Option B sample in the Business DISCOVERY uses 6.76 m.
   Two snapshots in time; the V3 is later.
4. **Professional Office total width** — 21 m (centerline) vs 21.6 m
   (token arithmetic with walls). Suggests a 0.6 m demising-structural
   overhead that should be encoded as a separate DTCG token.
5. **Token file location** — operator instructions referenced
   `woodfine-design-bim/tokens/bim/` but the tokens actually live at
   `woodfine-bim-library/tokens/bim/`. HTML deck corrected.
6. **Z3 corridor for Academic and Private Office** — V3 table shows
   "—" (dash), DTCG stores 0.0 m, HTML displays "—" / "none". All
   three agree functionally; conventions differ.
7. **Cosmetic note on Z2 Private Office** — V3 displays "1.4 m / 4'6"";
   DTCG stores "1.3716 m / 4'6"". HTML uses the precise 1.3716 m
   (now annotated).

---

## Hand-drawn content flagged for operator interpretation

- **Medical M3 sketch (cappelletti sestito) revisions 1–9** — hand
  annotations include a "bubble" capacity question ("what is the
  MAX?"), an instruction to "extend office into zone #2", and a
  reception line-of-sight requirement. All captured in the medical
  TOPIC; the "MAX" question remains an open question.
- **Medical M1 sketch revisions** — a vestibule pattern ("sink or
  seat before you go into the washroom") drawn as an inset diagram.
  Captured in the TOPIC.
- **Medical M2 sketch revision 7** — "move washrooms together" with
  a small diagram pairing two washrooms. Captured.
- **NTD margin notes** — three "Note To Do" annotations on the
  Medical sketches capture leasing principles (differential rent
  rate Habitat vs Magazine, narrower Staff Room table for Magazine
  reduction, "gold standard always achievable" certificate continuity).
  All preserved verbatim in the medical TOPIC.

No hand content was illegible; the Medical sketches are clean and
the cappelletti sestito hand is consistent.

---

## PDF regeneration status

The recipe `cd preview && NODE_PATH=/home/jennifer/sandbox/working/ps-talking-points/node_modules node build-pdf.mjs key-plans-index.html`
**could not execute** in this session — the sandbox blocked every
`node` invocation. The HTML edits are in place and self-contained
(no JS recomputation needed because the BIM_TOKENS values were
already V3-correct). Operator action: run the recipe to regenerate
`key-plans-index.pdf`. Page count should remain 5 (1 title + 4
content slides).

---

## Slide / page count

Title slide + 4 content slides = **5 pages**. No structural changes.
Visual design and print CSS / Playwright generator untouched per
constraints.
