---
schema: foundry-sub-agent-result-v1
task_id: BB.2
authored: 2026-04-28
authored_by: sub-agent (claude-sonnet-4-6)
authored_with: claude-sonnet-4-6
cluster: project-bim
target_decision: viewer library selection for app-workplace-bim v0.0.1 scaffold
primary_sources:
  - https://xeokit.io/
  - https://github.com/xeokit/xeokit-sdk
  - https://github.com/xeokit/sdk
  - https://github.com/ThatOpen/engine_components
  - https://docs.thatopen.com/
  - https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences
  - https://v2.tauri.app/security/http-headers/
---

# BB.2 — xeokit-sdk vs @thatopen/components: Viewer Library Selection for app-workplace-bim

Research date: 2026-04-28. All version numbers and status descriptions reflect information available at that date.

---

## 1. Feature Matrix

The two libraries share a large common capability surface but differ in architectural depth, rendering approach, and out-of-box UI completeness. The table below covers the features specifically required for `app-workplace-bim`.

| Capability | xeokit-sdk v2.6.x (stable) | @thatopen/components v3.4.0 |
|---|---|---|
| **Federated model rendering** | Yes — multiple models loaded simultaneously, each maintaining independent metadata; `loadModel()` with `id` differentiates them | Yes — FragmentsManager supports multiple `.frag` files; each model retains its identity |
| **IFC GUID-based picking** | Yes — raycasting returns entity objects carrying full IFC metadata from XKT, including GlobalId | Yes — Raycasters component returns item identifiers; IfcRelationsIndexer maps picks back to IFC data |
| **Section planes** | Yes — SectionPlanesPlugin ships in core SDK; interactive handles and caps supported | Yes — Clipper / ClipEdges components added in v2.2.0; ClipStyler handles fill styling |
| **Edge/sketch lines** | Yes — edge emphasis is a core rendering feature; configurable line color and width | Yes — ClipEdges / LOD edges added v3.3.0; X-ray edges pass added v3.4.0 |
| **Model LOD / tree-shaking** | Yes — ViewCullPlugin (frustum culling), FastNavPlugin (resolution scaling); XKT geometry reuse settings | Yes — LOD material visibility added v3.2.0; LOD edges v3.3.0; fast model picker v3.3.0 |
| **Measurement tools** | Yes — DistanceMeasurementsPlugin, AngleMeasurementsPlugin ship in core | Yes — LengthMeasurement, AngleMeasurement (v3.4.0), AreaMeasurement, VolumeMeasurement in @thatopen/components-front |
| **BCF round-trip** | Yes — BCFViewpointsPlugin in core SDK; saves/loads BCF 2.1 viewpoints with camera, visibility, selection, GUID references | Yes — BCFTopics component added v2.2.0; imports and exports BCF topics |
| **Annotations** | Yes — AnnotationsPlugin ships in core (2D labels anchored to 3D positions) | Yes — LeaderAnnotations, CalloutAnnotations, LinearAnnotations in core |
| **Navigation cube / orbit / pan / zoom** | Yes — NavCubePlugin ships in core; OrbitControls, FirstPersonNavPlugin, CameraPathAnimation | Yes — OrthoPerspectiveCamera supports orbit, first-person, plan modes; no dedicated NavCube component in current docs |
| **Double-precision rendering** | Yes — purpose-built RTC (Relative-to-Center) geometry coordinate system implemented in custom WebGL engine; handles coordinates like `[1842022, 10, -5173301]` without jitter | No — Three.js buffers geometry into Float32Array; single-precision GPU pipeline. Large-coordinate jitter is a documented Three.js ecosystem limitation; re-centring is a user-side workaround, not a built-in solution |
| **Mobile / WebGL vs WebGPU** | WebGL 1/2 today; next-gen xeokit/sdk alpha targets pluggable renderer backends (WebGPU planned); mobile browser confirmed working | Three.js underlies @thatopen; WebGL 2 today; Three.js has experimental WebGPU renderer; not yet surfaced in @thatopen/components API |
| **Model size ceiling** | Documented: 186 MB IFC → 12 MB XKT (~93% reduction); 45 MB IFC → 1.8 MB XKT (25× reduction); "hundreds of thousands of objects" stated; no hard documented ceiling | Documented: ~100 MB IFC → ~10 MB Fragments (10× reduction); web-ifc benchmark: 47 MB schependomlaan.ifc parses in 533 ms; 29 MB Holter Tower in 4,990 ms; "millions of elements in seconds" claimed |
| **Memory footprint (Schependomlaan)** | XKT loads ~1.5 MB into browser; render memory not benchmarked in published documentation | web-ifc parse on Apple M1: 533 ms for 714,485 entities, 3,569 meshes; .frag post-conversion memory not published |
| **Out-of-box UI panels** | xeokit-bim-viewer package: Objects/Classes/Storeys tree view, NavCube, X-ray, BCF viewpoint I/O, section planes, storeys navigation — all pre-built | @thatopen/ui-obc package: button/panel/table web components; Outliner component; but spatial tree, properties panel, and NavCube are not shipped as ready-to-deploy widgets — they are building blocks |

### Summary observations

xeokit-sdk ships a more complete ready-to-use BIM viewer (the `xeokit-bim-viewer` package wraps the SDK into a deployable UI), while @thatopen/components ships building blocks that require more assembly to reach equivalent UI coverage. xeokit's double-precision rendering is a structural capability, not a workaround. @thatopen's Three.js foundation cannot match it without re-centring the world at the JavaScript level, which requires deliberate per-model coordinate handling by the integrating application.

---

## 2. License Analysis

### xeokit-sdk: AGPL-3.0

xeokit-sdk is licensed under **AGPL-3.0** with a dual-licensing path. The AGPL-3.0 is available for open-source use; enterprises wanting closed-source distribution contact Creoox AG for a proprietary license (contact@creoox.com). Pricing for the commercial license is not published.

The OSArch community raised concerns (OSArch discussion #289) about historical messaging around "AGPL for non-commercial use" in older xeokit marketing copy. The current official position, confirmed by the xeokit-sdk wiki and Wikipedia, is that the license is AGPL-3.0 with commercial licensing as an alternative — not "non-commercial only." The older framing appears to have been an imprecise summary of the copyleft requirement rather than a distinct non-commercial restriction. Teams with public-release open-source products do not need the Creoox license.

**Creoox commercial license**: required only when distributing xeokit in a product whose source code will not be publicly released under AGPL-3.0 or a compatible copyleft license. For a EUPL-1.2 or AGPL-3.0 distributed product — both of which require source disclosure — the commercial license is not required.

### @thatopen/components: MIT

The `@thatopen/components` and `@thatopen/components-front` packages are MIT-licensed. `web-ifc` (the underlying IFC parser) is MPL-2.0. `@thatopen/fragments` is MIT. The Three.js dependency is MIT. No copyleft obligation arises from using this stack in any distribution model, including closed-source.

### EUPL-1.2 + AGPL-3.0 compatibility: the exact finding

The European Commission's Joinup Licensing Assistant and EUPL compatibility matrix establish the following:

**EUPL-1.2 → AGPL-3.0 (downstream, the operative direction):** The EUPL-1.2 appendix explicitly lists AGPL-3.0 as a downstream compatible license. The EUPL clause 5 states: "If the Licensee distributes or communicates copies of the Original Works or Derivative Works... based upon both the Work and another work licensed under a Compatible Licence, this Distribution or Communication can be done under the terms of this Compatible Licence." AGPL-3.0 is listed in the EUPL Appendix as such a Compatible Licence. Consequence: **a combined work integrating EUPL-1.2 code and AGPL-3.0 code (such as xeokit) may be distributed under AGPL-3.0, satisfying both licenses' source-disclosure requirements.** The resulting artifact is AGPL-3.0, not EUPL-1.2.

Source: [Interoperable Europe Portal — Matrix of EUPL compatible open source licences](https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences); [EUPL Licence Compatibility article](https://interoperable-europe.ec.europa.eu/collection/eupl/licence-compatibility-permissivity-reciprocity-and-interoperability).

**AGPL-3.0 → EUPL-1.2 (upstream direction):** The matrix marks this as **NOT compatible for incorporation**. AGPL-3.0 code cannot be statically incorporated into a EUPL-1.2 work and re-distributed under EUPL-1.2 alone; both licenses' copyleft requirements would conflict. The combined work must be distributed as AGPL-3.0 (the downstream-compatible license), not as EUPL-1.2.

**Practical result for `app-workplace-bim`:** If xeokit is embedded in the Tauri webview frontend and the product is distributed publicly, the combined work must be released under AGPL-3.0. The Rust backend (EUPL-1.2) and the JS webview frontend (AGPL-3.0) are separate processes communicating via IPC; the strong-copyleft question turns on whether the combination constitutes a "derivative work." Under AGPL-3.0's SaaS clause and derivative-work interpretation, distributing a binary that includes the xeokit JS along with the Rust binary likely makes the entire work subject to AGPL-3.0 or requires the Creoox commercial license. **This is the critical legal uncertainty: it is not resolved by the EUPL compatibility appendix alone, and it warrants a legal review before committing to xeokit in a product PointSav intends to distribute commercially or to customers under proprietary terms.**

The `BIM_Buildable Architecture.md` briefing document states that EUPL-1.2 + AGPL-3.0 is "legally clean." The matrix confirms the downstream compatibility direction exists, but "legally clean" overstates the position: the combined work must be distributed under AGPL-3.0 (source of entire combined work must be public), not under EUPL-1.2 alone. For a fully open-source product, this is fine. For a product where PointSav intends to keep the Rust backend or any application logic proprietary, it is not fine without the Creoox commercial license.

### Verdict on licensing

@thatopen/components (MIT + MPL-2.0) is unconditionally clean for any distribution model. xeokit (AGPL-3.0) is clean for open-source distributions and requires the Creoox commercial license for any closed-source distribution. The briefing document's claim that EUPL-1.2 + AGPL-3.0 is "compatible" is accurate in the narrow sense that the compatibility appendix permits the combination — but the combined work must ship as AGPL-3.0, not EUPL-1.2, and that has product-distribution implications.

---

## 3. Performance

### xeokit XKT format

XKT is a purpose-built binary geometry format with quantised positions, oct-encoded normals, and deflated geometry data. Published numbers from xeokit's own documentation:

- Revit sample project (rac_sample_project.ifc): 45.3 MB IFC → 1.78 MB XKT, 25.5× compression, loads in approximately 3–4 seconds over a standard connection. Model contained 5,563 objects, 2,871 geometries, 283,238 triangles.
- Schependomlaan: 49 MB IFC → approximately 1.5 MB XKT (estimated from briefing document and xeokit.io landing page). Loads in approximately 2–3 seconds.
- General claim: "hundreds of thousands of objects" rendered fluidly.

The XKT pipeline is: IFC → `convert2xkt` (Node.js/CLI using IfcOpenShell or web-ifc internally) → `.xkt` → browser load. The conversion step is offline; the browser load is fast.

xeokit's WebGL engine is purpose-built for BIM. It does not use Three.js. This architectural independence means xeokit does not inherit Three.js's Float32 precision ceiling, and its rendering pipeline is optimised specifically for large counts of instanced architectural elements.

### @thatopen Fragments format

Fragments is built on Google FlatBuffers and uses web-ifc (C++ compiled to WASM) for IFC parsing. Published numbers:

- web-ifc benchmark (schependomlaan.ifc, 47 MB, Apple M1, 8 GB RAM): 533 ms parse time, 714,485 entities, 3,569 meshes.
- web-ifc benchmark (Holter Tower, 29 MB): 4,990 ms parse time, 2,807,815 entities, 60,285 meshes — indicating parse time depends heavily on entity/geometry count, not file size alone.
- IFC → Fragments compression: ~100 MB IFC → ~10 MB Fragments (10× reduction, vs xeokit's 25× on comparable models).
- The 3D viewer claims 60 FPS via multi-threaded Web Worker architecture.
- ThatOpen claims "millions of elements in seconds" for pre-converted Fragments.

Source: [ThatOpen engine_web-ifc benchmark.md](https://github.com/ThatOpen/engine_web-ifc/blob/main/benchmark.md), [DeepWiki ThatOpen/engine_fragment](https://deepwiki.com/ThatOpen/engine_fragment).

### Comparative assessment

Direct head-to-head benchmarks on identical models do not exist in published literature. The comparison is complicated by different pre-processing pipelines: xeokit converts once with a CLI tool producing XKT; @thatopen converts once with web-ifc producing .frag. Both formats are then fast to load in the browser.

The structural difference is the rendering engine: xeokit's custom WebGL engine vs @thatopen's Three.js. For standard architectural models (10,000–200,000 visible elements) the difference is unlikely to be perceptible in practice. For very large federated models (500,000+ elements, large infrastructure projects, campus-scale models) xeokit's purpose-built engine and superior compression ratio provide a measurable advantage. The web-ifc Holter Tower benchmark (60,285 meshes, nearly 5 seconds to parse) suggests @thatopen's parser labors on high-geometry-count models even after compression.

**Caveats**: All numbers come from vendor-published or vendor-adjacent sources. No independent third-party benchmark comparing the two stacks on identical hardware and models has been located. Benchmark conditions (hardware, IFC version, model characteristics) differ between xeokit and @thatopen measurements.

---

## 4. Developer Ergonomics

### xeokit-sdk (v2.6.x stable)

- **Language**: JavaScript with JSDoc. The stable v2.6.x SDK does not ship TypeScript declarations. TypeScript types are available in the alpha next-gen `xeokit/sdk` repo. GitHub issue #731 ("Do you support TypeScript?") was filed and the resolution is the next-gen SDK — not backported to v2.
- **Framework adapters**: None official. xeokit is framework-agnostic WebGL; it binds to a `<canvas>` element. Integration with React/Vue/Svelte requires wrapping it in a component lifecycle hook, which is standard but not provided.
- **Documentation**: Official docs deprecated the old wiki and now live at xeokit.io and xeokit.github.io/sdk. Quality is good for the stable SDK; the next-gen SDK documentation is in progress.
- **Release cadence**: v2.6.108 was the latest as of April 3, 2026. 243 total releases, 6,291 commits on master. Active maintenance is confirmed; the stable SDK and the alpha SDK are both receiving commits.
- **Community**: Presented at AEC Hackathons in Zurich (2024 and 2025) and Wrocław (2024). OSArch forum has substantive discussions. No official Discord listed in documentation; GitHub Issues is the primary support channel.
- **Stars / forks**: 896 stars, 326 forks (xeokit-sdk repo as of research date).

### @thatopen/components (v3.4.0)

- **Language**: TypeScript (90.2% of codebase, per GitHub). Ships native TypeScript declarations. TSDoc throughout. This is a first-class TypeScript library.
- **Framework adapters**: Documented compatibility with React, Vue, Angular, Svelte, vanilla JS, Node.js, React Native, and Electron. The library is framework-neutral by design but explicitly tested on major frameworks.
- **Documentation**: docs.thatopen.com is actively maintained with tutorial-per-component structure. API reference generated from TSDoc. Quality is high.
- **Release cadence**: v3.4.0 on April 9, 2026; v3.3.2 on January 27, 2026; v3.3.0 on January 22, 2026; v3.2.0 October 2025; v3.1.0 July 2025. 15 documented releases total on the v3 line. Active development confirmed.
- **Community**: That Open Company maintains an active Discord and developer community. LinkedIn presence. Open-source BIM conferences. Well-known in the web-IFC/IFC.js community (formerly IFC.js).
- **Stars / forks**: 643 stars, 198 forks (engine_components repo as of research date).

### Summary

@thatopen/components is the stronger choice on ergonomics: native TypeScript, better framework documentation, more active community, and structured tutorial coverage. xeokit's stable SDK is JavaScript-first (types are in the alpha rewrite), which adds friction for a TypeScript Tauri project. Both are actively maintained.

---

## 5. Tauri Webview Integration Patterns

### General architecture

Both xeokit and @thatopen run in a browser environment; in Tauri 2 they run in the system webview (WKWebView on macOS/iOS, WebView2 on Windows, WebKitGTK on Linux). Neither library is modified for Tauri — they are imported as standard npm packages into the webview's JavaScript bundle.

IPC between the Rust backend and the JavaScript viewer is handled via Tauri's command system (`invoke`) or the Channel API for streaming. The typical pattern for a BIM viewer would be:

1. Rust backend reads a `.xkt` or `.frag` file from the local flat-file archive.
2. Rust emits the file bytes to the webview via a Tauri command returning raw bytes or via the asset protocol.
3. JavaScript viewer loads the bytes (ArrayBuffer) into xeokit or @thatopen.
4. User interactions (selection, property lookup) invoke Tauri commands that write back to the Rust object store.

### Asset protocol and CORS

Tauri 2's `asset:` protocol serves files from the local filesystem to the webview. The CSP must include `asset: http://asset.localhost` to load local binary files (XKT, .frag). Both libraries will consume models as ArrayBuffers or URLs — the asset protocol URL form works for this.

External CDN scripts should not be used; all JS dependencies must be bundled locally (standard for production Tauri apps).

### SharedArrayBuffer and COOP/COEP — critical @thatopen-specific issue

`web-ifc` (the @thatopen IFC parser) uses WebAssembly threads (Emscripten Pthreads) internally, which require `SharedArrayBuffer`. SharedArrayBuffer requires cross-origin isolation: the page must be served with `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` headers.

**Tauri 2.1.0+ resolves this.** The HTTP headers security feature, documented as available "since 2.1.0," allows setting COOP and COEP in `src-tauri/tauri.conf.json`:

```json
"security": {
  "headers": {
    "Cross-Origin-Opener-Policy": "same-origin",
    "Cross-Origin-Embedder-Policy": "require-corp"
  }
}
```

Source: [Tauri v2 HTTP Headers documentation](https://v2.tauri.app/security/http-headers/).

The development Vite server must also set these headers (Vite supports this via `server.headers` config). The earlier GitHub issue #5320 ("feat: SharedArrayBuffer") was closed as not planned — but that was before the headers security feature was added in Tauri 2.1.0. The 2.1.0 implementation addresses the use case via a different mechanism (HTTP response headers rather than a dedicated API).

xeokit does not use SharedArrayBuffer. Its WebGL engine is single-threaded JavaScript. It has no cross-origin isolation requirement.

**This is a non-trivial integration difference**: @thatopen with web-ifc requires explicit COOP/COEP configuration; xeokit does not. The configuration is achievable in Tauri 2.1.0+, but it adds a setup step and must be carried forward in the Vite dev config as well.

### WASM and CSP

web-ifc is a WASM binary (`web-ifc.wasm`). The Tauri CSP must include `'wasm-unsafe-eval'` in `script-src` to allow WASM execution:

```json
"csp": "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'"
```

xeokit does not use WASM in the stable SDK. No wasm-unsafe-eval requirement.

### IPC payload considerations

Large model files (1–2 MB XKT, 10 MB .frag) transferred via Tauri IPC should use the raw byte transfer ("Raw Requests") or Channel API introduced in Tauri v2, not JSON-encoded base64. Both libraries accept ArrayBuffer inputs, which maps cleanly onto Tauri's byte transfer primitives.

### Practical verdict on integration

Both libraries can be hosted in Tauri 2's webview. xeokit has fewer integration preconditions (no SharedArrayBuffer, no WASM CSP changes). @thatopen has more preconditions but they are solvable within Tauri 2.1.0+ with documented configuration. Neither requires a custom Tauri plugin.

---

## 6. AEC Interface Coverage

### xeokit out-of-box

The `xeokit-bim-viewer` package (separate from the bare `xeokit-sdk`) ships a deployable BIM viewer application with the following pre-built UI elements:

- Objects tree view (structural hierarchy from IFC)
- Classes tree view (IFC entity types)
- Storeys tree view (building stories navigation)
- NavCube (interactive orientation cube)
- BCF viewpoint save/load controls
- Section plane interactive controls
- X-ray mode toggle
- Object hide/show/isolate controls
- Model loading UI

Properties panel: **not included** in the packaged viewer. Displaying IFC property sets for a selected element requires custom implementation using the SDK's metadata API (`entity.metaObject.getMetaObjectsByType()` and related calls).

2D floor plan navigation: available as AnnotationsPlugin and SectionPlanesPlugin but not assembled into a ready-to-deploy 2D plan viewer in the standard package.

### @thatopen out-of-box

The `@thatopen/ui-obc` package (companion UI components) provides:

- Outliner component (model tree / spatial hierarchy)
- Classification table
- Properties table (property set viewer for selected elements) — this is present in @thatopen/ui-obc
- Button primitives, panels, configuration tables

BCF topic management: BCFTopics component in `@thatopen/components` handles import/export of BCF topics but does not ship a pre-styled UI for it.

Section planes: Clipper and ClipEdges components handle clipping; the front-end UI (handles, toggles) requires wiring up.

Navigation cube: **not present** in current @thatopen documentation. The camera supports orbit/pan/zoom natively; a NavCube widget would require custom implementation or a third-party Three.js NavCube component.

### Summary

Neither library ships a zero-custom-code full BIM UI. xeokit comes closer — the `xeokit-bim-viewer` package is a nearly complete deployable viewer needing only a properties panel. @thatopen provides more granular building blocks, with a properties table widget present but more custom wiring required to assemble section planes, BCF workflow, and navigation cube. The @thatopen approach is better for a custom-designed UI (which `app-workplace-bim` will eventually need); the xeokit approach is better for faster initial v0.0.1 scaffolding with less UI code.

---

## 7. Recommendation

**Recommendation: start with xeokit-sdk (AGPL-3.0), not @thatopen/components, for the v0.0.1 scaffold of `app-workplace-bim`.**

This contradicts the briefing's default tentative choice. The reasoning follows.

**The deciding factor is double-precision rendering, not license simplicity.** The `app-workplace-bim` use case involves IFC models of buildings that are frequently delivered with georeferenced site coordinates — coordinates expressed in a local or national coordinate system rather than relative to a model origin. For UK Ordnance Survey grid coordinates, Irish Grid, French Lambert, Dutch RD-New, or any coordinate system where the easting/northing values are in the millions, Three.js produces visible geometry jitter because it stores vertex positions in Float32 arrays before the GPU sees them. This is a fundamental architectural limitation of the Three.js pipeline that @thatopen inherits and cannot mitigate within its current renderer without the application re-centring every model's geometry on ingest.

xeokit built its entire rendering engine around the Relative-to-Center (RTC) pattern that flight simulators and geospatial engines use. It handles a model at coordinates `[1842022, 10, -5173301]` (a Lyon, France site) without jitter, natively, as a first-class supported workflow. The xeokit blog post on double-precision models demonstrates two models placed 259 million units apart rendering correctly in the same scene.

For `app-workplace-bim`'s target market — property managers, FM operators, small landlords — the IFC models they receive will routinely come from surveyors and architects who georeferenced the model to the national grid. Requiring the application to pre-process every model's coordinates before loading is a technical burden that will surface as bugs in the field, and it eliminates the flat-file portability value proposition (georeferenced coordinates are in the IFC file; stripping them to fix a renderer limitation would corrupt the archive). xeokit solves this problem structurally. @thatopen does not.

**The license situation is manageable.** The briefing document says EUPL-1.2 + AGPL-3.0 is "legally clean." The European Commission's compatibility matrix confirms that the EUPL-1.2 downstream compatibility appendix allows a combined EUPL + AGPL work to be distributed under AGPL-3.0. This means: if `app-workplace-bim` is distributed as open source (source published), xeokit is clean with no Creoox license required. If PointSav later intends a closed-source commercial distribution, a Creoox commercial license is required at that point. For the v0.0.1 scaffold and any open-source early releases, AGPL-3.0 distribution is the correct posture.

One clarification the briefing document needs: the combined work's license is AGPL-3.0, not EUPL-1.2. The Rust backend (if kept separately compilable and separately licensed as EUPL-1.2) may be able to maintain its EUPL-1.2 status as a separately distributable component — but the JavaScript webview bundle containing xeokit is AGPL-3.0. Legal review is warranted before any commercial launch regardless; the v0.0.1 scaffold does not require that review.

**Developer ergonomics: the TypeScript deficit is manageable.** xeokit-sdk v2.6.x ships without TypeScript declarations, which is a genuine friction point for a TypeScript Tauri project. The standard mitigation is to write a thin TypeScript facade over the xeokit API, declaring only the methods the application actually calls. This is a one-time cost of approximately two to four hours of work, producing a typed interface that isolates the application from xeokit's JavaScript internals. The `xeokit/sdk` alpha repo is a complete TypeScript rewrite and is the long-term path, but it carries alpha stability risk at v0.0.1.

**The @thatopen Tauri integration preconditions are solvable but add friction.** web-ifc's SharedArrayBuffer requirement is solvable with Tauri 2.1.0+ COOP/COEP headers and a wasm-unsafe-eval CSP entry. These are documented configurations. The risk is that WebKit (WKWebView on macOS) has historically had inconsistent cross-origin isolation behaviour. xeokit's absence of SharedArrayBuffer dependence eliminates an entire class of cross-platform webview compatibility risk.

**The faster path to a working scaffold favours xeokit.** The `xeokit-bim-viewer` package provides a near-complete viewer with tree views, BCF, section planes, and NavCube assembled. Wrapping it in a Tauri webview, adding a TypeScript IPC bridge to the Rust object store, and exposing a properties panel (custom, reading from the flat-file YAML sidecars) is the v0.0.1 scope. With @thatopen, the same v0.0.1 requires assembling more components, configuring SharedArrayBuffer, and building more custom UI. The schedule cost is real.

The recommendation is: **use xeokit-sdk v2.6.x for v0.0.1**. Write a TypeScript facade. Distribute the combined work as AGPL-3.0. Revisit the license when commercial distribution is planned and obtain a Creoox commercial license at that point if closed-source distribution is required.

---

## 8. Migration Path

### If starting with @thatopen and migrating to xeokit

Cost: moderate to high. The viewer API surfaces are entirely different. @thatopen is Three.js object references, component registries, and Fragment file paths. xeokit is its own scene graph, Entity/SceneModelMesh objects, and XKT file paths. Migration would require:

1. Replacing the conversion pipeline: `web-ifc → .frag` replaced by `convert2xkt → .xkt`. Both are CLI/Node.js offline tools; the Rust backend calling IfcOpenShell's `convert2xkt` is already in the recommended architecture.
2. Replacing the viewer API calls: raycasting, pick handlers, section plane controls, BCF viewpoint save/load — all have counterpart APIs in xeokit but must be re-implemented.
3. Replacing the model tree and properties UI: if using @thatopen/ui-obc's Outliner and PropertiesTable, those must be replaced with xeokit metadata API calls feeding a custom UI.

What stays portable: **the Rust backend entirely**. The flat-file object store, IFC metadata YAML sidecars, spatial queries, IFC GUID indexing, and all business logic in Rust are viewer-agnostic. The IPC interface between Rust and JavaScript changes only in the payload format (XKT bytes vs .frag bytes); the command structure remains the same.

The migration cost is primarily frontend JavaScript/TypeScript, not Rust. Estimate: one to two developer-weeks for a non-trivial application.

### If starting with xeokit and migrating to @thatopen

Cost: similar. The same API gap exists in reverse. xeokit entity references and XKT assets must be replaced with Fragment files and @thatopen component registrations.

The migration path from xeokit to the next-gen `xeokit/sdk` (TypeScript alpha) is a different scenario — the API is redesigned but the conceptual model (scene graph, data graph, WebGL renderer) is continuous. When `xeokit/sdk` reaches stable, migrating from `xeokit-sdk` v2.6.x should be a smaller effort than crossing to @thatopen.

### Portable Rust backend is the long-term hedge

The most important migration-cost insight is that the Rust-side architecture is fully portable. The flat-file object store pattern described in `BIM_Buildable Architecture.md` (Speckle-inspired content-addressed store with IFC-SPF, YAML sidecars, SVG, glTF) works identically regardless of which JavaScript viewer sits in the webview. The viewer is a display layer; the data layer is Rust. Designing the TypeScript IPC facade as a stable interface between the Rust backend and the webview (abstracting the specific viewer calls behind a viewer-neutral interface) makes any future viewer swap a frontend-only concern.

---

## 9. Open Questions and Risks

**License risk — commercial distribution.** The AGPL-3.0 copyleft extends to the entire distributed artifact when xeokit is included. If PointSav's commercial model involves distributing `app-workplace-bim` as a binary without publishing the full application source, the Creoox commercial license is required. The price is not published. This must be clarified before any commercial launch. Mitigation: obtain a price quote from Creoox early; factor it into the product pricing model.

**xeokit next-gen SDK stability.** The TypeScript `xeokit/sdk` is alpha. Migrating to it later reduces the TypeScript facade burden but introduces risk if the API breaks before stabilising. Monitor but do not block the v0.0.1 scaffold on it.

**web-ifc WASM SharedArrayBuffer on WebKit (macOS).** If the decision reverses to @thatopen, WKWebView's support for COOP/COEP headers with cross-origin isolation should be explicitly tested on macOS before committing to the stack. WebKit has historically been behind Chromium on web platform features. The Tauri 2.1.0 documentation confirms the header-setting mechanism; actual runtime behaviour on WKWebView with SharedArrayBuffer should be validated.

**Three.js precision for property managers' IFC models.** If @thatopen is used, every IFC model with large site coordinates must be re-centred before loading. The conversion pipeline (web-ifc → .frag) can apply this re-centring during export; IfcOpenShell's `ifcpatch` can strip IFCSITE coordinates before conversion. This is tractable but adds pipeline complexity that xeokit avoids entirely.

**Creoox marketing copy inconsistency.** The OSArch community has noted inconsistencies between Creoox's "non-commercial" marketing and the actual AGPL-3.0 license terms. The authoritative source is the SPDX AGPL-3.0 license text and the xeokit-sdk wiki, which confirm AGPL-3.0 without a non-commercial overlay. Verify any Creoox commercial license quote explicitly against this understanding.

**model size for the property-manager target.** The documented benchmarks cover commercial-scale models (Holter Tower at 29 MB, Schependomlaan at 47 MB). Property managers often have smaller models (single residential or light-commercial buildings, 5–20 MB IFC). At that scale, both stacks will be fast; the precision and out-of-box UI differences matter more than raw throughput.

**BCF round-trip fidelity.** Both libraries claim BCF support but neither is independently validated for full BCF 3.0 round-trip fidelity with a reference implementation. xeokit documents BCF 2.1 viewpoint save/load; @thatopen added BCFTopics in v2.2.0. Full BCF 3.0 topic editing (issue tracking, comments, due dates, assignees) is outside both libraries' scope — BCF I/O handles viewpoints, not full topic lifecycle management. A separate BCF client (BIMcollab Zoom, OpenProject BIM) would be the round-trip partner in a production workflow.

---

## 10. Sources

- [xeokit.io — product page and features](https://xeokit.io/)
- [xeokit-sdk GitHub repository](https://github.com/xeokit/xeokit-sdk)
- [xeokit next-gen SDK (alpha)](https://github.com/xeokit/sdk)
- [xeokit-sdk wiki — License](https://github.com/xeokit/xeokit-sdk/wiki/License)
- [xeokit blog — Viewing Double-Precision Models](https://xeokit.io/blog/viewing-double-precision-models/)
- [xeokit blog — Converting Models to XKT with convert2xkt](https://xeokit.io/blog/converting-models-to-xkt-with-convert2xkt/)
- [xeokit blog — Viewer Performance Tips](https://xeokit.io/blog/viewer-performance-tips/)
- [xeokit-bim-viewer demo](https://xeokit.github.io/xeokit-bim-viewer/)
- [xeokit Wikipedia article](https://en.wikipedia.org/wiki/Xeokit)
- [ThatOpen/engine_components GitHub repository](https://github.com/ThatOpen/engine_components)
- [ThatOpen engine_components releases](https://github.com/ThatOpen/engine_components/releases)
- [That Open docs — Getting started](https://docs.thatopen.com/components/getting-started)
- [That Open docs — IfcLoader tutorial](https://docs.thatopen.com/Tutorials/Components/Core/IfcLoader)
- [That Open docs — Fragments](https://docs.thatopen.com/Tutorials/Fragments/)
- [That Open docs — @thatopen/components API](https://docs.thatopen.com/api/@thatopen/components/)
- [ThatOpen/engine_web-ifc benchmark.md](https://github.com/ThatOpen/engine_web-ifc/blob/main/benchmark.md)
- [DeepWiki — ThatOpen/engine_fragment](https://deepwiki.com/ThatOpen/engine_fragment)
- [Interoperable Europe Portal — EUPL compatible licences matrix](https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences)
- [Interoperable Europe Portal — Licence Compatibility article](https://interoperable-europe.ec.europa.eu/collection/eupl/licence-compatibility-permissivity-reciprocity-and-interoperability)
- [Tauri v2 — Security: HTTP Headers](https://v2.tauri.app/security/http-headers/)
- [Tauri v2 — Security: CSP](https://v2.tauri.app/security/csp/)
- [OSArch — Xeokit and AGPL discussion #289](https://community.osarch.org/discussion/289/xeokit-agpl-not-quite-free-and-not-really-free-software) (access restricted; content inferred from search snippets)
- [Three.js forum — Large coordinates precision discussion](https://discourse.threejs.org/t/large-coordinates/50621)
- [xeokit Wikipedia — history and authorship](https://en.wikipedia.org/wiki/Xeokit)
