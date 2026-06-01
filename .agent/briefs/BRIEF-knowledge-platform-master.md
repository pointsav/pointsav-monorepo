---
artifact: brief
status: active
topic: app-mediakit-knowledge — knowledge platform master spec (federation + mobile-first + premium UX)
archive: project-knowledge
created: 2026-06-01
owner: totebox@project-knowledge
supersedes:
  - BRIEF-app-mediakit-knowledge-2030.md
  - archive/BRIEF-award-winning-wiki-overhaul.md
  - archive/BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md
  - archive/BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md
  - archive/BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md
  - archive/BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md
  - archive/BRIEF-institutional-chrome-sprint.md
  - archive/BRIEF-FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md
  - archive/BRIEF-INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md
  - archive/BRIEF-knowledge-platform.md
  - archive/BRIEF-gemini-handover-2026-05-30.md
research_sources:
  # 2026-05-28 swarm (carried forward from the 2030 brief)
  - agent-home-page-ux-internet-research-2026-05-28
  - agent-article-surface-internet-research-2026-05-28
  - agent-codebase-synthesis-2026-05-28
  # 2026-06-01 swarm (this consolidation)
  - opus-premium-docs-sites-2026-06-01      # Stripe, Vercel, Linear, Apple, Tailwind, Supabase
  - opus-editorial-craft-2026-06-01         # Economist, Stripe Press, GOV.UK, Tufte, Butterick
  - opus-mobile-ux-2026-06-01               # Apple HIG, Material, GOV.UK, Minerva, web.dev viewport
  - audit-current-css-chrome-2026-06-01
  - audit-current-mobile-2026-06-01
  - audit-content-repos-federation-2026-06-01
---

# BRIEF — app-mediakit-knowledge: Knowledge Platform (master)

> **This is the single source of truth for the knowledge platform.** It supersedes the
> 2030 brief and the historical "award-winning-wiki" / "WIKIPEDIA-PARITY" briefs (all
> archived, see frontmatter `supersedes:`). It consolidates the 2026-05-28 research swarm
> *and* the 2026-06-01 swarm (premium-docs UX, editorial craft, mobile UX) plus the
> current-state, mobile, and content-federation audits. Nothing from the predecessors is
> lost — still-valid decisions are carried forward verbatim below.

---

## 0. What changed in the 2026-06-01 consolidation (read first)

The platform was assessed as **C+** and, critically, **fragile on mobile** where ~80% of
traffic actually is. Four decisions reframe the work:

1. **Mobile-first.** ~80% of traffic is phones. The base stylesheet IS the phone layout;
   desktop is layered on with `min-width` queries. (Was: desktop-with-a-drawer.)
2. **Fonts → premium pairing.** Retire Oswald + Nunito Sans + Roboto Slab → **Inter**
   (UI + headings) + **Source Serif 4** (long-form reading body) + **system mono**.
   **This supersedes locked decision L8** (see §2 + Decision Log).
3. **Content federation.** Generalize the hardcoded one-content-dir + two-guide-dir model
   into declarative **mounts + content-type blueprints** (Kirby-style flat-file CMS) so the
   engine federates TOPICs/GUIDEs from many sources and works for external customers + community.
4. **Wikipedia-DNA, Stripe/Linear craft.** Keep the knowledge-base information model
   (wikilinks, history-as-talk, hatnotes, citations); render it in premium visual craft.
   Zero dead links (operator hard rule) — the red-link path is removed.

The full execution plan is §14. The design principle is §12. The federation architecture is §11.

---

## 1. Product identity

`app-mediakit-knowledge` is a sovereign-data, git-native HTTP knowledge platform.
Single Rust binary. Flat-file markdown content store. No PHP, no Node.js runtime, no
MediaWiki, no Hugo, no database migration ladder. **It is already a flat-file CMS in the
Kirby tradition** — this brief generalizes it into a federation platform.

**Live instances:**

| URL | Service unit | Port | Canonical content repo |
|---|---|---|---|
| `documentation.pointsav.com` | `local-knowledge-documentation.service` | 9090 | `media-knowledge-documentation` (+ fleet-deployment guides) |
| `projects.woodfinegroup.com` | `local-knowledge-projects.service` | 9093 | `media-knowledge-projects` |
| `corporate.woodfinegroup.com` | `local-knowledge-corporate.service` | 9095 | `media-knowledge-corporate` |

**One-sentence positioning:** A knowledge platform where every article is git-committed,
every claim is citable and planned for machine-queryable verification, AI agents are
first-class readers but never the author of record, and the entire stack runs on
infrastructure the customer owns — not a third-party cloud.

**Two usage modes (hybrid):** PointSav's own content keeps a curated editorial funnel
(project-* → project-editorial → media-knowledge-*); the declarative mount/blueprint system
is what external customers + community members use to federate their own git repos. One
engine, two modes.

**Why not MediaWiki (or a MediaWiki fork):** ~500K lines of PHP + MySQL; not customer-rooted;
GitHub/DB becomes source of truth; no claim-layer; no modern tokenized CSS. The C+ assessment
is 5 CSS properties + missing federation, not a platform problem. **Decided: do not adopt or
fork MediaWiki.**

**Why not Hugo:** static — no search-as-you-type, no auth-gated content, no edit workflow,
no revision-history UI, no claim verification, no MCP API.

**Market peer:** Q4 Inc. (Toronto; TSXV: QFOR) serves public-company IR. Gap: no
customer-rooted claim layer, no bilingual structured content, no edit-review queue for
regulatory disclosure text. Our differentiation: claim-layer citation verification,
tamper-evident git-native audit trail, no vendor lock-in on the content store.

---

## 2. What's locked (non-negotiable decisions)

Decided. Do not revisit within a session without operator confirmation.

| # | Decision | Rationale |
|---|---|---|
| L1 | Single Rust binary (`cargo build --release -p app-mediakit-knowledge`) | Customer-rooted; no runtime dependency |
| L2 | Git-native flat-file content store (`.md` + `git2`) | Markdown + Git = 50-year readable, diffable, auditable |
| L3 | DTCG token pipeline (`scripts/dtcg-bundle.json` → `dtcg-to-css.py` → `static/tokens.css`) | Single token vault, design-system aligned |
| L4 | Bilingual routing (`.es.md` sibling, single canonical slug) | All public content ships EN + ES |
| L5 | Self-hosted WOFF2 fonts — no CDN | GDPR Art. 44; `784ceea7` removed all Google Fonts CDN links |
| L6 | Wikipedia Vector 2022 DOM conventions where they serve the information model | tooling compatibility; muscle memory — *visual* language is now Stripe/Linear (see §12) |
| L7 | Canonical footer trademark text verbatim (see §9) | Legal; sourced from `wireframe-home-header-v2c.html` |
| ~~L8~~ | ~~Oswald + Nunito Sans + Roboto Slab~~ → **SUPERSEDED 2026-06-01** | **New L8: Inter (UI+headings) + Source Serif 4 (reading body) + system mono.** See §7 + Decision Log. |
| L9 | `--navy: #164679`; `--bg: #F7F9FA`; `--link: var(--navy)` | Core brand token triad; WCAG AA verified |
| L10 | MCP JSON-RPC 2.0 native (`src/mcp.rs`) | Doctrine claim #54 ("We Own It"); no vendor SDK |
| L11 | Claim-layer HTML comment markup (`<!--claim id=... confidence=... cites=[]-->`) | In production content; foundation for §13 |
| L12 | SYS-ADR-07: no structured data through AI | deterministic pipelines only |
| L13 | SYS-ADR-10: F12 mandatory; human commits only | edit-review queue enforces |
| L14 | SYS-ADR-19: no automated AI publishing to verified ledgers | AI marginalia is ephemeral overlay |
| L15 | Apache 2.0 licence | matches monorepo |
| L16 | Commit identity `jwoodfine`/`pwoodfine` only; `commit-as-next.sh` only | pre-commit gate enforces |
| **L17** | **Mobile-first** — base stylesheet = phone; desktop via `min-width`. ~80% traffic is mobile | 2026-06-01 mobile audit |
| **L18** | **Zero dead links** — every `[[ ]]` resolves or is not a link; no red-links | operator hard rule 2026-06-01 |
| **L19** | **Federation via declarative mounts + content-type blueprints** | hybrid platform decision 2026-06-01 |

---

## 3. Current implementation state (Phases 1–8 + Leapfrog design shipped, 2026-05-30)

All commits promoted to canonical; later typography fix `dff4e2a7` deployed 2026-06-01.

| Phase | Status | Notes |
|---|---|---|
| 1 / 1.1 — render + Wikipedia chrome | Shipped | `/wiki/{slug}`, TOC, hatnote, Article/Talk/History tabs, footer |
| 2 — edit (Steps 1–7) | Shipped | JSON-LD, atomic edit, CodeMirror 6, SAA squiggles, citation autocomplete |
| 3 — search + feeds | Shipped | Tantivy BM25, `/feed.atom`, `/sitemap.xml`, `/llms.txt` |
| 4 — git sync + MCP + DTCG | Shipped | git2, redb link graph, blake3, MCP JSON-RPC 2.0, git smart-HTTP, OpenAPI 3.1; 157 oklch tokens |
| 5 core — auth + edit review | Shipped | cookie sessions, argon2id, edit-review queue (`auth.rs`+`pending.rs`+`users.rs`) |
| 5.1+ ACLs/OIDC/webhooks | Deferred | gated on BP5 |
| 6A/6B/6C, 7A–7H, 8 | Shipped + deployed | AJAX nav, home caps, 80px topnav, article-tabs, reading mode, citation hover, mobile bottom bar, Tufte sidenotes, corporate auto-numbering, history surface |
| Leapfrog design (fonts/layout/content-types) | Shipped + deployed | Source Serif 4 reading body; full-width single-column; Kirby `content_type:` blueprint seed |
| Typography fix `dff4e2a7` | Deployed 2026-06-01 | (interim — superseded by the Inter migration in §7/Phase 1) |

**Still open / corrections needed (Command Session):**
- `.agent/manifest.md` + `briefs/README.md` are **contaminated** — they describe this archive as
  project-intelligence/project-bim. project-knowledge metadata needs correction.
- The briefs dir holds ~contamination from project-console/intelligence/infrastructure (see §15).
- Live services read STALE/old-named content dirs (`content-wiki-*`) vs canonical `media-knowledge-*`;
  projects is serving behind canonical. Resolved by the Phase 0 mount manifest (§11, §14).
- UX-B.7 BLOCKED: `WORDMARK_WOODFINE` still placeholder — operator must provide Woodfine SVG wordmark.

---

## 4. Three-instance differentiation

One binary serves three editorial brands. Flags: `brand_theme: BrandTheme` + `brand_instance:
BrandInstance`; `<html data-instance>` enables per-instance CSS scoping.

| Instance | Domain | Brand | Token file | `data-instance` |
|---|---|---|---|---|
| documentation | documentation.pointsav.com | PointSav | tokens.css | `pointsav` |
| projects | projects.woodfinegroup.com | Woodfine | tokens-woodfine.css | `woodfine-projects` |
| corporate | corporate.woodfinegroup.com | Woodfine | tokens-woodfine.css | `woodfine-corporate` |

**Per-brand contract (≤12 tokens, §16):** accent, optional display-accent font, scale-ratio,
density/line-height, drop-cap & pull-quote gating. Shared craft engine is constant. Docs =
denser technical; corporate/projects = editorial gravitas (institutional-finance register).

**Cross-instance isolation is structural:** each instance mounts a disjoint source set + owns
its link graph (`state` / `state-projects` / `state-corporate`). A `[[slug]]` resolves only
within its instance. Never introduce a shared/global resolver (see §11, §12).

---

## 5. Home / main page

### Current state
`home_chrome()` renders hero + featured + recent + category grid + sister-surface footer band.

### Targets (mobile-first)
Single-column phone layout first; step to 2-col @640, multi @1024. Search hero with instant
autocomplete (`/api/complete` exists). Featured article via frontmatter `featured: true`.
Category grid reads category from blueprint + frontmatter (+ mount `section`), auto-synthesizing
a section landing where `_index.md` is absent. Real tap/hover card lift (<150ms). All targets ≥44px.
"Did you know?" + reading paths are post-launch enhancements. Per-instance hero.

---

## 6. Header + navigation

### Current state
Single-row `header.topnav` (80px, `1fr/auto/1fr`): wordmark · search · language toggle · user menu.
Emitted three times (`home_chrome`/`wiki_chrome`/`chrome`).

### Targets
- **Mobile:** sticky 56px top bar `[☰] [wordmark] [🔍]`; sticky 56px **bottom action bar**
  (thumb zone) `[Search] [On-this-page] [Menu] [Back-to-section]`, padded by `safe-area-inset-bottom`.
- **Desktop (≥1024):** three-column shell — `docs-sidenav` 256px (active = accent text + 2px
  left bar + tint, animated chevron) · prose @measure · right TOC rail.
- **Command palette (Cmd/Ctrl-K):** full-screen overlay on mobile (svh-sized, input ≥16px,
  results ≥44px, programmatic focus after open-animation); palette on desktop. Fuzzy over titles.
- Article-tabs row (Article/Talk/Read/Edit*/History/Tools▾) on `wiki_chrome` only; History = git.
- Per-section anchor-share `¶` + contributor edit pencil; language switcher resolves `.es` sibling.

---

## 7. Article reading surface + typography

### Font stack (NEW L8 — 2026-06-01)
- **UI + headings:** **Inter** (400/500/600), self-hosted WOFF2 — already on disk.
- **Long-form reading body:** **Source Serif 4** (400/700 + italic), self-hosted — already on disk.
- **Code / labels:** system mono (`ui-monospace, "SF Mono", Menlo, Consolas`).
- Retire Oswald + Nunito Sans + Roboto Slab. `@font-face` with metric-override fallback (kill CLS).
- *Rationale:* the 2026-06-01 research found the three-voice display-condensed stack reads C+;
  premium docs (Stripe/Vercel/Linear) use one humanist sans + a serif reading body + mono.

### Reading targets (mobile-first)
- Body **17px** mobile (step to 19px ≥640, GOV.UK pattern), line-height 1.6, 16px side gutters.
- One measure token `--measure: 68ch` (degrades to ~38–42ch on a 390px phone naturally);
  delete the conflicting `--reading-max: 595px` and `--knowledge-editorial-article-max: 720px`.
- `text-wrap: pretty` body / `balance` headings; `scroll-margin-top` on headings for sticky-header
  anchor jumps; oldstyle numerals in prose, lining+tabular in tables; hairline blockquote (2px);
  figure/caption run-in `Fig. N —`.
- Heading hierarchy by weight + space-above≫space-below (not size alone — h2/h3 currently collapse).
- Code blocks: 1px border (not shadow), header bar w/ language label + copy button, lh ~1.6,
  light/dark-adaptive, **never wrap code**, horizontal scroll + right-edge fade on mobile.
- Reading mode (warm paper, hidden chrome, ~64ch) + density control (logged-in) retained as targets.
- Tufte sidenotes for `layout: journal` at ≥1280px (serves the J1–J6 academic programme).

---

## 8. Article toolbars
Progressive disclosure: anonymous sees Read + Article; everything else in `Tools ▾` or
contributor-only. Per-section `¶` anchor-share (all readers) + edit pencil (contributors).
"View source" (not "View on GitHub" — We Own It) via `?action=raw`. Print/export via `@media print`.
History surface: reverse-chron list, line-level diff, `article-integrity-bar` blake3 fingerprint.

---

## 9. Footer — canonical text (verbatim — do not modify)

Sourced from `wireframe-home-header-v2c.html`. All three instances:

```
© 2026 Woodfine Capital Projects Inc. All rights reserved.
Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital
Projects Inc. used in Canada, the United States, Latin America, and Europe. All other
trademarks are the property of their respective owners.
```

Year field updates annually. Corporate instance adds `effective_date:`/`supersedes:` disclosure
block under `h1`, auto-numbered sections, and suppresses the "Was this helpful?" widget (gate on
`brand_instance`).

---

## 10. Mobile — the headline fixes (80% of users)

The 2026-06-01 mobile audit found these CRITICAL defects. All are L17 (mobile-first) scope.

| # | Defect | Current | Fix |
|---|---|---|---|
| M1 | Hover-only features dead on touch | hover cards, glossary, footnote + citation tooltips fire on `mouseenter` only | tap-to-open popover/bottom-sheet under `@media (hover:none)`; hover = desktop enhancement |
| M2 | iOS zoom-on-focus | topnav search 12px / 28px | all inputs ≥16px on `pointer:coarse`; search → full-screen overlay |
| M3 | Touch targets below floor | nav/TOC links 26–35px | 44px min (48px primary); pad hit-area |
| M4 | No safe-area insets | bottom bar hides behind home indicator; no `viewport-fit` | `viewport-fit=cover` + `env(safe-area-inset-*)` on fixed chrome |
| M5 | `100vh` layout shift | sidenav/toc-rail use `100vh` | `svh`/`dvh` dynamic viewport units |
| M6 | Fragmented breakpoints | 640/767/768/875/1023/1100/1280 + orphan @760 | one ladder: base · 480 · 640 · 768 · 1024 · 1280 |
| M7 | No tap feedback | no `-webkit-tap-highlight-color` | `transparent` + custom `:active` |
| M8 | Drawers don't animate | `display:none` toggle | slide/fade + `overscroll-behavior:contain` |
| M9 | Tables/code overflow unmarked | `overflow-x:auto` no affordance; code unhandled | edge-fade mask; sticky copy bar; never wrap code |

Breakpoint reference: Minerva 320/720/1000 + GOV.UK 640. Bottom action bar = thumb-zone
(NN/g: bottom nav beats hamburger 30–50% on discovery). TOC = bottom sheet, IntersectionObserver
scroll-spy. `dvh` for full-height overlays (Baseline since 2025).

---

## 11. Content Federation Architecture — "flat-file mounts + content-type blueprints"

**Decisions: Hybrid federation + full blueprint registry.** Generalizes the engine's two
hardcoded assumptions (one `WIKI_CONTENT_DIR` + two `WIKI_GUIDE_DIR`s; free-text `type:`) into a
real product for external customers + community.

**1. Source mounts.** Per-instance `knowledge.toml`: list of `[[mount]]` `{ id, path,
default_type, section?, editable }`. Engine builds ONE virtual content tree across all mounts →
one per-instance slug namespace + link graph + search index. Generalizes the existing
`collect_all_topic_files(content_dir, &[guide_dir, guide_dir_2])`. AppState
`{content_dir, guide_dir, guide_dir_2}` → `{mounts: Vec<Mount>}`. The manifest **replaces** the
stale `WIKI_CONTENT_DIR`/`WIKI_GUIDE_DIR` env wiring — writing it with canonical
`media-knowledge-*` paths resolves the repoint as a side effect.

**2. Content-type blueprints.** Flat YAML, git-tracked: `blueprints/<type>.yaml` =
`{ required fields, section, template, relates_to[] }`. `topic` + `guide` built-in; customers add
`regional-market` (structured infobox: rank/score/tier_counts/suburb_of/distance_km/climate/civic),
`adr`, `changelog`. `relates_to` drives cross-link rails generically.

**3. GUIDEs stay in fleet-deployment — DO NOT move them.** The engine already federates
`WIKI_GUIDE_DIR`=`pointsav-fleet-deployment` + `WIKI_GUIDE_DIR_2`=`woodfine-fleet-deployment` on
the documentation instance only; guides serve at `/wiki/<slug>` "just like TOPICs". §14 (Foundry
taxonomy) honored, repo structure untouched. corporate/projects have no guide dirs → isolation intact.

**4. Linking + zero dead links (L18).** **Engine gap to fix:** `inject_wiki_prefixes(html,
content_dir)` (`render.rs:420`) only receives `content_dir`, so `[[guide-slug]]` from a TOPIC
red-links today. Thread the full mount set so wikilinks resolve across topics+guides+all mounts.
Build-time resolver = hard gate (unresolved `[[ ]]` blocks promote); render fallback = plain text;
**remove the red-link path** (`render.rs:464`). Typed backlinks (`links.rs::backlinks`) power the
TOPIC "How-to guides" rail ↔ GUIDE "Background concepts" rail.

**5. Provenance + edit-routing.** Each article remembers source mount + git origin; UI shows
"Source: <mount> · History"; Edit commits to editable mounts, "propose change in origin" for
read-only. Contributors' content never leaves their repos.

**Content-IA reconciliation:** the blueprint+mount model resolves the docs(subdirs+`_index`) vs
corporate/projects(flat, `topic-` prefix) inconsistency toward **engine-adapts**: section comes
from blueprint + frontmatter, no physical restructuring required. Still **normalize slugs** (strip
`topic-` prefix) for clean URLs + wikilink consistency (`migrate_corpus.py` already does this for docs).

---

## 12. Design principle — "Wikipedia model, Stripe/Linear craft"

Keep knowledge-base DNA as tasteful, esoteric nods; render in premium craft. Asset for a
Knowledge CMS — *if* restrained.
- **Keep (DNA):** `[[wikilinks]]` (every one resolves — L18); Article/Talk/**History**(=git) tabs
  as the engine's git-native identity; hatnotes; See-also; citation superscripts w/ hover-or-tap;
  categories footer; "What links here" backlinks.
- **Drop (C+ tells):** full-width serif at cramped leading; default-blue underlined links
  everywhere; grey-gradient Vector rail; encyclopedia/puzzle-globe visual language; metadata
  clutter; **red-links** (L18).
- Foundation craft (from 2026-06-01 research): 8px spacing grid + 4px half-step; modular type
  scale (~1.25 body) with size-dependent tracking; single neutral ramp, text not pure black, one
  accent/brand, `::selection` tint; borders for flat surfaces + hue-matched layered shadows;
  `:focus-visible` 2px accent on every interactive element; motion 150/200/280ms ease-out +
  `prefers-reduced-motion` kill-switch; custom thin scrollbars; `text-underline-offset:2px`.
- **Net: Wikipedia's information model, Stripe/Linear's visual language.**

---

## 13. Differentiation (built + planned)

**Already built (≈22):** native MCP JSON-RPC 2.0; claim-layer markup; redb wikilink graph +
backlinks; blake3 hashes; Tantivy BM25 + autocomplete; git smart-HTTP; OpenAPI 3.1 + `/llms.txt`;
bilingual `/es/`; edit-review queue; SAA squiggles; citation autocomplete; revision history + diff;
DTCG tokens; self-hosted fonts; `data-auth`/`data-instance` state machine; feeds; argon2id auth.

**Planned three:** (A) **claim-rail freshness sidebar** at ≥1280px (`citations` redb table +
nightly URL validation); (B) **AI marginalia** opt-in ephemeral overlay (SYS-ADR-19 compliant,
service-slm, never committed); (C) **cross-session reading state** (localStorage + optional
self-hosted sync). Plus `query_claims(topic, asof)` MCP API — the planned regulated-industry moat.

---

## 14. Execution plan

**Phase −1 (documentation consolidation) is the FIRST step** — this brief + aligning the
TOPIC/GUIDE/rules docs to the research, so it is the durable source of truth. (In progress now.)
Governance: BRIEFs are permanent (archive, never delete); TOPIC/GUIDE content is editorial →
stage to `.agent/drafts-outbound/` → route to project-editorial; generic tokens → project-design
DESIGN-TOKEN-CHANGE w/ master_cosign; Woodfine tokens → woodfine-media-assets.

- **Phase −1** — this master brief; archive predecessors; correct README; align content docs (drafts).
- **Phase 0** — federation engine (Rust): `knowledge.toml` mounts; `blueprints/*.yaml` + `src/blueprints.rs`;
  thread mounts into `inject_wiki_prefixes` (cross-mount resolution); build-time dead-link gate;
  remove red-link path; typed TOPIC↔GUIDE backlinks; slug normalization; Regional-Market infobox blueprint.
  Gates the nav/sidebar in Phases 2–3.
- **Phase 1** — foundation: breakpoint ladder; mobile primitives (M4/M5/M7); 8px grid; modular type
  scale; color/depth; motion+focus; Inter + Source Serif 4 `@font-face`; one `--measure` token.
- **Phase 2** — article surface, phone reading first; bottom action bar; bottom sheets; tap-popovers
  (M1); tables/code (M9); desktop three-column layer ≥1024.
- **Phase 3** — home page, phone first.
- **Phase 4** — Cmd+K palette (full-screen overlay on mobile) + animate the ~23 instant interactions.
- **Phase 5** — per-brand theming (≤12-token contract) + DTCG back-port.

CSS is embedded in the binary → each visual iteration = edit → `cargo build --release` →
`deploy-binary.sh` → restart 3 services. Verify phone-first (DevTools @390px + a real phone),
then desktop, all three instances, light + dark.

---

## 15. Brief hygiene (this archive)

The `.agent/briefs/` dir + `README.md` + `.agent/manifest.md` are **contaminated** by rebases —
they describe this archive as project-intelligence/infrastructure/console. Genuine
project-knowledge briefs: **this master** (active) + `BRIEF-active-work.md` (session queue) +
the archived wiki predecessors. Flagged for their owning archives (do not action here):
SLM (`BRIEF-slm-*`), intelligence (`BRIEF-project-intelligence-active-work`, `substrate-phd-thesis`),
infrastructure (`BRIEF-PPN-*`, `totebox-transformation`, `vm-hardening`, `VM-ARCHITECTURE`, `OS-FAMILY`),
console (`BRIEF-pairing-*`, `os-console-platform`, `cross-platform-release`, `tui-pivot`, `leapfrog-2030-coding`),
editorial (`BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN`, `journal-phd-programme`, `framework-pointsav-products-services`,
`overhaul-*`, `github-presence-elevation`, `publishing-tier-naming-cross-check`).

---

## 16. Key files

| File | Role |
|---|---|
| `src/main.rs` | mount-manifest loader (replaces guide_dir env args) |
| `src/server.rs` | HTTP handler, routing, AppState `{mounts}`; `home_chrome`/`wiki_chrome`/`chrome` |
| `src/render.rs` | `inject_wiki_prefixes` (cross-mount); remove red-link path @464 |
| `src/links.rs` | redb link graph + `backlinks()` typed by blueprint |
| `src/mcp.rs` / `src/auth.rs` / `src/pending.rs` / `src/claim.rs` | MCP, auth, edit-review, claims |
| new `src/blueprints.rs` + `blueprints/*.yaml` | content-type schema registry |
| new `knowledge.toml` (per instance) | declarative `[[mount]]` manifest |
| `static/style.css` | mobile-first rewrite (tokens, scale, components, all surfaces) |
| `static/wiki.js` | tap popovers, Cmd+K, animated drawers |
| `static/fonts/Inter-*`, `Source-Serif-4-*` | present; new `@font-face` (drop Oswald/Nunito/Roboto Slab) |
| `scripts/dtcg-bundle.json` / `dtcg-to-css.py` / `static/tokens.css` / `tokens-woodfine.css` | token pipeline + per-brand |
| `ARCHITECTURE.md` / `NEXT.md` / `openapi.yaml` | phase plan, open items, API spec |

---

## Decision Log

| Date | Decision | Rationale / authority |
|---|---|---|
| 2026-06-01 | **Supersede L8: Inter + Source Serif 4 + system mono** (was Oswald/Nunito/Roboto Slab) | Operator approved; 2026-06-01 premium-docs research found the 3-voice condensed stack reads C+. Surfaced as a BRIEF §7 conflict per workspace rules; logged here + NEXT.md + outbox to Command. |
| 2026-06-01 | **L17 mobile-first** | ~80% traffic is mobile; mobile audit found 9 critical defects (§10) |
| 2026-06-01 | **L18 zero dead links** | Operator hard rule "no links that go nowhere"; remove red-link path |
| 2026-06-01 | **L19 federation via mounts + blueprints (hybrid)** | Operator chose Hybrid + full blueprint registry; makes the engine a platform for external customers/community |
| 2026-06-01 | **Do not adopt/fork MediaWiki** | Operator question resolved; C+ is CSS + federation, not a platform problem |
| 2026-06-01 | **GUIDEs stay in fleet-deployment, federated via guide-dirs** | corrects an in-session "bring guides into each repo" idea; §11.3 |
| 2026-05-28 | Consolidated predecessors into the 2030 brief (now this master) | carried forward |

*This master supersedes all predecessors listed in frontmatter `supersedes:`.*
