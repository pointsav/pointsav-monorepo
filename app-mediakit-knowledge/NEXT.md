# NEXT.md — app-mediakit-knowledge

> Last updated: 2026-05-29 (session 2 — Phase 8 marked complete)

## Phase 7E — COMPLETE (2026-05-29)

Mobile chrome: bottom action bar, mobile table overflow, mobile code font.
Files: `src/server.rs`, `static/style.css`, `static/wiki.js`.

- **`nav.mobile-bottom-bar`** added to `wiki_chrome()` (after mobile-nav-overlay). Four actions:
  Contents (opens TOC drawer), Share (`navigator.share` or clipboard fallback), Edit (link to edit,
  auth-gated via `[data-auth="anon"] .tab-edit { display: none; }`), History.
  Fixed `bottom: 0; height: 56px; z-index: 100;` — visible only on `≤767px`.
- **`nav.article-tabs` hidden on mobile** (`@media (max-width: 767px) { nav.article-tabs { display: none; } }`).
  Bottom bar replaces it.
- **`body { padding-bottom: 56px; }`** on mobile — prevents bottom bar overlap with content.
- **Mobile table overflow:** `.page-body table { display: block; overflow-x: auto; -webkit-overflow-scrolling: touch; }`
- **Mobile code font:** `.page-body pre { font-size: 12.5px; }` on mobile.
- **`initMobileBottomBar()`** in wiki.js: Contents button delegates to existing `#toc-toggle-btn`;
  Share button calls `navigator.share()` with page title + URL, falls back to clipboard.
  Called in DOMContentLoaded boot sequence.

Binary rebuild + deploy required (rust-embed). Stage 6 pending.

---

## Phase 7D — COMPLETE (2026-05-29)

Citation hover preview, freshness dot, `CITATIONS` redb table.
Files: `src/links.rs`, `src/render.rs`, `src/server.rs`, `static/style.css`, `static/wiki.js`.

- **`src/links.rs`** — `CITATIONS` redb table added (`TableDefinition<&str, &str>`; key=`cite_id`,
  value=JSON blob). `record_citation(cite_id, url, title)`, `lookup_citation(cite_id)`,
  `citation_status(cite_id)` API added. Table initialised in `open_or_create()`.
- **`src/render.rs`** — `inject_citation_markers(html)` post-processor: finds comrak
  `<sup class="footnote-ref">` elements and appends a
  `<span class="freshness-dot" data-status="unknown"></span>` before `</sup>`.
- **`src/server.rs`** — `inject_citation_markers()` wired into wiki_page render chain after
  glossary tooltips, before heading extraction.
- **`static/style.css`** — `.freshness-dot` (5px circle, oklch colors per status: fresh/stale/unknown).
  `.cite-hover-card` (absolute positioned card, 300px max, shadow + border).
- **`static/wiki.js`** — `initCitationHoverCards()`: mouseenter on `sup.footnote-ref` injects card
  populated from matching `<li id="fn-N">` in the footnotes section. Card dismissed on mouseleave.
  Called in DOMContentLoaded boot.

Phase 7X is already implemented (YAML-based featured article + DYK; search in `section.hero`).
Next: Phase 7E (mobile chrome) or 7F (Tufte sidenotes).

Binary rebuild + deploy required (rust-embed CSS/JS). Stage 6 pending.

---

## Phase 7C — COMPLETE (commit `d649f051`, 2026-05-29)

Reading mode toggle. Files: `src/server.rs`, `static/style.css`, `static/wiki.js`.

- **`button.reading-mode-btn #reading-mode-btn`** added to article-tabs right section in `wiki_chrome`
  (after the Tools dropdown). `aria-pressed` attribute updated on toggle.
- **CSS:** `body.reading-mode` hides `nav.article-tabs`, `nav.crumb`, `nav.sidebar`, `footer.shell-footer`,
  `aside.toc`. Collapses `div.shell` to single column. `main.article-wrap` centered at `72ch`.
  `.reading-mode-btn` styles with `aria-pressed="true"` visual indicator.
- **JS `initReadingMode()`:** Reads/writes `wiki-reading-mode` localStorage key. Toggles `body.reading-mode`
  class and `aria-pressed` attribute on click. Called in DOMContentLoaded boot sequence.

Also fixed: `WIKI_BRAND_INSTANCE` env var added to `local-knowledge-corporate.service` and
`local-knowledge-projects.service` (was missing — both instances were defaulting to "documentation",
causing PointSav copyright and CC BY 4.0 to show on Woodfine instances).

Binary rebuild + deploy required. Stage 6 pending (commit `d649f051`).

---

## Phase UX-B — COMPLETE (commit `2a19c626`, 2026-05-29)

Rust chrome refactor. Files: `src/server.rs`, `static/style.css`.

- **Appearance dropdown removed from Rust HTML:** `div.wiki-appearance-wrap` deleted from `home_chrome`
  and `wiki_chrome`. Theme now follows `prefers-color-scheme` silently. UX-A CSS suppression removed
  (element is gone). `chrome()` was already clean.
- **Home standfirst added:** `p.home-standfirst` renders above "Browse by area" grid with per-instance
  description text (documentation / projects / corporate variants).
- **`shell_footer()` extracted:** Single shared footer function replaces three near-identical footer
  blocks in `home_chrome`, `wiki_chrome`, and `chrome`. Accepts `brand_instance` and optional
  `view_source_slug`.
- **Footer convergence:** Visible footer is now 3 lines max (cities · nav · copyright). Trademark notice,
  Contact, View source, and Media Kit links collapse into `details.footer-more`. Dramatically reduced
  information density matches home site standard.
- **CC BY 4.0 badge gated:** Badge not rendered on `brand_instance == "corporate"`. Corporate policy
  documents are proprietary — the CC licence badge was legally incorrect there.
- **Per-instance copyright line:** documentation → "© 2026 PointSav Digital Systems"; projects + corporate
  → "© 2026 Woodfine Management Corp."
- **Provenance ribbon:** `div.article-provenance` added to `wiki_chrome` under `h1.article__title`, showing
  last edited date (from `fm.last_edited`) and a "View history" link.

Binary rebuild + deploy required. Stage 6 pending (commit `2a19c626`).

**UX-B.7 — BLOCKED (Woodfine SVG wordmark):** `WORDMARK_WOODFINE` constant is still Unicode `■ Woodfine`.
Operator must provide the Woodfine Management Corp. SVG wordmark to complete this item.

---

## Phase UX-A — COMPLETE (commit `0dfe1647`, 2026-05-29)

CSS-only institutional quality pass. Files: `static/style.css`, `static/tokens-woodfine.css`.

- **Typography tokens wired:** `.page-body` now consumes `--knowledge-editorial-reading-body-size` (17px)
  and `--knowledge-editorial-reading-body-lh` (1.70) from DTCG pipeline. `--reading-max` corrected to 720px.
- **Dark-mode link contrast fixed:** `--navy` overridden to `oklch(62% 0.14 250)` (≈ #4d8fd1, 4.7:1 on
  `#0B1220`) in `html[data-theme="dark"]`. Woodfine `--interactive-link` matching override in
  `tokens-woodfine.css`.
- **Auto dark mode added:** New `@media (prefers-color-scheme: dark)` block mirrors all dark-mode
  variables — OS preference now activates dark mode without the manual toggle.
- **Appearance dropdown suppressed:** `.wiki-appearance-wrap { display: none !important; }` — dark mode
  follows `prefers-color-scheme` silently (Goldman/Bloomberg/Refinitiv institutional standard).

Binary rebuild + deploy required to serve updated embedded CSS. Stage 6 pending (commit `0dfe1647`).

---

## Phase 7B — COMPLETE (2026-05-29)

`nav.article-tabs` two-row header added to `wiki_chrome()` only: Article/Talk (left) +
Read/Edit/History/Tools▾ (right). Tools dropdown (`details.tools-dropdown > summary + ul`):
Cite/Permanent link/Printable/Page info/What links here. `¶` anchor-share buttons on
`h2[id]`/`h3[id]` headings (`initAnchorShare()` in wiki.js, re-runs on AJAX nav).
Auth-gating via `[data-auth="anon"]` CSS: Talk + Edit tabs hidden for anonymous users.
Dead `.shell-header,` selectors removed from `static/style.css` (3 occurrences).
106/106 lib tests pass.

## Phase 7A — COMPLETE (commit `168314a1`, 2026-05-28)

TOC toggle/pin buttons restored to `div.toc__header`; `div.topnav-search-wrap` +
`#search-autocomplete-dropdown` added to all three chrome functions. Stage 6 +
binary rebuild queued for nightly ~1am Vancouver 2026-05-28.

## Phase 7+ backlog (from BRIEF-app-mediakit-knowledge-2030.md §12)

| Phase | Scope | Status |
|---|---|---|
| **UX-A** | Typography tokens; dark-mode contrast; auto dark mode; suppress appearance toggle | **COMPLETE** (commit `0dfe1647`) |
| **UX-B** | Remove appearance dropdown (Rust); home standfirst; footer convergence; `shell_footer()`; CC BY 4.0 gate; provenance ribbon | **COMPLETE** (commit `2a19c626`) |
| **UX-B.7** | Woodfine SVG wordmark | **BLOCKED — operator must provide SVG asset** |
| **7B** | Article-tabs row (40px); Tools dropdown; anchor-share `¶`; auth-gated tabs | **COMPLETE** |
| **7C** | Reading mode toggle; CSS body-class; localStorage | **COMPLETE** (commit `d649f051`) |
| **7X** | Home page: search hero, featured article, DYK section | **Already implemented** (YAML-based: `featured-topic.yaml`, `leapfrog-facts.yaml`; hero search in `section.hero`) |
| **7D** | Citation hover preview; freshness dot; citations redb table | **COMPLETE** |
| **7E** | Mobile chrome: bottom bar; table overflow; code font; article-tabs hidden on mobile | **COMPLETE** |
| **7F** | Tufte sidenotes for `layout: journal` articles at ≥1280px | **COMPLETE** (commit `c240837b`) |
| **7G+7H** | Corporate: effective\_date block; auto-numbered sections (CSS counters) | **COMPLETE** (commit `c240837b`, CSS-only: auto-numbered sections) |
| **8** | History surface: revision list, diff UI, integrity-bar (blake3 SHA) | **COMPLETE** (commit `0e5fd685`, 2026-05-29) |
| **9** | Claim-rail freshness sidebar; citations redb table; nightly URL validator | Queued (after 7D) |
| **10** | Reading state persistence; progress bar; "Continue reading" home strip | Queued |
| **11** | `query_claims(topic, asof)` MCP API extension | Queued (after 9) |
| **12** | AI marginalia | **GATED — BP5 + SYS-ADR review required** |

## Phase 4 DTCG token wiring — COMPLETE (Commits F–H, 2026-05-22)

Phases 4.2–4.5 of `KNOWLEDGE-PLATFORM-PLAN.md` committed on monorepo `main`:

| Commit | Phase | What |
|---|---|---|
| `bce932b1` | 4.2 — DTCG build script | `scripts/dtcg-bundle.json` (vendored canonical) + `scripts/dtcg-to-css.py`; generates `static/tokens.css` (148 tokens, all colors in oklch()) |
| `1ddfca98` | 4.3+4.4 — reconcile `:root` + theme switch | `style.css` `:root` aliases → DTCG semantic vars; `tokens-woodfine.css` full Woodfine brand override; conditional `<link>` in chrome when `WIKI_BRAND_THEME=woodfine` |
| _(this commit)_ | 4.5 — WCAG audit | See findings below |

## Phase 4.5 — WCAG 4.5:1 audit findings (2026-05-22)

**Audit scope:** all color pairs in DTCG semantic token set — 12 foreground/background
combinations checked programmatically via relative-luminance formula.

**Results: 10 pass, 2 fail AA (4.5:1):**

| Token pair | Hex FG | Ratio | 4.5:1 AA | 3:1 large |
|---|---|---|---|---|
| `semantic.text.tertiary` on `semantic.surface.background` | #878d99 | 3.08:1 | FAIL | PASS |
| `knowledge.editpencil` on `semantic.surface.layer` | #878d99 | 3.33:1 | FAIL | PASS |

**Assessment:** Both failures use `#878d99`. Both are decorative/supplementary roles:
- `text.tertiary` — placeholder text, disabled labels; qualifies as non-text UI (WCAG 1.4.11, 3:1 threshold) rather than body text (4.5:1)
- `knowledge.editpencil` — edit pencil icon overlay on article text; decorative icon, non-interactive at hover-only visibility; 3:1 threshold applies

**Both colors PASS 3:1 large-text / non-text contrast.** No accessibility regression introduced by Phase 4.

**Fix required at token source (project-design scope):** To meet strict body-text 4.5:1, darken `#878d99` to ≈ `#767c8a` (ratio 4.52:1) in `dtcg-vault/tokens/dtcg-bundle.json`. Outbox message sent to project-design. This is not a blocker for Phase 5.

## Closed: Phase 5 — bilingual /es/ routing (2026-05-22 / 2026-05-23)

`/es/` + `/es/wiki/{*slug}`, ES file fallback, `html lang=`, hreflang tags, language
switcher in nav. Accept-Language → /es/ auto-redirect with `?noredirect=1` suppression
added 2026-05-23 (Commit O, `c2d4010c`). 4 tests added.

## Closed: crate hygiene (Commit K, 2026-05-22)

`cargo fmt` + `cargo clippy -D warnings` — 24 pre-existing lints fixed across
`feeds.rs`, `glossary.rs`, `history.rs`, `render.rs`, `search.rs`, `server.rs`,
`edit.rs`, `main.rs`, and test files. Committed 11d482f2.

## Closed: RATIFIED_CATEGORIES → 12 items (Commit K, 2026-05-22)

Added "company" (after "infrastructure") and "help" (after "reference").
All 8 home_test integration tests now pass. Committed 11d482f2.

## Closed: CLAUDE.md / ARCHITECTURE.md accuracy pass (Commit L, 2026-05-22)

Both files updated: collab removed from Phase 2 row; Phase 5 marked shipped;
new KNOWLEDGE-PLATFORM-PLAN.md phases 1/3/4/5 documented. Committed 6180b074.

## Closed: openapi.yaml accuracy pass (Commit N, 2026-05-23)

15 missing routes added: Phase 5 `/es/` routes, auth/pending special pages,
`/api/complete`, `/api/preview/{slug}`, `/category/{name}`, `/talk/{slug}`.
Category enum corrected (company + help). Collab flag reference removed. `826d42a5`.

## Closed: Accept-Language → /es/ redirect (Commit O, 2026-05-23)

`prefers_spanish()` helper; `IndexQueryParams.noredirect`; ES home lang-toggle
links to `/?noredirect=1`; 4 tests. `c2d4010c`.

## Closed: README refresh (Commit P, 2026-05-23)

Phase 2 row: collab removed. Phase 5.1 bilingual routing marked shipped.
Missing `<div>` in EN README fixed. `7a7beb46`.

## Phase 6A+6B+6C — COMPLETE (commit `afa67bfa`, 2026-05-28)

| Phase | What |
|---|---|
| 6A — wiki.js AJAX nav fix | `navigateTo()` stale selectors fixed (3 pairs); `initToc`, `initTocPin`, `initActiveTocTracking` corrected; `id="toc-list"` added to server.rs |
| 6B — home page section caps | Uncategorised block removed; guides capped at 6; data fetch aligned to 8 |
| 6C — topnav header | `header.topnav` 1fr/auto/1fr grid in all 3 chrome functions; `WORDMARK_SVG_POINTSAV` constant; `--header-h` 152px→80px |

Stage 6 promoted by Command. Binary rebuild queued in nightly (~1am Vancouver 2026-05-28).

**After rebuild verify:**
- documentation.pointsav.com topnav SVG wordmark visible; sidebar sticky top correct
- Click any article link — title, TOC, breadcrumb all update (was broken pre-6A)
- Home page guides section caps at 6 items

**Post-6C cleanup (future session, not blocking):**
- [ ] Remove legacy `.shell-header` CSS block (now dead code) — low priority
- [ ] ES bilingual pairs for 4 governance stubs (disclaimers, contact, about, contribute)
- [ ] `.agent/manifest.md` wrong `cluster_name` (project-bim) — Command correction needed
- [ ] Dark mode topnav: verify SVG invert looks correct on dark backgrounds

## Open: Stage 6 promotion

**COMPLETE (2026-05-28)** — `afa67bfa` promoted to canonical by Command. Binary rebuild
queued for nightly ~1am Vancouver. Prior binary remains active until rebuild completes.

---

> Historical NEXT.md content (pre-2026-05-22 plan) preserved below for reference.
> The items below reflect the old Phase numbering (git-based Phase 4, auth Phase 5).
> Cross-reference against `KNOWLEDGE-PLATFORM-PLAN.md` for current plan state.

---

> Last updated (historical): 2026-05-12

## Phase 4 — COMPLETE (Steps 4.1–4.8 all shipped)

All Phase 4 steps committed on `pointsav-monorepo` main branch. Stage 6
promotion pending (outbox message sent to Master). Release binary built.

| Step | State | Commit |
|---|---|---|
| 4.1 — git2 commit-on-edit | ✓ Shipped | `177813e` |
| 4.2 — /history + /blame | ✓ Shipped | `177813e` |
| 4.3 — /diff | ✓ Shipped | `177813e` |
| 4.4 — redb wikilink graph | ✓ Shipped | `177813e` |
| 4.5 — blake3 hashes | ✓ Shipped | `177813e` |
| 4.6 — MCP server (native, no vendor SDK) | ✓ Shipped | `055b2f8e` |
| 4.7 — git smart-HTTP remote | ✓ Shipped | pre-existing |
| 4.8 — OpenAPI 3.1 spec | ✓ Shipped | `c9db78da` |

**Notes on MCP implementation:** `rmcp` vendor SDK rejected per Doctrine claim #54
("We Own It"). Implemented natively in `src/mcp.rs` (~330 lines) using
`axum` + `serde_json`. Transport: HTTP JSON-RPC 2.0 (standard; no stdio/SSE split
needed). Default off behind `--enable-mcp` / `WIKI_ENABLE_MCP`.

## Open: activation defect (now closed)

CLAUDE.md + NEXT.md were missing (noted in registry since 2026-04-28). Added 2026-05-07 — defect closed.

## Open: README.es.md out of sync

`README.es.md` is a 4-file scaffold stub; the English README is 8 KB. Refresh pass needed before next public-facing milestone.

## Closed: site_title + guide_dir_2 config (production)

`local-knowledge-documentation.service` now supports `--site-title` and `--guide-dir-2` (shipped 2026-05-02). Verified 2026-05-14: `WIKI_SITE_TITLE=PointSav Documentation Wiki` and `WIKI_GUIDE_DIR_2=/srv/foundry/customer/woodfine-fleet-deployment` both set in the active unit. `local-knowledge-projects.service` and `local-knowledge-corporate.service` confirmed with correct per-instance titles; neither needs `WIKI_GUIDE_DIR_2`.

## Open: Step 7 collab smoke verification

Manual two-client collab smoke (two editors on the same TOPIC, cursor sync visible) is needed before marking Phase 2 Step 7 fully ratified. See `docs/STEP-7-COLLAB-SMOKE.md`.

## Closed: feeds.rs recursive walk

`collect_recent_items()` already implements a two-level walk (root + one category level)
matching the pattern in `collect_topic_files()`. Subdirectory TOPIC coverage verified by
`feeds_include_subdirectory_topics` test added 2026-05-12. NEXT.md note was stale.

## Phase 5 core — shipped

`src/auth.rs` (428 lines), `src/pending.rs` (505 lines), `src/users.rs` (186 lines) —
cookie sessions, argon2id passwords, edit review queue, accept/reject workflow.
Integration tests added 2026-05-12: `tests/auth_test.rs` (5 tests), `tests/pending_test.rs` (4 tests).

Phase 5.1+ not yet implemented: per-page ACLs (`read:`/`edit:` frontmatter), OIDC SSO,
webhook subscriptions, `asyncapi.yaml` 3.1 spec — gated on BP5.

## Phase 6 Part A — shipped (2026-05-13)

Three items implemented and tested:

1. **`inject_wiki_prefixes` trailing-quote fix** (`src/render.rs`) — `raw_slug` previously
   included the closing `"` of the `href` attribute, causing `is_redlink` to always return
   true and wikilink URLs to contain a trailing `"`. Fixed: `trim_end_matches('"')` + slug
   normalisation (decode `%20`, lowercase, spaces→hyphens).

2. **Slug normalisation fallback** (`src/server.rs`) — when a direct file lookup fails,
   tries the lowercase+hyphenated form and returns HTTP 301 to the canonical URL.
   e.g. `/wiki/Compounding-Substrate` → 301 → `/wiki/compounding-substrate`.

3. **Redirect hatnote** (`src/server.rs`, `static/style.css`) — `redirect_to:` 301 now
   includes `?redirectedfrom=<slug>`; `wiki_page` extracts it and passes to `wiki_chrome`;
   `wiki_chrome` renders `.wiki-redirected-from` hatnote at top of article body.

Tests: 4 new tests in `tests/slug_test.rs` — all pass. Full suite: 67 unit + 70+ integration,
all passing.

## Deferred / operator-gated

- Phase 5.1+ — per-page ACLs, OIDC SSO, webhooks, AsyncAPI 3.1 — gated on BP5 + Stage 6
- Phase 6 Part B — portable DID identity (`did:web:` + WebFinger) — needs BP6 design decision
- Phase 7-9 implementation — each gated on the preceding phase shipping + operator clearance
- Note: `libssl-dev` and `libgit2-dev` confirmed present on VM (Phase 4 release build succeeded)
- **Stage 6 + binary rebuild** — now 10 commits ahead of origin on `main`; requires Master session
  (`~/Foundry/bin/promote.sh` + `cargo build --release` + `sudo systemctl restart` all 3 services)
