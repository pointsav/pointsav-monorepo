# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

---

## 2026-05-23 | Totebox | claude-code

**Done this session — pre-build polish; Phases 1–5 fully closed:**
- **`openapi.yaml` accuracy pass (Commit N, `826d42a5`)** — 15 missing routes added: Phase 5 `/es/` + `/es/wiki/{slug}`, auth/pending special pages (`/special/logout`, `/special/pending-changes`, `/special/pending/{id}`, accept/reject, contributions), `/api/complete`, `/api/preview/{slug}`, `/category/{name}`, `/talk/{slug}`. Category enum fixed (company + help added, order corrected). Collab flag reference removed. Phase-5 tag added.
- **Accept-Language → `/es/` redirect (Commit O, `c2d4010c`)** — `prefers_spanish()` helper; `IndexQueryParams.noredirect: Option<String>`; ES home lang-toggle href → `/?noredirect=1`; 4 integration tests. Full suite green.
- **README refresh (Commit P, `7a7beb46`)** — EN + ES: Phase 2 row collab removed; Phase 5.1 bilingual routing marked shipped; missing `<div align="center">` in EN README fixed.
- **NEXT.md + BRIEF updated** — Stage 6 count → 16; Phase 5 marked closed; Phase 6 gate conditions documented.

**Stage 6 — ready for tonight's build:**
16 commits unpromoted on monorepo `main`. All pass `cargo test` + `cargo clippy -D warnings`. Promote via `~/Foundry/bin/promote.sh` (Command scope) → binary rebuild → `sudo systemctl restart` all 3 services.

**Pending / carry-forward:**
- **Phase 6 (three-instance deployment split)** — GATED. Two prerequisites not yet cleared: (1) `content-wiki-*` → `media-knowledge-*` GitHub rename (operator doing manually), (2) MASTER Doctrine amendment for source-of-truth inversion. No Totebox work until Command confirms both gates clear.
- **§3.4 continuous citation verification** — own sub-project; needs `reqwest` + background scheduler. Deferred indefinitely.
- **§3.6 claim-record MCP API** — cross-cluster; waiting on project-intelligence reply re: `slm-mcp-server` reconciliation.
- **WCAG token fix** (#878d99 → ~#767c8a) — project-design scope; outbox sent 2026-05-22.
- **`leapfrog-facts.es.yaml`** — ES DYK content; project-editorial scope.

**Operator preferences surfaced:**
- Pre-build sessions: wants everything lined up and committed before shutdown, minimal interruptions ("can we lin eit all up so I can leave ti on auto").
- Batch all remaining in-scope work and shut down clean — don't leave partial tasks.

---

## 2026-05-22 | Totebox | claude-code

**Done this session — Phase 1 + Phase 2 complete; Phase 3 Commits A–C:**
- **Branch-topology blocker found + escalated.** The monorepo's `cluster/project-knowledge` branch had diverged badly from `main` (a stale 2026-05-03 relic, 374 commits behind). Escalated to Command; Command deleted it — **`main` is the confirmed working branch** for project-knowledge engine work.
- **Phase 2 (claim-authoring convention)** — specced, staged, routed; Command ratified it as **doctrine claim #54** (`~/Foundry/conventions/claim-authoring-convention.md`). LANDED notice sent to project-editorial (their Track-A2 unblocked).
- **Phase 1 (engine dead-code descope) COMPLETE** — 4 commits on monorepo `main`: `8f51ddfc` templates, `959f8e6f` Doorman, `bf35f38d` MCP read tools, `3d9cd9ec` collab. ~−2,600 lines; full `cargo test` green. Fixed a latent implicit-dep bug (tokio `io-util` was transitively via axum `ws`).
- **Phase 3 (claim-layer engine) — Commits A/B/C** on `main`: `7887f8ec` (`claim.rs` extractor + Engine Verification Gate discharged), `c41bf85e` (per-claim citation resolution), `77e0d0a8` (claim graph in redb).
- Wrote `.agent/binary-targets.yaml` — SOFT- pipeline declaration for `app-mediakit-knowledge`.

**Pending / carry-forward (resolved next session):**
- Phase 3 Commits D–E, crate hygiene, docs accuracy pass, Phase 4 DTCG, Phase 5 bilingual routing — all completed 2026-05-22/23.
- Stage 6 count: was 7 at close of this session; now 16.

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
- All execution phases subsequently completed 2026-05-22/23.

**Operator preferences surfaced:**
- Collapse plan sprawl: one upstream vision + one execution plan per cluster; old plans deleted once the new exist.
- "We Own It" — sovereign in-house stack; the engine is custom-built, not adopted (saved as memory `principle-we-own-it`).
- Respect archive boundaries — draft for other clusters, route via outbox; never write/delete in another archive.
- Decisive, fast pace; reframe a question when asked rather than pushing it; confirm decisions in batches.
