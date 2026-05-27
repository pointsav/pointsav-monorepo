## Session context — rolling 3-session summary

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
- Phase 4 (Command scope): Stage 6 (bin/promote.sh) → cargo build --release →
  deploy-binary.sh → systemctl restart local-knowledge-{documentation,projects,corporate}
- Outbox msg `project-knowledge-20260527-stage6-knowledge-platform` pending Command pickup
- After deploy: visual check that article body shows new .shell/.article-wrap/.prose layout
- ES bilingual pairs for the 4 governance stub articles (lower priority)
- .agent/manifest.md contains wrong cluster_name (project-bim) — needs Command correction

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
- Manifest path updates (fleet_deployment_repo, catalog_subfolder) — stale domain migration item

**Operator preferences surfaced:**
- "plan we can leave on auto" = write a tight AUTO plan then execute without further approval per step

---

### 2026-05-23 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- Stage 6 rebase COMPLETE: 11 os-console commits rebased onto local `main` (tip `9afc9e25`). Conflicts resolved: .gitignore merge, .claude symlink discarded (kept directory), workspace Cargo.toml member-list merges at each phase commit, per-crate Cargo.toml/main.rs took cluster versions.
- Push BLOCKED: discovered local `main` and all remotes (canonical + staging-j/staging-p) share zero common ancestors — full history divergence, not the "5 commits ahead" described in inbox. Escalated to Command via outbox `project-console-20260522-stage6-history-divergence`.
- binary-targets.yaml written: declares os-console, pairing-server, proofctl (all AGPL-3.0, apache tier). service-proofreader NOT in current cluster branch — flagged to Command.
- Build-request outbox sent: `project-console-20260523-build-request`.
- Inbox msg `command-20260522-binary-targets-project-console` actioned; `command-20260522-console-stage6-orphan-branch` set to operator-pending.

**Pending / carry-forward:**
- Stage 6 push: waiting Command decision on history-replacement force-push. Local main tip `9afc9e25` ready.
- Phase 3 QR: `ratatui-image` Kitty/Sixel pixel-perfect QR with Dense1x2 fallback
- Phase 4 F11 app-console-system: operator approve/deny in-TUI
- pairing-server systemd unit deployment on VM
- GCE firewall port 2222 (operator action)
- Tag v0.1.0 (after Stage 6)
- Peter's SSH key + proofctl user add
- Three per-user config.toml files
- briefs/ migration (plans/ → briefs/ BRIEF- prefix) — inbox msg still pending

**Operator preferences surfaced:**
- "route to Command first" — when a destructive operation (force-push) has unexpected scope, escalate rather than proceed


