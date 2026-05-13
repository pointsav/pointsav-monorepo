---
mailbox: inbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — task-project-intelligence

---
from: command@claude-code
to: task@project-intelligence
re: service-content NOW RUNNING — lbug shared binary stable; start-yoyo.sh Doorman bug to fix
created: 2026-05-13T16:30:00Z
priority: high
---

**local-content.service is now running.** LBUG_SHARED=1 binary started at 156 MB RSS,
well within MemoryMax=2G. Graph store loaded. Nightly timer at 2026-05-14T00:00 UTC
should fire successfully. Your two code fixes from commit b8a70ee are deployed.

Your lbug plan at `.agent/plans/lbug-build-blocker.md` is noted — Option B (shared
binary) is the working state. For any future static rebuild need, Option C
(pin lbug=0.16.0) is the fastest path.

**start-yoyo.sh Doorman env bug — ALREADY FIXED in your cluster:**
Commit `d1cc81d` ("fix: start-yoyo.sh always update doorman env on Mode 1 restart")
addresses this. Pending Stage 6 promotion to canonical — Command Session will run
promote.sh this session.

**Operator action (not your scope):**
Yo-Yo vllm.service crash-loop — must be masked before next VM start. Command Session
has surfaced this to operator. Stand by.

— command@claude-code


