# NEXT.md — What to do right now

> The live action list. Update at the end of every session. Newest items at top.

---

## Immediate — Phase 1: Fork from memo

Claude Code's first session should fork `app-workplace-memo` into this repo without introducing anything presentation-specific. Verify the forked-but-inert shell compiles and launches before any presentation code lands.

### Paste this into Claude Code to start Phase 1:

```
Read CLAUDE.md, ROADMAP.md, and NEXT.md first.

Phase 1 task: fork the sibling app `app-workplace-memo` into this repo as
the foundation for `app-workplace-presentation`. Do not add any
presentation-specific features yet. Goal: a forked shell that compiles
and launches, showing a placeholder window.

Steps:

1. Copy unchanged from ../app-workplace-memo/:
   - src-tauri/src/main.rs (the four IPC commands are identical)
   - scripts/download-deps.sh
   - scripts/embed-fonts.sh
   - docs/licence-header.txt (if it exists; otherwise create one)

2. Adapt these files from memo, changing only what identifies the app:
   - src-tauri/Cargo.toml → package.name = "app-workplace-presentation"
     Make sure the file ends with an empty [workspace] table.
   - src-tauri/tauri.conf.json → productName, identifier
     (com.pointsav.workplace.presentation), window title
   - package.json → name field only

3. Copy PointSav gold chrome tokens from ../app-workplace-memo/src/styles/app.css
   into src/styles/app.css. Keep the tokens identical (colour, spacing, typography).
   Remove memo-specific layout rules.

4. Create src/index.html as a minimal three-pane shell:
   - Left pane (200px wide): slide navigator placeholder with text "Slides"
   - Centre pane (flex): canvas placeholder with text "Canvas"
   - Right pane (hidden by default): code view placeholder with text "Code"
   - Top menubar: File / Home / Insert / Design / Slide Show / View
   - Status bar bottom: "Slide 1 of 1 · 100%"

5. Generate icons:
   - Create src-tauri/icons/icon-source.png as 1024×1024 PointSav gold (#c8a96e)
     solid square (ImageMagick one-liner is fine).
   - Run: npx tauri icon src-tauri/icons/icon-source.png
   - The derived files are gitignored; only icon-source.png is committed.

6. Verify:
   - Run `make setup` (or `npm install`)
   - Run `make dev` (or `npm run tauri dev`)
   - App launches. Window title reads "Workplace Presentation".
     Three panes visible. No console errors.

7. Commit as a single commit:
   chore(init): fork from app-workplace-memo — Phase 1 shell

Ask before deviating from this list. Do not start Phase 2 yet.
```

---

## After Phase 1 is verified running

Update this file. Replace the Phase 1 block above with the Phase 2 commission prompt from `ROADMAP.md`.

---

## Resolved decisions — 2026-04-19

- **Slide aspect ratio:** **US Letter landscape (11″ × 8.5″, 1.294:1).** Not 16:9. Rationale: target audience (bankers, asset managers) prints every deck on US Letter paper. The deck *is* the handout; projector use is secondary. Tradeoff: black bars on modern 16:9 projectors — acceptable. See ADR-PR-09.
- **Default fonts:** Source Sans 3, 24pt body / 40pt title. Locked.
- **Slide layouts at ship:** Three only — **Title / Content / Blank**. **Blank is the startup default** (proforma discipline: no template pops up at launch). Two-Column moves to post-Phase-7 backlog.
- **Code view:** Ships in Phase 4 as planned. Plain monospace text, no syntax highlighting. The code view is an institutional feature — it lets a banker see their own file's raw HTML as proof of ownership. Highlighter deferred; revisit if feedback requests it.

---

## Deferred — track, do not action

- macOS build verification. Owner's iMac is on 10.13 High Sierra. Linux Mint is the primary dev target. macOS builds happen when the iMac is upgraded.
- Tauri v2 migration. Coordinated across all three workplace apps. Not this project.
- Real icon artwork. Currently using a solid gold square as `icon-source.png`. A commissioned grid-of-cells motif (proforma) and document-with-fold motif (memo) are the family direction; presentation gets a slide-stack motif in PointSav gold when commissioned.

---

## Session log

### 2026-04-19 — Project scaffolded + design decisions locked
- Created repo scaffold: CLAUDE.md, NEXT.md, ROADMAP.md, CLEANUP_LOG.md, ARCHITECTURE.md, DEVELOPMENT.md, README.md (bilingual), LICENCE, CHANGELOG.md, Makefile, package.json, .gitignore, src-tauri/Cargo.toml, src-tauri/tauri.conf.json, src-tauri/src/main.rs
- Four design decisions resolved: aspect ratio = US Letter landscape (ADR-PR-09), fonts = Source Sans 3 24/40pt, three ship layouts (Title/Content/Blank), code view ships plain without highlighter.
- No source frontend code written yet. That is Phase 1 work.
- Phase 1 commission prompt drafted above.
