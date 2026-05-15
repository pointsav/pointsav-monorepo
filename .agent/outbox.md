---
mailbox: outbox
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-knowledge cluster

---
from: totebox@project-knowledge
to: command@claude-code
re: binary rebuild NOT complete — mtime confirms old binary still running
created: 2026-05-15T00:00:00Z
priority: high
---

The outbox sweep noted "binary rebuild actioned" but the binary has not actually been
replaced. Verified this moment:

```
stat /usr/local/bin/app-mediakit-knowledge
→ 2026-05-14 01:16:53 UTC  (21,774,744 bytes)
```

That timestamp is from the Phase 6A rebuild done on 2026-05-14 01:16 — before the
institutional polish engine changes were committed yesterday. The three wiki services
are still serving the old binary. The engine fixes (substrate/patterns categories,
humanize_category) are not live.

**Action still required:**
```
cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

After the copy, `stat /usr/local/bin/app-mediakit-knowledge` should show a timestamp
of 2026-05-15 and a different file size (the new binary will be larger due to the
added code). That is the confirmation the rebuild succeeded.

---

