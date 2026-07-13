# Briefs — project-bim

Active briefs and artifact routing for this archive.

## Active briefs

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-bim-v3-hyperscaler-redesign.md](BRIEF-bim-v3-hyperscaler-redesign.md) | active | 2026-07-09: full-reset visual redesign (fresh audit + research + Fable synthesis, real prototype to local preview) + BIM Library/BIM Objects narrative initiative. Explicitly does NOT build on the RD.1-7/envelope-diagram lineage in BRIEF-app-privategit-bim.md — operator called that approach not working. |
| [BRIEF-app-privategit-bim.md](BRIEF-app-privategit-bim.md) | active | CMS/catalog for BIM Objects + Compositions; live on bim.woodfinegroup.com. Spanish (/es/*) support shipped Rounds 11-13 (2026-07-12/13) — full Tier 1 + Objects/Search/entity-data + Key Plans/Research chrome; Key Plan data + Research essay content stay English by design. Pushed to foundry-prod 2026-07-13; a real static-asset sync gap (bim-planroom.css 404s on prod) found same day, escalated to Command, unresolved as of session end. Prior redesign-research lineage (RD.1-7) superseded by BRIEF-bim-v3-hyperscaler-redesign.md. |
| [BRIEF-app-orchestration-bim.md](BRIEF-app-orchestration-bim.md) | active | Forward scope redefined 2026-07-06 as the **BIM Editor/Viewer** (distinct from the CMS above) — software not started, definition locked first. Carries the historical Key Plans Live Site record (that specific feature moved to app-privategit-bim). |
| [BRIEF-tool-keyplan.md](BRIEF-tool-keyplan.md) | active | tool-keyplan Rust crate v0.0.1; the Composition compiler; functionally unchanged since 2026-05-22 |
| [BRIEF-bim-server-external-positioning.md](BRIEF-bim-server-external-positioning.md) | active | 2026-07-10: "BIM Objects as a Carbon-Design-System-style product other companies adopt" — Opus+Fable research found a real license conflict (Apache-2.0 vs CC BY-ND on Object data) that needs explicit operator re-scoping before this idea can proceed; also stub IFC exports and catalog-breadth gaps. Not folded into JOURNAL essay seeding. |

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
