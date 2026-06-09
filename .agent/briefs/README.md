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

> **This archive is `project-bim`** — the BIM Objects system (woodfine-bim-library) and
> app-orchestration-bim serving bim.woodfinegroup.com. This README was contaminated by
> rebases from other archives (project-knowledge, project-intelligence); corrected 2026-06-09.
> The non-BIM briefs physically present in this dir are contamination — do not action them here.

## Active briefs

| File | Title | Updated |
|---|---|---|
| BRIEF-bim-website-pipeline.md | BIM website pipeline — nightly IFC + furniture generation | 2026-06-05 |
| BRIEF-bim-objects-system.md | BIM Objects system — 4-part architecture (Key Plans, Tiles, Floor Plates, Building Width Calculator) | 2026-06-09 |

## Contamination notes (2026-06-09)

All non-BIM `BRIEF-*.md` files physically present in this directory
(`BRIEF-comprehensive-improvement-proposal.md`, `BRIEF-knowledge-platform-master.md`,
`BRIEF-location-intelligence-archetypes-2026-06-01.md`, `BRIEF-project-console-master.md`,
`BRIEF-yoyo-cloud-run-migration.md`) are contamination from other archives. Per standing
discipline (never delete briefs), they are retained in place but must not be actioned here.
Their owning archives are: project-knowledge, project-intelligence, project-console, project-gis.

The `archive/` subdirectory and non-brief data files (`audit-foundry-wide-2026-05-16.md`,
`domain-map.tsv`, `vocabulary-baseline.tsv`) are likewise contamination — retained, not actioned.

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
