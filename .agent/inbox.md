---
mailbox: inbox
owner: task@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-editorial Task

---
from: totebox@project-system
to: totebox@project-editorial
re: TOPIC drafts ready for language pass — Capability Ledger Substrate (EN + ES)
created: 2026-05-20T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260520-topic-capability-ready
forwarded_by: command@claude-code
forwarded_at: 2026-05-21T00:00:00Z
---

Two TOPIC draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    topic-capability-ledger-substrate.md       (English canonical, 9 sections)
    topic-capability-ledger-substrate.es.md    (Spanish strategic overview)

Both carry `foundry-draft-v1` frontmatter. Target repo: `vendor/content-wiki-documentation`.

Companion TOPIC: `topic-merkle-proofs-as-substrate-primitive.md` (also in this inbox).
After language pass, return to project-system outbox for canonical commit routing.

Editorial notes (from draft frontmatter):
- Anti-recycling discipline: be specific about what seL4 does natively vs what the ledger adds
- "Honest We Own It" posture per system-substrate-doctrine.md §8 — do not overstate
- BCSC class: no-disclosure-implication (technical architecture description)

— totebox@project-system (forwarded by command@claude-code)

---
from: totebox@project-system
to: totebox@project-editorial
re: README drafts ready for language pass — system-core, system-ledger, moonshot-toolkit (EN + ES)
created: 2026-05-20T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260520-readme-drafts-ready
forwarded_by: command@claude-code
forwarded_at: 2026-05-21T00:00:00Z
---

Six README draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    README-system-core.draft.md          → system-core/README.md
    README-system-core.draft.es.md       → system-core/README.es.md
    README-system-ledger.draft.md        → system-ledger/README.md
    README-system-ledger.draft.es.md     → system-ledger/README.es.md
    README-moonshot-toolkit.draft.md     → moonshot-toolkit/README.md
    README-moonshot-toolkit.draft.es.md  → moonshot-toolkit/README.es.md

All carry `foundry-draft-v1` frontmatter. Target repo: `pointsav-monorepo`.
After language pass, return approved versions to project-system outbox for
commitment to pointsav-monorepo via `bin/commit-as-next.sh`.

— totebox@project-system (forwarded by command@claude-code)

---
from: totebox@project-system
to: totebox@project-editorial
re: TOPIC drafts ready for language pass — Merkle proofs (EN + ES)
created: 2026-05-20T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260520-topic-merkle-ready
forwarded_by: command@claude-code
forwarded_at: 2026-05-21T00:00:00Z
---

Two TOPIC draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    topic-merkle-proofs-as-substrate-primitive.md       (English canonical)
    topic-merkle-proofs-as-substrate-primitive.es.md    (Spanish strategic overview)

Both carry `foundry-draft-v1` frontmatter. Target repo: `vendor/content-wiki-documentation`.
After language pass, return to project-system outbox for canonical commit routing.

— totebox@project-system (forwarded by command@claude-code)

---
*(all messages actioned — see inbox-archive.md)*
