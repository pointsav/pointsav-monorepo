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
| BRIEF-active-work.md | Current work queue — project-knowledge session action list | active |
| BRIEF-app-mediakit-knowledge-2030.md | app-mediakit-knowledge product brief — all phases, locked decisions, 2030 vision. PRIMARY BRIEF for wiki engine work. | active |
| BRIEF-LEAPFROG-2030.md | Leapfrog 2030 os-* resource targets (Phase 2/3 disk/RAM). *Workspace-level — pending redistribution to Command.* | active |
| BRIEF-OS-FAMILY.md | Five os-* types reference. *Workspace-level — pending redistribution to Command.* | active |
| BRIEF-VM-ARCHITECTURE.md | VM-* to os-* canonical mapping; unikernel roadmap. *project-infrastructure scope — pending redistribution.* | active |
| BRIEF-slm-substrate-master.md | SLM Yo-Yo substrate + DataGraph + learning loop. PRIMARY PLAN for SLM work. *project-intelligence scope — pending redistribution.* | active |
| BRIEF-slm-learning-loop.md | SLM training pipeline + sovereign coding agent. *project-intelligence scope — pending redistribution.* | active |
| BRIEF-substrate-phd-thesis-2026-05-27.md | PhD thesis draft (trustworthy systems). *project-system scope — pending redistribution.* | active |
| BRIEF-totebox-transformation.md | VM fabric implementation plan. *project-infrastructure scope — pending redistribution.* | active |

## Archived briefs

| File | Archived | Superseded by / Notes |
|---|---|---|
| BRIEF-vm-hardening-and-consolidation.md | 2026-05-24 | Absorbed into BRIEF-slm-substrate-master.md |
| BRIEF-gemini-handover-2026-05-30.md | 2026-05-31 | Stale Gemini session note; wrong ports; archivist note inside |
| archive/BRIEF-institutional-chrome-sprint.md | 2026-05-23 | Superseded by active-work |
| archive/BRIEF-github-presence-elevation.md | 2026-05-23 | Superseded by active-work |
| archive/BRIEF-publishing-tier-naming-cross-check.md | 2026-05-23 | Superseded by active-work |
| archive/BRIEF-award-winning-wiki-overhaul.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| archive/BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| archive/BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| archive/BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| archive/BRIEF-overhaul-*.md | prior | BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md |
| archive/BRIEF-PPN-ARCHITECTURE.md | prior | Now in project-infrastructure archive |
| archive/BRIEF-PPN-DEV-BOOTSTRAP.md | prior | Now in project-infrastructure archive |

**Note on cross-archive BRIEFs:** Several active BRIEFs (LEAPFROG-2030, OS-FAMILY,
VM-ARCHITECTURE, slm-*, substrate-phd-thesis, totebox-transformation) belong to other
archives. They live here pending redistribution; outbox to Command filed 2026-05-31.

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
