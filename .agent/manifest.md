---
schema: foundry-cluster-manifest-v1
cluster_name: project-design
cluster_branch: cluster/project-design
created: 2026-04-28
state: active
doctrine_version: 0.0.11
doctrine_claims_codified: [38, 37]

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: |
        2 projects in scope (Tetrad-vendor leg) —
          * app-privategit-design (NEW; rename from existing app-privategit-design-system —
            Scaffold-coded → Active per CLAUDE.md §9. The productized substrate
            SMB customers self-host. Yew/Leptos web app over an Axum backend
            speaking design-tokens.json + DTCG to FIGMA, Penpot, Sketch via
            local plugins. AI-readable design-decision research surface.)
          * os-privategit (Scaffold-coded → Active. Operating system that hosts
            app-privategit-design + app-privategit-source-control as a single
            cohesive deployment artefact. Boots on either bottom per claim #34.)
        Codifies Doctrine claim #38 (Design System Substrate).
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: vault-privategit-design/
      tenant: pointsav
      purpose: |
        Vendor-side showcase + canonical-instance host. design.pointsav.com goes
        live from the vault-privategit-design-1 instance running on this VM.
        GUIDE-deploy-design-substrate.md + GUIDE-figma-interop.md +
        GUIDE-carbon-baseline-import.md + GUIDE-customer-fork-procedure.md.
      status: leg-pending — Task drafts; first GUIDE = deploy guide for design.pointsav.com launch
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: vault-privategit-design/
      tenant: woodfine
      purpose: |
        Customer-tier mirror — Woodfine's own design system instance once they
        opt in. Same catalog name; different tenant = vault-privategit-design-2
        when provisioned. Pre-declared so the fan-out is structurally visible.
      status: leg-pending — second deployment instance not yet provisioned; placeholder
  deployment:
    - path: ~/Foundry/deployments/vault-privategit-design-1/
      tenant: pointsav
      shape: design-system-canonical
      vault_layout:
        - tokens/      (DTCG-format design tokens — primitive + semantic + component layers; Carbon baseline imported as floor)
        - components/  (component recipes — HTML+CSS+ARIA semantic units; no JS framework lock-in)
        - themes/      (per-brand override layers — pointsav-brand initially; SMB customer themes when forked)
        - research/    (AI-readable design-decision rationale, accessibility notes, brand-voice rules — TOPIC-style files AI agents read at codegen time)
        - exports/     (rendered output — Figma plugin imports, JSON DTCG bundles, CSS variables, Tailwind config, Style Dictionary builds)
      runtime_artifacts:
        - inputs/  (operator-staged design-system additions before substrate ingestion)
        - working/ (Task in-progress drafts)
      hosted_url: https://design.pointsav.com (target; goes live with first iteration ratification)
      status: pre-created 2026-04-28 (v0.1.47); awaits first build + first deploy
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-design/.claude/drafts-outbound/
      gateway: project-language Task
      planned_topics:
        - TOPIC-design-system-substrate.md           (vendor-public knowledge of the substrate pattern; doctrine claim #38 narrative)
        - TOPIC-carbon-baseline-import.md            (technical reference: how Carbon's primitive + component vocabulary becomes the floor layer)
        - TOPIC-dtcg-editor-agnostic-tokens.md       (FIGMA / Penpot / Sketch / hand-author all interop via DTCG)
        - TOPIC-ai-readable-design-research.md       (the McLuhan-medium-as-message angle — AI codegen consumes design-decision research)
        - TOPIC-self-host-vs-hyperscaler-design-systems.md (landscape contrast: Carbon, Material, Fluent, Polaris, Spectrum, Untitled UI)
      status: leg-pending — Task drafts in .claude/drafts-outbound/; project-language sweeps + refines

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo (cloned via local filesystem; remotes set to admin SSH alias)
    focus: app-privategit-design (rename target) + os-privategit
  - repo: pointsav-design-system
    role: tokens-source
    path: pointsav-design-system/
    upstream: vendor/pointsav-design-system (cloned via local filesystem from project-orgcharts; remotes set)
    focus: |
      The CANONICAL TOKENS SOURCE — DTCG-format tokens + brand themes + Carbon-baseline imports
      live here. project-orgcharts is a downstream consumer; project-design owns the META-substrate
      around the tokens. See cross-cluster handoff entry below.
  - repo: pointsav-fleet-deployment
    role: vendor-fleet
    path: pointsav-fleet-deployment/
    upstream: vendor/pointsav-fleet-deployment (cloned from project-knowledge; remotes set)
    focus: vault-privategit-design/ catalog GUIDE drafts for the design.pointsav.com showcase

deployment_instance: ~/Foundry/deployments/vault-privategit-design-1/
trajectory_capture: enabled (capture-edit hook installed in all three sub-clones at provisioning)

adapter_routing:
  trains:
    - cluster-project-design  # design-substrate authoring skill — DTCG token construction, Carbon-baseline composition, brand-override overlay, AI-readable research-file authoring, FIGMA/Penpot/Sketch interop
    - tenant-pointsav         # Vendor-side brand = pointsav (this instance)
    - tenant-woodfine         # second declared tenant for the customer-leg fan-out
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-design
    - tenant-pointsav
    - role-task

cross_cluster_dependencies:
  - cluster: project-orgcharts
    why: |
      project-orgcharts also clones pointsav-design-system but uses it as a downstream consumer
      (extracts org-chart components into the design system). project-design OWNS the META-substrate
      (tokens / themes / templates / Carbon baseline / research backplane). Coordinate to avoid:
      (a) competing edits to the same primitive tokens, (b) duplicate component naming, (c)
      incompatible theme hierarchies. project-design heads-up sent on cluster-creation
      (2026-04-28) — see project-orgcharts inbox.
    interface: |
      project-design publishes the canonical tokens.json + themes/ + components/recipes via
      the substrate's DTCG export surface. project-orgcharts consumes via DTCG import. New
      org-chart-specific components flow project-orgcharts → project-design via cross-cluster
      handoff outbox messages.
    handoff: |
      Either cluster's Task drafts a component or token addition; the other reviews via outbox;
      Master ratifies if cross-cluster touches a shared file. For pure tokens, project-design owns;
      for usage-specific components, project-orgcharts owns.
  - cluster: project-language
    why: TOPIC drafts route through editorial gateway per cluster-wiki-draft-pipeline.md
    interface: drop drafts in .claude/drafts-outbound/; project-language sweeps + refines
  - cluster: project-slm
    why: |
      service-slm Doorman dispatches apprenticeship briefs on design work (claim #32). For AI-readable
      design research, the Doorman is the consumption point — AI agents querying design.pointsav.com
      route through the Doorman, which reads vault-privategit-design-1's research/ files at decode
      time. Adapter Composition Algebra (claim #22) applies: base ⊕ tenant ⊕ design-system adapter
      composes the per-tenant design-aware UI generation regime per request.
    interface: |
      service-slm reads research/*.md from the deployment instance directly (read-only). Future
      enhancement: Doorman exposes /v1/design-system/<tenant> endpoint that returns the DTCG token
      bundle + research summaries to AI agents requesting design context.
---

# Cluster manifest — project-design

Multi-clone N=3 cluster authored under Doctrine v0.0.11 §IV.c +
claim #38 (Design System Substrate). Three sub-clones in one
cluster directory; one Task session writes to one `.git/index` at
a time.

## Scope

A leapfrog-2030 self-hosted design-system substrate that:

1. **Per-tenant ownership.** Each SMB customer (or vendor org)
   owns their design system as a Git repo with vault-as-canonical
   structure (tokens / components / themes / research /
   exports). No hyperscaler SaaS holding the SMB's design system
   hostage.
2. **AI-readable research backplane.** Design decisions,
   accessibility notes, brand-voice rules, component
   justifications live as TOPIC-style markdown files AI agents
   read at codegen time. The McLuhan angle: in the AI era, the
   substrate IS the SMB's research deliverable, structured for
   AI consumption. The "medium is the message" — the well-
   structured design system substrate is the message SMBs send
   to their (human OR AI) implementation partners.
3. **Editor-agnostic via DTCG.** FIGMA, Penpot, Sketch, hand-
   authoring all interop via the W3C Design Tokens Community
   Group format. No editor lock-in; the substrate doesn't care
   which tool produced the tokens.
4. **Carbon muscle-memory floor + brand override layer.** IBM
   Carbon's component vocabulary becomes the BASE LAYER imported
   into every new instance. SMB customers extend with brand-
   specific overrides without re-learning a new component
   taxonomy. Designers who know Carbon already know the
   substrate.
5. **Productized as `app-privategit-design`.** The same code
   that runs design.pointsav.com runs at every SMB customer
   site. Self-hosted; no SaaS dependency. Customer can boot the
   instance on either substrate bottom (claim #34).
6. **Showcase = product.** design.pointsav.com is the canonical
   instance AND the product demonstration. SMB customers can
   inspect the running showcase before forking their own.

## Project tetrad satisfaction

This cluster will (per claim #37 Tetrad Discipline) ship four
legs together at every milestone:
1. **Vendor leg** — code in `pointsav-monorepo`
   (`app-privategit-design` + `os-privategit`)
2. **Customer leg** — `GUIDE-*` runbooks in
   `pointsav-fleet-deployment/vault-privategit-design/` (vendor
   showcase) AND `woodfine-fleet-deployment/vault-privategit-design/`
   (customer mirror, when Woodfine opts in)
3. **Deployment leg** — running instance at
   `~/Foundry/deployments/vault-privategit-design-1/` serving
   design.pointsav.com
4. **Wiki leg** — TOPIC drafts in `.claude/drafts-outbound/`,
   refined by project-language, published to
   `vendor/content-wiki-documentation`

Master ratification of any milestone work checks all four legs.

## First-iteration scope

Operator stated: "we need to go live as soon as the first
iteration is ready." First-iteration definition (target):

- `app-privategit-design` Rust scaffold serving DTCG token JSON
  + rendered HTML preview at port 9094 (next available local
  port after the 9080-9092 range)
- nginx vhost for design.pointsav.com → 9094
- Initial token import from Carbon's open-source DTCG export
  (or Carbon-equivalent token primitives — color, type, space,
  motion, focus)
- One brand override theme: pointsav-brand
- Two AI-readable research files: design-philosophy.md +
  carbon-baseline-rationale.md
- One GUIDE: GUIDE-deploy-design-substrate.md
- TLS via certbot once DNS resolves

This is the v0.0.1 milestone for the cluster. Deeper FIGMA
interop, Penpot interop, customer-fork procedure, AI-decode-
time integration with Doorman come in subsequent milestones.

## Cross-references

- Doctrine claim #38: `~/Foundry/DOCTRINE.md` §III row 38
- Doctrine claim #37 (Tetrad): `~/Foundry/DOCTRINE.md` §III row 37
- Strategic convention: `~/Foundry/conventions/design-system-substrate.md`
- Sub-agent research: `~/Foundry/.claude/sub-agent-results/A5-design-system-landscape-research-2026-04-28.md`
- McLuhan reference: "The Medium is the Message" (Marshall McLuhan, *Understanding Media*, 1964) — the substrate IS the message in AI-era SMB delivery
- Cluster predecessor pattern: `~/Foundry/clones/project-bookkeeping/.claude/manifest.md` (claim #36)
- Cross-cluster handoff destination: `~/Foundry/clones/project-orgcharts/.claude/inbox.md` (heads-up sent 2026-04-28)
