# Briefs — project-system

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs (native to project-system)

| Brief ID | Title | Status |
|---|---|---|
| `project-system-os-totebox-build-out` | os-totebox Build-Out | active |
| `project-system-os-totebox-ppn-build-out` | os-totebox PPN Build-Out | active |

## Foreign briefs (contamination — pending Command Session redistribution)

Most briefs in this directory belong to other archives and arrived via the contamination
event documented in outbox msg-id `project-system-20260614-drafts-outbound-contamination`.
Do not act on foreign briefs; route via outbox to Command Session for redistribution.

| Brief file | Likely owner |
|---|---|
| BRIEF-app-privategit-design.md | project-design |
| BRIEF-bim-website-pipeline.md | project-bim |
| BRIEF-design-system-platform-2030.md | project-design |
| BRIEF-knowledge-platform-master.md | project-knowledge |
| BRIEF-marketing-platform-master.md | project-marketing |
| BRIEF-workplace-workbench.md | project-workplace |
| BRIEF-os-orchestration-build-out.md | project-orchestration (verify) |
| BRIEF-trademark-changeover-mcorp-capability-geometry.md | project-system or Command (verify) |

## Routing

All editorial drafts → `.agent/drafts-outbound/` → project-editorial.
All design artifacts → `.agent/drafts-outbound/` → project-design.
Stage 6 requests → Command Session outbox.
