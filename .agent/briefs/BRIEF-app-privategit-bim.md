---
artifact: brief
schema: foundry-brief-v1
brief-id: project-bim-app-privategit-bim
title: app-privategit-bim — Carbon Framework Rewrite
status: active
owner: project-bim
created: 2026-06-20
updated: 2026-07-02
---

# Brief — app-privategit-bim Carbon Framework Rewrite

## Context

Clean-sheet rewrite of the BIM Object Library web surface. The old
`app-orchestration-bim` (v0.0.3) is a monolith served on port 9096 via nginx
at `bim.woodfinegroup.com`. The new `app-privategit-bim` (v0.1.0) is a
Rust/axum 0.8 SSR app with IBM Carbon Design System Web Components v2,
CodeMirror 6, HTMX, and a planned MCP server (`mcpkit-axum`).

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

- **Sidebar always open:** `expanded` attribute on `cds-side-nav`; no hamburger toggle.
  Hamburger pattern conflicts with `is-not-child-of-header` persistent rail.
- **Plain HTML table for token categories:** `cds-data-table` renders as inline
  elements before Carbon JS initialises; replaced with `<table class="bim-token-table">`.
- **Carbon Web Components v2:** `carbon.esm.js` + `carbon.min.css` bundled as static assets.
- **BIM Static Dir:** `BIM_STATIC_DIR` env var → `src/assets/` in source tree for preview.

## Decisions open

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

## Carry-forward

- Stage 6 promotion (monorepo cluster → canonical main) → Command Session outbox ✓
- Production deploy (systemd + nginx) → Command Session outbox — **re-escalated
  2026-07-02 as HIGH priority; live site is actively broken, not just pending
  an upgrade** (`command-20260702-escalation-bim-woodfinegroup-com-is-live`)
- `token_count: 0` root-caused 2026-07-02 → escalated to Command HIGH priority
  (`command-20260702-bim-woodfinegroup-com-local-bim-bim-desi`); one-line systemd
  unit env var fix, not a code bug
- **CSS/font/logo assets never committed to git** — only copy is
  `/var/lib/local-bim-orchestration/static/` on this VM; recommend committing before
  it's lost, target `app-privategit-bim/src/assets/` — blocked on the branch rebase below
- **Category content/CSS bridging plan drafted 2026-07-02** (full detail in audit report,
  not yet copied into this file in full) — restore old `CatMeta` intro text + uniclass/
  ifc_hierarchy/property_sets fields, restore chip-row + tabbed detail layout, restore
  `/about` page + sidebar Overview section, restore footer stats/closing line. Blocked on
  rebasing `cluster/project-bim` onto `main` to get `app-privategit-bim` into this working
  tree — operator has not yet confirmed the rebase; ask before running it.
