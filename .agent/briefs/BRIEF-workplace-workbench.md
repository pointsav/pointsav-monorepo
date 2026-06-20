---
artifact: brief
schema: foundry-brief-v1
brief-id: project-workplace-workbench
title: BRIEF — app-workplace-workbench consolidation, Rust-owned clean-sheet rewrite
status: active
owner: project-workplace
created: 2026-06-13
updated: 2026-06-19
---

# BRIEF — app-workplace-workbench: consolidate, Rust→WASM rewrite, full-stack ownership

> Living source-of-truth for the workbench rewrite. Approved plan mirror:
> `/home/jennifer/.claude/plans/1-can-you-plan-peaceful-sundae.md`.
> Consolidation of `BRIEF-workplace-roadmap` + workbench parts of `BRIEF-workplace-architecture`
> into this BRIEF is **in progress** (see Work log / Carry-forward) — do not delete those yet.

## Context

`app-workplace-workbench` today is three overlapping things: a 59-line Tauri v1.7 WebView wrapper
(`app-workplace-workbench/`), a three-pane HTTP IDE (`app-privategit-workbench/`, ~1,575 lines Rust
+ a single **3,891-line vanilla-JS file**), and the multi-schema prototype the office actually uses
(`app-workplace-http-prototype/`, routes memo/proforma/presentation/schedule/code/pdf/gis/bim).
The office loves working across all schemas in one files/viewer/editor environment. It is held back
by: lossy text-match viewer↔editor sync (not Figma-grade), a non-virtualized file tree with
synchronous per-folder IPC (the "massive delay"), no AST-aware editing, no section-scoped AI, and a
monolithic untyped frontend with zero tests. Office's biggest complaint: AI has to redo whole files
top-to-bottom instead of working on a selected section.

## Scope

Consolidate the three into **one prototype named `app-workplace-workbench`**, browser + localhost,
for fast iteration with local staff. Clean-sheet rewrite on a **Rust document engine compiled to
WASM** (browser = thin shell). Bring Figma-style bidirectional WYSIWYG↔code editing, VS-Code-grade
file-tree navigation, section-scoped AI editing, and a per-schema framework. Track every third-party
dependency as a `moonshot-*` ownership target en route to a full-Rust stack.

## Decisions locked (operator jwoodfine, 2026-06-13)

1. **Consolidate all three → one prototype `app-workplace-workbench`**; browser/localhost for now.
2. **Future vision:** workbench becomes a first-class surface *inside `os-workplace`* — the OS as
   chassis, not a discrete app. Coherent with Option-(j) dissent ("retire launcher into os-workplace
   later"). Requires a DOCTRINE-AMENDMENT (flagged; Command Session ratification).
3. **Spine = Rust→WASM document engine.** Same core drives the browser prototype today and the
   OS-native surface tomorrow, engine unchanged. This is the bridge, not a fight, to os-workplace.
4. **Own the full stack over time** — *everything eventually*, including foundations
   (tokio/serde/axum) on a very-long-fuse horizon. Every dependency = tracked `moonshot-*` target.
5. **AI section-editing = MCP-bridge to an external session, plumbing first.** Workbench exposes the
   highlighted selection as an MCP server; operator points their *own* Claude Code session (their
   subscription) at it. ToS-compliant (Anthropic forbids embedding claude.ai login), model-agnostic,
   never loads the app. **Prose/text/code schemas only — SYS-ADR-07 bars proforma/schedule/gis.**
6. **First schema push = ALL eight schemas, BIM emphasized** (memo, code, proforma, presentation,
   schedule, pdf, gis, bim). BIM is heaviest (IFC/ISO 16739; web-ifc/xeokit licensing gate).
7. **Frontend = thin shell around the WASM core**, not a heavy JS framework (sovereignty + ownership).

## Decisions open

- Commit branch for the new monorepo crates: clone is on `main`; manifest expects
  `cluster/project-workplace`. **Operator decision pending** — blocks commit of the 5 moonshot crates.
- Editor foundation to *start* on before owning inward: Loro (CRDT) + tree-sitter (parse) +
  CodeMirror 6 (editor) are the vetted, forkable, MIT/Rust starting points; replace inside-out.
- Whether to spin a dedicated home repo later vs keep crates in monorepo (kept in monorepo for now).

## Dependency-ownership ledger (scope = everything, eventually)

| Dep | License | Role | Moonshot target | Horizon |
|---|---|---|---|---|
| ProseMirror/Lexical/TipTap | MIT | rich doc model | **moonshot-docengine** | own from day 1 (Rust) |
| tree-sitter | MIT | incremental parse | **moonshot-parser** | start on it, own inward |
| Loro / Yjs / Automerge | MIT | CRDT sync/versioning | **moonshot-crdt** | start on Loro, own inward |
| CodeMirror 6 / Monaco | MIT | editor widget | **moonshot-editor** | thin WASM editor core |
| react-arborist / react-window | MIT | tree virtualization | **moonshot-editor** (tree) | own as WASM tree |
| web-ifc / xeokit | MPL/commercial | IFC/BIM render | **moonshot-bim-engine** | own; resolves BIM gate |
| notify | CC0/MIT | fs watch | moonshot-network / own | tracked |
| tokio / serde / axum | MIT/Apache | runtime/HTTP | own (foundation) | tracked, very-long-fuse |

## os-workplace chassis vision (light research)

Same Rust/WASM engine surfaced natively inside `os-workplace` (niri-fork compositor + WebKitGTK per
BRIEF-workplace-desktop-environment), document-centric chassis, `connect-src 'none'` restored for the
native build, MBA/IPC contract per BRIEF-workplace-architecture §8/§9. The browser prototype relaxes
`connect-src 'none'` (roadmap BRIEF exemption) and is an explicitly-scoped dev/staging surface; the
native build re-imposes sovereignty. Stage a DOCTRINE-AMENDMENT (workbench-as-OS-surface).

## Clean-sheet rewrite architecture (phases)

- **P0 BRIEF consolidation** (this file) + ledger. *(in progress)*
- **P1 Moonshot folders** — 5 crates created (see Work log). *(done — committed 8412516b on main)*
- **P2 Rust/WASM core** — canonical doc model; AST-accurate source↔render mapping (replaces lossy
  text-match at `app-privategit-workbench/src/assets/index.html:1038-1138`); virtualized file tree
  + notify-driven SSE deltas (replaces O(n) DOM at `:2389-2394` and re-fetch-on-toggle); core file
  ops (save/duplicate/delete+restore/rename/move/create/download/export) ported + tested.
- **P3 AI MCP-bridge** — Rust MCP endpoint: `read_selection`, `propose_edit(range,new)`,
  `commit_edit`; guard rejects proforma/schedule/gis (SYS-ADR-07). Plumbing first, model second.
- **P4 Schema framework matrix** — per-schema open/save/edit/export/viewer-fidelity status; all 8,
  BIM emphasized.
- **P5 os-workplace chassis** — BRIEF section above + DOCTRINE-AMENDMENT draft.

**STATUS (2026-06-15) — plan NOT complete.** What exists is the owned engine *substrate*, not yet
wired into the running workbench:
- **P0** ✅ done (2026-06-19) — BRIEF-workplace-roadmap + BRIEF-workplace-architecture both marked `status: superseded` in sub-clone `.agent/briefs/`.
- **P1** ✅ done (8412516b).
- **P2** 🟡 *foundations only* — all five Rust cores built + tested (see Work log 2026-06-15), but
  **no WASM bindings, no thin frontend, and `app-privategit-workbench`/the 3,891-line monolith is NOT
  replaced**; file ops not reimplemented in a running prototype.
- **P3** ❌ not started (the `section_span` hook exists in docengine; no MCP bridge).
- **P4** ❌ not started (8-schema matrix).
- **P5** 🟡 chassis-vision section present; DOCTRINE-AMENDMENT draft not staged.
The biggest remaining gap is **integration** — nothing runs in the app yet; that step needs a
browser/the operator's machine to verify and cannot be validated headless.

Reuse from `app-privategit-workbench/src/main.rs`: atomic save w/ mtime-conflict, soft-trash+restore,
SSE via notify, git-status, server-side document render, PDF export (WeasyPrint).

## Schema framework matrix (to build out)

| Schema | App | Canonical fmt | AI-eligible | First-push status |
|---|---|---|---|---|
| memo | app-workplace-memo | .html | yes | active; Figma-WYSIWYG + AI demo vehicle |
| code | (code surface) | source files | yes | VS-Code analogue; file-tree perf proof |
| proforma | app-workplace-proforma | .json | NO (SYS-ADR-07) | active; structured showcase |
| presentation | app-workplace-presentation | .json+.html | yes(prose) | Wave 1; visual WYSIWYG |
| schedule | app-workplace-schedule | TaskJuggler DSL | NO | not started; egui Gantt (per arch BRIEF §9) |
| pdf | app-workplace-pdf | .pdf | n/a | scaffold; pdfium-render |
| gis | app-workplace-gis | .geojson | NO | scaffold; MapLibre/PMTiles |
| bim | app-workplace-bim | .ifc | n/a | **emphasized**; moonshot-bim-engine; highest risk |

## Work log

- **2026-06-13 (totebox@claude-code):**
  - Deep research (3 Opus Explore agents): full codebase audit of app-workplace-workbench /
    app-privategit-workbench / app-workplace-http-prototype; full BRIEF/state review; web research on
    (1) Figma-style bidirectional WYSIWYG↔code (Loro/ProseMirror/tree-sitter/source-maps),
    (2) VS-Code file-tree at scale (virtualization, lazy load, notify/FSEvents, Tauri IPC bottleneck),
    (3) driving Claude via subscription — **finding: Anthropic ToS forbids embedding claude.ai login;
    compliant pattern = app-as-MCP-server + operator's own Claude Code session.**
  - Plan written + operator-approved: `/home/jennifer/.claude/plans/1-can-you-plan-peaceful-sundae.md`.
  - **Created 5 moonshot crates** (Sovereign Replacement Initiative template, bilingual READMEs,
    `[workspace]`-table standalone like moonshot-toolkit, `src/lib.rs` scaffold): `moonshot-docengine`,
    `moonshot-parser`, `moonshot-crdt`, `moonshot-editor`, `moonshot-bim-engine`. **All 5 cargo build
    clean.** Registry rows added to `.agent/rules/project-registry.md` (Moonshot section, alphabetical).
  - **UNCOMMITTED** at end of 2026-06-13 — deferred pending operator branch decision.

- **2026-06-14 (totebox@claude-code):**
  - Resolved the branch question: clone topology has a root clone (governance, on `main`) + nested
    `pointsav-monorepo/` sub-clone (code, on `main`). Manifest's `cluster/project-workplace` is a stale
    pointer **1046 commits behind `main`** (fully merged) → committed to **`main`** in the sub-clone.
    Logged the manifest drift for Command.
  - **P1 committed:** 5 moonshot crates → `8412516b` on `main` (nested sub-clone). Registry rows in the
    root clone (`.agent/rules/project-registry.md`). Governance docs (BRIEF/NEXT/session-context) are
    gitignored in both repos post-`4c3a7c24` "Option A" — durable on disk, versioning gap flagged to Command.

- **2026-06-15 (totebox@claude-code):**
  - Actioned Command's BRIEF schema audit (off-plan): added `brief-id`/`owner`/`updated` to 10
    project-workplace BRIEFs + `schema:` to one; README active table now lists 4 active BRIEFs;
    flagged ~20 foreign contaminated BRIEFs (M-17) to Command.
  - **P2 foundations — all five cores built, tested, committed on `main` (sub-clone):**
    - `e327d669` docengine — source-anchored document model + `section_span` (AI handle); 7 tests.
    - `c3b27db4` editor — file-tree virtualization (lazy load + viewport windowing); 5 tests.
    - `ad26f48d` parser — UTF-8-safe span tokenizer; 7 tests.
    - `3d88c5a0` docengine — `Edit` + `apply` + **`remap_span`** incremental-update seam; → 11 tests.
    - `dc9dca21` parser — incremental retokenize (prefix reuse, full-equivalence proven); → 11 tests.
    - `6a9e65de` crdt — reversible `Op` + undo/redo + monotonic version lineage; 7 tests.
    - `d3207aa8` bim-engine — IFC-SPF structural parser (HEADER + DATA instances); 8 tests.
    - `601d6dc9` bim-engine — STEP value grammar (typed `Value` tree + `Instance::values`); → 12 tests.
  - **~46 tests across the four/five cores; all clippy+fmt clean.** All commits queued for Stage 6
    (outbox `project-workplace-20260614-workbench-moonshot-stage6`).
  - **Did NOT do:** WASM bindings, thin frontend, replacing the running workbench, P3/P4, doctrine amendment.

## Carry-forward

- [ ] **Stage 6 (Command):** promote `8412516b` + `e327d669` + `c3b27db4` + `ad26f48d` + `3d88c5a0` +
      `dc9dca21` + `6a9e65de` + `d3207aa8` + `601d6dc9` (all on `main`, nested sub-clone).
- [ ] **P2 back half — INTEGRATION (needs operator/browser to verify):** WASM-bind the cores
      (wasm-bindgen), build a thin frontend around them, and *replace* `app-privategit-workbench`'s
      monolith; reimplement file ops (save/duplicate/delete+restore/rename/move/create/download) end-to-end.
- [ ] **P3 AI MCP-bridge:** Rust MCP server exposing `section_span`-derived selection
      (`read_selection`/`propose_edit`/`commit_edit`); SYS-ADR-07 guard. Plumbing first.
- [ ] **P4 Schema matrix:** make all 8 schemas demonstrably function (BIM emphasized).
- [x] **P0 finish:** flip overlapping sections of BRIEF-workplace-roadmap + -architecture to
      superseded-by this BRIEF (do not delete). Done 2026-06-19.
- [ ] **P5:** stage the DOCTRINE-AMENDMENT draft (workbench-as-os-workplace-surface) → Command ratification.
- [ ] **Next pure-Rust layers (optional, verifiable here):** concurrent-merge sequence CRDT (crdt),
      IFC geometry/tessellation (bim-engine), suffix-reuse retokenize (parser).
- [ ] **Governance (Command):** manifest `cluster_branch` drift; `.agent/briefs` versioning gap (Option A).
- [ ] Pre-existing inbox blocker (prior session): Stage 6 fmt + E0432 (tool-proforma-engine:370).
