---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "PointSav Software Distribution Substrate"
slug: topic-software-distribution-substrate
language: en
status: draft
paired_with: TOPIC-software-distribution-substrate.es.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-software-distribution-substrate.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-source/src/main.rs; app-privategit-marketplace/src/main.rs; tool-wallet/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# PointSav Software Distribution Substrate

The PointSav software distribution substrate is a three-component system that handles
binary release hosting, storefront and license issuance, and on-chain payment verification.
The three components — a release server, a marketplace storefront, and a payment watcher —
run as separate services accessible at `software.pointsav.com`. Each component has a
single, bounded responsibility, and together they form a custodian-free distribution path:
no customer accounts are required, and a customer can move from payment to binary download
in a single session.

## The three components

**Release server.** The release server serves compiled binaries and enforces Ed25519
license token verification before any download takes place. It also exposes product and
version indexes that allow tooling to discover what releases are available, and can redirect
requests for the latest version of a product to the appropriate versioned path. The server
is stateless: it holds no payment records and no customer data. Its only responsibility
is to verify that a token is genuine and authorises the requested product, then stream
the file.

**Storefront.** The storefront presents a product catalog to browsers, verifies incoming
Polygon USDC payments by cross-referencing confirmed on-chain transfers with a local
flat-file receipt store, and issues Ed25519-signed license tokens once payment is confirmed.
The storefront is CODE — it runs the vendor's system. The software products it sells are
SOFT — they carry Ed25519 license keys and marketplace listings. These are distinct
categories: the store is infrastructure; the merchandise is the licensed product.

**Payment watcher.** The payment watcher monitors the Polygon PoS network for inbound
USDC transfers to the vendor wallet address, writes a structured receipt for each confirmed
payment, and provides key-management utilities: Ed25519 keypair generation, BIP-39
mnemonic seed generation, and per-order hierarchical deterministic (HD) address derivation.
Receipts are the authoritative record of purchase; the storefront will not issue a license
token without one.

## License token format

A license token is an Ed25519 signature over a JSON payload, base64url-encoded. The
64-byte signature is prepended to the payload bytes before encoding, producing a single
opaque string. The payload records the product identifier, an expiry date, and a list of
entitlements that encode the license tier. The release server holds only the public half
of the signing keypair. Verification requires no network call and no shared state — the
server decodes the token, verifies the signature, and checks the product and expiry
entirely from the token itself.

## Payment and license flow

A purchase proceeds through five stages:

1. The customer visits the storefront, selects a license tier, and sends USDC to the
   vendor wallet on Polygon PoS — either to the static vendor address or to a per-order
   derived address for order tracking.
2. The payment watcher detects the confirmed on-chain transfer and writes a receipt
   identifying the product and the buyer's transaction hash.
3. The customer polls the storefront's license endpoint with the transaction hash; the
   storefront locates the receipt and issues a signed download token.
4. The customer presents the token to the release server via an HTTP header or query
   parameter; the release server verifies the token and streams the binary.
5. On subsequent use, the binary or installer can re-verify its own token against the
   release server's public key endpoint without contacting the storefront.

## License tiers

Two tiers are available. The open-source tier is licensed under Apache 2.0. The
commercial tier is licensed under the Functional Source Licence (FSL). Both tiers are
one-time purchases; there are no subscriptions. The tier is encoded in the token's
entitlements field and in the payment receipt. Current pricing is published at
`software.pointsav.com`.

## What this system is not

The substrate handles payments, token issuance, and binary delivery. It does not manage
subscriptions — all purchases are perpetual one-time transactions. It does not create
customer accounts. It does not implement DRM beyond token verification at download time.
It does not restrict access to source code: the GitHub repository is separately public
under the Apache 2.0 licence. Customers who purchase the commercial FSL tier receive a
license key that unlocks the binary distribution; source-code terms are governed by the
respective licence, not by this system.
