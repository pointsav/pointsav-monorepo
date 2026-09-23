# app-privategit-release

Signed binary release server for `software.pointsav.com`. Streams product
binaries, installer scripts, and manifests over HTTP, gating each download
behind an Ed25519 license token where the product requires one. Also exposes
a `/git/*` smart-HTTP endpoint stub (returns 503 — not yet implemented).

**Status:** active — deployed locally (`local-software-source.service`,
`127.0.0.1:9201`) and on `software.pointsav.com`.

## Routes

| Method | Path | Notes |
|---|---|---|
| GET | `/healthz` | Liveness check |
| GET | `/releases/` | Lists product directories under `RELEASES_DIR` |
| GET | `/releases/:product/` | Lists deposited versions for one product |
| GET | `/releases/:product/install.sh` | Streams the product's install script |
| GET | `/releases/:product/:version/MANIFEST` | Raw `MANIFEST.json`, unauthenticated |
| GET | `/releases/:product/latest/:platform` | 307 redirect to the numerically-highest deposited version |
| GET | `/releases/:product/:version/:platform` | The binary download — gated by license token unless the product's `MANIFEST.json` sets `requires_license: false` |
| GET/POST | `/git/*path` | Smart-HTTP stub — always 503, not implemented |
| POST | `/verify-key` | Validates a license token, returns its decoded payload |
| GET | `/verify-key.pub` | Returns this server's Ed25519 verify key as hex |
| POST | `/admin/reload-revocation-list` | Reloads the revocation set from disk — loopback-only |

`product`/`version`/`platform` path segments are validated (`is_safe_segment`)
before touching the filesystem — no `..`, path separators, or control
characters accepted.

## Operational notes

- **Rate limiting.** Two independent per-IP sliding-window limiters, 20
  requests/60s each: a "public" bucket shared by `/verify-key` and the
  `:product/:version/:platform` binary-download route (the two routes doing
  real Ed25519 verification per request), and an "admin" bucket scoping only
  `/admin/reload-revocation-list`. The client IP is the rightmost entry in
  `X-Forwarded-For` when present (this vhost's nginx always appends the real
  peer there), falling back to the direct TCP peer otherwise.
- **Caching.** Binary/manifest/script downloads go through one `stream_file`
  helper (`tower_http::ServeFile` for Range support) that also sets a weak
  `ETag` (mtime+size) and `Cache-Control: public, max-age=300,
  must-revalidate`, and honors `If-None-Match`/`If-Modified-Since` with a
  `304`.
- **Error shape.** Every JSON error response carries `{"error": "<message>",
  "code": "<stable-kebab-case-slug>"}`. `code` is safe to match on; `error`'s
  wording may change.

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `SOURCE_BIND` | `127.0.0.1:19201` | Listen address. **The default is a test port** — production deployments must set this explicitly (e.g. `127.0.0.1:9201`). |
| `RELEASES_DIR` | `/var/lib/local-software/releases` | Root of deposited product releases |
| `VERIFY_KEY_PUB` | unset | Ed25519 public key (hex, or a path to a file containing hex) used to verify license tokens minted by `app-privategit-marketplace`. If unset, `/verify-key` and licensed downloads return 503. |
| `REVOCATION_LIST_PATH` | unset | Path to a newline-delimited list of revoked token fingerprints |
| `RUST_LOG` | `info` | Standard `tracing-subscriber` env filter |

`VERIFY_KEY_PUB` must be the public counterpart of the `SIGNING_KEY_SECRET`
configured on the paired `app-privategit-marketplace` instance — a mismatch
produces uniform 401s on every licensed download. The marketplace's own
startup keypair self-test (`VERIFY_KEY_PUB` set on that process too) catches
this if configured; see that crate's README.

## License

FSL-1.1-ALv2. See the repository root `LICENSE`.
