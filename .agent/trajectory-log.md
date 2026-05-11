# project-design — trajectory log

This file is the cluster's session-trajectory write log per
DOCTRINE.md §XV and `conventions/trajectory-substrate.md`.
Every Task session appends a structured entry summarising what it
did, what it changed, and what surprised it.

`bin/capture-trajectory.sh` consumes this log to emit Stage-0
trajectory tuples into the apprenticeship corpus.

---

## 2026-04-28 — Session 7f2199099d10ff0f (continued) — v0.0.3

**Master's v0.0.2 RATIFIED reply (04:22Z) named three v0.0.3
priorities.** All three shipped this round.

### Two commits on `cluster/project-design`

- `32b2847` (pointsav-monorepo, Peter) — engine: live recipe
  rendering + query_research MCP method + nav.rs +12 entries
- `0f7bb44` (pointsav-design-system, Jennifer) — 4 new
  component recipes (select / checkbox / switch / tab)

### Live recipe rendering (the big one)

`render_recipe_demo()` in render.rs takes a Component and emits
a `<section class="ps-recipe-demo">` block at the top of the
Usage tab with:
- `<style>` injection of `recipe.css`
- Per-variant HTML rendering (substituting placeholders with
  per-variant default content — `Save changes` / `Cancel` /
  `More options` / `Delete account` for button variants;
  `Active` / `Verified` / `Expiring` etc. for badge variants)
- Hardcoded fallback demos for templates that use Handlebars-
  style `{{#each}}` blocks (navigation-bar, breadcrumb)
- ARIA notes summary as a collapsible `<details>` block

The substrate now visibly RENDERS its own recipes — you can see
the actual button working in the Usage tab, not just markdown
describing it. This is what Master called "moves from 'we have
data' to 'we render it'." Closes the dead-code warnings on
ComponentRecipe.{html, css, aria} fields naturally — the fields
are now consumed at render time.

style.css extended with `.ps-recipe-demo` block (surface-base
container, header band, stage area, per-variant uppercase mono
caption, ARIA notes summary).

### query_research MCP method

Substring search across research entry slugs / titles / markdown
bodies. Returns matches with title + URL + ~80-char snippet
window around the first match. Method count: 12 (was 11).
`tools/list` updated.

`research_snippet()` helper handles the snippet windowing —
saturating subtraction at the start, min-clamped end, with
ellipsis on either side as appropriate.

### 4 new component recipes

- `select` — single-choice picker; native `<select>` (anti-
  pattern note: custom `<div role="listbox">` rejected per
  WCAG 2.2 substrate floor)
- `checkbox` — boolean choice; visually-hidden native input
  drives behaviour
- `switch` — `role="switch"` on native checkbox; immediate-
  effect binary settings
- `tab` — URL-reflected page-section navigation; documents the
  substrate's own pattern (state-in-URL, not client memory)

nav.rs Components section expanded from 9 entries (Overview +
8 components) to 13 entries (Overview + 12 components).

### Smoke-tested at port 9095

| Endpoint | Result |
|---|---|
| `/readyz` | components=12, elements=4, research=2 |
| `/components/button/usage/` | 4 variant demos rendered live |
| `/components/badge/usage/` | 5 variant demos rendered live |
| `/components/select/usage/` | HTTP 200, 26975 bytes |
| `POST /mcp describe` | 12-method catalogue (incl. query_research) |
| `POST /mcp query_research {query: "carbon"}` | match + snippet |

Release binary 2.3 MB; build time 35s.

### Master's session notes

- v0.0.2 deployed at 04:28Z from commit `1dd58d1` to
  https://design.pointsav.com/. components_count=8 visible there
  right now.
- Master flagged build error in my mid-development WIP at
  04:28Z (`render_recipe_demo` declared but not defined). I had
  been mid-implementation; the function is now defined and the
  release build passes. Master's git stash + pop preserved
  state.
- Doctrine claim #38 + convention amendments still queued for
  v0.1.55/56 — Master's scope; my source material at
  `primitive-vocabulary-rationale.md` (commit `d2adf18`).

### What surprised me

1. **Live recipe rendering had a subtle CSS scoping concern.**
   My style.css already defines `.ps-btn` for the home-page
   CTAs. The button recipe's own CSS also defines `.ps-btn`
   (with variants `.ps-btn--primary`, `.ps-btn--secondary` etc.).
   Loading both could conflict — but turned out they're
   consistent (same root `.ps-btn` rules; recipe adds variant
   classes my style.css didn't have). No conflict; recipe.css
   simply extends the page CSS additively. Worth noting if a
   future component recipe defines a class my style.css already
   styles differently.

2. **The Handlebars-style template detection.** Two recipes
   (navigation-bar, breadcrumb) use `{{#each items}}` blocks.
   I detect this via simple substring `template.contains("{{#each")`
   and fall back to a hardcoded representative demo. Cleaner
   than running a Handlebars engine; the recipe HTML stays
   illustrative source for adapters, the live demo is hand-
   authored representative HTML.

3. **Master's "leave dead-code warnings visible" advice
   propagated to a real architectural choice.** I was tempted
   to add `#[allow(dead_code)]` on the `tabs_available` field
   when expanding the MCP. Resisted; Master's framing is right —
   the warnings drive prioritisation. Kept the field; expanded
   the MCP method to consume it.

### Tetrad ratification status (v0.0.3)

All four legs pass — same as v0.0.2 just adjusted upward by
component count. v0.0.1 wiki-leg TOPIC draft still pending
project-language sweep (no Tetrad regression — drafts move on
project-language's queue cadence).

### Next session priorities

Per outbox to Master: Style/Code/Accessibility tab content for
the 11 partial components, live-reload on vault writes,
card/alert/dialog/tooltip/accordion (5 more → 17 total).

L7-L10 leapfrog targets queued for later — WCAG audit endpoint,
theme composition, component diff, offline bundle.

---

## 2026-04-28 — Session 7f2199099d10ff0f (continued) — v0.0.2 reframe

**Operator clarification (mid-session):** "We want to make an
'original copy' of IBM Carbon, we do not want to be 'on top' of
IBM Carbon … We need the PointSav Design System to be 100% our
own system." Followed by: "we still want to provide the muscle
memory of IBM Carbon, but we need to make the design system our
own."

This settled the vocabulary question: keep Carbon's structural
patterns (numeric scales, primitive→semantic→component layering,
sidebar+tabs delivery, productive/expressive type split) while
replacing the literal vocabulary, hex values, and IBM Plex font
binding with PointSav-original equivalents.

### What changed

**Two Sonnet sub-agent research runs (G1 + G2):**

- A6 — carbondesignsystem.com structural deep-dive (8 properties
  to reproduce; saved to `.claude/sub-agent-results/`)
- A7 — 2025-2026 design-system frontier + 10 leapfrog targets
  (4 SMALL-cost wins selected for v0.0.2)

**Substrate engine rebuilt (sub-clone pointsav-monorepo, Peter,
commit `1dd58d1`):**

- `vault.rs` — per-component-directory layout reader
- `nav.rs` — static nav tree mirroring Carbon delivery shape
- `render.rs` — HTML layout shell (header + accordion sidebar +
  main + tabs + footer); Box::leak pattern for static-lifetime
  string ergonomics in the Page builder
- `style.css` — embedded stylesheet with PointSav CSS-variable
  vocabulary; `prefers-reduced-motion` global override
- `api.rs` — three leapfrog endpoints (L1 typed DTCG with
  `application/design-tokens+json` content type, L3 shadcn-
  compatible registry at `/r/registry.json` and `/r/<comp>.json`,
  L4 DESIGN.md export at `/api/design-<theme>.md`) plus
  `/api/components.json` and `/api/components/<name>.json`
- `mcp.rs` — L2 expanded MCP server, 11 methods, refactored
  dispatch into inner `run_method` helper that returns
  `Result<Value, (i32, String)>` so `?` works
- `main.rs` — multi-page Axum router with URL-reflected tab
  routes; custom 404 page using the same shell

**Vault content rebuilt (sub-clone pointsav-design-system,
Jennifer, commit `d2adf18`):**

- Tokens: PointSav-original primitive layer (neutral/primary/
  positive/caution/critical numeric scales, space-1..13,
  utility/display type split, ease-utility/display/enter/exit,
  speed-1..6, corner-1..3, focus ring)
- Theme: pointsav-brand.json with semantic mappings + voice
  rules + accessibility floor + GitHub repo URL
- 8 component recipes (button fully populated with all 4 tabs;
  7 others with recipe + usage + stub other tabs)
- 4 foundation pages (color full + tokens; typography, spacing,
  motion overviews)
- About + guidelines/accessibility overview
- Research: design-philosophy.md (updated in transit to drop
  Carbon-as-floor framing); primitive-vocabulary-rationale.md
  (NEW; replaces carbon-baseline-rationale.md)

### Smoke-tested at port 9095 (release binary, 2.3 MB)

| Endpoint | Result |
|---|---|
| `/healthz` | `{"status":"ok"}` |
| `/readyz` | components=8, elements=4, research=2 |
| `/api/components.json` | 8 components listed |
| `/api/components/button.json` | full recipe |
| `/r/registry.json` | shadcn-conformant 8-item registry |
| `/api/tokens/pointsav-brand.dtcg.json` | `Content-Type: application/design-tokens+json` ✓ |
| `/api/design-pointsav-brand.md` | DESIGN.md export with YAML frontmatter + 8 sections |
| `POST /mcp describe` | 11 methods, 4 leapfrog targets implemented |
| HTML pages (home, components/button/{usage,style,code,accessibility}, elements/color/overview, all-about-pointsav/what-is-pointsav-design, research/design-philosophy) | all 200 OK, ~25-27 KB |

### What surprised me

1. **The `?` operator in axum handlers.** My initial mcp.rs used
   `?` inside the `result:` match block to short-circuit on
   parameter validation failures, but the outer `dispatch` fn
   returns `(StatusCode, Json<...>)`, not `Result<...>`. Compile
   failed with E0277. Refactor: extract inner `run_method` fn
   that returns `Result<Value, (i32, String)>`; outer dispatch
   matches on the result. Cleaner API; also enables the future
   `tools/list` and `resources/list` discovery methods that share
   the same param-validation pattern.

2. **Box::leak for static-lifetime str ergonomics.** The Page
   builder takes `&'static str` for navigation labels and hrefs.
   For dynamic strings (per-component routes built at request
   time), Box::leak is the simplest path. Bounded leak — vault
   has fixed cardinality, low traffic — but a future refactor
   could swap to `Cow<'_, str>` for true zero-leak request
   handling.

3. **Stale binary on production port.** The currently-running
   `local-design.service` on 127.0.0.1:9094 is still the v0.0.1
   binary. Master fired bootstrap somewhere; the v0.0.2 binary
   is at `target/release/app-privategit-design` waiting for a
   re-bootstrap. Outbox to Master signals this.

4. **Doctrine amendment needed (Master scope).** Doctrine claim
   #38 narrative + `conventions/design-system-substrate.md`
   currently say "Carbon muscle-memory floor" implying token
   import. Per §11 action matrix this is Master scope to amend.
   Outbox carries the recommendation: drop "import Carbon's
   token vocabulary" framing while keeping "preserve Carbon-
   pattern structural muscle memory."

### Tetrad ratification status (v0.0.2)

| Leg | State |
|---|---|
| Vendor | Engine refactor + 8 component recipes (commit `1dd58d1`) |
| Customer | GUIDE-deploy-design-substrate.md from v0.0.1 still accurate |
| Deployment | Vault repopulated; `target/release/app-privategit-design` ready for redeploy |
| Wiki | TOPIC draft from v0.0.1 still pending project-language sweep |

Master ratification of v0.0.2 milestone should pass the four-leg
check. Outbox sent.

### Next session priorities

After Master re-runs bootstrap.sh + amends doctrine claim #38:

- Live-reload on vault writes (today reads once at startup)
- Sub-tabs for the 7 partial components (style/code/
  accessibility) — bulk authoring work
- L7 (WCAG audit endpoint) — medium cost; high SMB value
- L8 (theme composition endpoint) — medium cost; multi-brand
  fan-out
- Coordinate with project-slm Task on Doorman /v1/design-system
  endpoint integration

---

## 2026-04-28 — Session 7f2199099d10ff0f — v0.0.1 first iteration

**Operator brief:** "GO LIVE ASAP at design.pointsav.com" — first
iteration of Doctrine claim #38 (Design System Substrate) under
Tetrad Discipline (claim #37). Bounded scope: vendor + customer +
deployment + wiki legs in one milestone.

### What changed (three sub-clone commits, all on cluster/project-design)

**Sub-clone 1 — pointsav-monorepo (commit f868358, Jennifer):**

- Renamed `app-privategit-design-system/` → `app-privategit-design/`
  via git mv. Project-registry.md row updated (rename + state
  Scaffold-coded → Active per §9).
- Cargo.toml: renamed package, declared `[[bin]]`, added Axum +
  tokio + serde + reqwest + tracing + pulldown-cmark deps.
- Workspace Cargo.toml: added `app-privategit-design` and
  `os-privategit` to `[workspace.members]`.
- Authored `src/main.rs` (Axum router + env config), `src/vault.rs`
  (vault directory reader; DTCG bundle assembly; YAML-ish
  frontmatter decoder; markdown-to-HTML rendering), `src/showcase.rs`
  (HTML showcase renderer with embedded base CSS), `src/mcp.rs`
  (MCP JSON-RPC 2.0 dispatcher — 4 methods: describe, list_tokens,
  list_components, list_research). Removed prior stub `src/lib.rs`.
- Activated both projects per §9: CLAUDE.md + NEXT.md from templates.
  README.md + README.es.md rewritten in transit per §6 (replaced
  prior marketing-vocabulary scaffold copy: "Sovereign Data
  Protocol", "mathematically air-gapped", etc.).
- `os-privategit/`: state Scaffold-coded → Active; lib.rs + Cargo.toml
  + READMEs rewritten in §6 register; CLAUDE.md + NEXT.md authored.
  Boot pipeline lands in subsequent milestones.
- Release build: `cargo build --release -p app-privategit-design`
  succeeded; binary at `target/release/app-privategit-design`
  (2.2 MB ELF, dynamically linked, 4 dead-code warnings on schema-
  fields-not-yet-rendered — expected).
- Active counter 4 → 6; Scaffold-coded counter 53 → 51.

**Sub-clone 2 — pointsav-design-system (commit cd4d204, Peter):**

- Created `dtcg-vault/` canonical template (separate from existing
  YAML-canonical layer at repo root which serves project-orgcharts
  as downstream consumer).
- `dtcg-vault/tokens/primitive.json` — DTCG primitive layer
  baselined on IBM Carbon vocabulary (color, type scale, spacing,
  motion, focus ring; ~5 KB; references use DTCG `$value`/`$type`/
  `{color.blue-60}` token-reference forms).
- `dtcg-vault/themes/pointsav-brand.json` — semantic-layer overrides
  re-pointing roles at primitives; voice rules; accessibility
  commitments.
- `dtcg-vault/components/button-primary.json` — first component
  recipe (HTML + CSS + ARIA + token-references + WCAG specs +
  anti-patterns + research-link).
- `dtcg-vault/research/design-philosophy.md` — substrate doctrine
  narrative (~6 KB; foundry-design-research-v1 schema).
- `dtcg-vault/research/carbon-baseline-rationale.md` — primitive
  layer rationale (~8 KB).
- `dtcg-vault/README.md` + `README.es.md` describe the template
  + how it differs from the existing YAML layer.

**Sub-clone 3 — pointsav-fleet-deployment (commit 2701bf3, Jennifer):**

- Renamed `vault-privategit-design-system/` →
  `vault-privategit-design/`.
- Removed prior generic edge-node placeholder content
  (`guide-deployment.md` + `guide-provision-node.md` — generic
  boilerplate using banned vocabulary).
- Authored `MANIFEST.md` (foundry-fleet-catalog-v1, declaring
  canonical instance + infrastructure IaC pointer + tenant fan-out
  including future woodfine instance-2).
- Authored `GUIDE-deploy-design-substrate.md` — operator runbook
  (English-only per §14): prerequisites + 7 stages + SMB customer
  fork procedure + decommissioning + troubleshooting.
- Authored `README.md` + `README.es.md` (bilingual per §6).
- project-registry.md updated: state Reserved-folder → Active.
  Active counter 0 → 1.

### Deployment vault populated

`/srv/foundry/deployments/vault-privategit-design-1/{tokens,
components, themes, research}/` seeded from
`pointsav-design-system/dtcg-vault/`. Verified: 4 directories,
4 content files (primitive.json, button-primary.json,
pointsav-brand.json, plus 2 research markdown files).

### Wiki leg

- Staged TOPIC draft at
  `clones/project-design/.claude/drafts-outbound/topic-design-system-substrate.draft.md`
  (~20 KB; foundry-draft-v1 frontmatter; 10 sections covering all
  three structural inversions + Carbon framing + AI codegen
  frontier + customer fork + hyperscaler-cannot-replicate +
  what-it-is-not + cross-references).
- Emitted `draft-created` JSONL event to
  `~/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/draft-2026-04-28-topic-design-system-substrate.jsonl`
  (21 KB; full draft as `raw` field; project-language Task will
  pick up via draft-sweep.sh).

### What surprised me

1. **Rename loses git's automatic detection at full content
   rewrite.** I `git mv` then completely rewrote Cargo.toml + lib.rs
   replaced by new src/ structure + READMEs rewritten — git's
   default 50% rename threshold did not detect the rename, so the
   commit shows files as add+delete rather than rename. Fine
   semantically (the content is essentially new), but the lineage
   is preserved only via the `git log --follow`-able path through
   intermediate states. Not an issue but worth noting.

2. **The existing scaffold READMEs at app-privategit-design-system
   and os-privategit carried banned vocabulary** ("Sovereign Data
   Protocol", "mathematically air-gapped", "Architectural Mandate",
   "Component Isolation"). Per CLAUDE.md §6 these are §5 banned
   terms. Rewrote in transit. The pattern: when activating a
   Scaffold-coded project that has stale scaffold copy, treat the
   activation as the §6-edit-in-transit point.

3. **The pointsav-design-system repo carries TWO canonical token
   layers**: the existing YAML layer at `tokens/global/`, `themes/`,
   `components/*.css` (consumed by project-orgcharts), and the new
   DTCG layer I added at `dtcg-vault/`. I avoided clashing — the
   substrate engine reads from a deployment-instance vault, not
   from this repo directly. SMB customers fork `dtcg-vault/` as
   their template; project-orgcharts continues to consume the
   YAML layer. Migration to DTCG-only happens in subsequent
   milestones, coordinated with project-orgcharts.

4. **The fleet-deployment catalog folder rename hit a staged-vs-
   working-tree mismatch.** After `git mv vault-privategit-design-system
   vault-privategit-design`, attempting `git rm` on the renamed
   files inside the new folder failed with "changes staged in the
   index" until I added `-f`. Expected for `git mv` semantics; just
   noting the friction.

### Tetrad ratification readiness

All four legs of the cluster's tetrad now carry concrete content:
- **Vendor leg** — engine code in pointsav-monorepo (release-built,
  binary verified)
- **Customer leg** — MANIFEST + GUIDE in fleet-deployment
- **Deployment leg** — vault-privategit-design-1/ populated
- **Wiki leg** — TOPIC draft + JSONL event

Master ratification of v0.0.1 milestone should pass the four-leg
check. Outbox message to Master pending bootstrap coordination.

### Next session priorities

After Master runs bootstrap.sh + certbot:
- Live-reload on vault writes (today reads once at startup)
- service-fs ledger anchoring on vault changes
- More component recipes (input-text, link-primary, etc.)
- Coordinate Doorman /v1/design-system/<tenant> endpoint with
  project-slm Task
- Style Dictionary + Figma Variables export builds

Cluster trajectory captured. Session ending.

---
## 2026-05-06 — Task: v0.1.0-leapfrog Platform Delivery
- **Engine Evolution:** Iterated the Axum substrate into a live CMS. Added `notify` watcher, `ArcSwap` hot-reloading, and SSE browser refresh.
- **Leapfrog UI:** Injected a lightweight Command Palette (`Cmd+K`) and mobile-first bottom navigation for high-conversion discovery.
- **Auditability:** Implemented a 'History' tab on components reading native Git history via `git2`.
- **Tetrad Discipline:** Expanded `/api/artifacts` to programmatically generate TOPIC, GUIDE, DESIGN, ASSET, and TEXT artifacts. Saved a full snapshot to `dtcg-vault/exports`.
- **Handoff:** Initiated project-editorial review via mailbox for artifact promotion.
- **Status:** v0.1.0-leapfrog is LIVE on port 9094.
