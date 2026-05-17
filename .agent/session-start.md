---
schema: foundry-session-start-v1
archive: project-bim
updated: 2026-05-14
---

# Session start — project-bim

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Building Information Modeling (BIM) design system + City Code as Composable Geometry pattern (Doctrine claim #41). Owns `woodfine-design-bim` repo and BIM-specific tokens/components that are NEVER routed to `pointsav-design-system`.
- **Active branch:** `cluster/project-bim`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- **BIM artifacts never go to `pointsav-design-system`.** BIM tokens/components route to `woodfine-design-bim` only. This is a hard architectural boundary.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- See `.agent/manifest.md` and `manifest-notes.md` for full strategic context (split 2026-05-09 to keep manifest under 30 KB cap).

## Last session handoff

**2026-05-17 — totebox@claude-code**

Print/PDF pipeline resolved. DTCG token files and Rust scaffold are the active work.

- Print CSS fixed across all 3 preview HTMLs: `@page { size: 11in 8.5in; margin: 0; }` + slide matches page box exactly
- PDF generator at `preview/build-pdf.mjs` (Playwright/Chromium) — verified 6-page, 11×8.5in PDF
- Generate: `NODE_PATH=/home/jennifer/sandbox/working/ps-talking-points/node_modules node build-pdf.mjs <file.html>`
- PDFs committed alongside HTMLs; do NOT use browser print dialog
- Next: create 6 missing DTCG token files (see NEXT.md + `.agent/plans/tool-buildingwidth-architecture.md`)
- DTCG accuracy errors (3 files) remain operator-pending; do not touch
