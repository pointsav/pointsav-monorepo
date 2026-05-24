# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

---

## 2026-05-24 | Totebox | claude-code

**Done this session — design commission + 3 live fixes:**
- **Housekeeping (A1–A2):** cleanup-log committed; session-start.md rewritten (Phase state, working branch, gotchas); BRIEF renamed `BRIEF-knowledge-platform.md`; README updated.
- **Drafts-outbound triage (C1–C4):** 6 duplicate files git-rm'd; 9 landed articles moved to `archived/`; 3 misrouted project-intelligence drafts flagged to Command (C3); stale collab article flagged to project-editorial (C4).
- **Outbox maintenance (E1):** 7 superseded messages marked actioned.
- **5-agent OPUS design commission (2610f6ca):** DESIGN-RESEARCH-visual-language, DESIGN-RESEARCH-ux-writing, DESIGN-RESEARCH-service-design, DESIGN-SPEC-header-footer, DESIGN-RESEARCH-token-architecture — 3,700 lines total; routed to project-design. Key findings: IVC band leaks "Phase 7"; WCAG root at `text.tertiary`; recipe drift on home grid; duplicate `id="header-search-q"`; DS-ADR-07 conflict with CDN fonts.
- **3 live-issue fixes (23deea11):** IVC band text scrubbed ("Phase 7" removed); WCAG `#878d99→#666c78` (4 token locations); dtcg-to-css.py cubicBezier emit bug fixed. Tests + clippy green.
- **Stage 6:** 17 commits unpromoted on monorepo main. Build request outboxed to Command (`project-knowledge-20260524-session-close`).

**Pending / carry-forward:**
- **Phase 6 (three-instance split)** — still gated on: (1) `content-wiki-*` → `media-knowledge-*` GitHub rename; (2) MASTER Doctrine amendment. No Totebox work until Command confirms both.
- **Design commission** — project-design has the 5 drafts; MASTER COSIGN required on DTCG changes; ratified DESIGN-SPEC-header-footer returns to project-knowledge for implementation.
- **DS-ADR-07 amendment** — CDN font conflict flagged to Command; needed before font-loading token work (DESIGN-RESEARCH-token-architecture Stage E).
- **§3.4 continuous citation verification** — deferred; needs reqwest + background scheduler.
- **§3.6 claim-record MCP API** — waiting on project-intelligence re: slm-mcp-server reconciliation.
- **leapfrog-facts.es.yaml** — ES DYK content; project-editorial scope.
- **Misrouted project-intelligence drafts** — 3 files in drafts-outbound; Command to re-route.

**Operator preferences surfaced:**
- Auto mode approved for multi-agent commissions; minimal interruptions.
- Favours parallel OPUS agent launches for research; commits all outputs in one batch.

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

