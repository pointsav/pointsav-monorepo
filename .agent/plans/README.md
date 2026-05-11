# .agent/plans/ — Sandbox research & planning documents

Engine-agnostic planning space for the mathew sandbox.
All engines (Claude Code, Gemini CLI) read and write here.

## Lifecycle

| Stage | Action |
|---|---|
| Plan | Create `<topic>.md` here with research, spec, or todo list |
| Implement | Reference plan during implementation; update as decisions land |
| Milestone | Promote to artifact(s) via `drafts-outbound/` — see artifact registry |
| Archive | Move completed plan to `.agent/plans/archive/` or delete if superseded |

## Artifact routing

See `~/Foundry/conventions/artifact-registry.md` for the canonical routing table.
Sandbox sessions are Task-equivalent scope — same artifact types and destinations apply.

## Rules

- Save planning files HERE — not to `~/.claude/plans/` or `~/.gemini/tmp/`
- `session.lock` files in `.agent/engines/` are NOT committed
