# app-privategit-software

PointSav software storefront. Serves `software.pointsav.com`.

Every product is currently `$0`/BETA regardless of license tier — pricing is
tracked in the catalog but no product currently charges. Customers who do not
want to compile from source will eventually pay a license fee in USDC
(Polygon PoS) and receive a signed, pre-compiled binary; the open-source path
remains free at `github.com/pointsav/pointsav-monorepo`.

## Surfaces

- **Browse** (`/software`, `/software/:id`) — catalog rendered live from
  `products.yaml`, grouped by license tier
- **Pricing** (`/pricing`), **Licensing** (`/licensing`), and legal pages
  (`/page/contact`, `/page/disclaimer`, `/page/privacy`, `/page/accessibility`)
- **Pay** — USDC on Polygon PoS; payments verified via `tool-wallet`
- **License** — a bearer license token minted after on-chain payment
  confirmation, valid through the end of the UTC day it was minted (re-mints
  fresh on a later visit to `/order/:tx_hash`, not a fixed 24h window)
- **Download** — the license token authenticates one download against
  `app-privategit-release`

## Operational notes

- **Input validation.** `tx_hash` (64 hex digits), `wallet_address` (40 hex
  digits), and `binary_sha256` (64 hex digits) are all validated to their
  exact expected shape before touching the filesystem or the on-chain check —
  none of them can steer a path (receipts/claims directories are keyed by
  these values).
- **Error shape.** JSON error responses carry `{"error": "<message>", "code":
  "<stable-kebab-case-slug>"}`; match on `code`, not `error`'s wording.
- `resolve_license`'s `tool-wallet check` subprocess call is async
  (`tokio::process::Command`) — the Polygon RPC round-trip it performs does
  not block other requests on the same worker thread.

## Environment variables

| Variable | Default | Purpose |
|---|---|---|
| `MARKETPLACE_BIND` | `127.0.0.1:9202` | Listen address |
| `CATALOG_PATH` | `catalog/products.yaml` | Path to the product catalog YAML |
| `STATIC_DIR` | `static` | Static assets dir (serves `/static/*`; also where `licensing.html` is read from) |
| `POLYGON_WALLET_ADDRESS` | — | Receiving wallet for USDC payments |
| `POLYGON_RPC_URL` | `https://polygon-rpc.com` | Polygon JSON-RPC endpoint used for payment confirmation checks |
| `RECEIPTS_DIR` | `/var/lib/local-software/receipts` | Where confirmed-payment receipts are written |
| `CLAIMS_DIR` | `/var/lib/local-software/claims` | Off-chain claim-token records (`/v1/claim`) |
| `SOURCE_BASE_URL` | `https://software.pointsav.com/releases` | Base URL used to build download/manifest links — **set this to the local release server on non-prod hosts** or CSP will block the client-side MANIFEST fetch as cross-origin |
| `TOOL_WALLET_BIN` | `tool-wallet` | Binary name/path invoked to check payment confirmations |
| `TX_LOG_PATH` | `/var/lib/local-software/tx-log.jsonl` | Transaction log for confirmed orders |
| `USDC_CAD_SPOT_RATE` | `1.37` | Spot rate used for CAD-denominated log entries |
| `SIGNING_KEY_SECRET` | unset | Ed25519 private signing key (hex, or a path to a file containing hex) used to mint license tokens. If unset, `/order/:tx_hash/download` returns 503. |
| `VERIFY_KEY_PUB` | unset | **Optional, self-test only.** If set, compared against `SIGNING_KEY_SECRET`'s derived public key at startup — logs a loud error on mismatch instead of silently 401ing every download later. Should be the same public key configured on the paired `app-privategit-release` instance. |
| `RUST_LOG` | `info` | Standard `tracing-subscriber` env filter |

## Build

```
cargo build --release
```

Binary lands at `$CARGO_TARGET_DIR/release/app-privategit-software`
(this workspace's shared target dir: `/srv/foundry/cargo-target/<user>/release/`).

## License

FSL-1.1-ALv2. See the repository root `LICENSE`.
