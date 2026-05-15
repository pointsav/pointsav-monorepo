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
re: binary rebuild request — Sprint L Wikipedia parity features
created: 2026-05-15T02:00:00Z
priority: normal
---

Sprint L shipped on cluster branch (`cluster/project-knowledge` / monorepo sub-clone is on `main`
commit `78b5d890`). Changes cover five new features — requires binary rebuild and service restart.

**What changed (app-mediakit-knowledge):**

1. **Appearance menu** — "Aa" button in main header opens Day/Night/OS theme toggle + Width
   (Standard/Wide). Dark theme via CSS custom properties; anti-FOUT inline script in `<head>`.
2. **More actions dropdown** — caret after "View history" opens: Print/Export, Page information,
   Cite this page, Download as Markdown.
3. **Sticky header action tabs** — sticky header now repeats Read/Edit (or View source)/View history
   tabs with accesskeys, matching Wikipedia parity.
4. **Special:Categories** — new `/special/categories` handler with alphabetical group headers and
   per-category article counts. Added to left nav and mobile drawer.
5. **Printable mode** — `?printable=yes` adds `body.printable` class that hides all chrome; `@media
   print` rules also provided.

**Build and restart commands:**

```
cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo cp /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/target/release/app-mediakit-knowledge /usr/local/bin/
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

All 162 tests pass. `cargo check` clean.
Stage 6 promotion to canonical `pointsav/pointsav-monorepo` main is also pending.

