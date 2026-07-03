---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

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
     (built ad-hoc — **see Pending, this duplicates unread Command guidance**)
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

**Pending / carry-forward:**
- **Command's canonical merge + prod push — blocked on the staging-fork anomaly above.** Do not retry
  `self-service-promote.sh` until Command confirms the fork state is resolved.
- **`push-to-prod.sh`'s `target_bim()` is confirmed stale** (Command's own inbox message, 2026-07-02): wrong
  binary/service names, wrong design-system path (`pointsav-design-system` vs `woodfine-bim-library`). Worked
  around manually last time. Don't assume a clean run next time without checking this was fixed.
- **Command/project-knowledge already built a more rigorous "Important Information" + footer disclosure pattern**
  (inbox message `command-20260702-important-information-footer-structure-a`, unread until shutdown) —
  content sourced from a Git-owned markdown file (counsel owns the text, not hardcoded in Rust), a persistent
  one-line footer disclaimer always visible (not just the collapsible band), a dedicated `/disclaimers` page,
  CC BY-ND attribution to the issuer entity for editorial/research content specifically (not the Apache-2.0 BIM
  Object data), NI 45-106 language mirroring home.woodfinegroup.com. **This session's disclosure band was built
  ad-hoc without seeing this — should be redone against the real pattern next session**, not left as-is.
  Also: if BIM will host research-paper journals, project-knowledge's SPEC-journal-wiki-render-contract.md
  §§9-10 governs the render contract.
- `bim.woodfinegroup.com` is live in production but serving a build that **predates all of this session's
  redesign work** (Command's last deploy was 2026-07-02T16:43, before the redesign started).
- Search doesn't index property-set/compliance text inside entity `$value` objects — only title/slug/IFC-class/
  top-level `$description`. A query like "fire door" can legitimately return 0 even if both words exist in the
  corpus, if no single item's *indexed* fields contain both.
- Corporate wiki instance (port 9095) had 4/5 CSS files 404ing during a same-session recheck (projects/
  documentation instances fine) — not this archive's service, flagged verbally to operator, not escalated via
  mailbox; worth a mailbox flag to project-knowledge if seen again.
- Carried forward, still open: PO-1 furniture_refs, key-plans-registry.md Deliverable 1b, Corporate Office SVG
  diagrams (blocked on zone depth data — same root cause the redesign's "no zone/furniture layout modeled at
  this program level" caption now surfaces honestly instead of silently rendering blank), 6 missing DTCG files,
  5 Rust crates, DTCG accuracy errors (operator-pending citations, do not touch), 18 modified IFC files in
  `woodfine-bim-library/key-plans/` (present across 4+ sessions now, still uncommitted, origin/cause still
  unknown), `tool-keyplan` binary-targets.yaml declaration still missing, NOTAM.md permission still open per
  last check.

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
