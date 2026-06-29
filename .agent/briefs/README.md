# Briefs — project-gis

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs (native to project-gis)

| Brief ID | Title | Status |
|---|---|---|
| `gis-aec-climate-layers` | AEC Climate Layers | active |
| `gis-delivery-rearchitecture` | Delivery Rearchitecture | active |
| `gis-map-ux-audit-2026-06-20` | Map UX Audit 2026-06-20 | active |
| `gis-reports` | Reports | active |
| `gis-whitespace-cannibalization-model` | Whitespace + Cannibalization Model | active |
| `gis-top600-proforma-coverage` | TOP600 — Proforma Coverage Redesign | active |

## Foreign briefs (contamination — pending Command Session redistribution)

Briefs not native to project-gis; arrived via contamination events. Do not act on
foreign briefs — route via outbox to Command Session for redistribution.

| Brief file | Likely owner |
|---|---|
| BRIEF-app-privategit-design.md | project-design |
| BRIEF-bim-website-pipeline.md | project-bim |
| BRIEF-design-system-platform-2030.md | project-design |
| BRIEF-knowledge-platform-master.md | project-knowledge |
| BRIEF-marketing-platform-master.md | project-marketing |
| BRIEF-os-orchestration-build-out.md | project-orchestration |
| BRIEF-os-totebox-build-out.md | project-system |
| BRIEF-os-totebox-ppn-build-out.md | project-system |
| BRIEF-sel4-unikernel.md | project-system (verify) |
| BRIEF-slm-tier-split-architecture.md | project-intelligence (verify) |
| BRIEF-tier-b-gpu-restoration.md | project-intelligence (verify) |
| BRIEF-trademark-changeover-mcorp-capability-geometry.md | Command (verify) |
| BRIEF-training-pipeline-10x.md | project-intelligence (verify) |
| BRIEF-workplace-architecture.md | project-workplace |
| BRIEF-workplace-roadmap.md | project-workplace |
| BRIEF-workplace-workbench.md | project-workplace |

## Routing

All editorial drafts → `.agent/drafts-outbound/` → project-editorial.
All design artifacts → `.agent/drafts-outbound/` → project-design.
Stage 6 requests → Command Session outbox.
