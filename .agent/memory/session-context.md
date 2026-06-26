---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-06-25 | totebox@claude-code | startup + immediate shutdown (no work done)

**Done:**
- Session lock written; role confirmed (Totebox, cluster/project-bim)
- Startup reads: manifest, inbox, NOTAM (clear), session-context, briefs README, outbox, git status
- Detected: 18 modified IFC files in `woodfine-bim-library/key-plans/` (pre-existing; not created this session)

**Pending / carry-forward:**
- Stage 6 + production deploy of `app-privategit-bim` → Command Session (outbox dispatched 2026-06-20)
- NOTAM.md permission (`chmod 644`) → Command Session (outbox dispatched 2026-06-22)
- PO-1 furniture_refs: apply to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- key-plans-registry.md Deliverable 1b
- Corporate Office SVG diagrams: blocked on zone depth data
- 6 missing DTCG files; 5 Rust crates
- DTCG accuracy errors (operator-pending citations — do not touch)
- 18 modified IFC files in `woodfine-bim-library/key-plans/` — uncommitted; origin/cause unknown; review before next commit

**Operator preferences surfaced:** none new this session.

---

## 2026-06-22 | totebox@claude-code | context cleanup sweep + DTCG Decisions 2-4

**Done:**
- Inbox: 10 messages archived (inbox-archive.md); 2 operator-pending DTCG accuracy error messages kept
- Outbox: NOTAM permission flag prepended to command@claude-code — NOTAM.md is still `rw-------` despite `command-20260520-notam-permission-resolved` claiming it was fixed
- Draft states: `design-research-html-print-pdf-pipeline.draft.md` → `state: destination-committed` (sha `a6dc0df`)
- `cleanup-log.md`: corrected stale `woodfine-design-bim` reference → `woodfine-bim-library`
- **Decision 2 (HTML DTCG fetch):** Removed inline `BIM_TOKENS` block from `preview/building-width-calculator.html`; replaced with async `init()` fetching from `../woodfine-bim-library/tokens/bim/*.dtcg.json`; source footer updated. HTML now serves authoritative zone depths (professional-office magazine 3.8m, business habitat 6.0m, private-office corridor 0.0m)
- **Decision 3 (tile_code):** Added `tile_code: RS-A/B/C` to `retail-select.dtcg.json`; `tile_code: TI-A/B/C` to `tech-industrial.dtcg.json`
- **Decision 4 (tile stubs):** Added `corridor-expander.tile-t` (300 SF, operative) + `reserved.tile-j/k/l/m` stubs to `tile-system.dtcg.json`
- Commits: `05c8c38` (woodfine-bim-library, main, jwoodfine) + `b7ee3e6e` (cluster/project-bim, pwoodfine)

**Pending / carry-forward:**
- Stage 6 + production deploy of `app-privategit-bim` → Command Session (outbox `project-bim-20260620-stage6-deploy-bim`, dispatched)
- NOTAM.md permission still denied — Command needs to re-apply `chmod 644` (outbox `project-bim-20260622-notam-permission-still-denied`, pending)
- PO-1 furniture_refs: apply structured refs to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- key-plans-registry.md Deliverable 1b still open
- Corporate Office SVG diagrams: blocked on zone depth data
- 6 missing DTCG files; 5 Rust crates; DTCG accuracy errors (operator-pending citations)

**Operator preferences surfaced:** none new this session.

---

## 2026-06-20 | totebox@claude-code | app-privategit-bim UI polish

**Done:**
- Applied three UI fixes to `app-privategit-bim` (clean-sheet Carbon rewrite):
  1. **Hero + header/footer restored** — `cds-header-name` now shows "Woodfine | BIM Object Library"; home page has hero tagline, three article sections (problem statement, BIM Objects answer, browse CTA), dark three-column footer. Content sourced verbatim from `app-orchestration-bim/src/main.rs`.
  2. **Sidebar always open** — removed hamburger toggle (`cds-header-menu-button`), added `expanded` attribute to `cds-side-nav`. Resolves Carbon conflict between hamburger pattern and `is-not-child-of-header` persistent rail.
  3. **Token table fixed** — replaced `cds-data-table` / `cds-table-*` web components (inline-rendered before JS initialises) with plain `<table class="bim-token-table">`. CSS selectors added to `bim-components.css`; hero/footer CSS added to `bim-layout.css`.
- Files changed: `shell.rs`, `card.rs`, `home.rs`, `bim-layout.css`, `bim-components.css`
- Committed `39d3cb0b` to monorepo cluster branch: "feat(app-privategit-bim): restore header/footer/intro; fix sidebar; replace cds-data-table with plain table"
- Preview running on port 9206 (SSH tunnel: `ssh -L 9206:localhost:9206 jennifer@34.53.65.203 -N`); verified all fixes via curl
- Created `BRIEF-app-privategit-bim.md` + `briefs/README.md` (new files)

**Root cause learned:** Previous session edits landed in phantom `/srv/foundry/clones/project-bim/app-privategit-bim/` path (doesn't exist). Actual source is always in `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/`.

**Pending / carry-forward:**
- Stage 6 promotion of monorepo cluster branch → Command Session (outbox message sent)
- Production deploy of `app-privategit-bim` (replace `app-orchestration-bim` on port 9096, nginx config, systemd service) → Command Session

**Operator preferences surfaced:**
- User browses via SSH tunnel (`ssh -L 9206:localhost:9206 jennifer@34.53.65.203 -N`) — not WireGuard VPN


