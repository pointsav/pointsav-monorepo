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
| BRIEF-active-work.md | Current work queue — consolidated action brief | active |
| BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md | Knowledge-platform editorial execution plan (complete; archival operator-gated §9) | active |
| BRIEF-framework-pointsav-products-services.md | PointSav products & services — research-archive framework (standing reference) | active |

## Archived briefs

Superseded briefs retained per standing instruction — deleted only after the
overhaul ships, on explicit operator go-ahead.

| File | Archived | Superseded by |
|---|---|---|
| BRIEF-institutional-chrome-sprint.md | 2026-05-23 | BRIEF-active-work.md |
| BRIEF-github-presence-elevation.md | 2026-05-23 | BRIEF-active-work.md |
| BRIEF-publishing-tier-naming-cross-check.md | 2026-05-23 | BRIEF-active-work.md |
| BRIEF-award-winning-wiki-overhaul.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-overhaul-documentation-pointsav-com.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-overhaul-gemini-analysis.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| BRIEF-overhaul-progress.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |

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
