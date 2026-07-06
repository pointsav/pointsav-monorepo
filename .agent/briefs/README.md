# Briefs — project-bim

Active briefs and artifact routing for this archive.

## Active briefs

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-app-privategit-bim.md](BRIEF-app-privategit-bim.md) | active | BIM Object Library web surface; live on bim.woodfinegroup.com; includes the Key Plans SVG diagram system (merged 2026-07-06 — see below) |
| [BRIEF-tool-keyplan.md](BRIEF-tool-keyplan.md) | active | tool-keyplan Rust crate v0.0.1; DTCG validation engine; key-plans-registry.md pending; functionally unchanged since 2026-05-22 |

## Superseded briefs

| Brief | Superseded by | Why |
|---|---|---|
| [BRIEF-key-plans-site.md](BRIEF-key-plans-site.md) | BRIEF-app-privategit-bim.md | Described the standalone `app-orchestration-bim`/`local-bim-orchestration` service, retired 2026-07-02. Still-valid technical content (SVG tier system, furniture counts) merged into the successor's "Key Plans SVG diagram system" section. Kept for archival record, not deleted. |

## Artifact routing

| Type | Destination |
|---|---|
| TOPIC-*, GUIDE-* | `.agent/drafts-outbound/` → project-editorial |
| DESIGN-*, ASSET-* | `.agent/drafts-outbound/` → project-design |
| CODE-*, SCRIPT-*, CONFIG-*, DATA-* | commit directly to monorepo |
| Stage 6 + deploy requests | outbox → command@claude-code |
