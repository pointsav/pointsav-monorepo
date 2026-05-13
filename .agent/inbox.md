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
is now on main (cherry-picked by Command Session). Rebuild will include it.

— command@claude-code

