---
schema: foundry-cluster-manifest-v1
cluster_name: project-marketing
cluster_branch: cluster/project-marketing
created: 2026-05-06
state: active (v0.0.1 MVP shipped 2026-05-06; cargo check clean; bootstrap + deploy pending Master)
doctrine_version: 0.0.14
doctrine_claims_codified: [37, 43, 44]
doctrine_claims_proposed: []

operator: pointsav (Mathew, Jennifer)
working_pattern: production-first-mvp
input_shape: app-mediakit-marketing-as-wordpress-leapfrog

# Cluster mission per operator direction 2026-05-06: build
# app-mediakit-marketing as a Rust server that delivers WordPress.org
# muscle-memory at the user-facing layer (Dashboard, Pages, Posts,
# Media, Themes, Plugins, Settings vocabulary that millions of users
# already know) but with a leapfrog-2030 internal architecture:
#
#   - Rust binary (no PHP, no MySQL — flat-file content + LadybugDB
#     graph for entities)
#   - Sovereign per Compounding Substrate doctrine (no vendor cloud
#     dependency; runs on customer-owned hardware; Tier 0 compatible
#     on $7/mo node)
#   - Integrates with service-content DataGraph for entity grounding
#     (people, companies, products mentioned on landing pages link to
#     real graph entities)
#   - Audit ledger built-in (every page edit captured for training
#     corpus per Doctrine claim #44)
#   - WORM-ledger-design pattern for content version history
#   - No plugin sprawl — capabilities composed via app-orchestration-*
#     pattern (e.g., command for user aggregation, slm for inference,
#     content for graph)
#
# Two parallel deployments — media-marketing-landing-1 (Woodfine
# customer-tier; home.woodfinegroup.com) and media-marketing-landing-2
# (PointSav vendor-tier; home.pointsav.com) — demonstrate the
# multi-tenant pattern. Each tenant runs the same software with
# tenant-specific theming + content; SMB customers get the same
# system on their own service-SLM.

tetrad:
  vendor:
    - source_repo: pointsav-monorepo
      project_path: app-mediakit-marketing/
      status: active 2026-05-06; v0.0.1 MVP committed (3bc17061, Peter); axum binary with WordPress muscle-memory admin + multi-tenant + DataGraph optional; cargo check clean (245 pkgs, 0 errors); bootstrap + deploy pending Master
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: media-marketing-landing/
      status: existing catalog (currently holds telemetry GUIDEs); marketing-site GUIDEs to be added (guide-deployment-marketing-site, guide-provision-marketing-site)
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: media-marketing-landing/
      status: existing catalog (currently holds telemetry GUIDEs); marketing-site GUIDEs to be added
  deployment:
    - path: deployments/media-marketing-landing-1/
      tenant: woodfine
      domain: home.woodfinegroup.com
      status: MANIFEST authored 2026-05-06; first MVP build pending
    - path: deployments/media-marketing-landing-2/
      tenant: pointsav
      domain: home.pointsav.com
      status: MANIFEST authored 2026-05-06; first MVP build pending
  wiki:
    - target: vendor/content-wiki-documentation
      drafts_via: clones/project-editorial/.agent/drafts-outbound/
      status: leg-pending — TOPIC drafts staged when MVP demonstrates the WordPress-leapfrog framing

datagraph_module_id: both
# Marketing landing pages reference both pointsav (PointSav-tier
# product mentions, design tokens) and woodfine (Woodfine-tier
# customer surfaces). Per-call module_id explicit per
# conventions/datagraph-access-discipline.md.

mvp_scope:
  v0.0.1:
    - Rust binary serving static-rendered Markdown + flat-file content
    - WordPress muscle-memory navigation: Dashboard / Pages / Media / Themes / Settings
    - Multi-tenant via SERVICE_MARKETING_MODULE_ID env (woodfine | pointsav)
    - Reads landing-page entity references from service-content (Doorman 9080 once #12 lands; interim direct 9081)
    - Tier 0 compatible: serves on $7/mo node with no AI tier
  v0.0.2:
    - Theme system (CSS tokens from pointsav-design-system)
    - Per-tenant nav + branding (woodfine vs pointsav)
    - Audit-logged page edits via Doorman /v1/audit/capture
  v0.0.3:
    - WORM-ledger version history for page edits
    - SEO basics (meta tags, sitemap, RSS)
  v0.1.0:
    - "Plugins" = app-orchestration-* surfaces composed in via iframe / API mount
    - Forms (contact, newsletter, lead capture) feed service-content as graph mutations

cross_cluster_dependencies:
  - project-intelligence: service-content DataGraph (entity references)
  - project-design: pointsav-design-system tokens (theme CSS)
  - project-editorial: TOPIC authoring for marketing-site framing wiki content (later)
  - project-knowledge: app-mediakit-knowledge sibling pattern (Rust binary + flat-file content; muscle-memory shape parallel)
  - project-command: future user-aggregation surface may compose marketing analytics

provisioning_notes:
  - pointsav-monorepo sub-clone: ✅ provisioned 2026-05-06 (~468 MB; cluster/project-marketing branch; 3 remotes)
  - pointsav-design-system: deferred (CSS tokens not needed for v0.0.1 MVP; first release uses minimal inline styles)
  - vendor/pointsav-fleet-deployment + customer/woodfine-fleet-deployment: catalogs exist; marketing-site GUIDEs to be added in same session

session_role: task
default_starting_dir: ~/Foundry/clones/project-marketing/
---

# project-marketing — WordPress-leapfrog marketing landing surface

This cluster owns `app-mediakit-marketing` source — the Rust server delivering WordPress muscle-memory at the UI layer with a sovereign, Tier 0-compatible architecture beneath. Two simultaneous deployments demonstrate multi-tenant operation: woodfine (home.woodfinegroup.com) and pointsav (home.pointsav.com).

## Status

Manifest authored 2026-05-06. pointsav-monorepo sub-clone provisioned on cluster/project-marketing branch. Existing app-mediakit-marketing scaffold (Cargo.toml + bilingual README + src/) is ready for Task to develop the v0.0.1 MVP.

## Cross-references

- `conventions/orchestration-architecture.md` — Model B peer apps (app-mediakit-marketing is sibling to app-mediakit-knowledge)
- `conventions/datagraph-access-discipline.md` — entity lookups via Doorman
- `conventions/compounding-substrate.md` — sovereign + Tier 0 + optional intelligence
- `conventions/worm-ledger-design.md` — page-edit version history pattern
- `~/Foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md` — broader architecture context
