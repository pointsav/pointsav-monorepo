---
schema: foundry-brief-v1
artifact: brief
status: active
topic: app-mediakit-knowledge — knowledge platform master spec (fresh build + three live sites)
archive: project-knowledge
created: 2026-06-01
updated: 2026-06-04
owner: totebox@project-knowledge
supersedes:
  - BRIEF-app-mediakit-knowledge-2030.md
  - BRIEF-knowledge-platform.md
  - BRIEF-active-work-project-knowledge-2026-05-31.md
  - BRIEF-award-winning-wiki-overhaul.md
  - BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md
  - BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md
  - BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md
  - BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md
  - BRIEF-institutional-chrome-sprint.md
  - BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md
  - BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md
  - BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md
  - BRIEF-gemini-handover-2026-05-30.md
review_basis: >
  Opus-panel verdict 2026-06-04 (Skeptic + Preservationist + Archaeologist + Synthesis judge).
  Finding: 0 decisions rejected; 9 revised with enforcement clauses; 10 new decisions added
  to close confirmed gaps. All 12 confirmed defects traced to silence or implementation drift,
  not to wrong brief decisions. New BRIEF rule: every load-bearing decision carries its own
  acceptance test and merge gate.
---

# Knowledge Platform — Master BRIEF

> **This is the single source of truth for app-mediakit-knowledge and its three live sites.**
> No other brief needs to be read. All superseded briefs are in `briefs/archive/`.
> Superseded briefs are retained as historical record; do not delete them.

---

## §1. Mission

`app-mediakit-knowledge` is a sovereign, single-binary Rust wiki engine serving three live
branded instances via Markdown files in Git. It implements the Wikipedia information model
(wikilinks, Article/Talk/History tabs, hatnotes, TOC, bilingual routing) with modern
practitioner UX additions (scroll-spy TOC, Cmd+K command palette, mobile-first layout).
No MediaWiki. No cloud runtime dependency. No third-party CDN. Every database is derived,
regenerable state; the Markdown+Git tree is the system of record. Doctrine #54 ("We Own It")
governs all architectural choices.

---

## §2. Three Live Sites

| URL | Service unit | Port | Content repo | Brand | Blueprints | DNS status |
|---|---|---|---|---|---|---|
| documentation.pointsav.com | local-knowledge-documentation.service | 9090 | media-knowledge-documentation | PointSav | TOPIC + GUIDE | Confirmed: documentation.pointsav.com (Q3, 2026-06-04) |
| projects.woodfinegroup.com | local-knowledge-projects.service | 9093 | media-knowledge-projects | Woodfine | TOPIC only | Live |
| corporate.woodfinegroup.com | local-knowledge-corporate.service | 9095 | media-knowledge-corporate | Woodfine | TOPIC only | Live |

**Blueprint rule (operator-confirmed 2026-06-04):**
- Only the documentation instance serves GUIDEs. Projects and corporate serve TOPIC only.
- No COMMS blueprint on any instance.
- Cross-instance isolation is structural: never a global `[[slug]]` resolver. A `[[slug]]`
  resolves only within the mount set of the instance that rendered it.

---

## §3. Locked Decisions (L1–L29)

> **New BRIEF rule (from Opus-panel verdict):** Every load-bearing decision below carries
> its own acceptance test or merge gate. A decision that can ship with its enforcement
> mechanism unbuilt is an aspiration, not a lock.
>
> **[CARRY]** = verbatim from prior master; confirmed invariant by Opus panel.
> **[REVISE]** = prior decision with enforcement clause added.
> **[NEW]** = closes a gap the prior brief was silent on.

---

### Constitutional / ADR Hard Rules

**L12 [CARRY]** SYS-ADR-07: No structured data passes through AI. Constitutional hard rule.

**L13 [CARRY]** SYS-ADR-10: F12 mandatory. Every commit to a canonical content tree is an
explicit human operator action. Dead-code removal of the collab module does not weaken this gate.

**L14 [CARRY]** SYS-ADR-19: No automated AI publishing to verified ledgers without an
explicit F12 commit action.

---

### Legal / Governance Invariants

**L2 [CARRY]** Git-native flat-file content store. Markdown files in a Git tree are the
system of record. All databases are derived state, deletable and rebuildable. 50-year-readable.

**L5 [REVISE]** Self-hosted WOFF2 fonts, no CDN (GDPR Art. 44 — non-negotiable legal
invariant). Self-hosting carries its own loading contract: each above-the-fold typeface MUST
emit `<link rel="preload" as="font" type="font/woff2" crossorigin>` in `<head>` alongside a
metric-override fallback. See L23.

**L7 [CARRY]** Canonical footer trademark text is byte-for-byte locked (year field only
updates). See §10 for exact text. Legal invariant.

**L15 [CARRY]** Engine licence: Apache 2.0. Wiki content licence: CC BY 4.0.

**L16 [CARRY]** Commit identity: `jwoodfine` or `pwoodfine` only, via `commit-as-next.sh`.

---

### Deployment and Runtime

**L1 [REVISE]** Single Rust binary per instance (`cargo build --release`). Deployment unit
is one binary (Doctrine #54). **Single binary ≠ single source file ≠ single JS bundle.**
Internals MUST be modular per L20; client assets MUST be route-scoped per L25.

**L10 [CARRY]** MCP JSON-RPC 2.0 native endpoint, optional behind `--enable-mcp`.

**L11 [CARRY]** Claim-layer HTML comment markup format. In production; do not alter without
a Decision-Log entry.

---

### Content and Data Model

**L3 [REVISE]** `dtcg-bundle.json` is the single source of truth for all CSS custom
properties. Per-brand outputs (`tokens.css`, `tokens-woodfine.css`) are GENERATED by
`dtcg-to-css.py`. No hand-authored token or theme CSS may coexist with the generated bundle.
`theme-woodfine.css` is folded into the vault and deleted in Phase 1. See L21.

**L4 [REVISE]** Bilingual EN+ES via `.es.md` sibling on a single canonical slug. **Bilingual
scope includes chrome.** All reader-visible strings come from a `strings(locale)` map; `/es/`
MUST render the `.es` sibling title. Acceptance: `/es/` HTML contains zero hardcoded-English
chrome strings. See L22.

**L19 [REVISE]** Federation via declarative `Vec<Mount>` + content-type blueprints.
**Completion-gated, not "locked-done":** `AppState` carries `mounts: Vec<Mount>`; the
hardcoded `content_dir`, `guide_dir`, `guide_dir_2` fields are DELETED in the same commit
that wires mounts. `blueprints.rs` drives chrome dispatch. `inject_wiki_prefixes` resolves
across the full mount set (instance-scoped, not global). **Merge gate:** No Phase 2+ visual
work reaches canonical while any instance still uses the old hardcoded path.

---

### UX and Information Architecture

**L6 [REVISE]** Wikipedia Vector 2022 information-model conventions (wikilinks,
Article/Talk/History, hatnotes). Visual language follows Stripe/Linear craft. **Chrome
rendering lives in one parameterised `chrome.rs` emitter** — never multiple inline `*_chrome`
copies in the same handler file.

**L8 [CARRY]** Inter (UI + headings) + Source Serif 4 (reading body) + system monospace.
WOFF2 only. Supersedes the retired Oswald/Nunito/Roboto Slab stack (2026-06-01 decision log).

**L9 [CARRY]** Base palette: `--navy: #164679`; `--bg: #F7F9FA`; `--link: var(--navy)`.
WCAG AA verified. Protected by L3 token vault.

---

### Mobile and Performance

**L17 [REVISE]** Mobile-first: base stylesheet = phone; desktop via `min-width` only.
**Per-release enforcement checklist:**
- `env(safe-area-inset-bottom)` APPLIED (not merely defined) on all fixed/sticky chrome and body padding — see L24
- `viewport-fit=cover` in viewport meta
- All form inputs `font-size: ≥16px`
- `dvh`/`svh` used; `100vh` prohibited
- Phone smoke test required before every promote

---

### Content Quality Gate

**L18 [REVISE — SPLIT]** Build-time wikilink resolver is a **hard promote gate**. Any
unresolved `[[slug]]` across the mount set BLOCKS promote. The gate is a precondition of
the "zero dead links" claim — it MUST exist and pass before `wikilink-missing` render is
removed. Sequence: (1) build gate, (2) verify no false positives, (3) remove red-link
emission. See L29.

---

### New Decisions — Closing Confirmed Gaps

**L20 [NEW]** Source-file size discipline: no `.rs` file exceeds ~1,500 lines / 60 KB.
`server.rs` decomposes into modules along concern boundaries before any feature code lands.
Acceptance: `find src -name '*.rs' | xargs wc -l` — no file above 1,500 lines.

**L21 [NEW]** Exactly three CSS artifacts: `style.css` (shared), `tokens.css` (PointSav),
`tokens-woodfine.css` (Woodfine). Adding a fourth `.css` file requires a Decision-Log entry.
`theme-woodfine.css` is deleted in Phase 1.

**L22 [NEW]** Chrome strings are locale-keyed via `strings(locale)`. Acceptance test:
`cargo test es_homepage_chrome_is_spanish` must pass before any `/es/` route ships.

**L23 [NEW]** Font preload is mandatory in the base chrome `<head>`. Two
`<link rel="preload" as="font" crossorigin>` tags — Inter latin-regular and Source Serif 4
latin-regular — emitted unconditionally before stylesheet links. Acceptance: every rendered
`<head>` contains exactly two font preload links.

**L24 [NEW]** Safe-area insets APPLIED not merely defined. Every fixed/sticky bottom chrome
uses `calc(N + env(safe-area-inset-bottom))`. Bare `padding-bottom: 56px` on bottom chrome
is a lint error. Acceptance: phone smoke test confirms no Home Indicator overlap.

**L25 [NEW]** Route-gated client bundles. `editor.js` (CodeMirror 6 + SAA) loads only on
`/edit/*`. Article, home, category, and search pages load only `wiki.js`. Acceptance: article
page HTML contains zero references to `editor.js`.

**L26 [NEW]** Dead-code removal is a tracked deliverable. When a feature is superseded:
its module, vendor bundle entries, routes, and tests are deleted in the same commit and logged
in `cleanup-log.md`. Superseding briefs carry forward predecessor removal actions. Collab
module dead code removed in Phase 1.

> **Q1 resolution (2026-06-04):** In-browser editing is NOT required. The entire
> auth/edit/CodeMirror stack — `auth.rs`, `users.rs`, `pending.rs`, edit/admin/auth routes,
> `static/editor.js`, and both CodeMirror vendor bundles (`cm-collab.bundle.js`,
> `cm-saa.bundle.js`) — is removed under the git-only content model. L25 (route-gated editor
> bundle) and L26 (dead-code removal as tracked deliverable) govern this removal.

**L27 [NEW]** List micro-layouts carry explicit separators in markup. Recently-changed title
and date are in separate child elements, not a concatenated string. Acceptance: recently-changed
HTML contains no `"{title}{date}"` text node concatenation.

**L28 [NEW]** DNS provisioning is a named deliverable owned by the Command Session. The §2
table carries a DNS-status field. The conflict between `documentation.pointsav.com` and
`documentation.woodfinegroup.com` must be reconciled in a NEXT.md commit before DNS cutover.

**L29 [NEW]** No article may reference an uncommitted slug. Same build-time resolver as L18:
a `[[guide-slug]]` not present in any mount BLOCKS promote. Content-sequencing rule: guides
committed before articles that reference them. This is one resolver serving both L18 and L29.

---

## §4. Fresh-Build Decision (Hybrid A+B)

Source: competing-agent analysis 2026-06-04
(`DESIGN-knowledge-platform-fresh-slate-analysis.draft.md`).

**Hybrid A+B chosen.** From Architect A: full module decomposition (L20), mounts/blueprints
wired from day 1 (L19), CSS consolidation (L21), related-articles sidebar card (blueprints
`relates_to`), article status badge (`quality:` → chrome notice), three-way night-mode.
From Architect B: scroll-spy right-rail TOC (L26 borrow), Cmd+K command palette (L27 borrow).
Wikipedia tabs retained on all three instances. Architect C (static pipeline) deferred pending
Q1 answer on in-browser editing.

---

## §5. Module Architecture (target from first commit)

The full `src/` directory is replaced. No file from the prior `server.rs` monolith carries
forward as-is. This tree is the day-1 target before any feature code lands.

```
src/
├── main.rs              (~50 lines; CLI entry point + tokio bootstrap)
├── config.rs            (CLI flags, env vars, knowledge.toml parse)
├── state.rs             (AppState; mounts: Vec<Mount>; blueprints: Vec<Blueprint>)
├── mounts.rs            (Mount struct; directory walk; content index — WIRED day 1)
├── blueprints.rs        (Blueprint; YAML parse; content-type dispatch — WIRED day 1)
├── walker.rs            (ContentWalker; frontmatter parse; bilingual pair detection)
├── render.rs            (Markdown→HTML; comrak + wikilinks; blueprint-aware chrome call)
├── error.rs             (WikiError; IntoResponse)
├── jsonld.rs            (Schema.org JSON-LD from frontmatter)
├── glossary.rs          (auto-linker)
├── citations.rs         (citation registry; hover card data)
├── check.rs             (dead-link gate + frontmatter validator; `check --strict` subcommand — canonical, uses render::page_exists)
├── chrome/
│   ├── mod.rs           (base chrome; font preload L23; strings(locale) L22; dark-mode)
│   ├── article.rs       (tabs; TOC; hatnote; infobox; status badge; related-articles)
│   ├── home.rs          (category grid; leapfrog facts; invariant panels)
│   ├── palette.rs       (Cmd+K; <dialog>; keyboard shortcut)
│   └── mobile.rs        (bottom bar; safe-area L24; thumb-zone nav)
├── routes/
│   ├── mod.rs           (router() assembly — all routes visible in one place)
│   ├── wiki.rs          (GET /wiki/{slug}; GET /es/wiki/{slug})
│   ├── home.rs          (GET /; GET /es/)
│   ├── category.rs      (GET /category/{name})
│   ├── search.rs        (GET /api/search; GET /api/complete)
│   ├── feeds.rs         (GET /feed.atom; /feed.json; /sitemap.xml; /robots.txt; /llms.txt)
│   ├── edit.rs          (GET /edit/{slug}; POST /api/edit/{slug})
│   ├── admin.rs         (GET /admin/pending; POST /admin/pending/{id}/{action})
│   ├── git.rs           (GET /git/{slug}; GET /git-server/...)
│   ├── auth.rs          (GET+POST /auth/login; GET /auth/logout)
│   └── mcp.rs           (POST /mcp; behind --enable-mcp)
├── search.rs            (Tantivy 0.24 BM25; index rebuild; notify watcher)
├── links.rs             (redb 4.1 wikilink graph; blake3 hashing)
├── git.rs               (git2 write path; commit-on-edit)
├── history.rs           (gix read path; blame; diff)
├── auth.rs              (cookie sessions; auth extractors)
├── users.rs             (rusqlite; argon2id)
├── pending.rs           (edit review queue)
├── feeds.rs             (Atom RFC 4287; JSON Feed v1; sitemap)
└── mcp.rs               (MCP JSON-RPC 2.0; no vendor SDK)
```

---

## §6. CSS Architecture (three files exactly — L21)

```
static/
├── tokens.css           (PointSav DTCG output; generated from dtcg-bundle.json)
├── tokens-woodfine.css  (Woodfine DTCG output; generated; theme-woodfine.css deleted)
└── style.css            (shared; mobile-first; 9 sections:)
    1. Custom properties (token var references; no hardcoded hex)
    2. Reset and base
    3. Layout grid and breakpoints
    4. Chrome (header, tabs, TOC, palette, bottom bar)
    5. Article surface (body, headings, infobox, hatnote, status badge)
    6. Home surface (category grid, leapfrog, invariant panels)
    7. Edit surface
    8. Dark mode (prefers-color-scheme + [data-theme])
    9. Print
```

---

## §7. JavaScript Architecture (two files exactly — L25)

```
static/
├── wiki.js    (scroll-spy TOC; Cmd+K palette; citation hover; theme toggle; TOC pin)
└── editor.js  (CodeMirror 6 + SAA; loaded ONLY on /edit/* routes)
```

Predecessor files deleted: `toc-persistence.js`, `saa-init.js`.

---

## §8. Three-Instance Differentiation

| Dimension | documentation.pointsav.com | projects.woodfinegroup.com | corporate.woodfinegroup.com |
|---|---|---|---|
| Token file | tokens.css | tokens-woodfine.css | tokens-woodfine.css |
| Blueprints | **TOPIC + GUIDE** | **TOPIC only** | **TOPIC only** |
| COMMS | No | No | No |
| Guide mount | YES | None | None |
| Site title | PointSav Documentation | Woodfine Projects | Woodfine Corporate |
| Audience | Technical practitioners | Customer / GIS | Institutional |
| Leapfrog YAML | Platform facts | GIS / Regional Markets | Corporate milestones |
| Tab model | Article/Talk/Edit/History | Article/Talk/Edit/History | Article/Talk/Edit/History |

---

## §9. Content Repo Status (2026-06-04 audit)

**media-knowledge-documentation (~514 files):** ~98% frontmatter complete; bilingual coverage
complete. Blocking: (1) missing `professional-centres` article (red link in production),
(2) stale `featured-topic.yaml` path, (3) 62 guide-slug 404s. Conditionally publication-ready.

**media-knowledge-projects (~102 files):** 100% frontmatter complete; bilingual complete.
Blocking: (1) `reference-invariants.yaml` slug prefix mismatch, (2) `[[about]]` dead link
in `contact.md`. Conditionally publication-ready.

**media-knowledge-corporate (~51 files):** 34/38 frontmatter complete. Blocking: (1) two stub
articles linked from home page lede (`topic-perpetual-equity-model`,
`topic-investment-units`), (2) same slug prefix mismatch, (3) 4 articles missing `last_edited:`.
**NOT publication-ready.**

---

## §10. Canonical Footer Text (L7 — byte-for-byte locked)

```
© 2026 Woodfine Capital Projects Inc. All rights reserved.
Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital
Projects Inc. used in Canada, the United States, Latin America, and Europe. All other
trademarks are the property of their respective owners.
```

---

## §11. MVCC Cleanup (existing live sites — execute before or during fresh build)

Ten commits, each via `commit-as-next.sh`, ordered by user-visible impact:

1. Fix `/es/` homepage chrome language (L22 enforcement on old code)
2. Add font preload tags to base chrome `<head>` (L23)
3. Fix mobile safe-area: `calc(56px + env(safe-area-inset-bottom))` (L24)
4. Fix stale `featured-topic.yaml` path in documentation repo
5. Fix `reference-invariants.yaml` slug prefix in projects + corporate repos
6. Fix recently-changed list title/date concatenation (L27)
7. Create missing `professional-centres` stub article in documentation repo
8. Expand `topic-perpetual-equity-model` + `topic-investment-units` to `status: active`
9. Fix `[[about]]` dead link in projects `contact.md`
10. Move CodeMirror to editor-only load (L25 enforcement on old code)

---

## §12. Open Questions (operator decisions before Phase 1 scoping)

| # | Question | Gates | Resolution |
|---|---|---|---|
| Q1 | Is in-browser editing (CodeMirror → git commit) required? | Phase 6 scope | **RESOLVED 2026-06-04** — git-only; auth + edit + CodeMirror removed |
| Q2 | Corporate instance: Wikipedia tabs or Architect B left-rail? | Chrome model for corporate | **RESOLVED 2026-06-04** — Wikipedia tabs on all 3 instances |
| Q3 | `documentation.pointsav.com` or `documentation.woodfinegroup.com`? | L28 DNS cutover | **RESOLVED 2026-06-04** — documentation.pointsav.com |
| Q4 | Phase 6 GitHub renames + Doctrine amendment: current roadmap? | Content-repo naming | **RESOLVED 2026-06-04** — ran on auto |

---

## §13. Borrow List (from 2026-06-04 web benchmarking)

| Pattern | Source | Priority |
|---|---|---|
| Scroll-spy right-rail TOC | Stripe + Vercel docs | P0 — L26 |
| Cmd+K command palette | Tailwind CSS docs | P0 — L27 |
| Font preload in `<head>` | rustdoc | P0 — L23 |
| Mobile safe-area-inset | All audited sites | P0 — L24 |
| Build-time wikilink resolver + frontmatter validation | Hugo + MkDocs | P0 — L18/L29 |
| Sticky compact header on scroll | Wikipedia Vector | P1 |
| Pinnable TOC via localStorage | Wikipedia Vector | P1 |
| Article status badge (`quality:` → chrome notice) | ArchWiki | P1 |
| Related-articles sidebar card (blueprints `relates_to`) | ArchWiki | P1 |
| Three-way night-mode toggle (light/dark/system) | ArchWiki | P2 |

---

## §14. Implementation Phases

> Phase 0 is a hard merge gate: no Phase 2+ work reaches canonical while any Phase 0
> item is incomplete.

**Phase 0 — MVCC Sprint** (existing sites; parallel with Phase 1 setup)
Execute §11 items 1–10. No rebuild. ~3–4 days. Gate: all 10 committed, `cargo test` still passes.

**Phase 1 — Foundation** (new codebase)
New `src/` per §5; `config.rs`; `state.rs` with `Vec<Mount>` + `Vec<Blueprint>`; `mounts.rs`;
`blueprints.rs`; `walker.rs`; `error.rs`; `check.rs` (`check --strict` subcommand); `collab.rs`
deleted (L26). Gate: `cargo check` passes; xtask runs; no file above 1,500 lines.

**Phase 2 — Render Pipeline**
`render.rs`; `jsonld.rs`; `citations.rs`; `glossary.rs`.
Gate: TOPIC article renders to valid HTML; JSON-LD validates.

**Phase 3 — Chrome**
`chrome/` modules; `static/style.css` (9-section); `static/tokens.css` + `tokens-woodfine.css`
(generated); `theme-woodfine.css` deleted; `static/wiki.js` (merged from three predecessor
files); `toc-persistence.js` + `saa-init.js` deleted.
Gate: all three homepages render; Cmd+K opens; scroll-spy fires; `/es/` chrome fully Spanish
(L22 test); font preloads in `<head>` (L23 test).

**Phase 4 — Routes + Search**
All `routes/` modules; `search.rs`; `feeds.rs`.
Gate: all GET routes 200; search returns results; Cmd+K queries live; feeds validate.

**Phase 5 — Git + Link Graph + Dead-Link Gate**
`git.rs`; `history.rs`; `links.rs`; dead-link gate wired to all mount sets.
Gate: git history renders; What-links-here works; `check --strict` (canonical
subcommand, not the buggy `cargo xtask`) catches an intentional dead link and exits non-zero.

**Phase 6 — Auth + Edit** (conditional on Q1)
`auth.rs`; `users.rs`; `pending.rs`; edit routes; `static/editor.js` (editor-only load L25).
Gate: login/logout works; edit → commit flow completes; article HTML contains zero `editor.js`
references (L25 test).

**Phase 7 — MCP + OpenAPI**
`mcp.rs`; `openapi.yaml` regenerated.
Gate: MCP returns valid JSON-RPC; openapi.yaml validates.

**Phase 8 — Per-Brand Theming + DTCG Back-port**
12-token brand contract confirmed; token vault updated; DTCG back-port via DESIGN-TOKEN-CHANGE
artifact (requires `master_cosign:`); WCAG AA confirmed.

**Phase 9 — Deploy**
`cargo build --release`; binary installed; `knowledge.toml` per instance; systemd units
updated; DNS reconciliation (L28); dead-link gate passes on all three live mount sets.
Gate: all three instances HTTP 200; search works; font preloads in `<head>`; Cmd+K opens;
safe-area confirmed on phone.

---

## §15. Deployment Configuration

### knowledge.toml structure

```toml
# /etc/local-knowledge/documentation.toml
[site]
title     = "PointSav Documentation"
brand     = "pointsav"
bind      = "127.0.0.1:9090"
state_dir = "/var/lib/local-knowledge/state"

[[mount]]
path          = "/srv/foundry/clones/project-knowledge/media-knowledge-documentation"
role          = "primary"
blueprint_set = ["TOPIC", "GUIDE"]

[citations]
path = "/srv/foundry/citations.yaml"
```

```toml
# /etc/local-knowledge/projects.toml
[site]
title     = "Woodfine Projects"
brand     = "woodfine"
bind      = "127.0.0.1:9093"
state_dir = "/var/lib/local-knowledge/state-projects"

[[mount]]
path          = "/srv/foundry/clones/project-knowledge/media-knowledge-projects"
role          = "primary"
blueprint_set = ["TOPIC"]

[citations]
path = "/srv/foundry/citations.yaml"
```

```toml
# /etc/local-knowledge/corporate.toml
[site]
title     = "Woodfine Corporate"
brand     = "woodfine"
bind      = "127.0.0.1:9095"
state_dir = "/var/lib/local-knowledge/state-corporate"

[[mount]]
path          = "/srv/foundry/clones/project-knowledge/media-knowledge-corporate"
role          = "primary"
blueprint_set = ["TOPIC"]

[citations]
path = "/srv/foundry/citations.yaml"
```

### Systemd unit change (per instance)
```ini
# Add:
Environment="WIKI_KNOWLEDGE_TOML=/etc/local-knowledge/documentation.toml"
# Remove: WIKI_CONTENT_DIR, WIKI_GUIDE_DIR, WIKI_GUIDE_DIR_2
```

---

## §16. Archive References

| Artifact | Location | Purpose |
|---|---|---|
| Fresh-slate UX analysis | `.agent/drafts-outbound/DESIGN-knowledge-platform-fresh-slate-analysis.draft.md` | Source of §4; route to project-design |
| Opus brief review verdict | `.agent/drafts-outbound/BRIEF-REVIEW-old-brief-verdict.md` | Source of §3 REVISE/NEW decisions |
| Superseded briefs | `.agent/briefs/archive/` | Historical record; do not delete |
| ARCHITECTURE.md | `pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md` | Update after Phase 1 |
| dtcg-bundle.json | `pointsav-monorepo/app-mediakit-knowledge/scripts/dtcg-bundle.json` | Source of truth for L3 token generation |
| Staged DESIGN components | `.agent/drafts-outbound/DESIGN-doc-header-component.draft.md` + `DESIGN-docs-sidenav-component.draft.md` | Route to project-design for intake |

---

## §17. Navigation and Discovery Architecture

### 17.1 The Core Problem Per Site

**documentation.pointsav.com** — A practitioner arriving at this site almost always has a domain but not a destination. They know they are working on substrate architecture or network infrastructure, but they do not know whether the article they need exists or what it is called. The homepage must offer a credible entry into 514 articles without overwhelming, which means category-first navigation backed by a search bar that is immediately visible. "Great" for this audience means: a practitioner can reach the right category landing page in one click, find the article they approximately want in one more click, and from within that article navigate forward and backward through the relevant series without returning to the homepage.

**projects.woodfinegroup.com** — Customers land here without knowing what the platform contains. They may have been sent a link by a project manager, or they may have discovered the site themselves. They do not know that "Regional Markets" and "Co-location Archetypes" are distinct bodies of content, and they cannot navigate a 102-article taxonomy they have never seen. The homepage must perform active editorial curation: tell the reader what the three or four most important bodies of knowledge are, give them a one-sentence reason to enter each one, and surface the single best starting article in each body. Discovery is the primary function of this homepage; it is not a table of contents.

**corporate.woodfinegroup.com** — Institutional readers (investors, lawyers, due diligence teams) arrive with a specific question or a mandate to understand the company. A first-time visitor doing due diligence needs to know immediately which articles are authoritative disclosures and which are informational background. The homepage must present the content in two registers: "if you are here for due diligence, start here" and "if you are exploring the company, browse by subject." Quality signalling — which articles are complete, which are still being developed — is more important here than on either of the other two sites.

---

### 17.2 Homepage Architecture Per Instance

| Instance | Primary entry point for "I don't know" visitors | Category landing page role | Featured/curated mechanism | "Start here" pattern |
|---|---|---|---|---|
| documentation.pointsav.com | Category grid (9 tiles, each with name + article count + one-line scope description) | Primary navigation surface — shows article cards with lede, sorted by `featured:` then recency | Featured article rotation from `featured: true` pool; renders as header card with thumbnail-equivalent status badge + first 200 chars of body | None — practitioners self-navigate via category |
| projects.woodfinegroup.com | Three thematic clusters (Editorial story groups: "Location Intelligence," "Regional Markets," "Co-location Archetypes") each with 3–4 article cards | Secondary surface — readers arrive via cluster, not via direct category URL | Curated "Start here" card per cluster, editor-selected via `featured: true` within that category | Explicit "Start here" card in each cluster, linking to the foundational methodology article |
| corporate.woodfinegroup.com | Two-column layout: left = "Due Diligence Path" (ordered sequence of 5 articles); right = "Browse by subject" (category links with counts) | Thin — institutional readers use the due-diligence path or search; category pages are secondary | No rotation; one static "Featured Disclosure" card, editor-selected, updated at each governance milestone | Explicit "If this is your first visit" sequence link at top of homepage |

**documentation.pointsav.com:** The category grid is the homepage. Each tile names the category, shows the article count, and carries a one-sentence scope description authored in `content/category-config.yaml`. Practitioners scan the nine tiles, select the right domain, and land on the category page. The featured article rotation is a secondary strip below the grid — it surfaces content the editorial team considers exemplary, not necessarily the most recent.

**projects.woodfinegroup.com:** Thematic clusters replace the category grid entirely. Each cluster is a card group with an editorial frame headline ("Understanding Where Commercial Anchors Cluster") and three to four article cards below it, each showing title and a one-sentence summary from the `summary:` frontmatter field. A customer who does not know what "VWH" means can read the cluster headline and decide whether to enter without needing to understand the taxonomy first.

**corporate.woodfinegroup.com:** The due-diligence path is a numbered sequence of five articles rendered as an ordered list with sequence position numbers, article titles, and the `status:` badge for each. This tells an institutional reader exactly what to read in what order, and signals which articles are complete authoritative disclosures (status: complete → "Evergreen" badge) versus works in progress (status: pre-build → "In Development" badge).

---

### 17.3 Article-Level Discovery

Every article page on all three sites renders the following mechanisms, implemented once in the shared article template:

**Backlinks portlet** — Query `get_backlinks(slug)` from the redb graph at render time. Render as a sidebar section "Referenced by N articles" with linked titles. Hide conditionally if the backlink count is zero. This is the single highest-impact, lowest-effort change: the data already exists in redb; only the render step is missing. A practitioner reading about `genesis-protocol` and seeing that six other articles reference it immediately understands that this is a hub concept.

**`relates_to` field rendered as "See Also"** — The `relates_to` field in frontmatter is already populated but not rendered. Render it as a "See Also" block at article bottom with article titles as links. This is the manual-curation layer complementing the automatic backlinks portlet.

**Category membership** — Render the article's `category:` value as a clickable chip in the article header, linking to the category landing page. This is one line of maud HTML and gives every reader a one-click escape to the broader category.

**Status badge** — Render the `status:` field as a visible badge in the article header. Map: `stub` → "Seedling," `pre-build` → "In Development," `active` → "Active," `complete` → "Evergreen." This calibrates reader expectations before they invest time, and is especially critical on corporate.woodfinegroup.com.

**Next/previous within category** — At article bottom, render "Previous in [Category]" and "Next in [Category]" links based on the category-ordered article list. Order within a category is defined by `position:` in frontmatter (see §17.9); articles without `position:` sort alphabetically. This makes series navigation explicit without requiring sequence infrastructure.

**Inline Markdown "## See Also" section** — Content convention, not an engine feature (see §17.10).

---

### 17.4 Category Landing Pages

The existing `GET /category/{name}` route should render the following for all three sites:

**Article cards with lede, not a bare list.** Each card shows: title (linked), status badge, category chip, and the first sentence of the article body auto-extracted at build time (fallback: the `summary:` field if present). No thumbnails — this is a text-first corpus. Cards are sorted: `featured: true` articles first, then by `updated:` descending.

**Sub-category grouping on documentation.pointsav.com only.** The `content/category-config.yaml` file defines named cluster groups within each category (e.g., within "architecture": Protocols, Substrate, Network, BIM). The category page renders these as labeled sections with the relevant articles beneath each. Articles not assigned to a sub-category cluster appear in an "Other" section. This config file is editorial, not per-article frontmatter.

**Count badge.** Render "N articles" in the category page header. This is a one-line addition that communicates the depth of a category to a browsing reader.

**Featured article within category.** The `featured: true` article with the highest recency in that category renders as a header card above the cluster groups, with the auto-extracted lede displayed in full. One featured card per category page, not a rotation.

**"New this month" signal.** Below the featured card, a "Recently updated" strip shows the three articles with the most recent `updated:` date, each as a compact one-line entry (title + relative date "updated 3 days ago"). This gives returning visitors an immediate signal that the category is active without scrolling the full list.

---

### 17.5 Hub Articles

**Yes, support hub articles, but only for projects.woodfinegroup.com and documentation.pointsav.com.** The corpus sizes justify this: 102 articles (projects) and 514 articles (documentation) both have enough density that readers benefit from a curated map of a topic area. Corporate.woodfinegroup.com at 51 articles does not need hub articles — the category landing pages are sufficient at that scale.

Signal with `hub: true` in frontmatter. This is preferable to a `type:` field because it does not replace the article's existing blueprint classification — a hub is still a TOPIC.

The engine renders hub articles differently in two ways: (1) the article body may contain a `[[wikilink]]` list without surrounding prose and the engine renders these as article cards (title + lede) rather than inline links; (2) the backlinks portlet is suppressed on hub articles because their purpose is to route outward, not to surface inbound links. Category landing pages pin `hub: true` articles at the top of their section, above `featured: true` articles.

On documentation.pointsav.com, the architecture and substrate categories each warrant one hub article. On projects.woodfinegroup.com, a "Location Intelligence Overview" hub article is the natural "start here" for new customers. Existing articles can be promoted to hub status with a single frontmatter field addition — no content rewrite required.

---

### 17.6 Search as Discovery

The `/api/complete` autocomplete endpoint already exists. The search results page (currently missing) should render article cards identical to the category landing page format: title, status badge, category chip, auto-extracted lede. This means a reader who types a partial query and browses results gets the same information density as a reader browsing a category — they can assess relevance without clicking through.

**Search results page layout:** Three sections in order — (1) Exact title matches (if any), rendered as a single highlighted card; (2) Full-text BM25 results, rendered as article cards; (3) "Browse by category" links if fewer than three results are returned, giving the reader an escape route.

**Zero-result empty state:** Do not show "no results found" alone. Show: the query the user typed (to confirm it was received correctly), two or three category links ("You might find what you're looking for in: Architecture, Substrate, Systems"), and a prompt to try a shorter query. On projects.woodfinegroup.com, the zero-result state should surface the three thematic cluster entry points, since unfamiliar readers are most likely to search for terms that do not yet match article titles.

**Topic suggestions:** Not recommended at this scale. Our corpus is too small and too domain-specific for useful "related searches" — the category navigation serves that function.

---

### 17.7 What NOT to Build

**Algorithmic "readers also viewed" recommendations.** Our corpus is too small for collaborative filtering and we have no user analytics. The result would be low-quality or repetitive suggestions that erode trust rather than build it.

**Infinite scroll on category pages.** Our largest category is ~100 articles. Pagination at 25 articles per page is sufficient and keeps the page weight constant. Infinite scroll adds JS complexity for a problem we do not have.

**Social signals (view counts, likes, upvotes).** We have no analytics infrastructure and institutional readers on corporate.woodfinegroup.com would find engagement metrics inappropriate for governance disclosures. Quality is signalled by `status:` and `featured:`, both editorial decisions, not crowd signals.

**Personalisation and reading history.** Requires login infrastructure, user data storage, and session management — none of which align with the single-binary, flat-file architecture. The LessWrong progress-tracking pattern is compelling but requires a session cookie at minimum; exclude from this build.

**Date-anchored "On This Day" feature.** This requires a population of articles with historical event dates or corporate milestone dates. Our current corpus does not have this density, and forcing it would produce a mechanism that fires blank most days. Revisit if the corporate.woodfinegroup.com corpus grows to include a systematic timeline of corporate milestones.

**Quiz or game-based discovery.** Appropriate for Khan Academy and Britannica's general-audience model. Wrong register for technical practitioners, project customers, and institutional investors.

**Full LessWrong-style sequence system with progress tracking.** The sequence navigation pattern (prev/next within category, ordered by `position:`) captures the core benefit. The progress counter requires session state; omit it. Readers who want to track progress will bookmark.

---

### 17.8 New Routes Required

| Route | Purpose | Requirement |
|---|---|---|
| `GET /hub/{slug}` | Render hub articles with card-expanded wikilinks | Nice-to-have; hub articles render adequately on the standard article route if the engine detects `hub: true` |
| `GET /start` | Alias for the "start here" entry point on projects.woodfinegroup.com | Nice-to-have; the homepage serves this function |
| `GET /search?q={query}` | Full search results page with article cards | **Hard requirement** — currently missing |
| `GET /new` | "Recently updated" feed across all categories | Nice-to-have; useful for returning visitors on documentation.pointsav.com |
| `GET /category/{name}?sort=recent` | Category page sorted by update date rather than featured-first | Nice-to-have; a sort query param on the existing route |

The search results page is the only hard requirement. All others are enhancements to existing routes or optional aliases.

---

### 17.9 Frontmatter Schema Additions Required

| Field | Type | Purpose | Sites | Gates |
|---|---|---|---|---|
| `summary` | String, ≤160 chars | One-sentence article description for category cards and cluster cards | projects.woodfinegroup.com (required), others (optional) | Article card lede on homepage clusters |
| `hub` | Boolean | Marks article as a map-of-content hub; changes render behaviour | documentation.pointsav.com, projects.woodfinegroup.com | Hub article render path; category page pinning |
| `position` | Integer | Order within category for next/previous navigation and category page sort | documentation.pointsav.com (priority), others optional | Next/previous article links; category page order |
| `sequence` | Object `{name: string, position: int}` | Assigns article to a named reading sequence | projects.woodfinegroup.com, corporate.woodfinegroup.com | Sequence header on article pages (future phase) |

`summary` is the highest-priority addition for projects.woodfinegroup.com — without it, cluster cards show auto-extracted first sentences that may not be reader-facing prose. `hub` and `position` are low-touch additions with high navigation payoff. `sequence` is scoped to a later phase.

---

### 17.10 Content Conventions (no engine changes required)

**Every article includes a `## See Also` section in Markdown body.** This is the manually curated complement to the auto-generated backlinks portlet. Authors add this section; the engine renders it as standard Markdown. Convention: three to five links maximum; no links that are already in `relates_to` frontmatter to avoid duplication.

**Hub articles exist as regular TOPICs.** No special engine treatment required initially — a hub article is a TOPIC with `hub: true` in frontmatter and a body composed primarily of `[[wikilinks]]` grouped under `##` subheadings. The wikilink expansion to card format is the only engine change needed, and that can be deferred.

**Category-config clusters are editorial decisions.** The `content/category-config.yaml` file that defines sub-category groups within each category is maintained by editors, not generated. Adding a new sub-category group requires editing this YAML file and adding a `cluster:` field to affected articles' frontmatter — no engine deploy required.

**The `summary:` field is populated for all articles on projects.woodfinegroup.com before the site launches.** This is a content authoring standard, not an engine enforcement. Articles without `summary:` fall back to auto-extracted first sentence — acceptable but lower quality.

**Featured articles are rotated editorially, not algorithmically.** Editors set `featured: true` on articles they want surfaced. The engine selects from this pool (most recently updated featured article per category). Rotation is controlled by updating `updated:` in frontmatter, not by any automated schedule.

---

### 17.11 Implementation Priority

| Mechanism | Phase | Effort | Primary beneficiary |
|---|---|---|---|
| Status badge in article header | Phase 1 (core article template) | 2 hours | corporate.woodfinegroup.com |
| Category chip in article header | Phase 1 | 1 hour | All three sites |
| `relates_to` rendered as "See Also" block | Phase 1 | 2 hours | documentation.pointsav.com |
| Backlinks portlet (redb → render) | Phase 1 | 4 hours | documentation.pointsav.com |
| Category landing page: article cards with lede | Phase 2 (category routes) | 1 day | All three sites |
| Category landing page: featured card + "recently updated" strip | Phase 2 | 4 hours | documentation.pointsav.com |
| Search results page with article cards | Phase 2 | 1 day | projects.woodfinegroup.com |
| Next/previous within category (requires `position:` field) | Phase 2 | 4 hours | documentation.pointsav.com |
| Homepage: thematic clusters (projects) | Phase 3 (homepage differentiation) | 1 day | projects.woodfinegroup.com |
| Homepage: due-diligence path (corporate) | Phase 3 | 4 hours | corporate.woodfinegroup.com |
| `content/category-config.yaml` sub-category groups | Phase 3 | 4 hours + editorial time | documentation.pointsav.com |
| Hub article render path (`hub: true` detection) | Phase 4 | 4 hours | projects.woodfinegroup.com |
| `GET /new` recently updated feed | Phase 4 | 4 hours | documentation.pointsav.com |
| Sequence header on article pages | Phase 5 (post-launch) | 1 day | projects.woodfinegroup.com |
