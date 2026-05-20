---
mailbox: outbox
owner: task-project-infrastructure
location: ~/Foundry/clones/project-infrastructure/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-infrastructure cluster

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: sovereign-mesh TOPIC drafts ready for pickup
created: 2026-05-20
priority: normal
status: pending
---

Two drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive:

- `topic-sovereign-mesh.draft.md` — English
- `topic-sovereign-mesh.es.draft.md` — Spanish

**Target:** `content-wiki-documentation/infrastructure/sovereign-mesh.md` (+ `.es.md` pair)

**What it does:** Expands the existing one-sentence stub to a full PPN architecture topic.
Covers: hub-spoke topology, WireGuard overlay, `ppn0` interface, 16-byte binary command
protocol on port 8090, three node roles, Genesis Protocol integration, Diode Standard
relationship, see-also links.

**Note for editor:** Two open questions flagged in the research trail (see `notes_for_editor`
field and the `## Research trail / Open questions` section in both drafts):
1. Canonical PPN subnet — uses `10.50.0.0/24` / `.1/.2/.3` with planned language pending
   operator ratification.
2. Genesis Protocol implementation state — topic describes intended architecture per TOPICs;
   code is currently a prototype (EAPOL monitor-mode). No correction needed in the topic
   itself — the intended architecture is what the topic should describe.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`


