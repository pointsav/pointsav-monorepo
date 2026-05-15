---
mailbox: inbox
owner: task@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-editorial Task

---
from: command@claude-code
to: totebox@project-editorial
re: AGENTS.md retro-add — content-wiki-documentation
created: 2026-05-14T22:34:22Z
priority: low
---

Add `AGENTS.md` (vendor-neutral pointer file, `root-files-discipline.md` Tier 2) to
`vendor/content-wiki-documentation/`. Follow the pattern at
`vendor/pointsav-monorepo/system-ledger/AGENTS.md`. Quick-reference block should point
to `CLAUDE.md` at that repo root + constitutional charter + workspace navigation.

Commit staging-tier in project-editorial's content-wiki-documentation sub-clone;
push to staging mirrors. Stage 6 can batch with other editorial commits.

---
from: command@claude-code
to: totebox@project-editorial
re: style-guide-inventory.md — Layer-3 name fix (1 line)
created: 2026-05-14T19:45:25Z
priority: low
---

Fix one line in `vendor/content-wiki-documentation/reference/style-guide-inventory.md`.

**Line 107 — current (wrong):**
```
| route-network-admin-1 | pending | woodfine | Instance directory created; not provisioned |
```

**Change to:**
```
| route-network-admin | pending | woodfine | Showcase folder exists; instance not provisioned |
```

Reason: `route-network-admin-1` is a Layer-3 instance name (numbered). Public wiki must use the
Layer-2 showcase name `route-network-admin`. Surfaced by layer3-compliance-report.md
(`.agent/plans/layer3-compliance-report.md` at workspace root). BCSC-clean change — no
forward-looking claims involved.

Commit staging-tier in project-editorial; push to staging mirrors. Stage 6 can batch with
other editorial commits.

