# app-privategit-workbench

Browser-based developer workbench for Totebox Orchestration.

Serves a three-column IDE (file tree / viewer / editor) over a local HTTP
endpoint. The write-service enforces atomic writes, mtime conflict detection,
an extension allowlist, and root-containment security. Hosted by `os-privategit`.

## Architecture

- `src/main.rs` — Rust/axum HTTP server (GET/PUT file API, port 9210)
- `src/assets/index.html` — single-page application (sidebar tree / viewer / editor)
- `config.toml` — writable root declarations and bind address

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
