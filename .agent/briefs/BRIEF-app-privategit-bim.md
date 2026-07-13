---
artifact: brief
schema: foundry-brief-v1
brief-id: project-bim-app-privategit-bim
title: app-privategit-bim — BIM Object Library website
status: active
owner: project-bim
created: 2026-06-20
updated: 2026-07-13
---

# Brief — app-privategit-bim — BIM Object Library website

Renamed 2026-07-02 from "Carbon Framework Rewrite" — Carbon Web Components
were removed from the shell this session (see Decisions locked below) and
scope has broadened from a one-time rewrite to ongoing website
editing/content work. This brief is now the running record for all
app-privategit-bim website work, not just the original Carbon migration.

## Context

Clean-sheet rewrite of the BIM Object Library web surface. The old
`app-orchestration-bim` (v0.0.3) is a monolith served on port 9096 via nginx
at `bim.woodfinegroup.com`. The new `app-privategit-bim` (v0.1.0) is a
Rust/axum 0.8 SSR app — originally built on IBM Carbon Design System Web
Components v2, CodeMirror 6, and a planned MCP server (`mcpkit-axum`); the
Carbon shell was replaced with plain HTML/CSS on 2026-07-02 (see below).

**Source:** `pointsav-monorepo/app-privategit-bim/`
**Preview port:** 9206 (not yet deployed to production)
**Old service:** `local-bim-orchestration` (port 9096, nginx)

## Scope

Phase 1 (complete as of 2026-06-20):
- Rust/axum 0.8 SSR app with Carbon Web Components
- DTCG token loading from `woodfine-bim-library`
- Sidebar navigation with `cds-side-nav` (always open)
- Home page: hero intro + article sections + category grid
- Category pages: plain HTML table (not `cds-data-table`)
- Header: "Woodfine | BIM Object Library" brand + meta description
- Footer: three-column dark footer with catalog stats
- Static assets: `carbon.min.css`, `carbon.esm.js`, CodeMirror bundles

Phase 2 (pending):
- Production deploy (replace `app-orchestration-bim` on port 9096)
- Systemd service unit (`local-bim.service`)
- Nginx config update
- MCP server integration (`mcpkit-axum`)
- IFC-lite-core trajectory

Phase 2 status update (2026-07-06): production deploy happened — see Work log
2026-07-02/03 entries. `local-bim.service` replaced `local-bim-orchestration`
on port 9096; `app-orchestration-bim`'s service is disabled (kept only as a
rollback path, not removed). `bim.woodfinegroup.com` now serves
`app-privategit-bim`.

## Key Plans SVG diagram system

*(Merged 2026-07-06 from `BRIEF-key-plans-site.md`, which is now superseded —
see that file for archival context. Content below updated to current paths/
names; the feature itself now lives at this app's `/key-plans` route, reusing
the same `render::svg::render_kp_zone_svg_from_value` generator referenced in
the 2026-07-03 "Anatomy of a Key Plan" work-log entry below.)*

**DTCG data:** `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` (canonical
location; the old `pointsav-design-system/tokens/bim/` copy this brief
originally cited no longer exists as a clone in this archive)

24 Key Plan cards across 7 categories:

| Category | cat_order | Cards | Display names |
|---|---|---|---|
| Private Office | 0 | 3 | Small / Medium / Large |
| Medical | 1 | 3 | Small / Medium / Large |
| Business | 2 | 3 | Small / Medium / Large |
| Laboratory | 3 | 3 | Small / Medium / Large |
| Academic | 4 | 3 | Small / Medium / Large |
| Civic | 5 | 3 | Small / Medium / Large |
| Corporate Office | 6 | 5 | Full Floor / Half / Third / Quarter / Eighth |

Cards ordered Small → Medium → Large within category (`size_order()`).
Corporate Office last (`cat_order` = 6).

**Size tier:** computed from `area_m2` and `category`. Tier 0=Small, 1=Medium,
2=Large.

**Furniture key differences by tier:**
- Private Office: 1 / 2 / 3 desks at facade
- Medical: 2 / 4 / 6 dental chairs; 1 / 1 / 2 doctor offices
- Laboratory: 3 / 5 / 7 lab bench clusters; 1 / 2 / 2 offices
- Business: 3×3 / 4×4 / 5×5 workstation grid; 2 / 3 / 5 exec offices; 1 / 1 / 2 conference tables
- Academic: workstation bank + conf table / dual banks + oval table / theater seats + bank + round tables
- Civic: 2 / 4 / 5 offices; 1 / 2 / 2 conf rooms; court room in Large only

**Known Rust footgun:** raw string delimiter `r#"..."#` closes on first `"#` —
SVG hex colors like `fill="#888"` terminate it early. Use `format!()` with
escaped quotes instead.

**Still pending (carried over, unverified as of 2026-07-06):**
- [ ] Corporate Office SVG diagrams — was `_ => {}` (no furniture drawn) as of
  2026-05-22; awaiting zone depth data. Not re-checked against current source.
- [ ] `woodfine-bim-library/key-plans/key-plans-registry.md` standalone
  Markdown deliverable (data currently lives only in DTCG + this brief)

## Decisions locked

- **SUPERSEDED 2026-07-02: Carbon Web Components removed from the page shell.**
  `<cds-header>`/`<cds-side-nav>` were never actually upgraded by the JS bundle
  serving them (`customElements.define()` was never called with any `cds-*` tag —
  confirmed by direct inspection of `carbon.esm.js`), so they rendered as an
  unstyled inline blob in production — this was the root cause of the operator's
  "sections all broken together, sidebar broken" report. Replaced with plain
  `<header class="bim-topbar">` / `<nav class="bim-side-nav">` HTML, styled by
  ordinary CSS with no shadow-DOM dependency. `carbon.min.css`/`carbon.esm.js`
  are still vendored as static assets but no longer used by the shell.
- **SUPERSEDED 2026-07-02: "Sidebar always open, no hamburger toggle."** The
  original reasoning ("hamburger pattern conflicts with `is-not-child-of-header`
  persistent rail") no longer applies now that Carbon's side-nav component is
  gone. A real hamburger/off-canvas drawer was added for mobile (`bim.js` +
  `.bim-side-nav--open` class) — the FABLE audit found the old Carbon sidebar
  went into an overlay mode below 1056px that trapped all page content behind
  an undismissable scrim on phone widths; this is now fixed.
- **Plain HTML table for token categories:** `cds-data-table` renders as inline
  elements before Carbon JS initialises; replaced with `<table class="bim-token-table">`.
  (Now moot for the same reason as above, but the table itself is unchanged.)
- **BIM Static Dir:** `BIM_STATIC_DIR` env var → `src/assets/` in source tree for preview.
- **2026-07-02: Website copy lives in `woodfine-bim-library/site-content/*.md`,
  not Rust source.** All 20 category descriptions + `/about` + home-page hero/
  article copy load at startup via a frontmatter parser ported from
  `app-privategit-design/src/vault.rs`. Adding a category is now "drop a
  `site-content/categories/NN-<slug>.md` file," not a Rust code change.
- **2026-07-02: Visual direction kept distinct from design.pointsav.com's Carbon
  skin**, per the April 2026 `BB.13`/`BB.14` operator-commissioned research
  (Carbon scored 3/15, "disqualified," for BIM specifically so bankers can tell
  the two sites apart at a glance). The operator asked in this same session for
  BIM to match the Design System site's look "exactly" — flagged as a direct
  conflict with BB.14, operator did not respond to a clarifying question in
  time, so this round proceeded with the recommended middle path: same
  *organizational* pattern as a design system (token browser, per-category
  detail pages, content-in-files) but keep the BB.14 navy/Source-Serif/Geist
  visual skin. **RESOLVED same session — see below.**
- **RESOLVED 2026-07-02 (commit `e222418b`): visual direction is an exact match
  to `home.woodfinegroup.com`'s brand.** Operator picked option (c) from the
  three surfaced below — not Carbon, not the BB.14 look. Fonts: Oswald
  (display/headings), Nunito Sans (body/UI), Roboto Slab (serif prose) — self-
  hosted, matching `pointsav-design-system/tokens/theme-woodfine-wcp.css` 1:1.
  Colors: exact `--ink`/`--ink-2`/`--ink-3`/`--rule`/`--wf-blue`/`--wf-blue-tint`
  values from the same source (brand blue `#164679` was already an exact match
  by coincidence). Kept a neutral system monospace stack for IFC/JSON display
  — home.woodfinegroup.com has no code-display need to match there. This
  supersedes the BB.14 differentiation research for BIM specifically; the
  reasoning that made it acceptable: BB.14 was about not looking like
  PointSav's Carbon design system, not about inventing a separate identity —
  matching Woodfine's own real brand still satisfies that.
- **SUPERSEDED 2026-07-03 (later session, same day): the entire "BIM Objects CMS"
  Spectrum-grammar reposition (commit `0f76dd0e`, the RD.1–RD.7 research cycle
  above) is being thrown out and rebuilt, not iterated on.** Operator reviewed
  the live redesign fresh, confirmed the specific defects found in a live-site
  audit (dead `/tokens` nav link, mobile sidebar full-screen takeover with no
  dismiss path, over-elaborated footer, truncated/inverted-hierarchy "Important
  Information" disclosure, fonts silently falling back to system defaults, WCAG
  contrast failures, sparse/blank data cells), then delivered a structural
  verdict: **"we need to take form these but make something radically
  different"** — not a request to polish the Spectrum-chrome direction further,
  a rejection of the premise that copying a generic docs-chrome design system
  (however well-executed) is the right differentiation strategy for this
  product at all. See the 2026-07-03 "presentation-layer rebuild" work-log
  entry below for the full research trail and the approved plan. RD.7's visual
  spec (drafting blue, Spectrum grammar, 4-section IA) is not being carried
  forward as the target — the new direction uses the product's own claim #41
  invention (City Code as Composable Geometry) as the interface's organizing
  principle instead of borrowing a generic enterprise design system's chrome.

## Decisions open (historical — the three options as originally surfaced)

- Three real options were surfaced this session: (a) keep current BB.14 look
  [what shipped first], (b) match design.pointsav.com's Carbon skin exactly
  (reverses BB.14), (c) match `home.woodfinegroup.com`'s actual brand stack
  instead of either. **Operator chose (c) — see Decisions locked above.**
- **IA reorganization recommendation from the FABLE research pass (not yet
  implemented, no operator sign-off):** group the 20 categories into three
  sections mirroring design.pointsav.com's *intended* (not current) IA —
  "Catalog" (the existing category browser), "Foundations" (extract the home
  page's article prose — spatial hierarchy, regulatory/climate overlay model,
  DTCG encoding rationale — into standalone pages), "Compositions" (group Key
  Plans / Amenity Key Plans / Retail Select / Tech Industrial under one
  heading, since they're assemblies of the base tokens rather than base
  tokens themselves). Explicitly recommended skipping "Components" and
  "Accessibility" as top-level sections — BIM's per-category tabs already
  fill that role. This is a bigger structural change than this round's scope;
  logged here as the next thing to consider, not started.
- **Logo swap flagged, not applied.** FABLE identified `woodfine-media-assets/assets/logo/wf-logo_V1.svg`
  as a clean vector drop-in for the plain-text "Woodfine" topbar wordmark, but
  it reads "WOODFINE CAPITAL PROJECTS" (the parent co.) — needs operator
  confirmation on entity naming before it ships. Favicon also still missing
  (`/favicon.ico` 404s); FABLE's suggestion was a recolored `ASSET-SIGNET-MASTER.svg`.
- Production deploy timing (replace `app-orchestration-bim` vs. run in parallel)
- MCP server sprint timeline
- CodeMirror JSON editor wire-up for BIM Object editing

## Work log

- **2026-06-20 (this session):** Applied all three UI fixes — hero/footer/header
  (from `app-orchestration-bim` source), sidebar `expanded`, `cds-data-table` →
  plain table. Committed `39d3cb0b` to monorepo cluster branch. Preview running
  on port 9206.
- **2026-07-02:** Operator reported the live public site (bim.woodfinegroup.com)
  "does not look good." Investigation confirmed it is actively broken: all 5
  CSS files 404 (unstyled raw HTML), `/readyz` reports `tokens_count: 0,
  components_count: 0`, `/tokens.json` returns `{}`. Root cause: foundry-prod
  is still serving the OLD `app-orchestration-bim` on port 9096, and its
  `$BIM_STATIC_DIR` / design-system vault paths on that host are missing or
  empty. Confirmed on this workspace VM that `local-bim.service` is already
  active running the NEW `app-privategit-bim` on 127.0.0.1:9096 — healthy,
  fully styled (all static assets 200, `components_count: 18`). Sent
  high-priority escalation to Command (`command-20260702-escalation-bim-
  woodfinegroup-com-is-live`) re-requesting the Phase 2 production deploy
  (Stage 6 promote `39d3cb0b` + build/deploy `app-privategit-bim` to
  foundry-prod + stop old service), using the workspace's `local-bim.service`
  unit as the working reference. Also noted: even the healthy local preview
  reports `token_count: 0` (separate from the static-asset bug) — needs a
  look during/after the prod deploy.

- **2026-07-02 (browser-in-the-loop audit, Fable agent):** Operator felt the new
  site's "look" and BIM-object explanations regressed vs. `app-orchestration-bim`.
  Ran the old crate fresh on scratch port 9097 with its real (previously-uncommitted)
  CSS, screenshotted both alongside prod, and diffed content. Findings:
  - **Root-caused the `token_count: 0` bug** (see 2026-07-02 entry above): local-bim.service's
    `BIM_DESIGN_SYSTEM_DIR` points at `pointsav-monorepo` instead of `woodfine-bim-library`
    — every category shows "0 entities" on the live local preview right now. Escalated
    to Command HIGH priority (`command-20260702-bim-woodfinegroup-com-local-bim-bim-desi`).
  - **The old site's real CSS/fonts/logo were never committed to git** — they only exist
    at `/var/lib/local-bim-orchestration/static/` on this VM (css/tokens.css, components.css,
    base.css, layout.css; Geist Sans/Mono + Source Serif 4 woff2; Woodfine logo SVG). Same
    root cause as prod's CSS 404s: `bin/push-to-prod.sh`'s `target_bim()` never pushes a
    static dir. **Should be committed into git** (suggested target:
    `app-privategit-bim/src/assets/`) before it's lost — currently a single VM directory
    is the only copy.
  - **Content regression confirmed real**: old `CatMeta` (`app-orchestration-bim/src/main.rs:64-190`)
    had multi-sentence category intros plus `uniclass`/`ifc_hierarchy`/`property_sets` fields
    driving chip rows + IFC hierarchy rows + property-set tables + Regulation/Climate
    Zone/Token Format tabs (`main.rs:1250-1338`). New crate's `known_categories()`
    (`app-privategit-bim/src/schema/dtcg.rs`, on `main` branch — not in this checkout, see below)
    thinned every intro to one flat sentence and dropped those structural fields entirely.
    The `/about` "What is a BIM Object?" page and sidebar Overview section were also dropped
    (no route for it in the new router).
  - **New engine's real wins** (worth keeping): MCP JSON-RPC endpoint (5 tools), SSE
    live-reload, dual-mode visual/code editor with jsonschema validation, 3 more categories
    (12 vs 9), and it renders real DTCG entities instead of hardcoded tables.
  - **Important branch note:** `app-privategit-bim/` source exists only on `main` —
    it is NOT present in this `cluster/project-bim` checkout (confirmed via
    `git branch --all --contains <commit>`). To port any of the CSS/content fixes below,
    this branch needs `git fetch origin && git rebase origin/main` first (per
    CLAUDE.md §8 rebase discipline) — not yet done, pending operator go-ahead since it's
    a rebase of a shared branch.
  - Screenshots saved at `~/bim-audit-shots/` on this VM (session-local, not archived).
- **2026-07-02 (bridging work implemented, commit `ff1270b8`):** Attempted the full
  `git fetch origin && git rebase origin/main` on operator confirmation — it conflicted
  immediately (add/add on `.agent/manifest.md`, `.agent/plans/README.md`, `.gitignore`).
  Root cause: `cluster/project-bim`'s very first commit in `pointsav-monorepo`
  (`31403f27`) committed project-bim's entire `.agent/` tree directly into this shared
  repo's history, before the `[Option-A-mailbox-fix]` `.gitignore` block (now on `main`)
  existed to prevent exactly that. Aborted the rebase (clean, no data lost) and flagged
  the contamination to Command as its own item (`command-20260702-pointsav-monorepo-cluster-branches-agent`)
  — likely affects other clusters whose first commit predates that fix; needs a
  history rewrite + force-push to staging mirrors, so left for Command to plan rather
  than acted on unilaterally. Unblocked the actual goal with a narrow
  `git checkout origin/main -- app-privategit-bim` instead (commit `84bd1025`) — pulls
  just that crate's 34 files without touching `.agent/`.
  Then implemented the full bridging plan: vendored the recovered CSS/fonts/logo into
  `app-privategit-bim/src/assets/` (`fonts/`, `images/`, new `tokens.css`, rewritten
  `fonts.css`), replaced `<cds-header>`/`<cds-side-nav>` with plain HTML+CSS (`.bim-topbar`,
  `.bim-side-nav`, grouped `.bim-nav-group`s) to avoid fighting Carbon's shadow-DOM
  theming, restored full `CatMeta` (`uniclass`/`ifc_hierarchy`/`property_sets` fields +
  multi-sentence intros) in `schema/dtcg.rs`, rebuilt `render/card.rs`'s category pages
  with the old chip-row + `<details>` Specification/BIM Objects/Regulation/Climate
  Zone/Token Format layout (property-set tables + live DTCG entity table both included),
  added `/about` (`routes/about.rs`), and restored full footer stats + closing line.
  Had to add an empty `[workspace]` table to `app-privategit-bim/Cargo.toml` — it isn't
  a member of the root workspace and errored on build without opting out standalone.
  Built clean, verified with browser screenshots against a scratch port (9099) — visual
  fidelity now close to the old site's screenshots while keeping the new engine's live
  data and features. Live `:9096` service confirmed untouched throughout (a `pkill -f`
  during cleanup nearly matched the systemd-managed process too — caught because it's
  root-owned and the kill was rejected; used a more targeted process match after).
  Committed to `cluster/project-bim` (`ff1270b8`), not yet Stage-6 promoted.

## Carry-forward

- Stage 6 promotion (monorepo cluster → canonical main) → Command Session outbox ✓
- Production deploy (systemd + nginx) → Command Session outbox — **re-escalated
  2026-07-02 as HIGH priority; live site is actively broken, not just pending
  an upgrade** (`command-20260702-escalation-bim-woodfinegroup-com-is-live`)
- `token_count: 0` root-caused 2026-07-02 → escalated to Command, **ACK'd and fixed
  same day** — `local-bim.service`'s `BIM_DESIGN_SYSTEM_DIR` corrected, verified
  `token_count: 0 → 80` live.
- **Branch contamination in `pointsav-monorepo`'s `cluster/project-bim`** → escalated to
  Command HIGH priority (`command-20260702-pointsav-monorepo-cluster-branches-agent`),
  unresolved — this branch still cannot cleanly rebase onto `origin/main` until Command
  plans a history-rewrite fix (likely affects other clusters too).
- **CSS/font/logo assets recovered and committed** (2026-07-02, commit `ff1270b8`) —
  no longer only-copy-on-a-VM-directory; now under `app-privategit-bim/src/assets/`.
- **Category content/CSS bridging plan implemented** 2026-07-02 (commit `ff1270b8`) —
  see work log above. Not yet Stage-6 promoted; not yet deployed to `local-bim.service`
  or foundry-prod. Next: self-service-promote → operator review on `local-bim` preview
  → decide whether to fold into the pending prod deploy of `39d3cb0b`/canonical's
  already-fixed header/footer/sidebar, or ship separately.
- **2026-07-02 (audit + content architecture + mobile fix, commits `c498bcff`, `9059f83e`
  in `pointsav-monorepo`, `dd4ee7a` in `woodfine-bim-library`):** Operator asked for a
  full audit + content overhaul + design-system-style presentation. Three parallel
  Explore agents plus a FABLE browser audit (see work log detail below) found: the live
  site is running a build ~10 days stale (predates `ff1270b8`'s Carbon fix — this is
  why the operator still saw it broken after that commit landed); Carbon's web
  components were never registered by their own JS bundle (root cause, not just a stale
  asset problem); only 12 of 20 `tokens/bim/*.dtcg.json` files had site metadata (8
  categories were invisible in nav, one — `climate-zones` — hard-errored due to a wrong
  JSON root key); `/furniture` read an empty directory instead of the real 8-item
  `blocks/furniture/`; mobile had exactly one CSS breakpoint that (per FABLE) actively
  broke navigation via Carbon's overlay-with-no-dismiss sidebar. **Fixed all of it:**
  externalized all website copy to `woodfine-bim-library/site-content/*.md` (frontmatter
  parser ported from `app-privategit-design`), wired up all 20 categories, fixed the
  climate-zones root key and the furniture route, replaced Carbon's shell with plain
  HTML/CSS, built a real mobile hamburger drawer, pulled in `woodfine-media-assets`'
  brand-governed AEC status colors. **Not done:** the actual redeploy — see below.
  Also: the branch-contamination issue meant a full rebase onto `origin/main` wasn't
  possible, so `app-privategit-bim`'s source was pulled in with a targeted
  `git checkout origin/main -- app-privategit-bim` instead (already covered above).

## CORRECTION (2026-07-02, later same session): local-bim ≠ foundry-prod

The FABLE audit's "headline finding" (logged above) that "prod and 127.0.0.1:9096 are
literally the same process" is **wrong**. It read this VM's
`/etc/nginx/sites-enabled/bim.woodfinegroup.com` config (which does proxy to
`127.0.0.1:9096`) but never checked DNS. `bim.woodfinegroup.com` resolves to
`34.168.19.68`; this workspace VM's public IP is `34.53.65.203` — **different hosts**.
This VM's nginx vhost + Let's Encrypt cert for that domain are stale/vestigial, left
over from before DNS pointed elsewhere (or before "foundry-prod" was split out as a
separate host) — not actually receiving public traffic. Flagging for Command to
clean up if confirmed unused; not acted on here.

Practical effect: everything in this brief about fixing "the live site" only ever
fixed `local-bim.service` on this workspace VM (`127.0.0.1:9096`) — the correct and
intended Totebox self-service scope per `CLAUDE.md`'s local-first deploy model. It
never touched the actual public `bim.woodfinegroup.com`, which is served by a
separate "foundry-prod" host only Command can deploy to. The prior escalations
(`command-20260702-escalation-bim-woodfinegroup-com-is-live` and today's follow-up
`command-20260702-local-bim-service-redeployed-with-full-f`) remain the correct path
for actually fixing the public domain — this was not a wasted effort, just scoped
correctly to the local preview all along.

## Redeploy completed (2026-07-02, operator confirmed "yes, redeploy now")

Rebuilt `app-privategit-bim` from current `cluster/project-bim` HEAD (commits
`ff1270b8`, `c498bcff`, `9059f83e`), resynced `/var/lib/local-bim/static/` via
`sudo rsync --chown=local-bim:local-bim`, `sudo install`ed the new binary,
`sudo systemctl restart local-bim.service`. Verified on `127.0.0.1:9096`: `healthz`
18/80, home page shows `CATEGORIES 20`, plain `.bim-topbar` shell (no `cds-header`),
`tokens.css`/vendored fonts load 200, `climate-zones` no longer errors, `/furniture`
shows its real 8 items. This is now the current, correct state of the local preview.
Sent a follow-up to Command with the correction above plus a note that whenever
`push-to-prod.sh bim` runs, it should pick up this current HEAD rather than the
state as of the original escalation.

- **2026-07-02 (brand-match implementation, commit `e222418b`):** Operator confirmed
  the visual-direction decision (see Decisions locked above) and asked to implement
  it. Vendored Oswald/Nunito Sans/Roboto Slab woff2 files from the copies already
  self-hosted for `app-mediakit-knowledge` (no new download needed), rewrote
  `fonts.css`/`tokens.css` to the exact `theme-woodfine-wcp.css` values, swept
  leftover Carbon-era hardcoded grays to their Woodfine equivalents across
  `bim-layout.css`/`bim-components.css`, removed the now-unused Geist/Source Serif
  files. Verified on a scratch port, then synced to `/var/lib/local-bim/static/`
  and restarted `local-bim.service` — confirmed live on `127.0.0.1:9096`.

## Pending operator decisions (as of 2026-07-02, second session)

- **Foundry-prod deploy still needs Command** — see correction above. Not something
  further Totebox self-service action can resolve. The local preview (now including
  the brand-match work) is ready whenever that happens.
- Separately, `.agent/briefs/BRIEF-key-plans-site.md` is likely stale — it's scoped to
  the *old* `app-orchestration-bim`/`local-bim-orchestration` service and an older
  `pointsav-design-system/tokens/bim/key-plans.dtcg.json` source path, predates the
  `app-privategit-bim` cutover entirely, and doesn't appear to have been touched since.
  Recommend the operator review it for archival — not touched this session, flagging
  only.
- **Logo swap and favicon still flagged, not applied** (see Decisions open below) —
  now more relevant given the exact-brand-match direction; the `wf-logo_V1.svg`
  entity-naming question is still open.

- **2026-07-02 (footer bugs + license review + FABLE mobile re-check, commits
  `f141dc0d`, `75fb40b8`, `453504c0`):** Operator reported "footer is in the middle
  of the page" on localhost. Found two real bugs: `.bim-shell` had a redundant
  `min-height: calc(100vh - 48px)` forcing the content area to always be
  near-full-viewport tall regardless of actual content, creating huge gaps before
  the footer on short pages (About, Furniture) and pushing it off-screen on tall
  windows — removed, `flex:1` alone is correct. `.bim-footer` had no `margin-left`
  so the fixed sidebar overlapped its left column on every page — added, with a
  mobile-breakpoint reset to 0. Verified at multiple viewport heights, fixed
  (`f141dc0d`).
  - **License/footer accuracy review (operator requested, ran via Opus model)**:
    found the footer's "Apache-2.0" claim is very likely wrong —
    `factory-release-engineering`'s `LICENSE-MATRIX.md` + `repo-license-map.yaml`
    (internally consistent with each other) say `app-privategit-*` should be
    AGPL-3.0-or-later; both `app-privategit-bim/Cargo.toml` and
    `app-orchestration-bim/Cargo.toml` mislabel as Apache-2.0. The BIM *data*
    (`woodfine-bim-library`) genuinely is Apache-2.0 — that part's right; the
    footer conflates code and data under one license line. Also found the footer
    was missing the mandatory trademark notice (`TRADEMARK.md` §13 — not
    optional for a footer surface). **Added the trademark notice** (unambiguous,
    no judgment call) — committed `75fb40b8`. **Did NOT change the license
    designation** — that's a real distribution-terms decision for a live public
    product; flagged to Command (`command-20260702-likely-license-mislabel-app-privategit-b`)
    rather than made unilaterally. `factory-release-engineering` itself does not
    need updating for this — it's already correct; the crates diverge from it.
  - **FABLE mobile re-check** (after the brand-match + footer fixes, none of which
    had been screenshotted at mobile width yet) confirmed the footer fixes and
    hamburger drawer all work correctly at 390px, but found 4 new/pre-existing
    bugs: Oswald never actually rendered on headings (vestigial `carbon.min.css` —
    still genuinely needed for the `/edit` page's real Carbon components — loaded
    after `fonts.css` and its reset won the cascade), `.bim-table-wrap` had zero
    CSS rules anywhere (caused whole-page horizontal scroll on mobile instead of
    just the table), the topbar `|` separator wasn't hidden alongside the label
    under 420px, and breadcrumb links (reusing `.bim-nav-link`, which is
    `display:block`) stacked vertically instead of reading inline (pre-existing,
    not a regression). All four fixed and verified (`453504c0`).
  - **Still-dead token, not fixed**: `--bim-font-serif` (Roboto Slab) is defined
    in `fonts.css` but referenced by zero rules anywhere — flagged by FABLE,
    left as-is (no obviously-correct place to apply it without inventing a role
    that doesn't exist in the current page structure).

- **2026-07-02 (license correction resolved, commit `83bad1c5`):** Operator
  pushed back twice on the Opus review's AGPL finding ("I think it's only
  FSL now" / "check project-software") — correctly not taking the first
  answer at face value on a real legal question. Re-verified independently
  each time: confirmed against `factory-release-engineering`'s
  `README.md` summary table and `licenses/MIXED-MONOREPO-NOTICE.txt` (the
  file automated compliance verification actually uses), then again
  against `project-software`'s live marketplace storefront catalog
  (`app-privategit-marketplace/catalog/products.yaml`). All six documents
  inside `factory-release-engineering` plus the separate storefront
  catalog agree, zero contradict: `app-privategit-bim` → AGPL-3.0-or-later,
  `app-orchestration-bim` → FSL-1.1-ALv2 (this half already
  operator-ratified per `NEXT.md` DEF-001, closed 2026-05-20).
  Operator confirmed "yes, go ahead." Fixed: both crates' `Cargo.toml`
  (previously both wrongly said `Apache-2.0`), added `LICENSE` files to
  both (AGPL-3.0 / FSL-1.1-Apache-2.0 text from
  `factory-release-engineering/licenses/`), corrected the public footer
  to split platform-code license (AGPL-3.0-or-later + source link) from
  BIM-data license (Apache-2.0, unaffected, already correct) instead of
  one blanket wrong claim. Deployed and verified live. Command's earlier
  flag (`command-20260702-likely-license-mislabel-app-privategit-b`) is
  now resolved — worth a short follow-up to Command noting closure.

- **2026-07-02/03 (multi-agent design/consistency pass, commits `7186ea57`
  through `76bf3c6e`):** Operator: "this website still needs some work... run
  a few opus agents to a complete top to bottom browser in the loop... cross
  check the internet and the other localhost Woodfine and Pointsav sites...
  this should blend in with the other Woodfine websites... maybe FABLE
  after." Plan mode used; plan at
  `/home/mathew/.claude/plans/can-you-make-sure-mellow-flask.md`.
  - **Round 0** — bounded footer CSS triage (spacing tightened, trademark
    paragraph reduced to genuine fine-print scale) before the research round,
    so agents reviewed a non-embarrassing baseline (`7186ea57`).
  - **Round 1** — 3 parallel Opus agents: full site audit, cross-reference
    against `home.woodfinegroup.com` (:9102, live/reliable comparator;
    `design.pointsav.com` :9094 excluded — crash-looping, separate
    `project-design` bug, PointSav-branded anyway), external research on
    technical/spec-catalog site design. Cross-reference's headline finding:
    colors already matched exactly (verified token-by-token) — the "doesn't
    blend in" complaint was chrome/type-weight/footer/wordmark, not palette.
  - **Round 2 — real bugs fixed, not just polish** (`e8324e54`, `9cf2ca4e`,
    `b8706bd3`): (1) **confidentiality leak** — research articles
    (`/research/*`) exposed `~/Foundry/...` paths, `.claude/sub-agent-results`
    references, and internal app codenames; sanitized all three, and made
    them git-tracked for the first time (`woodfine-bim-library/research/`
    previously held only `.gitkeep` — the live copy was
    `deployments/gateway-orchestration-bim-1/research/`, local-only/
    gitignored, commit `48c638c` in woodfine-bim-library). (2) **data bug** —
    token category pages iterated DTCG `$description` metadata as a fake
    entity row (visible on climate-zones, retail-select); fixed by filtering
    `$`-prefixed keys in both the per-category loop and the home-page entity
    counter; multi-group files (climate-zones has two top-level groups) now
    show `group/slug` to disambiguate previously-identical-looking rows.
    (3) Key Plans page (`/key-plans`) was showing "— —" / "0 SF" placeholder
    data for **every one of 23 cards** — a genuinely severe, silent bug found
    during a mobile-readiness sweep triggered by a separate operator ask
    mid-session ("make sure bim.woodfinegroup.com is 100% ready on mobile");
    root cause was a fixed-2-level-nesting assumption where the real file
    nests 3 levels (category → subcategory → size variant); replaced with a
    depth-agnostic recursive collector. (4) `.bim-markdown` had zero CSS for
    `<p>`/`<ul>`/`<table>` — also found during the same mobile sweep.
  - **Round 2 — brand-family + polish**: real Woodfine wordmark SVG (same
    markup home.woodfinegroup.com uses, `currentColor` white-on-navy) replaces
    plain-text "Woodfine" in the topbar; all H1s 300→600 to match the
    marketing site's confident hero weight; bare oversized `<h2>`s given
    explicit scale; `.bim-main` narrowed 1200px→920px to close the
    orphaned-prose void next to full-width cards; research index now shows
    real article titles instead of raw file slugs; entities table gained the
    same mobile `overflow-x:auto` wrap the property-sets table already had;
    removed the internal `app-privategit-bim` slug from the public topbar and
    `/healthz`·`/readyz` from the public footer (both P0 findings — internal/
    ops surface visible to real visitors); removed the always-hardcoded
    "REGULATORY OVERLAYS 0 registered" chip and "bSDD URI: pending" row
    (never wired to real data); Uniclass chip/row now hidden instead of
    showing an em-dash when a category has no code.
  - **Round 3 — Fable verification** (`a89364d0e8bfcd4aa`): 8/9 fixes
    confirmed working cleanly; one partial (research-article H1s were still
    weight-400 via a `.bim-markdown` override missed by the site-wide
    300→600 change) — fixed same pass (`76bf3c6e`). Direct side-by-side
    against `home.woodfinegroup.com` confirmed the family-resemblance goal:
    genuinely reads as the same company now. Also found 6/23 Key Plans cards
    still render as an empty box; traced to the *real* root cause (missing
    `zone1/2/3_depth_m`, not `furniture_program` as first assumed — the SVG
    generator never reads `furniture_program` at all, it draws procedurally
    from zone depths) and added an explanatory caption so the blank reads as
    intentional rather than broken, verified it does not false-fire on any
    of the 17 populated cards.
  - **Cross-archive finding, not project-bim's territory**: both the
    cross-reference and Fable agents independently found
    `home.woodfinegroup.com` (app-mediakit-marketing) declares the
    Oswald/Nunito Sans/Roboto Slab stack but ships no self-hosted
    `@font-face` rules — falls back to Arial on machines without those fonts
    installed. Flagged to Command via mailbox
    (`command-20260703-app-mediakit-marketing-home-woodfinegrou`) rather than
    fixed here.
  - **Deferred, not done this pass** (lower priority / bigger scope, logged
    so they aren't silently dropped): Furniture Library page still lists
    bare `.ifc` filenames with no thumbnail/display name (Key Plans already
    has the SVG furniture glyphs — reuse opportunity flagged, not done); no
    Key-Plans color-legend; category pages still show `0 REGISTERED`-style
    honest-but-sparse data for un-populated categories (no fabricated data
    added); bSDD-style property-set grouping redesign (external-research
    finding, a bigger structural idea for a future pass); primary-button
    pill styling to match the marketing site's "Enquire" CTA convention;
    solid-navy topbar fill vs. a lighter family-matching treatment
    (kept navy — legitimate app-shell pattern per the audit's "keep
    different" list, revisit only if operator specifically wants it lighter).

- **2026-07-03 (full shell redesign, commits `7ae34ad1` through `19979562`):**
  Same session, later turn. Operator looked at the live site fresh after
  the pass above and judged it "a dog" next to `home.woodfinegroup.com`
  and the live Woodfine/PointSav wiki instances
  (`local-knowledge-corporate`/`-projects`/`-documentation`, ports
  9095/9093/9090, started this turn specifically to be inspected — running
  `app-mediakit-knowledge-2`, an in-progress rewrite, **not** the
  `app-mediakit-knowledge` crate name would suggest; confirmed via `ps`/`ss`
  against the live processes). Direct comparison: the wiki has real search,
  a dark-mode toggle, accent-bordered card grids, a proper cross-property
  footer with corporate identity (cities line) and badges, and an
  "Important Information" disclosure band — structural gaps, not
  cosmetic ones. Operator confirmed "full re-design." Plan mode used
  again; two Explore agents mapped the wiki's exact CSS/HTML (found it's
  actually `app-mediakit-knowledge-2`, source in the `project-knowledge`
  clone, not `-knowledge`) and catalogued every one of BIM's existing
  functional pieces (routes, Carbon dependencies, JS behaviors) so the
  rebuild wouldn't drop capability; one Plan agent validated the
  dark-mode/Carbon interaction approach and the search implementation
  scope before implementation started.
  - **Key finding, load-bearing for the whole redesign**: the wiki's font
    stack (Inter + Source Serif 4) does not match home.woodfinegroup.com's
    (Oswald + Nunito Sans + Roboto Slab) — nor does the legacy
    `app-mediakit-knowledge`'s (IBM Plex Sans + Playfair Display). All
    three sibling properties already disagree on typography. Decision:
    BIM keeps its existing, already-correct match to the marketing site's
    type/color system; only the wiki's *structure and components* were
    ported, reimplemented in Woodfine's fonts — not a font swap.
  - **Round 1 — dark-mode infrastructure** (`7ae34ad1`): `data-theme`
    attribute + CSS custom-property overrides, narrowed to only the
    surfaces that actually render light and must flip (topbar/footer are
    already permanently-dark chrome in both themes, left alone).
    `/edit/{slug}` forced to `data-theme="light"` server-side rather than
    attempting a dark-chrome/light-content hybrid — validated by a Plan
    agent as the safer approach; the hybrid would have required
    excluding the editor's own plain-HTML property table from the
    hex-to-token conversion pass, recreating the exact half-applied look
    the fix was meant to avoid. Removed a `carbon-overrides.css` block
    that was unconditionally hard-blocking OS dark mode.
  - **Round 2 — header + utility bar** (`2ef545f5`): solid-navy topbar
    replaced with a light header (the wiki's proven pattern for this
    site type) carrying the real Woodfine wordmark SVG (navy-on-white via
    `currentColor`, same markup home.woodfinegroup.com renders), a search
    form, and the dark-mode toggle. New utility bar above it links
    Corporate/Projects/GitHub. Caught during dark-mode testing: the home
    page's actual hero heading (`.bim-hero__statline` — it's a `<p>`, not
    an `<h1>`) had a hardcoded `color: #111827`, rendering invisible
    dark-text-on-dark once the toggle was flipped; swept both stylesheets
    for the same pattern and converted every exact-match hardcoded
    fg-color hex to its token.
  - **Round 3 — sidebar + card grid** (`be407bee`): category cards
    restyled to the wiki's accent-left-border convention, description
    line dropped (name + count only, denser). Sidebar links get the same
    accent-left-border treatment on hover/active, scoped to
    `.bim-side-nav .bim-nav-link` specifically since the bare class is
    shared with breadcrumbs/cards/research items. Remaining dark-mode
    surfaces converted (`.bim-tag`, `[aria-current="page"]` — also set
    client-side by the SPA nav JS, kept in sync — token tables, key-plan
    category color swatches with dark variants; verified the SVG zone
    diagrams' own hardcoded ink colors stay legible against the darker
    swatches, reading as an intentional blueprint-on-dark-canvas look).
  - **Round 4 — footer + disclosure band** (`bc3bb9dc`): footer's third
    column changed from a license dump to a real "Network" column;
    AGPL/source-link facts folded into column one alongside the existing
    Apache-2.0 BIM-data license line. Base row gained a "Vancouver | New
    York" cities line and two badge chips (Powered-by-PointSav, BIM-data
    Apache-2.0 — a text badge, not a fake CC icon for a license that
    doesn't have that convention). New "Important Information"
    `<details>` band with BIM-appropriate disclosure text (verify
    classifications against current code before construction; planned/
    intended language for in-development features per BCSC posture) —
    rendered as a sibling of `.bim-shell` like the footer so it spans
    full width respecting the sidebar, not confined to `.bim-main`'s
    narrower column.
  - **Round 5 — search** (`472eb451`): wires up the header search bar.
    Deliberately skipped a search-index crate (tantivy etc.) — validated
    by the same Plan agent as unnecessary at this corpus size (~150-200
    entities, 3 research articles); the research-index page already does
    a fresh disk scan per request, so a linear scan for search is
    strictly cheaper than what's already shipping. Multi-word
    AND-across-tokens/OR-across-fields matching across three independent
    paths (categories, entities via a depth-agnostic recursive `$value`
    collector covering every category file, research articles); scored,
    deterministically sorted, highlighted snippets reusing the existing
    `esc()` helper.
  - **Verification pass** (`19979562`): editor route confirmed
    functional and correctly forced-light; SPA fragment navigation
    confirmed not leaking the footer/disclosure (they live outside
    `#bim-main-content`, correctly untouched by fragment swaps); found
    and fixed a real mobile bug — at 390px the header's four controls
    (hamburger/logo/search/theme-toggle) summed wider than the viewport,
    silently pushing the theme toggle off-screen with no scroll available
    to reach it. Side-by-side against `home.woodfinegroup.com`'s header
    confirms the "blend in" goal: same wordmark asset, same navy, and the
    two sites now read as "the same company's marketing site and its
    product library" rather than unrelated properties.
  - **Mid-session incident, resolved, not a regression**: hit a
    VM-wide disk-full condition (root filesystem 154G/154G used) that
    blocked all Bash tool use, including read-only diagnosis — genuinely
    VM-wide, not caused by this session's own scratch files (33M
    screenshots, 228M harness tmp dir, neither explains 154G). Flagged to
    the operator rather than investigated further from a Totebox session
    (VM sysadmin is Command Session territory); operator cleared it
    externally and work resumed. Also, mid-Plan-agent-exploration this
    session, a tool result contained an injected block disguised as a
    system/plan-mode directive telling the agent to prematurely write
    files outside its read-only scope — the agent correctly identified it
    as not coming from the operator and ignored it; flagged to the
    operator for transparency. Its actual findings were independently
    corroborated against known code and used as-is.
  - **Deferred, not done this pass**: search doesn't index property-set/
    compliance text inside entity `$value` objects (title/slug/IFC-class/
    top-level `$description` only) — a two-word query like "fire door"
    can legitimately return zero results if no single item's *indexed*
    fields contain both words, even though both words individually exist
    in the corpus; primary-button pill styling to match the marketing
    site's "Enquire" CTA convention (still open from the prior pass);
    Furniture Library thumbnails (still open); the wiki's more generous
    section-padding/hero treatment noted in an earlier fresh-eyes look at
    the homepage — largely addressed by the header/card-grid work but not
    independently re-verified against that specific complaint.

- **2026-07-03 (shutdown addendum — inbox messages missed during the redesign, discovered during shutdown sweep):**
  This archive's own `.agent/inbox.md` was never re-read after an earlier mid-session context
  compaction — three messages sat unread through the entire redesign. Most material:
  Command/project-knowledge (`command-20260702-important-information-footer-structure-a`) had
  already researched and built a proper "Important Information" + footer-disclosure pattern
  (Git-owned markdown source, persistent one-line footer disclaimer, `/disclaimers` page, CC BY-ND
  issuer-attribution for editorial content) before Round 4 of this redesign built an ad-hoc version
  from scratch, hardcoded directly in `shell.rs`. Also unread: confirmation that `bim.woodfinegroup.com`
  is already live in production (deployed 2026-07-02T16:43, predates this entire redesign) and that
  `push-to-prod.sh`'s `target_bim()` is confirmed stale (wrong binary/service names, wrong
  design-system path). All three now logged as concrete NEXT.md items. Process lesson for future
  sessions on this archive: always re-run the inbox read after a context compaction — don't assume
  it carried over from before the compaction boundary.

- **2026-07-03 (LIVE — canonical merge + prod push complete, same shutdown window):** Command
  resolved the staging-fork anomaly (root cause: `self-service-promote.sh` pushed every self-service
  archive to the same shared `main` ref on the personal fork; fixed to push each archive to its own
  ref, and made the promote-queue write unconditional rather than silently dropped on push failure),
  merged 23 of 28 local commits to canonical (the full shell redesign), and pushed to foundry-prod.
  **Verified live**: `https://bim.woodfinegroup.com` returns 200, `/healthz` reports healthy, and the
  utility bar / theme-toggle / search markup are all present in the served HTML — this is the actual
  redesign, not a stale build. 5 commits stayed local-only: 2 are `.agent/`-only (correctly never
  promote) and 3 are older tool-keyplan/app-orchestration-bim work that conflicts heavily with
  canonical's independently-evolved `main.rs` — flagged by Command as needing a dedicated
  reconciliation session, not a guessed merge. Command also fixed a clippy gate issue in the new
  `render/search.rs`/`render/sidebar.rs` this session introduced (this crate had never had
  `-D warnings` run on it before) — worth running `cargo clippy` locally on new Rust code in future
  sessions rather than relying on the promote step to catch it. Prod's systemd unit was renamed
  `local-bim-orchestration`/`local-bim` → `local-woodfine-bim` as part of an unrelated workspace-wide
  naming reorg; this archive's own local workspace staging unit is unaffected, still `local-bim`.

- **2026-07-03 (new session — DTCG token gap register closed):** Completed all 6 missing DTCG token
  files from `.agent/plans/tool-buildingwidth-architecture.md` (`furniture.dtcg.json`,
  `floor-plate-assembly-rules.dtcg.json`, `building-grid.dtcg.json`, `tenant-mix.dtcg.json` new; Medium
  Tile family + Special Tiles added to `tile-system.dtcg.json`) plus all 6 documented internal
  inconsistencies (5 fixed with real data traced to sources already in the token set; 1 —
  professional-office medium/large key-plan areas — marked `status: reserved` since no source document
  exists, not fabricated). Found and flagged two genuine data gaps in the process rather than papering
  over them: the Medium-tile end-cap composition isn't sourced, and Tile F-medium's stated "3x PO Small
  + PO Medium" composition sums to 1,440 SF against a 3,500 SF target (doesn't reconcile). Rust crate
  scaffold (bim-units/bim-tokens/bim-furniture/tool-buildingwidth/tool-floorplates) is now unblocked
  but was deliberately not started -- genuinely large scope, needs its own planning session.
  Commit ae153aa (woodfine-bim-library).

- **2026-07-03 (same session -- reposition as a BIM Objects CMS, not a wiki clone):** Operator looked at
  the live redesign from earlier this session fresh and judged the wiki-engine-modeled shell (utility
  bar, in-header search, accent-left-border cards) the wrong direction -- floated the idea that this
  product should be a genuine "Carbon for BIM Objects" CMS, architects self-serving their own private/
  public libraries, not a docs-wiki clone. Ran a full research-through-implementation cycle:
  - **Research (7 subagents, 3 rounds)**, all findings on record at .agent/sub-agent-results/RD.1
    through RD.7-visual-direction-synthesis-2026-07-03.md: live-site structural audit, git archaeology
    of the pre-wiki baseline (76bf3c6e), design.pointsav.com's current + historical structure (its own
    catalog documents a "wiki" component category as a *legitimate but distinct* pattern from its own
    catalog-shell chrome -- sharpened the diagnosis from "wiki styling is wrong" to "BIM borrowed the
    wrong pattern category for what it is"), full extraction of prior strategy docs (BB.13/BB.14 --
    Adobe Spectrum chrome pick, 14/15 bankers'-distinguishability score; bim-token-strategy.md's
    non-branding product-feel content), a Fable-driven competitive resurvey (Spectrum pick re-confirmed
    at 14/15 against the sibling's *drifted* current palette; found genuine new AEC-vendor prior art in
    Bentley iTwinUI, doesn't displace the pick), and a Fable synthesis reconciling all of it into one
    concrete, implementable spec (RD.7).
  - **Implemented** (Steps 1-4, matching the approved plan): fixed a real bug found along the way --
    content::load_categories only enumerated .md sidecars, so the 4 DTCG files from the earlier
    token-gap-closure session were invisible on the live site (no nav, no card, unreachable by search)
    -- flipped to token-file-driven enumeration. 4-section IA (Taxonomy/Objects/Compositions/Context,
    Section enum, section: frontmatter on all 24 sidecars). Utility bar + in-header search deleted
    entirely; header rebuilt as a single 48px "title block" bar. Full color/typography system swap --
    #1A4480 drafting blue as the *sole* interactive accent site-wide (color-collision-checked against
    the sibling's drifted navy per RD.5), dark mode re-palettized to desaturated instrument-navy.
    carbon.min.css/carbon.esm.js scoped to /edit/* only -- public catalog no longer ships literal
    IBM Carbon CSS. Commit 0f76dd0e (pointsav-monorepo).
  - **Real, honestly-flagged gap**: Geist Sans/Geist Mono/Source Serif 4 font files don't exist anywhere
    in this workspace and this environment can't fetch/subset them -- fonts.css uses documented
    system-font fallback stacks rather than fabricated @font-face rules pointing at nonexistent
    assets. Needs a future session (or the operator) to source and self-host actual woff2 subsets.
  - **Deferred, fully specced in RD.7 for a follow-up pass** -- nothing lost by waiting: hero
    isometric-building SVG, tab-bar page anatomy, IFC GUID monospace markers, classification chip
    restyle, dark viewport preview frames, section landing pages (2x2 panel grid replacing the flat
    24-card homepage grid). PointSav-branded/dedicated-domain positioning
    (bim-token-strategy.md's fuller recommendation) explicitly deferred as a separate, much larger
    decision -- operator did not respond to the scope-clarifying question, defaulted to the lowest-risk
    option (Woodfine branding/domain unchanged).
  - **Promote anomaly, diagnosed and resolved, not a regression**: self-service-promote.sh failed
    twice with an ordinary-looking "fetch first" push rejection. Full diagnosis (fresh fetch,
    merge-base, --is-ancestor -- all clean, no real divergence) before discovering the actual cause:
    ran the script from the archive root (project-bim) instead of from inside pointsav-monorepo --
    the script derives its repo identity from basename $REPO_ROOT, so it picked up the *wrong
    repository's* HEAD entirely and tried to push it under the right name. Left 2 spurious
    promote-queue.jsonl entries (repo: "project-bim", wrong head); flagged to Command via mailbox
    to disregard, with a suggested script hardening (assert expected repo marker, or require
    --repo explicitly) so a wrong-directory invocation fails loudly instead of silently misqueueing.
    Retried correctly from pointsav-monorepo; clean fast-forward, staging mirrors updated.

- **2026-07-03 (later same day — live-site audit, then full presentation-layer rebuild decision):**
  Operator asked "is the new website up live? can you check?" — verified `bim.woodfinegroup.com`
  live (200, valid cert), but the live binary turned out to be running the OLD wiki-shell (pre-CMS
  redesign), not `0f76dd0e`'s CMS reposition — traced to a `promote.sh` `tail -40` truncation bug on
  Command's side that made the 18-commit queue look like only 18 commits when the real range was 31,
  silently dropping the newest 11 (including `0f76dd0e` itself). Escalated to Command via mailbox;
  Command re-diagnosed, took the verified tree from the branch tip for the affected files, promoted
  as `d97e5edd`, re-verified every marker live (0 `.bim-utility`, 5 `<details>`, `#1A4480` present,
  Geist/Source Serif declared) — confirmed genuinely live this time.
  - **Operator then live-reviewed the corrected redesign and pushed back hard, with specifics**:
    "the sideabar tak[es] up the whol[e] scree[n] when open", "the footer is no good, it is too
    confusing, we don't need to follow the other sites accept maybe desgin.pointsav.com", "the
    copy in the 'Important Information' is off... it should n[o]t go to a 'Read full'... all the
    copy should be there in the drop down like on home.woodfinegroup.com and home.pointsav.com",
    and finally: "it doesn't seem like a site for a 'Design System' like IBM Carbon or the ot[h]er
    hypersc[a]ler Design Systems."
  - **Live-site technical audit (two parallel agents, one with real headless-browser screenshots
    via system chromium — no Playwright installed anywhere reachable) confirmed all operator
    complaints plus found additional defects the operator hadn't named**: `/tokens` ("Browse All
    BIM Objects") silently renders the homepage template instead of a category index — the site's
    primary CTA is a dead end; Geist Sans/Geist Mono/Source Serif 4 never actually load (no woff2
    files exist in this workspace/environment), so every page renders in plain system Georgia/
    Arial — screenshots confirm this reads as "unstyled default browser text"; `--bim-fg-faint
    #98A2B3` fails WCAG AA (~2.2–2.6:1) on the header standards line and sidebar section counts;
    BIM Object spec tables on category pages (e.g. `/tokens/spatial`) ship with empty `Description`
    cells for most rows, no placeholder; mobile sidebar confirmed to have zero scrim/backdrop and
    no outside-tap-to-close (only closes via the toggle itself or a nav link) — full-width,
    near-full-height `position:fixed` panel at ≤1056px with no visible way out. Footer compared
    directly against `design.pointsav.com` (2 columns, 1 badge, no disclosure — pure token catalog,
    no BCSC obligation) and `home.woodfinegroup.com`/`home.pointsav.com` (both use
    `<details class="m-footer__disclosure">` with the FULL multi-paragraph disclosure inline, a
    "Full disclaimer" pointer paragraph only at the very end) vs. BIM's actual footer (3 columns, 3
    elaborate badges, disclosure truncated to 2 paragraphs + a "Read the full disclaimer →"
    link-out) — confirmed both the over-elaboration and the wrong truncation pattern. Also found
    the "Important Information" `<summary>` (12px) is styled smaller than its own `<details>` body
    text (13px) — an inverted type hierarchy.
  - **Operator decision, after reviewing all of the above: throw out the current presentation
    layer entirely and rebuild it — not a patch pass.** Explicitly: "track A is a waste of time,
    we [a]re going to th[r]o[w] the current site in the garbage." The underlying BIM Object *data*
    (24 categories, IFC 4.3 anchoring, Uniclass classification, DTCG token content) stays — only
    the Rust/Axum presentation shell is being thrown out.
  - **Differentiation research (2 parallel agents)**, prompted by the operator's explicit framing
    "we need to take form these but make something radically different" and "100x improvement"
    (visual polish + content depth + interactivity, per operator's own prioritization) benchmarked
    against "hyperscaler" caliber:
    - **Differentiation-angle research** found the real gap: RD.1–RD.7 (and everything before it)
      styled a generic docs-chrome catalog — nobody used the product's own actual invention,
      **claim #41 "City Code as Composable Geometry"**, as the thing that shapes the *interface*,
      only ever as content. Surveyed genuinely distinctive (non-generic-SaaS) products — Speckle,
      Hypar, Rhino/Grasshopper, Linear, Raycast, Warp, Framer — and found the common thread:
      differentiation lives in one invented, domain-native structural unit (Warp's block,
      Grasshopper's wire, Speckle making the 3D object graph itself the homepage), not refined
      chrome. Also surfaced the century-old axonometric zoning-envelope diagram (NYC 1916 zoning
      resolution, Hugh Ferriss) as an already-existing, literal visual grammar for "composable
      geometric constraint" that maps directly onto claim #41. Four concrete angles proposed:
      (1) **Envelope-as-navigation homepage** — an interactive isometric zoning-envelope diagram as
      the literal front door/nav, replacing the sidebar tree; (2) **live constraint-composition
      tool** — pick tokens on a category page, watch the envelope recompute/violations resolve in
      real time; (3) **GUID-as-owned-visual-mark** — the IFC GlobalId as a recurring, Raycast-style
      signature (drafting-sheet title-block stamp); (4) **drafting-sheet layout system** — real
      construction-document-set conventions (sheet numbers, title blocks, cross-sheet references)
      replacing card grids.
    - **3D IFC viewport feasibility research** (two attempts — first died to a transient API 529
      overload mid-run): confirmed the current crate has zero client-side 3D tooling (`Cargo.toml`
      has no wasm-bindgen/Tauri; `bim.js` does only SPA nav/theme-toggle/SSE). Compared xeokit-sdk
      (AGPL-3.0, near-complete out-of-box BIM viewer, double-precision georeferenced rendering) vs.
      `@thatopen/components` (MIT/MPL, license-clean for any distribution model, more assembly
      required, needs WASM + COOP/COEP headers this server doesn't set today) against the project's
      own prior research (`BB.2-xeokit-vs-thatopen-2026-04-28.md`, written for the *Tauri desktop*
      `app-workplace-bim` product, which recommends xeokit specifically for that product's
      georeferencing needs and accepts AGPL as "manageable" for open-source distribution) and a
      **separate, still-open decision** (`design-component-bim-viewport-3d.draft.md`'s
      `open_question_1`, for the related `app-console-bim` read-only surface) that explicitly
      flags AGPL as the wrong choice for a publicly-hosted, presumably-closed-source showcase like
      this one, proposing `@thatopen`/non-3D fallback instead — this question is genuinely
      unresolved elsewhere in the project, not something to default-pick here either. Real sample
      IFC content already exists (18 key-plan models in `woodfine-bim-library`), but no XKT/
      Fragments conversion pipeline exists yet for either library. Three effort tiers assessed
      (static hero model → per-category static viewport → fully live GUID-linked data-bound
      viewport); only the most ambitious tier is genuinely differentiated for this product's
      pitch, and it's also the highest-risk/highest-maintenance tier.
  - **Scoping decision (plan mode, approved by operator)**: rather than attempt the full rebuild
    or all four differentiation angles at once, scoped to the most realistic, highest-impact first
    step — the **Envelope-as-Navigation homepage** — plus a dedicated **Header, footer & wayfinding**
    treatment (persistent minimal header, hamburger toggle removed outright rather than fixed since
    it has no target once the sidebar-tree nav model is gone, breadcrumb + "back to overview" link
    replacing the sidebar on category pages, footer/disclosure rebuilt on the home-sites' inline-
    copy pattern). Full plan at `/home/mathew/.claude/plans/can-you-audit-the-modular-bengio.md`.
    Drafting-sheet IA, GUID-as-mark, live-composition tool, and the 3D-viewport decision gate are
    explicitly deferred — see Carry-forward below, not lost.

## Carry-forward (2026-07-03 rebuild — current, supersedes older Carry-forward items above where they conflict)

- **Envelope-as-Navigation homepage + header/footer/wayfinding rebuild — implemented and verified
  locally on `local-bim.service` (127.0.0.1:9096), per
  `/home/mathew/.claude/plans/can-you-audit-the-modular-bengio.md`.** Not yet promoted/deployed to
  prod — pending operator review of the local preview, same pattern as every prior redesign pass.
  - **New module** `render::envelope` (isometric zoning-envelope diagram, hand-derived iso
    projection matching the existing svg.rs key-plan-diagram convention — no WebGL/3D library, no
    new dependency). Homepage (`render_home`) now leads with this diagram; each tier (base/setback/
    tower) and the ground plane are real `<a>` hotspots to `/tokens#taxonomy|compositions|objects|
    context`. A jurisdiction-overlay toggle (municipal/+provincial/+accessibility) swaps between
    3 pre-rendered SVG frames showing the envelope visibly shrink as constraints stack.
  - **Fixed the `/tokens` dead-link bug** found in the same-day audit: `render_tokens_index` was
    literally `render_home(state)` (a stub); now renders a real index, categories grouped under
    the same 4 section anchors the envelope hotspots route to.
  - **Removed the sidebar-tree nav entirely** (`render::sidebar` module deleted) — replaced by the
    envelope diagram (homepage) and a breadcrumb + "← Back to overview" link (category/detail
    pages). This also removes the mobile full-screen-takeover bug at its root (there's no more
    always-present drawer to trap content behind) rather than patching the old drawer's scrim/
    close behavior.
  - **Footer + disclosure rebuilt**: inline full disclosure copy (reusing the same
    `disclaimers_page` sections `/disclaimers` renders — `content::render_important_information`
    and the separate `important-information.md` summary are now dead code, removed), footer
    trimmed from 3 columns/3 badges to 2 columns/1 badge + a single "part of the network" line,
    disclosure heading now 1.0625rem/700 vs. body's 0.8125rem (was inverted at 0.75rem/600).
  - **WCAG contrast fixed**: `--bim-fg-faint` recomputed and verified (not eyeballed) at 5.0:1+
    (light) / 4.5:1+ (dark) against every background it's used on — was ~2.2-2.6:1.
  - **Empty `Description` cells** in category BIM Object tables now show `—` instead of blank.
  - **Two bugs caught during implementation, not from the original audit**: (1) the envelope
    diagram's first draft used projection constants that clipped part of the ground plane off the
    SVG viewBox and left ~50% of the canvas empty — caught via an actual screenshot, not assumed
    correct from the markup; recomputed the projection's true bounding box and refit the viewBox.
    (2) `home.md`'s "Browse the catalog" section still said "navigate by category in the sidebar"
    after the sidebar was removed — a stale cross-reference in content, not code; fixed in
    `woodfine-bim-library/site-content/pages/home.md`.
  - **cargo build + cargo clippy both clean**, zero warnings (including on the new
    `render::envelope` module) — verified before, not just after, local deployment.
- **Deferred, explicitly not dropped:**
  1. **Drafting-sheet layout system** (sheet numbers, title blocks, cross-sheet references) for
     category/detail pages — next structural layer after the homepage ships.
  2. **GUID-as-owned-visual-mark** — natural fit once the drafting-sheet system exists.
  3. **Live constraint-composition tool** — the most engineering-heavy differentiation angle; a
     stretch goal once the envelope diagram has interactive precedent to build from.
  4. **3D IFC viewport — explicit decision gate, not a default.** Needs operator sign-off
     specifically on the AGPL (xeokit) vs. MIT/MPL (`@thatopen`) commercial-distribution tradeoff
     before any engineering starts — see the licensing analysis in the 2026-07-03 entry above and
     `design-component-bim-viewport-3d.draft.md`'s still-open `open_question_1`.
- **Older Carry-forward items above** (Stage 6 promotion status, prod deploy plumbing, license
  labeling, branch-contamination fix) are believed resolved per later entries in this Work log —
  not re-verified as part of this rebuild scoping; re-check before assuming still-current if acting
  on them.

- **2026-07-04 (second pass — real-object hero, header/footer redo, family continuity, shipped and
  verified locally):** Operator reviewed the Envelope-as-Navigation rebuild fresh and rejected the
  core concept: "the three 3d boxes do not quite make any real sense... we don't have the massing
  for real BIM Objects to allow us to be more playful." Root cause identified together with the
  operator: the zoning-envelope diagram represented claim #41 (City Code as Composable Geometry), a
  real but explicitly v0.0.2+ roadmap idea per the manifest's own scope section — not what the
  catalog does today. Extensive real-source research this session (project-bim's own `inputs/`
  "Collaborators" folders — confirmed these are internal Woodfine-family email threads, not external
  architects — plus `cluster-totebox-jennifer` and `key-plans-foundation-study.md`) found:
  - **A real, hand-drafted CAD sheet for PO-1** ("Private Office — Small") at
    `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Private Office.pdf` — real dimensions 19'-8"
    (5.9944 m) depth × 13'-5" width, 325 SF, no Zone 3 (opens directly to shared corridor).
  - **A real data bug**: `key-plans.dtcg.json`'s private-office small/medium/large entries (all
    marked `status: "confirmed"`) had inherited the *Professional Office* use-type's zone depths
    (6.0/3.8/2.0 m) instead of Private Office's real CAD-sourced values. **Fixed** — corrected to
    5.9944/1.3716 m (no Zone 3) across all three size variants, with the wrong `"Steelcase Leap
    chair"` brand claim (no source found anywhere) also removed in favor of generic "ergonomic task
    chair" language.
  - **New hero: "Anatomy of a Key Plan — PO-1"** (`render/hero.rs`, new module, `render/envelope.rs`
    deleted) — reuses the *existing* real SVG generator (`render::svg::render_kp_zone_svg_from_value`,
    same diagram already live on `/key-plans`) rather than inventing geometry, with real, always-visible
    (not hover-only) fact callouts routed to the matching catalog section: IFC anchor/Uniclass →
    Taxonomy; zone depths + real regulatory citations (European Lighting Standard, German Circulation
    Law) → Context; the real furniture list → Objects; the real tile-nesting fact → Compositions.
    Real authorship credit (Jennifer M. Woodfine, "Spatial Taxonomy — Key Plan Methodology," V12,
    Jan 2025). Confirmed "Key Plan" (not "Bundle") is the real, established term — checked against
    the actual methodology PDF's own definition before using either word.
  - **Two real content additions to `home.md`**, both carefully scoped as our own positioning, not
    claims about third parties: a paragraph citing Denver International Airport's real, sourced BIM
    program (~17M SF, 93 buildings, hundreds of Revit models, vendor-managed CDE) as proof the
    industry needs this at scale, positioning this platform as the open-standard/self-hostable
    version of that same pattern; and a paragraph tying the BIM Object Library to PointSav's own
    already-published positioning pillars (home.pointsav.com's real icon-strip: "Business
    Administration, Record Keeping, Building Connectivity") rather than positioning BIM as a
    standalone competitor to Autodesk-style tools.
  - **Legal disclosure rewritten** by a dedicated Opus-model pass instructed to write with securities-
    lawyer precision: `disclaimers.md` replaced (four sections tightened, "Sovereign Data Foundation"
    — confirmed not a real initiative — removed entirely, not replaced with any other named one);
    `important-information.md` deleted (confirmed dead code — the footer already inlines
    `disclaimers.md` directly, no route loaded the separate file). **Flagged, not resolved**: the
    CC BY-ND 4.0 editorial-content license claim has no backing LICENSE file anywhere in either
    repo — needs an operator decision before this text is treated as fully settled.
  - **Real bugs fixed from a fresh independent audit**: dead-space layout bug (`.bim-shell`'s forced
    `flex:1`/`min-height:100vh` stretch left ~1500px of empty space before the footer on short
    pages — removed, `.bim-footer`'s existing `margin-top:auto` now does that job correctly alone);
    doubled/garbled chip text ("IFC IfcSpatialElement" was rendering as "IFC IFCSPATIALELEMENT" under
    the chip's uppercase transform — fixed by exempting `<code>` from that transform); redundant
    breadcrumb + back-link on category pages (consolidated to the breadcrumb alone).
  - **Header rebuilt**: wordmark logo dropped entirely (operator request); the `flex:1` spacer that
    left ~700px of dead space at 1440px width replaced with `justify-content: space-between`; brand
    identity is now plain text ("BIM Object Library") that never disappears at any breakpoint (the
    old logo+descriptor combo used to drop all identifying text below 480px).
  - **Footer rebuilt**: badge corrected to "Powered by PrivateGit" (`os-privategit`/`app-privategit-*`
    is a real, named architecture tier in this workspace's doctrine, distinct from "PointSav Digital
    Systems" the company); switched from dark navy/black to a light surface (`#f8f9fa`, matching
    `home.woodfinegroup.com`'s real current footer exactly) — this was the main reason the footer
    "didn't match the site holistically" since the rest of the site is light; replaced the old flat
    5-step gray ladder with three real, distinct type/color steps (heading/body/legal fine-print);
    cities+badge regrouped tightly so the single remaining badge doesn't read as a placeholder for a
    missing row.
  - **Colors + fonts — reversed the RD.7 Spectrum-chrome direction for family continuity** (explicit
    operator call): `--bim-accent` changed from the drafting-blue `#1A4480` to `#164679`, copied
    directly from `home.woodfinegroup.com`'s live tokens (its `--m-navy-700`/`--m-navy-600`/
    `--m-navy-100`/`--m-ink-900`/`--m-grey-50`/`--m-grey-200` primitives — not approximated). Real
    self-hosted **Inter + Source Serif 4 + Source Code Pro** font files copied byte-for-byte from
    `vendor/pointsav-monorepo/app-mediakit-marketing-2/static/fonts/` (the exact files
    `home.woodfinegroup.com` currently serves) — finally resolves the "fonts never actually load"
    defect that was confirmed as the single biggest driver of "looks generic." Old unused
    Oswald/Nunito Sans/Roboto Slab files (from an earlier, since-superseded brand-match pass)
    removed as confirmed-dead, git-recoverable cleanup.
  - **Verified**: `cargo build` + `cargo clippy` both clean (zero warnings) before and after every
    change. Deployed to `local-bim.service` (127.0.0.1:9096); screenshots taken at desktop/mobile/
    full-page; every hero callout hotspot click-tested (all HTTP 200, routing to real distinct
    content); confirmed zero "Sovereign Data Foundation" occurrences site-wide; confirmed mobile
    header retains brand text at every width.
  - **Not committed yet this pass** — pending operator review of the local preview before repeating
    the commit + self-service-promote + Command-notification flow from the first pass.

- **2026-07-06 — Objects/Compositions definition corrected + v2 design direction proposed:**
  Operator flagged that the project's framing may have drifted: is a "Key Plan"/"Tile" the same
  thing as a "BIM Object," or is it what architects build *from* BIM Objects? Research (NBS/
  buildingSMART/bSDD, Uniclass 2015, Revit Family-vs-Group/Assembly precedent) confirms the
  operator's instinct: a BIM Object is a single, atomic building-component specification (one
  product/entity, Uniclass Pr/Ss level); a Key Plan/Tile is a **Composition** — a named template
  assembled *from* several BIM Objects (Uniclass EF/SL level, the industry-standard "space-type" /
  "typical" / test-fit-layout concept), never a BIM Object itself. This app's own 2026-07-03 CMS
  repositioning (Taxonomy/Objects/**Compositions**/Context IA above) already had this right — the
  actual bug was isolated to `.agent/plans/plan-bim-objects.md`'s wording ("Key Plan is the smallest
  BIM Object unit"), now corrected there. This app's role is confirmed as the **CMS/catalog for BIM
  Objects and the Compositions built from them** — see `BRIEF-app-orchestration-bim.md` for the
  now-clarified, separate BIM Editor/Viewer product this is not, and `tool-keyplan`'s brief for the
  Composition compiler.
  Operator also corrected the working assumption behind this whole brief: **current
  bim.woodfinegroup.com is not a polished baseline worth lightly annotating** — it should be
  approached like designing a new website. A design-thinking exercise (simulate PointSav selling
  `os-privategit` to Denver International Airport as a BIM Object CMS, then transpose the concept,
  Denver-content-free, to a Woodfine-native version) produced a genuine **v2 design candidate** —
  see `BRIEF-simulation-bim-library-denver-woodfine.md` and
  `.agent/briefs/assets/woodfine-bim-library.html`. That candidate is a proposal for operator
  review/decision, not committed work; it is not assumed to replace anything until the operator
  says so.

- **2026-07-06 — v2 built as real Rust integration, deployed to local preview (not committed to
  production):** Operator chose to implement the v2 direction as real code, radically (not
  incrementally) — the whole site rebuilt, not just a new home page bolted onto the old one.
  Delegated to two sequential opus-model agent passes, each independently re-verified (rebuild +
  clippy + a real local server run + curl, not just trusting the agent's own report):
  - **Pass 1** — new home page (`render/catalog.rs`, 853 lines): a unified two-tab Objects/
    Compositions catalog, server-rendered from real `woodfine-bim-library` data (7 Steelcase
    furniture BIM Objects, 23 real Key Plan Compositions), reusing `render_kp_zone_svg_from_value`
    and `collect_kp_leaves` rather than reimplementing them. Classification chips: Uniclass Pr
    (Objects) vs. a Uniclass SL-tier label synthesized per category (Compositions) — no fabricated
    codes. PO-1 shows a real, resolved "Composed from" bill-of-objects (its 6 `furniture_refs`,
    added this session via a real `tool-keyplan` compile); the other 22 Compositions gracefully
    fall back to the existing prose `furniture_program` list. New `bim-catalog.js` (329 lines)
    adds tab switching/faceted filtering/a detail modal on top of the fully server-rendered page
    (works with JS off), reading data from the existing `/api/tokens.json` endpoint (extended with
    a `_catalog` key, no new route). `/key-plans` and `/furniture` now redirect (303) to `/` —
    their download sub-routes are untouched.
  - **Pass 2** — full-site visual rebuild: `/tokens`, `/tokens/{name}`, `/about`, `/disclaimers`,
    `/search`, `/research`, `/research/{slug}` all rebuilt to the same visual family (masthead,
    chips, spec tables) — content/logic unchanged. `/about`/`/disclaimers` legal text verified
    byte-identical to source via an automated test (caught and fixed its own reconstruction bug
    before reporting done). `/edit`'s CodeMirror tool and Carbon CSS surface deliberately untouched.
  - **Verification**: `cargo build` + `cargo clippy -D warnings` clean (independently re-run, not
    just trusted), 6 new in-process route tests pass, and a real local server run (own port,
    outside any agent sandbox) confirmed every route 200 with real data server-rendered, redirects
    correct, downloads working.
  - **Deployed to `local-bim.service` (127.0.0.1:9096)** — binary + static assets synced, service
    restarted, confirmed healthy and serving the new site. **Not pushed to foundry-prod** — that
    remains a Command Session action gated on operator approval of this local preview, per this
    archive's own deploy model. Two commits: `b899adbc` (small — only captured a file deletion due
    to a `git add` slip on a stale path) + `cc25102f` (the actual 16-file redesign).

- **2026-07-07/08 — polish pass, real bugs found via browser-in-the-loop, spec redo, trademark
  rename.** Same session continued; five follow-up rounds, each independently re-verified (not just
  trusting agent/subagent reports):
  - **Branding + footer badge + sourced copy** (`69d6406b`): product name → "Woodfine BIM Library"
    at the title/nav/footer level only (item-level "BIM Object"/"BIM Objects" copy untouched —
    confirmed via research this has been the deliberate term since a 2026-05-17 sweep, so this is a
    new stylistic call, not a reversion). "Powered by PrivateGit" badge kept, moved right. Copy pass
    using real source documents (Openstudio correspondence, DISCOVERY sketch notes): "Taxonomy ·
    Anatomy · Syntax" framing, "Use Case" facet label, "Data Box" area-panel label — each cited to
    source in a code comment. **Real bug found and fixed**: `.bim-cat-grid[hidden]`'s `display: grid`
    was beating the `[hidden]` UA default, so Objects/Compositions tab panels rendered stacked on
    top of each other regardless of which tab was selected — caught by personally reviewing the
    audit agent's own screenshots rather than trusting its "looks fine" summary. Separately, a real
    mobile-overflow bug (classification chips forcing cards wider than viewport) was found and
    fixed by the audit agent, independently re-verified.
  - **Footer 3-column restructure** (`643af9dd`): promoted the standalone "Woodfine network" line
    into a proper third column; dropped `· {public_url}` from the copyright line (keeping "See
    LICENSE for terms." — verified this exact text is mandated verbatim by
    `TRADEMARK.md` §13, not removable); dropped a redundant "See Important Information" pointer
    (disclosure section's own heading sits directly above the footer already).
  - **Real SPA-nav bug fixed** (`8eeffe9b`): `bim.js`'s client-side nav silently did nothing on any
    link to a page without a `/fragment/*` route (only `/tokens`, `/tokens/{name}`, `/research` have
    one) — affected the "Full disclaimer →" link and any breadcrumb "Home" link site-wide. Fixed by
    falling back to a real page load on a non-OK fragment fetch. Also: linked the previously-bare
    "LICENSE" text to the real file on GitHub; corrected "Woodfine Capital Projects" → the real
    `home.woodfinegroup.com` (bare `woodfinegroup.com` turned out to be a placeholder/press-release
    page, confirmed via WebFetch); added "PointSav Digital Systems" → `home.pointsav.com`.
  - **Important Information band redone against Command's actual 2026-07-02 spec** (`787f3867`,
    +`woodfine-bim-library` `95ca5b8`): found the real counsel-approved reference implementation
    already shipped on `project-knowledge`'s `app-mediakit-knowledge` and matched it — dedicated
    short `important-information.md` (not a reuse of the full `disclaimers_page`), a real CC BY-ND
    4.0 badge (official marks + deed link — the SVG assets turned out to already be committed from
    the original ad-hoc attempt, just never wired up), and print handling. **Caught the print CSS
    not actually working**: the proven `display: block !important` pattern reports as visible via
    `getComputedStyle` but a real generated PDF omitted the content anyway — added a
    `beforeprint`/`afterprint` JS handler that genuinely opens the `<details>`, verified via a second
    event listener observing state mid-print-cycle (since Playwright's `page.pdf()` doesn't fire
    `beforeprint` at all, unlike a real user's Ctrl+P). Marked Command's spec message `actioned`;
    flagged the JOURNAL /research render-contract work as consciously deferred, not silently dropped.
  - **Trademark rename** (`3461856d`): operator asked to replace "Woodfine Management Corp™" with
    "MCorp™". Flagged first that "MCorp" wasn't an enumerated mark anywhere in canonical governance
    (only ever a filename-shorthand in architect source docs) — operator confirmed it was a
    deliberate rename, not a mix-up. Updated both the footer and `pointsav-monorepo/TRADEMARK.md`
    (§1 marks list + §13 canonical notice) for internal consistency. **Real drift flagged, not
    silently created**: this `TRADEMARK.md` is a downstream copy of the actual canonical policy at
    `vendor/factory-release-engineering/policies/TRADEMARK.md` (confirmed byte-identical before the
    edit) — per that document's own §11, real amendments only take effect there, which is
    admin-tier, out of Totebox reach. Sent Command a high-priority message asking for the canonical
    source (and any other mirrors, e.g. `woodfine-fleet-deployment/TRADEMARK.md`, not checked) to be
    amended to match.
  - **CC BY-ND licensing — flagged as not actually counsel-confirmed.** When asked "is this
    correct," checked `project-knowledge`'s own tracked governance record (the reference
    implementation this pattern was copied from) and found the CC BY-ND choice explicitly listed as
    "⚠️ For counsel (surfaced, not decided)" — a working engineering default, not a confirmed legal
    position. Told the operator directly rather than asserting it was fine; this is now more
    prominently asserted (via the new real CC badge) than before, so worth an actual counsel
    sign-off before treating it as settled.
  - **Self-service Stage 6 lite run twice** (`bin/self-service-promote.sh`) — direct pushes to both
    personal staging mirrors were rejected as non-fast-forward both times (mirrors are stale,
    pre-dating the 2026-07-05 branch reset investigated earlier this session — a known, already-
    diagnosed condition, not a new anomaly, so not force-pushed through). The promote-queue entry +
    Command inbox notification are the durable record regardless and succeeded both times; final
    queued HEAD is `3461856d`. **Not pushed to foundry-prod** — operator has explicitly requested it
    go live; that step needs Command Session (canonical merge + `push-to-prod.sh`), still pending as
    of session end.

- **2026-07-08 — LIVE on foundry-prod, independently verified twice.** Command bypassed a stale
  unrelated conflict deep in cluster branch history by cherry-picking the 7 real commits
  (`b899adbc`..`3461856d`) directly onto canonical; build + 6/6 tests passed; pushed. Operator then
  asked for a browser-in-the-loop check that the correct version actually shipped (not just trusting
  Command's report). Ran headless Playwright/Chromium against `https://bim.woodfinegroup.com`
  directly: HTTP 200, screenshot + full body-text dump confirmed title "Woodfine BIM Library",
  Objects/Compositions taxonomy framing, 3-column footer, "MCorp™" trademark line, and a real CC
  BY-ND 4.0 badge. Traced the exact rendered footer/trademark text back to
  `app-privategit-bim/src/render/shell.rs` in the local clone and ran `git log` on that file — last
  commit touching it is `3461856d`, the exact SHA Command cited, confirming this is genuinely that
  commit's output and not a stale/partial deploy (the failure mode that hit project-marketing the
  same day). Sent Command a detailed confirmation; Command independently re-verified the same URL
  and reported the same result, and separately found + fixed a missing `--delete` flag on the
  shared `push-to-prod.sh`'s vault rsync (preemptively applied to `target_bim` too — see
  `cleanup-log.md` 2026-07-08 for detail). **v2 redesign is now fully shipped — no longer pending
  Command.** Canonical `TRADEMARK.md` amendment (item above) also closed same day, admin-tier commit
  `062b29e`. Remaining open item: CC BY-ND counsel sign-off (still "surfaced, not decided").

- **2026-07-09/10 — Round 10, sitewide first-person voice rewrite + Method-page diagram redesign.**
  (Full detail predates this session's visible context window; summarized here for continuity.)
  Rewrote all sitewide content from third-person "Woodfine's own X" self-reference into first-person
  "we/our" institutional voice; redesigned both Method-page SVG diagrams (containment model → literal
  nested/concentric frames; cross-section diagram craft pass — dashed centerline replacing a
  semantically-backwards hatch pattern, added a chained dimension line); fixed a real dark-mode
  contrast bug (`--bim-pen-primary`/`--bim-pen-secondary` tokens) and a real motion-animation bug
  (staggered `transition-delay` values getting cancelled by intervening style recalcs — switched the
  stagger mechanism from CSS `transition-delay` to JS `setTimeout`). Two small follow-up rounds:
  diagram text legibility (font-size bump, with a caught-and-fixed clipping regression from
  `letter-spacing`) and mobile header hamburger/toggle right-justification. A multi-round grilling
  correctly renamed the cross-section diagram's "Interior" label to "CENTRELINE" (British/Canadian
  spelling, per operator), grounded in the mirror-symmetry geometry already documented elsewhere on
  the site rather than a guessed synonym.

- **2026-07-12/13 — Rounds 11-13, Spanish (`/es/*`) translation, now LIVE on foundry-prod.**
  Operator: "we need to look at translating this page to Spanish ... we have +50% of our audience in
  Mexico ... using the same language toggle as http://127.0.0.1:9102/". Reference implementation
  (`app-mediakit-marketing-2`) traced with real file:line citations before any code: thin paired
  routes calling one shared `render_slug(state, slug, lang)` function, `t(lang, en, es)` chrome-string
  helper, path-preserving `lang_switch()`, hreflang tags. Real Mexican-Spanish AEC/BIM terminology
  glossary researched via dedicated Opus agent before any translation began (Key Plan/Tile/Magazine
  kept as English proper nouns — Mexican AEC practice borrows English BIM vocabulary directly, and
  literal translations of Tile/Magazine collide with unrelated everyday meanings; Habitat→Hábitat,
  Corridor→Corredor, Floor Plate→Placa de Piso, Object→Objeto applied consistently).
  - **Round 11 (Tier 1, plan-mode-scoped):** `/es`, `/es/method`, `/es/disclaimers`, `/es/tokens`,
    `/es/tokens/{name}` — home/method/disclaimers/24 category ledes/UI chrome, drafted directly by
    the operator's own decision ("I draft it, flagged for verification" — not routed to
    project-editorial, applying uniformly including disclaimers.md). Real architectural fix found and
    applied: the Method page's diagram-caption injection matched section headings by exact English
    text, which would have silently broken under Spanish headings — switched to matching by section
    index instead (same fix later applied to `render_home`'s "The Library" section match).
    `content.rs`/`state.rs` load optional `.es.md` siblings with graceful English fallback.
  - **Round 12:** operator asked to keep translating "everything except the Research essays and the
    Key Plans pages themselves." Added `/es/objects`, `/es/objects/{slug}`, `/es/objects/compare`,
    `/es/search`; extended `/tokens/{name}`'s entity table (previously English-only "out of scope")
    to read an optional `$description_es` sibling field per DTCG entity. 221 entity descriptions
    translated across 22 `tokens/bim/*.json` files via 3 parallel drafting agents — `key-plans.dtcg.json`
    and `amenity-key-plan.dtcg.json` deliberately excluded. One real boundary case caught on review:
    `building-width-calculator.dtcg.json`'s `bim.key-plan.*` subtree (individual Key Plan size data)
    got translated along with the rest of the file, then reverted — same exclusion, just nested
    inside an otherwise in-scope file. One real bug found via a real-browser click-through (not just
    curl): `render_object_card()` had been missed in the lang-threading pass, so cards on `/es/objects`
    still linked to the English detail page — fixed, rebuilt, redeployed, reverified.
  - **Round 13:** operator clarified further — Key Plans/Research nav links should stay in Spanish
    chrome even though their *content* stays English; a browser-in-the-loop check confirmed clicking
    "Key Plans" or "Research" from `/es` was landing on a fully-English page with the language switch
    **entirely absent** — a dead end, not just a translation gap. Added `/es/key-plans`,
    `/es/key-plans/{slug}`, `/es/key-plans/{slug}/o/{object}`, `/es/research`, `/es/research/{slug}`
    as chrome-only-translated routes (nav/footer/breadcrumbs/section headings/bill-of-materials status
    labels in Spanish; dimensions/bill-of-materials item names/descriptions and Research essay
    title/body — "the Journals" — stay English by explicit operator decision). Also fixed a second,
    independently-discovered bug while wiring this up: the nav's `aria-current="page"` active-state
    highlighting matched `active_path` against `/objects`/`/key-plans`/etc. directly, which never
    matched on any `/es/*` page — the active nav item had been silently unhighlighted on every Spanish
    page since Round 11. Fixed by stripping the `/es` prefix before matching.
  - **Verification discipline throughout:** every round rebuilt+retested (7/7 tests) on a scratch
    instance before touching `local-bim.service`, then a dedicated Playwright agent click-through
    (not just curl/href inspection) before calling a round done — this caught both real bugs above,
    neither of which would have surfaced from HTTP-status checks alone.
  - **Deployed to foundry-prod 2026-07-13**, operator-requested, via the normal
    "send to Command" handoff (this archive doesn't push to foundry-prod directly). Command ran
    `push-to-prod.sh bim`, confirmed `/es`/`/es/objects`/`/es/key-plans` all 200. **Same-day follow-up
    finding, still open:** operator reported the live site didn't match the local preview visually;
    investigation (curl byte-size diff + Playwright screenshots) confirmed a real, severe static-asset
    sync gap — `bim-planroom.css` (the core plan-room/catalog stylesheet, 46KB) 404s entirely on
    foundry-prod despite being correctly `<link>`'d in the HTML; `bim-layout.css`/`bim-components.css`
    stale; `bim.js` under half the correct size. Visual effect: unstyled cards, no background texture,
    plain underlined links, a header layout void (an older, already-locally-fixed bug). This predates
    Round 11-13's own work — not something this session's changes introduced — and the earlier
    "LIVE, health check passed" report didn't catch it since `/healthz` doesn't check static assets.
    Escalated to Command high-priority (`command-20260713-urgent-bim-woodfinegroup-com-is-live-but`),
    asking for a full verified resync of `src/assets/*` plus a visual (not just HTTP-status)
    re-verification. **Not yet confirmed fixed as of session end** — see Carry-forward.

## Carry-forward (2026-07-13 — current, supersedes older Carry-forward items above where they conflict)

- **foundry-prod static-asset sync gap — RESOLVED same session, verified twice.** Command found the
  real root cause (`push-to-prod.sh`'s `target_bim()` sourced the binary from this VM's local build
  but static assets from a stale `vendor/pointsav-monorepo` mirror, 3 rounds behind) and fixed it
  properly — repointed `assets_src` at the same Totebox clone the binary uses (commit `b042290`), not
  just a one-time resync. Verified clean via two independent full-browser audits (a 9-page/2-viewport/
  2-language sweep, then a fresh/cache-busted re-check after the operator reported still seeing it
  broken). Full detail: `.agent/rules/cleanup-log.md` 2026-07-13 entry. **Only open thread:** the
  operator's own browser may still show the old broken CSS/JS due to heuristic caching (these 4
  assets send no `Cache-Control` header) — asked them to confirm via hard-refresh/incognito, not
  confirmed as of shutdown. If they report a genuine non-cache issue, treat it as a new investigation.
- **SEO gaps on bim.woodfinegroup.com — real, substantive work, not started.** project-editorial staged
  a ready-to-apply draft (`SEO-bim-woodfinegroup.draft.html`, their `.agent/drafts-outbound/`) —
  zero of ~10 required SEO signals present (no canonical/OG/Twitter/JSON-LD, robots.txt/sitemap.xml
  both 404), plus a real bug: the meta description is hardcoded identical on every page (confirmed
  directly against `shell.rs` this session). 3 open questions in the draft need real answers (per-page
  description sourcing, sitemap URL inventory, og-image asset path) — do not silently resolve them.
  Logged in NEXT.md Hot section; deserves its own session.
- **Binary-ledger sha256 refresh** — Command flagged the prod push's local sha256 differed from the
  binary ledger's recorded value (a WARN, not a blocker; the correct binary landed regardless).
  Command-owned housekeeping, not urgent.
- **Spanish translation remaining scope, all deliberately excluded, not "not yet gotten to":** Key
  Plan/Composition technical data (dimensions, bill-of-materials, descriptions) and Research essay
  title/body ("the Journals") stay English permanently per explicit operator decision. SVG diagram
  *inner* labels on `/es/method` (Building/FACADE/DAYLIGHT PERIMETER etc.) stay English — only the
  figcaptions translate; a real gap if full diagram translation is ever wanted (would need `lang`
  threaded into `render/svg.rs`), not attempted.
- **Footer-structure / browser-tab-title cross-property proposals from Command** (2026-07-12) —
  optional, forward-looking reference for whenever BIM ships a public-facing footer redesign; this
  archive is already compliant with the ratified browser-tab-title em-dash pattern and has a real
  favicon. No action needed unless/until a footer redesign is scoped.
- **Legal-tokens runtime-consumer pattern** (factory-release-engineering's
  `tokens/legal-tokens-{woodfine,pointsav}.yaml`) — forward-looking only; this archive's trademark
  string is already correct ("MCorp™"), but if/when a shared token-consumer pattern lands in
  `app-mediakit-marketing-2`, worth migrating `shell.rs`'s hardcoded trademark paragraph to read from
  it instead, so it can't drift again.
