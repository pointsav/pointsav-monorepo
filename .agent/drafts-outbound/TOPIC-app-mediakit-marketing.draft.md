---
schema: foundry-draft-v1
type: TOPIC-app-mediakit-marketing
language_protocol: PROSE-TOPIC
title: "app-mediakit-marketing — WordPress-leapfrog marketing landing server"
target_project: project-editorial
target_destination: vendor/content-wiki-documentation
created: 2026-06-01T00:00:00Z
author: totebox@project-marketing (claude-code / claude-sonnet-4-6)
bilingual_pair: required (topic-app-mediakit-marketing.es.md)
research_trail:
  source_commits:
    - b49b4ae (cluster/project-marketing — current HEAD)
    - 3bc17061 (pointsav-monorepo — v0.0.1 MVP committed, Peter Woodfine)
  prior_drafts:
    - .claude/drafts-outbound/leapfrog-2030/topics/topic-woodfine-design-system.draft.md
  citations:
    - conventions/compounding-substrate.md
    - conventions/worm-ledger-design.md
    - conventions/datagraph-access-discipline.md
    - conventions/orchestration-architecture.md
  operator_inputs:
    - "Cluster mission: build WordPress muscle-memory at the user-facing layer with leapfrog-2030 internal architecture"
    - "Two live deployments: home.woodfinegroup.com + home.pointsav.com — both active on foundry-workspace as of 2026-05-07"
    - "Doctrine claims codified in this cluster: 37, 43, 44"
  related_files:
    - clones/project-marketing/.agent/manifest.md
    - pointsav-monorepo/app-mediakit-marketing/Cargo.toml
    - deployments/media-marketing-landing-1/MANIFEST.md
    - deployments/media-marketing-landing-2/MANIFEST.md
---

# app-mediakit-marketing — WordPress-leapfrog marketing landing server

## What it is

`app-mediakit-marketing` is a Rust web server that delivers marketing landing sites. It presents the same vocabulary — Dashboard, Pages, Posts, Media, Themes, Plugins, Settings — that WordPress users already know, but replaces the PHP + MySQL stack beneath with a sovereign, Tier 0-compatible architecture: a single compiled binary, flat-file content storage, and a graph-entity integration layer that costs nothing to run on a $7/month node.

The name "WordPress leapfrog" describes the goal: preserve the user-facing interface that tens of millions of operators have internalized, and leapfrog the technical substrate to remove the constraints that make WordPress difficult to operate and extend.

## Background

WordPress powers roughly 40 percent of websites tracked by web crawlers. Its prevalence reflects a genuine product achievement: the admin interface standardized content management for an entire generation of operators. An operator who learned WordPress in 2010 can navigate a new WordPress install in 2026 without retraining.

The liability is the substrate. PHP execution environments, MySQL administration, plugin sprawl, and database-level content storage create operational overhead that is disproportionate for small and medium operators. Upgrade paths break plugins. Database corruption requires specialist recovery. Hosting costs scale with server capacity, not content volume. Multi-tenancy requires database-per-tenant isolation or complex shared-table schemas.

`app-mediakit-marketing` addresses this by keeping the interface contract while discarding the substrate.

## Architecture

### Binary

A single statically linked Rust binary (`app-mediakit-marketing`) runs the server. Built with the Axum web framework. No runtime dependencies beyond the OS kernel and a libc.

The binary:
- Serves static-rendered content from a flat-file content directory
- Exposes the WordPress-vocabulary admin UI at `/admin/`
- Reads tenant configuration from environment variables at startup
- Optionally queries the service-content DataGraph via Doorman for entity-grounded content references

### Flat-file content

Content lives in a directory on the host filesystem (`SERVICE_MARKETING_CONTENT_DIR`). Files are Markdown and HTML; there is no database. The binary reads content files on each request (with in-memory caching planned for v0.0.2). Edits to content files take effect on the next request with no service restart.

This eliminates the database administration surface: no schema migrations, no backup/restore complexity, no connection-pool management.

### Multi-tenant via environment variables

A single binary supports multiple tenants. Tenant identity is set at startup via `SERVICE_MARKETING_MODULE_ID` (e.g., `woodfine`, `pointsav`). Content directory, bind port, site title, and DataGraph module target all resolve from this value.

Two instances running the same binary on the same host demonstrate this:

| Instance | Tenant | Domain | Port |
|---|---|---|---|
| media-marketing-landing-1 | woodfine | home.woodfinegroup.com | 9102 |
| media-marketing-landing-2 | pointsav | home.pointsav.com | 9101 |

Each instance is a systemd service with its own unit file and environment block. Neither instance knows about the other.

### DataGraph integration (optional)

Landing pages can reference graph entities — people, companies, products — by ID. When `SERVICE_MARKETING_GRAPH_URL` is configured (pointing to a running Doorman instance), the binary resolves entity references at render time and embeds structured data. When the DataGraph is unavailable, the binary falls back to static content without error.

This integration is Tier 0 optional: a site running without DataGraph access is fully functional; DataGraph enriches it when present.

## Sovereignty and Tier 0 alignment

The Compounding Substrate doctrine defines Tier 0 as an operator-owned system that functions without any vendor cloud dependency. `app-mediakit-marketing` meets this bar:

- Single binary with no external runtime dependencies
- Flat-file content storage (no cloud database, no object storage required)
- nginx reverse proxy handles TLS; no managed load balancer required
- Runs on the smallest commercially available VPS ($7/month)
- DataGraph integration is optional — the site is not degraded in its absence

An SMB operator can run their own marketing landing site on hardware they own, with software built from auditable source, without any ongoing vendor relationship.

## WORM-ledger content history

Planned for v0.0.3: every content edit captured via the audit endpoint (`/v1/audit/capture` through Doorman) forms an append-only version history. This aligns with the WORM-ledger-design convention: content history is never deleted, only appended. The audit log serves two purposes — operator rollback, and training corpus for the AI tier (Doctrine claim #44).

## Deployment pattern

`app-mediakit-marketing` is deployed behind nginx. nginx handles:
- TLS termination (Let's Encrypt via certbot)
- Static file serving for `robots.txt` and `sitemap.xml`
- HTTP→HTTPS redirect
- Reverse proxy to the binary's loopback port

The binary never listens on a public port. All public traffic passes through nginx.

```
Internet → nginx :443 (TLS) → 127.0.0.1:PORT → app-mediakit-marketing
                              │
                              └→ CONTENT_DIR/ (flat-file reads)
                              └→ Doorman :9081 (DataGraph, optional)
```

## Live reference deployments

Two deployments are active as of 2026-05-07 on `foundry-workspace`:

- **home.woodfinegroup.com** — Woodfine Management Corp. customer-tier marketing site. Demonstrates the customer pattern: operator-branded, operated under the customer's identity.
- **home.pointsav.com** — PointSav vendor-tier open reference deployment. Demonstrates the vendor pattern: a public reference that prospective customers can inspect before deploying their own instance.

Both sites run the same `app-mediakit-marketing` binary. The difference is content and theme tokens.

## Roadmap

| Version | Key additions |
|---|---|
| v0.0.1 | Rust binary, WordPress vocab nav, multi-tenant env, DataGraph optional, Tier 0 |
| v0.0.2 | Theme system (CSS tokens from pointsav-design-system), per-tenant branding, audit-logged edits |
| v0.0.3 | WORM-ledger page-edit version history, SEO basics (meta, sitemap, RSS) |
| v0.1.0 | Plugin surface (app-orchestration-* composed via iframe/API mount), contact/lead forms feeding DataGraph |

## Source location

`pointsav-monorepo/app-mediakit-marketing/` on the `cluster/project-marketing` branch of `github.com/jwoodfine/pointsav-monorepo-staging`. Promoted to canonical `pointsav-monorepo` main via Stage 6.

## Related topics

- `topic-compounding-substrate` — sovereign architecture doctrine
- `topic-datagraph-access-discipline` — entity reference integration via Doorman
- `topic-worm-ledger-design` — append-only content version history pattern
- `topic-app-mediakit-knowledge` — sibling Rust server for knowledge-base content (same architectural pattern)
