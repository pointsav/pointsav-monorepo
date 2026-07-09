# Briefs — project-bim

Active briefs and artifact routing for this archive.

## Active briefs

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-app-privategit-bim.md](BRIEF-app-privategit-bim.md) | active | CMS/catalog for BIM Objects + Compositions; live on bim.woodfinegroup.com; 2026-07-06 v2 design direction proposed (see simulation brief) |
| [BRIEF-app-orchestration-bim.md](BRIEF-app-orchestration-bim.md) | active | Forward scope redefined 2026-07-06 as the **BIM Editor/Viewer** (distinct from the CMS above) — software not started, definition locked first. Carries the historical Key Plans Live Site record (that specific feature moved to app-privategit-bim). |
| [BRIEF-tool-keyplan.md](BRIEF-tool-keyplan.md) | active | tool-keyplan Rust crate v0.0.1; the Composition compiler; functionally unchanged since 2026-05-22 |

## Reference briefs

| Brief | Summary |
|---|---|
| [BRIEF-simulation-bim-library-denver-woodfine.md](BRIEF-simulation-bim-library-denver-woodfine.md) | Design-thinking exercise (2026-07-06): simulated PointSav→Denver Airport BIM CMS sale used as a Denver-content-free structural pass-through to derive a Woodfine-native v2 design candidate. Grounds the corrected BIM Object vs. Composition definition. |

## Superseded briefs

| Brief | Summary |
|---|---|
| [BRIEF-bim-website-pipeline.md](BRIEF-bim-website-pipeline.md) | Pre-redesign record (2026-06-03) of the old `app-orchestration-bim` v0.0.2 service. Reclaimed 2026-07-09 from a misfile in project-editorial's briefs dir; fully superseded by `BRIEF-app-privategit-bim.md`. Kept for history only. |

## Artifact routing

| Type | Destination |
|---|---|
| TOPIC-*, GUIDE-* | `.agent/drafts-outbound/` → project-editorial |
| DESIGN-*, ASSET-* | `.agent/drafts-outbound/` → project-design |
| CODE-*, SCRIPT-*, CONFIG-*, DATA-* | commit directly to monorepo |
| Stage 6 + deploy requests | outbox → command@claude-code |
