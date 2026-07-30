---
artifact: brief
schema: foundry-brief-v1
brief-id: project-design-design-system-platform-2030
title: "Design System Platform 2030 — Sovereign SMB-First DTCG-Native CMS"
status: active
owner: project-design
created: 2026-06-14
updated: 2026-06-14
parent: project-design-app-privategit-design
---

# BRIEF — Design System Platform 2030

## Context

The design-token standard war ended on 28 October 2025 when the W3C DTCG shipped
its first stable specification (version 2025.10), backed by reference
implementations across Style Dictionary, Tokens Studio, Terrazzo, Penpot, Figma,
and Sketch. This collapses the historical lock-in moat: tokens are now a
portable, vendor-neutral interchange format rather than a proprietary asset.

Value migrates up the stack — to automation, governance, AI generation, and
publishing — precisely the layers where, per the zeroheight 2026 Design Systems
Report, only 40% of teams have any pipeline automation and 60% still manually
sync tokens between design, docs, and code. The format is now commodity; the
management-and-publishing platform that sits on top of it is where
differentiation lives.

This BRIEF records the 2030 platform vision for which `app-privategit-design`
v0.2.0 is Phase 1. It is the strategic frame for the clean-sheet architecture
specification synthesized from 11 OPUS research tracks (carbon-v11,
wysiwyg-dual-mode, sidebar-nav, vision-2030, ai-integration, rust-sovereign,
mobile-css, marketing-pages, dtcg-schema, contributor-workflow, bundle-ux) plus
the project-marketing master BRIEF, the project-bim objects-system BRIEF, and a
direct audit of the live repository.

## Competitive landscape

The token format is open and self-hostable, but the token management platform —
the CMS that governs, versions, documents, and publishes from the token graph —
is almost entirely cloud SaaS: Supernova, zeroheight, Knapsack, Specify, Figma
Variables. All are US-hosted and structurally disqualified for regulated buyers
(financial services, legal, government) who cannot send brand IP to a US SaaS.

The major design systems do not close this gap for small regulated teams:

- **Carbon (IBM):** rich `--cds-*` token surface but React-first; no DTCG export
  yet (carbon issue #22437 open); `cds-side-nav` caps at ~2-level depth, no
  virtualization — unusable for a deep token tree.
- **Spectrum (Adobe) / Material (Google) / Polaris (Shopify):** vendor-coupled,
  cloud-tooled, and assume large in-house engineering teams. None ship a
  self-hostable management-and-publishing layer.
- **Tokens Studio / Style Dictionary:** Style Dictionary is only the *transform*
  step (token graph → CSS/Rust); Tokens Studio is a Figma plugin. Neither is a
  governance-and-publishing platform.

The single open-source, air-gappable, DTCG-native incumbent is **Penpot** — but
Penpot is positioned as a *design tool*, not a publishing/CMS substrate. No
incumbent occupies the slot of a self-hosted, DTCG-native design-system CMS that
publishes UI, marketing pages, *and* documents from one token source inside the
customer's perimeter.

## The gap we fill

"Sovereign DTCG-native" means: the token graph is authored, governed, and
published entirely inside the customer's own perimeter — self-hosted or
air-gapped on `os-privategit`, customer-held keys, no brand IP leaving for a US
SaaS, and AI generation served locally (OLMo via Doorman, SYS-ADR-07-safe)
rather than cloud inference on the customer's IP.

Who cannot use Figma/Adobe cloud tools today and is therefore stranded:

- Financial-services firms under data-residency and third-party-risk rules.
- Legal and professional-services firms whose brand and document templates are
  privileged work product.
- Government and public-sector bodies with on-prem-only procurement mandates.

These buyers are exactly the SMB-to-mid regulated segment the cloud platforms
cannot retrofit without abandoning their hosted-DB business model.

## Design system as CMS

The thesis: **one DTCG token graph drives UI components, token-browser
documentation, marketing pages, and legal/regulated documents.** The design
system *is* the CMS. This is articulated in the literature (Sanity, AEM
Universal Editor, CMS.gov) but no product ships a single token graph that
simultaneously generates UI components, marketing/landing pages, *and*
legal/regulated documents.

Technical thesis (from track-marketing-pages and the v0.2.0 spec §3):

1. **TOKEN leg** — token sets plus a DTCG 2025.10 Resolver Module file
   (`resolver.json`: `sets`, `modifiers`, `resolutionOrder`). The Resolver
   Module is the spec-blessed theming mechanism — `$modes` is dead (issue #210
   not adopted).
2. **Build step (Rust, no Node)** — a sovereign reimplementation of the Style
   Dictionary CSS transform flattens the token sets, applies context, resolves
   aliases, and emits `:root{--…}` custom properties per context (the STYLESHEET
   leg).
3. **TEMPLATE leg** — front-matter (template + theme context) plus a closed
   block sequence (`:::hero`, `:::feature-grid`, `:::cta`, `:::pricing`,
   `:::logo-wall`). Components consume only CSS custom properties, never
   literals — styling is structurally inaccessible to the author.
4. **Render (Axum)** — parse front-matter, select template + resolver context,
   render via minijinja, inject the per-context compiled CSS-variable block.

The same manifest pattern produces a DESIGN-BUNDLE (TOKEN + STYLESHEET +
TEMPLATE), mirroring a legal-document bundle. The marketing/design TEMPLATE leg
adds a composition grammar; the TOKEN leg adds resolver contexts.

## Token schema architecture for 2030

The complete 2030 token schema (from track-dtcg-schema; current bundle uses only
6 of 13 stable DTCG `$type` values). Tiers: `primitive` → `semantic` →
`component` → `workplace`.

Correctness fixes (scheduled with v0.2.0):

- Relocate invalid types — `$type: "string"` (4×) and `$type: "boolean"`
  (dtcg-vault) are not valid DTCG types; move to `$extensions`.
- Migrate legacy string forms (`"16px"`, `"150ms"`, `"0"`) to the 2025.10 object
  form `{value:16,unit:"px"}` — the largest migration item; required for strict
  validation, Style Dictionary v5, and Figma API round-trip.

Groups / `$type` values to add (highest impact first):

| Add | `$type` | Serves |
|---|---|---|
| `semantic.typography` | typography (composite) | viewer + marketing + editor |
| `semantic.elevation` | shadow (composite) | UI + marketing cards |
| `semantic.border` | border (composite) + strokeStyle | all surfaces |
| `semantic.transition` | transition (composite) | UI + marketing hover |
| `primitive.font.size` | dimension scale | prerequisite for typography composites |
| `semantic.z` | number | editor/token-browser overlays |
| `semantic.gradient` | gradient (composite) | marketing hero/CTA/brand-fade |
| `semantic.opacity` | number | disabled/muted/scrim |
| `semantic.print` | dimension group | print-aware viewer |
| `semantic.icon.size`/`.color` | dimension / color | all (icon identity by string ref) |

Theming structural change: split the single bundle into `primitive.json` +
`theme/{light,dark,dark-high-contrast}.json` + a Resolver Module file, with
high-contrast as a layered override. Do NOT invent `$type:"asset"` (no spec
support). All of this is DESIGN-* work owned by this Totebox — staged as DESIGN
drafts and committed here, not promoted from this archive.

## Contributor workflow vision

How non-engineers participate in a sovereign design system (from
track-contributor; proportionate for the 2–5 person team that is the *normal*
design-system size — small by design, not small by default):

1. **Propose** — one `DESIGN-RESEARCH-*` draft with `foundry-draft-v1`
   frontmatter + five research-trail fields: intent, named token list affected,
   mockup, variation-vs-new, WCAG 2.1 AA target. The increase-scope-or-don't
   gate.
2. **Token first, in Git** — define/edit tokens before any component code; the
   token-file diff is the intent preview.
3. **Build + document together** — component plus the four-section markdown doc
   (usage / style / code / accessibility) in one PR; non-engineer prose routes
   through drafts-outbound → project-design gateway (Foundry-native equivalent of
   Supernova's contributor-without-publish role).
4. **Two-layer preview + co-sign** — token-diff (intent) plus visual-diff
   (consequence); operator/Master adds the co-sign in frontmatter
   (DESIGN-TOKEN-CHANGE already requires Master co-sign in this archive).
5. **Commit, version, monthly audit** — `bin/commit-as-next.sh`; SemVer (minor =
   new component, patch = token/fix, major = breaking rename); Stage 6 routed to
   Command Session via outbox; one monthly token audit (not weekly — over-ritual
   at this scale).

Skip as premature at 2–3 people: weekly audits, champion programs, RFC
committees. Figma Dev Mode / MCP / Code Connect are import-only, draft-tier,
hand-verified — never a code source of truth.

## The leapfrog argument

app-privategit-design's leapfrog is to occupy the unbuilt slot with three claims
no incumbent can match simultaneously:

1. **Sovereign by architecture** — self-hosted/air-gapped on os-privategit,
   customer-held keys. The cloud platforms cannot retrofit this without
   abandoning their model.
2. **DTCG-native end-to-end** — authoring → governance → multi-target publishing
   in one open-standard graph, where Style Dictionary is only the transform step
   and Penpot only the design step.
3. **Design system as CMS** — the token graph drives marketing pages and legal
   documents, not just UI.

The "We Own It" moonshot discipline (sovereign reimplementations of the Style
Dictionary transform, the highlighter, the template engine, the search index)
plus a local-inference AI layer compounds the moat: the regulated buyer gets
on-prem AI generation over their own token graph rather than cloud inference on
their IP. Competitive flanks to watch: **Penpot extending upward** into
publishing/governance (nearest threat), and **AI-generation vendors forcing an
on-prem-inference requirement** (an opportunity to bundle).

## Relationship to app-privategit-design v0.2.0

Today's v0.2.0 work is **Phase 1** of the 2030 platform — it lays the sovereign,
DTCG-native, axum-SSR substrate everything else builds on.

- **v0.2.0 (now)** — decompose the 293-line `src/main.rs` into the chassis module
  tree; schema-aware rendering (COMPONENT / TOKEN / RESEARCH / MARKETING; BUNDLE
  deferred pending Command ratification); SSE live-reload sidebar with
  `content-visibility:auto`; Carbon `--cds-*` CSS-var absorption (no bundler);
  WYSIWYG edit-overlay from a shared AST (contenteditable + DTCG form fields);
  the `/ai/session` SSE relay (DoormanOlmo + own-credential ClaudeCloud); DTCG
  correctness fixes (invalid-type relocation, object-form migration). Sovereign
  day one for search (`moonshot-index`, tantivy never added) and FS-watch
  (raw inotify, notify never added).
- **v0.3.0** — typography / elevation / border / transition composite token
  groups land; the Rust resolver build emits per-context CSS; the marketing
  render surface ships (design.pointsav.com from the token graph). DESIGN-BUNDLE
  detail page if ratified.
- **v1.0** — full design-system-as-CMS: one token graph driving UI components,
  marketing pages, documentation, and legal/regulated document templates; the
  monthly-audit contributor workflow operational; SemVer-governed releases.
- **2030** — sovereign moonshot replacements mature (`moonshot-template`,
  `-highlight`, `-markup`, plus shared `-schema-validator`, `-code-editor`,
  `-registry`); on-prem AI generation over the customer's own token graph as a
  bundled differentiator against cloud-inference vendors.

## Decisions locked

- DTCG 2025.10 is the canonical token format; Resolver Module is the theming
  mechanism (`$modes` rejected).
- Sovereign-by-architecture, self-hosted on os-privategit, customer-held keys —
  the non-negotiable wedge against cloud SaaS incumbents.
- "Design system as CMS" — one token graph drives UI + marketing + documents —
  is the unbuilt slot and the strategic thesis.
- Chassis module split (mirrors project-marketing `app-mediakit-shell`) is the
  ratified Rust structure; Carbon CSS-vars + selective web-components + axum-SSR
  + minimal-JS is the cross-archive stack (shared with project-bim,
  project-marketing).
- We-Own-It: search and FS-watch are sovereign in v0.2.0; tantivy and notify
  never enter the dep tree; tokio/axum/tower-http/serde are pinned audited
  substrate, not replacement targets.
- AI bridge uses the user's own Anthropic credential held only in the per-user
  browser session — NOT a claude.ai session relay (unsupported, ToS-violating).
  Local leg is OLMo via Doorman. Human reviews diff, human approves (F12;
  SYS-ADR-07/10/19 maintained).

## Decisions open

- **DESIGN-BUNDLE renderer** — deferred pending Command ratification (operator
  decision 2026-06-14); `routes/bundle.rs` reserved, not built.
- **Marketing surface reuse vs reimplement** — should design.pointsav.com reuse
  project-marketing's `app-mediakit-shell` lib crate rather than reimplement the
  Section/render chassis? Cross-archive coordination needed.
- **Editor divergence** — project-bim chose CodeMirror 6; project-design research
  recommends the lighter contenteditable + form-field overlay. Reconcile via a
  shared `moonshot-code-editor`, or document the deliberate difference (BIM edits
  raw IFC/JSON; design edits prose + token forms).
- **Shared moonshot registry** — `moonshot-{template,highlight,markup,
  schema-validator,code-editor,registry,fs-watch}` overlap project-bim and
  project-marketing needs. Per the OXC "one AST, many tools" lesson, these should
  be a single shared registry, not per-archive duplicates — route to Command
  Session for cross-archive deconfliction.
- **Manifest/NEXT contamination check** — project-marketing inherited M-17
  contamination (wrong cluster in manifest.md, project-orgcharts content in
  NEXT.md). Verify app-privategit-design's manifest/NEXT before further BRIEF
  rewrites.

## Work log

- 2026-06-14: BRIEF created from 11-track OPUS research synthesis (carbon-v11,
  wysiwyg-dual-mode, sidebar-nav, vision-2030, ai-integration, rust-sovereign,
  mobile-css, marketing-pages, dtcg-schema, contributor-workflow, bundle-ux) +
  project-marketing master BRIEF + project-bim objects-system BRIEF + live
  repository audit (Cargo.toml v0.1.0, src/main.rs 293 lines, dtcg-bundle.json
  6-of-13 `$type` values). Companion to the app-privategit-design v0.2.0
  clean-sheet architecture specification.


---

## We Own It — Dependency Tier Table

Tier assignments per [we-own-it-principle](../../../conventions/we-own-it-principle.md).

| Component | Tier | Notes |
|---|---|---|
| app-privategit-design | Tier 1 — Ours | Design system browser; Rust/axum; v0.2.0 live |
| pointsav-design-system | Tier 1 — Ours | DTCG 2025.10 token repository; dtcg-bundle.json canonical source |
| moonshot-fs-watch | Tier 1 — Ours | Sovereign inotify FS watcher; replaces notify-rs |
| moonshot-index | Tier 1 — Ours | Sovereign search (planned); replaces Tantivy; no Tantivy added to this binary |
| moonshot-{highlight,markup,template} | Tier 1 — Ours | Planned Phase 3+ renderers; Bridge entries below active until shipped |
| Axum | Tier 3 — Vendored auditable | MIT; HTTP framework; shared across PointSav Rust binaries |
| Carbon CSS vars (--cds-*) | Tier 4 — Tooling only | IBM Carbon tokens absorbed as CSS custom properties only; no bundler; no runtime call-home |
| Style Dictionary | REJECTED | Transform CLI; replaced by own DTCG token compiler from day one |
| Parcel / Vite / Webpack | REJECTED | No bundler; CSS vars absorbed directly; browser-native modules only |
