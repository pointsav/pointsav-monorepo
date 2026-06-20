---
artifact: brief
schema: foundry-brief-v1
brief-id: project-marketing-platform-master
title: "Marketing Platform — Master Brief (app-mediakit-marketing + app-mediakit-shell)"
status: archived
owner: project-marketing
created: 2026-06-13
updated: 2026-06-20
---

## Context

`project-marketing` owns the marketing platform engine behind
`home.woodfinegroup.com` and `home.pointsav.com`. This is the archive's first
legitimate core-mission BRIEF (the 2026-06-12 audit,
`BRIEF-brief-audit-2026-06.md`, found zero and recommended creating one).

A clean-sheet rewrite of `app-mediakit-marketing` was commissioned 2026-06-13.
Two reframings, established with the operator during planning, drive the design.

**1. Agent-first backends.** Web research (2025–2026) confirms the industry is
moving to *Agent Experience (AX)* — backends whose primary authoring path is a
typed, machine-readable contract (API/MCP + schema), with humans as
**approvers**, not hand-editors. No production system runs "100% AI, zero
humans," and our doctrine forbids it (SYS-ADR-10 F12 human-commit; SYS-ADR-19 no
automated AI publishing). Operating model: **AI composes → human reviews diff →
human approves (F12).** This favors a structured content model over a visual
page-builder, and makes the design system the **constrained vocabulary the AI
assembles from** (AI selects components + binds data; it never writes CSS).

**2. OS vs chassis split.** `os-mediakit` is the bootable OS binary that launches
app instances (mirrors `os-console`). The shared header/footer/component
**chassis** is a separate `app-mediakit-*` crate — confirmed against the console
family, where `app-console-keys` *is* the chassis (`Cartridge` trait +
`AppConsoleKeys` registry + all shared chrome) and `os-console` merely
instantiates and runs it.

The prior state being replaced: a thin static-file binary serving 1.2–1.3 MB
single-file HTML monoliths in `deployments/media-marketing-landing-{1,2}/`, via a
fragile client-side "bundler/template" DOM-swap that caused the iOS Safari
viewport bug the `scripts/fix-viewport.sh` / `apply-mobile-fixes.sh` scripts
patch. Content was hand-edited in place; git did not drive the sites.

## Scope

In scope: `app-mediakit-marketing` (the app) and `app-mediakit-shell` (the shared
chassis). Out of scope this round: `os-mediakit` (the OS binary), and adoption of
the shell by `app-mediakit-knowledge` / `-distributions` (cross-archive, deferred
to P6). `software.pointsav.com` is a different stack (`app-privategit-*`) and is
not project-marketing's concern.

## Decisions locked

- **L1 — Content model: typed section-manifest.** A page is a Git-tracked YAML
  manifest (`content/<slug>/page.yaml`): metadata + an ordered `sections:` list
  of typed sections. The schema *is* the AI contract — a manifest either
  deserializes into the typed `Section` vocabulary or it is rejected. Markdown
  only inside `prose` sections. No visual page-builder is built (agents don't
  need drag-drop).
- **L2 — Chassis crate: `app-mediakit-shell`.** Function-named (web "app shell"),
  family-level, framework-agnostic (no axum dep). Owns chrome (maud
  header/footer ported from `templates/_shell-*.html`), the typed `Section`
  vocabulary, and DTCG token loading. `os-mediakit` stays the bootable OS.
- **L3 — Human surface: git-diff + approval queue.** Reuse the
  `app-mediakit-knowledge` pending-edit pattern. AI proposes → staged to a review
  queue → human approves (F12) → persisted. No automated publish (SYS-ADR-19).
- **L4 — MCP-first, day one.** Native JSON-RPC 2.0 (no vendor SDK), as in the
  knowledge engine. Tools expose the section vocabulary, read/validate/propose
  operations; proposals never auto-commit.
- **L5 — Design system as the constrained vocabulary.** Section components own
  their responsive CSS in `app-mediakit-shell/static/sections.css`, referencing
  only DTCG tokens. Content carries zero CSS. This is *how the platform absorbs
  CSS* and why an AI author cannot produce off-brand or broken-responsive output
  (HubSpot 2025: AI CSS is reliable only when constrained to production
  components).
- **L6 — Rust ownership.** Fully server-rendered (axum 0.8 + maud); the
  client-side bundler/DOM-swap and ad-hoc shell-script patching are eliminated.
  maud (not a third-party template engine) is the house pattern.
- **L7 — Per-instance config retained.** One binary, per-tenant env/args
  (`SERVICE_MARKETING_*`), matching the current two-systemd-unit model, so
  `os-mediakit` can later launch instances unchanged.

## Architecture (as built, P1)

- `app-mediakit-shell` — lib crate. `section.rs` (Section trait + `hero`/`prose`/
  `cta` types + `section_catalog()`), `page.rs` (Page manifest + structural
  validation), `shell.rs` (`Brand` {woodfine, pointsav} + `render_page`),
  `tokens.rs` (DTCG loader; built-in fallback overridable by the design-system
  bundle), `render.rs` (comrak prose). 8 unit tests.
- `app-mediakit-marketing` — axum binary. `content.rs` (load/validate
  manifests), `pending.rs` (file-based approval queue; stage→list→approve),
  `mcp.rs` (`list_section_types`, `read_page`, `validate_manifest`,
  `propose_page`, `list_pending`), `server.rs` (routes), `main.rs` (serve
  subcommand). Both added to workspace `members` + speed-optimised release
  profile.

## Agent-first research (citations for the AX thesis)

- Netlify (Matt Biilmann) coined "Agent Experience (AX)" as a discipline,
  Jan 2025; "AX is the holistic experience AI agents have as the user of a
  product." MCP SDK downloads ~2M (Nov 2024) → ~97M (2026).
- Shipped CMS MCP servers: Sanity (40+ tools; stages every agent change as a
  draft), Contentful, Storyblok, Directus, Hygraph. All retain a human approval
  gate; none run autonomous publish. Oracle ships a "Human Approval Node."
- Structured-output reliability tiers: prompt-only 80–95%; function-calling
  95–99%; constrained decoding (schema-valid by construction) 100%. Typed,
  schema-validated content beats free-form for AI reliability (Storyblok 2026).
- Figma 2025: design systems must become "active carriers of craft" AI applies;
  Figma MCP exposes tokens/components so AI generates *from* the system.
- HubSpot 2025: AI-generated CSS is reliable only when constrained to production
  components; free CSS yields hardcoded breakpoints + missing media queries.
- llms.txt adoption ~10% of domains (2026) — direction is clear, not yet default.

Honest bottom line: agent-first is real and shipping, but the 2026 operating
standard is "AI drafts, human approves" — which is exactly L3/L4 and our ADRs.

## Prior work folded in (cross-references, not duplicated)

- Hyperscaler mobile patterns (Apollo/Brookfield/Carlyle/Blackstone/Prologis):
  memory `project_hyperscaler_mobile_research.md` — hero photography of real
  assets, persistent enquire/click-to-call, ≥14px nav, shallow hamburger nav,
  full-width CTAs, single-column stacking, body ≥16px. Encoded as section
  requirements (P2) and partly in the P1 section CSS.
- Leapfrog-2030 CSS/WCAG 2.2 audit artifacts (commit `85099ed`) relayed to
  project-design; the responsive shell CSS already reflects them.
- Knowledge-platform decisions (L1–L29, `archive/BRIEF-knowledge-platform-master.md`):
  Git-native flat-file store, typeface roster, brand token triad (navy #164679 /
  bg #F7F9FA, WCAG AA verified), native MCP JSON-RPC. Marketing follows the same
  substrate patterns.

## Phased roadmap

- **P1 — scaffold (done 2026-06-13):** two crates, minimal real section set,
  MCP + approval queue, workspace wiring, registry. Verified: 11 tests, clippy
  clean, live render (no `__bundler/template`), MCP propose→approve round-trip.
- **P2 — section library:** full typed catalogue (card-grid, feature, media,
  gallery, …) + JSON Schema set; fold in hyperscaler/Leapfrog patterns as section
  requirements; ES bilingual content support.
- **P3 — review surface:** true unified-diff view (similar crate) + richer
  pending queue; git commit-on-approve (git2), mirroring the knowledge engine.
- **P4 — content migration:** re-express the live home.* pages (copy + assets)
  as section manifests.
- **P5 — deployment cut-over:** point the live systemd units at the new binary;
  retire the bundler/template monoliths and the mobile-fix scripts. Run the
  Playwright mobile diag (`scripts/iphone-emulation-diag.py`) against the new
  binary as the acceptance gate.
- **P6 — cross-archive adoption:** handoff to project-knowledge /
  project-distributions to adopt `app-mediakit-shell` (seamless shared chrome).
- **P7 — os-mediakit:** instance-launch integration.

## Open decisions / carry-forward

- [ ] **manifest.md contamination** — `.agent/manifest.md` declares
  `cluster: project-infrastructure`; the project-marketing mission/tetrad is not
  recorded. Needs a corrected manifest; surfaced to Command (cannot verify the
  correct tetrad legs from this session alone). NEXT.md is project-orgcharts
  content — same contamination.
- [ ] **BRIEF contamination cleanup** — tracked by `BRIEF-brief-audit-2026-06.md`.
  Several cross-archive strays remain at `briefs/` root
  (`BRIEF-crypto-license-payment-architecture`, `-ostotebox-phase1-deployment`,
  `-software-distribution-substrate`, `-substrate-phd-thesis-2026-05-27`,
  `-totebox-transformation`); these belong to project-software / project-data /
  project-system. Not re-classified here — moving other archives' artifacts
  unilaterally would violate the surface-don't-override rule. Next BRIEF-audit
  pass to action with the owning archives.
- [ ] **Stricter manifest validation** — internally-tagged enum cannot enforce
  `deny_unknown_fields`; unknown keys in a section are currently ignored. Add a
  stricter validation pass in P2/P3.
- [ ] **Deployed-binary provenance** — the current `/usr/local/bin/app-mediakit-marketing`
  source is unverified (the monorepo crate was a stub). The rewrite supersedes it
  regardless; confirm at P5 before the cut-over.

## Work log

2026-06-13 totebox@project-marketing (claude-code): Created this master BRIEF.
Scaffolded `app-mediakit-shell` + rewrote `app-mediakit-marketing` (P1); wired
workspace members + release profile; updated project-registry. Verified build,
11 tests, clippy clean, live server-render (bundler/template absent), MCP
propose→approve(F12) round-trip. CODE committed to the monorepo clone (Stage 6
pending).
