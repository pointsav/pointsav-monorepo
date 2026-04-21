# Driving service-slm with Claude Code

This repository is set up to be driven by Claude Code across many
sessions. This document explains the workflow.

## The memory hierarchy

Three levels, each scoped differently:

1. **Root [`CLAUDE.md`](../../CLAUDE.md)** — project-wide invariants,
   session protocol, voice guide. Read first in every session.
2. **Per-crate [`crates/*/CLAUDE.md`](../../crates/)** — crate-specific
   responsibilities, invariants, and next-work pointers. Read when
   you open a crate.
3. **[`.claude/`](../../.claude/)** — slash commands and subagents
   that encapsulate repeatable operations.

## The session protocol

Every session follows the same shape, documented in
[CLAUDE.md](../../CLAUDE.md#the-session-protocol):

1. Orient: read CLAUDE.md, STATUS.md, TASKS.md.
2. Pick: highest-priority open task you have context for.
3. Work in the crate: read its CLAUDE.md first.
4. Verify: `./scripts/check-all.sh` passes.
5. Update state: STATUS.md and TASKS.md.
6. Commit.

Step 5 is the one that prevents drift across sessions. Do not skip it.

## Slash commands

The `.claude/commands/` directory contains parameterised commands:

- **`/next-task`** — read TASKS.md, pick the highest-priority open
  task with context, propose a plan, and stop for approval.
- **`/scaffold-crate <name>`** — create a new workspace crate with the
  house scaffold. Requires an ADR first.
- **`/audit-licences`** — run `cargo deny check licenses` and report.
- **`/spec-check`** — compare a proposed change against the specs and
  report any conflict before it reaches PR.

These are designed so the command itself enforces the right
discipline — for example, `/scaffold-crate` refuses to proceed
without an ADR.

## Subagents

`.claude/agents/` holds two specialised subagents:

- **`licence-auditor`** — use before any dep change. Reads deny.toml,
  runs cargo-deny, reports.
- **`crate-stubber`** — use when moving a crate from scaffold to
  alpha. Writes types, traits, and errors without writing business
  logic.

## Working across multiple sessions

The state files (`STATUS.md`, `TASKS.md`, per-crate `CLAUDE.md`) are
the handoff between sessions. If session N cleanly ends with state
updated, session N+1 picks up with full context.

The anti-pattern is a session that does work but forgets to record
it. When you return, the next Claude will re-do the same exploration,
possibly disagreeing with the previous one.

## Escalations

See [CLAUDE.md §Escalation](../../CLAUDE.md). The short list:

- New non-allow-listed dependency.
- `unsafe` block.
- Second binary proposal.
- Edits to `specs/` or `LICENSE`.
- Spec/ADR conflict without resolution.

When any of these arise, **stop and ask**. Short, explicit escalation
beats long, confident drift.
