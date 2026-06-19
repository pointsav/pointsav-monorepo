---
artifact: topic
schema: foundry-draft-v1
title: "Private Git Paid Customer Endpoint"
slug: topic-private-git-paid-customer-endpoint
status: draft
language: en
bilingual_pair_required: true
bcsc_class: internal
forbidden_terms_cleared: false
route_to: project-editorial
created: 2026-06-14
updated: 2026-06-14
research_trail:
  sources_cited: false
  claims_verified: false
  sme_review: pending
  external_review: not-required
  last_checked: 2026-06-14
---

# Private Git Paid Customer Endpoint

The private git paid customer endpoint is the binary release server that delivers
compiled PointSav software to customers who hold valid license tokens. Despite the
"git" in the name, the endpoint serves pre-built binary artifacts rather than git
repositories; the name reflects its position in the PointSav infrastructure as the
successor to a planned private git hosting surface.

The service is `app-privategit-source`, deployed at port 9201 on
`vault-privategit-source-1`.

## What the Endpoint Serves

The endpoint serves binary release artifacts: compiled executables, archives, and
associated metadata. Each artifact is associated with a `product_id` and a version
string. The product catalog is populated by the marketplace (`app-privategit-marketplace`)
at the time of binary publication.

No source code is served. No git protocol is spoken. The HTTP API is the sole access
surface.

## Authentication

Every download request must carry an Ed25519 license token in the `Authorization`
header:

```
Authorization: Bearer <base64url-encoded-token>
```

The token is issued by `app-privategit-marketplace` at the time of purchase and
delivered to the customer. The binary server verifies the token locally:

1. Decodes the token envelope and recovers the public key reference.
2. Looks up the corresponding public key in its local license registry.
3. Verifies the Ed25519 signature over the token body.
4. Checks that `product_id` and `version_constraint` in the token cover the
   requested binary.

Step 4 is the authorization decision: a token purchased for product A does not
grant access to product B, and a token for version 1.x does not grant access to
version 2.0.

## API

| Endpoint | Method | Auth required | Description |
|----------|--------|--------------|-------------|
| `/v1/binaries/:product_id/:version` | GET | Bearer token | Download binary |
| `/v1/catalog` | GET | Bearer token | List available products and versions |
| `/v1/token/info` | GET | Bearer token | Return token metadata (product, expiry) |
| `/healthz` | GET | None | Health probe |

## Offline Verification

The binary server performs all verification without contacting the marketplace
or any external service at request time. The license registry (a local key store
populated from the marketplace at publication time) is the only dependency for
token verification. This means the binary server can serve downloads during
marketplace downtime, and it can be deployed in a network-isolated environment.

## License Registry Sync

The local license registry is populated when the marketplace publishes a new binary
or when a license token is issued. The sync mechanism between the marketplace and
the binary server is planned for Phase 2; in the current phase, the registry is
populated by direct write at the time of marketplace-side issuance.

## Token Lifecycle

Tokens may be perpetual (`expires_at = 0`) or time-limited. The binary server
enforces expiry at the time of each request. There is no revocation list in the
current phase; revocation via a deny-list of token IDs is planned.

Customers with expired tokens must renew through `app-privategit-marketplace`.
Renewal issues a new token; the old token is not invalidated (there is no revocation
in Phase 1, so the old token remains usable until expiry).

## Deployment

| Attribute | Value |
|-----------|-------|
| Host | `vault-privategit-source-1` |
| Port | 9201 |
| Version | v0.1.0 |
| Crate | `app-privategit-source` in `pointsav-monorepo` |

The binary server runs as a long-lived process managed by systemd. It does not
require a database; state is held in the local license registry file and the binary
artifact store on disk.

## Relationship to Other Topics

- [Software Distribution Substrate](topic-software-distribution-substrate) —
  full component inventory including the marketplace and payment watcher
- [Crypto License Sales Architecture](topic-crypto-license-sales-architecture) —
  how a license token is generated and delivered before it reaches this endpoint
