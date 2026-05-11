---
schema: foundry-cluster-manifest-v1
cluster_name: project-gis
cluster_branch: cluster/project-gis
created: 2026-04-30
state: provisioning
doctrine_version: 0.0.14
doctrine_claims_codified: []
doctrine_claims_proposed: []   # placeholder; will fold in claims surfaced by Sonnet research

operator: woodfine + pointsav (jointly — Location Intelligence platform)
working_pattern: research-then-scaffold
input_shape: open-gis-standards + retail-co-location-analysis

design:
  rules:
    - .agent/rules/design-tokens.md

# Cluster mission (workspace v0.1.88, 2026-04-30):
# Ship a "Location Intelligence" platform — a leapfrog-2030 flat-file
# open-GIS substrate parallel to project-bim's Building Design System.
# Same architectural commitments: flat-file storage, open standards,
# Rust + Tauri, offline-first, EUPL-licensed, seL4-hardened.
#
# Three Totebox Archive services to be added (per operator 2026-04-30):
#   - service-business: retail business locations (Walmart, Home Depot,
#     Costco, Ikea, regional equivalents)
#   - service-places: public-purpose locations (hospitals, higher
#     education, airports)
#   - service-parking: geo-fence parking lot coordinates
#
# Three new app surfaces:
#   - app-console-gis: query + dataset provider
#   - app-workplace-gis: PointSav's QGIS — manual layer editor on map
#   - app-orchestration-gis: meteoblue.com-quality map renderer
#     (browser-delivered via os-orchestration)
#
# Customer-facing deployment: gis.woodfinegroup.com
# Showcases co-location of Walmart/Ikea + Home Depot/X + Costco
# within 1km / 2km / 3km of each other.
#
# Bridge to BIM: location intelligence supplies the urban-scale
# context that BIM compositions live in (claim #41 City Code as
# Composable Geometry depends on a real geographic dataset).

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: ./pointsav-monorepo
      upstream: vendor/pointsav-monorepo
      focus: |
        service-business/, service-places/, service-parking/ (Ring 1
        boundary ingest), app-console-gis/, app-workplace-gis/,
        app-orchestration-gis/, os-orchestration/ (mapping browser
        delivery)
      status: leg-pending — sub-clone provisioning + scaffold
    - repo: pointsav-design-system
      path: ./pointsav-design-system
      upstream: vendor/pointsav-design-system
      focus: |
        DESIGN-* + COMPONENT-* + RESEARCH-* + token-* drafts for the
        gis.woodfinegroup.com surface (parallel to design-system
        substrate per Doctrine claim #38)
      status: leg-pending
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: gateway-orchestration-gis/
      tenant: woodfine
      purpose: customer-facing-location-intelligence-public-demo
      status: leg-pending — catalog folder authoring
  deployment:
    - path: deployments/gateway-orchestration-gis-1
      tenant: woodfine
      shape: long-running-service
      runtime_artifacts:
        - (planned) /usr/local/bin/app-orchestration-gis
        - (planned) /etc/systemd/system/local-gateway-orchestration-gis.service
        - (planned) /var/lib/gateway-orchestration-gis/tiles/
        - (planned) nginx vhost gis.woodfinegroup.com (HTTPS)
      status: leg-pending — pre-provisioning
    - path: clusters/cluster-totebox-personnel-1
      tenant: woodfine
      shape: data-archive
      runtime_artifacts:
        - service-business JSONL/Parquet (retail locations)
        - service-places JSONL/Parquet (public-purpose locations)
        - service-parking JSONL/Parquet (geo-fence polygons)
      status: leg-pending — data acquisition (Walmart/Home Depot/Costco
        US/CA/MX/ES + Ikea ES + Home Depot equivalent ES)
  wiki:
    # project-gis does NOT hold sub-clones of content-wiki repos (removed 2026-05-05).
    # All wiki drafts route via drafts-outbound model only.
    - drafts_via: clones/project-gis/.agent/drafts-outbound/
      gateway: project-editorial Task
      planned_topics:
        - topic-location-intelligence-platform.md  # what + why; Bloomberg-grade
        - topic-gis-substrate-architecture.md      # flat-file open-GIS pattern
        - topic-geo-fence-parking-pattern.md       # service-parking shape
        - guide-gis-deployment.md                  # how to operate gis.woodfinegroup.com
        - topic-walmart-homedepot-costco-co-location.md
        - topic-spanish-retail-equivalents.md
      status: leg-pending — scaffold after research lands; 6 rescued drafts in drafts-outbound/from-project-gis/

# Operator decisions surfaced (in workspace NEXT.md operator-presence carries):
#   1. Workplace OS deployment naming — operator suggested
#      "desktop-workplace-gis"; need Nomenclature Matrix amendment
#   2. service-business storage shape — flat (JSONL/Parquet) vs
#      database (PostgreSQL+PostGIS); pending Sonnet research outcome
#   3. Mapping tile/layer delivery stack — pending Sonnet research
#      (vector tiles via MapLibre? raster via Leaflet? meteoblue uses
#      proprietary; what's the open-source equivalent?)

# Bootstrap commits will be authored by jwoodfine/pwoodfine alternating
# via bin/commit-as-next.sh once sub-clones are provisioned.
---

# Cluster manifest — project-gis

This file is the cluster-manifest declaration per Doctrine §IV.c +
v0.0.10 Tetrad amendment. Read at session start.

## Cluster status

**State**: active (2026-05-04 — Master ratification of Gemini-era work)

Sub-clones provisioned: pointsav-monorepo, pointsav-design-system,
woodfine-fleet-deployment, content-wiki-documentation, content-wiki-projects,
woodfine-media-assets. Monorepo contains service-business, service-places,
service-fs, app-orchestration-gis crate directories.

Two-deployment architecture in place:
- `cluster-totebox-personnel-1` — data layer (service-business, service-places, service-fs)
- `gateway-orchestration-gis-1` — GIS platform (app-orchestration-gis + www)

gis.woodfinegroup.com is live. TOTEBOX_DATA_PATH path fixed 2026-05-04 (removed
spurious `/data/` suffix in config.py and MANIFEST.md).

## Standing in for v0.1.88+

This cluster bootstraps in three phases:

### Phase 1 — Strategy + cluster shell (THIS COMMIT v0.1.88)

- Cluster directory + manifest provisioned
- Workspace `PROJECT-CLONES.md` updated with project-gis row
- Sonnet research dispatched (Location Intelligence platform survey;
  Spain Home Depot equivalent; architecture flat-vs-database; map
  tile/layer delivery stack)
- Operator decisions surfaced (Workplace OS deployment naming;
  storage shape)

### Phase 2 — Sub-clone provisioning + scaffold (next session)

When operator green-lights the strategy doc, Master:
- Provisions sub-clones for pointsav-monorepo, pointsav-design-system,
  woodfine-fleet-deployment in this cluster
- Scaffolds service-business, service-places, service-parking
  directories + Cargo.toml stubs
- Scaffolds app-console-gis, app-workplace-gis, app-orchestration-gis
  directories
- Cluster-Task picks up after this point

### Phase 3 — Data acquisition + visualization (Task work)

- Walmart Superstore locations: US, Canada, Mexico, Spain (Spain = Ikea)
- Home Depot locations: US, Canada, Mexico (Spain = X, per Sonnet research)
- Costco locations: US, Canada, Mexico, Spain
- Ingest into service-business
- Co-location analysis: pairs/triples within 1km / 2km / 3km
- Visualization at gis.woodfinegroup.com

## What this cluster does NOT do

- BIM (project-bim's scope; cross-references for City Code claim #41)
- Editorial gateway (project-language's scope; project-gis stages
  drafts at `.claude/drafts-outbound/`)
- Doorman/SLM (project-slm's scope; project-gis consumes service-SLM
  for routine map-annotation work via SERVICE-SLM-PROPOSAL convention
  per v0.1.86 broadcast)

## Cross-references

- `~/Foundry/CLAUDE.md` §10 + §11 (cluster pattern + tetrad)
- `~/Foundry/DOCTRINE.md` claims #14, #16, #18, #28, #38
- `~/Foundry/conventions/four-tier-slm-substrate.md`
- `~/Foundry/conventions/project-tetrad-discipline.md`
- `~/Foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md`
- `~/Foundry/MEMO-2026-03-30-Development-Overview-V8.md` §4
  (deployment catalog)
- `~/Foundry/clones/project-bim/.claude/manifest.md` (parallel cluster
  pattern; same Tetrad shape)
