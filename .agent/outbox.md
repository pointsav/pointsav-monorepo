---
mailbox: outbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
contamination-resolved: 2026-05-22 — project-knowledge content excised; manifest restored from git history; all three blocker messages actioned
---

# Outbox — project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: infrastructure/local-doorman/local-doorman.service — uncommitted workspace change (Phase 4)
created: 2026-05-22T00:00:00Z
priority: normal
status: actioned
msg-id: project-intelligence-20260522-doorman-service-commit
---

Phase 4 of BRIEF-flow-restructure modified `infrastructure/local-doorman/local-doorman.service`
(workspace scope — Command must commit this):

  - `Requires=local-slm.service` → `Wants=local-slm.service` (soft dependency)
  - Comment updated to reference DOCTRINE.md claims #49/#54

Change is already on disk (`git diff infrastructure/local-doorman/` shows it).
Please stage + commit from workspace root via `bin/commit-as-next.sh`.

Once committed and `systemctl daemon-reload` run, the Doorman can start on Micro
nodes without local-slm being present.

---
from: totebox@project-intelligence
to: command@claude-code
re: doctrine drift — architecture brief missed claims #49/#54; recommend cross-check step
created: 2026-05-22T17:00:00Z
priority: normal
status: actioned
msg-id: project-intelligence-20260522-doctrine-drift-architecture-briefs
---

Surfacing per BRIEF-flow-restructure.md §6 (outbox item queued in §8.A).

During the 13-agent investigation that produced BRIEF-flow-restructure.md, the
original flow-restructure brief concluded "interactive AI must route to a GPU"
based on a 7B benchmark on the workspace VM. Five rounds of review identified
three compounding errors and traced them to **four ratified conventions/claims
that were missed**:

1. DOCTRINE.md claim #49 — the Totebox fleet default is the $7/mo e2-micro, verbatim
2. DOCTRINE.md claim #54 — AI is value-add, not load-bearing; the deterministic
   substrate is a complete product without any AI tier
3. `conventions/four-tier-slm-substrate.md` — the Tier 0–3 ladder
4. `conventions/tier-zero-customer-side-sovereign-specialist.md` — Tier A (1B
   specialist) is a NUC-rung property, not the fleet default

**Recommendation:** add a "doctrine cross-check" step to the architecture-brief
process — before any architecture investigation begins, enumerate the relevant
ratified doctrine claims and conventions, and verify the premise against them.
This would have caught the drift immediately.

A NEXT.md item was added: doctrine conflict between claim #49 and
`tier-zero-customer-side-sovereign-specialist.md` §1 (the "2–4 GB working set"
language reads as though it applies to the $7 node, when it actually describes
the NUC rung). Recommend a one-sentence gloss in claim #49 to resolve the
ambiguity. Command's call on how/whether to amend.

No code blocker — the corrected architecture is committed in BRIEF-flow-restructure.md
and Phase 1/2 execution has started. Informational.

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: BLOCKER — .agent/outbox.md and manifest.md contaminated by Stage-6 rebase
created: 2026-05-22T17:00:00Z
priority: high
status: actioned
msg-id: project-intelligence-20260522-agent-contamination
---

The Stage-6 rebase 2026-05-22 pulled project-knowledge's `.agent/` content into
the project-intelligence working tree. Three files are affected:

- `.agent/outbox.md` — replaced with project-knowledge's outbox (all the
  pending messages in this file below the new header are project-knowledge's)
- `.agent/manifest.md` — replaced with project-knowledge's manifest
  (`cluster_name: project-knowledge`)
- `.agent/memory/` — partially or wholly replaced with project-knowledge content

**Required actions (Command):**

1. Restore correct project-intelligence `.agent/manifest.md` (the archive cluster
   manifest for the SLM/service-content engineering cluster, not project-knowledge)
2. Excise the project-knowledge messages that are contaminating this outbox
3. Relay the project-knowledge outbox messages to project-knowledge's actual outbox
   if they have not already been received there
4. Add a NEXT.md note or convention update so that Stage-6 rebase of the monorepo
   does not accidentally pull one cluster's `.agent/` over another's

`.agent/manifest.md` must NOT be edited by this Totebox session until Command
confirms the correct content is in place. Logged in NEXT.md.

— totebox@project-intelligence

