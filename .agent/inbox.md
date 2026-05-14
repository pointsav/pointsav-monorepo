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
re: investigate Doorman routing returning invalid JSON during service-content startup scan
created: 2026-05-13T23:30:00Z
priority: normal
---

During the service-content startup scan on 2026-05-13 (~18:49–20:05 UTC), all 114 CORPUS_
files received `[SYS_HALT] Doorman response was not a valid entity JSON array` from
Doorman at `http://127.0.0.1:9080/v1/chat/completions`. The watcher retry storm fix
(b8a70ee / 3e8c8a4) is confirmed working — each file was attempted exactly once. However,
Doorman should either return a valid entity JSON array or return a structured error that
service-content can handle gracefully.

**Observed:** Doorman returned a non-JSON-array response for every extraction request
during a ~76-minute window. Yo-Yo (Tier B) was stopped. Tier A (local OLMo 7B) was the
only available backend.

**Questions to investigate:**
1. What does Doorman actually return when Tier A cannot process a DataGraph extraction
   request? Is it returning an error object `{"error": "..."}` or an empty response or
   something else — all of which would fail the `is_array()` check in service-content?
2. Should service-content distinguish between "Doorman returned an error" (retry later)
   vs "Doorman returned malformed data" (skip permanently)?
3. Should Doorman have a dedicated extraction endpoint that always returns a valid array
   (possibly empty) rather than routing through `/v1/chat/completions` which is designed
   for chat completions, not structured entity extraction?

**Context:** service-content calls Doorman at `/v1/chat/completions` and expects the
response to be a JSON array of entity objects. This is mixing a chat endpoint with a
structured-data contract — the mismatch may be the root cause.

**Not blocking** — service-content is stable and the watcher fix prevents VM hangs.
But the 114-file scan produced zero successful extractions, which means the DataGraph
was not updated from any of the corpus files during this boot.

— command@claude-code

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

