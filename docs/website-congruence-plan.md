# Woodfine marketing site — congruence plan (Graphic Designer brief)

> **PIVOTED 2026-05-09 (late session):** active work moved to
> **`clones/project-marketing/docs/header-footer-decomposition.md`**
> — the operator re-scoped to a top-down catalog of `DESIGN-*` and
> `ASSET-*` drafts decomposing the header + footer. Two wireframe
> samples were shipped to `~/sandbox/outputs/`
> (`wireframe-home-header.html` + `wireframe-corporate-header.html`)
> for visual approval before the catalog authors. Phases 4–8 (Newsroom
> server, L.I./BIM tool shells, Spanish translations, the corrective
> 2.5/3.5/5 sub-phases) are parked pending operator review of the
> wireframes and approval to proceed with the catalog.
>
> The original Phase 0–8 plan below is preserved for context. New
> sessions should start with `header-footer-decomposition.md`.

---

> **Status:** approved by operator 2026-05-09; execution in progress in
> the project-marketing cluster session that authored this file. This
> document is the canonical implementation reference — subsequent
> sessions can resume from the phase status table at the bottom.

## Context

The user (Jennifer) opened TASK Claude on `clones/project-marketing/`,
asked for a **graphic-designer-led plan** to make the Woodfine
marketing site congruent across pages. The site currently has three
fully-styled pages (Landing, Disclaimer, Contact us) but lacks the
three top-level destination pages the wireframes call for —
Corporate, Projects, Newsroom. The plan must reconcile:

1. Header + footer **shared shell** across Corporate / Projects /
   Newsroom (same chrome on all three).
2. **New-tab vs same-tab** decision for the Corporate / Projects /
   Newsroom links.
3. Header + footer **parity** between the inner pages
   (Disclaimer, Contact us) and the new top-level pages.
4. **Carve-out** for Location Intelligence and BIM Tokens — the user
   suspects those sub-products need a different shell, not the
   marketing-site chrome.
5. **Woodfine first** — PointSav is parked.

This is a design + IA plan, not a construction order. No file edits
are made until the user approves.

---

## Source-of-truth wireframes (parsed)

**Wireframe sketches V2 — Ian Kiprono.pdf (4 pages, digital):**

| Page | URL pattern | Header order | Body shell |
|---|---|---|---|
| Landing | `www.woodfinegroup.com` | Disclaimer · Contact us · **[Logo centre]** · Corporate · Projects · News | Hero space → Graphics (SVG) → Key highlights |
| Corporate | `www.woodfinegroup.corporate.com` (intent: `corporate.woodfinegroup.com`) | Disclaimer · Contact us · Corporate · Projects/Documentation Wiki · News + **Logo / Menu / Search** strip below | MediaWiki-style: side ToC + Article/Discussion/Read/View History tabs + Languages/Appearance panel |
| Newsroom | `www.woodfinegroup.newsroom.com` (intent: `newsroom.woodfinegroup.com`) | **[Logo left]** · Disclaimer · Contact us · Corporate · Projects/Documentation Wiki · News | Sidebar ToC (Current/Media Release/Regulatory/Archives/Future Designs) + Date/Month filter + News list |
| PointSav Documentation | `www.pointsav.documentation.com` | Same as Corporate, plus **Download** button | Same as Corporate |

**www.woodfinegroup.com hand sketches.pdf (4 pages, paper):**

Same four pages, hand-drawn, with explicit design notes:

- **Theme:** Blue `#0FA17B` (likely transcription error — Woodfine canonical is `#164679`), white Roboto, other-section `#FFFFFF`.
- **Common CSS / defined structure** — the operator already wants a shared shell.
- **Footer:** "Included in other deployments?" / "Footer from homepage?" — operator's instinct is one footer everywhere.
- **Numbering question raised in the sketch itself:** "Maybe we use common order of numbering ie Disclaimer being 7 instead of 16. Move logo to point a instead of point b. Case in-point → Arch Linux homepage." → The operator wants ONE component-numbering scheme reused across all three pages, with the **logo as point #1** (Arch Linux / Wikipedia pattern).
- **Same-domain question:** "Hosted separately?" / "together due to fee?" / **"Same domain"** (crossed out) — operator concluded subdomains, separate hosts.
- **CMS choices vary by page:** WordPress (landing) / MediaWiki (Corporate, Documentation) / FreshRSS → WordPress later (Newsroom).

---

## Current site audit

**Files actually deployed at `/srv/foundry/deployments/media-marketing-landing-1/content/`:**

| File | Role | Header | Footer | Issues |
|---|---|---|---|---|
| `index.html` | Landing (bundled — 180-line bundler shell wrapping a JSON manifest of the rendered DOM) | Cannot inspect without unbundling | — | Bundled artefact — source-of-truth is the manifest inside, not the HTML itself |
| `disclaimer.html` | Inner page — fully responsive, 4 breakpoints | `.topnav` grid: **Disclaimer · Contact us** \| **[wordmark centre]** \| **Corporate · Projects · Newsroom** | `.footer`: *Vancouver \| New York* + Contact us / Disclaimer | Newsroom link points to `https://woodfinegroup.com/` (root) — should be `newsroom.woodfinegroup.com`; Corporate/Projects/Newsroom open with `target="_blank"` (new tab) |
| `contact.html` | Inner page — same shell as disclaimer | Same | Same | Same Newsroom-link bug |

**Existing chrome (from `disclaimer.html` 691–778):**

```
.page (max-width 1440 + paper-2 #F7F9FA)
├── header.topnav  (grid 1fr-auto-1fr)
│   ├── nav.left   → Disclaimer · Contact us  (small caps, ink-3 grey)
│   ├── a.wordmark → SVG logo (institutional-fill #111827)
│   └── nav.right  → Corporate · Projects · Newsroom (wf-blue, target=_blank)
├── div.page-hero  → centred H1 in display font, all-caps
├── main.subpage-main → page content
└── footer.footer  → cities (left) + footnav (right)
```

This shell is solid and matches the wireframes' intent — it just
needs to be applied to **three more pages** (Corporate, Projects,
Newsroom) and the same-tab/new-tab + Newsroom-link decisions need
to land.

---

## Where Location Intelligence + BIM Tokens live

- **Location Intelligence** — referenced in `pointsav-monorepo/USER_GUIDE_2026-03-30_V2.md`; this is the GIS app surface, not a static marketing page.
- **BIM Tokens** — `customer/woodfine-design-bim/tokens/bim/*.dtcg.json` (DTCG design tokens for BIM concepts: assemblies, relationships, systems, performance, identity-codes). This is a design-system data product, not a marketing page.

Both are **operational sub-products** with their own technology
substrate. They should not share marketing-site chrome. (Decision
captured in the Recommendation section.)

---

## Three design decisions confirmed by operator (2026-05-09)

1. **Header layout — logo-centre everywhere.** ✓ Confirmed.
2. **New tab + visible ↗ glyph for Corporate/Projects/Newsroom.** ✓ Confirmed.
3. **Separate shell for L.I. + BIM Tokens, surfaced as product cards under Projects.** ✓ Confirmed in principle, but **deferred** — see "Phase 4 deferred" below. **No dark mode.**

The detailed reasoning behind each is preserved below for the
record. Two further decisions (footer scope, IA numbering map) carry
my recommendation as the working assumption — they are operational
defaults, not open questions, but operator can override at any time.

### Production reality check (2026-05-09)

- `https://projects.woodfinegroup.com/` — **live and functioning**.
- `https://corporate.woodfinegroup.com/` — **live and functioning**.
- `https://newsroom.woodfinegroup.com/` — status unconfirmed; assume **not yet provisioned** until verified.

This collapses Phase 2 substantially: Corporate and Projects do not
need placeholder pages on the landing host. The header links point
straight at the live subdomains. Only **Newsroom** needs a treatment
in this sprint, and only if its subdomain is not yet live.

## Five design decisions needed (and my recommendation)

### Decision 1 — Header pattern: one or two layouts?

The wireframes show two visually distinct top-bars:

- **Landing:** Disclaimer · Contact us · **[Logo centre]** · Corporate · Projects · News  (logo as centre anchor, links flank it)
- **Inner pages (Corporate / Newsroom / Documentation):** **[Logo left]** · Disclaimer · Contact us · Corporate · Projects · News  (logo demoted to left, full menu inline)

Operator's hand-sketch question explicitly asks: **"Move logo to point
a instead of point b. Case in-point → Arch Linux homepage."** This
asks whether the inner-page pattern (logo-left) should be used
everywhere — including the landing page.

**Recommendation:** Use **one header on all five pages** —
**logo-centre, links-flanking**, the pattern already implemented in
`disclaimer.html` / `contact.html`. Reasons:

1. The current built pages already use logo-centre and they look
   institutional and editorial — fits the Bloomberg-article tone
   `CLAUDE.md` mandates.
2. Reusing one shell means one CSS bundle, one breakpoint sheet, one
   place to fix bugs.
3. Logo-centre is a stronger brand statement on a corporate-LP site
   than a left-anchored utility wordmark; the Arch Linux precedent
   the operator cited works for distros, not for capital-projects
   firms.
4. "Common order of numbering" works fine with logo-centre — we
   simply assign Logo = 1 in the unified IA map.

If the operator strongly prefers logo-left on inner pages (the
Wikipedia/Arch convention), a one-line CSS swap can enforce that —
but I do not recommend it.

### Decision 2 — New tab vs same tab for Corporate / Projects / Newsroom?

Currently: `target="_blank" rel="noopener"` (new tab). This makes
sense **only if** those three live on different subdomains and
function as logically separate properties (different CMS, different
auth, different content cadence). Per the wireframes they do exactly
that: Corporate = MediaWiki, Newsroom = FreshRSS, Projects = MediaWiki
documentation. Different backends, different update cadences,
different mental contexts.

**Recommendation:** **Keep `target="_blank"` for Corporate, Projects,
Newsroom.** Reasons:

1. They are subdomain-isolated CMSes, not landing-page sections.
   Tab-spawning preserves the visitor's place on `woodfinegroup.com`.
2. Investor visitors typically want to scan disclosure and
   regulatory content while keeping the marketing context open.
3. New-tab is the standard pattern for "external-but-related"
   properties (Bloomberg Terminal → Bloomberg.com pages do this).
4. **However:** Disclaimer ↔ Contact us links should be **same-tab**
   (they currently are) — those are inner pages of the same property.

Add `aria-label="opens in new tab"` and a small visual indicator
(↗) on the right-side links to set the right expectation.

### Decision 3 — Footer scope: one or two?

The hand sketch asks "Footer from homepage?" (operator's instinct is
yes). The current `disclaimer.html`/`contact.html` footer is *Vancouver
| New York* + nav + copyright — appropriate everywhere.

**Recommendation:** **Same footer on Landing + Disclaimer + Contact +
Corporate + Projects + Newsroom**. The footer is the disclosure
anchor (cities, contact, disclaimer link, copyright, BCSC posture
language) — it should never disappear on an investor-facing page.

### Decision 4 — Carve-out for Location Intelligence + BIM Tokens?

Operator's instinct (correct in my view): **these are sub-products,
not marketing pages.** They have:

- Their own technology substrate (GIS app for L.I.; DTCG token JSON for BIM).
- Their own audiences (technical/design users, not LPs).
- Their own update cadence (engineering velocity, not quarterly disclosure).

**Recommendation:** **Do not put them in the marketing shell at all.**
Instead:

- Surface them under **"Projects"** (subdomain `projects.woodfinegroup.com`) as **product cards** with a **"Launch tool ↗"** button that opens the standalone app/site in a new tab.
- The L.I. and BIM tools then live on their own subdomains
  (e.g. `gis.woodfinegroup.com`, `bim.woodfinegroup.com`) with
  **product-app chrome** — minimal header (back-arrow to
  woodfinegroup.com + tool name + auth controls), no marketing
  footer. They are tools, not pages.
- This matches the wireframes' "Hosted separately?" question.

### Decision 5 — Numbering / IA map (graphic-designer deliverable)

Resolve the wireframe's numbering inconsistency (Landing 1–6,
Corporate 7–19, Newsroom 20–27, with Disclaimer = 1 on Landing but
16 on Corporate). The operator already flagged this in the hand
sketch. Propose a single IA component map:

| # | Component | Lives on |
|---|---|---|
| 1 | Logo / wordmark | All pages (header centre) |
| 2 | Disclaimer link | All pages (header left) |
| 3 | Contact us link | All pages (header left) |
| 4 | Corporate link | All pages (header right, target=_blank) |
| 5 | Projects link | All pages (header right, target=_blank) |
| 6 | Newsroom link | All pages (header right, target=_blank) |
| 7 | Page hero (H1 band) | Inner pages only (Disclaimer, Contact, Corporate, Projects, Newsroom) |
| 8 | Side ToC | Corporate + Newsroom + Projects |
| 9 | Topic / Article / Discussion / Read / View History tabs | Corporate + Projects |
| 10 | Date/Month filter | Newsroom |
| 11 | Languages / Appearance panel | Corporate + Projects |
| 12 | Search | Corporate + Projects |
| 13 | Log in (admin only) | Corporate + Projects |
| 14 | Cities + footnav + copyright (footer) | All pages |

This map is the artefact the next person (or session) builds from.

---

## Recommended approach (the design plan)

### Phase 1 — Shell extraction + design-draft reconciliation (do first)

Two parallel tracks:

**1a. Extract the shell.**
- Pull header / footer / responsive CSS from `disclaimer.html` into
  `clones/project-marketing/templates/_shell-header.html` +
  `_shell-footer.html` + `shell.css`. Source-of-truth lives in the
  cluster, not the deployment.
- Codify the IA component map (table earlier in this plan) at
  `clones/project-marketing/docs/ia-component-map.md`.

**1b. Reconcile yesterday's design drafts (commit `22abc8c`) with
the as-built site.** Audit found four mismatches:

| Draft | Issue | Resolution |
|---|---|---|
| `token-woodfine-brand-color` | none | leave as-is ✓ |
| `token-woodfine-breakpoints` | Drafted Carbon scale 320/672/1056/1312/1584; built uses 1200/1024/768/480 | **Iterate the draft in place** — replace with the built breakpoints; document the divergence-from-Carbon rationale in the `notes_for_designer:` field |
| `token-woodfine-typography` | Drafted system-font stack; built uses Oswald + Roboto Slab + Nunito Sans + Barlow Condensed (Google Fonts brand stack) | **Iterate the draft in place** — replace the font tokens with the actual Google Fonts brand stack and the 3-font system (display / body-slab / UI-sans) |
| `component-marketing-topnav` | Drafted logo-left + hamburger + 3 placeholder links; built is logo-centre + 6 specific links, no hamburger | **Iterate the draft in place** — rewrite the recipe to match the built `.topnav` (logo-centre, 6 links, target=_blank on right-side, ↗ glyph) |

`master_cosign: required` stays on the token drafts. The drafts can
be edited in this cluster freely; promotion-to-canonical at
`pointsav-design-system` still requires Master co-sign — that
happens later via project-design.

**1c. Author the missing drafts** (newly identified in this audit):

| New draft | Type | Captures |
|---|---|---|
| `component-marketing-footer.draft.md` | DESIGN-COMPONENT | The *Vancouver \| New York* + footnav + copyright band currently in `disclaimer.html` lines 773–778 |
| `component-marketing-page-hero.draft.md` | DESIGN-COMPONENT | The centred H1 band on inner pages (`disclaimer.html` lines 717–719) |
| `asset-woodfine-wordmark-svg.draft.md` | ASSET | Extract the Woodfine wordmark SVG (`disclaimer.html` lines 698–708) and propose writing it to `woodfine-media-assets/icons/woodfine-wordmark.svg`. This is the Woodfine-side companion to yesterday's PointSav `asset-favicon-ps-badge-svg-2026-05-08` |

These four reconciliation actions plus three new drafts replace the
old "Phase 1 = pure shell extraction" with a richer Phase 1 that
keeps the design-system substrate honest about what's actually
deployed.

**Out of scope for Phase 1:** building the token→CSS pipeline.
The marketing pages will continue to use inline `:root` CSS custom
properties; the tokens-to-CSS emit is a separate engineering track.
Drafts capture *intent* — the build still uses inline values until
the pipeline is constructed.

### Phase 2 — Disclaimer + Contact link audit

Two small fixes (lowest risk, highest signal):

1. **Newsroom link bug** — change `href="https://woodfinegroup.com/"`
   to `href="https://newsroom.woodfinegroup.com/"` in both
   `disclaimer.html` and `contact.html`.
2. Add a small `↗` glyph after each right-nav link via CSS
   `::after` to signal "opens new tab" without bloating the markup.

### Phase 3 — Token emission pipeline (operator addition 2026-05-09)

**Goal:** stop consuming raw hex/px values inline; have marketing
pages consume tokens from a single emitted CSS file.

**Why now:** Phase 4 (Newsroom server) and any future page added to
the Woodfine surface should consume tokens out of the box. Building
the pipeline before Phase 4 means the new Newsroom HTML is
token-driven from day one, not retrofitted.

**Pipeline shape:**

```
clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/tokens/*.draft.md
    │ (refined by project-design via cluster-design-draft-pipeline.md)
    ▼
pointsav-design-system/tokens/dtcg-bundle.json   ← canonical source
    │ (style-dictionary or equivalent emitter)
    ▼
pointsav-design-system/exports/
    ├── tokens.css           ← :root { --wf-blue-primary: #164679; ... }
    ├── tokens.scss
    ├── tokens.tailwind.js
    └── tokens.figma.json
    │
    ▼ (deployed to landing host alongside content)
deployments/media-marketing-landing-1/content/tokens.css
    │ (consumed by all .html via <link rel="stylesheet" href="/tokens.css">)
```

**Tasks in this phase:**

1. Pick the emitter: recommend **Style Dictionary** (Amazon, the de
   facto standard for DTCG JSON → CSS). Open source, MIT licensed,
   no service dependency.
2. Author a `pointsav-design-system/build/style-dictionary.config.js`
   that consumes the DTCG bundle and emits the four export files.
3. Wire the emit step into the design-system build (an `npm run
   build` or equivalent invocation) so token changes regenerate
   exports atomically.
4. Update the existing marketing HTML (`disclaimer.html`,
   `contact.html`, plus any new Phase-4 pages) to:
   - Drop inline `:root { --wf-blue: #164679; ... }` blocks.
   - Add `<link rel="stylesheet" href="/tokens.css">` in `<head>`.
   - Reference token names (`var(--wf-color-brand-primary)`) instead
     of raw hex.
5. Stage a deployment-side copy step so `tokens.css` lands in
   `media-marketing-landing-1/content/` whenever the design-system
   exports refresh.

**Gating:** the token-source-of-truth lives in
pointsav-design-system. Promotion of the token drafts (commit
`22abc8c` plus any iterations from Phase 1b) into canonical
`dtcg-bundle.json` requires **Master co-sign** per
`master_cosign: required` in the draft frontmatter. Until that
co-sign happens, this phase produces a *staging* `tokens.css`
emitted from the cluster's draft files (not the canonical bundle)
so the marketing pages can move to token-driven without waiting
on co-sign. Swap the source once tokens ratify.

**Effort:** ~4–6 hours for an experienced design-systems
engineer; depends mostly on Master co-sign turnaround.

**Risks:**
- Co-sign turnaround unknown — keep the staging-emit fallback
  ready so this phase doesn't block on it.
- Style Dictionary's default DTCG transform may need a custom
  format for the Carbon-divergence rationale fields (purely
  cosmetic — descriptive text, no impact on emitted CSS).
- Existing inline CSS in `disclaimer.html` / `contact.html` is
  ~500 lines per file; mechanical replacement only.

### Phase 4 — Newsroom server (operator direction 2026-05-09)

**Reference template:**
`/srv/foundry/vendor/pointsav.github.io/index.html` (529 lines).

**Why this template fits:**

The pointsav.github.io page is already a single-document editorial
shell with everything a Newsroom needs:

| Feature | Where in the template |
|---|---|
| `NewsArticle` Schema.org JSON-LD | Lines 18–38 |
| Sticky header bar with backdrop blur | `.header-bar` |
| Wordmark + desktop-actions slot | `.nav-container` |
| Content + sidebar grid (`1fr / 300px`) | `.main-grid` |
| Document canvas with editorial typography | `.document-canvas` |
| Sticky right-rail "ledger" sidebar (mono font) | `.sidebar-ledger` |
| Hook title / executive abstract / takeaways box | dedicated linguistic-token blocks |
| Ledger divider + manuscript title | for press-release sections |
| Legal-ledger footer block | for disclosure lines |
| Light-only colour scheme | `meta name="color-scheme" content="light only"` (matches operator's "no dark mode") |
| Mobile-first responsive | `viewport-fit=cover`, viewport-safe-area envs |
| Self-contained (no external CSS/JS) | one file, drop-in |

The only re-skin work needed is **token swap**:

| PointSav token (template) | Woodfine target |
|---|---|
| `--ps-canvas: #F9FAFB` | `--wf-canvas: #F7F9FA` |
| `--ps-card: #FFFFFF` | unchanged (`--wf-paper: #FFFFFF`) |
| `--ps-text: #09090B` | unchanged or `--wf-ink: #111827` to match marketing pages |
| `--ps-accent: #869FB9` (PointSav slate) | `--wf-blue-primary: #164679` (Woodfine institutional blue) |
| `--ps-muted: #6B7280` | unchanged (`--wf-ink-3: #6B7280`) |
| `--ps-border: #E5E7EB` | unchanged |
| Wordmark text "POINTSAV" | replaced with the Woodfine wordmark SVG (Phase 1c asset) |
| `theme-color: #F9FAFB` | `theme-color: #F7F9FA` |

**Server architecture (operator direction 2026-05-09):**

Two-binary split, both Rust, both sovereign — a leapfrog-2030
RUST equivalent of the FreshRSS + WordPress combo from the wireframe:

```
                    ┌─────────────────────────────────────┐
                    │  External press-release sources     │
                    │  (.md files, partner RSS feeds, …)  │
                    └─────────────┬───────────────────────┘
                                  │
                                  ▼
              ┌──────────────────────────────────────┐
              │  service-rss  (NEW Rust binary)      │
              │  — RUST equivalent of FreshRSS       │
              │  — aggregates inbound feeds          │
              │  — produces canonical RSS/Atom       │
              │  — serves /feed.xml                  │
              └─────────────┬────────────────────────┘
                            │ canonical RSS/Atom
                            ▼
       ┌────────────────────────────────────────────────┐
       │  app-mediakit-marketing  (existing scaffold)   │
       │  — reader: consumes service-rss feed           │
       │  — renders via reskinned pointsav.github.io    │
       │    editorial template + shared topnav/footer   │
       │  — exposes /newsroom and /newsroom/<slug>      │
       └─────────────┬──────────────────────────────────┘
                     │
       ┌─────────────┴──────────────┐
       ▼                            ▼
deployments/                  deployments/
media-marketing-landing-1     media-marketing-landing-2
(Woodfine — newsroom.         (PointSav — newsroom.
 woodfinegroup.com)            pointsav.com or equivalent)
```

**Naming, per operator:**

- **`service-rss`** = the SERVER (RSS aggregator/producer; new
  folder under `pointsav-monorepo/`).
- **`app-mediakit-marketing`** = the READER (existing folder; the
  Newsroom website-as-RSS-reader).

**Folder-creation permissions:**

`pointsav-monorepo/service-rss/` does not yet exist. Per
`CLAUDE.md` §11, creating new top-level folders inside a vendor
canonical repo requires **Master permission**. Phase 4 begins with
an outbox message to Master requesting:

1. Permission to scaffold `pointsav-monorepo/service-rss/` as a new
   Rust crate (Cargo workspace member).
2. Confirmation of the RUST RSS aggregator approach (so Master can
   surface any conflicts with the broader stack).
3. DNS + Doorman provisioning for `newsroom.woodfinegroup.com` and
   the equivalent PointSav newsroom subdomain.

Until Master approves, Phase 4 holds at the template-reskin step
(no new folders created, no commits to pointsav-monorepo).

**Why this split is the right call:**

1. **Source/reader separation.** Aggregator and presenter have
   different release cadences, different operational risks
   (aggregator pulls untrusted RSS; reader doesn't), and benefit
   from different test surfaces.
2. **Multi-tenancy clean.** Two deployments
   (`media-marketing-landing-1` Woodfine + `media-marketing-landing-2`
   PointSav) consume the same `service-rss` feed structure with
   tenant-specific theming via `SERVICE_MARKETING_MODULE_ID`.
3. **Sovereign per leapfrog-2030.** No PHP, no MariaDB, no
   FreshRSS-PHP install — all-Rust, Tier-0 compatible.
4. **Operator-directed.** This is the architecture chosen
   2026-05-09; not my recommendation, the operator's call.

**Tasks:**

**Phase 4a — Outbox to Master (do first, blocks everything else in Phase 4):**

1. Compose an outbox message from `task@project-marketing` to
   `master@claude-code` requesting:
   - Permission to scaffold `pointsav-monorepo/service-rss/` as a
     new Cargo workspace member.
   - Confirmation of the RUST-aggregator + reader split for the
     Newsroom architecture.
   - DNS + Doorman provisioning for `newsroom.woodfinegroup.com`
     (Woodfine deployment 1) and the PointSav newsroom subdomain
     (deployment 2).
2. Hold all subsequent Phase 4 work until Master responds.

**Phase 4b — Template reskin (does not need Master; can run in parallel with 4a):**

3. Reskin `vendor/pointsav.github.io/index.html`:
   - Save as `clones/project-marketing/templates/newsroom-page.html`.
   - Token-swap (PointSav → Woodfine via Phase 3 emitted tokens.css).
   - Replace the wordmark text with the Woodfine wordmark SVG
     (Phase 1c asset).
   - **Replace `.header-bar` with the shared `topnav`** from Phase 1
     (operator confirmed 2026-05-09). Newsroom wears the same
     logo-centre chrome as Disclaimer / Contact / Corporate / Projects.
   - Replace the bottom of `.document-footer` with the shared
     `footer` from Phase 1.
4. Author a tenant-variant template stub for PointSav at
   `clones/project-marketing/templates/newsroom-page-pointsav.html`
   (token-swap only — this is the deployment-2 variant).

**Phase 4c — Server scaffold (gated on Master approval from 4a):**

5. Scaffold `pointsav-monorepo/service-rss/`:
   - Cargo workspace member, axum-based.
   - Inbound: ingests press-release Markdown files from a
     configured directory (per-tenant via env var) and external
     partner RSS feeds (later — not in v0.0.1).
   - Outbound: serves `/feed.xml` (Atom 1.0 / RSS 2.0).
   - Storage: flat-file (no DB in v0.0.1) — Tier-0 compatible.
6. Add Newsroom routes to `pointsav-monorepo/app-mediakit-marketing/`:
   - `/newsroom` — index listing recent press releases (consumed
     from `service-rss /feed.xml`).
   - `/newsroom/<slug>` — single-release page rendered using the
     reskinned template from Phase 4b.
   - Sidebar-ledger surfaces date / source / version metadata.

**Phase 4d — Deployment + content:**

7. Create `deployments/media-marketing-landing-1/content/newsroom/`
   with two seed Markdown press releases (one operational
   "Newsroom launches" release; one disclosure placeholder for
   shape-checking the template).
8. Same for `deployments/media-marketing-landing-2/content/newsroom/`
   (PointSav tenant).
9. Coordinate with Master on subdomain DNS + Doorman routing once
   `service-rss` and the new `app-mediakit-marketing` routes pass
   `cargo check` and basic local smoke tests.

**Effort:** ~12–18 h total — re-skin is ~2 h; service-rss scaffold
is ~6–8 h; app-mediakit-marketing route work is ~4–6 h; deployment
plumbing ~2 h. Master coordination is the critical-path bottleneck.

**Verification:**

- Local smoke: `cargo run -p service-rss` serves `/feed.xml` with
  the seed Markdown; `cargo run -p app-mediakit-marketing` consumes
  it and serves `/newsroom` + `/newsroom/<slug>`.
- Browser smoke: open `http://localhost:<port>/newsroom` in browser,
  confirm shared topnav (logo-centre, 6 links) at top, editorial
  layout below, shared footer at bottom; click a release headline,
  confirm `/newsroom/<slug>` renders the full release with token-
  driven Woodfine colours.
- Production smoke (post-DNS): `https://newsroom.woodfinegroup.com`
  resolves; `target="_blank"` from `disclaimer.html` → Newsroom
  opens cleanly; light-mode only (no dark-mode toggle anywhere).

### Phase 5 — DEFERRED (Location Intelligence + BIM Tokens shells)

**Parked as a future TODO.** Out of scope for this sprint, queued
*after* Phase 4 lands.

When the operator picks this up later:

- The carve-out treatment will be a **light** utility shell, not
  dark mode. Operator preference recorded 2026-05-09: **no dark
  mode anywhere on Woodfine properties.**
- Likely shape: minimal top bar (`← woodfinegroup.com / [Tool name]`),
  same paper background as the marketing site, monospace UI font
  to signal "tool" without changing colour mode, shared
  `--wf-blue-primary` accent.
- Surfacing under `projects.woodfinegroup.com` as product cards
  with "Launch tool ↗" buttons remains the recommended pattern.

### Phase 6 — DEFERRED (Spanish translation)

Per `CLAUDE.md` §6 the investor-facing pages need Spanish
counterparts. Track as a separate task; not in this sprint.

---

## Plan-as-deliverable

This plan file currently lives at
`/home/jennifer/.claude/plans/tender-percolating-raccoon.md` (an
ephemeral location used only while plan mode is active). The
operator has asked that **the plan itself become a markdown file
checked into the cluster**.

**Proposed destination (when plan exits plan-mode and execution begins):**

`clones/project-marketing/docs/website-congruence-plan.md`

That path:

- Sits inside the Totebox archive that owns this work, so a future
  Task session opens it on session-start.
- Lives under `docs/` (the right home for a working-document plan,
  per `CLAUDE.md` §13 root-files-discipline).
- Persists across sessions and survives plan-mode exit.

A single workspace commit (staging-tier via `bin/commit-as-next.sh`)
adds the file alongside any Phase 1 work.

## Critical files to read or modify

- **Read:** `/srv/foundry/deployments/media-marketing-landing-1/content/disclaimer.html` — current shell reference
- **Read:** `/srv/foundry/deployments/media-marketing-landing-1/content/contact.html` — same shell, contact variant
- **Read:** `/home/jennifer/sandbox/inputs/project-marketing/website/Wireframe sketches V2- Ian Kiprono.pdf` — digital wireframes
- **Read:** `/home/jennifer/sandbox/inputs/project-marketing/website/www.woodfinegroup.com hand sketches.pdf` — operator's hand sketches with design questions
- **Modify (Phase 1a):** create `clones/project-marketing/docs/ia-component-map.md`, `clones/project-marketing/templates/_shell-header.html`, `_shell-footer.html`, `shell.css` (new files)
- **Modify (Phase 1b):** edit existing drafts in `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/`:
  - `tokens/token-woodfine-breakpoints.draft.md` (replace Carbon scale → built scale)
  - `tokens/token-woodfine-typography.draft.md` (replace system stack → Google Fonts brand stack)
  - `components/component-marketing-topnav.draft.md` (rewrite recipe to match built logo-centre header)
- **Modify (Phase 1c):** author new drafts:
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-footer.draft.md`
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-page-hero.draft.md`
  - `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/assets/asset-woodfine-wordmark-svg.draft.md`
  - Update `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/INDEX.md` to add the new entries
- **Modify (Phase 2):** two single-line edits to `disclaimer.html` + `contact.html` (Newsroom href fix + `↗` glyph CSS).
- **Modify (Phase 3 — token emission):**
  - New: `pointsav-design-system/build/style-dictionary.config.js`
  - New: `pointsav-design-system/exports/tokens.css` (emitted)
  - New: `deployments/media-marketing-landing-1/content/tokens.css` (deployed copy)
  - Edit: `deployments/media-marketing-landing-1/content/disclaimer.html` and `contact.html` — drop inline `:root` block, add `<link rel="stylesheet" href="/tokens.css">`, replace hex with `var(--…)` references.
- **Modify (Phase 4 — Newsroom server):**
  - Read template: `/srv/foundry/vendor/pointsav.github.io/index.html`
  - New: `clones/project-marketing/templates/newsroom-page.html` (re-skinned)
  - Edit (in `pointsav-monorepo/app-mediakit-marketing/`): add `/newsroom` and `/newsroom/<slug>` routes, Markdown rendering pipeline.
  - New: `deployments/media-marketing-landing-1/content/newsroom/` directory with seed `*.md` files.
  - Coordination: outbox to Master/ops to provision `newsroom.woodfinegroup.com` DNS + Doorman routing.
- **Phase 5:** deferred — no files to modify this sprint.
- **Phase 6:** deferred — no files to modify this sprint.

---

## Verification

For each phase:

- **Phase 1:** open `disclaimer.html` and the new template fragment side-by-side; the rendered header + footer must be byte-identical.
- **Phase 2:** open all five inner pages at viewport widths 1440 / 1200 / 1024 / 768 / 480 px; header position, link order, wordmark size, and footer layout must be visually identical at every breakpoint. Verify all six top-nav links resolve (200 OK on placeholder phase, then to subdomains in production).
- **Phase 3:** click every Newsroom link from disclaimer.html and contact.html; it must open `newsroom.woodfinegroup.com` in a new tab (confirmed by the new ↗ glyph being visible).
- **Phase 4:** a product card on `projects.html` clicked launches a new tab with the tool-shell chrome (no marketing header/footer); the back-arrow returns to `woodfinegroup.com`.
- **Phase 5:** every English page has a Spanish sibling at `/es/<slug>` with the same shell.

End-to-end: visitor lands on `woodfinegroup.com`, clicks Corporate
in header → new tab opens with same logo-centre header on the
Corporate destination, same footer, same disclaimer behaviour. They
return to the original tab and click Projects → see L.I. and BIM
product cards → click "Launch tool ↗" → arrives in tool-shell with
back-arrow to woodfinegroup.com.

---

## Out of scope (explicit)

- PointSav site (parked per user direction).
- Building MediaWiki / FreshRSS instances (subdomain provisioning is
  Master + ops scope; this plan unblocks the marketing-site front
  end so those backends can be plumbed in later).
- Token-driven CSS pipeline (the existing inline `:root` CSS
  variables are fine for now; design-system tokenisation is a
  parallel track captured at commit `22abc8c`).
- Spanish translation (Phase 5 — surfaced but explicitly deferred).

---

## Sprint shape (revised 2026-05-09 with operator additions)

| # | Phase | Status | Effort | Blockers |
|---|---|---|---|---|
| 1a | Shell extraction → cluster templates + IA map | this sprint | ~2 h | none |
| 1b | Reconcile yesterday's drafts (breakpoints, typography, topnav) with as-built | this sprint | ~3 h | none (drafts can be edited freely) |
| 1c | Author missing drafts (footer component, page-hero component, Woodfine wordmark ASSET) | this sprint | ~2 h | none |
| 2 | Newsroom-link fix + `↗` glyph in Disclaimer/Contact | this sprint | ~30 min | none |
| 3 | Token emission pipeline (Style Dictionary → tokens.css → consumed by HTML) | this sprint | ~4–6 h | Master co-sign on tokens for canonical emit; staging-emit fallback unblocks until co-sign lands |
| 4 | Newsroom: **service-rss** (NEW Rust aggregator) + **app-mediakit-marketing** (reader; reskinned pointsav.github.io editorial template + shared topnav). Two-tenant deployment (Woodfine + PointSav). | this sprint | ~12–18 h | Master permission to scaffold `service-rss/`; DNS + Doorman provisioning for two newsroom subdomains |
| 5 | Location Intelligence + BIM tool shells (light, no dark mode) | DEFERRED | — | parked as TODO |
| 6 | Spanish translations | DEFERRED | — | parked as TODO |

**Plan-as-deliverable:** commit this plan to
`clones/project-marketing/docs/website-congruence-plan.md` as part
of Phase 1 so it persists past plan mode and acts as the
implementation reference for subsequent sessions.

I will not start work until the operator approves this plan.
