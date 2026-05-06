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

Each entry carries a **Prescriptive actions** subsection — exact
commands the destination Root Claude can execute mechanically
without needing context from this session. A destination Root
Claude starting cold in the target repo can read this file, find
its repo name in the destination field, and run the listed
commands. Completion is signalled by a commit whose message
matches the pattern named in the entry's closing step; a future
Root Claude session in this repo grep-detects that pattern in the
destination repo's `git log` and then closes the outbox entry
with the source-side `git rm` commit.

---

## guide-operations.md → content-wiki-documentation

- **Source path:** `guide-operations.md` (pointsav-monorepo repo root)
- **Destination repo:** `content-wiki-documentation`
- **Destination path:** `guide-operations.md` (destination repo root)
- **Rationale:** Operations guide is cross-cutting documentation;
  per `repo-layout.md` sibling-repo table, documentation of this
  kind belongs in `content-wiki-documentation`. Not allowed at the
  monorepo root.
- **State:** `pending-destination-commit`
- **Opened:** 2026-04-23 by Root Claude
- **Notes:** Plain move — no rename, no content change.

### Prescriptive actions for a Root Claude in content-wiki-documentation

Run these from working directory
`/home/mathew/Foundry/factory-pointsav/content-wiki-documentation/`.

1. Copy the source file into the destination repo:
   ```
   cp /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/guide-operations.md ./guide-operations.md
   ```

2. Stage it:
   ```
   git add guide-operations.md
   ```

3. Commit via the staging-tier helper:
   ```
   ~/Foundry/tool-commit-as-next.sh "receive guide-operations.md from pointsav-monorepo — operations-guide relocation per repo-layout sibling-repo table"
   ```

4. No in-transit content edits. No callers in this repo. No
   registry updates needed (this repo has no project registry;
   content-wiki repos are flat collections of topics).

5. When complete, the source-side `git rm` will be committed in a
   future Root Claude session in pointsav-monorepo. That session
   detects completion by grepping this repo's `git log` for the
   commit-message pattern `"receive guide-operations.md from
   pointsav-monorepo"`.

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
- **In-transit edit 2026-04-23:** line 902 updated to reflect the
  `vendors-maxmind` → `app-mediakit-telemetry/assets/` reclass
  that happened in the monorepo the same day. The destination
  Root Claude will receive the corrected path; no separate
  follow-up needed on that line.

### Prescriptive actions for a Root Claude in content-wiki-documentation

Run these from working directory
`/home/mathew/Foundry/factory-pointsav/content-wiki-documentation/`.

1. Copy the source file into the destination repo **with the
   `_V2` suffix dropped** (rename-on-transit per CLAUDE.md §6
   edit-in-place rule):
   ```
   cp /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/USER_GUIDE_2026-03-30_V2.md ./USER_GUIDE_2026-03-30.md
   ```

2. Stage it:
   ```
   git add USER_GUIDE_2026-03-30.md
   ```

3. Commit via the staging-tier helper:
   ```
   ~/Foundry/tool-commit-as-next.sh "receive USER_GUIDE_2026-03-30 from pointsav-monorepo — user-guide relocation; _V2 suffix dropped per CLAUDE.md §6 edit-in-place rule"
   ```

4. In-transit content edits have already been applied in the
   source repo (line 902 path reference updated 2026-04-23). No
   further edits needed at receipt.

5. **Flag: BCSC disclosure review required before any public
   reuse.** The User Guide contains language treating the
   Sovereign Data Foundation as a current equity holder and
   active auditor. Per CLAUDE.md §6 BCSC disclosure rule, the
   Foundation must be referred to in planned / intended terms
   only. If this repo has a `NEXT.md` or `.agent/rules/cleanup-log.md`,
   log an entry flagging that a language-review pass is required
   before any public reuse of this document. **Do not perform
   the review in this commit** — only flag. The review is a
   separate editorial task.

6. When complete, the source-side `git rm` will be committed in
   a future Root Claude session in pointsav-monorepo. That
   session detects completion by grepping this repo's `git log`
   for the commit-message pattern `"receive USER_GUIDE_2026-03-30
   from pointsav-monorepo"`.

---

## BIM material → content-wiki-documentation

**Pattern variant: raw-material handover, not a file move.** Source
files remain in this repo permanently — they are project-framework
artefacts (`CLAUDE.md`, `RESEARCH.md` per BIM project, plus a
`.claude/rules/` rule file). The destination Root Claude transforms
this material into proper wiki topics per its own repo-layout
conventions. Both repos keep their copies. No source-side `git rm`
at closure.

- **Source files (this repo):**
  - `.claude/rules/bim-product-family.md` (9,238 B) — product family
    rules, naming constraints, IFC format decisions, licence split
  - `app-console-bim/CLAUDE.md` (3,064 B) — console BIM project card
  - `app-console-bim/RESEARCH.md` (5,311 B) — joint orchestration+
    console research, copy 1 (intentional duplicate — Task Claude —
    BIM will rationalise in the monorepo; the destination may treat
    the two copies as one document)
  - `app-orchestration-bim/CLAUDE.md` (3,018 B)
  - `app-orchestration-bim/RESEARCH.md` (5,311 B) — identical to the
    app-console-bim copy
  - `app-workplace-bim/CLAUDE.md` (4,742 B)
  - `app-workplace-bim/RESEARCH.md` (20,489 B) — workplace-specific
    research (editor product, AutoCAD / Revit muscle memory)
  - `service-bim/CLAUDE.md` (2,179 B) — archive daemon project card
  - `service-bim/RESEARCH.md` (4,386 B) — archive daemon research
- **Related file already in destination:** `RESEARCH-BIM-MARKET.md`
  at destination repo root (50,515 B, landed 2026-04-22 23:45).
  Treat as input alongside the source files above.
- **Destination repo:** `content-wiki-documentation`
- **Destination paths:** TBD by destination Root Claude per its own
  repo-layout. The internal references in the source material point
  at `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`,
  suggesting a `research/` subfolder is anticipated.
- **Rationale:** BIM product family is new surface area (four project
  directories plus rules extension). Corresponding wiki topics should
  be built from this material.
- **State:** `pending-destination-commit`
- **Opened:** 2026-04-23 by Root Claude
- **Notes:** Deliberately generous — destination is expected to pare
  back, not to cover every source sentence. No per-file 1:1 mapping
  required; the destination decides topic shape and count.

### Prescriptive actions for a Root Claude in content-wiki-documentation

Run from working directory
`/home/mathew/Foundry/vendor/content-wiki-documentation/`.

1. Read the full source set before drafting topics:
   ```
   less /home/mathew/Foundry/vendor/pointsav-monorepo/.claude/rules/bim-product-family.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/app-console-bim/CLAUDE.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/app-console-bim/RESEARCH.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/app-orchestration-bim/CLAUDE.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/app-workplace-bim/CLAUDE.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/app-workplace-bim/RESEARCH.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/service-bim/CLAUDE.md
   less /home/mathew/Foundry/vendor/pointsav-monorepo/service-bim/RESEARCH.md
   less ./RESEARCH-BIM-MARKET.md
   ```
   (The `app-orchestration-bim/RESEARCH.md` is byte-identical to
   `app-console-bim/RESEARCH.md`; reading one is enough.)

2. Draft topic files per this repo's conventions. Suggested themes
   from the material: product-family overview, per-component deep
   dives (four), IFC format and data-contract topic, licence-split
   topic (service-bim Apache 2.0 vs. app-*-bim proprietary), market
   context.

3. Commit each topic file via the staging-tier helper. Use the
   commit-message prefix `"receive BIM material from
   pointsav-monorepo"` on at least one commit in the series — this
   is the grep pattern a future Root Claude session in
   pointsav-monorepo will detect to close this outbox entry.
   Example:
   ```
   ~/Foundry/bin/commit-as-next.sh "receive BIM material from pointsav-monorepo — product-family topic"
   ```
   Subsequent commits in the same topicalization pass can use
   freeform messages.

4. Optional intra-repo cleanup (separate concern): the existing
   `RESEARCH-BIM-MARKET.md` at this repo's root may belong in a
   `research/` subfolder — the joint research file references it at
   `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`.
   Move decision is local to this repo's `repo-layout.md`.

5. BCSC disclosure check: none of the BIM source files appear to
   carry Sovereign Data Foundation language on quick scan, but a
   destination-side pass before public reuse is still appropriate.

6. No source-side `git rm` follows closure — source files persist.
   Closure of this outbox entry is recorded in
   pointsav-monorepo's `cleanup-log.md` with a reference to the
   detection-pattern commit, after which the entry moves from this
   file to a closed-handoffs section of the cleanup-log.
