---
schema: foundry-cluster-manifest-v1
cluster_name: project-bim
cluster_branch: cluster/project-bim
created: 2026-04-28
state: active (Ratified Tetrad 2026-05-03)
doctrine_version: 0.0.12
doctrine_claims_codified: [37, 40, 41]
doctrine_claims_proposed: []

operator: woodfine + pointsav (jointly — leapfrog 2030 BIM platform)
working_pattern: research-then-scaffold
input_shape: open-bim-standards + leapfrog-2030-architecture-source

# Cluster mission + Building Design System pattern + claim #41
# (City Code as Composable Geometry) extracted to manifest-notes.md
# 2026-05-09 to keep this manifest under the 30 KB cap. See
# `manifest-notes.md` sibling for the full strategic context.

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: |
        6 NEW projects: service-materials + service-buildings +
        service-codes (Ring 2 substrate triad) + app-orchestration-bim
        + app-workplace-bim + app-console-bim (apps tier). service-bim
        retired; split into the three service-* crates above. Full
        per-project detail in manifest-notes.md §Vendor focus blocks.
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: cluster-totebox-property/
      tenant: woodfine
      purpose: |
        Per-property BIM archive catalog. Existing folder
        (cluster-totebox-property/ already in repo with README.md +
        README.es.md + guide-deployment.md + guide-provision-node.md
        from prior project-system substrate work). Cluster Task
        extends with: GUIDE-bim-archive-operations.md +
        GUIDE-vault-export.md + GUIDE-bim-code-encoding.md (the
        City-Code-as-Composable-Geometry runbook).
      status: leg-pending — Task drafts inside cluster-totebox-property/
        catalog folder; Master coordinates §11 cross-repo rehoming if
        fan-out
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: gateway-orchestration-bim/
      tenant: woodfine
      purpose: |
        NEW catalog folder for the bim.woodfinegroup.com frontend.
        Cluster Task creates the catalog folder + GUIDE-deploy-bim-
        substrate.md + GUIDE-bim-orchestration-operations.md. Catalog
        does NOT yet exist; Task creates as part of first-iteration
        scope.
      status: leg-pending — Task creates catalog folder + drafts GUIDEs
        inside it
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: gateway-orchestration-bim/
      tenant: pointsav
      purpose: |
        Vendor-side showcase pre-declared. bim.pointsav.com would be
        the canonical-instance host once vendor showcase is opened
        (parallel to design.pointsav.com pattern). Pre-declared so
        the customer/pointsav fan-out is structurally visible from
        cluster creation.
      status: leg-pending — second deployment instance not yet
        provisioned; placeholder
  deployment:
    - path: ~/Foundry/deployments/gateway-orchestration-bim-1/
      tenant: woodfine
      shape: bim-token-catalog-showcase
      hosted_url: https://bim.woodfinegroup.com (LIVE on HTTP since
        2026-04-28T23:46Z; HTTPS pending Master certbot run; mirrors
        proofreader.woodfinegroup.com + design.pointsav.com pattern)
      audience: |
        BIM operators + architects + structural / MEP engineers +
        construction managers — Woodfine's collaborators consuming
        the Building Design System tokens to author IFC files for
        Woodfine projects.
      tenant_relationship: |
        Woodfine deploys; PointSav is the substrate provider. Brand
        on the page is Woodfine ("Powered by PointSav" appears only
        in the footer — same shape as a Stripe customer's dashboard
        relative to Stripe's marketing site).
      product_class: |
        Sibling design-system substrate to design.pointsav.com — both
        are token catalogs / showcases. design.pointsav.com provides
        tokens to UI/UX developers + creative designers; this surface
        provides tokens to BIM operators + architects + engineers.
        Same ARCHITECTURE (DTCG token bundle + component recipes +
        AI-readable research backplane); different AUDIENCE.
      explicitly_NOT: |
        This surface is NOT the operational property dashboard. The
        property archive view (per-property IFC archives, BCF topic
        coordination, IDS validation runs, work-order linkages, lease
        register linkages) lives at app-console-bim querying
        os-console, NOT here.
      runtime_artifacts:
        - app-orchestration-bim binary (Rust + Axum;
          server-rendered HTML; reads BIM extension from
          pointsav-design-system clone on cluster/project-bim)
        - service-buildings + service-materials + service-codes
          (NOT consumed by this surface at v0.0.1; reserved for
          v0.0.2+ if a "show example IFC element / material /
          code-overlay token" reference page lands)
        - service-fs (NOT consumed by this surface at v0.0.1;
          WORM ledger anchoring of token-bundle versions is a
          v0.0.2+ refinement)
        - service-slm (NOT consumed by this surface; Doorman
          /v1/audit_proxy is service-codes' integration, not
          this showcase's)
      v0.0.2_visual_upgrade: |
        Per operator framing 2026-04-28: bim.woodfinegroup.com gets
        an "original copy" of a best-in-class design-system website
        DIFFERENT from design.pointsav.com (which is Carbon-shape).
        Goal: bankers / collaborators / contributors can distinguish
        the two products at thumbnail scale. BB.14 sub-agent
        dispatched 2026-04-28 to recommend the best non-Carbon pick;
        AEC vernacular markers from BB.13 (IFC GUID monospace,
        classification chips, isometric building-mass hero) remain
        structural under whatever pick BB.14 returns.
      status: live on HTTP since 2026-04-28T23:46Z; v0.0.1 binary
        verified (/readyz returns components_count=3, tokens_count=8,
        research_count=3); HTTPS pending Master certbot run; v0.0.2
        visual upgrade pending operator sign-off on BB.14
        recommendation
    - path: ~/Foundry/deployments/cluster-totebox-property-1/
      tenant: woodfine
      shape: bim-property-archive
      vault_layout:
        - vault/ifc/             # Authoritative IFC-SPF files (.ifc — ISO 16739-1:2024); IFC 2x3 + 4 + 4.3 supported
        - vault/elements/        # Per-element YAML sidecars referencing IFC GUIDs + bSDD URIs
        - vault/bcf/             # BCF 3.0 unzipped — per-topic XML + PNG dirs (diff-friendly)
        - vault/ids/             # IDS 1.0 validation contracts (per-jurisdiction overlay; ifctester input)
        - vault/materials/       # Material database (flat files; service-materials)
        - vault/codes/           # Building-codes-as-composable-geometry overlays (service-codes; bsdd-*.json + *.ids + *.ifc fragments per jurisdiction)
        - vault/geometry/        # glTF + CityJSONSeq derivatives (regenerable visualization caches; not canonical)
        - vault/drawings/        # SVG 2D drawings (regenerable; IFC GUIDs in SVG element IDs per ROOT B.V. flatt. pattern)
        - vault/objects/<hash>.json  # Speckle-inspired hash-addressed object store (Merkle DAG semantically identical to git objects)
        - vault/refs/            # Git-style ref pointers into objects/ (branches, tags, HEADs)
      runtime_artifacts:
        - inputs/   (operator-staged IFC ingestion + materials)
        - working/  (Task in-progress builds)
        - exports/  (COBie-compatible spreadsheets via ifccsv;
                     IDS validation reports via ifctester;
                     BCF 3.0 issue exchange ZIPs;
                     CityJSONSeq portfolio exports)
      status: pre-created 2026-04-28 (this provisioning); awaits
        first IFC archive ingestion
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-bim/.claude/drafts-outbound/
      gateway: project-editorial Task (PROSE-* drafts) + project-design
        Task (DESIGN-* drafts)
      planned_topics:
        - TOPIC-flat-file-bim-leapfrog.md          (vendor-public; the strategic pitch from BIM_Buildable Architecture.md; doctrine claim #40 narrative)
        - TOPIC-city-code-as-composable-geometry.md (vendor-public; the leapfrog invention; doctrine claim #41 narrative; "validator vs composer" framing)
        - TOPIC-building-design-system.md          (vendor-public; AEC-equivalent of Carbon; 8 BIM token primitives + 10 universal interface components)
        - TOPIC-bim-tokens-substrate.md            (technical reference: IFC 4.3 anchor + Uniclass 2015 floor + bSDD URI publication mechanism)
        - TOPIC-asset-anchored-bim-vault.md        (technical reference: vault-as-canonical archive layout; Speckle-inspired hash-addressed object store; vendor-obsolescence-survivable)
        - TOPIC-open-bim-regulatory-acceptance.md  (US/EU government project acceptance; standards floor; certifications path; 9-jurisdiction comparison)
        - TOPIC-aec-interface-conventions.md       (Revit/ArchiCAD/Bonsai/BricsCAD universal vocabulary; "muscle memory" floor)
        - TOPIC-property-manager-bim-gap.md        (academic literature + Planon EasyFlow market evidence; Foundry's gap-fill positioning)
      planned_design_drafts:
        - DESIGN-RESEARCH-bim-token-taxonomy.md        (Building Design System token foundation: 8 primitives validated against IFC 4.3)
        - DESIGN-COMPONENT-bim-spatial-tree.md         (universal SpatialTree component; Site/Building/Storey/Space hierarchy)
        - DESIGN-COMPONENT-bim-properties-panel.md     (universal PropertiesPanel component; mode-prop variant)
        - DESIGN-COMPONENT-bim-viewport-3d.md          (universal Viewport3D component; xeokit/@thatopen embed pattern)
        - DESIGN-COMPONENT-bim-view-navigator.md       (universal ViewNavigator; named saved views as tabs)
        - DESIGN-COMPONENT-bim-guid-search.md          (console-unique GuidSearch; FM operator entry point)
        - DESIGN-COMPONENT-bim-audit-log.md            (console-unique AuditLog; activity feed of building's operational life)
        - DESIGN-COMPONENT-bim-code-rs1.md             (the leapfrog invention: city-code-as-composable-geometry component recipe)
      status: leg-pending — Task drafts in .claude/drafts-outbound/;
        project-editorial sweeps PROSE-*; project-design sweeps DESIGN-*

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo (cloned via local filesystem
      2026-04-28; remotes set to admin SSH alias + jwoodfine + pwoodfine
      staging mirrors)
    focus: 6 NEW projects scaffold + Building Design System integration (service-* triad + app-* triad).
  - repo: pointsav-design-system
    role: tokens-source
    path: pointsav-design-system/
    upstream: vendor/pointsav-design-system (cloned via local filesystem
      2026-04-28; same staging setup)
    focus: Building Design System extension — 8 BIM token primitive categories + 10 universal + 4+4 surface-unique components; Uniclass 2015 classification floor. Detail in manifest-notes.md §Clone focus blocks.
  - repo: woodfine-fleet-deployment
    role: customer-fleet
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment (cloned via local
      filesystem 2026-04-28; remotes set to mcorp-administrator SSH
      alias + jwoodfine + pwoodfine staging mirrors)
    focus: Two catalog subfolders — cluster-totebox-property/ (extends GUIDEs) + gateway-orchestration-bim/ (NEW, Task creates folder + drafts GUIDEs).

deployment_instances:
  - ~/Foundry/deployments/gateway-orchestration-bim-1/  (woodfine; bim.woodfinegroup.com target)
  - ~/Foundry/deployments/cluster-totebox-property-1/   (woodfine; per-property BIM archive)
trajectory_capture: enabled (capture-edit hook installed in all three
  sub-clones at provisioning 2026-04-28)

adapter_routing:
  trains:
    - cluster-project-bim     # BIM authoring + AEC interface conventions + flat-file BIM substrate skill — the cluster-specific adapter
    - tenant-pointsav         # vendor-side (vendor showcase + canonical pattern publication)
    - tenant-woodfine         # customer-tier (per-property archives + gateway-orchestration deployment)
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-bim
    - tenant-pointsav
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-design
    why: |
      project-design owns the META-substrate of the design system
      (Carbon baseline + DTCG vault + AI-readable research backplane
      per claim #38). project-bim adds the BIM-SEMANTIC LAYER on top —
      8 BIM token primitives + 18 BIM-specific components + Uniclass
      classification floor. This is the same shape as project-orgcharts
      consuming pointsav-design-system as downstream consumer:
      project-bim is a downstream consumer for the META-substrate but
      is the OWNER of the Building Design System sub-substrate.
    interface: |
      project-design publishes canonical DTCG tokens + Carbon-baseline-
      floor + brand themes via the substrate's DTCG export surface.
      project-bim consumes the DTCG substrate AND publishes BIM-semantic
      extensions (8 BIM token categories + 18 component recipes + AEC
      interface vocabulary). New BIM components flow project-bim →
      project-design via cross-cluster outbox messages; project-design
      sweeps DESIGN-* drafts from drafts-outbound/ via the cluster-
      design-draft-pipeline.
    handoff: |
      project-bim Task drafts a BIM token addition or component recipe;
      project-design sweeps via bin/draft-sweep.sh --gateway design;
      Master ratifies cross-cluster shared-file edits if any.
  - cluster: project-data
    why: |
      service-fs (WORM ledger) is the IFC archival backbone — IFC-SPF
      files + BCF 3.0 ZIPs + IDS 1.0 contracts + per-element YAML
      sidecars all land in the WORM ledger. service-input is the
      ingestion surface — IFC-SPF parser extension joins the existing
      PDF + Markdown + DOCX + XLSX parsers.
    interface: |
      service-fs HTTP API at FS_ENDPOINT (per local-fs systemd unit
      on workspace VM since v0.1.23). service-input MCP server
      consumed by project-bim Task during ingestion; project-data
      reviews + accepts the IFC-SPF parser extension via cross-cluster
      handoff. service-codes consumes service-fs at /v1/append for
      audit-trail of code-overlay versions.
    handoff: |
      project-bim Task extends service-input with IFC-SPF parser
      module (IfcOpenShell 0.8.5 sidecar pattern via Rust subprocess
      call); project-data Task reviews + accepts the extension via
      cross-cluster outbox.
  - cluster: project-editorial
    why: |
      TOPIC + GUIDE drafts route through editorial gateway per
      cluster-wiki-draft-pipeline.md. 8 PROSE-TOPIC drafts named in
      planned_topics above. Bilingual (.es.md) generation via
      Spanish strategic-adaptation pattern per DOCTRINE §XII.
    interface: |
      drop drafts in .claude/drafts-outbound/; project-editorial sweeps
      via bin/draft-sweep.sh --gateway language (default); refines
      against banned-vocab + BCSC + citation registry + bilingual
      pair generation; hands off to content-wiki-documentation Root
      for add-side commit.
  - cluster: project-intelligence
    why: |
      service-slm Doorman dispatches apprenticeship briefs on BIM
      authoring (claim #32 generalized). AS-2 grammar substrate
      (Lark grammar enforcement) consumed for IDS-aware output
      validation: IDS contract → Lark grammar → constrained generative
      output. service-codes consumes Doorman /v1/audit_proxy at
      refinement boundary (project-editorial A-4 pattern).
    interface: |
      capture-edit.py post-commit hook auto-dispatches via /v1/shadow
      when SLM_APPRENTICESHIP_ENABLED=true. service-codes consumes
      /v1/audit/proxy + /v1/audit/capture per service-slm/docs/
      audit-endpoints-contract.md v0.1.0.

standards_floor:
  ifc:
    - IFC-SPF (ISO 16739-1:2024 / IFC 4.3) — authoritative archival format
    - IFC 2x3 + IFC 4 — backward compatibility for ingestion (Revit-exported)
    - IFC 5 / IFCX — alpha; track development; design data model so
      migration is serialization swap not rewrite
  collaboration:
    - BCF 3.0 (XML + PNG ZIP; per-topic directory tree on disk)
  validation:
    - IDS 1.0 (XML; June 2024 final; ifctester via IfcOpenShell 0.8.5)
  classification:
    - bSDD URIs (JSON-LD; dereferenceable); Uniclass 2015 floor
    - OmniClass / UniFormat II / MasterFormat for North America cross-mapping
  asset_handover:
    - COBie via ifccsv (mandatory for GSA, USACE, VA, UK BIM Level 2)
  visualization:
    - glTF 2.0 (ISO/IEC 12113:2022) — regenerable visualization cache, NOT canonical
    - SVG (ISO/IEC 14496-22:2019) — 2D drawing derivative
  urban_context:
    - CityJSON 2.0 + CityJSONSeq (OGC community standard)
  iso_19650_cde:
    - Git-based flat-file repository qualifies as CDE solution under
      technology-neutral standard. Information container metadata:
      UID = commit hash, Status = branch/tag, Revision = git history,
      Classification = directory + YAML, Change History = git log

regulatory_acceptance:
  posture: |
    PointSav's BIM platform is designed for acceptance on US federal
    (GSA, USACE, VA, NAVFAC) and EU member-state government projects.
    The format stack — IFC-SPF + IDS 1.0 + BCF 3.0 + COBie — fulfills
    all mandatory open-standard delivery requirements across US and EU
    jurisdictions. The offline-first, flat-file architecture is the
    only approach that natively satisfies ITAR air-gapped requirements
    for defense projects, EU Data Act data sovereignty requirements,
    HIPAA technical safeguards for VA healthcare facilities, and GDPR
    data residency for EU government clients, without dependency on a
    cloud vendor's contractual assurances. The EUPL-1.2 license is
    OSI-approved, FAR 12.212-compatible, and preferred by EU public
    sector procurement.
  certifications_path:
    - buildingSMART IFC certification (highest-leverage; international acceptance)
    - buildingSMART openBIM software certification (EU procurement scoring weight)
    - CMMC Level 2 readiness documentation (DoD subcontractor positioning)
  jurisdictions_in_scope:
    - US federal: GSA + USACE + VA + NAVFAC (IFC + COBie required)
    - UK: ISO 19650 mandatory; IFC + COBie via UK BIM Framework
    - EU member states: Germany (BMDV Dec 2020 infrastructure),
      Italy (≥€1M from Jan 2025), Spain (BCF mandatory; 2024-2030 plan),
      Denmark (since 2007; world's first), Norway (since 2010),
      Netherlands (Open BIM Infrastructure 2012; CityJSONSeq for 3DBAG),
      Poland (€10M+ 2025 → all by 2030)
    - Singapore CORENET X (mandatory Oct 2026 for new projects);
      IFC-SG schema extension model is the closest precedent for the
      City-Code-as-Composable-Geometry invention
    - Dubai (mandatory since Jan 2024; ISO 19650 + IFC + IDS + BCF)
---

# Cluster manifest — project-bim

Multi-clone N=3 cluster authored under Doctrine v0.0.12 §IV.c +
claim #37 (Project Tetrad Discipline). Three sub-clones in one
cluster directory; one Task session writes to one `.git/index` at
a time.

Proposes ratification of two Doctrine claims at next operator-presence
pass (workspace v0.1.60 candidate):
- Claim #40 — Flat-File BIM Substrate
- Claim #41 — City Code as Composable Geometry

## Scope

A leapfrog-2030 flat-file open-BIM substrate that:

1. **Open BIM standards stack as the canonical archive.** Every
   building's authoritative file format is plain-text or standardised
   binary, readable without a proprietary SDK, and survives the
   vendor that created it by decades. IFC-SPF + IDS 1.0 + BCF 3.0 +
   COBie + glTF 2.0 + CityJSONSeq + per-element YAML sidecars +
   Speckle-inspired hash-addressed object store. The building outlives
   the software; the file format outlives the vendor; the archive
   travels with the land.

2. **Pattern 1 architecture (per `BIM_Buildable Architecture.md`).**
   Tauri 2.10 + Rust shell + xeokit/@thatopen webview + IfcOpenShell
   0.8.5 CLI sidecar. Architects the Rust side from day one with a
   Speckle-inspired flat-file object store so the migration to
   Pattern 2 (all-Rust) is a rendering and parsing swap, not a
   data-model rewrite.

3. **Building Design System (Doctrine claim #38 extension).** 8 BIM
   token primitives anchored to IFC 4.3 entity hierarchy + Uniclass
   2015 classification floor + 18 component recipes covering
   universal AEC interface conventions (10 shared) plus surface-
   specific (4 console-unique + 4 workplace-unique). Designers who
   know Revit/ArchiCAD/Bonsai already know the substrate. The
   Carbon-baseline-floor pattern from project-design extends naturally
   into the BIM domain: same architectural stance, different
   semantic layer.

4. **City Code as Composable Geometry (proposed claim #41 — the
   leapfrog invention).** Cities publish their building codes as
   composable BIM design tokens — bSDD dictionaries (zoning class +
   bylaw property URIs) + IDS 1.0 constraint specifications + IFC
   geometric exclusion-zone fragments (setback volumes, height
   envelopes) + per-jurisdiction overlay composition. Designers
   assemble inside pre-constrained envelopes from the first
   placement. Violations become geometrically impossible by
   construction. No prior art addresses this compositional-first
   framing — twenty years of prior art is post-design validation
   only (Solibri, CORENET X, Archistar, ACCORD AEC3PO, Symbium).
   Foundry's flat-file architecture + Tetrad cluster pattern + cross-
   cluster substrate composition enable this leapfrog naturally.

5. **Government regulatory acceptance.** Structurally favorable
   across US federal (GSA + USACE + VA + NAVFAC) and EU member-state
   mandates. Offline-first architecture is the ONLY approach that
   natively satisfies ITAR air-gapped, EU Data Act data sovereignty,
   HIPAA technical safeguards, and ISO 19650 CDE under technology-
   neutral standard. EUPL-1.2 is OSI-approved + FAR 12.212-compatible
   + EU procurement preferred. No certifications strictly required
   (agencies certify workflows, not tools); buildingSMART IFC
   certification is the highest-leverage path for international
   acceptance.

## Five hyperscaler-incompatible capabilities (proposed claim #40)

Per `BIM_Buildable Architecture.md` (workspace root), five capabilities
that hyperscaler business models structurally prohibit them from
replicating because copying them would destroy what they sell:

1. **Asset-anchored BIM** — The digital twin is a directory of flat
   files signed with the land title. Travels with the property deed
   when the asset changes hands. No multi-tenant SaaS can offer this
   (it would break the tenancy model).

2. **Offline-capable BIM for field use** — Basements, rooftops, remote
   construction sites, air-gapped defence facilities, healthcare
   campuses. ACC, Tandem, iTwin Experience cannot work offline by
   construction. Tauri small binary + Rust static compilation +
   flat-file storage + seL4 security make truly offline-authoritative
   client trivial.

3. **Vendor-obsolescence-survivable BIM** — Buildings live 50+ years;
   Revit's file format lasts three. Hyperscaler twins evaporate the
   moment billing stops. Foundry archives readable without any
   proprietary tool for decades — public-sector BIM (UK Government
   Level 2, US GSA, DoD, VA), cultural heritage custodians, long-
   horizon property owners.

4. **IoT integration directly into the BIM archive** — Per-element
   YAML sidecars ingest sensor readings via local MQTT broker; data
   never leaves the owner's premises. No sensor-count-based token
   charges; GDPR + HIPAA compliant by architecture; the sensor graph
   is versioned alongside the model.

5. **Convergence of BIM + lease register + financial ledger** — Most
   strategically novel. Multi-tenant cloud cannot commingle BIM +
   lease + rent roll in a single owner-controlled archive
   (commercial confidentiality, data residency, financial audit, and
   multi-tenant isolation all prevent it). For a property owner,
   these are the same asset. PointSav's existing app-workplace family
   (memo, presentation, proforma) + project-bookkeeping
   (claim #36 Data Vault Bookkeeping Substrate) extends naturally to
   BIM. The Totebox Archive becomes the first data architecture
   where a building's legal, financial, spatial, and operational
   identity are one portable artifact.

## Project tetrad satisfaction

This cluster will (per claim #37 Tetrad Discipline) ship four legs
together at every milestone:

1. **Vendor leg** — code in `pointsav-monorepo` (6 NEW projects above)
   + Building Design System extension in `pointsav-design-system`
2. **Customer leg** — `GUIDE-*` runbooks in
   `customer/woodfine-fleet-deployment/cluster-totebox-property/` AND
   `customer/woodfine-fleet-deployment/gateway-orchestration-bim/`
   (NEW catalog folder); vendor showcase pre-declared at
   `vendor/pointsav-fleet-deployment/gateway-orchestration-bim/`
3. **Deployment leg** — running instances at
   `~/Foundry/deployments/gateway-orchestration-bim-1/` (frontend) +
   `~/Foundry/deployments/cluster-totebox-property-1/` (per-property
   archive)
4. **Wiki leg** — 8 TOPIC drafts + 8 DESIGN drafts in
   `.claude/drafts-outbound/`, refined by project-editorial gateway
   (PROSE-*) and project-design gateway (DESIGN-*)

Master ratification of any milestone work checks all four legs.

## First-iteration scope (v0.0.1 — operator-overridable target)

Per the operator's "first priority will be to get a bim.woodfinegroup.com
up and running so people can 'see' this idea" framing, mirroring
project-design's first-iteration pattern:

- `app-orchestration-bim` Rust scaffold serving Building Design System
  showcase (8 token categories + 10 universal components + first BIM
  component recipes) at port 9096 (next available local port after
  9080-9095 range)
- nginx vhost for `bim.woodfinegroup.com → 9096`
- Initial token import: 8 BIM primitive categories + Uniclass 2015
  classification floor + IFC 4.3 entity hierarchy reference data
- One brand override theme: pointsav-brand (extending project-design's
  existing pointsav-brand)
- Three AI-readable research files at deployment instance research/:
  - `bim-design-philosophy.md`
  - `aec-muscle-memory-rationale.md`
  - `flat-file-vs-cloud-bim.md`
- One GUIDE: `GUIDE-deploy-bim-substrate.md`
- TLS via certbot once DNS resolves
- 8 PROSE-TOPIC drafts staged in drafts-outbound/ for project-editorial
  pickup (substrate-explainer batch)
- 8 DESIGN drafts staged in drafts-outbound/ for project-design pickup
  (Building Design System batch)

This is the v0.0.1 milestone for the cluster. City-Code-as-Composable-
Geometry first-deployment (single Woodfine BC zoning district encoded
as bsdd + IDS + IFC fragments) is the v0.0.2 milestone — 6-8 weeks
per sub-agent B's roadmap. Deeper FIGMA interop, Penpot interop,
xeokit integration, IfcOpenShell sidecar, full Revit IFC round-trip
fidelity come in subsequent milestones.

## Cross-references

- Doctrine claim #37 (Tetrad): `~/Foundry/DOCTRINE.md` §III row 37
- Doctrine claim #38 (Design System Substrate): `~/Foundry/DOCTRINE.md` §III row 38
- Doctrine claim #36 (Data Vault Bookkeeping Substrate): `~/Foundry/DOCTRINE.md` §III row 36
- Strategic source: `~/Foundry/BIM_Buildable Architecture.md` (96 lines, April 2026)
- Sub-agent A research: `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md` (414 lines)
- Sub-agent B research: `~/Foundry/.claude/sub-agent-results/B-bim-city-code-as-geometry-2026-04-28.md` (376 lines)
- Sub-agent C research: `~/Foundry/.claude/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md` (460 lines)
- Cluster predecessor patterns:
  - `~/Foundry/clones/project-bookkeeping/.claude/manifest.md` (claim #36; READ/PRODUCTIVE split)
  - `~/Foundry/clones/project-design/.claude/manifest.md` (claim #38; design-system substrate)
  - `~/Foundry/clones/project-orgcharts/.claude/manifest.md` (downstream consumer of pointsav-design-system)
- Cross-cluster handoff destinations:
  - `~/Foundry/clones/project-design/.claude/inbox.md` (heads-up sent at first ratification commit; project-bim consumes META-substrate, owns BIM-semantic sub-substrate)
  - `~/Foundry/clones/project-data/.claude/inbox.md` (service-fs + service-input extension proposal)
  - `~/Foundry/clones/project-editorial/.claude/inbox.md` (8 PROSE-TOPIC drafts queued)
  - `~/Foundry/clones/project-intelligence/.claude/inbox.md` (AS-2 grammar consumption + audit_proxy use case)
- Open BIM standards references:
  - IFC 4.3 / ISO 16739-1:2024: https://www.iso.org/standard/84123.html
  - IDS 1.0: https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specification-ids/
  - bSDD: https://www.buildingsmart.org/users/services/buildingsmart-data-dictionary/
  - BCF 3.0: https://technical.buildingsmart.org/standards/bcf/
  - Singapore CORENET X: https://info.corenet.gov.sg/
  - EU ACCORD AEC3PO: https://accordproject.eu/
  - IfcOpenShell 0.8.5: (April 2026 release)
  - ThatOpen Company web-ifc: https://github.com/ThatOpen/engine_web-ifc
  - xeokit-sdk: https://xeokit.io/
  - Speckle: https://speckle.systems/
