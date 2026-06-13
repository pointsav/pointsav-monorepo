# NEXT.md — project-software
# NEXT.md — project-knowledge

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> Scope: this archive only. Cross-repo items route via outbox.

Last updated: 2026-06-12
Last updated: 2026-06-13 [totebox@claude-code]

---

## Session-start procedure

1. `get_session_brief(role="totebox", archive="project-software")`
2. `~/Foundry/bin/foundry-role.sh`
3. Write `.agent/engines/claude-code/session.lock`
4. Read `.agent/rules/scope-discipline.md` + `datagraph-discipline.md`

---

## Active items

*(none — 2026-06-12 session cleared all outstanding inbox and restaged 3 TOPIC drafts)*
- [x] Refactor `AppState` to `mounts: Vec<Mount>`; delete hardcoded content/guide dir fields — dea5e8ae [2026-06-10]
- [x] Wire `blueprints.rs` into render pipeline — AppState loading (dea5e8ae); `relates_to` rail in `wiki_page_inner` (bd435cc3) [2026-06-11]
- [x] `tokens.css` regenerated from `dtcg-bundle.json`; added back to git tracking — bd435cc3 [2026-06-11]
- [x] Slug normalization: `/wiki/topic-foo` → 301 → `/wiki/foo`; `topic-foo.md` fallback; ES-locale aware — bd435cc3 [2026-06-11]
- [x] L25: `/edit/{slug}` route stub + CodeMirror 6 bundle + `toc-persistence.js` + conditional chrome load — bd435cc3 + 7a2b9b42 [2026-06-11]
- [x] M8/M5: Mobile drawer animations + tap-popover flip + Cmd+K trigger — 7a2b9b42 [2026-06-11]
- [x] Stage 6 promote for bd435cc3 + 7a2b9b42 — CONFIRMED; origin/main at 7a2b9b42 [2026-06-11]
- [x] `inject_wiki_prefixes` cross-mount resolution — DONE; link_roots() used at all call sites [2026-06-11]
- [x] Wire `check --strict` as xtask CI gate — `scripts/stage6-gate.sh` committed 9a1326df [2026-06-11]
- [x] Remove `wikilink-unresolved` render path from `render.rs` — DONE 9a1326df; display text only (L18 complete) [2026-06-11]
- [x] Stage 6 promote for 9a1326df — CONFIRMED; origin/main at 9a1326df [2026-06-12]
- [x] Content dead link fix — project-editorial applied fix; gate passes 689/0/0 [2026-06-12]
- [ ] Archive ops Stage 6 — 4e2ddf95 → 76671ddd + this session commits pending Command promote; binary rebuild + redeploy required after. [2026-06-12 totebox@claude-code]

---

## Smoke test fix — nightly binary ledger false failure

- [ ] Add `/health` alias to `app-mediakit-knowledge` (server/mod.rs) — `deploy-binary.sh` checks `/health` but binary only had `/healthz`; fix committed this session; Stage 6 READY pending. [2026-06-13 totebox@claude-code]

---

## Out-of-scope (tracked via outbox)

- Stage 6 promotion — Command Session runs `bin/promote.sh` after project-editorial
  confirms receipt of 6 TOPIC drafts.
  Signal: `project-software-20260612-editorial-dispatch-3-topics-restaged`
