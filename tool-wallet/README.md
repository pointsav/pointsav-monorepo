# tool-wallet

Polygon USDC payment watcher and receipt writer for the PointSav software storefront.

Distinct from `service-wallet` (Doctrine claim #53), which is the Ring 2 per-tenant
revenue ledger for the customer-side Reverse-Flow Substrate. `tool-wallet` is a
single-tenant vendor-side utility: PointSav receives inbound license payments.

## Architecture position

```
Customer pays USDC on Polygon
        ↓
tool-wallet watches eth_getLogs for Transfer events to POLYGON_WALLET_ADDRESS
        ↓
Confirmed transfer → LicenseReceipt written to service-fs WORM ledger
        ↓
app-privategit-marketplace looks up receipt → issues license key + signed binary URL
        ↓
service-bookkeeper (project-bookkeeping) reads receipt → Dr Wallet/USDC Cr Revenue/Software-Sales
```

## Subcommands

```
tool-wallet watch     # daemon — polls Polygon for incoming USDC; writes receipts
tool-wallet check     # one-shot — verify a specific tx_hash
tool-wallet address   # generate a per-order HD-derived payment address
tool-wallet export    # export receipts to CSV for bookkeeping
tool-wallet keygen    # generate Ed25519 signing keypair
tool-wallet generate-seed  # generate BIP-39 seed + master wallet address
```

## Environment variables

| Variable | Purpose |
|---|---|
| `POLYGON_RPC_URL` | Polygon JSON-RPC endpoint (primary) |
| `POLYGON_RPC_FALLBACK_URLS` | Comma-separated fallback endpoints tried in order when primary fails (optional; applies to both `watch` and `check`) |
| `POLYGON_WALLET_ADDRESS` | PointSav receiving wallet address |
| `WALLET_SEED_PATH` | Path to HD seed file — operator-provisioned, never in git |
| `FS_ENDPOINT` | service-fs WORM ledger endpoint |
| `FS_MODULE_ID` | Ledger module ID (default: `software`) |

## Security note

`WALLET_SEED_PATH` is operator-provisioned outside the Foundry workspace. No AI agent,
no git commit, no log output ever touches the seed. The seed generates per-order addresses
via BIP-32 m/44'/60'/0'/0/<order_index>; only the derived addresses are in application state.
