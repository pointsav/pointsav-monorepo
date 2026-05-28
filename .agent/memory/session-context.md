## Session context — rolling 3-session summary

---

### 2026-05-28 | totebox@project-knowledge | claude-sonnet-4-6

**Done this session:**
- Phase 6A COMPLETE: Fixed AJAX navigation (articles not loading on click). Root cause confirmed:
  wiki.js `navigateTo()` had 3 stale selectors from Phase 2 DOM renames. `#vector-toc` (now
  `aside.toc`), `h1.page-title` (now `h1.article__title`), `.wiki-breadcrumb` (now `nav.crumb`).
  Content body (`#mw-content-text`) still matched, giving a confusing partial-update symptom where
  article text changed but title/TOC/breadcrumb froze. All 6 occurrences fixed. Also fixed
  `initToc()`, `initTocPin()`, and `initActiveTocTracking()` heading selector.
  server.rs: added `id="toc-list"` to TOC `<ol>`.
- Phase 6B COMPLETE: Home page section caps. Removed uncategorised catch-all block entirely.
  Guides capped at 6 (`.iter().take(6)`). Data fetch aligned to render cap (10→8).
- Phase 6C COMPLETE: Header redesign matching home.pointsav.com. `WORDMARK_SVG_POINTSAV` constant
  with verbatim SVG path data (320×80). All three chrome functions (home_chrome, wiki_chrome, chrome)
  now render `header.topnav` grid `1fr auto 1fr`. `--header-h` 152px → 80px. Dark mode SVG invert.
  Commit `afa67bfa` (Jennifer). 106/106 tests pass.
- Command actioned Stage 6 (outbox `project-knowledge-20260528-phase6-knowledge-platform`):
  promoted `afa67bfa` to canonical; binary rebuild queued in nightly queue for ~1am Vancouver.

**Pending / carry-forward:**
- Binary rebuild deploying tonight (~1am Vancouver) — services active on prior binary until then.
  After rebuild: visual check on documentation.pointsav.com (topnav SVG wordmark, article link nav).
- Phase 6 Part B + deployment split (Phase 6 per KNOWLEDGE-PLATFORM-PLAN.md §6) gated on
  content-wiki-* GitHub rename + MASTER Doctrine amendment.
- `.agent/manifest.md` wrong `cluster_name` (project-bim) — needs Command correction.
- `.shell-header` legacy CSS can be cleaned up in a future session (now dead code).
- ES bilingual pairs for 4 governance stubs (lower priority).

**Operator preferences surfaced:**
- "leapfrog 2003 UI/UX" — dramatically higher visual quality; match live family sites exactly.
- OPUS agents for site audits — sends parallel agents to analyze current vs. target before planning.
- "plan we can leave on auto" = execute without per-step approval once plan approved.

---

### 2026-05-27 | totebox@project-knowledge | claude-sonnet-4-6

**Done this session:**
- Full site audit via OPUS agents: identified two root causes for "C grade / no links work"
  1. Four chrome nav articles missing (disclaimers, contact, about, contribute) → all 404
  2. CSS/HTML mismatch: new proto-platform-document CSS not wired to server.rs HTML classes
- Phase 1 COMPLETE: Created 4 governance stub articles in content-wiki-documentation (`86d7567`)
  All four `/wiki/{slug}` routes now return 200 immediately (disk-served)
- Phase 2 COMPLETE: Rewrote wiki_chrome() HTML structure to match proto-platform-document CSS
  Key changes: .wiki-layout → .shell, div#mw-panel → nav.sidebar, main.mw-body →
  main.article-wrap + article.article__body, h1.page-title → h1.article__title,
  p.topic-short-description → p.article__lede, div.page-body → div.prose, TOC moved
  to aside.toc beside article prose, right rail removed/consolidated into sidebar,
  dl.article__meta added for metadata row. Commit `1a2feb69` (jwoodfine).
- Phase 3 COMPLETE: Route wildcard fixes — /git/{slug} → /git/{*slug},
  /special/whatlinkshere/, /special/pageinfo/, /special/cite/ same treatment.
  Fixes 404 for category-scoped articles. Same commit `1a2feb69`.
- 106/106 lib tests pass; clippy clean. Outbox msg to Command for Stage 6 + rebuild.

**Pending / carry-forward:**
- See 2026-05-28 entry — Phase 6 deployed; binary rebuild tonight.

**Operator preferences surfaced:**
- OPUS agents for site audits — "send out several OPUS agents to make full analysis"
- "plan we can leave on auto" = execute without per-step approval once plan approved

---

### 2026-05-24 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- T1-A: app-console-system added to Cargo.toml workspace members (`7e47fd05`)
- T1-C/D: NEXT.md updated (Phase 3+4 complete, Phase 5 queued); service-extraction CLAUDE.md created (`e9b84f21`, `3a5b11f9`)
- Phase 5 COMPLETE — draft mode: `/new <title>` slash command in ContentCartridge; Doorman Tier B SSE streaming client (`draft.rs`); `drafts-outbound` write with `foundry-draft-v1` frontmatter; `drafts_outbound_path` added to ConsoleConfig. Commits `6422c2a8` + `5118ce77`. `cargo check --workspace` exits 0.
- Session close-out: NEXT.md updated (Phase 5 → Complete, Phase 6 → Next, commit `894452c1`); binary-targets.yaml notes updated; Phase 5 outbox notification sent to Command (`053847d`); inbox archived 8 actioned/stale messages, only Stage 6 blocker retained (`edc2b84`)

**Pending / carry-forward:**
- Stage 6 push: waiting Command decision on history-replacement force-push authorization. See outbox msg `project-console-20260522-stage6-history-divergence` for the 3 questions requiring sign-off.
- Phase 6: offline mode + Tantivy full-text search (next coding phase)
- pairing-server systemd unit deployment on VM (Command/operator)
- GCE firewall port 2222 (operator action)
- Tag v0.1.0 (after Stage 6)
- Peter's SSH key + proofctl user add (Command is generating this — seen in COMMAND shell 2026-05-24)

**Operator preferences surfaced:**
- "plan we can leave on auto" = write a tight AUTO plan then execute without further approval per step
