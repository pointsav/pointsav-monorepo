

---

## 2026-07-02 | totebox@claude-code | live-site diagnosis + escalation to Command

**Done:**
- Session lock written; role confirmed (Totebox, cluster/project-bim)
- Startup reads: manifest, session brief (inbox/outbox/NOTAM/context), rules, briefs README, session-start
- Operator asked me to check bim.woodfinegroup.com ("does not look good"). Diagnosed root cause: foundry-prod
  is still serving the OLD `app-orchestration-bim` — all 5 CSS files 404 (unstyled page), `/readyz` shows
  `tokens_count: 0, components_count: 0`, `/tokens.json` returns `{}`
- Confirmed on this workspace VM that `local-bim.service` already runs the NEW `app-privategit-bim` on
  127.0.0.1:9096, fully healthy and styled (components_count: 18) — the fix already exists locally, it just
  hasn't been promoted/deployed to foundry-prod
- Sent HIGH-priority escalation to Command (`command-20260702-escalation-bim-woodfinegroup-com-is-live`)
  re-requesting the Phase 2 production deploy originally requested 2026-06-20 (`project-bim-20260620-stage6-
  deploy-bim`, still un-actioned as of this session)
- Updated `BRIEF-app-privategit-bim.md` work log + carry-forward with full diagnosis
- No file edits/commits in project-bim this session (investigation + mailbox only)

**Pending / carry-forward:**
- Stage 6 + production deploy of `app-privategit-bim` → Command Session — **now HIGH priority**, live site is
  actively broken (see escalation msg-id above), not just a pending upgrade
- `token_count: 0` in app-privategit-bim's own local preview (separate bug from the static-asset issue;
  components load fine at 18) — needs investigation, likely in the DTCG token-loading path
- NOTAM.md permission (`chmod 644`) → Command Session (outbox dispatched 2026-06-22; still unresolved per
  inbox — Command claimed fixed 2026-05-20 but it recurred)
- PO-1 furniture_refs: apply to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- key-plans-registry.md Deliverable 1b
- Corporate Office SVG diagrams: blocked on zone depth data
- 6 missing DTCG files; 5 Rust crates
- DTCG accuracy errors (operator-pending citations — do not touch)
- 18 modified IFC files in `woodfine-bim-library/key-plans/` — still uncommitted; origin/cause unknown;
  present across at least 3 sessions now (2026-06-25 → 2026-07-02); review before next commit
- New inbox item (2026-07-02, broadcast): binary distribution tracking — `.agent/binary-targets.yaml` exists
  but only declares `app-orchestration-bim`; `tool-keyplan` also has a `[[bin]]` target and is undeclared.
  Not actioned this session — flagged, awaiting operator go-ahead to add the entry.

**Operator preferences surfaced:** none new this session.

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

---

## 2026-05-28 | totebox@claude-code | preview-viewers + routing-correction

**Done:**
- Created `preview/po-1-floor-plan.html` — SVG floor plan of PO-1 at 1:20 scale; 6 furniture pieces; dimension callouts; zone fills; HTML legend table
- Created `preview/interior-tokens.html` — HTML viewer for interior.dtcg.json; 7 furniture Object cards + circulation constraint panel; colour-coded by category
- Copied `preview/interior.dtcg.json` — point-in-time snapshot in preview/ for local serving
- Copied all 3 new preview files to `outputs/` for rsync access
- Copied `interior.dtcg.json` to correct location: `woodfine-bim-library/tokens/bim/interior.dtcg.json`
- Updated BRIEF-tool-keyplan.md: interior.dtcg.json and key-plans.dtcg.json locations corrected

**Pending / carry-forward:**
- Apply PO-1 structured furniture_refs to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- Deliverable 1b: `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
- Corporate Office SVG diagrams: blocked on zone depth data
- DTCG 6 missing Object files; 5 Rust crates
- Binary ledger entry at Command Session for app-orchestration-bim v0.0.3

**Operator preferences surfaced:** say "BIM Objects" not "tokens" in project-bim; all BIM Object files route to woodfine-bim-library.

---

## 2026-05-23 | totebox@claude-code | tool-keyplan-scaffold

**Done:**
- Scaffolded `tool-keyplan` Rust crate — TOML config → validated DTCG JSON engine (v0.0.1)
- Created `pointsav-design-system/tokens/bim/interior.dtcg.json` — 7 furniture tokens + 1 circulation constraint token
- Created `pointsav-monorepo/tool-keyplan/` crate; updated workspace Cargo.toml
- Updated key-plans.dtcg.json PO-1 with structured furniture_refs + compliance
- Engine validation: ASR A1.2 ✓ European Lighting ✓ Wheelchair ✓
- Created `.agent/briefs/BRIEF-tool-keyplan.md`

**Carry-forward:** Deliverable 1b key-plans-registry.md; Corporate Office SVG; Decisions 1–4; 6 DTCG files; binary ledger v0.0.3

---

## 2026-05-21b | totebox@claude-code | plan-bim-objects-v2-committed

**Done:**
- Merged Deliverable 1 spec (authoritative FIN.xlsx sizes, Q1–Q6, Decisions 1–4) into plan-bim-objects.md → status: draft-v2
- Committed: 667c5f2 "plan: BIM Objects draft-v2 — authoritative sizes + Q1-Q6 + Decisions 1-4"
- Copied v2 to `outputs/plan-bim-objects.md` — accessible via `fpull bim outputs/`

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` — READY, all data in plan-bim-objects.md v2
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (HIGH)
- DTCG 6 missing files (unblocked by Decision 3)
- Rust crate scaffold (deferred until DTCG complete)
- Stage 6: 32+ commits ahead of origin

**Operator preferences surfaced:** will proceed with Deliverable 1 in next session.

---

## 2026-05-21 | totebox@claude-code | plan-bim-objects

**Done:** Read 15+ source documents; created plan-bim-objects.md first draft; resolved Q1–Q6; read FIN.xlsx authoritative sizes; specified Deliverable 1.

**Pending:** Deliverable 1 registry MD, DTCG standardisation, HTML BIM_TOKENS removal, Rust scaffold, Stage 6 (~31 commits ahead).

**Operator preferences:** `fpull bim outputs/`; eco-region variants deferred; Corporate Office sizing deferred.
