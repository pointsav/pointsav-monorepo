---
schema: foundry-draft-v1
artifact: JOURNAL-NOTES
journal: j6
journal_title: "Muscle-Memory-Preserving Desktop Environments for Professional AEC Software Migration"
section: "§3 Design Principles + §4 Implementation — keyboard power moves (2026-06-02)"
state: draft-pending-editorial-review
originating_cluster: project-workplace
created: 2026-06-02
to: project-editorial
language_protocol: PROSE-TOPIC
bcsc_class: current-fact
research_trail:
  source_files:
    - app-workplace-http-prototype/src/assets/memo.html
    - app-workplace-http-prototype/src/assets/workbench/index.html
    - app-workplace-http-prototype/src/assets/proforma.html
  commit: 6ae5e97c
  notes: >
    Phase 1+2 Leapfrog 2030 implementation. All shortcuts verified live
    at http://10.8.0.9:9200 (nginx proxy, prototype port 9110).
    Build: CARGO_TARGET_DIR=/srv/foundry/cargo-target/jennifer cargo build --release
---

# JOURNAL-NOTES — J6 §3 Design Principles + §4 Implementation
## Keyboard Power Moves — 2026-06-02

**Routing:** project-editorial → J6 §3 Design Principles + §4 Implementation

---

## Memo surface — Word muscle memory

`Ctrl+B`, `Ctrl+I`, `Ctrl+U` wire to `document.execCommand('bold')`, `execCommand('italic')`,
`execCommand('underline')` via the existing `fmt()` helper. An editor-focus guard
(`editor === document.activeElement || editor.contains(document.activeElement)`) prevents
the shortcut from firing when focus is outside the document area. These three bindings are
identical to Microsoft Word's default keymap, unchanged since Word 1.0. No retraining is
required for existing Word users — the muscle memory transfers unconditionally.

`Ctrl+Alt+1`, `Ctrl+Alt+2`, `Ctrl+Alt+3`, `Ctrl+Alt+0` apply H1, H2, H3, and Normal
paragraph formatting respectively via the existing `fmtBlock()` helper. These match Word's
heading shortcuts exactly: Ctrl+Alt+N applies Heading N style in Word's default keymap,
and Ctrl+Alt+0 applies Normal. This is the primary structural formatting shortcut path
for Word users working with multi-level document outlines — a workflow common in AEC
specifications, design reports, and project briefs.

## Workbench surface switching — custom position-based convention

`Ctrl+1` through `Ctrl+9` switch the active workbench surface. The mapping is positional
and matches the visual order of the surface toolbar: Files=1, Memo=2, Proforma=3,
Presentation=4, Schedule=5, Code=6, PDF=7, GIS=8, BIM=9. The binding fires
`window.addEventListener('keydown')` and calls `.click()` on the corresponding toolbar button.

Position-based tab switching is a convention in terminal multiplexers (tmux Ctrl+B N,
iTerm2 Cmd+N, GNU Screen Ctrl+A N) and browser environments (Ctrl+N in most browsers
selects tab N). The binding is learnable without a reference card because the tab order
is permanently visible in the toolbar. No app-specific shortcut legend is required.

The workbench toolbar shows all 9 surfaces in a fixed left-to-right order; the Ctrl+N
shortcut collapses to the same spatial relationship. This is the pattern J6 §3 should
cite as the "positional indexing" design principle for multi-surface environments.

## Proforma surface — Excel muscle memory

`Ctrl+Z` and `Ctrl+Y` invoke a 50-entry ring buffer undo/redo system. A `snapshot()`
function serialises the full grid state (data + formulas) to a plain object; `pushUndo()`
is called before every cell mutation and trims the redo branch before appending. `undo()`
steps the pointer back if `undoPtr > 0`; `redo()` steps forward if the pointer is not
at the stack head.

Undo/redo at these key bindings is universal across Excel, Google Sheets, LibreOffice Calc,
and every modern spreadsheet. It is a non-negotiable expectation for a spreadsheet surface.
The 50-entry buffer is consistent with Excel's default undo history depth.

`Ctrl+C` captures the current cell's value and formula into an in-memory clipboard object.
`Ctrl+V` pastes to the active cell. The current implementation handles single-cell clipboard;
multi-cell range clipboard is deferred. The binding matches Excel's copy/paste shortcut
unconditionally.

`Ctrl+D` fills the current cell's formula or value down to the selected range. Relative
cell references are rewritten on fill: the pattern `([A-Z]+)(\d+)` matches A1-style column+row
references and the row number is incremented by the fill distance (fill-down of one row
increments by 1, i.e. A1 → A2). This matches Excel's Ctrl+D fill-down behaviour for
relative references, which is the dominant spreadsheet convention.

## Design rationale — zero-retraining principle

The shortcut selections across all three surfaces apply a single governing principle:
match the dominant existing application in each domain rather than introducing a custom
convention. Memo binds to Word. Proforma binds to Excel. The one genuinely custom
convention — Ctrl+1..9 surface switching — borrows from terminal-multiplexer conventions
already familiar to the target audience of professional technical users.

The zero-retraining principle differs from the approach taken by web-based alternatives,
which typically introduce custom shortcut schemes optimised for discoverability rather than
retention. A workbench used daily for years benefits more from retention than discoverability:
the user will never need to discover the shortcut again after the first day, but will execute
it hundreds of times per week thereafter.

J6 §3 should present this as a falsifiable design claim: applications that match existing
dominant shortcuts produce measurably lower error rates and time-to-task than applications
that introduce custom shortcut schemes, in a within-subjects comparison with experienced users.
The prototype is the implementation vehicle for testing this claim in the §6 Results study.
