# Development — Workplace\*Proforma

Setup instructions, build steps, platform notes, and migration guides.

---

## Prerequisites

| Tool | Version | Purpose |
|---|---|---|
| Rust | 1.70+ (Tauri v1) / 1.78+ (Tauri v2) | Backend compilation |
| Node.js | 20+ | Tauri CLI and `npm install` |
| make | any | Task orchestration via Makefile |
| WebView | see below | Frontend rendering |

### Platform-specific WebView

- **macOS 10.13 High Sierra** — WKWebView (bundled with OS). Tauri v1 required.
- **macOS 10.15 Catalina and later** — WKWebView (bundled). Tauri v2 preferred.
- **Linux** — WebKitGTK via `libwebkit2gtk-4.1-0` (Ubuntu 22.04+) or `libwebkit2gtk-4.0-37` (older). Install with: `sudo apt install libwebkit2gtk-4.1-dev`
- **Windows** — WebView2 runtime. Required on Windows 10 1803 and later; auto-installed on Windows 11.

---

## First-time setup

```bash
cd app-workplace-proforma
npm install
```

That is the whole setup for Phase 1. No font download step (unlike memo), no vendor JS download — the formula engine is shipped inside the application itself at this phase.

---

## Running in development

```bash
npm run tauri dev
# or
make dev
```

This starts Tauri in development mode with hot-reload on the frontend. The Rust backend rebuilds incrementally on changes; the frontend reloads on file save.

---

## Production build

```bash
npm run tauri build
# or
make build
```

Output locations:

- **macOS:** `src-tauri/target/release/bundle/dmg/Workplace\ Proforma_0.1.0_x64.dmg`
- **Linux (Debian):** `src-tauri/target/release/bundle/deb/workplace-proforma_0.1.0_amd64.deb`
- **Linux (AppImage):** `src-tauri/target/release/bundle/appimage/workplace-proforma_0.1.0_amd64.AppImage`
- **Windows:** `src-tauri/target/release/bundle/msi/Workplace\ Proforma_0.1.0_x64_en-US.msi`

Typical bundle size: 8–12 MB depending on platform. (Compare to Electron-based spreadsheet tools which typically ship 120–180 MB.)

---

## Tauri v1 → v2 migration

The development machine runs macOS 10.13, which constrains us to Tauri v1 for `make dev`. The Linux production target runs Tauri v2. The frontend code is identical between versions; only Cargo and `tauri.conf.json` differ.

### Linux production build with Tauri v2

```bash
# Update package.json
npm install @tauri-apps/cli@2 @tauri-apps/api@2

# Update src-tauri/Cargo.toml:
#   tauri = { version = "2", features = [] }
#   tauri-plugin-fs = "2"
#   tauri-plugin-dialog = "2"
# And [build-dependencies]:
#   tauri-build = { version = "2", features = [] }

# Convert allowlist → capabilities/default.json (see Tauri v2 docs)

# Update src-tauri/src/main.rs IPC signatures for v2's plugin-based FS API
```

A detailed step-by-step migration guide will be added here when the dev machine is upgraded.

---

## Running tests

No test suite ships with Phase 1. Phase 2 adds:

- Rust unit tests for IPC command handlers (`cargo test`)
- JavaScript unit tests for the formula engine covering every function with edge cases
- Fixture-driven integration tests: open a canonical proforma file, evaluate, save, reopen, confirm byte-identical
- Cross-version compatibility tests: every file written by Schema 1.0 opens unchanged in subsequent schema versions

---

## Formula engine — debugging

The Phase 1 JS engine exposes its state on `window.WorkplaceEngine` for console inspection:

```javascript
// In the dev WebView console
WorkplaceEngine.getAllCells()           // all cells in the active sheet
WorkplaceEngine.getCell("C5")            // single cell detail
WorkplaceEngine.evaluateFormula("=SUM(C2:C10)")
```

The engine uses a simple memoised recursive evaluator with cycle detection. A cell showing `#CIRC!` indicates a circular reference; `#NAME?` indicates an unresolved semantic identifier; `#DIV/0!` indicates a division by zero.

See [`docs/engine.md`](./docs/engine.md) for the full function reference and known limitations.

---

## Schema validation

The Rust side validates that every file read is parseable JSON and every file written is parseable JSON. The JS side validates document shape against the schema defined in `src/js/schema.js`. A full JSON Schema validator is planned for Phase 2 (candidate: `ajv`, vendored at a pinned version — no npm dependency at runtime).

To inspect a proforma file manually:

```bash
jq '.' my-proforma.json                # pretty-print
jq '.metadata'  my-proforma.json       # just metadata
jq '.audit'     my-proforma.json       # just the audit chain
jq '.assumptions[] | .label' my-proforma.json  # list all assumption labels
```

Every field has a human-readable name. The schema is self-describing; an LLM or a human reader can understand the document without the application.

---

## CSP debugging

If the console reports CSP violations during development:

1. Verify the violation is legitimate (not an outbound call we wish to prevent).
2. If the violation is from a feature we want, extend the CSP in `src-tauri/tauri.conf.json` with the minimum necessary origin — never a wildcard.
3. Never add `unsafe-eval` to `script-src`. The formula engine parser is written to avoid `eval()` entirely.

---

## Cross-platform fonts

Unlike memo, proforma does not embed fonts at build time. The grid uses the OS's default monospace stack (`ui-monospace, 'SF Mono', 'Menlo', 'Consolas', monospace`) for tabular figures. This is deliberate: monospace fonts are universally installed, so the proforma renders consistently everywhere without embedding any font binaries.

Phase 2 adds the option of embedding an OFL monospace family (candidate: IBM Plex Mono, JetBrains Mono, or Berkeley Mono if licensing permits) for absolute cross-platform determinism, matching the embed-at-build-time pattern memo uses for its serif families.

---

## Hot rebuild without full Tauri restart

When developing the frontend, the Tauri v1 dev server does not always pick up HTML changes. Workaround:

```bash
# In the Tauri dev window
Cmd+R (macOS) / Ctrl+R (Linux/Windows)  # reload the WebView
```

Or kill and restart `npm run tauri dev` after significant HTML changes.

---

## Releasing

Release workflow mirrors `app-workplace-memo`:

1. Update `CHANGELOG.md` with the new version
2. Bump `package.json` version
3. Bump `src-tauri/Cargo.toml` version
4. Bump `src-tauri/tauri.conf.json` → `package.version`
5. Tag: `git tag v0.1.0 && git push origin v0.1.0`
6. CI (when provisioned) builds binaries for each target platform
7. GitHub release (mirror) and Forgejo release (canonical) with checksummed artefacts

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `npm install` fails on macOS 10.13 | Node.js > 18 dropped 10.13 support | Use `nvm` to install Node 18.x |
| `cargo build` fails: missing WebKitGTK | System WebView not installed | `sudo apt install libwebkit2gtk-4.1-dev` on Linux |
| Save dialog does not appear | Tauri `allowlist.dialog.save` disabled | Check `src-tauri/tauri.conf.json` |
| Formula shows `#NAME?` | Semantic identifier not resolving | Check that the referenced line has a matching `id` in the schema |
| File opens but shows nothing | Schema validation failed silently | Check browser console for details |

---

## See also

- [README.md](./README.md) — product overview and positioning
- [ARCHITECTURE.md](./ARCHITECTURE.md) — architectural decisions and rationale
- [docs/schema.md](./docs/schema.md) — canonical JSON format specification
- [docs/engine.md](./docs/engine.md) — formula engine reference
- [docs/print-pipeline.md](./docs/print-pipeline.md) — print/PDF technical details
