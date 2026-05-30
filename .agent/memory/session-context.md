---
schema: foundry-session-context-v1
archive: project-design
format: rolling-3-entries  # oldest entry pushed to session-context-archive.md
---

# Session context — project-design

---

## 2026-05-26–30 (working session) | Totebox | claude-code

**Done this session:**
- BIM supplement (project-bim-20260517-design-sweep-supplement) fully actioned:
  - html-print-pdf-pipeline.md committed to `pointsav-design-system/research/` (`a6dc0df`, Jennifer Woodfine)
  - BIM design-index accepted as-is; namespace decision: keep `tokens/bim/`, `components/bim-*/`, `research/bim-*.md`
  - 9 generic components flowback acknowledged; ps- naming convention chosen for META-substrate; P1/P2/P3 priority queue noted
  - ACK sent to project-bim (outbox `project-design-20260526-bim-design-index-ack`)
- Knowledge design commission (command-20260524-knowledge-design-routing) fully actioned:
  - 10 files committed to `pointsav-design-system/dtcg-vault/research/` + `competition/` (`36770dd`, Peter Woodfine)
  - 5 DESIGN-RESEARCH: visual-language, ux-writing, service-design, token-architecture, market-positioning (BCSC-reviewed, internal only)
  - 5 DESIGN-COMPETITION: 4 HTML prototypes (A/B/C/D) + jury report (hybrid = D tokens + A shell + B TOC)
  - ACK sent to project-knowledge (outbox `project-design-20260530-knowledge-design-ack`)
- New inbox message noted: `command-20260529-journal-relay-design-j6` — J6 journal article; flag token/component decisions re professional power-user patterns; route to project-editorial as JOURNAL-NOTES-j6

**pointsav-design-system HEAD:** `36770dd` on `main`
**woodfine-media-assets HEAD:** `5753b96` on `main` (up to date with canonical)
**project-design archive HEAD:** `934651ca` on `cluster/project-design`

**Pending / carry-forward:**
- [ ] DTCG chart entity-role tokens from project-orgcharts (BLOCKED: waiting on project-orgcharts Stage 6; msg-id: project-orgcharts-20260521-chart-tokens-dtcg)
- [ ] 9 generic BIM components → META-substrate: CodeBlockWithCopy, EmptyStateCard, ChipRow (P1), SidebarAccordion, TabBarDisclosure, BreadcrumbNav, PreviewFrame, MachineSurfaceFooter (P2), EditOnGitHubLink (P3)
- [ ] J6 journal relay: flag relevant token/component decisions to project-editorial (msg-id: command-20260529-journal-relay-design-j6)
- [ ] GIS screenshots: asset-gis-map-screenshots-2026-05-06.md at asset-capture-pending-operator (operator action)
- [ ] pointsav-design-system Stage 6 still pending (commits on main not yet promoted to canonical via promote.sh)

**Operator preferences surfaced:** none new.

---

## 2026-05-23 (working session) | Totebox | claude-code

**Done this session:**
- `binary-targets.yaml` committed (a0222ff) — declared `app-privategit-design` scaffold, FSL-1.1-ALv2, soft_enabled: false
- 5 project-editorial DESIGN drafts committed to pointsav-design-system (7a50a43): 4 GIS component recipe.html files (brand-family-swatch, country-filter-chips, map-side-drawer, map-stats-panel) + research/zoom-tier-reveal-pattern.md; ACK sent to project-editorial
- icon-tab component committed (4d46147): HTML + CSS + ARIA; 3 open items in aria.md; ACK sent to project-marketing
- woodfine-blue-tint token committed + pushed to canonical woodfine-media-assets (5753b96); ACK sent to project-marketing
- fleet-deployment .claude→.agent migration + guide lowercase rename committed (4188310)
- Mailbox updates committed (1bf4f02)

**pointsav-design-system HEAD:** `4d46147` on `main`
**woodfine-media-assets HEAD:** `5753b96` on `main` (up to date with canonical)

**Pending / carry-forward:**
- [ ] DTCG chart entity-role tokens from project-orgcharts (BLOCKED: waiting on project-orgcharts Stage 6; msg-id: project-orgcharts-20260521-chart-tokens-dtcg)
- [ ] 3 BIM supplemental drafts from project-bim (NEW 2026-05-24; msg-id: project-bim-20260517-design-sweep-supplement): html-print-pdf-pipeline research, BIM design-index, 9 generic components
- [ ] GIS screenshots: asset-gis-map-screenshots-2026-05-06.md at asset-capture-pending-operator (operator action)
- [ ] pointsav-design-system Stage 6 still pending (commits on main not yet promoted to canonical via promote.sh)

**Operator preferences surfaced:** none new.

---

## 2026-05-23 (startup-shutdown) | Totebox | claude-code

**Done this session:** Startup-then-immediate-shutdown. Stale session lock from 2026-05-20 cleared (PID 702884 dead). No work performed.

**Pending / carry-forward (superseded by working session above).**
