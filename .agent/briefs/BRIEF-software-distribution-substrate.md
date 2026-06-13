---
artifact: brief
schema: foundry-brief-v1
brief-id: project-software-distribution-substrate
title: "Software Distribution Substrate — software.pointsav.com"
status: active
owner: project-software
created: 2026-06-12
updated: 2026-06-12
---

# Software Distribution Substrate — software.pointsav.com

## §1 Mission

Build and maintain the PointSav software distribution substrate — binary release server,
storefront, and Polygon USDC payment watcher at `software.pointsav.com`.

## §2 Deployed components (as of 2026-06-12)

| Crate | Port | Version | Role |
|---|---|---|---|
| `app-privategit-source` | 9201 | v0.1.0 | Binary release server; Ed25519 license token verification |
| `app-privategit-marketplace` | 9202 | v0.0.3 | Storefront; product catalog; license issuance; payment verification |
| `tool-wallet` | — | v0.0.3 | Polygon USDC payment watcher; receipt writer; Ed25519 keygen; BIP-32 HD address derivation |

All three deployed on `vault-privategit-source-1` (this VM).

## §3 Decisions locked

- **Pricing tiers:** Apache 2.0 (open-source) and FSL (commercial); one-time purchase; no subscriptions
- **Payment:** Polygon USDC only
- **Artifact classification:** Storefront is CODE (runs our systems); products sold are SOFT
  (carry Ed25519 license keys and marketplace listings). Per CLAUDE.md cash-register test.
- **Scrub rule (operator decision 2026-06-03):** no exact pricing, no USDC contract address,
  no env var names in any public artifact; pricing deferred to `software.pointsav.com`

## §4 Tetrad state

- **vendor:** active — `cluster/project-software` branch; 3 crates deployed
- **deployment:** live — `vault-privategit-source-1`; `software.pointsav.com` ports 9201 + 9202
- **customer:** leg-pending — `woodfine-fleet-deployment/software` not yet created
- **wiki:** 6 TOPIC drafts staged 2026-06-12; pending project-editorial language pass + Stage 6

## §5 Open items

- [ ] Stage 6 promotion — Command runs `bin/promote.sh` after project-editorial confirms
  receipt of 6 TOPIC drafts (dispatch: `project-software-20260612-editorial-dispatch-3-topics-restaged`)
- [ ] Customer leg — create `woodfine-fleet-deployment/software`; requires Command Session
- [ ] Token revocation — no revocation list implemented; tokens valid until expiry date only
- [ ] On-chain NFT claim — `/v1/claim` endpoint writes off-chain pairing only; on-chain minting deferred
- [ ] BRIEF-brief-audit-2026-06.md — audit BRIEF; may be archived once README active table is stable
