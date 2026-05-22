# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

---

## 2026-05-22 | Totebox | claude-code

**Done this session — Phase 1 + Phase 2 complete; Phase 3 Commits A–C:**
- **Branch-topology blocker found + escalated.** The monorepo's `cluster/project-knowledge` branch had diverged badly from `main` (a stale 2026-05-03 relic, 374 commits behind). Escalated to Command; Command deleted it — **`main` is the confirmed working branch** for project-knowledge engine work.
- **Phase 2 (claim-authoring convention)** — specced, staged, routed; Command ratified it as **doctrine claim #54** (`~/Foundry/conventions/claim-authoring-convention.md`). LANDED notice sent to project-editorial (their Track-A2 unblocked).
- **Phase 1 (engine dead-code descope) COMPLETE** — 4 commits on monorepo `main`: `8f51ddfc` templates, `959f8e6f` Doorman, `bf35f38d` MCP read tools, `3d9cd9ec` collab. ~−2,600 lines; full `cargo test` green. Fixed a latent implicit-dep bug (tokio `io-util` was transitively via axum `ws`).
- **Phase 3 (claim-layer engine) — Commits A/B/C** on `main`: `7887f8ec` (`claim.rs` extractor + Engine Verification Gate discharged), `c41bf85e` (per-claim citation resolution), `77e0d0a8` (claim graph in redb).
- Wrote `.agent/binary-targets.yaml` — SOFT- pipeline declaration for `app-mediakit-knowledge`.

**Pending / carry-forward (resume tomorrow):**
- **Phase 3 Commit D — start here.** §3.5 two-clock temporality. **Operator chose Option A**: per-span `gix-blame` for `published_at` (not coarse file-level). Plus `?asof=` past-revision view. Then Commit E (§3.7 JSON content-negotiation + §3.8 JSON-LD). Full pick-up detail: `.agent/briefs/BRIEF-knowledge-platform-phase3.md`.
- **Deferred:** §3.4 continuous citation verification (own sub-project); §3.6 claim-record MCP API — outbox sent to project-intelligence for `slm-mcp-server` reconciliation.
- **Stage 6:** 7 commits unpromoted on monorepo `main` (Phase 1 ×4 + Phase 3 ×3) — Command scope.
- **Crate-hygiene drift (pre-existing):** `app-mediakit-knowledge` is not `cargo fmt`/`clippy -D warnings`-clean — standalone task logged in `NEXT.md`.
- **Crate-doc accuracy pass:** crate `CLAUDE.md`/`ARCHITECTURE.md` still describe collab/Doorman/removed-MCP-tools as shipped — logged in `NEXT.md`.

**Operator preferences surfaced:**
- Per-commit discipline endorsed — small, compile-and-test-verified commits; present a plan for multi-step engine work, then execute on go-ahead.
- Surface drift, don't bundle it — pre-existing fmt/clippy non-compliance was reverted out of the Phase 1 descope, not silently absorbed into a feature commit.

---

## 2026-05-21 | Totebox | claude-code

**Done this session — knowledge-platform re-architecture + plan consolidation:**
- Major strategic session. Dispatched recon agents (engine + design-system audit) and a 4-agent OPUS research sweep (leapfrog-2030 vision, AI-native contribution model, machine-readable/flat architecture, build-vs-adopt).
- Authored `KNOWLEDGE-PLATFORM-VISION.md` (rev 4, `.agent/plans/`) — upstream vision & architecture. And `KNOWLEDGE-PLATFORM-PLAN.md` — project-knowledge's 8-phase execution plan.
- **Six decisions confirmed by operator:** (1) claim-native data model — convention-first; (2) pairing contribution model (web-login retired; os-console ↔ os-mediakit); (3) MCP reversal — keep transport, re-found as claim+contribution API; (4) `INVENTIONS.md` disclosure machinery descoped → future `project-disclosure` tenant; (5) os-console/os-mediakit a later cross-project phase; (6) Stage 6 first.
- Settled: custom Rust engine continues (not MediaWiki); three sovereign deployment instances; source-of-truth inversion (instance content repo canonical, GitHub downstream); rename `content-wiki-* → media-knowledge-*` (operator doing the GitHub side); Main Page owned by project-knowledge.
- Cross-checked with project-editorial — accepted; reconciled (Gate-0 editorial standard; one-ruleset/two-consumers linter). Recorded as Vision §14.
- Cleaned `.agent/plans/`: 12 superseded plans deleted, 3 Wikipedia-parity archived. Committed `80d50931` (Jennifer).
- Drafted project-editorial's plan → `.agent/drafts-outbound/KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.draft.md`.
- Outbox: Doctrine-amendment request to Command (`project-knowledge-20260521-doctrine-amendment-request`); consolidated-plan handoff + strict 7-step cleanup to project-editorial (`project-knowledge-20260521-editorial-plan-handoff`).

**Pending / carry-forward:**
- **Execution not started.** `KNOWLEDGE-PLATFORM-PLAN.md` Phase 0 (Stage 6 — Command) and Phases 1–2 (dead-code descope + claim-authoring convention spec) are the next Totebox work. Start here next session.
- **Awaiting:** Command relay + response on the Doctrine amendment; project-editorial to finalize+commit its plan, run the strict cleanup, and reply.
- `~/.claude/plans/` (~60 stray plan-mode files) — decided: left as-is (inert scratch).
- Carried: D10 wikilink validation; production binary ~16 commits behind canonical, Stage 6 blocked.

**Operator preferences surfaced:**
- Collapse plan sprawl: one upstream vision + one execution plan per cluster; old plans deleted once the new exist.
- "We Own It" — sovereign in-house stack; the engine is custom-built, not adopted (saved as memory `principle-we-own-it`).
- Respect archive boundaries — draft for other clusters, route via outbox; never write/delete in another archive.
- Decisive, fast pace; reframe a question when asked rather than pushing it; confirm decisions in batches.

---

## 2026-05-20b | Totebox | claude-code

**Done this session:** Startup sequence only. No work performed — operator issued shutdown immediately after startup.

**Pending / carry-forward:**
- **Items 6 + 7 (start here next):** corporate glossary expansion (`glossary-corporate.csv`, 459 rows, many incomplete) + documentation wiki thin-category audit.
- **D10:** wikilink validation pass — blocked on Stage 6 binary rebuild (Command Session scope).
- **G2 canonical:** `README-TOTEBOX-EGRESS.md` still in `/srv/foundry/customer/woodfine-fleet-deployment/` — Command Session `git rm` + admin-tier commit needed.
- **Stage 6 outstanding:** content-wiki-projects (6 commits), content-wiki-corporate (10 commits, blocked on cluster/canonical divergence), content-wiki-documentation (4 commits), monorepo (16 commits).

**Operator preferences surfaced:** None new this session.
