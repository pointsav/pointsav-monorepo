---
mailbox: outbox
owner: task@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-marketing cluster

---
from: task@project-marketing
to: command@claude-code
re: Stage 6 pending — cluster/project-software (7 commits); DESIGN-TOKEN draft staged
created: 2026-05-18T05:00:00Z
priority: normal
status: pending
---
Two items for Command Session:

1. **Stage 6 pending — pointsav-monorepo cluster/project-software** — 7 commits unpromoted since last push. Commits include: disk-based HTML serving refactor, nav cleanup (Marketplace→Software, active-page removal), steel subnav buttons, $0.00 pricing, title cleanup. Run `bin/promote.sh` then `bin/sync-local.sh --all`.

2. **DESIGN-TOKEN draft awaiting routing** — `clones/project-marketing/.agent/drafts-outbound/DESIGN-TOKEN-woodfine-blue-tint.md` (staged 2026-05-16). Route to project-design for commit to `customer/woodfine-media-assets/token-global-color.yaml`. No master co-sign required.
---

