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

> **This archive is `project-knowledge`** — the app-mediakit-knowledge knowledge platform.
> The prior version of this README (and `.agent/manifest.md`) were contaminated by rebases and
> described it as project-intelligence/project-bim; corrected 2026-06-01. The genuine
> project-knowledge briefs are below. The SLM / intelligence / infrastructure / console /
> editorial briefs physically present in this dir are **contamination flagged for their owning
> archives** — see "Contamination flagged" at the bottom; do not action them here.

## Active briefs

*(none — all briefs in this archive are contamination from other sessions. See Archived section below.)*

Legitimate project-data briefs should be created here when GIS/data pipeline or JOURNAL
programme work generates planning artifacts that span multiple sessions.

## Archived briefs

All briefs in `archive/` are retained per standing discipline (never delete). The 2026-06-01
cleanup archival was a contamination sweep — none of these described project-data work.

### Archived 2026-06-01 — contamination sweep (previous sessions ran other archives' work here)

| File | Origin | Notes |
|---|---|---|
| archive/BRIEF-slm-substrate-master.md | project-intelligence | SLM Yo-Yo + Doorman + tier routing. Redistribute to project-intelligence. |
| archive/BRIEF-slm-learning-loop.md | project-intelligence | DPO corpus + LoRA training pipeline. Redistribute to project-intelligence. |
| archive/BRIEF-project-intelligence-active-work.md | project-intelligence | 3-session forward plan for project-intelligence. Redistribute. |
| archive/AI-AUDIT-baseline-2026-05-31.md | project-intelligence | Gemini CLI vs architecture audit. Redistribute to project-intelligence. |
| archive/BRIEF-substrate-phd-thesis-2026-05-27.md | project-intelligence | seL4/verified systems PhD thesis prep. Redistribute to project-intelligence. |
| archive/BRIEF-vm-hardening-and-consolidation.md | project-intelligence | Absorbed into BRIEF-slm-substrate-master.md; already archived before sweep. |
| archive/BRIEF-app-mediakit-knowledge-2030.md | project-knowledge | Knowledge platform Leapfrog 2030 vision. Redistribute to project-knowledge. |
| archive/BRIEF-active-work-project-knowledge-2026-05-31.md | project-knowledge | project-knowledge session work queue. Redistribute to project-knowledge. |
| archive/BRIEF-cross-platform-release.md | project-console | os-console cross-platform release engineering. Redistribute to project-console. |
| archive/BRIEF-totebox-transformation.md | project-infrastructure | VM fabric implementation. Redistribute to project-infrastructure. |

### Archived 2026-06-01 — prior sweep (moved by previous session)

| File | Archived | Notes |
|---|---|---|
| BRIEF-app-mediakit-knowledge-2030.md | 2026-06-01 | Superseded by BRIEF-knowledge-platform-master.md (content absorbed) |
| archive/BRIEF-award-winning-wiki-overhaul.md | 2026-06-01 | Historical wiki brief — superseded by the master |
| archive/BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md | 2026-06-01 | Historical wiki brief — superseded by the master |
| archive/BRIEF-WIKIPEDIA-PARITY-*.md | 2026-06-01 | Historical wiki briefs (parity-era) — superseded by the master |
| archive/BRIEF-institutional-chrome-sprint.md | 2026-06-01 | Historical wiki brief — superseded by the master |
| archive/BRIEF-knowledge-platform.md | 2026-05-28 | Predecessor (already archived) — chain points to the master |
| ~~archive/BRIEF-VM-ARCHITECTURE.md~~ | 2026-06-01 | Redistributed to project-infrastructure 2026-06-01 — git rm'd |
| ~~archive/BRIEF-OS-FAMILY.md~~ | 2026-06-01 | Redistributed to workspace root 2026-06-01 — git rm'd |
| ~~archive/BRIEF-LEAPFROG-2030.md~~ | 2026-06-01 | Redistributed to workspace root 2026-06-01 — git rm'd |
| archive/todo-open-items.md | 2026-06-01 | Contamination from project-editorial — read from there |

### Earlier archived briefs (knowledge platform era)

Numerous briefs in `archive/` from earlier project-knowledge and project-editorial sessions
(BRIEF-KNOWLEDGE-PLATFORM-*, BRIEF-Wikipedia-parity-*, BRIEF-overhaul-*, etc.). These were
from sessions that predated the separate archive structure. See `archive/` directory listing.

## Non-brief files kept here

| File | Role |
|---|---|
| audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary / trademark / file-hygiene audit |
| domain-map.tsv | Overhaul domain-map data |
| vocabulary-baseline.tsv | Overhaul vocabulary-baseline data |
| archive/ | Archived and superseded briefs |

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
