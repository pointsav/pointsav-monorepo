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
| `MARKETPLACE_BIND` | `127.0.0.1:9200` | Listen address (nginx reverse-proxies) |
| `POLYGON_WALLET_ADDRESS` | — | Receiving wallet for USDC payments |
| `FS_ENDPOINT` | `http://127.0.0.1:8020` | service-fs WORM ledger for receipt storage |
| `CATALOG_DIR` | — | Path to flat-file product YAML catalog |

## Build

```
cargo build --release
```

Binary lands at `$CARGO_TARGET_DIR/release/app-privategit-marketplace`
(global target: `/srv/foundry/cargo-target/release/`).
