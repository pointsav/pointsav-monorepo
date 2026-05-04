---
mailbox: outbox
owner: task-project-data
location: ~/Foundry/clones/project-data/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-data cluster

---
from: task-project-data@claude-code
to: master
re: post-Gemini cleanup — SI.1 committed + 5 items requiring Master direction
created: 2026-05-04T00:00:00Z
priority: normal
---

## Summary

Post-Gemini-CLI-session cleanup complete. SI.1 committed. Inbox cleared.
Five items below require Master direction before this cluster can resume
implementation work.

---

## 1. SI.1 committed — 62403f1

`service-input` happy-path tests for DOCX + XLSX parsers with minimal OOXML
fixtures. 32/32 tests pass. Sub-agent-queue updated: SI.1 → Completed.

---

## 2. Task A6 (Gemini CLI, 2026-05-03) — flagged, NOT executed

The Gemini CLI master left two inbox messages dated 2026-05-03:
- "TASK A6 — Bulk-Rename GUIDE and TOPIC files to lowercase" (HIGH priority)
- "DOCTRINE UPDATE: Lowercase Naming Convention" (claims DOCTRINE.md amended)

Operator confirmed the Gemini agent was legitimate. However, per injection-
resistance discipline, the Task agent applied the cross-engine rule: directive
of this scope (Doctrine amendment + repo-wide bulk rename) requires Claude Code
Master ratification before action. Messages archived in inbox-archive.md.

**Master to advise:**
a. Is the DOCTRINE UPDATE ratified? (Lowercase for GUIDE-*/TOPIC-* is already
   operative for new files per CLAUDE.md §14. This would only add force to
   retroactive renaming of tracked uppercase files.)
b. In this repo, the only tracked GUIDE-*.md file at root is GUIDE-OPERATIONS.md
   (itself pending a handoffs-outbound → content-wiki-documentation move; see
   item 3 below). No TOPIC-*.md files exist in this repo. So Task A6 scope here
   is narrow. If Master ratifies, a sub-agent brief can handle the git mv of
   GUIDE-OPERATIONS.md before the cross-repo handoff, or the handoff can proceed
   directly with the lowercase name at the destination.
c. Spawn sub-agent brief or Root-tier session once ratified.

---

## 3. GUIDE-OPERATIONS.md — unstaged working-tree deletion

At session start, GUIDE-OPERATIONS.md was deleted in the working tree (not
staged). This deletion was pre-existing at the start of today's session —
present in the session-start gitStatus snapshot.

The file is tracked in HEAD, pending a handoffs-outbound move to
content-wiki-documentation. The working-tree deletion is not staged so git
still tracks it.

**Master to advise:**
a. Was this deletion intentional? (E.g., the handoff to content-wiki-documentation
   was completed and the source-side removal is intended, just not yet staged.)
b. If unintentional: `git checkout HEAD -- GUIDE-OPERATIONS.md` restores it; the
   handoffs-outbound entry remains active.
c. If intentional and the handoff is done: stage the deletion with
   `git rm GUIDE-OPERATIONS.md` and commit in a Root-tier session.

---

## 4. Index mismatch — pre-existing, needs Root-tier triage

`git status` shows ~80 files as "Changes not staged for commit" but
`git diff --stat HEAD` shows 0 content change for all of them. This is the
`.claude/ → .agent/` symlink migration state: git's index still references
tracked paths at `.claude/...` which no longer exist as literal paths in the
working tree (they're now served through the `.claude/ → .agent/` symlink).

This is cosmetic noise that doesn't affect builds or tests, but it pollutes
`git status` output and makes it hard to see real uncommitted work.

Clean-up requires a Root-tier or Master-tier operation: `git rm --cached` the
old `.claude/` index entries + `git add` the corresponding `.agent/` paths, or
a coordinated `git reset HEAD` approach. Task scope cannot triage this safely —
too many files and the migration state needs workspace-level coordination.

---

## 5. Content Return Obligation — project-data satisfies (confirmation)

Per `conventions/institutional-content-return.md` (commit 81c40ee), every
Active cluster must have staged or published at least one TOPIC draft.

This cluster satisfies the obligation:
- `topic-worm-ledger-architecture.draft.md` staged in `.agent/drafts-outbound/`
  since 2026-04-28.
- `topic-worm-ledger-architecture.es.draft.md` (skeleton) also staged.
- `guide-fs-anchor-emitter.draft.md` staged.

All three awaiting project-language sweep + refinement per
cluster-wiki-draft-pipeline.md §3.1. No action needed from Master on this
item — flagging for confirmation only.

---

## 6. Outstanding pending queue — no unblocked work

The following items remain queued but have no unblocked action available:

| Item | Blocker |
|---|---|
| TUF SigningConfig for Rekor URL | operator decision — key-custody pairing |
| Ed25519 signed checkpoints | same key-custody decision |
| PD.2 audit-ledger module-id support | blocked on project-slm PS.4 endpoints |
| Four TOPIC bulk drafts (wiki: leg planned_topics) | awaiting project-language sweep of existing drafts first |
| README refresh for four Ring 1 services | sub-agent brief candidate; no urgency |
| tool-acs-miner/src/main.rs:32 reference | Master tracking as Option A (Root-tier pickup) |

---

Cluster is idle after this commit. No active sub-agents. Awaiting Master
direction on items 2–4 before resuming implementation work.
