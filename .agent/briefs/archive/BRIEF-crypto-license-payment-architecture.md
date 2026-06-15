---
artifact: brief
schema: foundry-brief-v1
brief-id: project-software-crypto-license-payment-architecture
title: "Crypto License Payment Architecture — Polygon USDC + Ed25519"
status: archived
owner: project-software
created: 2026-06-12
updated: 2026-06-12
contaminated_note: "M-17 contamination — belongs to project-software; archived from project-gis 2026-06-13 by command@claude-code"
---

# Crypto License Payment Architecture — Polygon USDC + Ed25519

## §1 Purpose

Records locked architectural decisions for the Polygon USDC payment flow and Ed25519
license issuance system, so future sessions do not re-derive them. Companion to
`BRIEF-software-distribution-substrate.md`.

## §2 Payment architecture (locked)

- **Network:** Polygon PoS — ERC-20 USDC (native USDC, not bridged)
- **Watcher:** `tool-wallet watch` — polls Polygon JSON-RPC; walks blocks for Transfer logs
  to vendor wallet address; writes structured flat-file receipts; maintains JSONL transaction log
- **Amount mapping:** two price points (6-decimal USDC units) map to two product IDs
  (open-source tier, commercial tier)
- **Fallback RPC URL:** supported for resilience; no env var names in public artifacts
- **Per-order HD addresses:** optional; BIP-39 master seed + BIP-32 Ethereum derivation path;
  per-order index stored locally; static vendor address remains the default

## §3 License token format (locked)

```
base64url( Ed25519_signature[64 bytes] || JSON_payload_bytes )
```

- **Payload fields:** product identifier, expiry date (one year from issuance), entitlements (tier)
- **Signing key:** storefront (`app-privategit-marketplace`) only; never transmitted to customer
- **Verification key:** release server (`app-privategit-source`) only; exposed at `/verify-key.pub`
- **Verification:** stateless, no network call — decode → split sig → verify → check product + expiry
- **Error codes:** wrong product → 403; invalid signature → 401; expired → 403 `channel-expired`

## §4 Scrub rules (operator decision 2026-06-03)

These items must never appear in any public artifact (TOPICs, READMEs, GUIDEs, marketplace copy):

- USDC contract address (hex)
- Exact pricing figures in any currency
- Env var names: `SIGNING_KEY_HEX`, `POLYGON_WALLET_ADDRESS`, `RECEIPTS_DIR`,
  `SOURCE_BASE_URL`, `CATALOG_PATH`, or any other config-level identifier
- BC/CRA tax threshold specifics
- Internal file paths (`/srv/foundry/…`, `/var/lib/…`)

*Pricing deferred to `software.pointsav.com`.*

## §5 Idempotency and claim flow (locked)

- `/v1/license/:tx_hash` — idempotent; first call may do a chain round-trip; subsequent calls
  served from flat-file receipt (no latency beyond local I/O)
- `/v1/claim` — records off-chain pairing: `binary SHA-256 ↔ buyer wallet address`;
  on-chain NFT minting deferred to future version

## §6 Open items

- [ ] Token revocation — no revocation list; tokens valid until expiry; rotation planned
- [ ] On-chain claim minting — `/v1/claim` is off-chain only; NFT minting deferred
- [ ] Per-order address mapping persistence — flat-file only; no backup/export mechanism
