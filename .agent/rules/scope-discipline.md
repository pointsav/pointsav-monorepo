---
name: scope-discipline
description: Items in this archive NEXT.md must be actionable from a Totebox session. Command-scope and cross-archive tasks route via outbox.
metadata:
  type: project
---

# Scope Discipline

## What belongs in this NEXT.md

Only items that can be completed by a Totebox Session starting in this archive's CWD.

In scope:
- Feature work, bug fixes, and tests in this archive's crates or files
- Editorial drafts staged to `.agent/drafts-outbound/`
- Inbox messages to action
- BRIEF updates for in-progress work

Out of scope — route to outbox instead:

| If the item requires... | Route to |
|---|---|
| `bin/promote.sh` or Stage 6 | Command Session outbox |
| `bin/sync-local.sh --all` | Command Session outbox |
| Any `bin/` workspace script | Command Session outbox |
| Work in another `clones/project-*` archive | That archive's outbox |
| VM sysadmin, systemd, deploy | Command Session outbox |
| Canonical merge (`vendor/` or `customer/` write) | Command Session outbox |

## How to route out-of-scope items

Post an outbox message per the mailbox protocol (`AGENT.md §Mailbox protocol`).
Prefer `send_mailbox_message()` MCP tool over hand-editing frontmatter.

Once an outbox message is sent, remove the item from NEXT.md.
NEXT.md is a work queue for this session, not a tracking system for all pending work.

## Detecting violations

Signs that a NEXT.md item is out of scope:
- It mentions `promote.sh`, `sync-local.sh`, `bin/`, or another `project-*` archive
- It requires a git push to `vendor/` or `customer/`
- It cannot be completed by commands running under this archive's CWD
