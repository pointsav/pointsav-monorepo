# Briefs — project-knowledge

Durable, git-tracked project briefs for this archive. Read at session start
(AGENT.md startup step 7). A brief carries `artifact: brief` frontmatter and a
`status: active | archived` field. Briefs are permanent — supersede by editing
`status: archived` or `git mv` to an `archive/` subdir; never delete.

## Active briefs

| Brief | Topic | Status |
|---|---|---|
| `BRIEF-knowledge-platform.md` | Knowledge Platform — Phases 1–5 done; design competition in progress; Phase 6 gated; product strategy open | active |

## Artifact routing (this archive)

Per `~/Foundry/conventions/artifact-classification.yaml`:
- TOPIC-* / GUIDE-* / COMMS-* → `.agent/drafts-outbound/` → project-editorial
- DESIGN-* / ASSET-* → `.agent/drafts-outbound/` → project-design
- CONVENTION-* → Command Session → `~/Foundry/conventions/`
- CODE-* / SCRIPT-* / CONFIG-* / DATA-* → commit directly (self-contained)
