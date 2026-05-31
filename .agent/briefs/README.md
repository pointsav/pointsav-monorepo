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
| BRIEF-slm-substrate-master.md | SLM substrate ops — Yo-Yo VM, DataGraph, tier routing, circuit resilience. PRIMARY PLAN OF RECORD for all SLM sessions. | active |
| BRIEF-slm-learning-loop.md | Training pipeline — ToS boundary, DPO corpus, sovereign coding agent, sprints, Fix A/B/C | active |
| BRIEF-project-intelligence-active-work.md | Active work queue — 3-session forward plan; session-start reading | active |
| AI-AUDIT-baseline-2026-05-31.md | Gemini CLI automated audit vs architecture principles; findings integrated into substrate master §5 | active |

## Archived briefs

Superseded briefs retained per standing instruction — never delete; supersede by editing
`status: archived` or `git mv` to `archive/`.

| File | Archived | Notes |
|---|---|---|
| archive/BRIEF-VM-ARCHITECTURE.md | 2026-06-01 | Contamination from project-infrastructure — read from there |
| archive/BRIEF-OS-FAMILY.md | 2026-06-01 | Contamination from project-infrastructure — read from there |
| archive/BRIEF-LEAPFROG-2030.md | 2026-06-01 | Contamination from project-infrastructure — read from there |
| archive/todo-open-items.md | 2026-06-01 | Contamination from project-editorial — read from there |

## Non-brief files kept here

| File | Role |
|---|---|
| audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary / trademark / file-hygiene audit |
| domain-map.tsv | Overhaul domain-map data |
| vocabulary-baseline.tsv | Overhaul vocabulary-baseline data |
| archive/ | Archived and superseded briefs |

## Unverified briefs (present on disk, not yet audited for correct archive)

The following files exist in this directory but were not reviewed in the 2026-06-01
consolidation. A future session should confirm each belongs here or move to archive/.

| File | Origin suspicion |
|---|---|
| BRIEF-app-mediakit-knowledge-2030.md | Possible project-knowledge contamination |
| BRIEF-substrate-phd-thesis-2026-05-27.md | Likely project-intelligence (system-*/seL4 thesis) |
| BRIEF-totebox-transformation.md | Possible project-infrastructure contamination |
| BRIEF-vm-hardening-and-consolidation.md | Absorbed into BRIEF-slm-substrate-master.md (listed in that brief's `replaces:`) |

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
