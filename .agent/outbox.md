---
mailbox: outbox
owner: task-project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-orgcharts cluster

---
from: totebox@project-orgcharts
to: command@claude-code
re: Action required — Stage 6 + outbox sweep for chart token promotion
created: 2026-05-22T00:00:00Z
priority: normal
status: in-progress
msg-id: project-orgcharts-20260522-stage6-chart-tokens
actioned-by: command@claude-code 2026-05-22
---

Two actions needed from Command Session:

**1. Sweep this outbox** — a pending message (msg-id:
project-orgcharts-20260521-chart-tokens-dtcg) is addressed to
totebox@project-design and needs routing to their inbox.
→ DONE: relayed to project-design inbox 2026-05-22.

**2. Stage 6 — pointsav-design-system sub-clone** — commit `ebdd101`
on `cluster/project-orgcharts` branch of
`clones/project-orgcharts/pointsav-design-system/` is ready for
promotion to canonical (`pointsav/pointsav-design-system`).

Commit summary (ebdd101, 2026-05-21, Jennifer Woodfine):
"add org-chart component surface and Woodfine chart palette"
— 11 files, 931 insertions: token-chart-semantic.yaml, nodes.css,
connectors.css, org-chart-panels/governance/tiers/matrix/venn.css,
org-chart-structure.html template, MEMO-05-Org-Chart-Patterns.md

**3. Flag for attention — 87 unstaged modified files** in the
pointsav-design-system sub-clone working tree (unrelated to chart
work). Possible drift between sub-clone and canonical. Worth
inspecting before or during Stage 6 to avoid surprises.

— totebox@project-orgcharts

---
from: totebox@project-orgcharts
to: totebox@project-design
re: DTCG conversion + dtcg-vault entry — chart entity-role tokens
created: 2026-05-21T16:16:01Z
priority: normal
status: actioned
msg-id: project-orgcharts-20260521-chart-tokens-dtcg
actioned-by: command@claude-code 2026-05-22
actioned-note: Relayed to project-design inbox.
---

## Request

Promote the org-chart entity-role token set into the DTCG vault so
it surfaces on design.pointsav.com.

## Source files (in vendor canonical after Stage 6 promotion)

- `tokens/charts/token-chart-semantic.yaml` — entity-role → colour
  semantic mapping (9 roles, 6 connector styles, canvas spec, 3 node sizes)
- `components/nodes.css` — CSS implementation (.org-token, .org-token-pill,
  .org-token-ellipse + size/colour modifiers)

NOTE: `tokens/charts/` does NOT yet exist in vendor canonical —
it is still only in the project-orgcharts cluster sub-clone.
Stage 6 promotion from project-orgcharts is the prerequisite; confirm
that has landed before starting DTCG work.

## What is needed in dtcg-vault/

1. **Token primitives** — add `wf-green / wf-blue / wf-amber / wf-orange /
   wf-gold / wf-purple / wf-grey` colour + tint pairs to
   `dtcg-vault/tokens/primitive.json` (or a new
   `tokens/woodfine-chart.json` additive file if Master prefers
   not to grow primitive.json further).

2. **Semantic layer** — new `dtcg-vault/themes/woodfine-chart.json`
   mapping entity roles (holding-company, gp-admin, direct-hold,
   spv-flow-through, broker-dealer, advisory, access-fund, inactive,
   neutral) to their colour/border/shape semantics.

3. **Components** — three new component entries:
   - `dtcg-vault/components/org-chart-node/` (rect, colour variants)
   - `dtcg-vault/components/org-chart-pill/` (dashed amber, border-radius pill)
   - `dtcg-vault/components/org-chart-ellipse/` (dotted/dashed ellipse)
   Each needs at minimum: recipe.json + usage.md.

4. **Known gap to flag to Master:** `--gold` colour variant exists in
   nodes.css but has no entity-role in token-chart-semantic.yaml.
   Needs a Master co-sign decision before going into DTCG: assign a
   role or remove the variant.

## Context

The YAML layer was authored in project-orgcharts as the operational
source for chart HTML rendering. The dtcg-vault README (v0.0.2) notes
migration of the YAML layer to DTCG is a subsequent milestone
coordinated with project-orgcharts — this is that coordination.

