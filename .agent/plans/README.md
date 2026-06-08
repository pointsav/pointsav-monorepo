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
| TOPIC-* | project-editorial | content-wiki-documentation | Bilingual required (EN + ES) |
| DESIGN-COMPONENT, DESIGN-RESEARCH | project-design | pointsav-design-system | |
| Self-contained | this project-proforma | own drafts-outbound/ or direct commit | HTML tools commit directly |

## Rules

- Save planning files HERE — not to `~/.claude/plans/` or `~/.gemini/tmp/`
- Files here are git-tracked, survive session resets, and are accessible to all engines
- `session.lock` files in `.agent/engines/` are NOT committed (excluded via `.gitignore`)
- One file per research area; use descriptive names
