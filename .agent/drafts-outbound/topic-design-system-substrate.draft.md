---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-design
target_repo: content-wiki-documentation
target_path: substrate/   # candidates: substrate/, architecture/, applications/ — project-language decides per its taxonomy
target_filename: design-system-substrate.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-28T01:45:00Z
authored_by: task-project-design (cluster v0.0.1 first iteration)
authored_with: claude-opus-4-7
references:
  - https://design-tokens.github.io/community-group/format/
  - https://carbondesignsystem.com/
  - https://www.w3.org/TR/WCAG22/
  - https://modelcontextprotocol.io/
  - https://penpot.app/
  - https://tokens.studio/
  - clones/project-design/pointsav-monorepo/app-privategit-design/CLAUDE.md
  - clones/project-design/pointsav-design-system/dtcg-vault/research/design-philosophy.md
  - clones/project-design/pointsav-design-system/dtcg-vault/research/carbon-baseline-rationale.md
  - DOCTRINE.md claim #38 (Design System Substrate)
  - DOCTRINE.md claim #36 (Vault-Bookkeeping pattern)
  - DOCTRINE.md claim #34 (Two-Bottoms Sovereign Substrate)
  - DOCTRINE.md claim #33 (Capability Ledger Substrate)
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #28 (Designed-for-Breakout Tenancy)
  - DOCTRINE.md claim #22 (Adapter Composition Algebra)
  - DOCTRINE.md claim #18 (Compounding Substrate)
  - conventions/design-system-substrate.md
  - conventions/project-tetrad-discipline.md
  - mcluhan-medium-message
notes_for_editor: |
  This is the cluster's first wiki contribution under the Tetrad
  Discipline (claim #37). Audience: design-system practitioners +
  financially-literate-non-technical readers (Bloomberg-article
  standard). The strategic claim being made publicly is the
  three-structural-inversions framing — substantive, not marketing.

  §1 is the elevator. §2-4 are the substantive treatment per
  inversion. §5 is the IBM Carbon framing (load-bearing for the
  SMB-practitioner muscle-memory argument). §6 is the AI-readability
  angle (load-bearing for the 18-24 month structural lead claim).
  §7 is the customer-fork shape. §8 is the editor-agnosticism /
  DTCG / Penpot framing.

  Things to preserve in editorial pass:
  - The IBM Carbon discussion. SMB practitioner population data
    (39% at 5,000+ employees per 2026 Design Systems Report) is
    citation-grounded; preserve the count and the citation.
  - The McLuhan reference in §1 / §6. Used once each; not
    repeated; load-bearing for the strategic positioning.
  - The hyperscaler-design-systems list. They are named factually
    (Material is Google's, Carbon is IBM's, etc.) — this is
    structural-positioning per `~/Foundry/CLAUDE.md` §6, not
    competitive contrast.

  Things to pare:
  - Repetition between §1 and §2 — both touch the inversions.
    Project-language picks one or merges per audience.
  - The implementation-detail asides (port numbers, env vars,
    file paths) belong in the GUIDE / engine CLAUDE.md, not in
    a public TOPIC. Strip in pass.

  target_path candidates listed; project-language decides per its
  taxonomy decision. Recommended placement: `substrate/` if that
  category exists (per the taxonomy proposal awaiting operator
  ratification); otherwise `architecture/`.

  Stage-2 craft DPO target: a Creative Contributor edit pass after
  publication adds the opening hook, narrative arc, brand-voice
  graphics, and any layout enhancements the wiki engine supports.
  This draft is at register-correct register; the craft layer is
  separate.
---

# The design-system substrate

## §1 What it is

The PointSav design-system substrate is a self-hosted,
customer-owned design-system engine. The vendor showcase at
`design.pointsav.com` is the canonical instance. Each SMB customer
who forks the substrate runs their own instance at their own
domain. Single codebase, single deployment shape, two contexts —
vendor showcase and customer instance.

Most SMB design systems are hosted on hyperscaler infrastructure.
IBM Carbon Cloud, Google Material Design, Microsoft Fluent,
Salesforce Lightning, Adobe Spectrum, Shopify Polaris all place
the design system inside the hyperscaler's infrastructure.
Enterprise platforms — Specify, Backlight, Knapsack, Tokens Studio
Pro — do the same at $40-100/seat/month entry pricing. The SMB
beneath the enterprise tier has structurally been unserved.

The substrate inverts that pattern on three structural axes:

1. The customer's design system lives in the customer's Git
   repository, signed by the customer's key, replayable into any
   tool. Migration cost falls toward zero — the customer always
   has the source.
2. Design-decision research lives in the same vault as the tokens
   and components, served through the same Model Context Protocol
   (MCP) endpoint AI agents use to query everything else. The
   well-structured substrate IS the message the SMB sends to its
   implementation partners.
3. Tokens are stored in the W3C Design Tokens Community Group
   (DTCG) format. FIGMA, Penpot, Sketch, hand-authoring all
   interop. The substrate stays editor-agnostic by construction.

The structural gap that motivates the substrate is concrete and
recent:

- 39% of design-system practitioners work at companies with
  5,000+ employees (2026 Design Systems Report). The practitioner
  population is concentrated at enterprise scale; SMBs without an
  in-house practitioner have nowhere to source the discipline.
- Knapsack raised $10M Series A in 2025 with explicit enterprise-
  only pricing. Specify, Backlight, Tokens Studio Pro target the
  same tier. None build for SMBs.
- Only 23% of design systems were structured for AI consumption
  as of December 2025. FIGMA shipped its design-system MCP June
  2025, locked to FIGMA cloud + FIGMA-licensed customers. The
  first self-hosted MCP-shipping substrate has an 18-24 month
  structural lead window before mainstream tooling catches up.

The substrate is what fills that gap.

## §2 The vault-as-canonical pattern

The substrate's content lives in a per-tenant vault directory:

```
<tenant-vault>/
├── tokens/        DTCG primitive + semantic + component layers
├── components/    HTML+CSS+ARIA recipe files
├── themes/        per-brand semantic-layer overrides
├── research/      AI-readable design-decision rationale (markdown)
└── exports/       derived caches (Figma, Tailwind, Style Dictionary)
```

The vault is the only canonical layer. Rendered exports — Figma
Variables JSON, Tailwind config, CSS variables, Style Dictionary
builds — are derived caches recomputable from the canonical four
directories above. Migration cost falls toward zero because the
canonical state is what gets committed, reviewed, replicated, and
disclosed.

The substrate engine is a stateless HTTP service that reads the
vault from disk. Restarting the engine re-reads the vault.
Per-tenant isolation is achieved by running one engine process per
tenant, each pointed at its own vault. Multi-tenant fan-out is
operational, not architectural — the architecture is single-tenant
per process, by design.

Persistence happens above the vault filesystem via the substrate's
WORM ledger (service-fs, the workspace's append-only audit
infrastructure). Token and component history is anchored monthly
to Sigstore Rekor, producing a customer-rooted Merkle log apex per
the workspace's Capability Ledger Substrate (claim #33). SMB
customers receive a Trustworthy System Attestation (TSA) every
quarter — the customer's substrate operator signs a report citing
the vault checkpoints, the customer's `allowed_signers` chain, and
the WORM ledger property. No hyperscaler SOC 2 covers this — those
cover the hyperscaler's controls, not the customer's design data.

## §3 AI-readable research backplane

The `research/` directory is the substrate's most novel element.
Every design decision lives as a TOPIC-style markdown file with
explicit structure:

```
---
schema: foundry-design-research-v1
component_or_token: button-primary
decision_type: component-introduction
authored: 2026-04-28
authored_by: design-team-member-name OR ai-agent-id
status: ratified
brand_voice_alignment: [confident, direct, professional]
accessibility_targets: [wcag-2-2-aa, focus-visible]
ai_consumption_hint: "When generating a button for a primary
  action, use this component. When the action is destructive, use
  button-danger instead."
---
```

The frontmatter is machine-readable; the body is prose-readable.
AI agents (Claude artifacts, Vercel v0, GitHub Copilot UI
generation, internal Claude Agent SDK agents) consume the research
through the substrate's MCP endpoint at decode time. JSON-RPC 2.0
over HTTP POST; methods include `list_tokens`, `list_components`,
`list_research`, and `describe`. An AI agent registers the
substrate as an MCP server, then queries it during UI generation
to align with the SMB's brand intent.

The research closes the AI-productivity / brand-coherence
trade-off. Hyperscaler workflows treat AI as substitute for the
lowest-skill design step (suggest a button color); the substrate
treats AI as substitute for the highest-volume design step (apply
established brand intent across thousands of UI surfaces). Creative
Designers operate at their actual leverage point — taste, craft,
brand evolution — rather than re-deciding the same questions every
session.

Hyperscaler design systems publish only the WHAT (token values,
component shapes). They omit the WHY. The substrate publishes
both, in the same machine-readable tier, served through the same
endpoint. That is the structural inversion.

## §4 Editor-agnostic via DTCG

The W3C Design Tokens Community Group format reached its 2025.10
stable specification in October 2025. FIGMA ships native DTCG
import / export November 2026. Penpot (the open-source FIGMA
alternative, BSD-licensed and self-hostable) has been DTCG-native
since Penpot 2.x. Style Dictionary, Specify, Tokens Studio all
consume DTCG. A Git-based DTCG-canonical token store — the
substrate's pattern — is structurally future-proof because every
editor and every downstream tool in the 2026-2030 roadmap
converges on DTCG.

On the consumer side:

- **FIGMA** via Tokens Studio plugin (the dominant DTCG-aware
  FIGMA plugin) or via FIGMA's 2024-2026 native variable system;
  the substrate ships an `exports/figma-variables.json` converter
  for the latter
- **Penpot** imports DTCG natively; the substrate-Penpot pairing
  is the fully-open-source design workflow Foundry recommends to
  SMB customers who do not want Adobe / Figma SaaS
- **Sketch** via the Tokens Studio Sketch plugin
- **Hand-authoring** — designers edit DTCG JSON directly; this is
  also the AI-friendly path, since AI agents author DTCG natively

The customer chooses the editor; the substrate doesn't care.
That is how editor-agnosticism works in practice.

Compare this to enterprise platforms whose commercial incentive is
to lock the customer into their editor. Specify, Backlight, Tokens
Studio Pro all want to be the customer's editor. The substrate has
no such incentive. DTCG is the common denominator and the
substrate stays editor-agnostic by construction.

## §5 IBM Carbon as the floor

The substrate imports IBM Carbon's primitive token vocabulary as
the floor layer of every new instance. Color naming follows
Carbon's convention (`gray-10` through `gray-100`, `blue-10`
through `blue-80`, ...). Type scale follows Carbon's productive +
expressive families. Spacing follows Carbon's 8px grid. Motion
follows Carbon's productive + expressive easing. Focus ring
follows Carbon's WCAG 2.2 AAA-conformant style.

Brand-specific work happens at the semantic layer
(`themes/<brand>/`), not the primitive layer. SMB customers extend
with brand overrides without re-learning a new component taxonomy.

Carbon was chosen for three reasons:

1. **Familiarity surface.** Among the global design-system
   practitioner population, Carbon has the largest accessible-
   design muscle memory. Practitioners who have worked at
   Fortune-500 companies, financial services, healthcare,
   government IT, or education have a high probability of having
   touched Carbon tokens. The naming convention is cognitively
   cheap to recover.
2. **Accessibility floor.** Carbon ships with WCAG 2.2 AAA
   conformance built into its primitive choices. Importing Carbon
   imports the floor for free.
3. **Permissive licensing on the form.** Carbon's token values
   are publicly documented; the IBM Plex font is permissively
   licensed; the *naming convention* is a documentation artefact,
   not a trademarked asset. The substrate references Carbon as a
   vocabulary source, not a brand.

What is NOT imported from Carbon:

- The IBM Cloud-specific themes (Carbon White, Carbon Gray 10,
  ...) — these carry IBM brand framing
- The React-specific component implementations — the substrate is
  framework-agnostic; ships HTML+CSS+ARIA recipes
- The IBM logo and "Carbon" wordmark — trademarked assets
- Carbon-specific component micro-interactions — the substrate's
  recipes use Carbon's motion primitives but compose them per-
  component

Future work may layer alternative primitive bottoms. Untitled UI
ships under MIT-style licensing and carries no brand association;
it is a candidate for the substrate's second primitive bottom in
a subsequent milestone. v0.0.x ships Carbon only.

## §6 The AI codegen frontier

In the AI era of 2026-2030, an SMB's design-system substrate is a
medium. Its form — machine-readability, editor-agnostic interop,
self-hostability, AI-consumable research — shapes how the SMB's
brand reaches every customer-facing surface. Marshall McLuhan's
"the medium is the message" (*Understanding Media*, 1964) named
this property at the level of communications media; the substrate
operationalises it at the level of an SMB's design intent.

What the substrate's AI consumption pattern looks like in practice:

- An AI agent generating UI for an SMB-tenant surface registers
  `https://design.<smb-name>.com/mcp` as a Model Context Protocol
  server in its agent runtime.
- At UI-generation time, the agent calls `list_tokens` to fetch
  the DTCG bundle, `list_components` to discover the SMB's
  component recipes, `list_research` to discover available
  decision rationale.
- For any non-trivial generation step, the agent fetches the
  relevant research entry and reads the brand-voice rules,
  accessibility targets, and anti-patterns before deciding what
  to generate.
- The output aligns with the SMB's brand intent without re-
  deciding the same questions every session.

The pattern is not speculation. The MCP standard reached stable
specification in 2025; FIGMA's June 2025 design-system MCP
demonstrates the model on the FIGMA side. The substrate ports the
pattern onto the customer-owned, editor-agnostic side that FIGMA
explicitly does not address.

The first self-hosted substrate shipping a complete MCP layer
(tokens + component APIs + research TOPICs) has an 18-24 month
structural lead before mainstream tooling catches up. The lead
window is finite; the structural inversions persist beyond it.

## §7 Customer fork procedure

The catalog entry IS the productized substrate. Any SMB customer
can fork:

```
git clone github.com/pointsav/app-privategit-design <smb-name>-design
cd <smb-name>-design

# Build the substrate engine
cargo build --release -p app-privategit-design

# Initialise the vault from the canonical template
mkdir -p /srv/<smb-name>/vault-privategit-design-1
cp -r pointsav-design-system/dtcg-vault/* \
      /srv/<smb-name>/vault-privategit-design-1/

# Edit themes/<smb-brand>.json — re-point semantic references
# at your brand colors, type, motion choices.

# Run bootstrap (parameterised — see infrastructure docs)
sudo bootstrap.sh \
    --tenant <smb-name> \
    --vault-dir /srv/<smb-name>/vault-privategit-design-1 \
    --domain design.<smb-name>.com

# Issue TLS
sudo certbot --nginx -d design.<smb-name>.com ...
```

The customer ends up with a fully self-hosted, customer-owned
design-system substrate. No SaaS dependency. No hyperscaler
runtime. Their `themes/<smb-brand>.json` and any research files
they author are their work product, in their Git repo, signed by
their key.

The fork procedure is the same for the PointSav showcase at
`design.pointsav.com` and any SMB customer instance. Single
codebase, single deployment shape, two contexts.

## §8 What hyperscalers structurally cannot replicate

Some of the substrate's structural choices look superficially
replicable by hyperscaler platforms. They are not, on close
examination:

1. **Per-tenant Git ownership** requires per-tenant signing
   identity (the customer's `allowed_signers` chain — the
   substrate they don't have). Hyperscaler architectures cannot
   accommodate it without becoming a Git hosting provider, which
   they are not.

2. **AI-readable research backplane in customer-owned form**
   requires per-customer hosting. Hyperscaler design systems can
   publish their own design research (Material has
   design.google.com/articles; Carbon has carbondesignsystem.com/
   all-about-carbon). They cannot publish the SMB customer's
   research because they don't host the SMB customer's design
   system. The substrate-customer pairing produces SMB-specific
   research AI agents can consume — structurally inaccessible to
   hyperscalers.

3. **Customer-rooted attestation.** The substrate's TSA pattern
   inherits the workspace's WORM ledger + Sigstore Rekor anchoring
   + per-tenant `allowed_signers`. The customer gets a customer-
   rooted attestation no hyperscaler SOC 2 can match — theirs
   covers their controls; TSA covers the customer's design data.

4. **Editor agnosticism.** Tokens Studio Pro, Specify, Backlight
   all want to be the customer's editor. Their commercial
   incentive is lock-in. The substrate has no such incentive —
   DTCG is the common denominator and the substrate stays
   editor-agnostic by construction.

The four are compounding. Removing any one weakens the others;
keeping all four is what makes the substrate structurally
distinct from the hyperscaler design-system pattern.

## §9 What the substrate is NOT

To prevent confusion:

- The substrate is **not** a Storybook replacement. Storybook is a
  parallel renderer; the substrate owns its rendering. SMB
  customers do not need both.
- The substrate is **not** a FIGMA / Penpot / Sketch competitor.
  Those are design editors; the substrate is the canonical store
  the editors interop with via DTCG.
- The substrate is **not** a SaaS platform. It is self-hosted by
  design. Foundry may later offer a managed-hosting option for
  SMB customers who do not want operations responsibility, but
  the substrate code is always self-hostable.
- The substrate is **not** a JS-framework choice. Components are
  HTML+CSS+ARIA recipes; the customer's chosen framework consumes
  the recipe. React, Vue, Svelte, vanilla — all work.
- The substrate is **not** a container artefact. Per the
  zero-container-runtime convention, the substrate ships as
  native binaries deployed via systemd. No Docker, no Kubernetes,
  no OCI artefacts.

## §10 Where to look next

Operational documentation:

- Substrate engine source —
  `pointsav-monorepo/app-privategit-design/`
- Substrate engine architecture —
  `pointsav-monorepo/app-privategit-design/CLAUDE.md`
- Catalog entry —
  `pointsav-fleet-deployment/vault-privategit-design/`
- Operator runbook —
  `vault-privategit-design/GUIDE-deploy-design-substrate.md`
- Canonical vault template —
  `pointsav-design-system/dtcg-vault/`
- Infrastructure IaC — `infrastructure/local-design/`

Doctrine and convention:

- Doctrine claim #38 — `DOCTRINE.md` §III row 38
- Convention — `conventions/design-system-substrate.md`
- Cluster manifest — `clones/project-design/.claude/manifest.md`

External standards:

- W3C DTCG format — design-tokens.github.io/community-group/format/
- IBM Carbon Design System — carbondesignsystem.com/
- Penpot — penpot.app/
- Tokens Studio for FIGMA — tokens.studio/
- WCAG 2.2 — w3.org/TR/WCAG22/
- Model Context Protocol — modelcontextprotocol.io/
