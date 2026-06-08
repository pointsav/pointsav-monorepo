# Session Context Archive — project-workplace

Entries pushed from session-context.md when file exceeds 3 entries. Newest on top.

---

## 2026-05-31 — Totebox@claude-code (Session 3)

**Done this session:**
- BRIEF consolidation: 5 active BRIEFs → 3 canonical BRIEFs (BRIEF-workplace-architecture, BRIEF-workplace-roadmap, BRIEF-workplace-desktop-environment; 4 archived).
- SSE file-watch reload proper fix (commit c7efdd1c): watch ALL roots; convert absolute→root-relative path; emit real mtime; polling reduced 4s→30s.
- Light/dark theme toggle for workbench (commit cb44f3b1): ☀/🌙 button; localStorage persistence; anti-flash script.

**Pending / carry-forward (all resolved by session 4):** Stage 6, Proforma Stage 2, JOURNAL relay.

**Operator preferences surfaced:** Light mode preference confirmed session 4 — Jennifer prefers light, dark is hard to see.

---

## 2026-05-28 — Totebox@claude-code (Session 2)

**Done this session:**
- Startup sequence executed; inbox empty; NOTAM clear (NOTAM permissions fixed by Command Session).
- Operator clarified naming and scope for the workplace suite:
  - Presentation stays (the "PowerPoint" surface; Wave 1 active)
  - Schedule is a first-class surface: construction scheduling + employee scheduling; NOT a calendar
  - Platform user-facing name → **Workbench**
  - Coding IDE surface → **code** (`app-workplace-code`); resolves the naming collision
  - Launcher/chassis → **`app-workplace-launcher`** (previously called "workbench" — ambiguous)
- Edited `BRIEF-workplace-software-suite.md` and created `BRIEF-workplace-http-prototype.md`.
- Committed both as `2144477` (pwoodfine).

**Pending / carry-forward:**
- [ ] Stage 6: resolved — all prior commits promoted. [resolved]
- [ ] HTTP prototype Stage 1 (Memo): complete. [resolved]
- [ ] Selection bug: resolved — was in project-orgcharts SVG wireBox (fixed 705a86d9). [resolved]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Presentation stays in the suite ("the PowerPoint").
- Schedule is NOT a calendar — Gantt/CPM/WBS; MS Project muscle memory.
- Platform name = "Workbench"; coding surface = "code"; launcher = `app-workplace-launcher`.

---

## 2026-05-28 — Totebox@claude-code (Session 1)

**Done this session:**
- Startup sequence executed; NOTAM blocked (rw------- mathew-only; jennifer session cannot read).
- Operator onboarding: Jennifer self-described as absolute beginner to the development/Tauri workflow; wants to use AND work on app-workplace-workbench.
- Sent Explore agent to investigate object selection bug in app-privategit-workbench (reported: clicking routing lines selects too many objects; background layer moves accidentally). Agent confirmed app-privategit-workbench is a file browser with no graphical selection system — wrong app.
- Session ended before operator identified the correct app with the bug.

**Pending / carry-forward:**
- [ ] Identify correct app for selection bug: await operator response with URL or interface description. [2026-05-28 totebox@claude-code]
- [ ] NOTAM permissions: flag to Command Session — jennifer uid=1002 cannot read /srv/foundry/NOTAM.md (rw------- mathew:foundry). [2026-05-28 totebox@claude-code]
- [ ] When operator has a Mac: walk through prerequisites (Rust, Node.js, Xcode CLT) and first build of app-workplace-workbench. [2026-05-28 totebox@claude-code]
- [ ] Stage 6 still pending (from 2026-05-27): cluster/project-workplace branch 14+ commits ahead of main. [carry-forward]
- [ ] Command Session BRIEF archive commit still pending (from 2026-05-27 outbox). [carry-forward]

**Operator preferences surfaced:**
- Absolute beginner to development workflow — explain prerequisites and steps simply; do not assume familiarity with Rust, Tauri, or build tools.
- Wants to both use and build app-workplace-workbench.
