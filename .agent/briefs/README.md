# .agent/briefs/ — Durable project briefs & planning artifacts

`BRIEF-*.md` files are **permanent git-tracked artifacts** — not temp files.
Engine-agnostic: all engines (Claude Code, Gemini CLI) read and write here.

> Migrated from `.agent/plans/` on 2026-05-21 (workspace hardening Phase 1 —
> `command-20260521-briefs-migration-project-editorial`).

## Rules

- **Never delete a brief.** Supersede by editing `status: archived`, or
  `git mv` to `archive/`.
- **Frontmatter required:** `artifact: brief`, `status: active|archived`.
- **Filename:** `BRIEF-<topic>.md` — uppercase prefix signals permanence.
- Save planning files HERE — not `~/.claude/plans/` or `~/.gemini/tmp/`.
- AGENT.md startup step 7 reads this file; shutdown step 1 writes `BRIEF-<topic>.md`.

## Active briefs

| File | Subject | Status |
|---|---|---|
| BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md | Editorial execution plan for the knowledge-platform overhaul (current) | active |
| BRIEF-institutional-chrome-sprint.md | Three-site wiki redesign — institutional chrome sprint (E1/E3/E4 pending) | active |
| BRIEF-github-presence-elevation.md | GitHub presence elevation — PointSav & Woodfine (implementation queued) | active |
| BRIEF-publishing-tier-naming-cross-check.md | Publishing-tier vs canonical taxonomy naming cross-check | active |
| BRIEF-framework-pointsav-products-services.md | PointSav products & services — research-archive framework | active |

## Archived briefs

Superseded by `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`. Retained pending the
operator go-ahead recorded in that plan's §9 delete set — removed only after the
overhaul ships.

| File | Status |
|---|---|
| BRIEF-award-winning-wiki-overhaul.md | archived |
| BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md | archived |
| BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md | archived |
| BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md | archived |
| BRIEF-overhaul-documentation-pointsav-com.md | archived |
| BRIEF-overhaul-gemini-analysis.md | archived |
| BRIEF-overhaul-progress.md | archived |

## Non-brief files kept here

| File | Role |
|---|---|
| todo-open-items.md | Persistent open-items tracker (AGENT.md shutdown step 3) |
| audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary / trademark / file-hygiene audit |
| domain-map.tsv | Overhaul domain-map data |
| vocabulary-baseline.tsv | Overhaul vocabulary-baseline data |
| archive/ | Superseded plan copies retained for history |

## Lifecycle

| Stage | Action |
|---|---|
| Plan | Create `BRIEF-<topic>.md` here with `artifact: brief` + `status: active` |
| Implement | Reference brief during implementation; update as decisions land |
| Milestone | Promote to artifact(s) via `drafts-outbound/` — see routing table below |
| Supersede | Edit `status: archived`, or `git mv` to `archive/` — never delete |

## Artifact routing (at milestone)

| Artifact type | Gateway project | Destination | Notes |
|---|---|---|---|
| TOPIC-* | project-editorial | content-wiki-documentation / -projects / -corporate | Bilingual required (EN + ES) |
| GUIDE-* | project-editorial | woodfine-fleet-deployment/\<cluster\>/ | EN only; target cluster in frontmatter |
| COMMS-*, LEGAL-*, TRANSLATE-* | project-editorial | varies | Per language-protocol-substrate |
| TEXT-* | project-intelligence or project-editorial | data/training-corpus/ or drafts-outbound/ | Corpus text vs. editorial prose |
| DESIGN-COMPONENT, DESIGN-RESEARCH | project-design | pointsav-design-system | |
| DESIGN-TOKEN-* (generic) | project-design | pointsav-design-system | Requires master_cosign in frontmatter |
| DESIGN-TOKEN-* (PointSav / Woodfine branded) | project-design | pointsav-media-assets / woodfine-media-assets | |
| ASSET-* | project-design | pointsav-media-assets or woodfine-media-assets | |
| BIM-* | project-bim | pointsav-bim-system (pending repo transfer) | UI/UX tokens vs BIM tokens — separate substrates |
| LICENSE-* | Command Session (admin-tier) | factory-release-engineering | |
| Self-contained | this project-* | own drafts-outbound/ or direct commit | Artifact stays in originating project |
