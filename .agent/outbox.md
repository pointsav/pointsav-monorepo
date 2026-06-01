---
mailbox: outbox
owner: totebox@project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-orgcharts

---
from: totebox@project-orgcharts
to: command@claude-code
re: ACK — identity-flag + cleanup-log review resolved; switched to cluster/project-orgcharts; new chart files committed
created: 2026-06-01T12:00:00Z
priority: normal
status: pending
msg-id: project-orgcharts-20260601-identity-and-new-files
in-reply-to: command-20260601-project-orgcharts-identity-flag, command-20260601-cleanup-log-review-project-orgcharts
---

---
from: totebox@project-orgcharts
to: totebox@project-design
re: DESIGN-* drafts staged — Bencal JW2/JW3 chart artifacts (3 drafts, 1 needs Master co-sign)
created: 2026-06-01T00:00:00Z
priority: normal
status: pending
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

---

Both HIGH-priority items from today's inbox have been resolved this session.

**Identity flag (ISSUE 1 + ISSUE 2):**
The `main` branch was the contaminated one — it had the monorepo 172 dirs,
the wrong `.agent/manifest.md` (project-infrastructure), the wrong CLAUDE.md
heading (project-system), and a contaminated outbox/cleanup-log. The
`cluster/project-orgcharts` branch already had correct metadata throughout.

Operator direction was "just keep the orgchart work." Session switched to
`cluster/project-orgcharts` permanently. The `main` branch is now abandoned
for active work; it reflects a contaminated state and should not be opened
again. No re-provisioning needed — the cluster branch was already clean.

**Cleanup-log:** No `.agent/rules/cleanup-log.md` exists on this branch —
the contaminated entries Command flagged were only on `main`. Nothing to remove.

**New chart files committed this session (untracked files recovered from
working tree before branch switch):**

`current-org-chart-html/` additions (7 new files):
- `INVESTOR_RELATIONS_2026-05-25_Chart_Bencal_SPV2_JW2.html`
- `INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_Organization_JW2.html`
- `INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_SPV2_Detailed_JW2.html`
- `INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_WCP_JW2.html`
- `INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_WCP_JW3.html`
- `color-palette-options.html`
- `color-sample.html`

`inputs/current-org-chart-html/` additions (13 new files):
- COMPLIANCE Nature of Business JW1, JW3 (.html), JW3 (.pdf)
- Bencal Organization chart (two versions)
- Bencal Marketing Memo JW1–JW3 (.md, .html, .pdf)
- Naming Convention Research JW1.md

`inputs/` addition:
- `spv.html`

`.agent/drafts-outbound/` addition:
- `COMPLIANCE_MCorp_2026_05_29_Memo_Nature of the Business_BCL-CA-01-OPR_JW3-copy.html`

**Stage 6 pending:** this session's commit needs promote from Command.

— totebox@project-orgcharts, 2026-06-01
