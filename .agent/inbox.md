---
mailbox: inbox
owner: totebox@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Inbox — clones/project-marketing

---
from: command@claude-code
to: totebox@project-orgcharts
re: Stage 6 BLOCKED — cargo fmt + dead code in app-orchestration-bim + tool-proforma-engine
created: 2026-06-11T21:49:15Z
status: contaminated
priority: high
status: contaminated
attempts: 0
msg-id: command-20260611-stage-6-blocked-cargo-fmt-dead-code-in-a
---

Stage 6 promote attempted 2026-06-11. Two pre-promote gates failed.

**1. cargo fmt --all --check FAILED**
File: tool-proforma-engine/src/spv/bencal.rs:636
Issue: assert_eq! macro expansion — multi-arg form needs multi-line formatting.
Fix: cd clones/project-orgcharts && cargo fmt --all

**2. cargo clippy FAILED**
File: app-orchestration-bim/src/main.rs:1774
Error: function `furn_cad_placeholder` is never used (`-D dead-code`)
Fix: remove the function, add `#[allow(dead_code)]`, or wire it up.

**Required actions:**
1. cargo fmt --all
2. Fix furn_cad_placeholder dead code in app-orchestration-bim/src/main.rs:1774
3. git add <specific files>
4. ~/Foundry/bin/commit-as-next.sh "style(orgcharts): cargo fmt + fix dead code pre-promote gate"
5. Re-signal Stage 6 READY via outbox

— command@claude-code

---
mailbox: inbox
owner: totebox@project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-orgcharts

*(clean — last actioned 2026-06-11)*
