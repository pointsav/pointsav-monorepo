---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Crypto Payment and License Issuance Architecture"
slug: topic-crypto-license-sales-architecture
language: en
status: draft
paired_with: TOPIC-crypto-license-sales-architecture.es.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-crypto-license-sales-architecture.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-marketplace/src/main.rs; tool-wallet/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# Crypto Payment and License Issuance Architecture

A software license purchase on `software.pointsav.com` flows from a Polygon USDC
transfer on-chain to an Ed25519-signed token that authorises binary downloads. The
design is intentionally custodian-free: the customer never creates an account, the
vendor never holds customer funds beyond the moment of settlement, and no intermediary
is required to route the payment. The architecture has three moving parts — a payment
watcher, a storefront, and a release server — described here at the level of their
interactions.

## Why USDC on Polygon

USDC is a USD-pegged stablecoin issued by Circle. Its value is anchored to the US dollar,
which makes it practical for fixed-price software purchases without exposing either party
to exchange-rate volatility. Polygon PoS is a proof-of-stake EVM-compatible chain with
lower transaction fees than Ethereum mainnet, making it economical for purchases in the
single-digit dollar range. The payment system operates as a read-only observer of public
blockchain state: it watches for ERC-20 Transfer log events on the USDC contract addressed
to the vendor wallet. No vendor-side smart contract is required. Any blockchain explorer
can independently verify a payment using the transaction hash.

## Payment watcher mechanics

The payment watcher polls the Polygon JSON-RPC endpoint at a configurable interval,
walking forward through confirmed blocks and inspecting ERC-20 Transfer log entries. When
it finds a Transfer to the vendor wallet address, it inspects the transferred amount to
determine which license tier the payment corresponds to — the two tiers each map to a
distinct USDC amount. On each confirmed match, it writes a structured flat-file receipt
containing the transaction hash, the sender address, the block number, the confirmation
timestamp, and the derived product identifier. It also appends an entry to a JSONL
transaction log for bookkeeping and audit purposes.

Receipts are authoritative. The storefront will not issue a license token without a
matching receipt on disk. The two-stage design — watcher writes receipt, storefront reads
receipt — means the storefront never queries the chain directly; it delegates that
responsibility entirely to the watcher.

The watcher also supports a fallback RPC URL for resilience: if the primary RPC endpoint
is unreachable, it retries against the fallback before failing.

## Per-order address derivation

By default, all payments are directed to a single static vendor wallet address, and the
transaction hash serves as the order identifier. For customers who prefer a dedicated
receiving address for order tracking or accounting purposes, the storefront can derive one
from the vendor's BIP-39 master seed using BIP-32 hierarchical deterministic key
derivation along the standard Ethereum derivation path. Each order receives a unique index,
and the mapping from order identifier to derivation index is stored locally. Payments to
derived addresses are watched and receipted by the same watcher that monitors the static
address. The derived-address flow is optional; the standard single-address flow remains
the default.

## License token issuance

Once a receipt exists for a transaction hash, the storefront issues an Ed25519-signed
license token. The signing key is held exclusively by the storefront and never leaves it.
The corresponding public verification key is held exclusively by the release server.
Neither component holds the other's key material, and no key material is transmitted
to the customer.

The token payload records the product identifier, an expiry date (one year from the time
of issuance at current configuration), and the license tier as a list of entitlements.
The token is formed by prepending the 64-byte Ed25519 signature to the raw payload bytes
and encoding the result as a base64url string. The output is a single opaque string the
customer stores and presents to the release server.

## Verification at the release server

Verification is stateless and requires no network call. When a download request arrives
with a token, the release server base64url-decodes the string, splits off the first 64
bytes as the Ed25519 signature, verifies the signature over the remaining bytes using
the stored public key, parses the payload, and checks that the product matches the
requested product and that the expiry date has not passed. A mismatched product returns
403; an invalid signature returns 401; an expired token returns 403 with a reason string
indicating the channel has expired. Because the release server holds no signing key, a
compromise of the release server does not allow an attacker to mint new tokens.

The release server exposes the public verification key at a well-known endpoint. External
tooling — such as a customer's own installer script — can download the public key once
and subsequently verify tokens offline without contacting the release server at runtime.

## Receipt idempotency and the claim flow

The storefront's license issuance endpoint is idempotent: querying the same transaction
hash multiple times always returns the same token. If a receipt is already on disk, the
token is issued immediately. If not, the storefront delegates a chain lookup to the
payment watcher's check subcommand and, on confirmation, writes the receipt before issuing
the token. The first call for a new transaction may involve a chain round-trip; every
subsequent call is served from disk with no latency beyond local I/O.

A separate claim endpoint records an off-chain association between a binary SHA-256 hash
and the buyer's wallet address. This forms the basis for a future on-chain ownership
attestation. On-chain NFT minting is deferred to a future version of the system; the
claim record is written now so the data is available when that capability is added.
