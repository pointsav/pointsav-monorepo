---
schema: foundry-plan-v1
archive: project-orgcharts
created: 2026-05-20
status: active
---

# Org Chart Audit — DESIGN-* & ASSET-* Breakdown

Living reference for the org-chart → design-system workstream.
All 18 source charts are in `inputs/current-org-chart-html/`.

---

## Chart inventory by category

### Category 1 — Corporate Structure Charts (9)

Hierarchy trees showing entity ownership chains (parent → subsidiary → direct-hold solution).
Visual pattern: token-network with SVG path connectors and arrow markers.

| Filename | JW# | Date |
|---|---|---|
| INVESTOR_RELATIONS_MCorp_2026_01_06_Org_Chart_Management_Extended_JW7.html | JW7 | 2026-01-06 |
| INVESTOR_RELATIONS_2026-03-28_Chart_Woodfine-Group_JW9.html | JW9 | 2026-03-28 |
| INVESTOR_RELATIONS_2026-03-28_Chart_Canada_JW13.html | JW13 | 2026-03-28 |
| INVESTOR_RELATIONS_2026-03-28_Chart_Mexico_JW10.html | JW10 | 2026-03-28 |
| INVESTOR_RELATIONS_2026-03-28_Chart_Cross-Border_JW21.html | JW21 | 2026-03-28 |
| INVESTOR_RELATIONS_2026-03-28_Chart_Cross-Border-2_JW19.html | JW19 | 2026-03-28 |
| INVESTOR_RELATIONS_2026-04-04_Chart_Transaction-1_JW29.html | JW29 | 2026-04-04 |
| INVESTOR_RELATIONS_2026-04-04_Chart_Transaction-2_JW18.html | JW18 | 2026-04-04 |
| INVESTOR_RELATIONS_2026-04-04_Chart_Transaction-3_JW15.html | JW15 | 2026-04-04 |

### Category 2 — Fund & Flow Charts (2)

Complex multi-color token networks for investment vehicles, fee flows, and ICAV structures.
Visual pattern: 170×90px tokens, 6+ colors, panel-block sidebars, fee-legend glyphs.

| Filename | JW# | Date |
|---|---|---|
| INVESTOR RELATIONS_2026-04-06_Chart_Access Fund_Internal_JW9.html | JW9 | 2026-04-06 |
| INVESTOR RELATIONS_2026-04-06_Chart_Access Fund_External_JW11.html | JW11 | 2026-04-06 |

### Category 3 — Governance, Compliance & Analysis Charts (7)

Grid/table/diagram layouts. No token-network pattern.
Visual pattern: section-based grids, tier bars, comparison matrices, Venn diagrams.

| Filename | JW# | Date |
|---|---|---|
| INVESTOR RELATIONS_2026-04-06_Chart_Accouting Statements_JW9.html | JW9 | 2026-04-06 |
| INVESTOR RELATIONS_2026-04-06_Chart_Accouting Statements_JW10.html | JW10 | 2026-04-06 |
| INVESTOR RELATIONS_MCorp_2026-04-15_Chart_Accounting_JW14.html | JW14 | 2026-04-15 |
| INVESTOR RELATIONS_MCorp_2026-04-15_Chart_Counsel_JW14.html | JW14 | 2026-04-15 |
| COMPLIANCE_MCorp_2026_04_25_SPV Arrangements_Slide1_JW1.html | JW1 | 2026-04-25 |
| COMPLIANCE_MCorp_2026_04_25_SPV Arrangements_Slide2_JW1.html | JW1 | 2026-04-25 |
| CONSTRUCTION_MCorp_2026-04-25_Chart_Venn_Diagram_JW2.html | JW2 | 2026-04-25 |

---

## Drift inventory

Five divergences identified across the 18 charts. Old charts stay as historical artifacts — no back-migration.

| Drift point | Early charts (JW7–JW19, Cat. 1) | Later charts (JW14+, Cat. 3) | Decision (2026-05-20) |
|---|---|---|---|
| **Green** | `#2E7D32` Material Green-800 | `#54924E` Woodfine brand | → `woodfine-green: "#54924E"` |
| **Blue** | `#1565C0` Material Blue-800 | `#164679` Woodfine brand | → `woodfine-blue: "#164679"` (already canonical) |
| **CSS var naming** | Hardcoded hex | `--wf-*` or `--woodfine-*` | → `--woodfine-*` everywhere |
| **Token box size** | 210×110px | 170×90px | → Two named variants (see tokens) |
| **Border weight** | 2px solid | 0.75px solid | → 2px for token nodes; 0.75px for governance nodes |
| **Rendering** | Static HTML/CSS/SVG | React/Babel bundler (2 files) | → Static HTML only for all new charts |

---

## DESIGN-TOKEN artifacts

### DESIGN-TOKEN-WOODFINE-COLOR
File: `woodfine-media-assets/token-global-color.yaml`
Action: Add missing colors (woodfine-green, woodfine-orange, woodfine-gold, woodfine-purple, woodfine-amber + tints).
Status: **DONE 2026-05-20**

### DESIGN-TOKEN-CHART-SEMANTIC
File: `pointsav-design-system/tokens/charts/token-chart-semantic.yaml`
Action: Machine-readable entity-role → color mapping.
Status: **DONE 2026-05-20**

### DESIGN-TOKEN-CANVAS-PRINT
File: `pointsav-design-system/tokens/global/token-global-print.yaml`
Action: Verify 1056×816px landscape letter is the canonical print canvas token.
Status: pending — verify in next session

---

## DESIGN-COMPONENT artifacts

### Extend nodes.css — org-chart token node system
File: `pointsav-design-system/components/nodes.css`
Adds: `.org-token`, `.org-token--sm`, `.org-token--compact`, color variants, `.org-token-pill`, `.org-token-ellipse`
Status: **DONE 2026-05-20**

### Extend connectors.css — SVG arrow connector system
File: `pointsav-design-system/components/connectors.css`
Adds: SVG defs fragment, `.org-connector` color variants, `.org-connector--dashed`, `.inactive-jurisdiction`
Status: **DONE 2026-05-20**

### New: org-chart-panels.css
File: `pointsav-design-system/components/org-chart-panels.css`
Adds: `.org-panel-block`, `.org-ledger-block`, `.org-floater`, `.org-boundary-text`
Status: **DONE 2026-05-20**

### New: org-chart-governance.css
File: `pointsav-design-system/components/org-chart-governance.css`
Adds: `.gov-sheet`, `.gov-title-block`, `.gov-section`, `.gov-node`, `.gov-disc`, `.gov-duties`
Status: **DONE 2026-05-20**

### New: org-chart-tiers.css
File: `pointsav-design-system/components/org-chart-tiers.css`
Adds: `.tier-bar` + color variants, `.tier-ebox`
Status: **DONE 2026-05-20**

### New: org-chart-matrix.css
File: `pointsav-design-system/components/org-chart-matrix.css`
Adds: `.org-matrix`, `.org-matrix__col-head`, `.org-matrix__body-row`, `.org-matrix__cell`
Status: **DONE 2026-05-20**

### New: org-chart-venn.css
File: `pointsav-design-system/components/org-chart-venn.css`
Adds: `.venn-diagram`, `.venn-region`, `.venn-label`
Status: **DONE 2026-05-20**

---

## DESIGN-RESEARCH artifacts

### MEMO-05-Org-Chart-Patterns.md
File: `pointsav-design-system/guidelines/MEMO-05-Org-Chart-Patterns.md`
Contents: conventions extracted from this audit — entity role→color, canvas, sizing, connectors, print rules.
Status: **DONE 2026-05-20** — route to project-design for ratification

---

## ASSET-* artifacts

None. No embedded brand logos or images found in any of the 18 source charts. All rendering is pure CSS/SVG.

---

## Chart template (Step 6)

File: `pointsav-design-system/templates/org-chart-printable.html`
Purpose: Canonical starting point for new charts — links design-system CSS, declares Woodfine token variables, static HTML only.
Status: **DONE 2026-05-20**

---

## Next actions

- [ ] Verify `token-global-print.yaml` declares 1056×816px canvas token
- [ ] Author first new chart using the template (operator to specify subject)
- [ ] Customer leg: draft `GUIDE-orgchart-authoring.md` for woodfine-fleet-deployment/cluster-totebox-corporate/
- [ ] Commit all design-system changes via `commit-as-next.sh` + stage for project-design review
