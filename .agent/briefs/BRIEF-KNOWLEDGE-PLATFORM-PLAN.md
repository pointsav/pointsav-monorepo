---
artifact: brief
status: active
---

# Knowledge Platform — project-knowledge Execution Plan

> **Created:** 2026-05-21 · Totebox@claude-code · project-knowledge cluster
> **Type:** execution plan (downstream). **Upstream:** `KNOWLEDGE-PLATFORM-VISION.md`.
>
> This is project-knowledge's single comprehensive execution todo for the
> knowledge platform. It **replaces** `KNOWLEDGE-PLATFORM-REBUILD-PLAN.md`, the
> Gen-2 blueprint set, the Wikipedia-parity plans, and the older research docs
> (see §11 — disposition). It does **not** restate the vision — it references
> it by section. project-editorial's parallel plan is its own document; the two
> synchronize per Vision §14.
>
> All six §12 vision decisions are **confirmed** (operator, 2026-05-21) and
> baked in below.

---

## 1. Decisions baked into this plan

| # | Decision | Confirmed |
|---|---|---|
| 1 | Adopt the claim-native data model (MVL). Convention-first sequencing. | ✅ |
| 2 | Adopt the pairing contribution model. Engine `auth.rs`/`pending.rs` removed *after* the `os-mediakit` broker exists — never before. | ✅ |
| 3 | MCP reversal — keep transport, delete redundant tools, re-found as claim-query + contribution API; reconcile with `service-slm`'s `slm-mcp-server`, do not duplicate. | ✅ |
| 4 | `INVENTIONS.md` descope — Bitcoin anchoring / W3C VC / federation / adapter algebra leave the engine's scope, recorded as a future `project-disclosure` tenant (tenant NOT created now). | ✅ |
| 5 | `os-console` / `os-mediakit` build is a distinct later phase (Phase 7) — cross-project scope; does not block near-term engine/content work. | ✅ |
| 6 | Production binary ~16 commits behind canonical, Stage 6 blocked — cleared first, as Phase 0. | ✅ |

---

## 2. Phase 0 — Stage 6 unblock (Command Session scope)

Precondition for everything user-visible. project-knowledge Totebox cannot run
`promote.sh`; this is a Command Session action. Tracked via outbox.

- [ ] **0.1** Stage 6 promote `cluster/project-knowledge` HEAD → canonical (`echo "y" | ~/Foundry/bin/promote.sh`).
- [ ] **0.2** `cargo build --release` in `app-mediakit-knowledge/`.
- [ ] **0.3** `sudo cp` binary + restart the wiki service(s).
- [ ] **0.4** Verify `/healthz` on all live ports.
- [ ] **0.5** Reconcile the `content-wiki-corporate` cluster/canonical divergence before its Stage 6.
- [ ] **0.6** Carried: D10 wikilink validation pass once the rebuilt binary serves the bare-slug resolver.

---

## 3. Phase 1 — Engine descope (dead code) — no external dependency

The *dead*-code removals from Vision §10. None depend on `os-mediakit`; safe to
do now. **Not** in this phase: `auth.rs` / `pending.rs` (Phase 7 — gated).

- [ ] **1.1** Remove the MCP redundant read tools (`search_topics`, `get_revision`, `list_backlinks`) — keep the MCP transport (Decision 3; Vision §8).
- [ ] **1.2** Remove real-time collab (`collab.rs`, `--enable-collab`, the yjs route + bundle).
- [ ] **1.3** Remove the Doorman proxy stubs (`/api/doorman/*`) — dead, no `WIKI_DOORMAN_URL` deploy.
- [ ] **1.4** Delete the dead `templates/*.html` (engine renders via `maud`; unreferenced).
- [ ] **1.5** Shrink `openapi.yaml` to the surviving public read routes (or drop — decide at edit time).
- [ ] **1.6** One pass for the `AppState` field deletions + the ~20 test-fixture edits they ripple to (batch, not per-feature). Run the full test suite.

---

## 4. Phase 2 — Claim-authoring convention spec — immediate, small, fast

The convention-first deliverable (Decision 1). This **unblocks project-editorial's
12 flagship TOPIC rewrites** — they rewrite against it once, no double-touch.

- [x] **2.1** Spec the inline claim-authoring convention: the markdown syntax for "this span is a claim" + its citation set. Hard requirement: **degrades gracefully** — claim-annotated markdown renders correctly on *today's* engine (annotations inert/invisible); the future engine extracts structure. No separate sidecar file. [2026-05-21 totebox@claude-code — HTML-comment carrier; `render.rs:181` `unsafe=true` confirms pass-through]
- [x] **2.2** Spec the per-claim fields a claim carries: ID, content hash, citation set, `valid_at`, `published_at`, confidence grade. [2026-05-21 totebox@claude-code — split into 4 authored fields + 4 engine-derived fields; `content_hash`/`published_at` are derived, never authored]
- [x] **2.3** Stage the convention as a CONVENTION-* artifact; route a copy to project-editorial (they need it for their Track A2 rewrites) and to project-design if it affects rendering tokens. [2026-05-21 totebox@claude-code — `.agent/plans/claim-authoring-convention.PROPOSAL.md`; routed to Command (ratify), project-editorial + project-design (outbox)]
- [ ] **2.4** Cross-cluster: notify project-editorial the convention has **landed** → their 12 rewrites proceed against it. *Heads-up sent 2026-05-21; LANDED confirmation pending Command ratification.* [2026-05-21 totebox@claude-code]

> **Carry-forward — Engine Verification Gate.** The convention §3 names one
> unverified fact: comrak with `render.rs:167–182` options + `unsafe=true` must
> emit `<!--claim …-->` markers into output HTML unchanged. Owner: Phase 3.1
> (a render-pass test discharges it).

---

## 5. Phase 3 — Claim layer engine implementation (the MVL)

The core leapfrog build — Vision §9. Follows Phase 2 (the convention must exist first).

- [ ] **3.1** `Claim` struct in the data model; render-time extraction from claim-annotated markdown (`render.rs`).
- [ ] **3.2** Per-claim citation resolution against `citations.yaml` (extend the existing resolver from per-article to per-claim).
- [ ] **3.3** Claim graph in the redb store, alongside the existing wikilink graph (`depends_on` / `cited_by`).
- [ ] **3.4** Continuous citation verification — background re-fetch + re-hash of cited sources; drift surfaced in the citation ribbon.
- [ ] **3.5** Two-clock temporality — `valid_at` + `published_at` per claim; freshness ribbon reads from it; `?asof=` view from git history.
- [ ] **3.6** Claim-record MCP API — `query_claims(topic, asof)` returning cited, freshness-tagged claim records (Decision 3). Reconcile with `service-slm`'s `slm-mcp-server` — coordinate via outbox; do not duplicate.
- [ ] **3.7** `GET /wiki/{slug}` JSON content-negotiation → `{frontmatter, body_md, blake3, revision_sha, backlinks}` (Vision §6).
- [ ] **3.8** Enrich `jsonld.rs` — `dateModified`, `description`, `citation` (from resolved `cites:`), `version`, `keywords`.

---

## 6. Phase 4 — Token wiring & visual

Wire the design system into the engine. **Dependency:** use the current
`knowledge.*` bundle — `wiki.*` was renamed and freshness-ribbon tokens removed
(Vision §14); project-design must declare which `dtcg-bundle.json` is canonical
before this phase starts.

- [ ] **4.1** Outbox to project-design: which `dtcg-bundle.json` is canonical (`tokens/` vs `dtcg-vault/tokens/`)? Blocking question.
- [ ] **4.2** Build-time transform: DTCG bundle → CSS variables vendored into `static/`. Colors as `oklch()`.
- [ ] **4.3** Reconcile the engine's hand-authored `:root` block with the DTCG semantic names.
- [ ] **4.4** Theme switching — `theme-pointsav` / `theme-woodfine` swap the full token set, not just `--accent`.
- [ ] **4.5** WCAG 4.5:1 verification on every token text/UI pair; fix at the token source.
- [ ] **4.6** REJECTED, do not build: `feDisplacementMap` refraction, `backdrop-filter` on scrolling content, `saturate(180%)` (Vision §2). `backdrop-filter: blur(8px)` on the fixed sticky header only is permitted.

---

## 7. Phase 5 — Bilingual `/es/` routing (Phase E)

Implements project-editorial's `design-phase-e-bilingual-routing.draft.md`
(inbox `command-20260520-phase-e-forward`). Self-contained engine work.

- [ ] **5.1** Steps §12.1–9 of the Phase E draft, in one commit: `Locale` enum; `/es/` + `/es/wiki/{*slug}` routes; `home_inner()` / `article_inner()` refactors; `lang=` threaded through chrome; language switcher; hreflang tags.
- [ ] **5.2** `load_dyk_localized()` + the three `leapfrog-facts.es.yaml` (project-editorial produces the content).

---

## 8. Phase 6 — Three-instance deployment split

Vision §3/§4. **Gated on:** the operator's `content-wiki-* → media-knowledge-*`
GitHub rename, and the MASTER Doctrine amendment (Vision §11).

- [ ] **6.1** Replace the one-binary/three-content-dirs multiplexing with three isolated deployment instances — `media-knowledge-{documentation,projects,corporate}`, one engine instance each.
- [ ] **6.2** Each instance: a git clone of its own `media-knowledge-*` content repo; per-instance config; gitignored runtime state.
- [ ] **6.3** Wire the instance-canonical / GitHub-downstream publication chain (Vision §4b).
- [ ] **6.4** Ownership split honored: documentation = PointSav instance; projects + corporate = Woodfine instances.

---

## 9. Phase 7 — Contribution model (cross-project, later)

Decision 5 — distinct later phase. Spans `os-console` + `os-mediakit` (both
`Hello, world!` scaffolds today). Does **not** block Phases 0–6.

- [ ] **7.1** `os-mediakit` — the pairing broker: issues scoped/expiring pairings; mints two capability classes (draft pairings vs human-only promotion capability); enforces F12 (Vision §5).
- [ ] **7.2** `os-console` — the contributor client: holds identity, embeds the editor surface, signals F12.
- [ ] **7.3** Reposition the engine's in-browser editor (`edit.rs`, CodeMirror, SAA squiggles, citation autocomplete) as an `os-console`-embedded component / library — remove the public `/edit` route.
- [ ] **7.4** **Only after 7.1 lands:** remove `auth.rs` / `users.rs` (accounts) and `pending.rs` (moderation queue) from the engine; engine keeps pairing-token verification only (Decision 2 sequencing).
- [ ] **7.5** Cross-project coordination: this phase touches projects beyond project-knowledge — surface scope to MASTER.

---

## 10. Phase 8 — Editorial linter integration

The `validate_editorial_standards` capability. **Dependency:** project-editorial's
Track D ruleset (one ruleset, two consumers — Vision §14).

- [ ] **8.1** Receive project-editorial's canonical editorial ruleset (their Track A0/D deliverable) — the **Gate-0-reconciled** standard, not the raw blueprint.
- [ ] **8.2** Engine-side `validate_editorial_standards` consumes that ruleset — no second rule set authored here.
- [ ] **8.3** Expose it as an MCP tool on the re-founded MCP surface (or on `slm-mcp-server` — reconcile per 3.6).

---

## 11. Standing items & old-plan disposition

**Main Page ownership** (Vision §5) — project-knowledge owns each wiki's Main
Page (`index.md`, `featured-topic.yaml`, `leapfrog-facts.yaml`, the grid).
project-editorial supplies recommended lede drafts + reviews lede prose. Ongoing,
maintenance cadence — not a phase.

**`INVENTIONS.md` descope** (Decision 4) — one-time records move: mark the
Bitcoin/VC/federation/adapter material as out-of-engine-scope, belonging to a
future `project-disclosure` tenant. The tenant is not created now.

**Old-plan disposition (project-knowledge `.agent/plans/`).** Once this plan is
ratified, delete the superseded knowledge-platform plans. Proposed delete set
(operator to confirm before removal):

- *Delete:* `KNOWLEDGE-PLATFORM-REBUILD-PLAN.md`, `MASTER-EXECUTION-BLUEPRINT.md`, `KNOWLEDGE-PLATFORM-CONSOLIDATED-RESEARCH-AND-BLUEPRINT.md`, `RESEARCH-UI-UX-SPRINT.md`, `THREE-WIKI-REBUILD-MASTER.md`, `DESIGN-TOKENS-SPEC.md`, `ARTICLE-FRAMEWORK-SPEC.md`, `leapfrog-2026.md`, `PHASE-6B-DID-IDENTITY.md`, `init-task-state.md`, `d5-canonical-message-sprint1.md`, `task3-task4-session-2026-05-19.md`.
- *Archive to `archive/`:* `WIKIPEDIA-PARITY-MASTER-PLAN.md`, `WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md`, `WIKIPEDIA-PARITY-RESEARCH-LOG.md` (parity work complete; research-log value).
- *Keep — knowledge platform:* `KNOWLEDGE-PLATFORM-VISION.md`, this file, `README.md`.
- *Keep — unrelated workstreams (NOT in scope):* `service-slm-architecture-2026.md`, `service-content-architecture-2026.md`, `service-slm-hardening-2026-05-18.md`, `service-audit-2026-05-16.md`, `sovereign-routing-comprehensive.md`, `universal-ai-gateway.md`, `learning-loop-master-plan-2026-05-18.md`, `tier-architecture-2026.md`, `MASTER-PLAN-2026.md`, `lbug-build-blocker.md` (review individually — may be non-kp).
- Update `NEXT.md` to point at this plan; clear any stray `~/.claude/plans/` or `~/.gemini/tmp/` knowledge-platform files.

---

## 12. Cross-cluster handoffs

- **→ Command:** Phase 0 (Stage 6); the Doctrine amendment (Vision §11).
- **→ project-editorial:** the claim-authoring convention (2.3/2.4); receive their editorial ruleset (8.1) and recommended Main Page lede drafts.
- **→ project-design:** the canonical `dtcg-bundle.json` question (4.1).
- **→ project-intelligence:** MCP reconciliation with `slm-mcp-server` (3.6).

---

## 13. Sequencing summary

```
Phase 0 (Stage 6 — Command)         ─ clear first, independent
Phase 1 (dead-code descope)         ─ now, no deps
Phase 2 (claim convention spec)     ─ now, small/fast — unblocks project-editorial
Phase 3 (claim layer build)         ─ after Phase 2
Phase 4 (token wiring)              ─ after project-design declares canonical bundle
Phase 5 (bilingual /es/)            ─ self-contained, any time after Phase 1
Phase 6 (three-instance split)      ─ gated on rename + Doctrine amendment
Phase 7 (contribution model)        ─ later; cross-project; gates the auth/pending removal
Phase 8 (editorial linter)          ─ gated on project-editorial's ruleset
```

*Authored 2026-05-21 from `KNOWLEDGE-PLATFORM-VISION.md` rev 4 + the six confirmed §12 decisions.*
