# .agent/briefs/ — Durable project briefs & planning artifacts

`BRIEF-*.md` files are **permanent git-tracked artifacts** — not temp files.
Engine-agnostic: all engines (Claude Code, Gemini CLI) read and write here.

> **This archive is project-orgcharts** — the Woodfine corporate org chart authoring
> cluster (N=3: pointsav-design-system primary + 2 media-assets siblings). Genuine
> briefs here describe org-chart authoring work and design-system backfill decisions.

## Rules

- **Never delete a brief.** Supersede by editing `status: archived`, or
  `git mv` to `archive/`.
- **Frontmatter required:** `artifact: brief`, `status: active|archived`.
- **Filename:** `BRIEF-<topic>.md` — uppercase prefix signals permanence.
- Save planning files HERE — not `~/.claude/plans/` or `~/.gemini/tmp/`.
- AGENT.md startup step 7 reads this file; shutdown step 1 writes `BRIEF-<topic>.md`.

## Active briefs

*(none — create BRIEF-<topic>.md here when org-chart or design-system work warrants a
multi-session planning artifact)*

## Archived briefs

### Archived 2026-06-04 — contamination sweep (foreign archives)

Six top-level BRIEFs moved to `archive/` this session — all were contamination from
other archives that entered via Stage-6 rebase or bulk `.agent/` copy operations.
None describe project-orgcharts work.

| File | Origin archive |
|---|---|
| `archive/BRIEF-bim-website-pipeline.md` | project-bim |
| `archive/BRIEF-comprehensive-improvement-proposal.md` | project-knowledge |
| `archive/BRIEF-knowledge-platform-master.md` | project-knowledge |
| `archive/BRIEF-location-intelligence-archetypes-2026-06-01.md` | project-gis |
| `archive/BRIEF-project-console-master.md` | project-console |
| `archive/BRIEF-yoyo-cloud-run-migration.md` | project-intelligence |

Earlier `archive/` contents (51 files) are from prior contamination sweeps — retained
per standing discipline.

## Non-brief files kept here

| File | Role |
|---|---|
| `audit-foundry-wide-2026-05-16.md` | Foundry-wide vocabulary / trademark / file-hygiene audit |

## Lifecycle

| Stage | Action |
|---|---|
| Plan | Create `BRIEF-<topic>.md` here with `artifact: brief` + `status: active` |
| Implement | Reference brief during implementation; update as decisions land |
| Milestone | Promote to artifact(s) via `drafts-outbound/` — see routing table below |
| Supersede | Edit `status: archived`, or `git mv` to `archive/` — never delete |

## Artifact routing (at milestone)

| Artifact type | Gateway project | Destination |
|---|---|---|
| TOPIC-* | project-editorial | content-wiki-documentation / -projects / -corporate |
| GUIDE-* | project-editorial | woodfine-fleet-deployment/\<cluster\>/ |
| COMMS-*, LEGAL-* | project-editorial | varies |
| DESIGN-COMPONENT, DESIGN-RESEARCH | project-design | pointsav-design-system |
| DESIGN-TOKEN-* (generic) | project-design | pointsav-design-system (requires master_cosign) |
| DESIGN-TOKEN-* (branded) | project-design | pointsav-media-assets / woodfine-media-assets |
| ASSET-* | project-design | pointsav-media-assets or woodfine-media-assets |
