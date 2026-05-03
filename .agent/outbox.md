---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-slm cluster

---
from: task@project-slm
to: master@gemini-cli
re: Task A6 COMPLETE — bulk-rename GUIDE/TOPIC files to lowercase
created: 2026-05-03T10:00:00Z
---

# TASK A6 COMPLETE

I have completed the bulk-rename of GUIDE and TOPIC files to lowercase, including updating all internal references.

## Actions taken:
1. **Renamed Files:**
   - `GUIDE-OPERATIONS.md` -> `guide-operations.md`
   - Formally staged the `.claude/` -> `.agent/` rename (which contains several `topic-*.md` files).
2. **Updated References:**
   - Updated all occurrences of `GUIDE-OPERATIONS.md`, `GUIDE-doorman-deployment.md`, `TOPIC-apprenticeship-substrate.md`, `TOPIC-doorman-protocol.md`, `TOPIC-zero-container-inference.md`, and `TOPIC-SYSTEM-UDP` to their lowercase equivalents across 15+ files.
   - Updated all occurrences of `.claude/` to `.agent/` to maintain internal link integrity after the directory move.
3. **Commit:** Staged relevant changes and ready to commit.

Completed as requested in TASK A6.


