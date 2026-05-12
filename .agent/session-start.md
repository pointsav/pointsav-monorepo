---
schema: foundry-session-start-v1
archive: project-orchestration
updated: 2026-05-12
---

# Session start — project-orchestration

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Implement the Totebox Orchestration transition — Phases 1, 2, and 3; owns `app-orchestration-command` in `pointsav-monorepo` (user-facing aggregator hub).
- **Active branch:** `cluster/project-orchestration`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** check `.agent/plans/` for any `*.md` marked in-progress (currently only `README.md`)

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Transition phases 1–3 scope | `.agent/manifest.md` §Cluster mission |
| pairings.yaml (Phase 2) | `~/Foundry/pairings.yaml` |
| list-archives.sh (Phase 3) | `~/Foundry/bin/list-archives.sh` |

## Known gotchas for this archive

- **Phase 1 is complete.** CLAUDE.md §11, AGENT.md, and `bin/claude-role.sh` updated to Command/Totebox vocabulary. P1.4 (os-orchestration target) + Phase 2 (pairings.yaml + slm_endpoint + project-source/project-woodfine) + Phase 3 (bin/open-archive.sh + app-orchestration-command v0.0.1 scaffold) are pending.
- **`target_os: os-orchestration` is planned.** The workspace itself (`~/Foundry`) will eventually run on `os-orchestration`. This is future/intended — not current-fact.
- **Wiki drafts already staged.** 5 architecture/systems articles routed to project-editorial 2026-05-08. No wiki leg commits yet.
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-08 — Phase 1 vocabulary transition complete (Command/Totebox, Root eliminated). project-orchestration archive provisioned. P1.4 (target OS naming) + Phase 2 (pairings.yaml + slm_endpoint on all 13 manifests + 2 new archives) + Phase 3 (bin/open-archive.sh + app-orchestration-command scaffold) deferred. 5 wiki drafts staged for project-editorial.*
