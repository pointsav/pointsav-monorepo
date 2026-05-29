## Session context — rolling 3-session summary

---

### 2026-05-29 (session 2) | totebox@project-knowledge | claude-sonnet-4-6

**Done this session:**
- Phase 8 COMPLETE: `app-mediakit-knowledge` integrity bar (blake3, 16 hex chars in `div.article-integrity`),
  history pagination (?page=N, 25/page, older→/←newer links), diff stats header (+N/−M lines),
  `/special/hash-lookup/{hash}` route, Phase A5 `.shell-header` dead CSS confirmed already removed.
  3 new tests in `tests/history_test.rs` — all 7 history tests pass. CSS in `static/style.css`.
  Commit `0e5fd685` (Peter).
- Block B (ES governance stubs): `about.es.md`, `contact.es.md`, `disclaimers.es.md`, `contribute.es.md`
  created in `content-wiki-documentation/` with correct `slug: {name}.es` / `paired_with:` convention.
- Block C (A6 pickup): `content-wiki-documentation/research/geometric-site-selection-national-tenancy.md`
  committed with preprint WIP block + Forward-Looking Statements block per journal-artifact-discipline.md.
  `research/_index.md` + `research/_index.es.md` category landing pages created.
  Artifact-registry A6 status updated: `COMMITTED 2026-05-29 — commit 13b8caa`.
- Block D1 (TOPIC-privategit-workbench): pre-existing `applications/app-privategit-workbench.md` (2026-05-28)
  was already more complete than incoming draft; removed draft-only frontmatter fields; quality → pre-build.
- Block D2 (GUIDE-workbench-setup): copied to `.agent/drafts-outbound/GUIDE-workbench-setup.md`
  for Command Session routing to woodfine-fleet-deployment.
- Block D3 (inbox): all 3 inbox messages actioned + archived to inbox-archive.md. Inbox cleared.
- Outbox: Stage 6 request prepended (msg-id: `project-knowledge-20260529-phase8-stage6`) listing
  monorepo commit `0e5fd685` + 16 prior pending commits + content-wiki-documentation commit `13b8caa`.
- Both commits in content-wiki-documentation repo (`13b8caa`) and pointsav-monorepo (`0e5fd685`).
- Release build (`cargo build --release`) running; deploy to 9090/9093/9095 pending build completion.

**Pending / carry-forward:**
- Release binary deploy: copy to `/usr/local/bin/`, restart all three services, healthcheck + smoke-test.
- Stage 6 promotion: 16+ monorepo commits pending `bin/promote.sh` from Command Session.
- content-wiki-documentation `13b8caa` also needs Stage 6.
- GUIDE-workbench-setup.md needs Command Session routing to woodfine-fleet-deployment/vault-privategit-source/.
- UX-B.7 BLOCKED: Woodfine SVG wordmark — operator must provide the asset.
- `.agent/manifest.md` wrong `cluster_name` (project-bim) — Command correction needed.
- Phase 9: next platform features (TBD from NEXT.md after Phase 8 deploys).

**Operator preferences surfaced:**
- AUTO mode continues: plan approved, execute without per-step approval.
- "all three wikis live" — complete; content-wiki-documentation ES stubs + A6 committed.

---

### 2026-05-29 | totebox@project-knowledge | claude-sonnet-4-6

**Done this session:**
- Phase 7F COMPLETE: `inject_sidenotes()` post-processor in `render.rs`. Parses comrak
  footnote `<sup class="footnote-ref">` markers and `<section class="footnotes">` definitions;
  replaces inline refs with `<span class="sidenote-anchor">` + checkbox toggle structure;
  removes `<section class="footnotes">` block. `is_journal` flag checked against `frontmatter.layout`.
  CSS: sidenotes absolute-positioned at ≥1280px for `[data-layout="journal"]`; checkbox toggle
  inline display at <1280px. Test fixture `tests/fixtures/journal/sample.md` + integration test
  `tests/journal_test.rs` (1/1 pass). Commit `c240837b` (Peter).
- Phase 7G+7H COMPLETE (CSS-only): Auto-numbered sections via CSS counters for
  `[data-instance="woodfine-corporate"]`. `data-numbered="false"` opt-out on `div.prose`.
  Both phases shipped in same commit `c240837b`.
- Raw string `"#` pitfall: `r#"href="#fn-"#` terminates early at the `"#` inside the string.
  Fix: use regular escaped strings `"href=\"#fn-"`. Same for `"href=\"#fnref-"`.
- NEXT.md updated: Phase 7F+7G+7H marked complete; Phase 8 flagged next. Commit `c7b7106a`.
- Release binary built (7m 50s) and deployed to all three instances (9090/9093/9095).
- Outbox Stage 6 request prepended. Commit `110c7926` (project-knowledge archive).
- BRIEF-app-mediakit-knowledge-2030.md updated: §3 phase table extended through 7H.

**Pending / carry-forward:**
- Stage 6 promotion: 15 monorepo commits pending `bin/promote.sh` from Command Session.
  Binary ledger update needed after promote.
- Phase 8 next: history revision list at `/history/{slug}`, side-by-side diff UI,
  `article-integrity-bar` with blake3 SHA fingerprint.
- UX-B.7 BLOCKED: Woodfine SVG wordmark — operator must provide the asset.
- ES bilingual pairs for four governance stubs — lower priority.
- `.agent/manifest.md` wrong `cluster_name` (project-bim) — Command correction needed.

**Operator preferences surfaced:**
- AUTO mode continues: plan approved, execute without per-step approval.
- "leapfrog 2003 UI/UX" — Goldman Sachs banker standard for all three instances.

---

### 2026-05-28 | totebox@project-knowledge | claude-sonnet-4-6

**Done this session:**
- `docs(brief)`: BRIEF-slm-learning-loop.md §7.2-3 updated with live SSE test result. Commits `df1a5e64` (Peter) + `3fd2dfef` (Jennifer).
- **§7.3 live SSE test completed**: sent `/v1/messages` with `Read` tool to OLMo 7B; got `stop_reason: end_turn` with text response. OLMo 7B is not fine-tuned for tool invocation — describes how to use `cat` instead of invoking the tool. Shim code is correct (no llama-server format errors); model capability is the limit.
- Confirmed Goose v1.36.0 installed at `/usr/local/bin/goose`.
- QEMU vm-mediakit PID updated in NEXT.md: 3949093 → 4039898 (project-infrastructure restarted it).

**Pending / carry-forward:**
- **QEMU vm-mediakit** (PID 4039898, -accel tcg software emulation): system load 17+. Confirm with project-infrastructure owner; `kill 4039898` to unblock inference and enable §7.2 Goose test.
- **§7.2 Goose chat round-trip**: `ANTHROPIC_HOST=http://127.0.0.1:9080 ANTHROPIC_API_KEY=foundry-local GOOSE_MODEL=claude-haiku-4-5-20251001 goose run --text "Say hello"`. Blocked by CPU saturation.
- **§7.3 tool_use in Doorman log**: OLMo 7B does not invoke tools. Options: (a) wait for Yo-Yo VM with OLMo 3 32B-Think, (b) upgrade Tier A to a tool-use-tuned model (e.g. Qwen2.5-7B-Instruct), (c) mark §7.3 as "not achievable with current Tier A model".
- **§7.4 entity extraction**: Yo-Yo VM must be started to close Tier B circuit. Command: `service-slm/scripts/start-yoyo.sh --runtime=2h`.
- **Stage 6 promote**: archive is 32+ commits ahead of origin/main (Command Session scope; prereq rebase per `command-20260520-stage6-rebase-required`).
- **Binary ledger**: `data/binary-ledger/slm-doorman-server.jsonl` in workspace (Command Session scope). SHA256: `9e8542b6...` (slm-doorman-server rebuilt 2026-05-29T02:14Z).

**Operator preferences surfaced:**
- Working autonomously on verification; blocked items documented with clear next-action commands

