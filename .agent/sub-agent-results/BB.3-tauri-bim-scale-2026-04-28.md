---
schema: foundry-draft-v1
state: raw
originating_cluster: project-bim
target_repo: project-bim
target_path: .claude/sub-agent-results/BB.3-tauri-bim-scale-2026-04-28.md
audience: task-claude
bcsc_class: internal
language_protocol: PROSE-TECHNICAL
authored: 2026-04-28
authored_by: sub-agent (research)
authored_with: claude-sonnet-4-6
research_done_count: 14
research_suggested_count: 6
research_provenance: web-fetch + web-search across Tauri docs, GitHub issues, xeokit.io, WebKit storage blog, MDN, deepwiki.com
research_inline: true
references:
  - https://v2.tauri.app/release/
  - https://v2.tauri.app/blog/tauri-20/
  - https://v2.tauri.app/develop/sidecar/
  - https://v2.tauri.app/plugin/file-system/
  - https://v2.tauri.app/security/capabilities/
  - https://v2.tauri.app/develop/calling-rust/
  - https://v2.tauri.app/plugin/updater/
  - https://v2.tauri.app/develop/debug/
  - https://v2.tauri.app/reference/webview-versions/
  - https://xeokit.io/blog/converting-models-to-xkt-with-convert2xkt/
  - https://xeokit.io/blog/automatically-splitting-large-models-for-better-performance/
  - https://webkit.org/blog/14403/updates-to-storage-policy/
  - https://github.com/tauri-apps/tauri/discussions/6461
notes_for_editor: Internal research brief for project-bim cluster. No BCSC-sensitive content. Not for external publication.
---

# BB.3 — Tauri 2.10 at BIM Scale: Architecture Research Report

**Sub-agent brief:** Pre-scaffold research for `app-workplace-bim`  
**Cluster:** project-bim (`/srv/foundry/clones/project-bim/`)  
**Date:** 2026-04-28  
**Status:** Raw research. Not yet reviewed by Task Claude.

---

## 1. Tauri 2.10 Stable Feature Set

### Release Cadence from 2.0 to 2.10

Tauri 2.0 stable shipped on 2 October 2024. From that baseline, the 2.x minor series has advanced to 2.10.3 (released 2026-03-04) at the time of research. The release cycle averages one minor version per four to six weeks. Key versions in the progression:

| Version | Date | Notable changes |
|---|---|---|
| 2.0.0 | 2024-10-02 | Stable: mobile (iOS/Android), new ACL capability system, rewritten IPC with raw payloads, multi-webview behind unstable flag |
| 2.6.0 | 2025-Q3 | `setAutoResize` for webviews; `Monitor::work_area` getter; `Service-Worker-Allowed` header config; Linux x11 feature flag for binary size; ACL dynamic functionality moved behind feature flag; JS callbacks relocated to `window.__TAURI_INTERNALS__.callbacks` |
| 2.7.0 | 2025-Q4 | `js_init_script_on_all_frames` for iframe-spanning plugin init scripts; boxed plugin support (`AppHandle::plugin_boxed`); mobile dev external URL loading fix; mobile request body forwarding fix |
| 2.8.0 | 2026-Q1 | Window focusability (`set_focusable`); simple fullscreen (`set_simple_fullscreen`); cookie management (`set_cookie`, `delete_cookie`); document title change callbacks; `on_new_window` handlers; async mobile plugin runner; submenu icons |
| 2.9.0 | 2026-Q1 | `scroll_bar_style` for webviews; mobile back/exit button events; Android plugin `onStop`/`onDestroy`/`onRestart`/`onConfigurationChanged` hooks; Swift async plugin methods; stack-overflow fix for many commands in one invoke handler |
| 2.10.0 | 2026-02-02 | `set_simple_fullscreen` on `WebviewWindow`; Android external storage `convertFileSrc` fix (500 errors on video files); `WindowConfig::focus` defaults to `false`; webkit2gtk-rs crates updated to v2.0.2 (breaking for `with_webview` users); wry upgraded to v0.54 |
| 2.10.3 | 2026-03-04 | Patch: various stability fixes |

### Core Architecture (unchanged since 2.0)

Tauri 2.x wraps the OS-provided webview — WebKit2GTK on Linux, WKWebView on macOS/iOS, WebView2 (Chromium/Edge) on Windows, Android System WebView on Android — via the `wry` abstraction layer. The Rust core process is a full OS process; the webview is a second process. They communicate over Tauri's IPC layer.

### Multi-Surface Windows

Multiple windows (`WebviewWindow`) are stable. Multiple webviews inside a single window (the "multiwebview" feature) remains behind the `unstable` Cargo feature flag as of 2.10. Do not rely on multiwebview for `app-workplace-bim` without accepting instability. For a BIM viewer, one `WebviewWindow` per context (viewer, properties panel, document list) is the safe pattern.

### Mobile (iOS and Android)

Mobile support is stable since 2.0. Specifics:
- Android targets `.apk` and `.aab`; both are produced by default when neither flag is specified.
- iOS targets `.ipa` at `src-tauri/gen/apple/build/arm64/$APPNAME.ipa`.
- The mobile webview uses Android System WebView (Chromium-based) and WKWebView (WebKit) respectively.
- Hot-module replacement extends to mobile emulators and physical devices.
- Android requires NDK 28+ for 16 KB page size support (mandatory on Google Play for Android 15+ apps as of November 2025).
- The shell plugin (subprocess spawning) is restricted on iOS and Android to URL-opening via the Opener plugin. Sidecar execution is desktop-only. This is a hard architectural constraint for BIM workflows that depend on IfcConvert.

### Plugin Architecture

All non-core functionality is distributed as official plugins (`tauri-plugin-*` crates + `@tauri-apps/plugin-*` npm packages). Each plugin is independently versioned and has a declared stability level. Plugins used by `app-workplace-bim` will include: `tauri-plugin-fs`, `tauri-plugin-shell`, `tauri-plugin-dialog`, `tauri-plugin-process`, `tauri-plugin-updater`, and optionally `tauri-plugin-store` for structured local config.

### Migration from Earlier 2.x

The 2.0 → 2.10 migration notes most relevant to `app-workplace-bim`:

- The ACL capability system replaced the v1 `allowlist`. Every command a webview can access must be explicitly granted in a capability file. There is no "allow all" shortcut.
- The 2.6.0 ACL feature-gating change (dynamic ACL behind a feature flag) reduces compile time and binary size but requires opting in to restore dynamic ACL if needed.
- The 2.10.0 `webkit2gtk-rs` v2.0.2 update is a breaking change for any code that calls `with_webview` for platform-specific WebKit2GTK customisation (relevant if trying to pass `--js-flags` to the V8 engine — see Section 2).
- `WindowConfig::focus` defaulting to `false` in 2.10.0 means new windows do not auto-focus unless explicitly configured; relevant for modal property panels.

---

## 2. Memory Practices for Large BIM Models

### The Fundamental Constraint

The webview process obeys the same heap constraints as a browser tab. The Rust backend process has no practical memory limit beyond physical RAM. This asymmetry is the central architectural fact for BIM at scale: large model data should live in Rust, not in the JavaScript heap.

### Per-Platform JavaScript Heap Behaviour

| Platform | Webview engine | V8/JSC heap default | Notes |
|---|---|---|---|
| Linux | WebKit2GTK 2.36+ (V8 is not used; JavaScriptCore) | System-governed; typically 2–4 GB on 64-bit | WebKit2GTK 2.36 is on Ubuntu 22.04 LTS |
| macOS | WKWebView / WebKit | JSC; practical limit ~2–4 GB; process memory pressure triggers early | OS manages via memory pressure framework |
| Windows | WebView2 / Chromium | V8; 64-bit process; 4 GB addressable | WebView2 exposes `SetMemoryUsageTargetLevel` (Low/Normal) API via Tauri's `WebviewExtWindows` |
| iOS | WKWebView | JSC; RAM on device is limiting factor (4–8 GB on modern iPad) | WKWebView on iOS is a distinct process; memory pressure kills webview without killing app |
| Android | Android System WebView (Chromium) | V8; device-dependent; budget is ~500 MB–2 GB realistic | Android aggressively kills background processes |

**The --js-flags equivalent in Tauri 2.10:**

Tauri does not expose a cross-platform `--js-flags` or `--max-old-space-size` equivalent. On Linux/macOS (WebKitGTK / WKWebView) there is no V8; the JavaScript engine is JavaScriptCore, which does not accept `--max-old-space-size`. On Windows (WebView2 / V8), you would need to use `with_webview` and call the WebView2 controller's environment creation API with a custom browser argument. This path requires compiling against the `webkit2gtk-rs` or `webview2-com` bindings directly and bypasses Tauri's abstraction layer. It is not a stable pattern.

**Practical implication:** Do not attempt to raise JS heap limits as a strategy. Instead, keep the JS heap as thin as possible. The 3D viewer (xeokit or @thatopen) retains geometry buffers in GPU VRAM and ArrayBuffer memory (off-heap for V8). The property graph should live in Rust.

### RAM Footprint for a 100 MB IFC Model

No publicly reproducible benchmark for Tauri-specific BIM workloads was found. Based on the xeokit documentation and general WebGL practice:

- **Raw IFC in WebAssembly (web-ifc):** A 100 MB IFC-SPF file loaded through web-ifc's WASM parser will consume approximately 300–800 MB of WebView process RSS (the WASM heap holds the parsed entity table; geometry generation via WASM->WebGL pipeline keeps a second buffer). Anecdotal reports in the Tauri issues tracker note that files 50–80 MB consume 2–5 GB of process RSS in naive implementations that hold the full model in memory while building the mesh.
- **XKT pre-converted (xeokit's native format):** The documented example converts a 45.3 MB IFC to a 1.8 MB XKT file (25.5x compression). Loading an XKT equivalent of a 100 MB IFC (~4 MB XKT) should consume 100–300 MB of WebView process RSS. The geometry is quantised and oct-encoded, stored in typed arrays; WebGL retains the vertex buffer on the GPU.
- **Fragments (.frag, ThatOpen format):** The ThatOpen stack reports fragments load more than 10x faster than IFC. Conversion is required before shipping. Memory footprint is broadly comparable to XKT.

**Recommendation:** Never load raw IFC files directly in the production viewer. The offline-first workflow for `app-workplace-bim` is: ingest IFC via the Rust backend (calling IfcConvert sidecar), produce XKT or .frag file, cache on disk, load the compact format in the webview. This moves the expensive parsing step out of the render loop and caps webview memory consumption.

### Stream-Decoding vs. Whole-File Loading

For models too large for a single XKT file, xeokit's `ifc2gltfcxconverter` supports the `-s <MB>` flag to split at a specified maximum size per output file, producing a manifest JSON. The `XKTLoaderPlugin` loads from a manifest, combining all fragments into one `SceneModel`. Split files of 20 MB or less are the recommended ceiling for reliable browser loading. At 25x compression, a 500 MB IFC produces ~20 MB of XKT total, which splits cleanly into one or two manifest-referenced files.

ThatOpen's fragments approach similarly supports partial model loading — only visible or queried parts are loaded. This is the preferred pattern for the field-inspection persona described in the strategic source: load the floor plan for the current storey, not the whole building.

---

## 3. IPC Cost — The Rust/Webview Boundary

### The invoke() System

`invoke()` is the standard frontend-to-Rust RPC mechanism. Under the hood it uses a JSON-RPC-like protocol: all arguments and return values are serialized to JSON unless raw payloads are used. 

Performance observations from Tauri GitHub discussions:
- Sending 3 MB over basic JSON IPC: ~200 ms on Windows.
- Sending 10 MB over binary IPC: ~5 ms on macOS, ~200 ms on Windows.
- The Windows IPC performance gap is a known issue (WebView2 imposes cross-process message overhead).

Tauri 2.0 introduced two optimisations:
1. **Raw Requests:** For large data from frontend to Rust, pass a `Uint8Array` (or any `ArrayBufferView`) as the command argument. Tauri bypasses JSON serialization entirely. Use `__TAURI__.core.invoke('command', rawBinaryPayload, { headers: {...} })`.
2. **`tauri::ipc::Response`:** For large data from Rust to frontend, return `tauri::ipc::Response::new(bytes: Vec<u8>)` instead of a `serde::Serialize` type. This bypasses the JSON round-trip.
3. **Internal optimisation for medium-sized JSON:** Tauri 2.x uses `JSON.parse()` rather than JavaScript literal evaluation for payloads above approximately 10 KB, which is significantly faster for JSON with complex structures (V8 parses ~400 MB/s from JSON.parse vs ~20 MB/s from JS literal eval).

### Channel API for Streaming

The `tauri::ipc::Channel<T>` type is the recommended mechanism for streaming data from Rust to the frontend — the Tauri documentation explicitly names it as the preferred approach for "streamed HTTP responses" and similarly shaped data. It provides:

- Ordered delivery (index-stamped)
- Non-blocking: Rust sends chunks while the frontend processes earlier ones
- Bidirectional: the frontend can signal back via a separate `invoke()`

For a BIM property-graph query returning 10,000 wall elements, the correct architecture is:

```rust
#[tauri::command]
async fn query_walls(
    filter: WallFilter,
    channel: tauri::ipc::Channel<WallBatch>
) {
    // run Rust spatial query, page results in batches of ~500
    for batch in wall_store.query_walls(&filter).chunks(500) {
        channel.send(batch).unwrap();
    }
}
```

The frontend renders each batch incrementally rather than waiting for a 10k-element JSON blob. This also avoids the 200 ms Windows penalty on large single payloads.

### The `convertFileSrc` / Asset Protocol Path

For large binary files (XKT models, .frag files, images, PDF drawings) that do not require Rust preprocessing, `convertFileSrc(absolutePath)` converts a local filesystem path into an `asset://localhost/...` URL that the webview can load via the standard `<img>`, `<script>`, or `fetch()` APIs. This path bypasses IPC entirely and uses the OS's shared-memory webview pipeline, which is substantially faster than IPC for large static blobs.

**This is the primary loading path for XKT model files in `app-workplace-bim`.**

Configuration required in the capability file:
```json
{
  "identifier": "bim-asset-access",
  "windows": ["main"],
  "permissions": [
    "core:asset:default",
    {
      "identifier": "core:asset:allow-asset-protocol",
      "allow": [{ "path": "$APPDATA/models/**/*" }]
    }
  ]
}
```

### Recommendation for BIM Property-Graph Queries

| Query type | Recommended mechanism |
|---|---|
| Load XKT model file into viewer | `convertFileSrc()` + xeokit `XKTLoaderPlugin` |
| Query property graph: small result (<50 elements) | `invoke()` with JSON serialization |
| Query property graph: medium result (50–2000 elements) | `invoke()` returning `tauri::ipc::Response` with JSON bytes, bypass serialization overhead |
| Query property graph: large result (2000–10000+ elements) | `Channel<T>` with batched streaming, 500 elements/batch |
| Upload IFC for conversion | `invoke()` with raw `Uint8Array` from `File.arrayBuffer()` |
| Receive conversion progress | `Channel<ProgressEvent>` |

A local HTTP sidecar is not necessary. The Channel API covers the streaming case without the complexity of an embedded HTTP server, and avoids the port-management and security-scope overhead.

---

## 4. Mobile-Readiness for BIM

### Platform Capabilities

**iOS / iPadOS:**
- WKWebView supports WebGL2 and WebGPU (from iOS 16+ and Safari 16+). The Metal-based WebGL implementation is well-regarded for performance on Apple Silicon iPads, though Safari's WebGL conformance has historically lagged Chromium.
- Screen sizes: iPad mini (6th gen) 1488x2266, iPad Air M2 1640x2360, iPad Pro M4 2064x2752. All are high-DPI. The `devicePixelRatio` is typically 2x; BIM viewers must account for this to avoid blurry text labels.
- Storage: iOS 17+ WKWebView apps get 15% of total disk per origin (up from earlier hard caps). On a 256 GB iPad, that is ~38 GB per origin. Sufficient for a multi-building XKT cache.
- Eviction policy: least-recently-used. Data is not evicted while the app is in the foreground or has persistent storage.

**Android:**
- Android System WebView (Chromium-based) has strong WebGL2 support across devices released since 2020.
- Typical device RAM: 4–8 GB on field-class tablets (Samsung Galaxy Tab S7+). Realistic WebView budget is 1–2 GB.
- Storage: governed by the app's data partition; not subject to browser quota eviction in the same way as web content. Offline model cache in `$APPDATA` is stable.

### Battery and Rendering Cost

WebGL rendering of a BIM model is GPU-intensive. On a mobile device under field conditions:
- Continuous rotation/pan of a large model may draw 3–6 W (iPad) or 2–4 W (Android mid-range). A 40 Wh battery supports 6–10 hours of intermittent viewing.
- **Mitigation:** Implement a render-on-demand pattern. xeokit supports a `snapshots` mode where the viewer only re-renders when the scene changes. Between user interactions the GPU idles. This is the correct default for a field-inspection persona that is primarily reading properties, not flying through a model.
- Reduce draw calls: fragment-based loading loads only the relevant storey's geometry.

### Field-Use Persona Implications

The strategic source identifies the field-use persona as performing "basement walks, rooftop inspections." This implies:
- No reliable network connectivity. The offline-first architecture is non-negotiable.
- Physical constraints: gloves, bright sunlight, one-handed use. Favour large touch targets, high-contrast UI, and avoid precise multi-finger gestures for primary navigation.
- IFC data needed: a subset. A maintenance technician inspecting an HVAC unit needs the unit's properties, its containing space, associated maintenance records, and the floor plan. Not the structural steel model for the building.

**Recommendation for mobile:** Implement storey-based partial loading as the default on mobile. The Rust backend serves the floor-plan SVG for the current storey and the XKT fragment for selected element types only. The full-building 3D view is an explicit "load more" action.

### Sidecar Limitation on Mobile

Shell execute and sidecar spawning are unavailable on iOS and Android. IFC conversion (IfcConvert) cannot run on-device. The mobile app is a viewer only; IFC ingestion and conversion happen on the desktop app or on a local server. This is architecturally correct for the property-manager use case: the owner ingests the handover IFC on their workstation, the vault stores the XKT, and field staff access the vault over LAN or from a local sync.

---

## 5. Plugin Architecture for IFC Sidecar

### The Sidecar Pattern

Tauri's "external binary" (sidecar) feature bundles pre-compiled executables alongside the application. The binary is invoked via the shell plugin at runtime. This is the correct pattern for IfcConvert (CLI tool from IfcOpenShell 0.8.5).

### Binary Naming Convention

Each sidecar binary must be named with a platform-target-triple suffix. For IfcConvert:

```
src-tauri/binaries/
  IfcConvert-x86_64-unknown-linux-gnu      # Linux x86_64
  IfcConvert-aarch64-unknown-linux-gnu     # Linux ARM64 (cloud/Raspberry Pi)
  IfcConvert-x86_64-apple-darwin           # macOS Intel
  IfcConvert-aarch64-apple-darwin          # macOS Apple Silicon
  IfcConvert-x86_64-pc-windows-msvc.exe   # Windows x86_64
```

Obtain the current platform's triple with: `rustc --print host-tuple`

IfcOpenShell 0.8.5 ships pre-built IfcConvert binaries for all five targets above via the GitHub Releases page. Download and rename for the sidecar layout.

### tauri.conf.json Sidecar Declaration

```json
{
  "bundle": {
    "externalBin": [
      "binaries/IfcConvert"
    ]
  }
}
```

### Capability File for Shell Execute

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "bim-ifc-convert",
  "description": "Allows spawning IfcConvert sidecar for IFC→glTF/XKT conversion",
  "windows": ["main"],
  "platforms": ["linux", "macos", "windows"],
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "binaries/IfcConvert",
          "sidecar": true,
          "args": true
        }
      ]
    }
  ]
}
```

The `"args": true` field allows any arguments to be passed. If tighter security is required, replace with an array of argument patterns using regex validators:

```json
"args": [
  { "validator": "^[\\w/\\-\\.]+\\.ifc$" },
  "--use-world-coords",
  { "validator": "^[\\w/\\-\\.]+\\.(glb|gltf)$" }
]
```

### Invoking from Rust

```rust
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

#[tauri::command]
async fn convert_ifc(
    app: tauri::AppHandle,
    input_path: String,
    output_path: String,
    progress: tauri::ipc::Channel<String>
) -> Result<(), String> {
    let (mut rx, mut _child) = app
        .shell()
        .sidecar("binaries/IfcConvert")
        .map_err(|e| e.to_string())?
        .args([&input_path, &output_path, "--use-world-coords"])
        .spawn()
        .map_err(|e| e.to_string())?;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                progress.send(String::from_utf8_lossy(&line).into_owned()).ok();
            }
            CommandEvent::Terminated(payload) => {
                if payload.code != Some(0) {
                    return Err(format!("IfcConvert exited with {:?}", payload.code));
                }
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
```

The `Channel<String>` sends conversion progress lines back to the frontend in real time.

### Sandbox Considerations

Tauri 2.x does not run sidecars in an OS-level sandbox beyond the ambient process permissions of the parent. On macOS, App Sandbox (required for App Store distribution) would require the app to have the `com.apple.security.temporary-exception.files.absolute-path.read-write` entitlement for the paths IfcConvert reads and writes. For direct distribution (outside the App Store), sandboxing is opt-in. Foundry's EUPL-licensed distribution path avoids the App Store constraint on the first version.

On Linux, a security-conscious deployment can invoke IfcConvert inside a `bwrap` (bubblewrap) sandbox by wrapping the sidecar invocation. This is an optional hardening step.

### IDS Validation (ifctester)

The same sidecar pattern applies to `ifctester` for IDS 1.0 validation at ingest time. IfcOpenShell 0.8.5 ships ifctester as a CLI entrypoint in the Python distribution. For Tauri sidecar use, prefer the pre-compiled `ifctester` binary from IfcOpenShell's GitHub Releases, or build a thin Rust wrapper around libIfcOpenShell for in-process validation.

---

## 6. Filesystem API and the Flat-File Vault

### Tauri 2.10 Filesystem Plugin

The `tauri-plugin-fs` plugin provides filesystem access from the webview via JavaScript (`@tauri-apps/plugin-fs`) and from Rust via the standard `std::fs` API (no plugin needed for Rust code).

### Permission Model

All filesystem operations are blocked by default. Access is granted through capability files using the scope system. Key points:

- **Path variables:** `$APPDATA`, `$APPCONFIG`, `$APPLOCALDATA`, `$HOME`, `$DOWNLOAD`, `$DOCUMENT`, `$DESKTOP`, `$RESOURCE`, `$TEMP`, `$PUBLIC` are available. Custom absolute paths are also valid.
- **Glob patterns:** `$APPDATA/**/*` grants recursive access.
- **Deny precedence:** deny rules override allow rules. Use deny to carve out sensitive subdirectories (e.g., deny `$HOME/.ssh/**`).
- **Mobile restriction:** On iOS and Android, filesystem access from the webview JS layer is restricted to the application's own data directory. The Rust backend can access the full filesystem (subject to OS permissions), but the JS `fs` plugin cannot.

### Flat-File Vault Capability Configuration

For `app-workplace-bim`, the vault directory layout is under `$APPDATA/vault/`. A suitable capability:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "bim-vault-access",
  "description": "Read access to the BIM vault; write to working directory only",
  "windows": ["main"],
  "permissions": [
    "fs:default",
    {
      "identifier": "fs:allow-read-recursive",
      "allow": [{ "path": "$APPDATA/vault/**/*" }]
    },
    {
      "identifier": "fs:allow-write-recursive",
      "allow": [{ "path": "$APPDATA/vault/working/**/*" }]
    },
    {
      "identifier": "fs:allow-read-recursive",
      "allow": [{ "path": "$HOME/Documents/bim-vault/**/*" }]
    }
  ]
}
```

The read-only grant on the parent vault plus write access to a `working/` subdirectory prevents accidental corruption of the archival IFC files.

### File Watching

```javascript
import { watch, watchImmediate } from '@tauri-apps/plugin-fs';

// Debounced — fires after a quiet period following changes
const unwatch = await watch(
  '$APPDATA/vault',
  (event) => console.log('Vault changed:', event),
  { recursive: true }
);

// Immediate — fires on every filesystem event
const unwatchImmediate = await watchImmediate(
  '$APPDATA/vault/incoming',
  (event) => handleIncomingIFC(event),
  { recursive: false }
);
```

The `watch` feature must be enabled in `Cargo.toml`:
```toml
tauri-plugin-fs = { version = "2", features = ["watch"] }
```

### Capability Model in 2.10

The 2.10 ACL model is declarative and compile-time verified. Capabilities reference permissions by identifier; the Tauri build system validates that all referenced permissions exist. The `removeUnusedCommands = true` option (available since 2.4) strips commands that are never allowed in any capability, reducing binary size. This is worth enabling in `app-workplace-bim` once the capability files are stable.

---

## 7. Build Size and Binary Size

### Typical Sizes for a Tauri 2.x App (No Sidecar)

Published benchmarks for a minimal Tauri 2.x app with a small Vite/React frontend:

| Format | Typical size |
|---|---|
| Linux `.deb` | 3–8 MB |
| Linux `.AppImage` | 7–15 MB |
| macOS `.dmg` (universal binary) | 8–20 MB |
| Windows `.msi` | 5–15 MB |
| Windows `.exe` (NSIS installer) | 4–12 MB |

For `app-workplace-bim` with the JavaScript BIM viewer bundle (xeokit or @thatopen, which can be 2–5 MB minified/gzipped) and the TypeScript frontend, expect the baseline installer to be 15–30 MB before sidecars.

### Sidecar Impact: IfcConvert

IfcConvert from IfcOpenShell 0.8.5 is a statically linked C++ binary:
- Linux x86_64: ~40–60 MB (includes IfcOpenShell + OpenCascade + OCCT geometry kernel)
- macOS universal: ~80–120 MB
- Windows x86_64: ~50–70 MB

Adding IfcConvert as a sidecar substantially increases installer size. Strategies to mitigate:
1. **On-demand download:** Ship the app without IfcConvert; download it on first-run via a Rust download-and-verify step. Store in `$APPDATA/bin/`. Verified against the IfcOpenShell release SHA256. This keeps the initial installer under 30 MB.
2. **Lazy bundling:** Include IfcConvert only in the "desktop full" variant; ship a viewer-only variant for mobile and for users who receive pre-converted vaults from a collaborator.

### Sidecar Impact: Python venv (not recommended)

Do not bundle IfcOpenShell as a Python package with a venv. A minimal IfcOpenShell Python environment is 150–300 MB. Use the pre-compiled IfcConvert binary instead.

### Android and iOS

No publicly published size benchmarks for Tauri mobile bundles were found. A Tauri Android `.aab` will include the Rust binary for each target ABI (armeabi-v7a, arm64-v8a), plus the frontend assets. Estimate 20–40 MB `.aab` before IfcConvert (which is not bundled on mobile; see Section 4).

Cargo profile optimisation for release builds (recommended for all targets):

```toml
[profile.release]
codegen-units = 1
lto = true
opt-level = "s"
panic = "abort"
strip = true
```

The `removeUnusedCommands = true` ACL option in `tauri.conf.json` further reduces the Rust binary size.

---

## 8. Developer Ergonomics

### Development Workflow

```bash
# Start dev server (Vite + Tauri)
cargo tauri dev

# Equivalent with explicit beforeDevCommand:
# Vite dev server starts, Tauri compiles Rust core, webview opens
```

`tauri dev` manages the Vite dev server (via `build.beforeDevCommand`), watches Rust source files for recompilation, and hot-reloads the webview on frontend changes. Rust changes trigger a full recompile (typically 5–20 seconds for incremental builds on a warm cache).

### Hot Reload

Frontend HMR (Hot Module Replacement) works out of the box with Vite. The webview refreshes on TypeScript/HTML/CSS changes without restarting the Rust core. This is the standard fast inner loop.

Rust changes to Tauri commands do require recompile + restart. For BIM work, the Rust side (file I/O, spatial queries, IFC conversion coordination) changes less frequently than the viewer TypeScript. The dev experience is tolerable.

### Debugging

**Webview (JavaScript/TypeScript):**
Right-click in the webview → Inspect Element. Opens a platform-native dev tools:
- Linux: WebKit Inspector
- macOS: Safari Web Inspector
- Windows: Microsoft Edge DevTools (Chromium-based; familiar to Chromium users)

Programmatic access from Rust: `window.open_devtools()` (requires the `devtools` Cargo feature in dev/debug builds).

**Rust backend:**
Use `println!()` / `tracing` crate to the terminal where `tauri dev` runs. For attach-and-debug, VS Code + the LLDB extension supports breakpoints in the Rust core process. Set `RUST_BACKTRACE=1` for crash traces.

**IPC round-trip tracing:**
CrabNebula DevTools (open-source, available as a Tauri plugin) provides a visual timeline of IPC calls, events, and command invocations. Recommended for diagnosing latency in property-graph query paths.

**Mobile debugging:**
- Android: Android Studio's logcat captures Rust panics and `println!` output. Chrome DevTools remote debugging works for the webview via `chrome://inspect`.
- iOS: Xcode's device console captures the Rust process output; Safari's Web Inspector attaches to the WKWebView.

### Common Development Pitfalls

- **Capability not declared:** Commands inaccessible from the frontend produce opaque errors. Enable `RUST_LOG=tauri=debug` to see ACL rejection logs.
- **serde_json deserialization mismatch:** Command argument types must exactly match the TypeScript payload shape. The `specta` crate (generates TypeScript types from Rust types) reduces this class of bug.
- **Windows IPC latency:** 200 ms for a 3 MB payload is enough to stall the UI. Always use raw payloads or `convertFileSrc` for large data on Windows.
- **WebView2 installation dependency on Windows 10:** WebView2 is pre-installed on Windows 11 and has been pushed to Windows 10 via Automatic Updates since 2021, but it is possible to encounter a system without it. Include the WebView2 bootstrapper in the NSIS installer to handle this case.

---

## 9. Production Concerns

### Auto-Update

The `tauri-plugin-updater` plugin handles in-app updates. Key constraints:

- **Signature verification is mandatory and cannot be disabled.** A keypair must be generated (`tauri signer generate`). The private key signs installer artifacts; the public key is embedded in `tauri.conf.json`.
- **Mobile auto-update is not supported** by the Tauri updater plugin. Android and iOS updates go through the respective app stores.
- **Update server:** Can be a static JSON file on any CDN (GitHub Releases, S3, Cloudflare R2). The JSON lists platform-specific download URLs and their signatures. Serves HTTP 204 when no update is available, HTTP 200 with update metadata otherwise.
- **The private signing key must never be lost.** Losing it means the next update cannot be verified and users are stuck on the last version. Store in a hardware security module or at minimum an encrypted vault (1Password, Bitwarden, age-encrypted file).

Example `tauri.conf.json` updater configuration:
```json
{
  "plugins": {
    "updater": {
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk...",
      "endpoints": [
        "https://updates.woodfinegroup.com/app-workplace-bim/{{target}}/{{arch}}/{{current_version}}"
      ],
      "dialog": true
    }
  }
}
```

### Code Signing

| Platform | Requirement | Notes |
|---|---|---|
| macOS | Developer ID Application certificate + Notarization | Gatekeeper blocks unsigned apps. Notarization is automated during `tauri build`. Requires Apple Developer Program ($99/year). App Store requires separate provisioning profile. |
| Windows | OV or EV code signing certificate | Prevents SmartScreen warning. EV certificates ($300–500/year) get immediate reputation; OV builds reputation over time. Azure Key Vault integration available. |
| Linux | GPG-signed AppImage optional | No system enforcement. Users who download `.deb` via apt repository get GPG verification at the apt layer. |
| iOS | Apple provisioning profile | Mandatory. Ad-hoc distribution possible for testing; App Store requires submission. |
| Android | Keystore file | Required for Play Store. Generate once; store securely (same key management rules as updater private key). |

### Telemetry

Tauri itself does not collect telemetry from end-user applications. The Tauri CLI collects no usage telemetry at build time. 

WebView2 on Windows ships with Chromium-based telemetry enabled by default in the underlying WebView2 runtime. This is Microsoft Edge telemetry at the webview component level, not application-level telemetry. For a privacy-first deployment, disable via the Windows Group Policy `InternetExplorer/AllowSuggestedSites` and Edge-specific policies, or instruct enterprise customers to manage this via their MDM. Tauri does not provide a programmatic API to disable WebView2 telemetry from application code.

For `app-workplace-bim`, Foundry's privacy stance (SYS-ADR-07 — structured data never routes through AI; the flat-file sovereignty architecture) requires that no model data, user identifiers, or building metadata leave the device without an explicit user action. This is achieved architecturally: no third-party SDK is bundled, no analytics endpoint is configured, no crash reporter is enabled. If crash reporting is added in a later version, it should use a self-hosted service (e.g., Sentry self-hosted) and transmit only stack traces, not model data.

---

## 10. Recommendation — Concrete Scaffold

### `src-tauri/tauri.conf.json` Skeleton

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "app-workplace-bim",
  "version": "0.1.0",
  "identifier": "com.woodfinegroup.app-workplace-bim",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Workplace BIM",
        "width": 1440,
        "height": 900,
        "minWidth": 1024,
        "minHeight": 768,
        "resizable": true,
        "fullscreen": false,
        "focus": true,
        "decorations": true
      }
    ],
    "security": {
      "capabilities": [
        "bim-vault-access",
        "bim-asset-access",
        "bim-ifc-convert"
      ],
      "dangerousDisableAssetCspModification": false,
      "csp": "default-src 'self' asset: http://asset.localhost; script-src 'self'; style-src 'self' 'unsafe-inline'",
      "removeUnusedCommands": true
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.icns", "icons/icon.ico"],
    "externalBin": ["binaries/IfcConvert"],
    "resources": [],
    "publisher": "PointSav Digital Systems",
    "copyright": "Copyright 2026 PointSav Digital Systems",
    "license": "EUPL-1.2",
    "windows": {
      "wix": {},
      "nsis": {
        "installMode": "perMachine"
      },
      "webviewInstallMode": {
        "type": "downloadBootstrapper"
      }
    },
    "macOS": {
      "entitlements": "entitlements.plist",
      "exceptionDomain": "",
      "frameworks": [],
      "signingIdentity": null,
      "providerShortName": null
    }
  },
  "plugins": {
    "updater": {
      "pubkey": "REPLACE_WITH_GENERATED_PUBLIC_KEY",
      "endpoints": [
        "https://updates.woodfinegroup.com/bim/{{target}}/{{arch}}/{{current_version}}"
      ],
      "dialog": true
    },
    "shell": {
      "open": false
    }
  }
}
```

### `src-tauri/Cargo.toml` Skeleton

```toml
[package]
name = "app-workplace-bim"
version = "0.1.0"
edition = "2021"
rust-version = "1.77"

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-fs = { version = "2", features = ["watch"] }
tauri-plugin-shell = { version = "2" }
tauri-plugin-dialog = { version = "2" }
tauri-plugin-updater = { version = "2" }
tauri-plugin-process = { version = "2" }

# IFC property graph store
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Spatial queries on property graphs
# (no Rust IFC parser yet; IfcConvert handles geometry; 
#  property metadata stored as JSON after extraction)

# Async runtime
tokio = { version = "1", features = ["full"] }

# Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.release]
codegen-units = 1
lto = true
opt-level = "s"
panic = "abort"
strip = true
```

### Capability Files Layout

```
src-tauri/capabilities/
  bim-vault-access.json     # fs read/write scoped to vault paths
  bim-asset-access.json     # asset protocol for XKT/frag files
  bim-ifc-convert.json      # shell:allow-execute for IfcConvert sidecar
  bim-core.json             # core:default permissions
```

### Data Flow Architecture

```
 IFC File (disk)
      │
      ▼
 Rust backend: invoke("convert_ifc", input, output)
      │
      ├─ spawns IfcConvert sidecar
      │   └─ produces .xkt file + metadata.json
      │
      ├─ streams progress via Channel<String>
      │
      └─ writes metadata to vault (JSON sidecar with IFC GUIDs)

 XKT File (disk, $APPDATA/vault/models/<guid>.xkt)
      │
      ▼
 convertFileSrc(path) → asset://localhost/...
      │
      ▼
 xeokit XKTLoaderPlugin.load(assetUrl)    ← no IPC, uses asset protocol
      │
      ▼
 3D viewer in webview

 Property query: invoke("query_elements", { ifcType: "IFCWALL", filter: ... })
      │
      ▼
 Rust: reads JSON metadata files, applies filter
      │
      ├─ < 500 results: return as JSON via tauri::ipc::Response
      └─ >= 500 results: stream via Channel<ElementBatch>
```

---

## 11. Open Questions and Risks

### Confirmed Risks

1. **XKT conversion toolchain on Windows is Node.js-dependent for the `convert2xkt` approach.** Node.js has a hard 1 GB heap limit for string operations, which causes the converter to fail on very large IFC files (~500 MB+). The `ifc2gltfcxconverter` tool (a C++ binary in the IfcOpenShell ecosystem) does not have this limit and should be the primary conversion path for large models. This binary is not as prominently documented as `convert2xkt`.

2. **IfcConvert sidecar size (~50–120 MB).** The installer size impact is large. The on-demand download strategy (Section 7) is recommended but adds complexity (SHA256 verification, progress feedback, offline install story for air-gapped sites).

3. **Multiwebview remains unstable.** If the desired UX requires multiple concurrent 3D viewports in one window (e.g., split-screen comparison of two model states), this is not available without the `unstable` feature flag as of 2.10.

4. **Windows IPC latency for large property graph results.** The ~200 ms overhead for 3 MB payloads on Windows is real and reproducible. The Channel API mitigates this for streaming queries, but any single large response (e.g., exporting all properties to JSON) will be slow on Windows via IPC. The workaround is to write the export to a temp file and use `convertFileSrc` or a direct `download` call.

5. **iOS WebGL in WKWebView is Safari-based.** Known Apple quirks:
   - No persistent WebGL context across backgrounding: the context may be lost when the app is sent to background. xeokit handles WebGL context loss via its `contextLostHandler`. Verify this is wired correctly.
   - Metal-backed WebGL may behave differently from Chromium WebGL on Android: test on physical hardware early.
   - WKWebView does not support WebGPU in an iOS app's WKWebView as of iOS 17 (WebGPU is available in Safari browser but not in embedded WKWebView). Confirm WebGL2 suffices.

6. **Rust IFC parsing is not production-ready.** The `ifc_rs` (0.1.0-alpha) and `ifc-lite-core` projects are both pre-production. Property extraction from IFC files in the Rust backend must currently go through IfcConvert's JSON output (`IfcConvert --json`) or through a Python/WASM-bound IfcOpenShell step. Tracking: expect `ifc-lite-core` to reach production viability for read-only property extraction in late 2026 based on the development trajectory described in the strategic source.

### Unconfirmed / Needs Investigation

7. **Tauri mobile + WebGL2 on Android low-memory devices:** The documented Android budget of ~500 MB–2 GB WebView process is device-dependent. Field devices (ruggedised tablets, older Samsung Tab A models) may have less. Testing on a representative low-spec device is required before a mobile-first launch.

8. **`convert2xkt` with manifest-based split loading in xeokit:** The split-manifest workflow is documented in xeokit's blog but the toolchain maturity for production use needs verification. The Tauri sidecar would need to call `ifc2gltfcxconverter` → `convert2xkt` in sequence. This is a two-step sidecar chain: either bundle both binaries or build a Rust orchestrator that calls them sequentially.

9. **IfcConvert 0.8.5 support for IFC 4.3 (ISO 16739-1:2024):** The strategic source notes IFC 4.3 is the target format. IfcOpenShell 0.8.5 was released April 2026 and should support IFC 4.3. Verify IFC 4.3 round-trip fidelity before committing to it as the archival format.

10. **WKWebView storage persistence in Tauri apps:** The 15%-of-disk quota per origin (iOS 17+) applies to browser apps and apps embedding WKWebView. In a Tauri app, the "origin" is `tauri://localhost`. Verify that Tauri's WKWebView instance uses persistent storage configuration (WKWebsiteDataStore.defaultDataStore) rather than an ephemeral store. Ephemeral stores are wiped on app restart; IndexedDB-cached XKT files would be lost.

11. **IfcConvert sandboxing on macOS with App Sandbox:** If App Store distribution is later required, the sidecar invocation requires sandbox entitlements. Verify the entitlement path before investing in the App Store distribution workflow.

---

## 12. Sources

- Tauri 2.0 Stable Release blog post: https://v2.tauri.app/blog/tauri-20/
- Tauri Core Ecosystem Releases (index): https://v2.tauri.app/release/
- Tauri v2.6.0 release notes: https://v2.tauri.app/release/tauri/v2.6.0/
- Tauri v2.7.0 release notes: https://v2.tauri.app/release/tauri/v2.7.0/
- Tauri v2.8.0 release notes: https://v2.tauri.app/release/tauri/v2.8.0/
- Tauri v2.9.0 release notes: https://v2.tauri.app/release/tauri/v2.9.0/
- Tauri v2.10.0 release notes: https://v2.tauri.app/release/tauri/v2.10.0/
- Tauri Capabilities documentation: https://v2.tauri.app/security/capabilities/
- Tauri Filesystem plugin: https://v2.tauri.app/plugin/file-system/
- Tauri Sidecar (Embedding External Binaries): https://v2.tauri.app/develop/sidecar/
- Tauri Shell plugin: https://v2.tauri.app/plugin/shell/
- Tauri Calling Rust from Frontend: https://v2.tauri.app/develop/calling-rust/
- Tauri Updater plugin: https://v2.tauri.app/plugin/updater/
- Tauri Debug guide: https://v2.tauri.app/develop/debug/
- Tauri Webview Versions reference: https://v2.tauri.app/reference/webview-versions/
- Tauri App Size documentation: https://v2.tauri.app/concept/size/
- Tauri IPC Concepts: https://v2.tauri.app/concept/inter-process-communication/
- Tauri memory discussion #6461: https://github.com/tauri-apps/tauri/discussions/6461
- Tauri IPC improvements discussion #5690: https://github.com/tauri-apps/tauri/discussions/5690
- xeokit — Converting Models to XKT with convert2xkt: https://xeokit.io/blog/converting-models-to-xkt-with-convert2xkt/
- xeokit — Automatically Splitting Large Models: https://xeokit.io/blog/automatically-splitting-large-models-for-better-performance/
- ThatOpen engine_web-ifc GitHub: https://github.com/ThatOpen/engine_web-ifc
- WebKit Storage Policy updates (iOS 17+): https://webkit.org/blog/14403/updates-to-storage-policy/
- MDN Storage quotas and eviction criteria: https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria
- WebGL in Mobile Development — challenges: https://blog.pixelfreestudio.com/webgl-in-mobile-development-challenges-and-solutions/
- Tauri macOS Code Signing: https://v2.tauri.app/distribute/sign/macos/
- Tauri Windows Code Signing: https://v2.tauri.app/distribute/sign/windows/
- AlterSquare — Handling Large IFC Files in Web Applications: https://altersquare.io/handling-large-ifc-files-in-web-applications-performance-optimization-guide/

---

## Research Trail

### Done
1. Tauri 2.x release changelog from 2.6 through 2.10.3 — extracted via release pages and blog post
2. Tauri IPC model — invoke(), raw payloads, Channel API, convertFileSrc/asset protocol
3. Tauri capability and permission system — filesystem, shell, asset
4. Tauri sidecar pattern — external binary naming, configuration, Rust invocation
5. Tauri memory model — webview JS heap, Rust backend, per-platform constraints
6. xeokit XKT format — compression ratios (25x documented), split-manifest for large models
7. ThatOpen web-ifc fragments — memory characteristics, conversion workflow
8. iOS WKWebView storage quotas — 15% of disk per origin (iOS 17+), eviction policy
9. WebGL on mobile — iOS Metal/WebGL, Android, battery management, field-use implications
10. Tauri mobile — sidecar restriction on iOS/Android, mobile capabilities, Android NDK requirement
11. Tauri auto-update — mandatory signature verification, server options, mobile not supported
12. Tauri code signing — macOS notarization, Windows SmartScreen, Linux GPG
13. Tauri privacy/telemetry — no application telemetry; WebView2 has Chromium telemetry at runtime level
14. Developer ergonomics — hot reload, debugging, CrabNebula DevTools

### Suggested for Next Session
1. Verify IfcOpenShell 0.8.5 IfcConvert support for IFC 4.3 (ISO 16739-1:2024) — test conversion of a real IFC 4.3 file
2. Benchmark `convertFileSrc` load times for 2 MB, 10 MB, and 50 MB XKT files on macOS, Windows, and Linux under Tauri 2.10
3. Investigate `ifc2gltfcxconverter` binary availability, licensing, and integration as the primary large-model conversion sidecar (preferable to `convert2xkt` Node.js for large files)
4. Confirm WKWebView persistent storage configuration in Tauri (default vs ephemeral data store) and test IndexedDB persistence across app restarts on iOS
5. Test WebGL context loss/restore in xeokit under iOS backgrounding (WKWebView app lifecycle)
6. Evaluate `specta` crate for TypeScript type generation from Tauri Rust commands to reduce IPC type mismatch bugs

### Open Questions
1. Does `tauri::ipc::Channel<T>` maintain ordering guarantees across the Windows IPC boundary (IPC2 vs Pipe)? The 200 ms Windows latency for 3 MB payloads — does it apply per-send to Channel messages too, or only to single invoke() calls?
2. What is the actual WebView process memory budget on a 4 GB Android tablet when loading a 4 MB XKT file alongside the xeokit viewer JavaScript (~3 MB)? No reproducible benchmark found.
3. Is xeokit's `WebIFCLoaderPlugin` (direct IFC loading without pre-conversion) viable for the initial prototyping phase, accepting its performance limitations? Threshold for acceptable load time in a field-inspection context needs user research.
