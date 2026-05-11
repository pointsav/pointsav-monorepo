# .agent/plans/ — Research & planning documents

Engine-agnostic, git-tracked planning space for this Totebox Archive.
All engines (Claude Code, Gemini CLI) read and write here.

## Lifecycle

| Stage | Action |
|---|---|
| Plan | Create `<topic>.md` here with research, spec, or todo list |
| Implement | Reference plan during implementation; update as decisions land |
| Milestone | Promote to artifact(s) via `drafts-outbound/` — see routing table below |
| Archive | Move completed plan to `.agent/plans/archive/` or delete if superseded |

## Artifact routing (at milestone)

| Artifact type | Gateway project | Destination | Notes |
|---|---|---|---|
| TOPIC-* | project-editorial | content-wiki-documentation, content-wiki-projects, or content-wiki-corporate | Bilingual required (EN + ES) |
| GUIDE-* | project-editorial | woodfine-fleet-deployment/\<cluster\>/ | EN only; target cluster in frontmatter |
| COMMS-*, LEGAL-*, TRANSLATE-* | project-editorial | varies | Per language-protocol-substrate |
| TEXT-* | project-intelligence or project-editorial | data/training-corpus/ or drafts-outbound/ | Corpus text vs. editorial prose |
| DESIGN-COMPONENT, DESIGN-RESEARCH | project-design | pointsav-design-system | |
| DESIGN-TOKEN-* (generic) | project-design | pointsav-design-system | Requires master_cosign in frontmatter |
| DESIGN-TOKEN-* (PointSav branded) | project-design | pointsav-media-assets | |
| DESIGN-TOKEN-* (Woodfine branded) | project-design | woodfine-media-assets | |
| ASSET-* | project-design | pointsav-media-assets or woodfine-media-assets | |
| BIM-* | project-bim | woodfine-design-bim | Never routes to pointsav-design-system |
| LICENSE-* | Command Session (admin-tier) | factory-release-engineering | |
| Self-contained | this project-* | own drafts-outbound/ or direct commit | Valid — artifact stays in originating project |

## Rules

- Save planning files HERE — not to `~/.claude/plans/` or `~/.gemini/tmp/`
- Files here are git-tracked, survive session resets, and are accessible to all engines
- `session.lock` files in `.agent/engines/` are NOT committed (excluded via `.gitignore`)
- One file per research area; use descriptive names (e.g., `WIKIPEDIA-PARITY-MASTER-PLAN.md`)
