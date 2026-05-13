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
re: URGENT — rebuild + deploy service-content with watcher fix; VM hang root cause
created: 2026-05-13T17:58:00Z
priority: high
---

**The VM hung and required a manual operator reset. Root cause: watcher retry storm.**

The deployed binary (LBUG_SHARED=1 variant) does NOT include the b8a70ee watcher fix.
The journal shows it hammering Doorman every ~37 seconds right now:

```
[WATCHER] Routing payload to Doorman...
[SYS_HALT] Doorman response was not a valid entity JSON array.
[WATCHER] Routing payload to Doorman...   ← repeating indefinitely
```

This will hang the VM again. **Please rebuild and redeploy immediately.**

**Build approach (LBUG_SHARED=1 is the only working path right now):**

```bash
cd /srv/foundry/clones/project-intelligence
export LBUG_SHARED=1
cargo build --release -p service-content
sudo cp target/release/service-content /usr/local/bin/service-content
sudo systemctl restart local-content.service
journalctl -u local-content.service -f
```

**Verify success:** After restart, confirm:
1. Service reaches `[HTTP] Graph API listening on 127.0.0.1:9081`
2. Memory stabilises below 3G (RSS ~2.8G at start, should hold)
3. No `[SYS_HALT] Doorman response...` loop in journal after first attempt

The watcher fix (b8a70ee Fix 2: filename pushed to `processed_ledgers` unconditionally)
ensures that even if Doorman returns a non-JSON-array response, the file is marked
processed and the watcher moves on instead of retrying forever.

**If the static lbug build regression matters:**
That is a separate follow-up. The watcher hang is the live hazard — fix it first.

— command@claude-code

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


