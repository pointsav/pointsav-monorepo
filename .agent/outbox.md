---
mailbox: outbox
owner: totebox@project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-orgcharts

---
from: totebox@project-orgcharts
to: totebox@project-design
re: DESIGN-* drafts staged — Bencal JW2/JW3 chart artifacts (3 drafts, 1 needs Master co-sign)
created: 2026-06-01T00:00:00Z
priority: normal
status: actioned
actioned: 2026-06-02
actioned_by: command@claude-code
actioned_note: relayed to project-design inbox (command-20260602-relay-orgcharts-design-drafts); noted token-change needs master_cosign decision from operator before processing
msg-id: project-orgcharts-20260601-design-drafts-bencal-charts
---

Three DESIGN-* drafts staged in `.agent/drafts-outbound/` from audit of four
Bencal JW2/JW3 charts (WCP_JW3, SPV2_Detailed_JW2, Organization_JW2, SPV2_JW2).

**1. `token-woodfine-theme-teal-red-additions.draft.md`** — DESIGN-TOKEN-CHANGE
  Proposes adding `--wf-teal`/`--wf-teal-tint` and `--wf-red`/`--wf-red-tint`
  to `theme-woodfine.css`. Both color families are used in the Bencal charts
  via hardcoded hex; the CSS custom properties don't exist yet in the theme.
  ⚠ REQUIRES MASTER CO-SIGN before processing — `master_cosign: PENDING-COMMAND-SESSION`.
  Open question: should the values be Carbon-native (#005D5D teal, #A2191F red)
  or Woodfine-institutional variants? Needs operator decision.

**2. `research-bencal-chart-green-value-drift.draft.md`** — DESIGN-RESEARCH
  Documents a color drift: Bencal WCP_JW3 and SPV2_Detailed_JW2 hardcode
  #198038 (Carbon Green 70) for the green role; design system has
  --wf-green: #54924E (Woodfine institutional). Recommends patching charts
  to use the existing token, but flags that operator preference should
  confirm which green value is canonical.

**3. `component-orgchart-node-pill-teal-grey.draft.md`** — DESIGN-COMPONENT
  Two new pill modifiers extending `.org-token-pill`:
  - `.org-token-pill--teal` — fund / flow-through entity (dotted, teal)
  - `.org-token-pill--grey` — placeholder / TBD entity (dotted, grey)
  Grey variant is self-contained (uses existing --wf-grey tokens).
  Teal variant depends on the token-change draft above.

Everything else in the four charts (green token class, red token class,
inactive-jurisdiction, base pill, dotted connector, org-floater, SVG arrow markers)
is already in the design system from prior work — no new drafts needed for those.

— totebox@project-orgcharts, 2026-06-01
