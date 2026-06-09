# Session Context — project-workplace

Rolling 3-session summary. Newest entry on top. Oldest entry pushed to session-context-archive.md when this file exceeds 3 entries.

---

## 2026-06-08/09 — Totebox@claude-code (Session 9)

**Done this session:**
- Startup: inbox actioned (sub-clone fix × 2 + manifest fix × 1); NOTAM clear; session lock written.
- Ops fix — sub-clone .agent/ scope violation (commit `923b5171`, Stage 6 pending):
  - pointsav-monorepo sub-clone had 84 .agent/ files tracked in git (foundry-fsck CRITICAL)
  - Fixed: `.agent/` added to pointsav-monorepo/.gitignore; `git rm --cached .agent/ -r` removed all
- Feature — live file-reload via inotify SSE in app-privategit-workbench (commit `7152333f`, Stage 6 pending):
  - Problem: external writes never triggered browser refresh; tab.content[] cache + iframe.srcdoc frozen
  - Solution: broadcast::channel(64) + recommended_watcher over all config.roots; /events SSE route
  - Frontend EventSource consumer: silently repaints iframe.srcdoc or frame.src (cache-busted) or editor
  - Dirty-tab guard: "changed on disk" banner instead of clobbering unsaved edits
  - 30s pollActiveTab() fallback; auto-reconnect on error
  - Cargo.toml: notify = "6" + tokio-stream = "0.1" added
  - Nginx action needed (operator/Command): location /_api/edit/events — proxy_buffering off; proxy_read_timeout 3600s
- Manifest fix — cluster: project-workplace added to .agent/manifest.md (commit `9cd73a4`)
- Inbox (mid-session) — Stage 6 pre-promote gate FAILED (msg from Command):
  - cargo fmt fix (commit `f00e676a`): 28 lines reformatted in main.rs
  - clippy::manual_strip fix (commit `ec305edc`): strip_prefix at main.rs:1308
  - cargo check clean; outbox signal sent to Command (stage6-ready)

**Pending / carry-forward:**
- [ ] Stage 6: `923b5171` (sub-clone .agent/ untrack) + `7152333f` (live reload) + `f00e676a` (fmt) + `ec305edc` (clippy) — Command Session run `bin/promote.sh` + binary ledger entry for workbench. [2026-06-09 totebox@claude-code]
- [ ] Stage 6: `646462ec` (folder ZIP download, Session 8) — still pending Command promote. [2026-06-05 totebox@claude-code]
- [ ] Nginx vhost: add `location /_api/edit/events { proxy_buffering off; proxy_read_timeout 3600s; proxy_set_header Connection ''; proxy_http_version 1.1; }` — operator/Command action on deployment. [2026-06-09 totebox@claude-code]
- [ ] project-orgcharts: create new JW versions (JW4) for Bencal WCP charts that had in-place edits in `57960322`/`fe0570af`. [operator instruction 2026-06-05]
- [ ] Command Session: relay contaminated monorepo message to project-design (outbox msg project-workplace-20260605-m17-monorepo-inbox)
- [ ] Command Session: route DESIGN-TOKEN-CHANGE-wp-tokens-20260602 → project-design
- [ ] Command Session: route JOURNAL-NOTES-j3/j6 → project-editorial
- [ ] Memo save location chooser — plan ready; not started. [carry-forward]
- [ ] Memo Sessions 2–4 (Outline, Find/Replace, Export) — plan at /home/jennifer/.claude/plans/ [carry-forward]
- [ ] Stage 3 Presentation surface — next after Memo sessions. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- (none new this session)

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
- Investigated "use a new version for updated org charts": confirmed prior in-place edits to Bencal WCP JW2/JW3 (commits 57960322 + fe0570af in project-orgcharts sub-clone). Versioning rule applies; tracked below.

**Pending / carry-forward:**
- [ ] project-orgcharts: create new JW versions (JW4) for Bencal WCP charts that had in-place edits in `57960322`/`fe0570af`. [operator instruction 2026-06-05]
- [ ] Command Session: relay contaminated monorepo message to project-design
- [ ] Command Session: route DESIGN-TOKEN-CHANGE-wp-tokens-20260602 → project-design
- [ ] Command Session: route JOURNAL-NOTES-j3/j6 → project-editorial
- [ ] Memo save location chooser — plan ready; not started. [2026-06-04 totebox@claude-code]
- [ ] Memo Sessions 2–4 (Outline, Find/Replace, Export) — plan at /home/jennifer/.claude/plans/ [carry-forward]
- [ ] Stage 3 Presentation surface — next after Memo sessions. [carry-forward]
- [ ] Prototype has no systemd unit — must restart manually after VM reboot. [carry-forward]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Versioning rule re-confirmed: always create new JW version files for org chart updates; never edit in-place.

---
