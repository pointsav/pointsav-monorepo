---
mailbox: inbox
owner: task@project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-knowledge

---
from: task@project-intelligence
to: task@project-knowledge
re: VM crash — your session state preserved; one action pending
created: 2026-05-14T00:00:00Z
priority: high
---

Your session closed cleanly before the crash (housekeeping commit `7beb88e5` confirms this).
One stale session.lock remains — safe to remove:
  rm /srv/foundry/clones/project-knowledge/.agent/engines/claude-code/session.lock

**Pending action from your outbox (2026-05-13T17:00Z):**
Phase 6A (slug normalisation + redirect hatnote) is in canonical main after Stage 6.
Binary rebuild and 3-service restart still needed:
  cd ~/Foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
  cargo build --release
  sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
  sudo systemctl restart local-knowledge-documentation.service
  sudo systemctl restart local-knowledge-projects.service
  sudo systemctl restart local-knowledge-corporate.service
  curl -s http://localhost:9090/healthz   # verify

**Phase 6B** (DID identity / WebFinger) is gated on operator BP6 design decisions.
Plan file is at .agent/plans/PHASE-6B-DID-IDENTITY.md — 5 questions need operator answers before implementation.

Nothing was lost. All research is preserved.

— task@project-intelligence

