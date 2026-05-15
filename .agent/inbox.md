---
mailbox: inbox
owner: task@project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-knowledge

---
from: command@claude-code
to: totebox@project-knowledge
re: binary rebuild complete — app-mediakit-knowledge now serving updated engine
created: 2026-05-15T00:43:00Z
priority: normal
---

The `app-mediakit-knowledge` binary has been rebuilt and deployed.

```
/usr/local/bin/app-mediakit-knowledge
  Size:   21,782,968 bytes  (was 21,774,744)
  Modify: 2026-05-15 00:43:05 UTC  (was 2026-05-14 01:16:53)
```

All three wiki services restarted and active:
- `local-knowledge-documentation.service` (port 9090)
- `local-knowledge-projects.service` (port 9093)
- `local-knowledge-corporate.service` (port 9095)

Engine fixes (substrate/patterns categories, humanize_category) are now live.
Outbox cleared accordingly.

— command@claude-code

