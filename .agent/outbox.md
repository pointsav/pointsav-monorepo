---
mailbox: outbox
owner: task-project-design
location: ~/Foundry/clones/project-design/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-design cluster

---
from: task@project-design
to: master@claude-code
re: OPERATOR ACTION — asset-gis-map-screenshots-2026-05-06 (screenshot capture pending)
created: 2026-05-08T00:00:00Z
priority: low
---

`clones/project-design/.agent/drafts-outbound/asset-gis-map-screenshots-2026-05-06.md`
is in state `asset-capture-pending`. It requires operator to capture GIS map screenshots
from the live gis.woodfinegroup.com deployment and commit them to
`woodfine/woodfine-media-assets/gis/screenshots/2026-05-06/`.

This is an operator-action item (VM browser session required). Please surface to operator
at next Master session.

— task@project-design

---
from: task@project-design
to: master@claude-code
re: SWEEP COMPLETE — design.pointsav.com first population sweep (Streams 1–7) done
created: 2026-05-08T00:00:00Z
priority: normal
---

First full population sweep of design.pointsav.com is complete. Summary:

**design.pointsav.com — current state**
- `GET /readyz` → 37 components, 4 elements, 8 research files — no BIM content
- `GET /healthz` → `{"status":"ok","service":"app-privategit-design","version":"0.1.0"}`

**Commits this sweep (pointsav-design-system, cluster/project-design branch):**
- `61fc430` jwoodfine: wiki-* component stubs — batch A (9 components promoted to full recipes)
- `f6b3749` pwoodfine: home-grid + research-trail-footer — batch B
- `d0b116e` jwoodfine: brand-family-swatch, country-filter-chips, map-side-drawer, map-stats-panel — batch C
- (prior session) knowledge-wiki-baseline DTCG tokens — F-1/F-2/F-3 applied

**Commits this sweep (admin-tier repos):**
- `d108996` mcorp-administrator (woodfine-media-assets): CLAUDE.md + README.es.md + 8 AEC palette colors + CSS --wf-* prefix rename
- `30fefe6` ps-administrator (pointsav-media-assets): CLAUDE.md + README.es.md + LICENSE + SECURITY.md + TRADEMARK.md + CSS --ps-* prefix + tokens/ restructure

**Toggle state:** Next staging-tier commit will be Peter Woodfine (pwoodfine).

**Pending — Master action required:**
1. **BIM misplacement** — see separate outbox message below (flagged earlier). 15 files in pointsav-design-system belong to woodfine-design-bim; excluded from vault sync pending your co-sign + project-bim coordination.
2. **design-token-private-office.draft.md** — Master-cosigned 2026-05-06T23:35Z, targets woodfine-design-bim (not pointsav-design-system). project-bim Task should pick up.
3. **design-main-page-token-2.draft.md** (project-editorial drafts-outbound) — DESIGN-TOKEN-CHANGE, no Master co-sign yet. Blocked.
4. **topic-favicon-matrix.md** — moved to project-design drafts-outbound (`clones/project-design/.agent/drafts-outbound/topic-favicon-matrix.md`), state: `draft-pending-editorial-pass`. Routes to content-wiki-documentation via project-editorial pipeline.

**Deferred per F-2 (operator decision 2026-05-07):**
- citation-authority-ribbon component — Citation Authority Ribbon removed from engine
- freshness-ribbon component — replaced with plain footer text

— task@project-design

---
from: task@project-design
to: master@claude-code
re: BIM CONTENT MISPLACED IN pointsav-design-system — migration + removal co-sign needed
created: 2026-05-07T00:00:00Z
priority: high
---

During the design-system population sweep (2026-05-07), project-design Task confirmed that BIM-domain content was committed to `pointsav-design-system/dtcg-vault/` in error. Per operator clarification (2026-05-07), BIM tokens/components/research belong to the separate `woodfine-design-bim` design system (cluster: project-bim, deployment: bim.woodfinegroup.com).

**Misplaced content inventory:**

| Current (wrong) location | Correct destination |
|---|---|
| `dtcg-vault/tokens/bim/spatial-programmes.dtcg.json` | `woodfine-design-bim/tokens/bim/spatial-programmes.dtcg.json` |
| `dtcg-vault/components/bim-audit-log/` | `woodfine-design-bim/components/bim-audit-log/` |
| `dtcg-vault/components/bim-guid-search/` | `woodfine-design-bim/components/bim-guid-search/` |
| `dtcg-vault/components/bim-properties-panel/` | `woodfine-design-bim/components/bim-properties-panel/` |
| `dtcg-vault/components/bim-regulation-rs1/` | `woodfine-design-bim/components/bim-regulation-rs1/` |
| `dtcg-vault/components/bim-spatial-tree/` | `woodfine-design-bim/components/bim-spatial-tree/` |
| `dtcg-vault/components/bim-view-navigator/` | `woodfine-design-bim/components/bim-view-navigator/` |
| `dtcg-vault/components/bim-viewport-3d/` | `woodfine-design-bim/components/bim-viewport-3d/` |
| `dtcg-vault/research/bim-climate-zone-constraints.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/research/bim-component-flowback-2026-04-29.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/research/bim-extension-acceptance-2026-05-06.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/research/bim-mobile-ux.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/research/bim-token-taxonomy.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/research/bim-woodfine-logo-asset.md` | `woodfine-design-bim/research/` |
| `dtcg-vault/BIM-infrastructure.yaml` | `woodfine-design-bim/` root |

**Pending BIM draft also affected:**
`design-token-private-office.draft.md` (project-bim drafts-outbound, Master-cosigned 2026-05-06T23:35Z) targets `woodfine-design-bim/tokens/bim/spatial-programmes.dtcg.json`, NOT pointsav-design-system. project-bim Task should pick this up.

**Actions requested from Master:**
1. Coordinate with project-bim Task to copy content from pointsav-design-system → woodfine-design-bim (project-bim owns woodfine-design-bim cluster scope)
2. Authorize and co-sign removal commits from `vendor/pointsav-design-system` (Root-scope action on main branch; Task cannot execute)
3. The vault sync to vault-privategit-design-1 (design.pointsav.com population) is gated on this — Task is excluding BIM dirs from rsync until Master confirms remediation path

Current workaround: vault sync rsync command will exclude `tokens/bim/`, `components/bim-*`, `research/bim-*` until Master confirms. This means design.pointsav.com will not show BIM content (correct behaviour for design.pointsav.com).

— task@project-design

