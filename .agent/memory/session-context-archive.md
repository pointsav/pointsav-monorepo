

---

## 2026-07-03 | totebox@claude-code | DTCG token gap closure + BIM-Objects-CMS redesign (research-heavy, 7 subagents)

**Done:**
- **DTCG token gap register closed**: all 6 missing files from `tool-buildingwidth-architecture.md`
  (`furniture.dtcg.json`, `floor-plate-assembly-rules.dtcg.json`, `building-grid.dtcg.json`,
  `tenant-mix.dtcg.json` new; Medium Tile family + Special Tiles added to `tile-system.dtcg.json`) plus
  all 6 documented internal inconsistencies (5 fixed with real data already in the token set; 1 —
  professional-office medium/large — marked `status: reserved`, no source exists, not fabricated).
  Found and flagged two new data gaps rather than papering over them (Medium-tile end-cap composition
  not sourced; Tile F-medium's stated composition doesn't arithmetically reconcile with its SF target).
  Commit `ae153aa` (woodfine-bim-library). Rust crate scaffold now unblocked, deliberately not started.
- **Reposition as a BIM Objects CMS, not a wiki clone** — operator looked at the redesign shipped
  earlier this session fresh and judged the wiki-engine-modeled chrome (utility bar, in-header search,
  accent-left-border cards) the wrong direction; floated "Carbon for BIM Objects" positioning. Full
  research-through-implementation cycle, Plan Mode, 7 subagents across 3 rounds — all findings on
  record at `.agent/sub-agent-results/RD.1` through `RD.7-visual-direction-synthesis-2026-07-03.md`:
  live-site + git-history audit, design.pointsav.com current+historical audit (its own catalog
  documents a "wiki" component category as *legitimate but distinct* from its own catalog-shell chrome
  — sharpened the diagnosis), full extraction of prior strategy docs (`BB.13`/`BB.14` — Adobe Spectrum
  pick, 14/15 bankers'-distinguishability; `bim-token-strategy.md`), a Fable-driven competitive resurvey
  (Spectrum pick re-confirmed against the sibling's drifted current palette; new AEC-vendor prior art
  found — Bentley iTwinUI, doesn't displace the pick), Fable synthesis into one implementable spec.
  **Implemented**: fixed a real bug (`content::load_categories` only enumerated `.md` sidecars, so the
  4 new DTCG files were invisible — flipped to token-file-driven enumeration), 4-section IA
  (Taxonomy/Objects/Compositions/Context, `Section` enum, `section:` frontmatter on all 24 sidecars),
  utility bar + in-header search deleted, header rebuilt as a 48px "title block" bar, full color/type
  system swap (`#1A4480` drafting blue as sole interactive accent, color-collision-checked; dark mode
  re-palettized to instrument-navy), Carbon CSS scoped to `/edit/*` only. Commit `0f76dd0e`
  (pointsav-monorepo). **Honestly flagged, not silently degraded**: Geist/Source-Serif-4 font files
  don't exist in this workspace and can't be fetched here — `fonts.css` uses documented system-font
  fallbacks, not fabricated `@font-face` rules. **Deferred, fully specced in `RD.7`**: hero SVG, tab
  bars, GUID markers, chip restyle, dark preview frames, section landing pages — nothing lost by
  waiting. PointSav-branded/dedicated-domain positioning explicitly deferred as a separate decision
  (operator didn't respond to the scope question; defaulted to lowest-risk: Woodfine branding unchanged).
- **Promote anomaly, correctly diagnosed, not forced through**: `self-service-promote.sh` failed twice
  with an ordinary-looking "fetch first" rejection. Full diagnosis (fresh fetch, `merge-base`,
  `--is-ancestor` — all clean) before finding the real cause: ran the script from the archive root
  instead of from inside `pointsav-monorepo` (script derives repo identity from `basename $REPO_ROOT`,
  so it picked up the wrong repo's HEAD). Flagged 2 spurious `promote-queue.jsonl` entries to Command
  via mailbox with a script-hardening suggestion; retried correctly, clean fast-forward, staging
  mirrors updated.
- Fable-model agents used explicitly per operator instruction ("use FABLE as well where appropriate")
  for the two genuinely open-ended/creative research steps (competitive resurvey, final synthesis) —
  kept factual audits (live-site DOM, git archaeology) on standard Explore-type agents.
- Sub-agents in the `Explore`/general-purpose read-only role do NOT have Write tool access even when
  asked to write a results file — they return the content as their final message instead; the
  orchestrating session has to persist it. Learned this the hard way after asking 4 agents to Write
  and having to manually persist all 4 outputs. Design future multi-round research prompts around this.

**Pending / carry-forward:**
- **Not yet in Command's canonical merge** — staging mirrors updated (`0f76dd0e`), Command inbox
  notified, promote-queue entry correct. Command will process at next startup per standard flow.
- RD.7's deferred visual polish (hero SVG, tab bars, GUID markers, chip restyle, dark preview frames,
  section landing pages) — fully specced, ready to implement whenever picked up next.
- Font files (Geist Sans/Mono, Source Serif 4) need sourcing + self-hosting before the typography spec
  fully lands — needs either operator-provided files or a session with font-fetch capability.
- All carry-forward items from the prior session below remain open (PO-1 furniture_refs, key-plans-
  registry.md, Corporate Office SVGs, 18 IFC files — actually resolved this session's earlier turn per
  NEXT.md, 5 Rust crates, DTCG accuracy errors — operator-pending, do not touch, port 9095 wiki CSS).

**Operator preferences surfaced:**
- Explicitly validates the "escalate a git anomaly, diagnose fully before touching anything, don't
  force/rebase" pattern a second time — this is now a confirmed, reusable rule for this archive, not a
  one-off. Fully diagnosing before acting (not just reporting "it failed") is the expected standard.
- Wants real competitive/design research grounded in re-reading ALL existing BRIEFs/TOPICs/GUIDEs and
  prior research before proposing new direction — explicitly said "cross check the internet after the
  review" (sequencing: internal review first, external research second, synthesis last).
- Comfortable with large multi-agent research efforts for genuinely strategic/positioning questions
  (not just implementation) — this session's 7-subagent, 3-round research cycle before any code change
  was the right scale for "is our product positioning correct," not overreach.

---

## 2026-07-02/03 | totebox@claude-code | mobile fixes + full shell redesign (largest session on this archive to date)

**Done:**
- Continued from a compacted context; earlier turns (mobile-readiness sweep, license footer split, license Cargo.toml
  correction) already summarized/committed before this entry's window starts. **Did not re-read `.agent/inbox.md`
  after the compaction** — missed 3 real messages until surfaced during shutdown (see Pending below). Flag for
  next session: always re-run the inbox read after a context compaction, don't assume it carried over.
- **Mobile-readiness sweep** (operator: "make sure bim.woodfinegroup.com is 100% ready on mobile"): found and fixed
  two real bugs — Key Plans page showed placeholder "— —"/"0 SF" on every card (data-nesting assumption wrong,
  file nests 3 levels not 2); `.bim-markdown` had zero CSS for `<p>`/`<ul>`/`<table>`.
- **Multi-agent design/consistency pass** (Plan Mode; 3 parallel Opus research agents + synthesis + Fable
  verification): fixed a confidentiality leak in research articles (internal `~/Foundry/...` paths, app codenames
  publicly visible — sanitized, made git-tracked for the first time), a `$description`-as-fake-entity-row data bug,
  duplicate-looking rows in multi-group token files, internal app slug + ops endpoints visible in public
  chrome, H1 weight 300→600, orphaned-prose layout, duplicate "Key Plans" sidebar label pointing to two pages.
- **Full shell redesign** (operator, after fresh look: "is a dog... check out how clean the new sites are" —
  compared against `home.woodfinegroup.com` and the live wiki instances, `app-mediakit-knowledge-2` at
  corporate/projects/documentation.woodfinegroup.com, ports 9095/9093/9090, started this session specifically to
  inspect). Two Explore agents mapped the wiki's exact CSS/HTML and BIM's functional inventory; a Plan agent
  validated dark-mode/Carbon interaction and search-implementation scope. Five rounds, all committed:
  1. Dark-mode infrastructure (`data-theme`, narrowed token overrides, `/edit/*` forced light — Carbon-dependent)
  2. Header + utility bar rebuild (light chrome replacing navy topbar, real wordmark, search bar, theme toggle)
  3. Sidebar + card-grid restyle (accent-left-border convention)
  4. Footer rebuild (Network column, cities line, badges) + new "Important Information" disclosure band
     (built ad-hoc — this duplicated unread Command guidance; redone properly 2026-07-07/08, see current
     session-context.md)
  5. Server-side search (categories/entities/research articles, no new crate)
  Found and fixed along the way: hardcoded `#111827` making the homepage H1-equivalent invisible in dark mode
  (swept both stylesheets for the same pattern), a mobile header-overflow bug hiding the theme toggle at 390px.
- Attempted Stage 6 self-service push (`bin/self-service-promote.sh`) — rejected, and `git merge-base` found
  **no common ancestor** between local `cluster/project-bim` and `origin-staging-j/main` (that fork's main had
  advanced with unrelated `project-knowledge` commits). Did not force-push or guess at a rebase. Escalated to
  Command (`command-20260703-project-bim-ready-for-canonical-merge-pr`, high priority) with full detail, asking
  them to resolve the fork state, process the promote-queue, and run `push-to-prod.sh bim` once done.
- Two mid-session incidents, both resolved, neither a regression: a VM-wide disk-full condition (154G/154G) that
  blocked all Bash tool use — genuinely workspace-wide, operator cleared it externally; and a detected-and-ignored
  prompt-injection attempt inside a tool result during Plan-agent exploration (flagged to operator for transparency).
- 41 commits total this session to `pointsav-monorepo` cluster/project-bim; `woodfine-bim-library` research/*.md
  (3 files, made git-tracked for the first time) on `main`; `BRIEF-app-privategit-bim.md` updated after each phase.

**Pending / carry-forward (resolved by 2026-07-07/08, kept for history):**
- Command's canonical merge + prod push was blocked on a staging-fork anomaly; resolved by Command in a
  later session. `push-to-prod.sh`'s `target_bim()` staleness was also fixed by Command by then.
- The Important Information band was redone against the real Command/project-knowledge pattern on
  2026-07-07/08 (dedicated `important-information.md`, real CC BY-ND badge, real print-open JS) — see
  current `session-context.md` for that entry.
- Search's property-set/compliance-text indexing gap, the corporate wiki CSS 404s, and the 18 modified
  IFC files were all still open as of 2026-07-07/08 — not addressed by the redesign session.

**Operator preferences surfaced:**
- Wants fresh-eyes re-verification even after "done" work — asked to recheck mobile/desktop twice more after the
  redesign was already verified once, and asked for a direct side-by-side against comparison sites a second time.
  Don't treat one verification pass as sufficient when the operator asks to "check again."
- Explicit go-ahead needed before actions with external/shared-state footprint (staging pushes, force-pushes) —
  correctly held off on a rebase-and-force-retry when the git history diverged unexpectedly; operator confirmed
  this was the right call rather than pushing through it.
- Reacts strongly and specifically to competitor/sibling-site comparisons ("is a dog," "way better") — take this
  as a genuine escalation signal warranting a structural response (full redesign), not just another polish pass.

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
