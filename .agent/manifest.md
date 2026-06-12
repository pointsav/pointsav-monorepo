---
schema: foundry-cluster-manifest-v1
cluster: project-software
cluster_name: Software Distribution
cluster_branch: cluster/project-software
created: 2026-05-21
state: active (v0.1.0 source + v0.0.3 marketplace + v0.0.3 wallet deployed 2026-05-21)
slm_endpoint: http://localhost:9080
module_id: software
doctrine_version: 0.0.14
doctrine_claims_codified: [37]
doctrine_claims_proposed: []

tetrad:
  vendor: pointsav-monorepo (cluster/project-software) — app-privategit-source, app-privategit-marketplace, tool-wallet
  customer: woodfine-fleet-deployment/software — leg-pending
  deployment: vault-privategit-source-1 (live; software.pointsav.com; ports 9201 + 9202)
  wiki: content-wiki-documentation — leg-pending

datagraph_module_id: software
cross_cluster_dependencies:
  - project-system (infrastructure; OS layer)

provisioning_notes: |
  Archive cloned from pointsav-monorepo cluster/project-software branch.
  Working in: ~/Foundry/clones/project-software/
  Sub-clone (monorepo): ~/Foundry/clones/project-software/pointsav-monorepo/
  Stage 6 promotion: bin/promote.sh from Command Session.
  Deployed on: vault-privategit-source-1 (this VM).

session_role: totebox
default_starting_dir: ~/Foundry/clones/project-software/
---

## Cluster mission

Build and maintain the PointSav software distribution substrate — the storefront,
binary release server, and payment infrastructure at `software.pointsav.com`.

### Deployed components

| Crate | Port | Version | Role |
|---|---|---|---|
| `app-privategit-source` | 9201 | v0.1.0 | Binary release server; Ed25519 license token verification |
| `app-privategit-marketplace` | 9202 | v0.0.3 | Software storefront; product catalog; license issuance; payment verification |
| `tool-wallet` | — | v0.0.3 | Polygon USDC payment watcher; BIP-39/BIP-32 HD address derivation; Ed25519 keygen |

### Pricing model (ratified 2026-05-22)

| Tier | Price | Licence |
|---|---|---|
| Open source | $1 | Apache 2.0 |
| Commercial | $19 | FSL (Functional Source Licence) |

Payment: Polygon USDC only. No subscriptions. One-time purchase per licence key.
BC tax posture: below $30k threshold.

### Licence key format

Ed25519 signature over `{product_id}:{customer_id}:{expiry}`.
Verification in `app-privategit-source` at `/verify` endpoint.

### Architecture note

`app-privategit-marketplace` (the storefront) is **CODE** — it runs our systems.
The software products it sells are **SOFT** — they carry Ed25519 licence keys and
marketplace listings. Do not conflate the storefront with the merchandise.
