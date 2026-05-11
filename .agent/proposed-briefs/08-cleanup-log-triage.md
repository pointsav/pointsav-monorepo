# Brief: Cleanup-log triage — pointsav-monorepo

**target**: Read-only triage of the cleanup log + project registry + handoffs outbox in the project-knowledge cluster's working copy of pointsav-monorepo. Produce a categorised report: closed-in-fact-but-still-listed-as-open; needs-re-triage-given-recent-doctrine-update (v0.1.31, v0.1.42, claim #37); stale-but-still-relevant; confirmed-open-no-change-needed.
**target_files** (read-only):
- `/srv/foundry/clones/project-knowledge/pointsav-monorepo/.claude/rules/cleanup-log.md`
- `/srv/foundry/clones/project-knowledge/pointsav-monorepo/.claude/rules/project-registry.md`
- `/srv/foundry/clones/project-knowledge/pointsav-monorepo/.claude/rules/handoffs-outbound.md`
- recent git log (last 20 commits on `cluster/project-knowledge`)
**expected_output**: One structured markdown report with 4 categories (A/B/C/D). For Category A, propose exact table-cell edit or strike-through to close the item. No writes.
**max_response_lines**: 120
**model_tier**: sonnet
**parallelisable**: yes (read-only)
**confidence_gate_passes**: yes — mechanical cross-check of structured tables against session entries and 20-commit log
**layer_scope**: task — all files in cluster's working copy
**anti_slop_check**: directly unblocks cleanup-log maintenance before next Task session writes Phase 4 entries on top of stale state
**dependencies**: none

## Specification

The sub-agent walks the cleanup log's structured sections + project registry + handoffs outbox, cross-referencing each open item against recent commits and recent doctrine updates. Categorise per the four-bucket schema:

### Category A — Closed-in-fact, still listed as open

Items where evidence in cleanup-log session entries, registry notes, or git log shows the work is done but the table row or open-question entry has not been struck through or moved. For each Category-A item, propose the exact edit (table-cell change, strike-through, or "Move to Completed migrations" instruction).

### Category B — Needs re-triage given doctrine update

Items whose handling guidance was written before:
- v0.1.30 (sub-agent dispatch convention — exit+re-enter deprecated)
- v0.1.31 (Reverse-Funnel Editorial Pattern — claim #35; drafts-outbound input port; Tasks have apprenticeship corpus write permission)
- v0.1.42 (SLM Operationalization Plan — PK.X enumeration; Sonnet sub-agent dispatch named for PK.2/PK.3)
- v0.1.10 (Tetrad Discipline — claim #37; wiki leg mandatory)

The item itself may still be open; the recommended action or framing requires updating to reflect newer convention.

### Category C — Stale-but-still-relevant

Items with no touches in multiple sessions, genuinely open, should be re-surfaced as active work given current cluster trajectory (Phase 4 plan landed, HTTPS live, Tetrad backfilled).

### Category D — Confirmed open, no change needed

Open items correctly labelled, handling guidance current; brief acknowledgement only.

### Pre-loaded context (parent supplies; sub-agent may skip re-reading the log lines)

**Recent commits (last 20 on `cluster/project-knowledge`)** — newest first:
- `7b7248e` — cleanup-log: Tetrad backfill (claim #37 / doctrine v0.0.10; triad → tetrad)
- `ea26118` / `e09d9a8` — PK.4 smoke runbook + PK.1 BP1 decision packet
- `8d8ed7c` — cleanup-log: Phase 2 Step 7 collab + Phase 4 plan + HTTPS-launch
- `73e931e` / `05f1dab` — Phase 4 plan + Phase 2 Step 7 shipped
- `9fcd73c` / `bbd995a` / `72c4756` / `0ace07e` — Phase 3 Steps 3.1–3.4 shipped
- `24449f5` / `2bd74e9` / `fd1adf9` / `8f5f010` / `69e5610` / `b8580f9` — Phase 2 Steps 1–6 shipped
- Earlier: Phase 1.1 chrome, session-2 research synthesis, Phase 1

**Registry summary (last updated 2026-04-23)**: Active 4 / Scaffold-coded 53 / Reserved 36 / Defect 0 / Total 97

**Handoffs outbox (2 entries, both `pending-destination-commit`)**:
1. `GUIDE-OPERATIONS.md` → `content-wiki-documentation`
2. `USER_GUIDE_2026-03-30_V2.md` → `content-wiki-documentation`

### Triage scope per section

- **Active renames** (4 rows): cross-check against registry + commits
- **Deprecations** (1 row: `fleet-command-authority`): confirm still-open
- **Open questions** (~7 active rows + 1 struck-through): each — answered? need re-triage under doctrine v0.0.10?
- **Session entries**: scan for items logged "pending" or "in-flight" now closed by subsequent commits
- **Handoffs outbox**: confirm no evidence in recent git log that destination commits landed

## Output shape

One structured markdown report, sections A–D as above. Each entry: item identifier (table row or question text), evidence basis (commit SHA / session-entry date / registry note), and (Category A only) the exact proposed edit. Cap 120 lines. No preamble.

## Acceptance criteria

- All 4 categories present in report
- Each open question + active rename + deprecation + handoffs-outbox-entry is categorised exactly once
- Category A entries name a concrete edit
- Category B entries name which doctrine version triggers re-triage
- Cap 120 lines
- No writes

## Risks / unknowns

- The log's line numbers may shift between sessions; sub-agent should reference items by row text or question text, not line number
- Some Category-A candidates may be ambiguously closed (commit SHA shows partial closure); sub-agent should mark these as "Category A (partial)" rather than force-categorise
- The handoffs outbox's "destination committed" signal lives in the destination repo's commit history, not visible from this cluster's git log; sub-agent should report "evidence absent in current cluster's log; destination repo separate" rather than presume open or closed
