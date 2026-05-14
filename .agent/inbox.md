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

