# .agent/briefs/ — Durable project briefs & planning artifacts

`BRIEF-*.md` files are **permanent git-tracked artifacts** — not temp files.
Engine-agnostic: all engines (Claude Code, Gemini CLI) read and write here.

---

## Rules

- **Never delete a brief.** Supersede by editing `status: archived`, or `git mv` to `archive/`.
- **Frontmatter required:** `artifact: brief`, `status: active|archived`.
- **Filename:** `BRIEF-<topic>.md` — uppercase prefix signals permanence.
- Save planning files HERE — not `~/.claude/plans/` or `~/.gemini/tmp/`.
- AGENT.md startup step 7 reads this file; shutdown step 1 writes `BRIEF-<topic>.md`.

---

## Active briefs

| File | Subject | Status |
|---|---|---|
| BRIEF-slm-substrate-master.md | PRIMARY PLAN OF RECORD — service-slm / service-content / Yo-Yo substrate. Tier routing, DataGraph flow, Yo-Yo VM operations, audit findings. | active |
| BRIEF-slm-learning-loop.md | Training pipeline + sovereign coding agent architecture. Apprenticeship substrate, Goose setup, Sprint 1 tool_use shim, ToS boundary, Leapfrog 2030 strategic context. | active |

---

## Archived briefs

Contamination from 2026-05-22 Stage-6 rebase (27 BRIEFs from project-console,
project-infrastructure, project-editorial, project-knowledge, project-gis) moved to
`archive/` on 2026-05-29. They are not deleted — `archive/` retains them per the
permanent-artifact rule.

| File | From archive | Moved |
|---|---|---|
| BRIEF-vm-hardening-and-consolidation.md | project-intelligence (absorbed into slm-substrate-master) | still in briefs/, status: archived |
| BRIEF-PPN-ARCHITECTURE.md | project-infrastructure | archive/ 2026-05-29 |
| BRIEF-PPN-DEV-BOOTSTRAP.md | project-infrastructure | archive/ 2026-05-29 |
| BRIEF-leapfrog-2030-coding.md | project-console | archive/ 2026-05-29 |
| BRIEF-os-console-platform.md | project-console | archive/ 2026-05-29 |
| BRIEF-tui-pivot-2030.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-ceremony.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-engineering-brief.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-phase3-4.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-system-design.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-ui-design.md | project-console | archive/ 2026-05-29 |
| BRIEF-pairing-ux-design.md | project-console | archive/ 2026-05-29 |
| BRIEF-journal-phd-programme.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-active-work.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-award-winning-wiki-overhaul.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-github-presence-elevation.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-institutional-chrome-sprint.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-overhaul-documentation-pointsav-com.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-overhaul-gemini-analysis.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-overhaul-progress.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-publishing-tier-naming-cross-check.md | project-editorial | archive/ 2026-05-29 |
| BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md | project-knowledge | archive/ 2026-05-29 |
| BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md | project-knowledge | archive/ 2026-05-29 |
| BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md | project-knowledge | archive/ 2026-05-29 |
| BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md | project-knowledge | archive/ 2026-05-29 |
| BRIEF-knowledge-platform.md | project-knowledge | archive/ 2026-05-29 |
| BRIEF-TIER-REBALANCE-2026-05-24.md | project-gis | archive/ 2026-05-29 |
| BRIEF-framework-pointsav-products-services.md | Command Session (workspace) | archive/ 2026-05-29 |

---

## Non-brief files kept here

| File | Role |
|---|---|
| todo-open-items.md | Persistent open-items tracker |
| audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary / trademark / file-hygiene audit (workspace-scope; kept here for reference) |
| domain-map.tsv | Domain-map data (workspace-scope; kept here for reference) |
| vocabulary-baseline.tsv | Vocabulary-baseline data (workspace-scope; kept here for reference) |
| archive/ | Superseded + contamination briefs retained for history |

---

## Lifecycle

| Stage | Action |
|---|---|
| Plan | Create `BRIEF-<topic>.md` here with `artifact: brief` + `status: active` |
| Implement | Reference brief during implementation; update as decisions land |
| Supersede | Edit `status: archived`, or `git mv` to `archive/` — never delete |
