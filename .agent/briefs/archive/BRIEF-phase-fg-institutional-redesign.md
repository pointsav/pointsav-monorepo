---
artifact: brief
status: archived
status_note: content absorbed into BRIEF-artifact-style-guide.md §13 (2026-06-11); open operator questions documented there; no standalone content
archived: 2026-06-11
created: 2026-06-11
updated: 2026-06-11
author: totebox@project-editorial (claude-sonnet-4-6)
contaminated_note: "M-17 contamination — belongs to project-editorial; archived from project-gis 2026-06-13 by command@claude-code"
---

# BRIEF — Phase F+G: Institutional Redesign Follow-through

## Origin

Workspace session-context carry-forward (2026-06-11 startup): "Phase F+G — 6 GUIDEs +
DESIGN-wiki-institutional-redesign; no BRIEF yet; scoping needed in project-editorial
Totebox first."

Prior context: `archive/BRIEF-institutional-chrome-sprint.md` (archived 2026-05-23)
documents Phases A–E of the three-site wiki redesign for documentation.pointsav.com,
corporate.woodfinegroup.com, and projects.woodfinegroup.com. Phases B1–B6 (CSS redesign),
C1–C7 (Rust rebuild), and D (per-site theme verification) were COMPLETE at archival.
Phase E quality gates (E1/E3/E4 — wanted, category counts, title QA) and Stage 6
promotion were PENDING.

---

## Interpreted scope (operator confirmation required before any work)

### Phase F — DESIGN-wiki-institutional-redesign

A DESIGN artifact capturing the institutional chrome patterns established in the chrome
sprint for routing to project-design → pointsav-design-system:

- **DESIGN-TOKEN-CHANGE:** CSS custom-property tokens confirmed in the build:
  `--wf-claret`, `--wf-slate`, `--ds-*` system; Oswald + Roboto Slab + Nunito Sans
  typography scale.
- **DESIGN-COMPONENT:** Three-row header recipe (utility | brand | nav); footer recipe
  (cities | copyright | trademark); wordmark SVG spec per site.

Requires `master_cosign:` before committing to design-system.
Routed to project-design via `.agent/drafts-outbound/`.

### Phase G — 6 GUIDEs

Operational runbooks for the three wiki sites. Interpretation: 2 GUIDEs × 3 sites = 6.

| GUIDE | Target in woodfine-fleet-deployment |
|---|---|
| guide-media-knowledge-documentation-deploy.md | media-knowledge-documentation/ |
| guide-media-knowledge-documentation-content-operations.md | media-knowledge-documentation/ |
| guide-media-knowledge-corporate-deploy.md | media-knowledge-corporate/ |
| guide-media-knowledge-corporate-content-operations.md | media-knowledge-corporate/ |
| guide-media-knowledge-projects-deploy.md | media-knowledge-projects/ |
| guide-media-knowledge-projects-content-operations.md | media-knowledge-projects/ |

Each GUIDE covers: service start/stop/restart, content sync workflow, binary rebuild
trigger, nginx/certbot renewal, log inspection, rollback procedure.

---

## Open questions for operator (send via outbox to command@claude-code)

1. Is the scope interpretation above correct? If the 6 GUIDEs differ, list them.
2. Are Phase E gates (E1/E3/E4 quality gates) complete, or does Phase F depend on them?
3. Are canonical WFD subdirectory names `media-knowledge-documentation/`,
   `media-knowledge-corporate/`, `media-knowledge-projects/` confirmed?
4. For Phase F (DESIGN): is there an existing design-system draft or token file to build from,
   or does project-editorial author from scratch based on the live CSS?
5. Who co-signs the DESIGN-TOKEN-CHANGE (Master cosign requirement)?
6. Is Stage 6 for the chrome sprint now complete, or still pending Command?

---

## Prerequisites before any content work

- [ ] Operator or Command confirms scope interpretation
- [ ] Phase E gates confirmed complete (or waived)
- [ ] Stage 6 for chrome sprint confirmed promoted
- [ ] Master cosign identified for Phase F token change
