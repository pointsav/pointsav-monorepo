---
artifact: topic
schema: foundry-draft-v1
title: "Software Distribution Substrate"
slug: topic-software-distribution-substrate
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

# Software Distribution Substrate

The software distribution substrate covers the full path from a customer's payment
on the PointSav marketplace through license token issuance to authenticated binary
delivery. Three deployed services form the substrate: a payment watcher, a
storefront with product catalog and license issuance, and a binary release server
with token verification.

## Components

### Payment Watcher (`tool-wallet`)

`tool-wallet` monitors an on-chain address for incoming USDC transfers on the
Polygon network. For each confirmed transfer it:

1. Derives a per-customer HD address from a BIP-39 mnemonic via BIP-32 (m/44'/60'/0'/0/index).
2. Generates an Ed25519 keypair for use as the license credential.
3. Writes a payment receipt to the local WORM store.

The wallet is stateless with respect to the blockchain: it reads confirmed
transactions and writes receipts; it does not submit transactions.

### Marketplace (`app-privategit-marketplace`)

`app-privategit-marketplace` (port 9202, deployed on `vault-privategit-source-1`)
provides the customer-facing storefront at `software.pointsav.com`. It:

- Presents a product catalog of available binary releases.
- Accepts order creation and links each order to the payment address derived by
  `tool-wallet` for that customer.
- Issues signed license tokens upon payment confirmation.
- Verifies payment receipts against the WORM store before issuing.

The marketplace holds no private key material for signing binaries; it issues
license tokens that the binary endpoint independently verifies.

### Binary Release Server (`app-privategit-source`)

`app-privategit-source` (port 9201, deployed on `vault-privategit-source-1`)
serves compiled binary releases to authenticated customers. Each download request
must carry a valid license token. The server:

1. Decodes the Ed25519 license token from the `Authorization` header.
2. Verifies the token signature against the corresponding public key stored at
   issuance time.
3. Checks the token's product and version fields against the requested binary.
4. Serves the binary if all checks pass; returns 403 otherwise.

The server does not contact the marketplace at verification time — token
verification is local and offline-capable.

## License Token Format

License tokens are Ed25519-signed payloads in a compact binary envelope. The
signed body carries:

| Field | Type | Purpose |
|-------|------|---------|
| `product_id` | string | Identifies the licensed binary |
| `version_constraint` | string | SemVer range or exact version |
| `customer_id` | string | Opaque customer identifier |
| `issued_at` | u64 | Unix timestamp of issuance |
| `expires_at` | u64 | Unix timestamp of expiry (0 = perpetual) |

Tokens are not revocable after issuance in the current phase. Revocation is
planned via a token-id deny-list at the binary release server.

## Deployment

All three components run on `vault-privategit-source-1`:

| Service | Port | Version |
|---------|------|---------|
| `app-privategit-source` | 9201 | v0.1.0 |
| `app-privategit-marketplace` | 9202 | v0.0.3 |
| `tool-wallet` | daemon | v0.0.3 |

`tool-wallet` runs as a daemon, polling the Polygon RPC endpoint at a configurable
interval. It does not expose an HTTP surface.

## Trust Model

The substrate makes no assumption that the marketplace and the binary server share
a network trust boundary. The license token is the sole trust carrier: a token
issued by the marketplace is sufficient for binary access without further
marketplace involvement. This allows the binary server to be moved to a different
host or network segment without changes to the marketplace.

## Relationship to Other Topics

- [Crypto License Sales Architecture](topic-crypto-license-sales-architecture) —
  detailed walk-through of the payment-to-delivery flow
- [Private Git Paid Customer Endpoint](topic-private-git-paid-customer-endpoint) —
  the binary endpoint from the customer's perspective
- [PPN Small-Business Compute](topic-ppn-small-business-compute) — compute capacity
  sold through the same substrate
