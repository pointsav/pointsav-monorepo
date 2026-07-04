# Briefs — project-bim

Active briefs and artifact routing for this archive.

## Active briefs

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-app-privategit-bim.md](BRIEF-app-privategit-bim.md) | active | BIM Object Library web surface; live on bim.woodfinegroup.com; 2026-07-03 CMS reposition superseded — presentation-layer rebuild in progress (Envelope-as-Navigation direction) |
| [BRIEF-key-plans-site.md](BRIEF-key-plans-site.md) | active | Key Plans live site (app-orchestration-bim v0.0.3); SVG diagrams + Corporate Office blocked |
| [BRIEF-tool-keyplan.md](BRIEF-tool-keyplan.md) | active | tool-keyplan Rust crate v0.0.1; DTCG validation engine; key-plans-registry.md pending |

## Artifact routing

| Type | Destination |
|---|---|
| TOPIC-*, GUIDE-* | `.agent/drafts-outbound/` → project-editorial |
| DESIGN-*, ASSET-* | `.agent/drafts-outbound/` → project-design |
| CODE-*, SCRIPT-*, CONFIG-*, DATA-* | commit directly to monorepo |
| Stage 6 + deploy requests | outbox → command@claude-code |
