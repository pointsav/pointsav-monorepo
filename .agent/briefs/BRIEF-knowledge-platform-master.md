---
artifact: brief
status: active
title: Knowledge Platform — Master Brief
cluster: project-knowledge
updated: 2026-06-08
supersedes: archive/BRIEF-knowledge-platform-master.md
verdict_source: .agent/drafts-outbound/BRIEF-REVIEW-old-brief-verdict.md
---

# BRIEF — Knowledge Platform Master

Durable decision register for `app-mediakit-knowledge`. The verdict panel
(three-Opus review, 2026-06-04) found **0 decisions REJECTED**, 15 CARRY verbatim,
9 REVISE (enforcement clauses added), and 10 new decisions (L20–L29).

**Rule for this BRIEF:** every load-bearing decision names its own acceptance test
and merge gate. "Locked" means verifiable, not aspirational.

---

## §1 — Cluster Mission

`app-mediakit-knowledge` is a Wikipedia-pattern HTTP knowledge wiki for `os-mediakit`.
It substitutes for MediaWiki per Doctrine claim #29 ("We Own It" — sovereign in-house
engine; GitHub is a downstream mirror, not a dependency). The engine serves Git-native
Markdown content over three instances:

| Instance | Domain | Port | Content |
|---|---|---|---|
| documentation | `documentation.pointsav.com` | 9090 | `content-wiki-documentation` |
| projects | `projects.woodfinegroup.com` | 9093 | `content-wiki-projects` |
| corporate | `corporate.woodfinegroup.com` | 9095 | `content-wiki-corporate` (planned) |

Single Rust binary. No runtime system dependencies. Apache 2.0.

---

## §2 — Status Snapshot (2026-06-08)

| Phase | Label | State | Notes |
|---|---|---|---|
| Phase 1 | Render foundation | Shipped | `/wiki/{slug}`, `static/`, `/healthz`; modular src/ scaffold |
| Phase 2 | Wikipedia chrome | Shipped | Article/Talk/History tabs, TOC, hatnotes, language switcher, footer |
| Phase 3 | Search + feeds | Shipped | Tantivy BM25, `/feed.atom`, `/feed.json`, `/sitemap.xml`, `/robots.txt`, `/llms.txt` |
| Phase 4 | Git sync + MCP | Shipped | `git2`, history/blame/diff, redb wikilink graph, blake3; MCP JSON-RPC 2.0; git smart-HTTP; OpenAPI 3.1 |
| Phase 5 | Auth + edit review | Shipped (core) | Cookie sessions, argon2id, edit review queue; ACLs/SSO/webhooks deferred (BP5) |
| Phase 6 | Three-instance split | Gated | Awaits: content-wiki-* GitHub rename + MASTER Doctrine amendment |
| Phase 7 | MCP federation | Designed | ActivityPub + cross-instance queries |
| Phase 8 | Token theming | Shipped | DTCG token layout vars + `knowledge.toml` templates |
| Phase 0 | Federation engine | NEXT MILESTONE | Unblocked; see §4 |
| Phase 9 | Production deploy | Command-gated | Stage 6 promote + DESIGN-TOKEN-CHANGE cosign; see §4 |

Sub-clone tip: `2381a169` (22 commits ahead of canonical; rebase aborted — conflict report in outbox).

---

## §3 — Decision Register (L1–L29)

### CARRY verbatim — 12 decisions

**L2** — Git-native flat-file content store (`.md` + `git2`). 50-year-readable, diffable,
underpins F12 audit trail. No schema migration ladder. Databases/indexes are derived state only.

**L7 / §9** — Canonical footer trademark text, byte-for-byte. Only the year field updates.

**L8** — Typeface roster: Inter (UI + headings), Source Serif 4 (body), system mono; WOFF2 only.
Operator-approved 2026-06-01. Supersedes old Oswald/Nunito/Roboto stack.

**L9** — Brand token triad: `--navy: #164679`; `--bg: #F7F9FA`; `--link: var(--navy)`.
WCAG AA verified. Protected by routing through L3's vault.

**L10** — MCP JSON-RPC 2.0 native (`src/mcp.rs`). Shipped differentiator.

**L11** — Claim-layer HTML comment markup. In production; foundation for claim-rail.

**L12** — SYS-ADR-07: no structured data through AI. Constitutional hard rule.

**L13** — SYS-ADR-10: F12 mandatory; human commits only. Collab dead-code removal must
not weaken this gate.

**L14** — SYS-ADR-19: no automated AI publishing to verified ledgers. Constitutional.

**L15** — Apache 2.0 licence. Legal invariant.

**L16** — Commit identity `jwoodfine`/`pwoodfine` only via `commit-as-next.sh`. Pre-commit
gate enforced.

**§4** — Cross-instance isolation is structural; never a global `[[slug]]` resolver. Security
invariant — a global resolver would leak corporate slugs into documentation.

---

### REVISE — 9 decisions with enforcement clauses

**L1** — Single Rust binary deployment unit (Doctrine #54 "We Own It"). Single binary ≠
single source file ≠ single JS bundle: internals MUST be modular (see L20) and client assets
MUST be route-scoped (see L25).

**L3** — `dtcg-bundle.json` is the single source of truth for all CSS custom properties.
Per-brand outputs (`tokens.css`, `tokens-woodfine.css`) are GENERATED through `dtcg-to-css.py`.
No hand-authored token/theme CSS may coexist with the generated bundle; brand variation is a token
override in the vault, not a new file. (`theme-woodfine.css` to be folded in and deleted — see L21.)

**L4** — Bilingual EN+ES via `.es.md` sibling on a single canonical slug. Bilingual scope
includes chrome: all reader-visible strings (nav, headings, footer, featured/recent titles) come
from a `strings(locale)` map; `/es/` MUST prefer the `.es` sibling's title.
Acceptance: rendering `/es/` asserts zero hardcoded-English chrome strings (see L22).

**L5** — Self-hosted WOFF2 fonts, no CDN (GDPR Art. 44 — non-negotiable legal invariant).
Self-hosting carries its own loading contract: each above-the-fold face MUST emit
`<link rel=preload as=font type=font/woff2 crossorigin>` plus metric-override fallback (see L23).

**L6** — Adopt Wikipedia Vector 2022 information-model conventions (wikilinks, Article/Talk/History,
hatnotes); visual language is Stripe/Linear. Chrome rendering lives in one parameterised `chrome.rs`
emitter — never three inline `*_chrome` copies in the handler file.

**L17** — Mobile-first: base stylesheet = phone, desktop via `min-width`. Enforcement checklist
(per release): `env(safe-area-inset-bottom)` APPLIED (not merely defined) on all `position:fixed`/
sticky chrome and body bottom padding; `viewport-fit=cover`; `≥16px` inputs; `dvh` not `100vh`.
Per-release phone smoke test required (see L24).

**L18** — SPLIT.
- CARRY (load-bearing): Build-time wikilink resolver is a HARD GATE — any unresolved `[[ ]]`
  across topics + all federated mounts BLOCKS promote. This gate is a precondition of the "zero
  dead links" claim; it MUST land before the red-link render path is removed.
- REJECT (red-link affordance): delete `wikilink-missing` emission and invert the render test
  to assert no red-link — but only AFTER the gate exists, so dead links are never invisible.

**L19** — Federation via declarative `Vec<Mount>` + content-type blueprints. Completion-gated,
not "locked-done": `AppState` carries `mounts: Vec<Mount>` (the hardcoded `content_dir`/
`guide_dir`/`guide_dir_2` fields are DELETED); `blueprints.rs` is wired into render;
`inject_wiki_prefixes` resolves across the full mount set. No Phase 2+ visual work merges to
canonical while the old path survives or any instance is unwired.

**§11.3** — GUIDEs stay in `woodfine-fleet-deployment` (Foundry TOPIC/GUIDE taxonomy). The
L18 build-time gate MUST resolve `[[guide-slug]]` across all federated guide mounts; until
cross-mount resolution exists, guides land in fleet-deployment BEFORE any article references
them (content-sequencing rule — see L29).

---

### NEW locked decisions — L20–L29

**L20** — Source-file size discipline: no source file exceeds ~1,500 lines / 60 KB.
`server.rs` decomposes into `routes.rs`, `chrome.rs`, `state.rs`, `pages/`, `walker.rs`
along the concern boundaries enumerated in the old §16.

**L21** — Exactly three CSS artifacts permitted: `style.css` (shared), `tokens.css` (PointSav),
`tokens-woodfine.css` (Woodfine). `theme-woodfine.css` is folded in and deleted. Any new `.css`
file requires a Decision-Log entry. Binds L3's "single vault" to a hard file count.

**L22** — Chrome strings are locale-keyed via `strings(locale)`; `/es/` prefers the `.es` sibling
title; test asserts zero hardcoded-English chrome on `/es/`. Operationalizes L4.

**L23** — Font preload mandatory: `<head>` emits `<link rel=preload as=font type=font/woff2 crossorigin>`
for the two latin regular faces, alongside the metric-override fallback. Completes L5/L8 loading
contract. Prevents FOIT/CLS.

**L24** — Safe-area insets APPLIED not merely defined: `padding-bottom: calc(56px + var(--safe-b))`
on all fixed bottom chrome AND body; CSS lint enforces inset presence. Operationalizes L17.

**L25** — Route-gated client bundles: editor assets (CodeMirror, SAA) load only on `/edit/*` via
`editor.js`; article/home ship only `wiki.js` + `toc-persistence.js`. No payload rule existed
previously; per-route payload budget tracked in acceptance.

**L26** — Dead-code removal is a tracked deliverable on every plan rebase: removed feature's
module, vendor bundle, routes, and tests are deleted in the same change and logged. Superseding
briefs carry forward predecessor removal actions. Closes the consolidation-loss root cause
(collab dead code).

**L27** — List micro-layouts carry an explicit separator element in markup (not CSS-only);
recent-changes title/date use a separator element or block structure.

**L28** — DNS provisioning is a named deliverable: each §1 instance row carries a DNS-status
field and owning session (Command). Reconcile `documentation.pointsav.com` vs
`documentation.woodfinegroup.com` before cutover.

**L29** — No article may reference an uncommitted guide/topic slug. Same build-time resolver
as L18 blocks promote on unresolved `[[ ]]`; content-sequencing rule applies until the gate
exists. Same mechanism as L18 gate; the resolver is implemented once and referenced from both.

---

## §4 — Phase Roadmap

### Phase 0 — Federation engine (NEXT UNBLOCKED milestone)

**Hard merge gate: no Phase 2+ feature or visual work reaches canonical while any Phase 0
item is incomplete.**

Scope:
- Refactor `AppState` to `mounts: Vec<Mount>`; delete hardcoded `content_dir`/`guide_dir`/`guide_dir_2`
- Wire `blueprints.rs` into render pipeline
- Implement `inject_wiki_prefixes` cross-mount resolution
- Build-time dead-link gate (`check --strict` passes; any unresolved `[[ ]]` blocks promote)
- Slug normalization across all mounts
- Remove red-link render path (`render.rs` — `wikilink-missing` emission) after gate exists

Completion test: `knowledge.toml` is the live source of truth for all three instances;
`check --strict` passes with 0 dead links; red-link path absent from source.

### Phase 9 — Production deploy (Command-gated)

Gate 1: Stage 6 promote — outbox signal `project-knowledge-20260608-stage-6-ready-2381a169`
(rebase conflict report also in outbox; Command must choose resolution strategy A/B/C/D first).

Gate 2: DESIGN-TOKEN-CHANGE `master_cosign` from Command Session on
`.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md`.

Checklist: `.agent/drafts-outbound/PHASE-9-DEPLOY-CHECKLIST.md`.
Note: `/etc/local-knowledge/` does NOT yet exist on host — must be created before deploy steps.

### Post Phase 9 (after production verify)

- L21: Fold `theme-woodfine.css` into DTCG vault (DESIGN-TOKEN-CHANGE pipeline to project-design)
- L20: Decompose `server.rs` along concern boundaries (routes, chrome, state, pages, walker)
- L25: Audit + gate CodeMirror/editor bundle to `/edit/*` only
- Mobile polish: M8 drawer animation, tap-popover positioning, Cmd+K affordance
- Phase 6 (three-instance split): gated on content-wiki-* GitHub rename + Doctrine amendment

---

## §5 — Items Awaiting Operator Input

**Per-brand editorial gravitas contract** — approximately 12 tokens beyond the accent: density,
serif headings, drop-cap gating. Needs brand-design direction before Phase 5 instance
differentiation can proceed.

**`tokens.css` / `config/*.toml` deletion intent** — restored from git HEAD in commit `ff7cd16d`
(both are referenced in source and needed for Phase 9 deploy per checklist). Confirm if these
should be deleted before or after production deploy.

**`check --strict` as pre-promote gate** — Command can wire once project-editorial has triaged
the 17 dead links + 6 missing-slug guides surfaced by the June 2026 content audit.

**Sub-clone rebase** — 4 resolution options in outbox (msg-id: `project-knowledge-20260608-rebase-conflict-report`).
Command must choose strategy before Stage 6 promote.

**DNS reconciliation (L28)** — `documentation.pointsav.com` is live; `documentation.woodfinegroup.com`
status unknown. Confirm target domain per instance before Phase 6 cutover.
