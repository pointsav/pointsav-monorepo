---
schema: foundry-draft-v1
state: complete
originating_cluster: project-design
target_repo: n/a
target_path: .claude/sub-agent-results/A7-design-system-frontier-leapfrog-2026-04-28.md
audience: project-design Task Claude, Master Claude
bcsc_class: internal-operational
language_protocol: PROSE
authored: 2026-04-28
authored_by: sub-agent (Sonnet 4.6)
authored_with: claude-sonnet-4-6
references:
  - https://developers.figma.com/docs/figma-mcp-server/
  - https://www.figma.com/blog/design-systems-ai-mcp/
  - https://www.w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/
  - https://blog.google/innovation-and-ai/models-and-research/google-labs/stitch-design-md/
  - https://github.com/google-labs-code/design.md/blob/main/docs/spec.md
  - https://ui.shadcn.com/docs/registry/mcp
  - https://ui.shadcn.com/docs/registry/registry-json
  - https://vercel.com/blog/ai-powered-prototyping-with-design-systems
  - https://v0.app/docs/design-systems
  - https://medium.com/design-bootcamp/when-design-system-documentation-becomes-ai-readable-14f7a3180233
  - https://react-spectrum.adobe.com/architecture.html
  - https://help.figma.com/hc/en-us/articles/32132100833559-Guide-to-the-Figma-MCP-server
  - https://www.designtokens.org/tr/drafts/
  - https://styledictionary.com/info/dtcg/
notes_for_editor: Research-only artifact. All citations live-fetched 2026-04-28.
---

# A7 — Design-System Delivery Frontier: Leapfrog Analysis for 2026

*Research task for project-design / pointsav-design-system. Commissioned 2026-04-28.*

---

## §1 The 2025-2026 delivery landscape — survey

### Carbon IBM (baseline, not repeated in depth)
Carbon remains the benchmark for open-source enterprise design systems: component-level tokens, extensive accessibility documentation, React/Web Components dual delivery, and public Storybook showcase. Its structural weaknesses are the same ones that motivate every other system on this list: Figma-coupling, no live API surface, heavy JS framework assumptions, and no path for a customer to fork and self-host.

### Material 3 — Google
Material 3 distinguishes itself by delivering across four first-class targets simultaneously: Web (Lit-based web components), Android (Jetpack Compose), iOS (SwiftUI), and Flutter. Tokens are available as CSS custom properties for web and as native platform variables for mobile targets, all generated from a single Material Theme Builder export. The April 2026 announcement of DESIGN.md (see §4) indicates Google is moving toward a file-format specification that any AI agent can read — a recognition that Figma-centric delivery leaves half the market behind. Material's structural lead is cross-platform token coherence at a scale no other system matches. Its structural weakness: tight coupling to Google's brand language makes it difficult for a third-party SMB to fork without significant token rebasing, and there is still no live HTTP API for querying components or tokens programmatically.

### Shopify Polaris — Commerce-tier vertical integration
Polaris is the clearest example of a design system that deliberately does *not* try to be general-purpose. It serves Shopify Admin and Shopify merchant-facing surfaces exclusively. Tokens are commerce-vocabulary-first (terms like `--p-color-bg-fill-success-active` are semantically commerce-meaningful, not generic). Delivery is npm-only (`@shopify/polaris`) with React as the only first-class framework. The Figma library is private by default — Polaris is not a design system you adopt; it is one you conform to. This is a deliberate moat for Shopify's ecosystem. The lesson for PointSav: vertical-semantic token naming (tokens whose names encode business meaning rather than just visual properties) is a structural advantage when the design system owns a business domain. Polaris's structural weakness from PointSav's perspective: zero portability, zero self-hosting, zero non-React delivery.

### Atlassian Design System — Tokens-first + AI patterns acknowledged
Atlassian's homepage explicitly calls out "Rovo and AI patterns" as a foundations pillar — one of the first enterprise design systems to name AI interaction patterns as a first-class concern alongside color, typography, and iconography. Token delivery is DTCG-compatible JSON exported from Figma via Tokens Studio, with a documented three-tier token hierarchy (global → semantic → component). The system is React-only for implementation. No live HTTP API exists. Structural lead: naming AI patterns as a foundations layer, not just an add-on, is the right mental model. Structural weakness: Figma-dependency for the token pipeline means the source of truth lives in a SaaS tool the customer does not own.

### Adobe Spectrum — Headless behavior layer as structural innovation
Spectrum is architecturally the most interesting in this survey because it separates *behavior*, *tokens*, and *rendering* into independent layers. React Aria implements ARIA-compliant keyboard, pointer, and screen-reader interaction behavior as unstyled React hooks — completely decoupled from visual appearance. Spectrum tokens (CSS custom properties) apply visual design on top. Spectrum Web Components apply both layers as native custom elements. This three-layer separation means a team can adopt React Aria's behavior without any Spectrum visual opinion. No other major design system has published this decomposition at production scale. No live HTTP API exists; tokens are available only as npm packages. Structural lead: behavior-tokens-rendering decomposition. Structural weakness: immense complexity — the three-layer model is genuinely hard to consume for a small team, and the docs are aimed at experienced engineers.

### GitHub Primer — Open tokens, MIT license, minimal framework opinion
Primer's tokens are MIT-licensed JSON, maintained in a public GitHub repository (`primer/primitives`), and cover color, spacing, and typography. The system ships both React components and CSS-only implementations, making it one of the few major design systems where HTML + CSS alone is a supported consumption path. Primer is structurally closest to what PointSav is building because it does not assume Figma ownership, does not require a SaaS subscription, and publishes tokens in a format that is directly usable. Its weakness: the documentation and component library remain GitHub-branded and GitHub-scoped, offering no clear fork-and-rebrand path for a third party. No MCP or HTTP API surface exists.

### Pinterest Gestalt — Performance-first React
Gestalt is React-only, Pinterest-scoped, and ships tokens as CSS custom properties from a Figma source. Its notable structural feature is aggressive performance discipline: component code is optimized for Lighthouse scores, and the documentation is built with Next.js rather than Storybook. No programmatic API or MCP surface.

### Untitled UI — Paid SaaS, Figma-native
Untitled UI is the leading commercial design system in the Figma-native space: $199/year for a Figma file with 20,000+ components, DTCG-compatible variable exports, and no code delivery at all. Its structural model is the opposite of PointSav's: entirely Figma-dependent, entirely SaaS, no self-hosting. It represents the ceiling of what a Figma-native product can charge SMBs — and by implication, the floor PointSav needs to clear.

### Uber Base Web — Multi-framework, theming-focused
Base Web (now largely superseded by Uber's internal rebrand) historically shipped a theme-first model where every component accepted a complete theme object, enabling white-labeling at the component level. The approach proved difficult to maintain — theme-object APIs surface-area-exploded as components proliferated. The lesson for PointSav: per-component theme injection is a seductive architecture that becomes a maintenance liability at scale. DTCG token aliasing (semantic tokens reference global tokens) is the right structural answer to what Base Web tried to do with runtime theme objects.

---

## §2 What Figma's design-system MCP does (and doesn't)

### What it does (as of mid-2025, open beta through 2026)
Figma's MCP server was announced in June 2025 and shipped in open beta. It provides the following to an MCP-capable AI agent (Cursor, Claude Code, VS Code Copilot, Windsurf):

1. **Design context extraction**: Given a selected Figma frame, the server transforms the Figma REST API payload into a compact, token-efficient representation. Pixel coordinates become layout relationships ("centered inside parent"). Raw hex colors become design token references. Deeply nested layers are flattened.
2. **Code Connect integration**: If a design team has mapped Figma components to code components via Code Connect, the MCP server returns the code component name alongside the Figma component, allowing AI agents to generate code that matches the actual codebase rather than reimplementing components.
3. **Write-to-canvas**: Agents can create and modify Figma frames, components, auto-layout rules, and variables programmatically via the MCP server. This is the first bi-directional design-tool MCP surface.
4. **Automated rule generation**: The server can scan a codebase and output a structured rules file — token definitions, component library map, style hierarchy, naming conventions.

### What it does NOT do
- It does not serve DTCG-format token exports at a live HTTP endpoint. Tokens are inferred from Figma variables, not from a canonical token store.
- It does not work without a Figma account. The surface is entirely gated on Figma's SaaS.
- It does not serve component source code. It serves design metadata that an agent then translates to code.
- It cannot query "what does the Button component look like in dark mode on brand X?" across multiple tenants. It is single-file, single-design-team scoped.
- It does not expose design *rationale* — why a token has the value it has, what accessibility constraint it satisfies, what brand decision it encodes.

### The gap PointSav fills
PointSav's MCP server runs at the same HTTP endpoint as the design showcase. It serves:
- Per-component DTCG-format token exports (live, from the Git-tracked vault — not inferred from a design tool)
- Component recipes: HTML + CSS + ARIA annotations, with no framework opinion
- Design-decision research markdown: the *why* behind every token, readable by both humans and LLMs
- Multi-tenant: one server, any number of brand themes, with namespace isolation

This is structurally different from Figma's MCP in a decisive way: PointSav's token store *is* the source of truth. Figma's MCP infers tokens from a design file that is downstream of wherever the real decisions were made. When an SMB asks "why is this button color #1A73E8?", Figma cannot answer. PointSav's vault can.

---

## §3 The AI-codegen / MCP frontier

### State of the art, April 2026

MCP achieved 97 million monthly SDK downloads as of early 2026, with every major AI client — Claude Code, Cursor, VS Code Copilot, Windsurf, ChatGPT — speaking MCP natively. The protocol has become the USB-C connector of AI tooling within 18 months of Anthropic's publication of the spec.

On the design-system side, the MCP surface breaks into three categories:

**1. Design-tool MCP servers (Figma, Penpot)**
Figma's server is described above. Penpot (the open-source Figma competitor, self-hosted) has experimental MCP support in the community but no official server as of April 2026. This is a gap: the only design-tool MCP server with official support is gated on a proprietary SaaS.

**2. Component-registry MCP servers (shadcn)**
shadcn shipped CLI 3.0 and an official MCP server in August 2025. The shadcn MCP server reads a `registry.json` at a well-known URL, lists available components, and allows an AI agent to install component source code into a project by invoking `shadcn add <component>`. The key detail: any registry hosted at `https://example.com/r/registry.json` automatically gets MCP support — the server does not need to be shadcn-branded. This is the registry-as-MCP-surface pattern.

**3. Token-store HTTP servers (emerging)**
No major design system — as of April 2026 — ships a live HTTP endpoint that returns DTCG-format tokens for a named component or theme. This is an open gap. The closest approximation is Style Dictionary v4/v5 running as a build step that outputs static JSON files; but these are build artifacts, not live queryable endpoints.

### Vercel v0 registry spec
v0 (Vercel's AI component generator) uses the shadcn registry format as its design-system integration mechanism. A `registry.json` at a known URL, conforming to the shadcn schema, allows v0 to generate brand-aware prototypes. The registry also supports MCP, which means a properly formatted PointSav registry would be queryable by v0 without any additional integration work. This is a direct go-to-market hook: "your design system works in v0" is a customer benefit PointSav can ship at no additional implementation cost, given the shadcn registry format overlaps substantially with what PointSav already intends to expose.

### Claude artifacts and AI-generated UI
AI coding agents that have access to a design system's MCP server generate component code that matches the actual system — verified at integration time rather than requiring human review. Figma's internal testing shows that Code Connect + MCP dramatically reduces the delta between AI-generated code and reviewed production code. The implication for PointSav: an MCP server that returns component recipes (not just tokens) makes every AI coding agent a design-system-aware code generator.

---

## §4 Beyond DTCG — what else is moving?

### DTCG 2025.10 stable
The W3C Design Tokens Community Group published the first stable version of the Design Tokens Specification on 28 October 2025. Key additions over earlier drafts:
- **Theming support**: light/dark/accessibility variants as first-class citizens, not workarounds
- **Modern color spaces**: Display P3, Oklch, and full CSS Color Module 4 support — resolving years of frustration with sRGB-only token stores
- **Token relationships**: aliases, inheritance, and component-level references are now formally specified
- **Cross-platform generation targets**: iOS (Swift), Android (Kotlin/XML), Flutter, and web — one token file to rule all four

Style Dictionary v4 has first-class DTCG support; v5 (in progress) targets the full 2025.10 stable spec. Tokens Studio defaults to DTCG format. Figma Variables can export to DTCG via Tokens Studio.

The media type `application/design-tokens+json` and file extensions `.tokens` or `.tokens.json` are now part of the stable spec — meaning PointSav can serve tokens at a Content-Type that tooling will recognise without custom negotiation.

### Google DESIGN.md (April 2026)
On 21 April 2026 — one week before this research was commissioned — Google Labs open-sourced the DESIGN.md specification under Apache 2.0. This is a parallel track to DTCG rather than a replacement:

- **Structure**: YAML frontmatter (machine-readable tokens) + markdown body (human-readable rationale, organized into prescriptive sections: Overview, Colors, Typography, Layout, Elevation, Shapes, Components, Do's and Don'ts).
- **Token syntax**: `{path.to.token}` references cross-link tokens within the file.
- **Interoperability**: The CLI exports to DTCG (`npx @google/design.md export --format dtcg DESIGN.md`) and Tailwind config.
- **AI interface**: Drop a `DESIGN.md` at the project root; any compatible AI agent reads it and generates brand-consistent interfaces. The CLI also provides a linter that can be integrated into CI pipelines.

The DESIGN.md format is explicitly designed to carry *design rationale* alongside tokens — the "why" that DTCG explicitly excludes from its scope. This is the first standardised attempt to make design reasoning machine-readable.

PointSav's per-component AI-readable research markdown is structurally aligned with this direction. PointSav was doing this before Google published the spec. DESIGN.md should be treated as a convergence target: PointSav should be able to export a `DESIGN.md` from the vault at `/api/design.md` or per-theme at `/api/design-<theme>.md`.

### Style Dictionary v4/v5 status
Style Dictionary v4 has stable DTCG support. v5 is in progress, targeting full 2025.10 compliance. For PointSav's purposes, Style Dictionary is useful as a build-time transform (vault DTCG tokens → platform-specific CSS/Swift/Kotlin), not as a runtime. The vault stores canonical DTCG; Style Dictionary runs at the cluster build step or on-demand via the HTTP API.

### Tokens Studio standalone platform
Tokens Studio (264,000+ active Figma plugin users) is moving beyond a Figma plugin into a standalone platform with integrations to Framer, InDesign, Blender, and PowerPoint — a recognition that the Figma-centric workflow does not cover the full design production lifecycle. For PointSav, Tokens Studio's move confirms the direction: DTCG as the interchange format, with multiple tool integrations as outputs. PointSav's Git-tracked vault already implements this pattern.

---

## §5 Storybook decline — what's replacing it?

### The situation in April 2026
Storybook 10 is the current version. It remains the ecosystem leader by weekly downloads (2.5M/week), addon count, and enterprise adoption. For large React shops with accessibility testing, visual regression, and a dedicated design-systems team, Storybook remains the correct choice.

Ladle (Vite-native, React-only, 1.2s cold start) and Histoire (Vue/Svelte-native) are the primary alternatives. Neither has broken Storybook's ecosystem dominance in the target market of large teams.

### The more important trend: design systems skipping Storybook entirely
Several significant design systems have moved to custom documentation sites rather than Storybook:
- **Material 3**: Custom-built docs at m3.material.io, no Storybook.
- **Primer**: Custom GitHub Pages docs; Storybook used internally but not the public documentation surface.
- **Atlassian**: Custom React + Next.js documentation site.
- **Gestalt** (Pinterest): Custom Next.js site.

The structural reason: Storybook is a *component development workshop*, not a *design system documentation platform*. When a design system's audience is primarily downstream consumers (rather than the team building the components), Storybook's developer-experience affordances are mismatches. Consumers want: live preview, code copy, token reference, accessibility notes, and a clear path to use the component. Storybook gives them: a canvas with controls, addon panels, and a sidebar navigation optimized for component authors.

### PointSav's position
PointSav declined Storybook in favor of a custom Axum-rendered showcase. This is the right call, confirmed by the pattern above. The showcase should be optimized for consumer needs: live component rendering, one-click DTCG export, MCP query, and design-decision rationale inline. Storybook would impose a Node.js runtime dependency, a JavaScript build pipeline, and a developer-first UX that conflicts with PointSav's "SMB-owned, self-hosted binary" value proposition.

The risk to avoid: building a showcase that only a developer can navigate. Material Design's documentation is an example of excellent consumer-facing design system docs — interactive, annotated, accessible to a designer or PM who has never opened a terminal.

---

## §6 The shadcn/ui copy-paste model — implications

### What the model is
shadcn (and its successors) inverts the traditional component library model. Instead of publishing `@company/ui` to npm and having consumers `import Button from '@company/ui'`, shadcn copies the Button source code directly into the consumer's project at `components/ui/button.tsx`. The consumer owns the code. Updates are opt-in diffs, not version bumps. There is no runtime dependency.

In August 2025 shadcn shipped CLI 3.0 + an MCP server, and in early 2026 added Base UI as an alternative to Radix as the underlying headless primitive. The result is a copy-paste system with ~1,400 blocks and 1,189 components, a formal `registry.json` spec, and MCP support out of the box.

### Why this matters for PointSav's vault model
shadcn validates a core PointSav premise: customers want to *own* their components, not depend on a vendor's npm release cycle. The DTCG + HTML + CSS + ARIA recipe model in PointSav's vault is the non-React-specific version of shadcn's model — it gives customers the component as a recipe they can port to any framework rather than React source they still have to adapt.

The registry.json spec is the key interoperability surface. If PointSav's showcase exposes a `/r/registry.json` endpoint that conforms to the shadcn registry schema, it becomes instantly compatible with:
- shadcn CLI (`npx shadcn add https://design.pointsav.com/r/<component>`)
- v0 (Vercel's AI component generator)
- Any MCP client that uses shadcn-registry-aware MCP servers (Cursor, Claude Code, Windsurf)

This is a zero-marginal-cost distribution channel. One HTTP endpoint, conforming to a published schema, wires PointSav into the entire shadcn ecosystem.

### The tension: vault-canonical vs. copy-paste
shadcn's model has a weakness PointSav's vault model does not: there is no canonical source of truth. Once a component is copied into a consumer's project, the consumer owns the drift. When shadcn ships an accessibility fix, every consumer project is independently out of date.

PointSav's Git-tracked vault with DTCG tokens + per-brand themes + design-decision research is a *canonical upstream* that the consumer forks from. The fork model means the consumer owns their version AND can pull upstream diffs. This is architecturally stronger than shadcn's pure-copy model: it's the difference between a Git fork and `cp -r`. PointSav should make the Git-fork-from-vault path as clean as the shadcn-CLI-copy path.

---

## §7 Federation / portability — moat absences

### Where the migration story is weak across the field
Every major design system surveyed has the same structural portability problem: the source of truth lives in a tool the customer does not own.

- **Figma-sourced systems** (Polaris, Atlassian, Gestalt, Untitled UI): tokens live in Figma. If Figma raises prices, changes the Variables API, or is acquired, the design system's source of truth is at risk.
- **Storybook-documented systems**: documentation lives in a format requiring Node.js, a build step, and a deployment pipeline. Self-hosting is nominally possible but operationally burdensome.
- **npm-delivered systems** (Polaris, Primer, Carbon): component code lives in a vendor's npm registry. A consumer who forks must maintain a separate npm scope, retool CI/CD, and manage peer dependency conflicts.

None of the surveyed systems offers a clean "download your entire design system as a Git repository and run it yourself" path.

### Where the absence-of-moat is sharpest
The weakest point in the field's portability story is **token provenance**: no major system makes it easy to answer "why does this token have this value?" in a machine-readable, auditable way. Figma stores tokens as Figma Variables, which are opaque to anything outside Figma's ecosystem. DTCG specifies format but not provenance. DESIGN.md (Google, April 2026) is the first attempt to standardise rationale alongside tokens, and it is 8 days old as of this writing.

PointSav's vault stores design-decision research markdown alongside DTCG tokens, in Git, with commit history providing change provenance. This is a structural moat that no SaaS platform can match: the audit trail lives in the customer's own Git history, signed with the customer's own keys, under the customer's own control.

### What PointSav's customer-owned-Git story leapfrogs
1. **Figma dependency**: Customer never loses tokens if Figma changes pricing or API.
2. **SaaS lock-in**: The design system binary runs on any Linux VM. Customer can move hosting without touching design data.
3. **Framework lock-in**: HTML + CSS + ARIA recipes are framework-agnostic. The customer's React team, Vue team, and iOS team all consume the same source.
4. **Vendor opacity**: Every token value has a Git commit explaining why it exists. Every theme change is a signed diff. Audit is inherent, not bolted on.

---

## §8 LEAPFROG TARGETS — 10 specific structural innovations for 2030

---

### L1 — Live DTCG Token HTTP API with Content-Type negotiation

**Title**: Live per-component and per-theme DTCG endpoint

**Rationale**: No major design system — as of April 2026 — exposes a live HTTP endpoint that returns DTCG-format tokens for a named component or theme. Every existing system delivers tokens as build artifacts (static JSON files, npm packages). A live endpoint allows AI agents and build tools to query the canonical token store at decode time rather than at build time, enabling always-current code generation.

**Specific implementation surface**:
- `GET /api/tokens/<theme>.dtcg.json` — full theme token tree in DTCG 2025.10 format, `Content-Type: application/design-tokens+json`
- `GET /api/tokens/component/<name>.dtcg.json` — component-scoped tokens only
- `GET /api/tokens/<theme>/diff?from=<git-sha>&to=<git-sha>` — token diff between two vault revisions, DTCG format
- `GET /api/tokens/<theme>/resolved.css` — CSS custom properties, computed from the DTCG tree, ready for `<link>`

**Why hyperscalers and SaaS platforms cannot match it**: These endpoints serve from a Git-tracked vault where every value has provenance. SaaS platforms would need to replicate the vault structure and expose it; none have done so. Hyperscaler design systems (Material, Carbon) are not in the hosting business — they cannot serve per-customer token stores at a per-tenant URL.

**Estimated implementation cost**: Small (the vault already stores DTCG; this is a read path + serialisation handler in Axum).

---

### L2 — MCP JSON-RPC server at the showcase endpoint

**Title**: Native MCP server co-located with the design showcase

**Rationale**: Figma's MCP server is the only major design-system MCP surface as of April 2026, and it requires Figma. A design system that speaks MCP at its own HTTP endpoint becomes queryable by every MCP-capable AI coding agent — Cursor, Claude Code, VS Code Copilot, Windsurf — without any design-tool dependency.

**Specific implementation surface**:
- MCP JSON-RPC 2.0 at `POST /mcp` (or via SSE transport at `GET /mcp/sse`)
- Tools exposed: `list_components()`, `get_component_recipe(name, theme)`, `get_tokens(theme, scope?)`, `search_components(query)`, `get_design_decision(token_name)`, `get_accessibility_notes(component_name)`
- Resources exposed: the full DTCG token tree per theme, the component recipe library
- Prompts: "generate a Button for theme X" returns the recipe + usage context as an LLM-ready prompt block

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscaler design systems do not self-host; they cannot wire an MCP server into their existing documentation CDN. SaaS platforms (Untitled UI, Supernova, Zeroheight) could add MCP, but only to serve their own token format and only while the customer pays. PointSav's MCP server serves from the customer's own Git vault — the source of truth is customer-owned.

**Estimated implementation cost**: Medium (Axum MCP server implementation; the hard part is the tool schema and response formatting, not the transport).

---

### L3 — shadcn-compatible registry endpoint for v0 / AI-codegen ecosystem

**Title**: `/r/registry.json` shadcn-schema-compatible registry

**Rationale**: shadcn's registry format is the de-facto standard for "design system as code distribution" in 2026. Conforming to the schema gives PointSav zero-marginal-cost integration with v0 (Vercel), shadcn CLI, and any MCP client built on the shadcn registry protocol.

**Specific implementation surface**:
- `GET /r/registry.json` — conforming to the shadcn registry schema, listing all available components with metadata
- `GET /r/<component>.json` — conforming to the registry-item schema, with files array containing HTML+CSS+ARIA recipe as the `content` field (and optionally React/Vue/Svelte adaptations)
- The MCP server at `/mcp` additionally exposes the registry as a resource, making v0's registry-MCP integration path work automatically

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscalers do not publish in the shadcn registry format because they are React-component-library vendors, not format-agnostic recipe stores. SaaS platforms could publish registry.json files but cannot bundle the actual component source into the response in the same way — their business model depends on the customer staying on the SaaS platform, not on distributing source code they can self-host.

**Estimated implementation cost**: Small-medium (registry.json is a mechanical serialisation of the existing component inventory; the main work is the recipe-to-registry-item format mapping).

---

### L4 — DESIGN.md export endpoint (Google's new open spec)

**Title**: `GET /api/design-<theme>.md` — DESIGN.md format export

**Rationale**: Google open-sourced the DESIGN.md specification on 21 April 2026 (7 days before this research). It is the first standard that bundles machine-readable tokens (YAML frontmatter) with human-readable design rationale (markdown). PointSav's vault already stores both; exporting DESIGN.md is a thin serialisation layer. Being the first non-Google design system to expose a live DESIGN.md endpoint puts PointSav at the front of an emerging ecosystem.

**Specific implementation surface**:
- `GET /api/design-<theme>.md` — DESIGN.md for the named theme, with YAML token frontmatter + all eight required markdown sections populated from vault data
- `GET /api/design-<theme>.md?format=dtcg` — the token frontmatter rendered as DTCG JSON instead of DESIGN.md YAML (for tooling that wants the rationale prose but a different token format)
- Include a `Link: <rel=alternate>` header pointing to the DTCG endpoint so tool discovery works automatically

**Why hyperscalers and SaaS platforms cannot match it**: The DESIGN.md spec requires *rationale prose* alongside tokens. No hyperscaler design system stores rationale in machine-readable form. SaaS platforms store tokens but not rationale. PointSav's vault stores both, making this endpoint a trivial serialisation of existing data rather than a net-new authoring effort.

**Estimated implementation cost**: Small (YAML frontmatter generation from DTCG + markdown section generation from existing rationale docs).

---

### L5 — Git-provenance audit trail for every token value

**Title**: Per-token provenance at `GET /api/tokens/<theme>/<token-name>/history`

**Rationale**: No design system in the survey exposes a machine-readable audit trail showing *why a token changed* and *who decided it*. Git already provides this for PointSav's vault. Surfacing it via HTTP makes the audit trail consumable by compliance tooling, AI agents, and design-system governance workflows.

**Specific implementation surface**:
- `GET /api/tokens/<theme>/<token-path>/history` — JSON array of `{sha, date, author, commit_message, old_value, new_value}` for the named token, reading from the vault's Git history
- `GET /api/tokens/<theme>/<token-path>/rationale` — returns the associated design-decision research markdown snippet (the "why" file from the vault)
- These endpoints together constitute a design-decision ledger that satisfies the WORM/audit requirements in `conventions/worm-ledger-design.md`

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscaler systems do not expose commit-level token history via HTTP. SaaS platforms store token history in their own databases, which the customer does not own and cannot export. PointSav's vault is the customer's Git repository; the history is the customer's property by definition.

**Estimated implementation cost**: Medium (Axum handler reads `git log -- <token-file>` and parses diffs; the main work is efficient diff parsing for large token trees).

---

### L6 — Per-brand theme fork workflow with upstream-pull

**Title**: `GET /api/fork/<theme>` — theme fork as a Git-repository-ready archive

**Rationale**: The shadcn model (copy-paste) and the npm model (version dependency) both have the same weakness: there is no structured path to receive upstream improvements while preserving downstream customisations. Git branching is the correct answer; PointSav should make "fork this theme" a first-class operation that produces a Git repository the customer can pull from the canonical upstream.

**Specific implementation surface**:
- `GET /api/fork/<theme>` — returns a `.tar.gz` of the vault subtree for the named theme, structured as a valid Git repository with `remote.origin` pointing at `design.pointsav.com/git/<theme>.git`
- `GET /git/<theme>.git` — bare Git endpoint serving the theme vault over HTTP (git-smart-HTTP protocol), enabling `git pull upstream main` from any customer fork
- GUIDE documentation in the catalog entry explains the fork → customise → upstream-pull workflow

**Why hyperscalers and SaaS platforms cannot match it**: No SaaS design system can provide a Git remote that the customer owns and can pull from. Hyperscaler design systems are single-tenant (the system is theirs, not the customer's). PointSav's model — one canonical upstream served over git-smart-HTTP, customer forks as Git repositories — requires no SaaS infrastructure beyond the Axum binary.

**Estimated implementation cost**: Large (git-smart-HTTP protocol in Axum is non-trivial; git2-rs or a subprocess to `git upload-pack` is the implementation path).

---

### L7 — Accessibility compliance check endpoint (WCAG + token audit)

**Title**: `GET /api/audit/wcag?theme=<theme>` — automated WCAG contrast + token compliance report

**Rationale**: DESIGN.md's CLI includes a WCAG contrast checker. No surveyed design system exposes this as a live HTTP API endpoint that runs against a named theme. An SMB that forks the PointSav design system and customises tokens can hit this endpoint to verify WCAG AA/AAA compliance before deploying — without installing a CLI, running a build, or hiring a specialist.

**Specific implementation surface**:
- `GET /api/audit/wcag?theme=<theme>` — runs WCAG contrast ratio checks across all colour token pairs in the theme, returns JSON `{pass: [], fail: [], warnings: []}` with token names, computed contrast ratios, and WCAG level (AA / AAA / fail)
- `GET /api/audit/tokens?theme=<theme>` — validates that all semantic tokens resolve to defined global tokens (no dangling aliases), all required token categories are present, no deprecated token names are used
- Both endpoints return a machine-readable report suitable for CI/CD integration (`exit 1` on fail) or AI-agent consumption

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscaler systems run these checks internally; they do not expose them as per-customer HTTP endpoints. SaaS platforms (Supernova, Zeroheight) have token management features but not live WCAG audit APIs. PointSav runs the audit against the customer's Git-tracked vault, which means results are reproducible, audit-logged, and tied to specific vault commits.

**Estimated implementation cost**: Medium (WCAG contrast ratio computation is well-understood; the work is wiring it to the token resolver and formatting the JSON output).

---

### L8 — Declarative multi-brand theme composition

**Title**: Theme arithmetic: `GET /api/themes/compose?base=<theme>&override=<theme>` 

**Rationale**: Every SMB that runs multiple brands (a holding company, a franchise operator, a white-label reseller) needs to maintain base tokens shared across brands and per-brand overrides. No surveyed design system exposes theme composition as a live, queryable operation. PointSav's DTCG token aliasing model makes this possible at the server side without requiring the customer to manually merge JSON files.

**Specific implementation surface**:
- `GET /api/themes/compose?base=woodfine&override=woodfine-seasonal` — returns a fully resolved DTCG token tree where `override` values replace `base` values, with composition provenance metadata (`{value, source_theme, override_depth}`)
- `POST /api/themes/compose` — body contains an ordered array of theme names; server returns the composed tree (enables three-way composition: global-base + tenant-brand + campaign-theme)
- The resulting token tree can be served back through any other `/api/tokens/` endpoint using a `composed` pseudo-theme name that is session-scoped

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscaler systems serve one brand. SaaS platforms that support multi-brand (Supernova's multi-brand feature) require a paid enterprise tier and keep composition logic on their servers. PointSav's composition runs server-side from the customer's vault, with no per-composition SaaS fee.

**Estimated implementation cost**: Medium (DTCG alias resolution is already required for token rendering; composition is an extension of the resolver that takes a priority list of token trees rather than a single tree).

---

### L9 — Component recipe diff endpoint for migration tooling

**Title**: `GET /api/components/<name>/diff?from=<version>&to=<version>` — structured component change report

**Rationale**: When a design system releases an update, the hardest work is understanding what changed and what to update in consuming codebases. No design system exposes this as a machine-readable API. PointSav's Git-tracked vault means every component version is a Git commit, and diffs are inherent.

**Specific implementation surface**:
- `GET /api/components/<name>/diff?from=<git-sha-or-tag>&to=<git-sha-or-tag>` — JSON report of what changed in the component recipe: which tokens changed, which HTML attributes changed, which ARIA roles changed, what was added/removed
- `GET /api/components/<name>/migration-guide?from=<tag>&to=<tag>` — human-readable (and AI-readable) migration guide auto-generated from the diff, formatted as DESIGN.md-compatible markdown
- AI agents querying via MCP can invoke `get_component_migration(name, from, to)` to receive the same data as a tool response

**Why hyperscalers and SaaS platforms cannot match it**: No hyperscaler or SaaS design system surfaces migration diffs as a queryable API. They publish changelogs as blog posts. PointSav's Git-backed vault makes per-component diffs a read operation, not an editorial effort.

**Estimated implementation cost**: Medium (git diff parsing for component files; the challenging part is structured diff output for token changes vs. HTML changes vs. ARIA changes).

---

### L10 — Offline-first binary distribution (zero runtime dependency)

**Title**: `GET /api/bundle/<theme>.tar.gz` — complete offline-deployable design system

**Rationale**: Every SaaS design-system platform requires an internet connection to function. For SMBs in regulated industries (healthcare, legal, financial services) that need to demonstrate data sovereignty or operate air-gapped, this is a disqualifying constraint. PointSav's Axum binary already runs offline; the bundle endpoint makes the *entire design system* portable as a single file.

**Specific implementation surface**:
- `GET /api/bundle/<theme>.tar.gz` — archive containing: the Axum binary, the vault for the named theme, a systemd unit file, and a quickstart README. Customer unpacks and runs; design system is live at `localhost:8080` in under 60 seconds.
- `GET /api/bundle/<theme>.tar.gz?include=all-themes` — full multi-tenant vault for operators who run multiple brands
- The bundle is a complete, reproducible snapshot of the design system at the vault's current Git HEAD, with the SHA embedded in the bundle manifest

**Why hyperscalers and SaaS platforms cannot match it**: Hyperscalers are not in the self-hosted software business. SaaS platforms cannot distribute themselves as a binary — their value is the hosted service. PointSav's Rust binary + systemd pattern means the entire delivery surface fits in a single archive that a non-technical SMB administrator can deploy in minutes.

**Estimated implementation cost**: Medium (bundle assembly is straightforward; the main work is ensuring the binary and vault are hermetically packaged and that the systemd unit is correctly parameterised at bundle time).

---

## §9 What NOT to copy from any surveyed system

### Storybook as the public documentation surface
Storybook is a component development workshop. Design systems that use Storybook as their public docs are optimising for the engineers who build the system, not the engineers who consume it — and certainly not designers or PMs. PointSav's custom Axum showcase is the right call.

### Material's heavy framework opinion
Material 3's web implementation uses Lit (Google's web components library). This creates an implicit framework dependency even though web components are nominally framework-agnostic. PointSav's HTML+CSS+ARIA recipe model avoids this: the recipe is framework-agnostic by construction, and framework-specific wrappers are generated *from* the recipe, not co-authored *with* it.

### Polaris's Shopify-platform coupling
Polaris tokens encode commerce semantics that are meaningful only in the Shopify Admin context (`--p-color-interactive-selected`). This is correct for Polaris — it is a vertical system — but the lesson is negative: do not encode business-domain semantics into the token names themselves unless the design system is intentionally vertical. PointSav serves general SMB use cases. Tokens should use generic semantic names (`--ps-color-action-primary-hover`) that any business domain can layer meaning onto.

### Base Web's runtime theme-object pattern
Base Web's per-component theme injection (a complete theme object passed at render time) solved the white-labeling problem at the cost of an exploding API surface. DTCG token aliasing (semantic tokens alias to global tokens; themes override at the global level) is the correct abstraction. Runtime theme objects as a component API are an anti-pattern.

### Figma as the source of truth
Every design system that uses Figma as its canonical token store — Polaris, Atlassian, Gestalt, Untitled UI — has the same structural dependency: a SaaS tool the customer does not own. PointSav's Git-tracked vault is the source of truth. Figma (or Penpot, or any future design tool) is a downstream consumer of tokens, not an upstream source.

### npm as the only distribution channel
Delivering components exclusively as an npm package requires the consumer to have a Node.js toolchain, a bundler, and a JS framework. This excludes PHP shops, Django shops, static site generators, and any HTML+CSS team. PointSav's recipe model plus the registry/API distribution channels make npm one optional output format among many.

### Undocumented design rationale
Every surveyed system treats design decisions as internal knowledge. Tokens are values without explanation. DESIGN.md is the first attempt to standardise rationale; PointSav's vault already stores it. Do not let the vault rationale docs become stale or inconsistent — the rationale is the competitive moat.

---

## §10 References

- Figma MCP Server documentation: https://developers.figma.com/docs/figma-mcp-server/
- Figma Guide to MCP Server: https://help.figma.com/hc/en-us/articles/32132100833559-Guide-to-the-Figma-MCP-server
- Figma Blog — Design Systems and AI/MCP: https://www.figma.com/blog/design-systems-ai-mcp/
- DTCG stable 2025.10 announcement: https://www.w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/
- Design Tokens Community Group: https://www.designtokens.org/
- DTCG Technical Reports 2025.10: https://www.designtokens.org/tr/drafts/
- Style Dictionary DTCG support: https://styledictionary.com/info/dtcg/
- Google Stitch DESIGN.md blog: https://blog.google/innovation-and-ai/models-and-research/google-labs/stitch-design-md/
- DESIGN.md GitHub spec: https://github.com/google-labs-code/design.md/blob/main/docs/spec.md
- Google open-sources DESIGN.md (The Decoder): https://the-decoder.com/googles-open-source-design-md-gives-ai-agents-a-prompt-ready-blueprint-for-brand-consistent-design/
- shadcn MCP server docs: https://ui.shadcn.com/docs/registry/mcp
- shadcn registry.json schema: https://ui.shadcn.com/docs/registry/registry-json
- shadcn CLI 3.0 + MCP changelog: https://ui.shadcn.com/docs/changelog/2025-08-cli-3-mcp
- Vercel v0 design systems: https://v0.app/docs/design-systems
- Vercel v0 AI prototyping blog: https://vercel.com/blog/ai-powered-prototyping-with-design-systems
- Figma design-to-code workflow (Figma blog): https://vercel.com/blog/working-with-figma-and-custom-design-systems-in-v0
- AI-readable design system documentation (Medium/Bootcamp): https://medium.com/design-bootcamp/when-design-system-documentation-becomes-ai-readable-14f7a3180233
- Adobe Spectrum Web Components (GitHub): https://github.com/adobe/spectrum-web-components
- Adobe React Spectrum architecture: https://react-spectrum.adobe.com/architecture.html
- Adobe Spectrum design tokens: https://spectrum.adobe.com/page/design-tokens/
- Storybook vs alternatives 2026: https://dev.to/themachinepulse/storybook-10-why-i-chose-it-over-ladle-and-histoire-for-component-documentation-2omn
- shadcn/ui in 2026 — owning your components: https://dev.to/whoffagents/shadcn-ui-in-2026-why-i-stopped-installing-component-libraries-and-started-owning-my-components-2eel
- Tokens Studio DTCG format: https://docs.tokens.studio/manage-settings/token-format
- Thoughtworks Radar — design system decision records: https://www.thoughtworks.com/radar/techniques/design-system-decision-records
- Design System decision records article (Pierre Bremell): https://medium.com/design-bootcamp/when-design-system-documentation-becomes-ai-readable-14f7a3180233
- Figma-context MCP explained: https://skywork.ai/blog/figma-context-mcp-mcp-server-ai-integration/
- Building a Figma-Driven MCP production pipeline: https://www.francescatabor.com/articles/2026/3/31/building-a-figma-driven-mcp-production-pipeline
- MCP 2026 adoption: https://dev.to/pooyagolchian/mcp-in-2026-the-protocol-that-replaced-every-ai-tool-integration-1ipc
