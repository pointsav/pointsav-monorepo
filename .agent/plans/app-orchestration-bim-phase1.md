---
title: app-orchestration-bim — Phase 1 Architecture Spec
created: 2026-05-17
version: 0.1.0
status: active
---

# app-orchestration-bim — Phase 1 Architecture

## Access model (two-tier, operator-locked 2026-05-17)

| Tier | Gate | Content |
|---|---|---|
| Public | No login | Generic BIM objects: parking stalls, corridors, staircases, standard finishes. Served at bim.woodfinegroup.com. |
| Operational | os-console only | Full BIM object archives, BCF coordination workspace, IDS validation pipeline. |

The gated tier (lease or architect-of-record attestation login) is permanently out of scope.

## Phase 1 scope

1. Serve the woodfine-bim-library DTCG object catalog as browsable HTML
2. Apply BIM Objects terminology throughout all user-facing copy
3. Reflect two-tier model in any access language on the site
4. Endpoints: object categories, IFC mappings, regulatory overlays, climate zone parameters

## Out of scope (Phase 1)

- User authentication (removed from design)
- Write or edit interface (Phase 2+)
- BCF coordination (Operational tier, Phase 2+)
- IDS validation pipeline (Operational tier, Phase 2+)

## Technology

- Rust + Axum, server-rendered HTML
- Reads DTCG JSON from woodfine-bim-library vault at startup
- Flat-file: no database required
- Offline-capable, air-gap-deployable

## Deployment

- URL: bim.woodfinegroup.com (nginx → 127.0.0.1:9096)
- Service: local-bim-orchestration.service
- Vault: /srv/foundry/deployments/gateway-orchestration-bim-1/
- Object vault: /srv/foundry/clones/project-bim/woodfine-bim-library/
- Design system: /srv/foundry/clones/project-bim/pointsav-design-system/

## Website text changes needed (B5 — separate task)

The live binary (v0.0.2, built 2026-05-07) has hardcoded strings to update:
- Title: "BIM Token Catalog" → "BIM Object Library"
- Hero: "Woodfine BIM Token Catalog" → "Woodfine BIM Object Library"
- Nav: "What are BIM Tokens?" → "What are BIM Objects?"
- Nav: "Browse All Tokens" → "Browse the Library"
- Nav: "About BIM Tokens" → "About BIM Objects"

Source must be written from scratch (Reserved-folder → Active crate). Scope for next session.
