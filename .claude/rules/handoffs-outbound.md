# Handoffs Outbound — pointsav-monorepo

Pending cross-repo file moves. Each entry is a file that belongs in
another repo per `repo-layout.md`, left in place here until a Root
Claude started in the destination repo commits the add-side.

Pattern: see `~/Foundry/CLAUDE.md` §9 (cross-repo handoff addendum
pending — surfaced for Master Claude in `cleanup-log.md` 2026-04-23
entry).

Maintained by Root Claude in this repo. Read by Master Claude
during workspace review, and by a Root Claude started in the
destination repo when it comes time to commit the add-side. An
outbox entry is passive state: it describes intent, but nothing
moves until Master Claude, a Root Claude, or a Task Claude picks
it up.

---

## GUIDE-OPERATIONS.md → content-wiki-documentation

- **Source path:** `GUIDE-OPERATIONS.md` (this repo root)
- **Destination repo:** `content-wiki-documentation`
- **Destination path:** `GUIDE-OPERATIONS.md` (destination root)
- **Rationale:** Operations guide is cross-cutting documentation;
  per `repo-layout.md` sibling-repo table, documentation of this
  kind belongs in `content-wiki-documentation`. Not allowed at the
  monorepo root.
- **State:** `pending-destination-commit`
- **Opened:** 2026-04-23 by Root Claude
- **Notes:** Plain move — no rename, no content change.

---

## USER_GUIDE_2026-03-30_V2.md → content-wiki-documentation

- **Source path:** `USER_GUIDE_2026-03-30_V2.md` (this repo root)
- **Destination repo:** `content-wiki-documentation`
- **Destination path:** `USER_GUIDE_2026-03-30.md` (destination
  root) — `_V2` dropped in transit per CLAUDE.md §6 edit-in-place
  rule.
- **Rationale:** User-facing documentation plus `_V2` filename
  violation; per `repo-layout.md` sibling-repo table, user guides
  belong in `content-wiki-documentation`. Not allowed at the
  monorepo root.
- **State:** `pending-destination-commit`
- **Opened:** 2026-04-23 by Root Claude
- **Notes:** Rename on transit — destination filename drops the
  `_V2` suffix. BCSC-language review of User Guide content
  (Sovereign Data Foundation treated as "current equity holder /
  active auditor") is a separate open question in `cleanup-log.md`;
  a Root Claude in the destination repo should flag the content as
  pending review in its own `cleanup-log.md` at add-side commit
  time.
