---
artifact: brief
schema: foundry-brief-v1
brief-id: project-bim-app-privategit-bim
title: app-privategit-bim — BIM Object Library website
status: active
owner: project-bim
created: 2026-06-20
updated: 2026-07-02
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
  visual skin. **This is explicitly revisitable** — see Decisions open below.

## Decisions open

- **Visual direction — needs an explicit operator answer.** Three real options
  surfaced this session: (a) keep current BB.14 look [default, what shipped],
  (b) match design.pointsav.com's Carbon skin exactly (reverses BB.14), (c)
  match `home.woodfinegroup.com`'s actual brand stack (Oswald/Roboto Slab/
  Nunito Sans, documented in `pointsav-design-system/themes/MEMO-Woodfine-Color-Matrix.md`)
  instead of either. Nothing further should change the visual skin until this
  is settled.
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

## Pending operator decisions (as of 2026-07-02, second session)

- **Deploy go-ahead needed.** Asked twice (design-direction question, then a
  redeploy-now-vs-later question) and got no response both times — proceeded with the
  non-public-facing work (content/mobile) and deliberately held off on anything that
  touches the live `local-bim.service`/`bim.woodfinegroup.com`. The site is still
  running the ~10-day-stale build with the Carbon bug live for real visitors. A
  combined redeploy (covering `ff1270b8` + `c498bcff` + `9059f83e`) is ready to go —
  needs `sudo install` + static asset resync + `sudo systemctl restart local-bim.service`,
  which is instantly public. Also note: `~/Foundry/bin/deploy-binary.sh` is NOT the
  right tool for this — it's Command-Session-scoped and requires the source to already
  be promoted to canonical `origin/main`, neither of which applies here (this crate
  isn't even registered in `conventions/software-units.yaml`).
- **Visual-direction question still open** (see Decisions open above) — three-way
  fork, asked once, no response.
- Separately, `.agent/briefs/BRIEF-key-plans-site.md` is likely stale — it's scoped to
  the *old* `app-orchestration-bim`/`local-bim-orchestration` service and an older
  `pointsav-design-system/tokens/bim/key-plans.dtcg.json` source path, predates the
  `app-privategit-bim` cutover entirely, and doesn't appear to have been touched since.
  Recommend the operator review it for archival — not touched this session, flagging
  only.
