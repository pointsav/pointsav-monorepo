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
re: three more TOPIC draft pairs ready for pickup — genesis-protocol, ppn-command-protocol, service-pointsav-link
created: 2026-05-20
priority: normal
status: pending
---

Six drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive
(commit `94290124`):

- `topic-genesis-protocol.draft.md` — English
- `topic-genesis-protocol.es.draft.md` — Spanish
- `topic-ppn-command-protocol.draft.md` — English
- `topic-ppn-command-protocol.es.draft.md` — Spanish
- `topic-service-pointsav-link.draft.md` — English
- `topic-service-pointsav-link.es.draft.md` — Spanish

**Targets:**
- `content-wiki-documentation/architecture/genesis-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/ppn-command-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/service-pointsav-link.md` (+ `.es.md`)

**What each covers:**

**genesis-protocol** — The fleet-bootstrapping sequence for `os-infrastructure` first boot.
Covers: the sequencing-dependency problem; five steps (blind boot, scan, genesis fork,
holding pattern, claim); deferred fleet assembly; relationship to machine-based-auth.
One open question in research trail: EAPOL vs Genesis Protocol implementation state
(topic describes intended architecture; no correction needed).

**ppn-command-protocol** — The 16-byte binary wire format broadcast over UDP port 8090.
Covers: design constraints (no broker, no plaintext, no verbosity); packet format
(2-byte opcode + 14-byte payload); 4-step dispatch sequence; why simultaneous broadcast;
relationship to the Diode Standard. No open questions.

**service-pointsav-link** — The hot-pluggable `pointsav-protocol` adapter.
Covers: four properties (default not installed, hot-plug activation, clean severance,
policy in adapter not kernel); default state invariant; activation sequence; failure mode;
Universal Standard (same package across all os-* pairs). No open questions.

**Note for editor:** All six drafts carry `research_inline: true` with full research
trails. The genesis-protocol drafts carry one noted open question (EAPOL vs intended
architecture) that requires no correction — the topic correctly describes intended
architecture. Product names (Genesis Protocol, Diode Standard, WireGuard, Noise Protocol,
WebSocket, service-pointsav-link, pointsav-protocol, os-infrastructure, os-network-admin,
service-slm, service-udp) are not translated in the Spanish drafts.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

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


