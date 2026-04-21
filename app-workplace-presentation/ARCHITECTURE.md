# ARCHITECTURE.md — Workplace✦Presentation

> Architecture decisions, dependency audit, and the rationale behind each.
> ADRs are commitments to decisions already made — not proposals.

---

## 1. Dependency audit

| Layer | Dependency | Version | Licence | Jurisdiction | Rationale |
|---|---|---|---|---|---|
| Language | Rust | 1.95 | MIT / Apache 2.0 | — | Same as memo, proforma |
| Desktop framework | Tauri | 1.7 | MIT / Apache 2.0 | Netherlands | EU-governed, small footprint |
| Pagination | Paged.js | latest | MIT | — | Same library memo uses for print |
| Fonts | SIL Open Font Licence families | — | OFL | — | 8 families, same set as memo |
| Crypto | Web Crypto API | browser-native | W3C | — | SHA-256 seal |
| App licence | EUPL v1.2 | — | — | European Commission | Standing family licence |

No network dependencies. `connect-src 'none'` enforced via CSP. No analytics. No telemetry. No account system. No auto-updater that reaches external servers.

---

## 2. ADRs — Architecture Decision Records

### ADR-PR-01 — Tauri v1, not v2

**Context:** Tauri v2 is current; Tauri v1 is in maintenance mode.

**Decision:** The workplace family stays on Tauri v1.7 until all three apps can migrate together.

**Rationale:** The owner's iMac runs macOS 10.13 High Sierra. Tauri v2 requires 10.15+. Piecemeal migration creates a version skew across sibling apps that share IPC patterns and build scripts. Coordinated migration is a one-time cost.

**Consequences:** Cannot use v2-specific features (new mobile targets, updated API surface). Linux Mint requires a pkg-config shim for webkit 4.0→4.1 (already in place, documented in `DEVELOPMENT.md`).

---

### ADR-PR-02 — Single-file `.html` as native format

**Context:** PowerPoint uses `.pptx` (an Open XML zip with multiple files). Keynote uses a proprietary bundle. Google Slides uses cloud storage.

**Decision:** The native file format is a single `.html` file containing DOCTYPE, metadata in a `<meta>` tag, inlined CSS, base64-embedded fonts, inlined slideshow runtime, and one `<section>` per slide.

**Rationale:**
1. The file opens standalone in any browser as a runnable slideshow — no accompanying files, no application required.
2. HTML is universally readable by text editor on any computer in fifty years.
3. The user visibly owns their file. It is not a proprietary container.
4. Single-file means no broken-link failure modes (missing media, missing fonts, missing runtime).

**Consequences:** File size is larger than a `.pptx` (fonts are embedded). For a 20-slide deck with 8 fonts, expect 2–6 MB. This is acceptable — the tradeoff buys portability.

---

### ADR-PR-03 — Blur-driven sync between canvas and code view

**Context:** Two panes representing the same underlying data can sync on every keystroke or on blur.

**Decision:** Sync on blur only.

**Rationale:** Keystroke-level sync creates cursor thrash — typing in one pane causes the other to re-render, which can steal focus or jump the cursor. Blur-driven sync matches how Word, Excel, and IDE split panes behave. Users already expect it.

**Consequences:** If a user types in one pane and closes the app without blurring the field, the most recent changes may not be reflected in the saved file. Mitigation: the editor listens for the window `beforeunload` event and forces a blur on any focused editable element.

---

### ADR-PR-04 — Slideshow runtime embedded in saved files

**Context:** A saved `.html` file must run as a slideshow when opened in a browser — no editor available.

**Decision:** `src/js/slideshow.js` is a self-contained runtime (~100 lines, zero dependencies) that is (a) loaded by the editor for F5 mode and (b) inlined into every saved file by `export.js`.

**Rationale:** One runtime, two usage sites. No code duplication. A saved file from today runs in a browser from the year 2050 without any PointSav software available.

**Consequences:** The runtime cannot use any editor state or IPC. It operates purely on DOM manipulation and keyboard events. Any feature that requires Rust IPC (e.g. file I/O) cannot be part of the runtime.

---

### ADR-PR-05 — Four IPC commands only

**Context:** Temptation exists to add Rust commands for every native operation (file watching, OS shell integration, printer enumeration).

**Decision:** The Rust layer exposes exactly four commands, copied unchanged from memo: `open_file`, `save_file`, `get_app_data_dir`, `read_font_file`. Adding a fifth requires an ADR.

**Rationale:** The IPC surface is the attack surface. Keeping it tiny and reviewed keeps the security boundary clear. Almost everything the frontend needs is available from the browser environment directly.

**Consequences:** If a feature genuinely requires new Rust functionality, it needs architecture review before implementation. Do not silently expand the IPC surface.

---

### ADR-PR-06 — Split code view is scoped to the active slide

**Context:** The code view could show the entire deck's HTML or only the active slide's.

**Decision:** Active slide only.

**Rationale:** Showing the whole deck produces an unwieldy text area (potentially thousands of lines). Scoped-to-active-slide keeps the view focused and fast to sync. It also maps cleanly to the slide-as-discrete-unit mental model.

**Consequences:** Users cannot directly edit the document-level structure (metadata, global CSS) through the code view. Metadata edits happen through File → Properties; global CSS is not currently user-editable (deferred to post-Phase-7 backlog).

---

### ADR-PR-07 — Editor state lives in app data, not in the file

**Context:** Memo bundles document content only. Proforma bundles document content only plus an audit hash. The question: what about current slide index, split pane open/closed, zoom level?

**Decision:** None of those go in the file. The file is the user's. Editor state is the editor's.

**Rationale:** When a user shares a `.html` file with a colleague, the colleague should not inherit the sender's split-pane-open preference or current-slide index. The file represents the deck, not the session. Editor state lives in the Tauri app data directory.

**Consequences:** Opening a file always starts at slide 1 with the default pane layout. Users can re-open their last-edited file with their last position via a "recent files" feature (not yet implemented — deferred).

---

### ADR-PR-08 — No framework, no build step for the frontend

**Context:** React, Vue, Svelte, Solid would provide component models.

**Decision:** Vanilla JS. Plain HTML/CSS served directly by Tauri from `src/`. No webpack, vite, rollup, or esbuild for the frontend.

**Rationale:** The frontend is modest (a few thousand lines). A framework adds a build step, a lockfile, and a dependency tree that must be audited for every update. Vanilla JS runs in the browser as-is, is trivially auditable, and cannot drift from what the developer wrote.

**Consequences:** Some patterns (virtual DOM diffing, reactive state) must be implemented by hand or avoided. The pattern chosen is: a plain document object held in `editor.js`, with explicit re-render calls after mutations. No reactivity layer.

---

### ADR-PR-09 — US Letter landscape, not 16:9

**Context:** PowerPoint defaults to 16:9 widescreen slides since 2013 (matching projectors and monitors). US Letter landscape (11″ × 8.5″, aspect 1.294:1) is closer to 4:3 than to 16:9. The question is which to use as the default slide size in this editor.

**Decision:** Default slide size is **US Letter landscape: 11″ × 8.5″** (logical canvas 1100 × 850). Not 16:9. No 16:9 option at ship — added post-Phase-7 if requested.

**Rationale:** The target audience is bankers, asset managers, and institutional investors. That audience prints every deck on US Letter paper. Board materials are printed. Investor handouts are printed. The printed artefact is the primary consumption mode; projector display is secondary. A 16:9 slide printed on US Letter leaves awkward white bands at top and bottom — wasted paper real estate on material that is already information-dense. US Letter landscape uses the full page natively.

This choice is also consistent with the file-is-the-product principle: the file the user saves, the file they print, and the file they project should all render the same geometry. Avoiding aspect-ratio transformation at print time means the print output is a direct 1:1 render of the canvas — no reflow, no scaling, no surprises.

**Consequences:**
- Projection on a modern 16:9 display produces black bars on the left and right (~18% of horizontal space). Acceptable — the deck is built for paper first.
- Canvas logical units (1100 × 850) are not the "standard" 960×540 or 1920×1080 that web developers expect. Document this clearly in `canvas.js` comments.
- If the product later targets a different institutional audience (e.g., European enterprise sales where A4 is the paper default), a second aspect-ratio option (A4 landscape, 297×210mm, 1.414:1) may become the priority over 16:9.
- Phase 7 print pipeline renders 1:1 with no aspect transformation — simpler implementation than the equivalent 16:9→US-Letter conversion would require.

---

## 3. The file-is-the-product principle

This is the single most important principle across the workplace family. Every design choice defers to it.

What this means concretely:
- Opening a saved file in a text editor must produce something a human can read and understand.
- Opening a saved file in any browser (Firefox, Chrome, Safari, Edge) must produce a working slideshow.
- Opening a saved file in fifty years, when this app no longer exists, must still produce a readable document.
- The file has no external dependencies. No CDN links. No font URLs. No script sources.
- The file has no proprietary binary regions. No base64-encoded blobs the user cannot inspect.

The only base64 in the file is font data inside `@font-face` rules — a standard CSS pattern universally understood by browsers since 2015.

---

## 4. Comparison with sibling apps

| Concern | Memo | Proforma | Presentation |
|---|---|---|---|
| Output format | `.html` (single file) | `.json` | `.html` (single file) |
| Canvas type | Continuous flowing document | 26×50 cell grid | Discrete US Letter landscape slides |
| Canvas geometry | A4/Letter portrait (flowing) | 26 columns × 50 rows | 11″ × 8.5″ landscape (1100×850 logical) |
| Split code pane | No (dropped in early design) | No | **Yes** — unique to this app |
| Fullscreen mode | Print preview | None | F5 slideshow |
| Print pipeline | Paged.js portrait | None yet | Paged.js 1:1 US Letter landscape |
| Dirty format detection | Meta tag + SHA-256 | Meta field + SHA-256 | Meta tag + SHA-256 |
| Frontend build step | None | None | None |
| Framework | Vanilla JS | Vanilla JS | Vanilla JS |

The code-view pane is the only genuine UX divergence. Everything else is stylistic variation on shared patterns.

---

## 5. What is not in this app

Deliberate omissions, for clarity:

- No account system. No login.
- No cloud sync. No "open from cloud".
- No telemetry. No analytics. No crash reporter phoning home.
- No auto-updater. Updates are manual via GitHub releases.
- No AI assistance in the editor. (Platform-level SLM is a separate concern, not relevant to this app.)
- No collaborative editing. Single user, single machine.
- No import from `.pptx`, `.key`, or Google Slides in the shipping plan. Deferred to post-Phase-7 backlog.
- No presentation templates gallery at startup. Blank canvas only.
