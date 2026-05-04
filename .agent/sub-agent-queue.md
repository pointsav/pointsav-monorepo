---
queue: cluster-scope sub-agent briefs
owner: task-project-data
location: ~/Foundry/clones/project-data/.claude/
schema: foundry-sub-agent-queue-v1
---

# Sub-agent queue — project-data cluster (cluster-scope)

Cluster-scope sub-agent briefs ratified by Master per v0.1.30 §1A.
Master-scope briefs go to `~/Foundry/.claude/sub-agent-queue.md`.

Each brief is dispatched by this Task session via the `Agent` tool
with `subagent_type: "general-purpose"`, `model: "sonnet"`,
foreground + serial when writing (git-index race per §1A.2).
Per §1A.6: parent Task reviews + commits the result; never
delegates the commit decision.

After dispatch + commit, the brief moves to the **Completed** section
at the bottom of this file (preserves audit trail).

---

## Pending

*(none)*

---

## Completed

### SI.1 — service-input happy-path tests for DOCX + XLSX parsers

- **Status:** completed 2026-05-04 in commit `62403f1`. Brief preserved verbatim below for audit trail.
- **Sub-agent:** Sonnet (general-purpose), dispatched by prior Claude Code session (2026-04-28); staged files reviewed by parent before commit; commit made by this Task session (2026-05-04) after 32/32 test pass confirmed.
- **Outcome:** 2 test functions added (`docx.rs::tests::happy_path_minimal_docx_parses` + `xlsx.rs::tests::happy_path_minimal_xlsx_parses`); 2 binary OOXML fixtures staged (`minimal.docx` 943 bytes, `minimal.xlsx` 1904 bytes). `cargo test -p service-input` 32/32 pass.

**(brief preserved below)**

### SI.1 — service-input happy-path tests for DOCX + XLSX parsers (original brief)

- **Status:** ratified by Master v0.1.42 (in the broader fixtures-and-tests remit) + sub-agent dispatch authorized 2026-04-28 by operator green-light
- **Cluster:** project-data
- **Branch:** cluster/project-data
- **Tier:** Sonnet (mechanical test writing)
- **Foreground / serial:** required (writes git index — though tests-only, no library code change)
- **Cap:** report under 200 words

**Context (self-contained):**

`service-input` has 30 unit tests today. PDF has both error-path
tests + a happy-path test using a hand-crafted minimal.pdf
fixture. Markdown already has 5 tests including 3 happy-path
tests using inline string fixtures (no file fixtures needed).
**DOCX and XLSX have only error-path tests (2 each); they need
happy-path tests against file fixtures.**

The parent Task has **already generated the fixtures**:
- `service-input/tests/fixtures/minimal.docx` (943 bytes; one
  paragraph containing exactly the text "Hello DOCX World")
- `service-input/tests/fixtures/minimal.xlsx` (1904 bytes; one
  sheet named "Sheet1" with cell A1 containing the shared string
  "Hello XLSX World")

Both are valid OOXML (hand-crafted minimal Open Packaging
Convention ZIPs via Python stdlib zipfile). Parser shapes:

- `DocxParser::parse` returns `ParsedDocument` with `text` =
  concatenated body text + metadata = `{paragraph_count: <n>,
  parser: "docx-rust"}`.
- `XlsxParser::parse` returns `ParsedDocument` with `text` =
  rows-newline-separated cells-space-separated + metadata =
  `{sheet_count: <n>, sheets: [<names>], parser: "calamine"}`.

**Steps (execute in order; commit ALL in one commit at the end —
but do NOT run the commit yourself; parent Task does):**

1. Read the existing happy-path test pattern at
   `service-input/src/pdf.rs` `happy_path_minimal_pdf_parses`.
   Use this as the reference shape.

2. In `service-input/src/docx.rs` `tests` mod, add a new test
   `happy_path_minimal_docx_parses`:
   - Use `include_bytes!("../tests/fixtures/minimal.docx")` to load the fixture.
   - Construct `DocxParser` and call `.parse("fixture-1", bytes)`.
   - Assert the result is `Ok(doc)`.
   - Assert `doc.text.contains("Hello DOCX World")` (the known
     fixture content).
   - Assert `doc.metadata["paragraph_count"].as_u64() >= 1`.

3. In `service-input/src/xlsx.rs` `tests` mod, add a new test
   `happy_path_minimal_xlsx_parses`:
   - Use `include_bytes!("../tests/fixtures/minimal.xlsx")`.
   - Construct `XlsxParser` and call `.parse("fixture-1", bytes)`.
   - Assert result is `Ok(doc)`.
   - Assert `doc.text.contains("Hello XLSX World")`.
   - Assert `doc.metadata["sheet_count"].as_u64() == 1`.
   - Assert `doc.metadata["sheets"][0] == "Sheet1"`.

4. **DO NOT** add any markdown happy-path tests — markdown
   already has happy-path coverage via inline strings (5 tests
   in `service-input/src/markdown.rs`). If the brief seems to
   imply markdown work, IGNORE it — markdown is done.

5. Run `cargo test -p service-input` — all tests must pass
   (existing 30 + new 2 = 32 expected). If any fail, STOP and
   report — do not modify the parser source code or the
   fixtures to make tests pass.

6. **Do NOT commit yourself.** Stage all changes (`git add` the
   two new test additions; the fixtures are already in the
   working tree but untracked — `git add` them too). Parent Task
   reviews + commits via `~/Foundry/bin/commit-as-next.sh`.

**Deliverable (cap 200 words):**

- One-line confirmation each step completed (or noted as failed)
- `cargo test -p service-input` final pass count
- Names of files staged

**Anti-slop:** if the fixtures don't parse cleanly (DOCX or XLSX
returns an error), STOP and report the exact error — DO NOT modify
the fixtures or parser code to make them pass. Parent Task will
investigate. The brief is to verify the fixtures work; if they
don't, that's a parent-side fixture-generation bug.

---

### PD.4 — sovereign-acs-engine → people-acs-engine directory rename

- **Status:** completed 2026-04-28 in commit `b3e4bb5`. Brief preserved verbatim below for audit trail.
- **Sub-agent:** Sonnet (general-purpose), dispatched via Agent tool, foreground + serial. Returned in ~9 minutes (25 tool uses). Sub-agent staged all changes; parent Task reviewed diff + cargo check --workspace clean + committed via `~/Foundry/bin/commit-as-next.sh`.
- **Outcome:** 3 file renames (Cargo.lock + Cargo.toml + src/main.rs preserved through git rename detection) + 5 content edits in CLAUDE.md / NEXT.md / src/person.rs / schema/DESIGN.md / people-acs-engine/src/main.rs eprintln usage string. tool-acs-miner/src/main.rs:32 out-of-cluster reference deliberately deferred (outboxed to Master for routing).

**(brief preserved below)**

### PD.4 — sovereign-acs-engine → people-acs-engine directory rename (original brief)

- **Status:** ratified by Master v0.1.33 (2026-04-27); dispatch green-lit by Master v0.1.43 (2026-04-28)
- **Cluster:** project-data
- **Branch:** cluster/project-data
- **Tier:** Sonnet (mechanical edits)
- **Foreground / serial:** required (writes git index)
- **Cap:** report under 200 words

**Context (self-contained):**

The directory `service-people/sovereign-acs-engine/` is a Rust
binary that does email-regex + UUIDv5 deterministic identity
anchoring. The Cargo `name` field was already updated from
`sovereign-acs-engine` → `people-acs-engine` per the Do-Not-Use
"sovereign" prefix discipline, but the **directory name** and
several in-repo references still use the old name. Close that
gap.

**Steps (execute in order; commit ALL in one commit at the end —
but do NOT run the commit yourself; parent Task does):**

1. `git mv service-people/sovereign-acs-engine service-people/people-acs-engine`
2. Update the eprintln Usage string in
   `service-people/people-acs-engine/src/main.rs:33` —
   change `sovereign-acs-engine` to `people-acs-engine`.
3. Edit `service-people/CLAUDE.md` — update three references on
   lines 35, 47, 72 (also the file-layout box that names the
   directory). Drop in-line `sovereign-acs-engine/` everywhere.
4. Edit `service-people/NEXT.md` line 85 — drop the
   `sovereign-acs-engine` reference; the rename is the closure.
5. Edit `service-people/src/person.rs:11` doc-comment — change
   `sovereign-acs-engine/` to `people-acs-engine/`.
6. Edit `service-people/schema/DESIGN.md:35` — change
   `Inherited from sovereign-acs-engine/ (now people-acs-engine/)`
   to just `Inherited from people-acs-engine/`.
7. **Out-of-cluster reference (DO NOT touch):** The string
   `sovereign-acs-engine` also appears in
   `tool-acs-miner/src/main.rs:32` (eprintln Usage). `tool-*`
   is outside cluster project-data scope per
   `~/Foundry/clones/project-data/.claude/manifest.md`. **Leave
   it as-is.** Will be surfaced as an outbox item to Master for
   routing to the appropriate Root or other Task session.
8. **Do NOT touch** the cluster cleanup-log mentions
   (`.claude/rules/cleanup-log.md` lines around 265, 297, 839)
   or the cluster manifest line ~86 — those are historical notes
   about the rename being queued; leave as historical.
9. **Do NOT commit yourself.** Parent Task runs the commit via
   `~/Foundry/bin/commit-as-next.sh` after reviewing the
   diff. Stage all changes (`git add` the rename + the 5 file
   edits) but stop before commit.
10. Verify `cargo check --workspace` passes clean (the directory
    is not a workspace member, but the workspace lockfile may
    update — leave any Cargo.lock changes staged for parent to
    commit too).

**Deliverable (cap 200 words):**

- One-line confirmation each step completed (or noted as skipped/
  failed)
- `cargo check --workspace` final status
- One outbox-suggestion line for Master regarding the
  out-of-cluster `tool-acs-miner` reference

**Anti-slop:** if any step fails or the directory rename hits
unexpected file references not listed above, STOP and report —
do not improvise.

---

## Completed

*(none yet)*
