# Session Context — project-workplace

Rolling 3-session summary. Newest entry on top. Oldest entry pushed to session-context-archive.md when this file exceeds 3 entries.

---

## 2026-06-05 — Totebox@claude-code (Session 8)

**Done this session:**
- Folder download as ZIP shipped (commit `646462ec`, pwoodfine, Stage 6 pending):
  - Added `zip = "2"` dependency to Cargo.toml
  - New `GET /workbench/download?path=<api-path>` route in workbench.rs
  - `resolve_download_path` translates `_api/<prefix>/` paths → workbench root filesystem paths; handles the `command/` → `_command/` and `clones/` → `_clones/` underscore discrepancy
  - `build_zip` + `add_dir_to_zip` helpers: Deflated, skips dotfiles + `target/`, skips files >50 MB
  - "⬇ Download as ZIP" item added to folder context menu (all folders, before writable-only items)
- Build: clean (0 errors, 0 warnings)
- Service restarted; verified `/workbench` → 200, download endpoint → 200 for folders, 400 for files

**Pending / carry-forward:**
- [ ] Stage 6: `646462ec` (folder ZIP download) — Command Session needs `bin/promote.sh`
- [ ] project-orgcharts: create new JW versions (JW4) for Bencal WCP charts that had in-place edits in `57960322`/`fe0570af`. Current JW3 is the canonical state (#198038 green). [operator instruction 2026-06-05]
- [ ] Command Session: relay contaminated monorepo message to project-design (outbox msg project-workplace-20260605-m17-monorepo-inbox)
- [ ] Command Session: route DESIGN-TOKEN-CHANGE-wp-tokens-20260602 → project-design (outbox msg project-workplace-20260605-design-token-route)
- [ ] Command Session: route JOURNAL-NOTES-j3/j6 → project-editorial (outbox msg project-workplace-20260605-journal-notes-route)
- [ ] Memo save location chooser — plan ready; not started. [2026-06-04 totebox@claude-code]
- [ ] Memo Sessions 2–4 (Outline, Find/Replace, Export) — plan at /home/jennifer/.claude/plans/ [carry-forward]
- [ ] Stage 3 Presentation surface — next after Memo sessions. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- (none new this session)

---

## 2026-06-05 — Totebox@claude-code (Session 7)

**Done this session:**
- Startup: inbox all actioned; NOTAM clear; session lock written.
- Inbox cleanup (this session):
  - Archived 3 actioned inbox messages → inbox-archive.md
  - Archived 14 actioned outbox messages → outbox-archive.md
  - Composed 3 new outbox routing messages (DESIGN-TOKEN-CHANGE → project-design; JOURNAL-NOTES-j3+j6 → project-editorial; M-17 flag → Command)
  - Flagged monorepo inbox contamination: `command-20260603-wiki-institutional-redesign-master-cosig` marked `status: contaminated`
  - Session context carry-forward updated (3 completed items struck)
  - Committed session-context.md (ef6d2bf) and monorepo M-17 flag (7cbcd936)
- Investigated "use a new version for updated org charts": confirmed prior in-place edits to Bencal WCP JW2/JW3 (commits 57960322 + fe0570af in project-orgcharts sub-clone). Versioning rule applies; tracked below.

**Pending / carry-forward:**
- [ ] project-orgcharts: create new JW versions (JW4) for Bencal WCP charts that had in-place edits in `57960322`/`fe0570af`. Current JW3 is the canonical state (#198038 green). [operator instruction 2026-06-05]
- [ ] Command Session: relay contaminated monorepo message to project-design (outbox msg project-workplace-20260605-m17-monorepo-inbox)
- [ ] Command Session: route DESIGN-TOKEN-CHANGE-wp-tokens-20260602 → project-design (outbox msg project-workplace-20260605-design-token-route)
- [ ] Command Session: route JOURNAL-NOTES-j3/j6 → project-editorial (outbox msg project-workplace-20260605-journal-notes-route)
- [ ] Memo save location chooser — plan ready; not started. [2026-06-04 totebox@claude-code]
- [ ] Memo Sessions 2–4 (Outline, Find/Replace, Export) — plan at /home/jennifer/.claude/plans/ [carry-forward]
- [ ] Stage 3 Presentation surface — next after Memo sessions. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Versioning rule re-confirmed: always create new JW version files for org chart updates; never edit in-place (even for small fixes like token color changes).

---

## 2026-06-03/04 — Totebox@claude-code (Session 6)

**Done this session:**
- Startup: inbox all actioned; NOTAM clear; session lock written.
- Workbench file browser — drag-and-drop file move (commit `d451dcd2`, Stage 6 pending):
  - Backend: `POST /move` added to `app-privategit-workbench/src/main.rs` (port 9210); deployed binary + restarted service
  - Frontend: `wireDragOnItem()`, `doWbMoveFile()`, drag CSS, `#wb-toast`; drag-to-open on `#viewer`
- Drag-drop bug fix (commit `7870683f`, Stage 6 pending): handler was in wrong service (port 9110 vs 9210); removed dead code; fixed `/_clones/` draggable guard
- Workbench undo last file move (commit `6866eb3a`, Stage 6 pending): `moveHistory` stack (cap 10); `showWbToast(msg, undoFn)` with 6s Undo button; Ctrl+Z shortcut gated on `!isEditing`
- Plan saved (deferred): Memo save location chooser at `/home/jennifer/.claude/plans/`

**Pending / carry-forward:**
- [x] ~~Stage 6: `d451dcd2` + `7870683f` + `6866eb3a` (workbench drag+undo)~~ — COMPLETE per outbox actioned 2026-06-04 (810a2277 canonical).
- [x] ~~Stage 6: `3768ba89` (Memo Session 1)~~ — COMPLETE per outbox actioned 2026-06-03 (da8025b2 canonical).
- [x] ~~app-privategit-workbench binary ledger entry~~ — COMPLETE per outbox actioned 2026-06-04 (sha 75d5c068).
- [ ] Memo save location chooser — plan ready; not started. [2026-06-04 totebox@claude-code]
- [ ] Memo Sessions 2–4 (Outline, Find/Replace, Export) — plan at /home/jennifer/.claude/plans/ [carry-forward]
- [ ] Stage 3 Presentation surface — next after Memo sessions. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Plan-mode + Opus agents for feature design before implementation — pattern working well; continue.

---

## 2026-06-03 — Totebox@claude-code (Session 5)

**Done this session:**
- Startup: inbox all actioned; NOTAM clear.
- Memo Session 1 feature set shipped (commit `3768ba89`, Stage 6 pending):
  - Toolbar: added Underline (U), Strikethrough (S), Normal text (P), Numbered list (1.), Align left/center/right (≡L/≡C/≡R), Clear formatting (clr)
  - Light/dark theme toggle: anti-flash `<script>`, `wp-theme` localStorage key, `html.light` CSS overrides; ☀/🌙 button matching proforma.html pattern
  - Word count in status bar: live debounced (50ms) — `{filename} · N words · N chars`
  - Paste sanitization: `paste` event intercepted; allow-list strips all `style=`/`class=` and disallowed tags; `execCommand('insertHTML')` keeps in undo stack
  - Crash recovery draft: `memo-crash:<path>` localStorage key; 2s throttle; recovery banner with Restore/Dismiss
  - Placeholder CSS: `:empty::before` now renders the `data-placeholder` attribute
  - Extended `updateToolbarState()` to cover all 9 new stateful buttons
  - Keyboard shortcuts added: Ctrl+Shift+7 (OL), Ctrl+Shift+X (strikethrough), Ctrl+E (center), Ctrl+Shift+L (left), Ctrl+Shift+R (right), Ctrl+\ (clear fmt)
  - `ol` CSS rule added
  - Build verified (clean), prototype restarted, /memo returns 200

**Pending / carry-forward:**
- [ ] Stage 6: `683fc671` + `3ffaa8f6` (proforma; prior session) + `3768ba89` (memo Session 1) — Command Session needs `bin/promote.sh`. [2026-06-03 totebox@claude-code]
- [ ] Memo Session 2: Outline/TOC sidebar (left panel, jump-to-heading, scroll spy). [plan at /home/jennifer/.claude/plans/]
- [ ] Memo Session 3: Find & Replace (CSS Custom Highlight API). [plan]
- [ ] Memo Session 4: Export — Markdown (Turndown), plain text, PDF via window.print(). [plan]
- [ ] Stage 3 Presentation: next surface after Memo sessions complete. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Plan-mode + Opus agents used for feature research before implementation — good pattern to repeat.

---

## 2026-06-02/03 — Totebox@claude-code (Session 4)

**Done this session:**
- BIM schema upgraded to proper W3C DTCG: `$schema` URI, flat `$value`/`$type` tokens, `$extensions.bim-workspace` for visibility and project metadata. Backward compat for old bim-workspace-v1.0 files. Colorizer fixed (hex color regex, `$extensions` highlighting). Commits: dfb07944 (promoted 5aa88c3f).
- Proforma schema v2.0: entity/date/analyst metadata subbar; editable column header labels; per-column format badge (T→$→%→#); currency accounting format. Rust skeleton updated. Commits: 8d8049c6 (promoted 4a7e3499).
- Proforma light/dark theme toggle: ☀/🌙 button, html.light CSS overrides, wp-theme localStorage key. Jennifer confirmed dark mode was hard to see — this was operator-driven. Commit: 683fc671 (Stage 6 pending).
- Proforma formula functions: evalRange() helper; AVERAGE/AVG/MIN/MAX/COUNT added to evalExpr; AutoSum Σ toolbar button + Alt+= shortcut (detects numeric run above → column SUM, or left → row SUM). Commit: 3ffaa8f6 (Stage 6 pending).

**Pending / carry-forward:**
- [ ] Stage 6: `683fc671` (theme toggle) + `3ffaa8f6` (formula/AutoSum) — Command Session needs `bin/promote.sh`. [2026-06-03 totebox@claude-code]
- [ ] Stage 3 Presentation: next surface to build (`/presentation` route, slide JSON schema, custom HTML/CSS/JS canvas). [2026-06-03 totebox@claude-code]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Dark mode is hard to see — new surfaces should default to light or ship with ☀/🌙 toggle from day one; use `wp-theme` localStorage key for shared preference.



