---
schema: foundry-draft-v1
draft_id: design-token-private-office
language_protocol: DESIGN-TOKEN-CHANGE
master_cosign: master@claude-code 2026-05-06T23:35Z
state: master-cosigned
target_path: vendor/pointsav-design-system/tokens/bim/spatial-programmes.dtcg.json
created: 2026-05-05T00:00:00Z
revised: 2026-05-06T17:45:00Z
author: task@project-bim
cites: [ifc-4-3, uniclass-2015, bsdd-v1, dtcg-w3c]
master_cosign_required: true
research_done_count: 1
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Originated: .agent/artifacts/TEXT-PRIVATE-OFFICE-TOKEN.md (2026-05-05)
  Schema updated to foundry-draft-v1 DESIGN-TOKEN format.
  Note: DESIGN-TOKEN changes require master co-sign per DOCTRINE.md.
research_inline: true
---

# Design Token Proposal: Private Office Baseline BIM Token

## Token Identity

| Field | Value |
|---|---|
| Token ID | `study.private-office` |
| DTCG type | `bim-element` |
| IFC class | `IfcSpace` |
| IFC predefined type | `USERDEFINED` |
| IFC object type | `PrivateOffice` |
| Uniclass 2015 | `SL_25_10_60` |
| Uniclass title | Private office |
| bSDD URI | `https://identifier.buildingsmart.org/uri/bim4ren/broenergy/0.1/class/PrivateOffice` (provisional) |

## Geometry Archetype

- **Floor plate:** 12–15 m² rectangular
- **Clear height:** 2.7 m minimum
- **Acoustic isolation:** Rw ≥ 40 dB (partition to adjacent offices)
- **Lighting:** 300–500 lux at working plane (EN 12464-1)

## Sub-Token References

| Sub-token alias | Token reference | Notes |
|---|---|---|
| `{study.private-office.wall}` | `--bim-mat-office-partition` | Acoustic-rated partition |
| `{study.private-office.ceiling}` | `--bim-mat-acoustic-ceiling-tile` | Suspended acoustic ceiling |
| `{study.private-office.floor}` | `--bim-mat-raised-access-floor` | Optional; programme-dependent |
| `{study.private-office.glazing}` | `--bim-elem-interior-glazing-solid` | Optional clerestory |

## DTCG JSON Fragment (proposed)

```json
"study.private-office": {
  "$type": "bim-element",
  "$description": "Private office space — 12-15 sqm, acoustic-rated enclosure",
  "ifc_class": "IfcSpace",
  "ifc_object_type": "PrivateOffice",
  "uniclass_ref": "SL_25_10_60",
  "bsdd_uri": "https://identifier.buildingsmart.org/uri/bim4ren/broenergy/0.1/class/PrivateOffice",
  "area_min_sqm": { "$value": 12 },
  "area_max_sqm": { "$value": 15 },
  "clear_height_m": { "$value": 2.7 },
  "acoustic_isolation_rw_db": { "$value": 40 },
  "wall": { "$value": "{study.private-office.wall}" },
  "ceiling": { "$value": "{study.private-office.ceiling}" }
}
```

## Open Questions

1. The bSDD URI above (`bim4ren/broenergy`) is provisional — bSDD does not yet have a stable
   "Private Office" entry in the buildingSMART hosted dictionary. The correct URI should be
   confirmed when bSDD content for office space types is published, or a PointSav-hosted bSDD
   dictionary entry should be created.

## Co-sign Note

Per DOCTRINE.md, DESIGN-TOKEN changes require master co-sign in frontmatter before commit
to design-system. This draft is staged for Master review and co-sign before the Root session
commits to `pointsav-design-system`.
