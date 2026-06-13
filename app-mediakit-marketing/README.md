# app-mediakit-marketing

The marketing platform engine for the `os-mediakit` family. Serves
`home.woodfinegroup.com` and `home.pointsav.com`.

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

> **State:** Active (clean-sheet rewrite, P1 scaffold, 2026-06).
> Registry row: `pointsav-monorepo/.agent/rules/project-registry.md`.
> Architecture + roadmap: `.agent/briefs/BRIEF-marketing-platform-master.md`.

## What it is

A single Rust binary (axum 0.8) that renders marketing pages **server-side**
from typed section-manifests. It replaces the prior 1.2 MB single-file HTML
monolith and its fragile client-side bundler/template DOM-swap (the cause of
the iOS Safari viewport bug) with a clean server-rendered path.

It is **agent-first**: AI authors compose pages by emitting a typed manifest
through the MCP server; proposals stage to a human review queue; a human
approves (F12) before anything persists. There is no automated publish path
(SYS-ADR-10, SYS-ADR-19).

- **Chrome + components:** `app-mediakit-shell` (shared chassis). The header,
  footer, and every section component — including all responsive CSS — live
  there. Content manifests carry no CSS.
- **Content model:** `<content_dir>/<slug>/page.yaml` — an ordered list of
  typed sections. The schema is the contract.

## Run

```
cargo run -p app-mediakit-marketing -- serve \
  --content-dir app-mediakit-marketing/content \
  --state-dir /tmp/marketing-state \
  --module-id woodfine \
  --bind 127.0.0.1:9109 \
  --enable-mcp
```

Then `curl http://127.0.0.1:9109/` (fully server-rendered HTML),
`curl http://127.0.0.1:9109/page/contact`, `curl http://127.0.0.1:9109/healthz`.

## HTTP surface

| Route | Purpose |
|---|---|
| `GET /` | Render the home page |
| `GET /page/{slug}` | Render a page |
| `GET /healthz` | Health check |
| `POST /api/mcp` | MCP JSON-RPC 2.0 (agent authoring; when `--enable-mcp`) |
| `GET /api/pending` | List proposals awaiting approval |
| `GET /api/pending/{id}/manifest` | Proposed manifest YAML |
| `POST /api/pending/{id}/approve` | Approve (F12) — persist to content tree |

MCP tools: `list_section_types`, `read_page`, `validate_manifest`,
`propose_page`, `list_pending`.

## Build and test

```
cd app-mediakit-marketing
cargo test
cargo clippy --all-targets -- -D warnings
```

## Status

P1 scaffold: minimal section set (`hero`, `prose`, `cta`), file-based review
queue. Full section catalogue, true unified-diff review, content migration of
the live homepages, and the deployment cut-over are later phases — see the
master BRIEF.
