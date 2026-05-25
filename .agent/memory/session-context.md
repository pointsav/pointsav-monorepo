# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

---

## 2026-05-25 | Totebox | claude-code

**Done this session — round-2 design competition + self-hosted fonts:**
- **Round-2 design competition:** Three OPUS agents produced new prototypes (platform-document / institutional-register / editorial-standard). OPUS jury scored proto-platform-document 84/100 as winner; institutional-register contributed three grafts (three-row header, ledger stripe, article-meta DL). editorial-standard rejected.
- **Jury spec produced:** `DESIGN-WIKI-REDESIGN-SPEC.draft.md` (416 lines) in `.agent/drafts-outbound/`.
- **Redesign implemented (`70259d32`):** CSS rewritten from 4,362 → 2,347 lines using platform-document as base. Three-row header (utility/brand/nav-row, 32/72/48px). Oswald + Nunito Sans + Roboto Slab. Ledger stripe. Article-meta DL. Dark mode `#0B1220` canvas. 106 tests pass.
- **Self-hosted fonts (`784ceea7`):** Command blocked `70259d32` for GDPR Art. 44 — Google Fonts CDN transfers IPs to Google US. Downloaded Nunito Sans + Roboto Slab WOFF2 (latin + latin-ext) into `static/fonts/`; added 6 `@font-face` declarations to `style.css`; removed all Google Fonts `<link>` tags from `server.rs`. Zero third-party origin calls. Tests pass.
- **Both commits queued for Stage 6.** Outbox updated.

**Pending / carry-forward:**
- **Stage 6 — two commits pending** (`70259d32` + `784ceea7`): Command runs `bin/promote.sh` + `deploy-binary.sh` + restart 3 services.
- **Phase 6 (three-instance split)** — still gated on GitHub rename + MASTER Doctrine amendment.
- **§3.6 claim-record MCP API** — waiting on project-intelligence.
- **§3.4 continuous citation verification** — deferred.
- **Phase 5.1+** — gated on BP5.

**Operator preferences surfaced:**
- Design direction: "completely different approach" — wanted to move away from Wikipedia-clone aesthetic. Platform-document (Stripe/Linear institutional) was the right call.
- Responsive to Command blocks: accepted self-hosting fix immediately ("yes").

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


