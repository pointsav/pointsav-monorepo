---
artifact: brief
schema: foundry-brief-v1
brief-id: knowledge-platform-master
owner: project-knowledge
status: active
title: Knowledge Platform — Master Brief
cluster: project-knowledge
updated: 2026-06-17
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

## §2 — Status Snapshot (2026-06-10)

| Phase | Label | State | Notes |
|---|---|---|---|
| Phase 1 | Render foundation | Shipped | `/wiki/{slug}`, `static/`, `/healthz`; modular src/ scaffold |
| Phase 2 | Wikipedia chrome | Shipped | Article/Talk/History tabs, TOC, hatnotes, language switcher, footer |
| Phase 3 | Search + feeds | Shipped | Tantivy BM25, `/feed.atom`, `/feed.json`, `/sitemap.xml`, `/robots.txt`, `/llms.txt` |
| Phase 4 | Git sync + MCP | Shipped | `git2`, history/blame/diff, redb wikilink graph, blake3; MCP JSON-RPC 2.0; git smart-HTTP; OpenAPI 3.1 |
| Phase 5 | Auth + edit review | Shipped (core) | Cookie sessions, argon2id, edit review queue; ACLs/SSO/webhooks deferred (BP5) |
| Phase 6 | Three-instance split | **Shipped 2026-06-14** | Doctrine v0.1.2 §IV.g committed 2026-06-11; GitHub renames confirmed; all three instances serving correct content; gate 689/0/0; xtask wired; red-link path removed. One cleanup: staging-j remotes for media-knowledge-projects/corporate still point to old content-wiki-* names — relayed to project-editorial via outbox. |
| Phase 7 | MCP federation | Designed | ActivityPub + cross-instance queries |
| Phase 8 | Token theming | Shipped | DTCG token layout vars + `knowledge.toml` templates |
| Phase 0 | Federation engine | **Complete 2026-06-12** | All scope items done; gate 689 articles / 0 dead links; Stage 6 at 9a1326df; archive ops pending Stage 6 |
| Phase 9 | Production deploy | **Shipped 2026-06-11** | WIKI_KNOWLEDGE_TOML migration; /etc/local-knowledge/; all 3 instances healthy |

Sub-clone tip: `c5658afe` (Session 88, 2026-06-17). Sprints D–N committed (J–N this session, 2026-06-17); Stage 6 READY (4th) sent to Command (msg-id: command-20260617-stage-6-ready-app-mediakit-knowledge-spr). Live binary still at `8599b14c` pending rebuild. Sprints S/P/Q/T (Lapfrog 2030 + marketing header parity) planned in Session 89; pending plan approval before implementation.

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

### Phase 0 — Federation engine — **COMPLETE 2026-06-12**

**Hard merge gate: no Phase 2+ feature or visual work reaches canonical while any Phase 0
item is incomplete.**

Scope:
- ~~Refactor `AppState` to `mounts: Vec<Mount>`; delete hardcoded `content_dir`/`guide_dir`/`guide_dir_2`~~ — DONE dea5e8ae
- ~~Wire `blueprints.rs` into render pipeline (AppState loading + `relates_to` rail in `wiki_page_inner`)~~ — DONE dea5e8ae + bd435cc3
- ~~`tokens.css` regenerated from `dtcg-bundle.json`; added back to git tracking~~ — DONE bd435cc3
- ~~Slug normalization: `/wiki/topic-foo` → 301 → `/wiki/foo`; `topic-foo.md` file fallback; ES-locale aware~~ — DONE bd435cc3
- ~~L25: `/edit/{slug}` route stub; CodeMirror 6 bundle; `toc-persistence.js` extracted; conditional chrome load~~ — DONE bd435cc3 + 7a2b9b42
- ~~M8/M5: Mobile drawer animations; tap-popover viewport flip; Cmd+K visible trigger~~ — DONE 7a2b9b42
- ~~Implement `inject_wiki_prefixes` cross-mount resolution~~ — DONE (already present at all call sites; confirmed this session)
- ~~Wire `check --strict` as xtask CI gate (blocks promote on any unresolved `[[ ]]`)~~ — DONE 9a1326df (`scripts/stage6-gate.sh`; xtask check-content across 3 mounts)
- ~~Remove red-link render path (`render.rs` — `wikilink-missing` emission)~~ — DONE 9a1326df (display text only; gate active; L18 complete)

Completion test: `knowledge.toml` is the live source of truth for all three instances (**DONE** — Phase 9);
`check --strict` passes with 0 dead links — **DONE 2026-06-12** (689 articles / 0 dead links / 0 missing fields; project-editorial fix applied);
red-link path absent from source — **DONE 9a1326df**.

### Phase 9 — Production deploy — **DEPLOYED 2026-06-11**

Gate 1: Stage 6 promote — **CONFIRMED** in canonical (git log origin/main..HEAD = 0); tip `0e18aff3`.

Gate 2: **CLEARED** — DESIGN-TOKEN-CHANGE committed by project-design at `af51d86`
(2026-06-09); ACK received in inbox and actioned 2026-06-10. Tokens: nav-h, sidebar-w,
bottom-bar-h, reading-measure, safe-area-bottom.

**Deployed 2026-06-11 by Command Session:**
- `/etc/local-knowledge/` created; documentation.toml + projects.toml + corporate.toml installed
  (mount paths corrected to `project-editorial/media-knowledge-*`)
- 3 unit files rewritten: `WIKI_KNOWLEDGE_TOML` env var added; old env vars removed
- `wiki-content-repoint.conf` drop-ins removed from all 3 `.service.d/` dirs
- All 3 instances restarted and verified: 9090 (PointSav Documentation), 9093 (Woodfine Projects),
  9095 (Woodfine Corporate) — healthz=ok, font preloads=2, search=200
- Binary ledger entry: sha256 `e5e8995efc7d6da2f1eba10c235161a90e6c4290aa2b65951c54eb92948c8cd1`

**Remaining Totebox work:** ~~fix TOML template paths in sub-clone~~ — DONE 2026-06-11 (ece90408):
config/*.toml corrected; static/tokens.css removed; NEXT.md updated.

### Post Phase 9 (after production verify)

- ~~L21: Fold `theme-woodfine.css` into DTCG vault~~ — DONE; exactly 3 CSS files (style.css, tokens.css, tokens-woodfine.css); theme-woodfine.css removed
- ~~L20: Decompose `server.rs` along concern boundaries~~ — DONE; server/ (mod.rs + 4 handlers), chrome/, routes/, state.rs, walker.rs; all files ≤1,287 lines
- ~~L25: Audit + gate CodeMirror/editor bundle to `/edit/*` only~~ — DONE (7a2b9b42); editor.js + cm-saa.bundle.js load only on `/edit/*`
- ~~Mobile polish: M8 drawer animation, tap-popover positioning, Cmd+K affordance~~ — DONE (7a2b9b42)
- Phase 6 (three-instance split): gated on content-wiki-* GitHub rename + Doctrine amendment

---

## §5 — Items Awaiting Operator Input

**Per-brand editorial gravitas contract** — approximately 12 tokens beyond the accent: density,
serif headings, drop-cap gating. Needs brand-design direction before Phase 5 instance
differentiation can proceed.

**`tokens.css` / `config/*.toml` deletion intent** — ~~RESOLVED 2026-06-11~~: operator chose
"delete tokens.css only; keep config/*.toml as templates." Committed at ece90408.

**`check --strict` as pre-promote gate** — DONE 9a1326df: `scripts/stage6-gate.sh` wired; `wikilink-unresolved` span removed from render.rs (L18 complete). Gate currently reports 1 dead link (`substrate-without-inference-base-case.es.md` → fix sent to project-editorial).

**Sub-clone rebase** — ~~Resolved 2026-06-10~~. Merge strategy chosen; post-merge tip `ca6ae410` → `8480f68e`; Stage 6 signal updated in outbox. No further action needed.

**DNS reconciliation (L28) — RESOLVED 2026-06-14** — All three `/etc/local-knowledge/*.toml`
configs confirmed correct:

| Port | Domain | Brand | Blueprint set |
|------|--------|-------|---------------|
| 9090 | documentation.pointsav.com | pointsav | TOPIC + GUIDE |
| 9093 | projects.woodfinegroup.com | woodfine | TOPIC only |
| 9095 | corporate.woodfinegroup.com | woodfine | TOPIC only |

`documentation.woodfinegroup.com` was correctly omitted; the PointSav brand instance
uses the `pointsav.com` domain. Phase 6 cutover complete.

---

## §6 — Session Log

### 2026-06-16 | totebox | claude-code (Session 87 — cargo gates + BRIEF updates)

Cargo check + test: 200 passed, 0 failed, 1 pre-existing ignored (integrity_bar). 14.9 GB RAM available; stale lock from prior session — no active cargo processes.
Stage 6 READY signal sent to Command: msg-id `command-20260617-stage-6-ready-app-mediakit-knowledge-bin`; sub-clone canonical `3d90e76d`.
NEXT.md: project-intelligence contamination (66 lines) removed; D/E/F/G + cargo gate items checked off; committed `a7171768` (pwoodfine).
BRIEF §2/§6/§8.4/§8.5 updated this session. Artifact registry E9 → Stage 6 READY.
§8.4 decisions resolved: #1 LOCKED (use draft §8.2 mapping); #2 DECIDED (Woodfine instances get own IA; candidates proposed).
Sprint H (ActivityPub wiring) executed this session — see §8.5.

---

### 2026-06-16 | totebox | claude-code (Sprint D/E/F/G — comprehensive plan execution)

Sprints D, E, F, G executed in AUTO mode. All code changes in `app-mediakit-knowledge/`. Stage 6 pending — compile gate in progress.

**Phase 1 (engine defects):**
- Defect 2 (footnote CSS): prophylactic `sup/sub/footnotes` CSS added to `style.css`
- M13 (/openapi.json redirect): `GET /openapi.json` route added → 301 to `/openapi.yaml`

**Sprint D (home page peers band):**
- `home_handlers.rs`: `peers: &[PeerConfig]` param added to `home_chrome()`; `.peer-band` aside rendered when peers non-empty
- `style.css`: `.peer-band` + `.peer-band__label` + `.peer-band__link` + `.peer-band__arrow` CSS added
- `wiki_handlers.rs`: call site updated to pass `&state.peers`

**Sprint E (article chrome):**
- `render.rs`: `audience: Vec<String>` + `aliases: Vec<String>` fields added to `render::Frontmatter` (correct struct used by wiki_handlers)
- `walker.rs`: same fields added to `walker::Frontmatter` (used by check/walker)
- `wiki_handlers.rs`: audience chips rendered below H1; `resolve_alias_slug()` async fn + 301 alias redirect in NotFound path
- `style.css`: `.audience-chips` + `.audience-chip` CSS added

**Sprint F (header/footer chrome):**
- `home_handlers.rs`: `p.footer-version { "app-mediakit-knowledge v" (env!("CARGO_PKG_VERSION")) }` added to `shell_footer()`
- `style.css`: `.footer-version` CSS added
- `wiki_handlers.rs`: `peers: &[PeerConfig]` added to `wiki_chrome()` signature + call site; `.peer-strip` nav rendered above `.shell`
- `style.css`: `.peer-strip` + `.peer-strip__label` + `.peer-strip__link` CSS added

**Sprint G (search enhancements):**
- `mod.rs`: `search_complete` now returns `{title, slug, lede}` (lede = `short_description` or first paragraph snippet)
- `mod.rs`: `SearchQueryParams` extended with `category: Option<String>` + `status: Option<String>`
- `mod.rs`: `search_page` applies post-search category (slug-prefix) + status (frontmatter read) filters

Cargo check running (blocked on package cache lock from project-intelligence session).

---

### 2026-06-16 | totebox | claude-code (Stage 6 confirmed; live audit)

Stage 6 confirmed via binary ledger: sub-clone tip `d0abd9ad` promoted Session 86
(2026-06-16T16:55:53Z); sha256 `a2bcfce2ee310d26c144e8e7b6784ce8cc6860f73832ff67a26506ba5340528b`;
smoke_test=pass; all 3 instances restarted and healthy.

Live audit (HTTP probe + route verification): / → 200 on all 3; sitemap absolute URLs
confirmed (E7 live: `<loc>https://documentation.pointsav.com/wiki/...`); data-instance
per-instance correct — documentation/projects/corporate (M14 fix live); internal nav
links use clean slugs, no `topic-` prefix (M1 partial confirmed); images route registered
(E8; `/images/missing.png` → 404 is correct behaviour for absent files). Outstanding
project-editorial items F1–F7 pending responses. M10 502 flapping pending Command load-test gate.

BRIEF §2 sub-clone tip updated; §8.3 Stage 6 reference updated; §8.5 Sprint 0/A/B/C
status updated from "pending"/"planned" to confirmed. Sprint D (home page editorial
redesign) is the next engineering sprint.

---

### 2026-06-14 | totebox | claude-code (Phase 6 shipped; Phase 7 scaffold)

Phase 6 closed: GitHub renames confirmed by operator; DNS verified from live TOML files
(L28 RESOLVED); all prerequisites met. Phase 6 status updated to "Shipped 2026-06-14".
Outbox message sent to Command to relay staging-j remote update for media-knowledge-projects
and media-knowledge-corporate to project-editorial.

Phase 7 scaffold committed (sub-clone `6d554ec6`):
- `config.rs`: `PeerConfig` struct + `peers: Vec<PeerConfig>` in `AppConfig`
- `server/mod.rs`: `peers: Vec<PeerConfig>` added to `AppState`
- `main.rs`: `peers` param threaded through `serve()` + TOML/legacy paths wired
- `mcp.rs`: `"knowledge/search"` federated search method — local BM25 + peer fan-out
- `src/activitypub.rs`: new stub — `Actor`, `Article`, `CreateActivity`, `on_article_saved()`
- `lib.rs`: `pub mod activitypub;` registered
- `Cargo.toml`: `reqwest = { version = "0.12", features = ["json", "rustls-tls"] }` added
Compiles clean (cargo check); Stage 6 READY sent.

Sub-clone contamination fixed: `pointsav-monorepo/CLAUDE.md` rewritten from "project-design"
to generic monorepo guide; `pointsav-monorepo/.agent/rules/brief-discipline.md` heading fixed.

Archive state files: manifest.md corrected to project-knowledge tetrad (local-only; gitignored).

---

### 2026-06-13 | totebox | claude-code (contamination recovery)

Status restored to `active`. project-console Totebox had written `status: archived` and a
`contaminated_note` into this file after finding a copy of it in project-console's archive.
The original in project-knowledge was legitimate and active; project-console should not have
modified a foreign archive's file. Contamination fields removed; `updated` bumped to 2026-06-13.
NEXT.md and briefs/README.md also restored from reflog `c33a2747` (pre-rebase content).
tantivy E0119 vendor-patch (`a1c9238b`) Stage 6 READY pending Command.

---

### 2026-06-12 | totebox | claude-code (close-out)

Phase 0 complete. Stage 6 confirmed at 9a1326df (origin/main updated). Dead link fix applied by
project-editorial (`substrate-without-inference-base-case.es.md` line 26). Gate verified:
689 articles / 0 dead links / 0 missing fields (exit 0). BRIEF Phase 0 → Complete; completion
test all three conditions DONE. NEXT.md Phase 0 section marked COMPLETE. Archive ops commits
(4e2ddf95 → e6d01e9c) Stage 6 READY sent to Command. Binary rebuild + redeploy to 9090/9093/9095
required from Command after archive ops promote.

---

### 2026-06-12 | totebox | claude-code

Phase 0 gate commit (9a1326df): `scripts/stage6-gate.sh` xtask runner; `wikilink-unresolved` span
removed from render.rs (display text only; L18 complete); `inject_wiki_prefixes` cross-mount wiring
confirmed already present at all call sites. Gate reports 1 dead link in project-editorial content
(`substrate-without-inference-base-case.es.md`); fix sent to project-editorial (msg-id:
command-20260612-dead-link-fix-needed-substrate-without-i). Stage 6 READY sent (msg-id:
command-20260612-stage-6-ready-project-knowledge-sub-clon). NEXT.md pre-promote code-fix items
closed (both already implemented; 129 tests pass); Phase 6 standing deferred split into code/content
entries (4e2ddf95). BRIEF §2 tip + Phase 0 scope + completion test + Post Phase 9 updated. L21/L25/M8
marked done. Only remaining Phase 0 item: project-editorial content fix (1 dead link).

---

### 2026-06-11 | totebox | claude-code

Phase 0 work committed in two commits (bd435cc3 + 7a2b9b42). Covered: tokens.css
regenerated and re-tracked; blueprints `relates_to` rail wired into `wiki_page_inner`;
topic- slug normalization (301 locale-aware redirect + `topic-foo.md` file fallback,
including `topic-foo.es.md` for ES locale); `/edit/{slug}` route stub; CodeMirror 6
bundle built (`cm-saa.bundle.js`); `editor.js` init script; `toc-persistence.js` extracted
from `wiki.js`; conditional chrome loading (editor assets on `/edit/*` only); M8 mobile
drawer CSS (transform + transition; `:not([aria-hidden])` open state; overlay fade-in);
tap-popover viewport flip; Cmd+K visible trigger button + `window.openCmdK` exposure.
129 tests green. Clippy clean. Stage 6 READY sent to Command
(msg-id: `command-20260611-stage-6-ready-project-knowledge-sub-clon`).

---

### 2026-06-11 | command | claude-code

Phase 9 production deploy complete. Stage 6 confirmed in canonical (tip 0e18aff3). `/etc/local-knowledge/`
created; 3 TOML configs installed with corrected `project-editorial/` mount paths; 3 unit files rewritten
to `WIKI_KNOWLEDGE_TOML`; `wiki-content-repoint.conf` drop-ins removed; all 3 instances restarted and
verified healthy (9090/9093/9095). Binary ledger entry appended. Outbox contamination alert actioned;
Stage 6 READY outbox actioned. artifact-registry A1 → DEPLOYED. BRIEF §4 Phase 9 updated.
NEXT.md Phase 9 carry-forward cleared.

---

### 2026-06-14 | totebox | claude-code (engine repairs — Sprint A/B)

Engine repairs from the audit findings (§7) committed in two sub-clone commits.

**Commit 1 — `3106b2e1` (pwoodfine):** `fix(style): WCAG 2.2 focus outline + article body link underline (C3/M15)`
- C3: `:focus-visible` outline changed from `var(--accent)` (gold, 2.26:1) → `var(--interactive-focus-ring, var(--navy))` (9.1:1). Passes WCAG 2.2 §1.4.11.
- M15: `.article__body a { text-decoration: underline; }` added after the global `a { text-decoration: none }` rule. Chrome/nav links unchanged.

**Commit 2 — `48bfa7e7` (jwoodfine):** `fix(app-mediakit-knowledge): audit repairs M1/M11/M12/M14 — sitemap absolute URLs, ES tab i18n, edit form POST removed, brand-instance from TOML`
- M1 (sitemap): `canonical_url: Option<String>` added to `SiteConfig` and `AppState`. Sitemap emits `{canonical_url}/wiki/{slug}` when set; falls back to relative when unset.
- M14 (data-instance): `instance: Option<String>` added to `SiteConfig`. `brand_instance` resolution in `main.rs` now prefers TOML `[site].instance` over `WIKI_BRAND_INSTANCE` env var. Command Session must add `instance =` line to each `/etc/local-knowledge/*.toml`.
- M12 (ES i18n): Article tab labels (Article/Talk/History/Edit), breadcrumb Home, and mobile bottom-bar Edit/History all route through inline `match locale { Locale::En => "...", Locale::Es => "..." }` in `wiki_handlers.rs`. Uses the local server `Locale` enum (not `crate::chrome::Locale` — incompatible types). `html lang` on `/edit/` left as `lang="en"` — edit_page handler has no locale param and there is no `/es/edit/` route.
- M11 (dead POST form): `form.edit-page-form method="post" action={...}` → `form.edit-page-form` (no method, no action). Edit view remains as a read-only stub consistent with git-only posture.
- All `AppState` initializers in `src/server/misc_handlers.rs` (12 entries) and 18 test files updated with `canonical_url: None`. Dedup pass applied to `misc_handlers.rs` (sed double-inserted; Python dedup fixed).
- 129 tests pass. Clippy clean. 7 pre-existing infobox_test failures (301 routing on legacy infobox routes) — not introduced by these changes.

**Items investigated and dropped:**
- **M3 (category grid 1 card):** Not an engine bug. `HOMEPAGE_CATEGORIES` count is already dynamic via `.len()`; the "1 card" renders because articles lack `category:` frontmatter. Content gap, not engine defect. Relayed to project-editorial.
- **M4 (literal `[[..]]` leak + claim comment bleed):** `strip_claim_markers()` already called unconditionally at `wiki_handlers.rs:443`. Adding a `wikilink-missing` span contradicts L18 (zero dead links hard gate) and `render.rs` tests at lines 917–921 assert `!html.contains("wikilink-missing")`. Both sub-items skipped; any `[[..]]` visible in prose is a content issue (literal brackets in article text, not wikilink syntax). Relayed to project-editorial.
- **M13 (/llms.txt 502):** Handler uses `tokio::fs::read_to_string` already (fully async). Deterministic 502 = upstream process instability (C1 root cause), not a handler bug. Relayed to Command via outbox as smoke test addition.

**Outbox messages sent:**
- → Command (high priority): C1 restart + `Restart=on-failure` for all 3 units + nginx `proxy_next_upstream`; M10 concurrency profiling + load test gate; M14 add `instance =` line to each `/etc/local-knowledge/*.toml`; M13 add `/llms.txt` to smoke test suite.
- → project-editorial (high priority): C2 tier semantics reconciliation (critical — CBRE/review blocker); M2 onboarding chip 404s; M5 guide catalog hatnote (public presentation of operator-only links); M6 TOPIC/GUIDE drift + M3 category frontmatter gap; M7 snapshot dating; M8 OSM ODbL attribution (legal obligation); M9 EN/ES parity sweep.
- Stage 6 READY sent to Command.

**Pending after this session:** Command must add `instance =` TOML lines + rebuild binary + restart units before M14 is observable. Project-editorial to action C2/M2/M5/M6/M7/M8/M9.

---

### 2026-06-14 | totebox | claude-code (12-agent external audit)

External audit run via 12-agent Opus workflow. Agents fetched live pages from all three instances,
cross-checked against BRIEF decisions L1–L29, benchmarked against Wikipedia Vector 2022 / Stripe
/ Linear / MDN, and synthesized findings. Full findings in §7.

**Overall grades:** BRIEF completion B+ · Institutional UI/UX A− · Content quality C+

**Critical (3):** C1 documentation.pointsav.com 502 mid-audit (Phase 9 gate falsified); C2 tier
semantics contradiction T1=highest vs T1=lowest across articles; C3 WCAG 2.2 focus-outline
colour failure (gold 2.26:1 on white).

**Major engine issues (8):** M1 301 redirect tax on all navigation + relative sitemap URLs;
M3 category grid "6 areas" renders 1 card; M4 literal [[..]] leaking into prose; M11 dead POST
/edit form (Phase 5 superseded); M12 ES chrome strings hardcoded English; M13 /llms.txt 502;
M14 data-instance mislabel on all instances; M15 article body links colour-only (WCAG 1.4.1).

**Major content issues (7, project-editorial scope):** M2 dead onboarding chips (4× 404); M5
guide catalog dead links; M6 TOPIC/GUIDE drift in archetype articles; M7 inconsistent cluster
counts (7,594 vs 6,493); M8 OSM ODbL attribution missing (legal); M9 EN/ES parity lag; M10
intermittent 502 flapping under concurrency.

Repair plan committed to NEXT.md. Engine fixes (C3, M1, M3, M4, M11–M15) are Totebox scope.
Content fixes (C2, M2, M5–M9) are project-editorial scope. C1/M10 are Command scope.

---

### 2026-06-10 | totebox | claude-code (continuation — post-compaction)
Inbox triage (stale re-send + 2 actioned + 2 contaminated archived). artifact-registry A2→B
(Gate 2 CLEARED af51d86). DESIGN-TOKEN-CHANGE draft marked cosigned-and-committed. Binary
rebuilt from ca6ae410 (12M, 2026-06-10). PHASE-9-DEPLOY-CHECKLIST updated (binary + Gate 2
checked off). BRIEF-project-intelligence contamination archived. NEXT.md rebuilt from
project-gis contamination. MEMORY.md header fixed. Phase 0 AppState mounts refactor committed
(eeb60cbb); fmt/clippy fixes (10670370, 0e18aff3). Stage 6 READY re-sent multiple times;
current tip 0e18aff3 (outbox: project-knowledge-20260610-stage-6-ready-0e18aff3). Archive
`main` contaminated via `git reset --hard origin/main` (project-intelligence content) — recovered
via reflog to 0b45bb3e. Phase 9 Gate 1 (Stage 6) still pending Command.

---

## §7 — Audit Findings 2026-06-14 (12-agent external audit)

**Auditors:** 12 Opus agents — UI/UX ×2, content quality ×2, functionality, mobile/performance,
BRIEF completion verification, accessibility, typography/visual identity, corporate instance,
web research ×2. 16 agents total including scouts and synthesis.

**Verdict:** The engine is genuinely institutional-grade (A− UI/UX). The content spine has
critical integrity problems (C+ content). The flagship instance went down mid-audit (C1).

---

### §7.1 — Critical Issues

**C1 — documentation.pointsav.com upstream 502 (Phase 9 gate falsified)**
- Observed: nginx 502 Bad Gateway for 8+ minutes during audit, never recovered.
- Impact: Flagship PointSav property serves raw nginx errors to external visitors.
- Fix: `systemctl restart local-knowledge-documentation.service`; add `Restart=on-failure` to
  unit file; nginx `proxy_next_upstream` retry; external uptime probe. **Command scope.**
- Gate: Phase 9 completion cannot be re-asserted until `/healthz` is stable under sustained probe.

**C2 — Tier semantics contradiction (T1 = best vs T1 = worst)**
- Observed: `co-location-ranking-system` defines T1 = ★ (lowest, "Anchor Only", T5 = highest).
  Every other article (tier-nomenclature, archetypes, regional-markets-system) defines T1 = highest.
- Impact: A practitioner reading two articles draws opposite conclusions. CBRE/peer-review blocker.
- Fix: Rewrite ranking-system's "Quality Tiers" to T1 = highest (the rest of the site is already
  correct) OR explicitly separate the 5-star quality index from the T1–T3 compositional tier.
  Add authoritative [[co-location-tier-nomenclature]] wikilink from every tier-using article.
  **Project-editorial scope.**

**C3 — Focus outline fails WCAG 2.2 §1.4.11 non-text contrast**
- Observed: `:focus-visible { outline: 2px solid var(--accent) }` where --accent = gold #C7A961.
  Contrast ratio: 2.26:1 on white, 2.09:1 on page bg — below 3:1 minimum.
- Fix: `outline: 2px solid var(--interactive-focus-ring, var(--navy))` (navy = 9.1:1).
  **Totebox scope — style.css.**

---

### §7.2 — Major Issues: Engine (Totebox scope)

**M1 — 301 redirect tax on all internal navigation + relative sitemap `<loc>`**
- Every home link and sitemap URL uses `/wiki/topic-<slug>` → 301 → `/wiki/<slug>`.
  Canonical `<link rel=canonical>` points to clean slug, so sitemap and canonical disagree.
- Fix: `home_chrome()` and sitemap generator emit `/wiki/<slug>` directly; add absolute URLs
  and `<lastmod>` to sitemap. Keep 301 only as legacy safety net.

**M3 — Home portal grid: "6 areas" label renders 1 category card**
- Category grid query filters out all but one populated category. Articles breadcrumb to
  "Governance" which is absent from the grid.
- Fix: Fix grid enumeration query; reconcile displayed count with rendered cards; make
  Governance first-class in the grid.

**M4 — Literal `[[slug|Label]]` leaks in prose; `<!--/claim-->` bleeds into rendered HTML**
- documentation.pointsav.com: 7 literal unrendered wikilinks; 8 stray `<!--/claim-->` HTML
  comments in prose; `[[totebox-os]]` resolves to wrong path (404).
- Fix: Wikilink resolver validates slugs against published set; emit a styled "wanted" anchor
  (using existing `--color-link-redlink` token) for unresolved slugs instead of raw `[[..]]`.
  Strip `<!--/claim-->` markers in render stage. Add to build dead-link gate.

**M11 — Phase 5 edit surface superseded but dead form survives**
- POST /edit → 405; `/edit` view self-labels read-only but still emits `<form method=post>`.
  OpenAPI, UI, and HTTP behavior all disagree. Phase 5 effectively descoped (git-only).
- Fix: Remove dead POST form from `/edit` view; add BRIEF note marking Phase 5 superseded;
  align OpenAPI (currently claims /edit exists as a 200 but with "Q1: git-only" note).

**M12 — `/es/` leaks English chrome strings (L4 partial)**
- Article tab labels (Article/Talk/History), search placeholder, search aria-labels,
  skip-to-content, and `html lang="en"` all remain English on /es/ pages.
- Fix: Route all chrome strings through `strings(locale)` i18n map; set `lang="es"` on ES
  variants; add /es/ chrome scan to xtask gate (fail on known English tokens in ES context).

**M13 — `/llms.txt` hard 502; `/openapi.json` missing**
- `/llms.txt` returns 502 deterministically (handler registered but fails). OpenAPI only at
  `/openapi.yaml`, not `.json`. Both affect AI/tool discovery.
- Fix: Debug or unregister /llms.txt handler; add it to smoke test suite. Optionally serve
  `/openapi.json` via `Content-Type: application/json` redirect or separate handler.

**M14 — `data-instance="documentation"` emitted on all three instances**
- projects.woodfinegroup.com and corporate.woodfinegroup.com both emit `data-instance="documentation"`.
  Used for CSS/JS instance-scoping; incorrect value means per-instance styles may not apply.
- Fix: Emit `data-instance` from `knowledge.toml` `[site]` instance name field (already present
  as `brand_instance` from `WIKI_BRAND_INSTANCE` env var — wire it through `AppState` to the
  HTML `<body>` attribute).

**M15 — Article body links colour-only (WCAG 1.4.1)**
- `a { color: var(--link); text-decoration: none; }` globally — inline prose links differ
  from body text by colour alone.
- Fix: `.article__body a { text-decoration: underline; }` — reserve no-underline for chrome/nav.

---

### §7.3 — Major Issues: Content (project-editorial scope)

**M2 — Dead onboarding chips ("New here? Start with these" → 4× 404)**
- Most prominent first-run CTA dead-ends on all four links on projects + corporate home.
- Fix: Author four start-here TOPICs per instance OR repoint chips to extant slugs.
  Gate chip list on per-instance content existence. Add CI link-check on internal hrefs.

**M5 — Guide catalog advertises ~80 guides not hosted on this site**
- GUIDEs live in woodfine-fleet-deployment (English-only, operational). Catalog presents
  them as live links; all 404.
- Fix: Add hatnote: "These guides are accessible to Woodfine operators; they are not public
  wiki articles." Do not present unresolvable guide slugs as live links.

**M6 — TOPIC/GUIDE drift: archetype articles contain GUIDE-shaped sections**
- vertical-warehouse TOPIC contains "Data collection plan", "Priority additions", "test
  results as of 2026-06-01" — transient operational content in a durable declarative TOPIC.
  ranking-system straddles TOPIC/GUIDE (algorithm steps, build-config dates).
- Fix: Split transient operational/research content into GUIDE or BRIEF artifacts; keep TOPICs
  to durable definition + spatial signature + rationale. Add `type:` to rendered frontmatter.

**M7 — Cluster counts inconsistent across articles (no snapshot dating)**
- 7,594 vs 6,493 clusters; 13 vs 17 vs 18 countries. Partly different snapshots but nothing
  tells the reader that — reads as errors.
- Fix: Add dated data-snapshot line per article ("Data as of 2026-06-11 build"). Reconcile
  home headline to one snapshot or label it a superset.

**M8 — OSM ODbL attribution absent (legal obligation)**
- Methodology rests on OSM + Wikidata. ODbL requires attribution in any published work.
  No published article names OpenStreetMap or its license.
- Fix: Add "Data Sources" section naming © OpenStreetMap contributors / ODbL; surface
  Wikidata Q-IDs in the brand-family taxonomy article. **This is a license obligation.**

**M9 — EN/ES parity lag: flagship article ES is 25% of EN length**
- co-location-ranking-system ES: ~450 words vs EN ~1,800. Real translation but not parallel.
  Per L4 and workspace bilingual rule, this is a release blocker.
- Fix: Parity sweep across all EN/ES pairs; bring lagging articles to full parallel coverage.

**M10 — Intermittent 502 flapping under concurrency (Command scope)**
- Both remaining instances return intermittent 502s on heavier routes under modest sequential
  probing. Likely blocking on git2/Tantivy reads or worker exhaustion.
- Fix: Profile binary under concurrency; confirm nginx `proxy_read_timeout` + keepalive;
  add load testing to xtask gate. **Command scope.**

---

### §7.4 — What the Audit Confirmed Working Well

- Vector 2022 chrome: complete and faithful on all three instances (tabs, TOC, hatnotes, footer)
- Typography: Inter + Source Serif 4, 17px/1.70/68ch, metric fallbacks, self-hosted WOFF2
- Mobile-first discipline: 100dvh, env(safe-area-inset-bottom), 16px+ inputs — exemplary
- Route-scoped assets: no editor bundle on article pages (L25 fully confirmed)
- Font preloads in `<head>` for latin regular faces (L23 confirmed; metric-override gap noted)
- WCAG AA colour contrast: navy #164679 on #F7F9FA = 9.1:1 (L9 confirmed)
- Zero CDN/analytics/third-party references — full sovereignty
- Git-backed article history — exceeds most enterprise wikis; serves BCSC/NI 51-102
- MCP server: 654 lines, JSON-RPC 2.0, built + tested — dark pending WIKI_ENABLE_MCP flag
- Marketing vocabulary: zero banned terms across all sampled articles
- BCSC forward-looking disclosure language correctly applied throughout

---

### §7.5 — Benchmark Gap (vs Wikipedia Vector 2022 + Stripe/MDN standard)

1. **No per-TOPIC content schema enforced at build time.** C2 tier contradiction, M6 drift,
   and M7 count inconsistencies are all symptoms. MDN quality = same page type → same sections
   in same order, validated at authoring. Define mandatory section skeletons per TOPIC type;
   validate at `cargo xtask check-content`. This is the highest-ROI governance investment.
2. **No cross-instance (sister-projects) search.** At ~700 articles across three sites this
   is the largest functional capability gap. Wikipedia sister-projects model fits: instance-scoped
   default + "search all three" broker, results grouped by instance.
3. **Typed hatnote vocabulary absent.** Hatnotes were missing on every sampled article.
   MDN/Wikipedia render notices from a closed typed vocabulary, never freehand.
4. **Citation rendering:** `[ni-51-102]`-style cues exist in source but citations.yaml is not
   resolved inline — readers see bracketed tokens that look like broken markup.

---

### §7.6 — Repair Priority Order

| Priority | ID | Scope | Complexity |
|---|---|---|---|
| 1 | C1: service restart + Restart=on-failure + uptime probe | Command | Low |
| 2 | C3: focus outline WCAG fix | Totebox (style.css) | Trivial |
| 3 | M15: article body link underline | Totebox (style.css) | Trivial |
| 4 | M14: data-instance per-instance fix | Totebox (chrome.rs / template) | Low |
| 5 | M12: /es/ chrome i18n pass | Totebox (chrome.rs strings map) | Medium |
| 6 | M4: wikilink resolver + claim comment strip | Totebox (render.rs) | Medium |
| 7 | M13: /llms.txt handler + openapi.json | Totebox (routes.rs) | Low |
| 8 | M11: remove dead POST /edit form | Totebox (server handler) | Low |
| 9 | M1: canonical slug links + sitemap abs URLs | Totebox (home_chrome / sitemap) | Medium |
| 10 | M3: category grid fix | Totebox (home_chrome query) | Medium |
| 11 | C2: tier semantics reconciliation | Project-editorial | High (content) |
| 12 | M2: onboarding chips fix | Project-editorial | Medium |
| 13 | M5: guide catalog hatnote | Project-editorial | Low |
| 14 | M8: OSM ODbL attribution | Project-editorial | Low (legal obligation) |
| 15 | M7: snapshot dating | Project-editorial | Low per article |
| 16 | M6: TOPIC/GUIDE split | Project-editorial | High (content) |
| 17 | M9: EN/ES parity | Project-editorial | High (content) |
| 18 | M10: 502 flapping under concurrency | Command (profile + load test) | Medium |
| 19 | §7.5.1: xtask content schema gate | Totebox (xtask) | High |
| 20 | §7.5.2: cross-instance search broker | Totebox (Phase 7 scope) | High |

---

## §8 — UI/UX Research Register

### §8.1 — Canonical Design Resources

- **Navigation IA:** `project-orgcharts/.agent/briefs/BRIEF-design-system-slides.md` (JW4, approved 2026-06-14).
  BRIEF states: "Same IA governs both the slide deck and wiki documentation (documentation.pointsav.com)."
  Carry-forward item in that BRIEF: "Route IA to project-knowledge for wiki navigation."
- **Component specs** (`project-design/.agent/drafts-outbound/`):
  - `DESIGN-docs-sidenav-component.draft.md` — sidenav HTML/CSS/ARIA; source commit `914cd836`
  - `DESIGN-doc-header-component.draft.md` — article header breadcrumb + `<h1>` + meta row
  - `DESIGN-RESEARCH-service-design.md` — highest-priority IA changes
  - `DESIGN-RESEARCH-ux-writing.md` — label improvements
- **Article classification (10 internal categories):** `naming-convention.md` (ratified 2026-05-09, `project-editorial/media-knowledge-documentation/.agent/rules/`)
- **Home page format invariants:** `guide-keep-the-home-page-the-gold-standard.md` (project-design)
- **DTCG 10-slot home page anatomy:** `tokens/main-page/main-page.dtcg.json` (project-design)
- **Benchmarks:** Wikipedia Vector 2022, Stripe Docs, MDN Web Docs, Linear Docs, GitLab Docs

---

### §8.2 — Category Systems (two layers)

**Layer 1 — Navigation IA: 7-category slide structure (approved 2026-06-14)**

Source: project-orgcharts `BRIEF-design-system-slides.md` (JW4). Governs home page grid and sidenav
top-level headings. Enterprise naming audited against SAP, Oracle, IBM, NetSuite, Sage and investor
vocabulary (S-1s, PE memos, EU Data Act) — passes both developer and pension-fund-manager tests.

| # | Slide category | Description |
|---|---|---|
| 01 | Developer Platform | "Who we are, how you join, and the house style for everything running on the platform." |
| 02 | Operator Workspace | "The Console OS surfaces operators work in every day." |
| 03 | System of Record | "Toteboxes, archives, and the services that keep the records." |
| 04 | Integration & Data Portability | Totebox Services (containers + ladder), Products & Services |
| 05 | Machine-Based Authorization | "Pairing as permission across the private network — authorization by device, not by role." |
| 06 | Multi-Entity Consolidation | "Aggregating fleets of archives and scaling across user tiers and composition." |
| 07 | Platform Foundation | "Where the platform runs — on-prem, leased, public cloud, hybrid — and the GIS engine beneath it." |

**Layer 2 — Article classification: 10 internal categories (ratified 2026-05-09)**

Used in article frontmatter `category:` field. Immutable order:
`architecture / substrate / patterns / systems / services / applications / governance / infrastructure / reference / design-system`

These continue as the article routing key (slug structure, landing pages, sidenav grouping within an article). The 7-category slide IA is a navigation aggregation layer on top.

**Proposed slide → internal mapping (operator to confirm — §8.4 open decision #1):**

| Slide nav tile | Internal article `category:` values |
|---|---|
| 01 Developer Platform | design-system, reference, governance |
| 02 Operator Workspace | applications, systems |
| 03 System of Record | systems, services |
| 04 Integration & Data Portability | services, patterns |
| 05 Machine-Based Authorization | infrastructure, patterns |
| 06 Multi-Entity Consolidation | architecture, systems |
| 07 Platform Foundation | substrate, infrastructure |

---

### §8.3 — Phase 1 Audit Findings (2026-06-14)

See §7 (12-agent external audit). Benchmark gap analysis in §7.5.

Engine issues (C3/M1/M3/M4/M11–M15) addressed in Sprint A/B (commits `3106b2e1`/`48bfa7e7`, Stage 6 confirmed Session 86, 2026-06-16).
Content issues (C2/M2/M5–M9) routed to project-editorial (msg-id: `command-20260614-content-repairs-app-mediakit-knowledge-a`).

Top institutional quality gaps beyond §7 findings (from §7.5 benchmark analysis):
1. No per-TOPIC content schema enforced at build time (C2/M6/M7 all symptoms)
2. No cross-instance search (Phase 7 scope)
3. Typed hatnote vocabulary absent (every sampled article missing hatnotes)
4. `[citation-id]` tokens not resolved inline — visible as broken markup

---

### §8.4 — Open Design Decisions

1. **Slide → internal category mapping** — **LOCKED 2026-06-16**: use draft §8.2 mapping as-is for documentation.pointsav.com. Sprint I may proceed.
2. **Woodfine instances (9093/9095) navigation IA** — **RESOLVED 2026-06-16**: category grid suppressed on both instances (Sprint I, `588496f2`). Content audit showed ~18 corporate topics and ~50 projects topics; not enough volume to warrant a category taxonomy. "Browse by area" section removed from projects + corporate home pages; documentation.pointsav.com unchanged.
3. **Per-brand editorial gravitas contract** — **RESOLVED 2026-06-17** (operator direction, Session 89): Lapfrog 2030 design direction decided. See §8.6 for full specification. Key decisions locked: (a) 3-font system matching `app-mediakit-shell/static/tokens.css` — Oswald/Barlow Condensed display, Nunito Sans body, Source Serif 4 reading; (b) marketing header (`left-nav → wordmark-center → right-cluster+CTA`) matching `home.woodfinegroup.com`; (c) article titles at `clamp(36px, 4.5vw, 60px)` display font; (d) remove Wikipedia editing affordances for anon readers via `[data-auth="anon"]` CSS.
4. **Design slides federation** — embedding project-orgcharts slide diagrams as article visual assets (one slide per TOPIC). Phase 7 scope; deferred.

---

### §8.5 — Implementation Sprint Log

| Sprint | Focus | Commit(s) | Status |
|---|---|---|---|
| 0 | Defect fixes: images route (Defect 6), manifest/CLAUDE.md cluster name, D1b rename | `2e0993e9` + `9fb431cb` + `c06c4ae8` + `9eda459f` | Done — Stage 6 confirmed (Session 86, 2026-06-16; sub-clone `d0abd9ad`) |
| A | WCAG 2.2 focus outline + article link underline (C3/M15) | `3106b2e1` | Done — Stage 6 confirmed (Session 86, 2026-06-16) |
| B | Sitemap absolute URLs, ES i18n, brand-instance from TOML, dead edit form (M1/M11/M12/M14) | `48bfa7e7` | Done — Stage 6 confirmed (Session 86, 2026-06-16) |
| C | 7-category home page grid + sidenav (from §8.2 Layer 1 + sidenav component spec) | `9cc1a80c` | Done — Stage 6 confirmed (Session 86, 2026-06-16) |
| D | Home page redesign: peer-band (cross-instance discovery), version in footer | `575776a8` chain → `3d90e76d` | Done — 200 tests green; Stage 6 READY sent 2026-06-16 (msg-id: command-20260617-stage-6-ready-app-mediakit-knowledge-bin); pending binary rebuild (Command) |
| E | Article chrome: audience chips + aliases frontmatter + render::Frontmatter schema | `575776a8` chain → `3d90e76d` | Done — Stage 6 READY sent 2026-06-16; pending binary rebuild (Command) |
| F | Header/footer: doc-header CSS, cross-instance peer-strip in article header, engine version in footer | `575776a8` chain → `3d90e76d` | Done — Stage 6 READY sent 2026-06-16; pending binary rebuild (Command) |
| G | Search: lede snippet in autocomplete, ?category= and ?status= filters in search_page | `575776a8` chain → `3d90e76d` | Done — Stage 6 READY sent 2026-06-16; pending binary rebuild (Command) |
| H | Phase 7 ActivityPub wiring: FederationConfig + AppState.activitypub_outbox_url + on_article_saved() wired into file watcher | `2c0ed559` | Done — 200+ tests green; Stage 6 READY (2nd) pending; note: wired into content-dir file watcher (not edit handler — no HTTP POST write path exists in git-only architecture) |
| I | Suppress category grid + category count on projects/corporate home pages | `588496f2` | Done — documentation unchanged; Stage 6 READY (3rd) sent |
| J | Configurable start-here chips via `knowledge.toml` `[[start_here]]` — fix 404 chips on Woodfine instances | `2c60282e` | Done — tests green; Stage 6 READY (4th) sent 2026-06-17; TOML entries for projects/corporate relayed to Command |
| K | Citation inline token resolution — `[citation-id]` in article body → `<a class="cite-ref">` superscript | `db08ebd0` | Done — tests green; Stage 6 READY (4th) |
| L | Typed hatnote vocabulary — closed set (main/see-also/disambig/note); .wiki-hatnote CSS fix | `36700138` | Done — tests green; Stage 6 READY (4th) |
| M | Cross-instance search web route `GET /search/all?q=` — reuses `federation_search()` from mcp.rs | `c80bf265` | Done — tests green; Stage 6 READY (4th) |
| N | xtask section schema gate — warn when topic/guide articles missing required level-2 headings | `c5658afe` | Done — tests green; Stage 6 READY (4th) |
| S | Marketing header parity — Rust: topnav HTML → `left-nav / wordmark-center / right-cluster+CTA`; wiki-bar secondary bar for search+auth; WORDMARK_SVG_WOODFINE → path-based institutional mark from shell.rs; external nav links per instance | pending (Session 89) | Planned |
| P | CSS: topnav `1fr auto 1fr` grid, marketing nav style (display font, 0.16em letter-spacing), wiki-bar below header, anon-hide Wikipedia editing UI, home editorial grid, start-here chips, featured hero base; tokens-woodfine.css: featured blue panel, wikilink color, remove solid blue topnav bar | pending | Planned |
| Q | Rust HomeStrings per-instance (for_instance matcher), audience chip human labels, instance wordmark text | pending | Planned |
| T | Lapfrog 2030 CSS — font system switch to marketing 3-font stack, article title large editorial, H2/H3 display font no-border, wider measure 80ch, blockquote editorial, marketing footer (shell_footer Rust + CSS), home hero full-bleed, mobile responsive pass | pending | Planned |

---

## §8.6 — Lapfrog 2030 Design Direction (locked 2026-06-17)

Operator direction from Session 89: the three wiki instances must match the institutional
quality of `home.woodfinegroup.com` and `home.pointsav.com` as reference standards.
Bankers arrive at the wiki directly from those sites; visual continuity is required.

### Reference sources

- `app-mediakit-shell/src/shell.rs` — `Brand::woodfine()` and `Brand::pointsav()` structures
- `app-mediakit-shell/static/shell.css` — marketing nav/footer CSS (canonical `.topnav` spec)
- `app-mediakit-shell/static/tokens.css` — marketing design token palette (36 lines)
- `app-mediakit-shell/static/woodfine-wordmark.svg` — institutional path SVG (`viewBox="0 0 144 36"`)

### Locked design decisions

**D-L1 — Marketing header structure** (replaces Wikipedia-style search-bar topnav):
```
<header.topnav>
  <nav.left>  [Disclaimer, Contact us]  </nav>
  <a.wordmark>  [path SVG]  </a>
  <div.right-cluster>
    <nav.right>  [Corporate↗, Newsroom↗]  </nav>
    <a.header-cta>  Enquire  </a>   [Woodfine only]
  </div>
</header>
<div.wiki-bar>  [search form + auth + lang]  </div>
```
Grid: `1fr auto 1fr` (matches `app-mediakit-shell/static/shell.css` `.topnav`).
Padding: `56px` horizontal (matching `shell.css`).
Search drops to `wiki-bar` secondary bar sticky at `top: var(--header-h)`.

**D-L2 — 3-font system matching marketing** (replaces Inter/Inter):
- `--font-display`: `"Oswald", "Trade Gothic LT Std", "Barlow Condensed", "Helvetica Neue", Arial, sans-serif`
- `--font-body`: `"Nunito Sans", "Avenir LT Std", "Avenir Next", "Mulish", -apple-system, "Segoe UI", Helvetica, Arial, sans-serif`
- `--font-reading`: `'Source Serif 4', Georgia, ui-serif, serif` (keep — best reading font)
No font files needed — system fallbacks (Helvetica Neue, system-ui) provide institutional feel.
All H2/H3/H4 automatically get display font because they already reference `var(--font-display)`.

**D-L3 — Article title: magazine-grade display** (was `clamp(28px, 3.6vw, 44px)`):
`clamp(36px, 4.5vw, 60px)` display font, weight 700, line-height 1.05, letter-spacing −0.02em,
`text-wrap: balance`. Target: Financial Times / Bloomberg front-page headline treatment.

**D-L4 — H2 section headings: editorial, not Wikipedia** (remove border-bottom):
`font-size: 30px`, `font-weight: 700`, `letter-spacing: 0.01em`, `margin-top: 2.8em`,
`border-bottom: none` (was `1px solid var(--rule)` — Wikipedia artifact). With Oswald/Barlow
Condensed at 700 weight this renders as Bloomberg-style section dividers.

**D-L5 — Wider reading measure** (was 68ch):
`--measure: 80ch`. Body `font-size: 18px` (was 17px), `line-height: 1.75` (was 1.70).
Blockquote: remove italic (institutional, not blog); add `background: var(--bg-subtle)`.

**D-L6 — Marketing footer** (replaces current plain footer):
`footer.footer` with `.cities` (serif, 14px) and `.footnav` (display font, 11px, 0.18em tracking,
uppercase). Per-instance content: Woodfine → `Vancouver | New York`, Contact + Disclaimer;
PointSav → `Vancouver | New York`, Disclaimer only.

**D-L7 — Wikipedia editing UI hidden for anonymous readers** (highest-impact single rule):
`[data-auth="anon"]` selector on `<html>` already set by engine. Add to `style.css`:
`.wiki-page-tabs, .edit-pencil, .doc-edit-row, .wiki-tagline, .stub-notice, .search-trigger { display: none }`
Authenticated editors still see full UI. Clean product-page reading experience for all visitors.

**D-L8 — Instance token split**:
- `projects` / `corporate` instances → Woodfine tokens (`#164679` Woodfine Blue accent, CTA)
- `documentation` instance → PointSav tokens (`#B4C5D5` steel-blue accent) — unchanged
- Marketing header is WHITE/LIGHT on ALL instances (remove the solid-blue nav bar from Sprint O)
- Woodfine Blue applied to: `header-cta` button, right-nav link color, featured hero panel background

**D-L9 — Mobile-first topnav** (≤768px stacks):
```css
.topnav { grid-template-columns: 1fr; grid-template-rows: auto auto; justify-items: center; gap: 12px; }
.topnav .wordmark { order: -1; }
.topnav .left, .topnav .right { justify-content: center; }
```
Article reading on mobile: `max-width: 100%` on `.prose`.

### Meet-in-the-middle (outbox to Command — NOT in this sprint)

`app-mediakit-shell/src/shell.rs` `Brand::pointsav()` should add `Documentation↗` link
pointing to `documentation.pointsav.com` to its `right_nav`. Small change in project-marketing
scope. Outbox message to Command to relay to project-marketing archive.

---

### §8.8 — Session 90 Log (2026-06-17, Totebox, claude-sonnet-4-6)

Session 90 — Lapfrog 2030 implementation (context-resumed from Session 89 plan). All Sprints S/P/Q/T applied.

**Rust changes (home_handlers.rs, wiki_handlers.rs, misc_handlers.rs):**
- Sprint S1: WORDMARK_SVG_WOODFINE replaced with institutional path-based SVG (viewBox 0 0 144 36, 320×80)
- Sprint S2: All three topnav locations (home, article, edit, chrome) → marketing structure (left nav | wordmark center | right-cluster + CTA)
- Sprint S3: Removed `button.search-trigger` emoji button; search moved to `.wiki-bar` secondary bar
- Sprint Q1: Per-instance HomeStrings overrides (section_featured, section_start) as local variables
- Sprint Q2: Audience chip labels humanised (customer-woodfine → "Woodfine Group", etc.)
- Sprint T5: shell_footer() replaced with marketing footer (.footer + .copyright, per-instance entity)
- misc_handlers.rs: `chrome()` also updated to marketing topnav + wiki-bar structure

**CSS changes (style.css, tokens-woodfine.css):**
- Sprint T1: Font system → Oswald/Barlow Condensed display, Nunito Sans body, Source Serif 4 reading
- Sprint P1: Topnav grid `1fr auto 1fr`; padding 56px; min-height; `.right-cluster`, `.header-cta`; wordmark 80px
- Sprint P2: `.wiki-bar` secondary sticky bar at `top: var(--header-h)`; `.wiki-bar-right`
- Sprint P3 (in tokens-woodfine.css): Removed solid blue topnav; Woodfine Blue on right links + CTA only
- Sprint P4: `[data-auth="anon"]` hides `.wiki-page-tabs`, `.edit-pencil`, `.doc-edit-row`, `.wiki-tagline`, `.stub-notice`, `.search-trigger`
- Sprint P5: `.wiki-home-editorial` 3fr/2fr grid, `.wiki-home-editorial__right` column flex
- Sprint P6: `.starthere-row`, `.starthere-chip` pill styles
- Sprint T2: `.article__title` clamp(36px,4.5vw,60px), weight 700, letter-spacing -0.02em
- Sprint T3: `.prose h2` border-bottom removed, margin-top 2.8em; `.prose h3` 22px, 2.0em
- Sprint T4: `--measure` 80ch; prose 18px, line-height 1.75; blockquote redesigned (bg-subtle, no italic)
- Sprint T5 CSS: `.footer`, `.cities`, `.sep`, `.footnav`, `.copyright` marketing footer rules
- Sprint T7: Mobile breakpoints 1200/1024/768/480px; topnav stacked on ≤768px; article full-width
- Sprint P7 (tokens-woodfine.css): Featured hero panel — `background: #164679`, white title 36px, excerpt rgba(255,255,255,0.88)
- Sprint P8 (tokens-woodfine.css): `a.wikilink { color: #164679 }`, blockquote gold, footer footnav hover, chip hover Woodfine Blue fill

**Status:** cargo check passed (6m build, 0 errors). Tests running. Pending: commit + Stage 6 outbox.

---

### §8.7 — Session 89 Log (2026-06-17, Totebox, claude-sonnet-4-6)

Session 89 design direction session. Audit of live sites (:9090/:9093/:9095) confirmed sites
still appear Wikipedia-like; no institutional banking aesthetic. Operator direction:
- Make wiki header match `home.woodfinegroup.com` exactly (same wordmark, same nav links, same CTA)
- Token split: projects/corporate → Woodfine Blue; documentation → PointSav steel
- Lapfrog 2030 now, not deferred: adopt marketing font system, editorial article reading, mobile-first
- Article reading redesign: "so good bankers won't like going back" — display font headings at magazine scale
- Mobile: all three instances must work well on phones ("most people check first on phones")
- Meet-in-the-middle: document what `app-mediakit-shell` needs (outbox to Command, NOT this sprint)

Plan file: `/home/mathew/.claude/plans/update-the-breif-and-rippling-newt.md`.
Sprints S + P + Q + T defined in plan file. BRIEF §8.4 decision #3 resolved; §8.6 added.
Sub-clone tip still `c5658afe` — Sprints S/P/Q/T pending next session after plan approval.
