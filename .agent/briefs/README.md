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

| File | Subject | Status |
|---|---|---|
| **BRIEF-knowledge-platform-master.md** | **Knowledge platform master spec — federation (mounts + blueprints), mobile-first, premium UX (Wikipedia-model/Stripe-craft), linking model + zero dead links, Inter+Source-Serif font decision. SINGLE SOURCE OF TRUTH.** | **active** |
| BRIEF-active-work.md | project-knowledge current session work queue | active |

## Archived briefs

Superseded briefs retained per standing instruction — never delete; supersede by editing
`status: archived` or `git mv` to `archive/`.

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

## Non-brief files kept here

| File | Role |
|---|---|
| audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary / trademark / file-hygiene audit |
| domain-map.tsv | Overhaul domain-map data |
| vocabulary-baseline.tsv | Overhaul vocabulary-baseline data |
| archive/ | Archived and superseded briefs |

## Contamination flagged for owning archives (do not action here)

Physically present in this dir from rebases, but they belong to other archives. Flagged for
those archives to reclaim; not actioned in project-knowledge. (Was "Unverified briefs"; the
app-mediakit-knowledge brief was confirmed genuine project-knowledge and is now the archived
predecessor of the master.)

| File(s) | Owning archive | Status |
|---|---|---|
| ~~BRIEF-slm-substrate-master.md, BRIEF-slm-learning-loop.md~~ | project-intelligence (SLM) | **redistributed 2026-06-01** |
| BRIEF-project-intelligence-active-work.md, ~~BRIEF-substrate-phd-thesis-2026-05-27.md~~, AI-AUDIT-baseline-2026-05-31.md | project-intelligence | phd-thesis redistributed 2026-06-01; others pending |
| ~~BRIEF-totebox-transformation.md~~, BRIEF-vm-hardening-and-consolidation.md, archive/BRIEF-PPN-*.md | project-infrastructure | totebox-transformation redistributed 2026-06-01; others pending |
| BRIEF-cross-platform-release.md, archive/BRIEF-os-console-platform.md, archive/BRIEF-pairing-*.md, archive/BRIEF-tui-pivot-2030.md, archive/BRIEF-leapfrog-2030-coding.md | project-console | pending (not in this pass) |
| archive/BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md, archive/BRIEF-journal-phd-programme.md, archive/BRIEF-framework-pointsav-products-services.md, archive/BRIEF-overhaul-*.md, archive/BRIEF-github-presence-elevation.md, archive/BRIEF-publishing-tier-naming-cross-check.md | project-editorial | pending (not in this pass) |

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
