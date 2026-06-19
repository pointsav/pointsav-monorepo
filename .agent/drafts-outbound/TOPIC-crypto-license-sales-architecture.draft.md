---
artifact: topic
schema: foundry-draft-v1
title: "Crypto License Sales Architecture"
slug: topic-crypto-license-sales-architecture
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

# Crypto License Sales Architecture

The crypto license sales architecture describes how a customer moves from
expressing intent to purchase through on-chain payment to receiving a license
token that grants binary access — all without a traditional payment processor
or identity provider in the critical path.

## Design Principles

Three principles shape the architecture:

1. **Payment and identity are separated.** The on-chain payment address is
   derived deterministically from a customer index; no identity information is
   required before payment arrives.
2. **The license token is the sole trust carrier.** Once issued, a license
   token can be verified by the binary server without contacting any other
   service.
3. **Key custody stays with the customer.** The marketplace never holds a
   private key on the customer's behalf. The Ed25519 keypair derived at
   issuance is the customer's credential.

## End-to-End Flow

### Step 1 — Address Derivation

When a customer creates an order on `app-privategit-marketplace`, the marketplace
requests a payment address from `tool-wallet`. The wallet derives an HD address
using the BIP-32 path `m/44'/60'/0'/0/<customer_index>` from a BIP-39 mnemonic
held in the wallet's key store. The derived Polygon address is unique to that
customer slot and is returned to the marketplace for display.

### Step 2 — On-Chain Payment

The customer sends USDC on the Polygon network to the displayed address.
`tool-wallet` polls the Polygon RPC endpoint at a configurable interval.
When a transfer to the customer's address is confirmed at the required block
depth, the wallet writes a payment receipt to its WORM store and notifies the
marketplace.

The wallet does not submit transactions; it reads only.

### Step 3 — License Issuance

On receiving payment confirmation, `app-privategit-marketplace` calls the wallet
to generate an Ed25519 keypair for this license. The public key is stored in the
marketplace's license registry. The signed license token is constructed with:

| Field | Value at issuance |
|-------|-------------------|
| `product_id` | Ordered product identifier |
| `version_constraint` | Purchased version range |
| `customer_id` | Opaque marketplace customer ID |
| `issued_at` | Current Unix timestamp |
| `expires_at` | 0 (perpetual) or configured expiry |

The token is signed with the Ed25519 private key generated in this step.
The private key is delivered to the customer and is not retained by the
marketplace. The marketplace retains only the public key for future verification
queries.

### Step 4 — Token Delivery

The signed license token is delivered to the customer over HTTPS. The customer
stores the token and presents it in the `Authorization: Bearer <token>` header
on all subsequent download requests to `app-privategit-source`.

### Step 5 — Binary Access

`app-privategit-source` receives the download request with the embedded token.
It decodes the token, recovers the public key from its own license registry
(populated from the marketplace at token-issuance time via a sync mechanism
planned for Phase 2), and verifies the Ed25519 signature locally. If the
product and version fields match the requested binary, the binary is served.
The binary server contacts no external service during this step.

## Failure Modes

| Failure | Behaviour |
|---------|-----------|
| Polygon RPC unavailable | Wallet queues payment checks; no impact on existing tokens |
| Marketplace unavailable | New orders blocked; existing license tokens continue to work |
| Binary server unavailable | Downloads blocked; license tokens remain valid |
| Token expired | Binary server returns 403; customer must renew via marketplace |

## Key Custody

The architecture guarantees that no single service holds both the ability to
confirm payment and the ability to issue a valid license. `tool-wallet` holds
the BIP-39 mnemonic (address derivation and Ed25519 keygen) but has no access
to product catalog or customer records. `app-privategit-marketplace` holds the
product catalog and customer records but calls the wallet only for key generation
and payment confirmation — it does not hold the BIP-39 mnemonic directly.

## Relationship to Other Topics

- [Software Distribution Substrate](topic-software-distribution-substrate) —
  component inventory and deployment details
- [Private Git Paid Customer Endpoint](topic-private-git-paid-customer-endpoint) —
  the binary server from the customer's perspective
- [PPN Small-Business Compute](topic-ppn-small-business-compute) — compute
  capacity sold through this same architecture
