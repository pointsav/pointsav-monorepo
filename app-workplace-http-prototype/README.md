# app-workplace-http-prototype

HTTP prototype surface for `os-workplace`. Serves all 8 Workplace tool
surfaces from a single axum binary accessible over WireGuard PPN while
native Tauri builds await a macOS host. Stage 1 delivers the Memo editor.

## Run

```sh
cargo build --release
WORKPLACE_PROTO_WORKSPACE=/home/jennifer/workbench \
WORKPLACE_PROTO_PORT=9110 \
./target/release/app-workplace-http-prototype
```

Open `http://10.8.0.9:9200` on the WireGuard PPN (nginx proxy; prototype binds internally on port 9110).

## Surfaces

| Surface | Stage | Status |
|---------|-------|--------|
| Memo | 1 | Active |
| Proforma | 2 | Pending |
| Presentation | 3 | Pending |
| Schedule | 4 | Pending |
| Code | 5 | Pending |
| PDF | 6 | Pending |
| GIS | 7 | Pending |
| BIM | 8 | Pending |

## File formats

- Memo: `.html` (innerHTML fragments, compatible with native `app-workplace-memo`)
- Proforma: `.json` (Stage 2)
- Presentation: `.json` (Stage 3)

## Environment variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `WORKPLACE_PROTO_WORKSPACE` | `/home/jennifer/workbench` | Root workspace directory |
| `WORKPLACE_PROTO_PORT` | `9110` | HTTP bind port |
