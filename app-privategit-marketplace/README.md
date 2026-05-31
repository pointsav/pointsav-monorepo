# app-privategit-marketplace

PointSav software storefront. Serves `software.pointsav.com`.

Customers who do not want to compile from source pay a license fee in USDC (Polygon PoS)
and receive a signed, pre-compiled binary. The open-source path remains free at
`github.com/pointsav/pointsav-monorepo`.

## Surfaces

- **Browse** — product catalog with version history and pricing
- **Pay** — USDC on Polygon PoS; payments verified by `tool-wallet`
- **License** — signed license key issued after on-chain payment confirmation
- **Download** — signed binary URL valid for 24 h; re-issuable with license key

## Environment variables

| Variable | Default | Purpose |
|---|---|---|
| `MARKETPLACE_BIND` | `127.0.0.1:9202` | Listen address (nginx reverse-proxies on port 9202) |
| `POLYGON_WALLET_ADDRESS` | — | **Required.** Receiving wallet address for USDC payments |
| `POLYGON_RPC_URL` | `https://polygon.drpc.org` | Polygon JSON-RPC endpoint |
| `CATALOG_PATH` | `/var/lib/local-software/catalog/products.yaml` | Path to flat-file product YAML catalog |
| `RECEIPTS_DIR` | `/var/lib/local-software/receipts` | Directory for LicenseReceipt JSON files |
| `CLAIMS_DIR` | `/var/lib/local-software/claims` | Directory for binary SHA256 claim tokens |
| `SOURCE_BASE_URL` | `https://software.pointsav.com/releases` | Base URL for binary download links |
| `LICENSE_SIGNING_KEY` | — | Optional. Ed25519 signing key (64 hex chars). Enables `/v1/issue-token`. |
| `WALLET_SEED_PATH` | — | Optional. Path to BIP-39 mnemonic file. Enables `/v1/order-address`. |
| `ORDER_INDEX_PATH` | `/var/lib/local-software/data/order-index.json` | HD wallet derivation counter |
| `RUST_LOG` | `info` | Log level |

## Build

```
cargo build --release
```

Binary lands at `$CARGO_TARGET_DIR/release/app-privategit-marketplace`
(global target: `/srv/foundry/cargo-target/release/`).
