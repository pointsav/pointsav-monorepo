# Architecture — Workplace\*Proforma

Architectural decisions, dependency sovereignty audit, and technical rationale.

---

## Guiding Principle

Every layer of this stack must pass a single test: **can we fork it, rewrite it, and own it completely, with no dependency on a US hyperscaler's goodwill, infrastructure, or legal jurisdiction?**

On Linux — the production sovereign platform — the answer is yes at every layer.

---

## Relationship to Workplace\*Memo

Workplace\*Memo and Workplace\*Proforma are sibling applications that share a philosophical and architectural lineage but not a codebase. This is deliberate:

- **Same licensing posture** (EUPL v1.2)
- **Same stack** (Tauri v1 for macOS 10.13 dev, Tauri v2 for Linux prod)
- **Same security discipline** (CSP `connect-src 'none'`, minimal IPC surface, path-traversal canonicalisation, CSP-locked WebView)
- **Same file-is-the-product commitment** (self-contained canonical format, embedded fonts or JSON, no hidden sidecars)
- **Same sovereignty profile** (DINUM / ZenDiS / openDesk alignment)

But **independent UX trajectories**. Memo preserves Word muscle memory; Proforma preserves Excel muscle memory. Each application evolves toward category-leading UX for its own users, without compromise forced by shared chrome. There is no `workplace-shared/` directory. Each application vendors its own dependencies.

The shared lineage lives in the chrome tokens (CSS custom properties defining colour, spacing, typography size) which are deliberately aligned so both applications look like members of the same family when viewed side-by-side. The *content surface* — what happens inside the canvas — diverges freely.

---

## Decision Record

### ADR-001: Tauri over Electron

**Decision:** Use Tauri (Rust + OS WebView) rather than Electron (Chromium bundled).

**Rationale:**
- Tauri bundles are 8–15 MB vs Electron's 150+ MB
- Tauri uses the OS WebView which receives security patches from OS maintainers, not from application release cycles
- Tauri is governed by the Tauri Foundation within the Dutch non-profit Commons Conservancy (EU jurisdiction, MIT + Apache 2.0)
- Electron bundles a specific Chromium version controlled by Google

**Trade-off:** WebView behaviour differs across OS (WKWebView/WebView2/WebKitGTK). CSS `@page` support and print fidelity vary. Mitigated by Linux-first development and a test matrix.

**Inheritance:** This is the same decision made in `app-workplace-memo` ADR-001. The decision propagates through the family because the rationale is identical at the sovereignty level.

---

### ADR-002: Tauri v1 for macOS 10.13, Tauri v2 for Linux

**Decision:** Use Tauri v1.x for the macOS 10.13 development build. Use Tauri v2.x for the Linux production build.

**Rationale:**
- Tauri v2 requires macOS 10.15+. The current development machine runs 10.13 (High Sierra)
- The application code (HTML/CSS/JS frontend, Rust IPC commands) is identical between v1 and v2
- Only the `src-tauri/Cargo.toml` dependency versions and `tauri.conf.json` schema differ
- The `package.json` scripts and notes block document the v1/v2 migration

**Migration path:** When the development machine is upgraded to macOS 10.15+, update `src-tauri/Cargo.toml` to Tauri v2 and remove the macOS-specific workarounds documented in `DEVELOPMENT.md`.

**Inheritance:** Same decision as memo ADR-002.

---

### ADR-003: Canonical JSON file format

**Decision:** The native save format is a self-contained `.json` file conforming to the schema specified in [`docs/schema.md`](./docs/schema.md).

**Rationale:**
- JSON is an ISO standard (ISO/IEC 21778:2017), IETF Internet Standard (RFC 8259 / STD 90), and ECMA standard (ECMA-404) — stronger institutional backing for long-term archival survival than `.xlsx`, `.numbers`, or `.gsheet`
- Plain text — readable in any editor on any OS in any decade
- Self-describing with semantic field names (not positional codes) — an LLM reading the raw JSON understands the financial model without an external parser
- Diff-friendly with enforced canonical formatting — standard `git diff` shows meaningful cell-level changes
- The schema is published as a public specification; any competent engineer can implement a compatible reader from the specification alone

**Why not CSV:** CSV was evaluated and rejected as a secondary "exhaust" file. CSV cannot carry formatting, named ranges, conditional highlighting rules, or metadata that survives transport. Generating CSV alongside JSON would double the file surface without adding any capability the canonical JSON does not already have. One canonical format per application — JSON — with PDF as print exhaust and XLSX as legacy exhaust.

**Why not XLSX as canonical:** XLSX is an opaque zipped XML bundle requiring Microsoft's schema interpretation. It has broken backward compatibility three times since 1995. It is structurally unsuitable for a fifty-year operational horizon.

---

### ADR-004: EUPL v1.2 application licence

**Decision:** Licence Workplace*Proforma under the European Union Public Licence v1.2.

**Rationale:**
- Designed by the European Commission for European public sector software
- Available in all 23 EU official languages
- Copyleft (like GPL) — prevents a hyperscaler from taking the code and distributing it as a proprietary product
- Explicitly compatible with GPL v2/v3 and AGPL v3
- Jurisdiction clause: disputes governed by the law of the EU member state where the licensor is established
- Used by openDesk (Germany) and several La Suite Numérique (France) components

**Inheritance:** Same decision as memo ADR-004. Both applications ship under EUPL-1.2.

---

### ADR-005: Phase 1 JavaScript formula engine; Phase 2 IronCalc via IPC

**Decision:** The MVP embeds a pure JavaScript formula engine (`src/js/engine.js`) sufficient for arithmetic, cell references, ranges, and the common functions institutional users need: SUM, AVERAGE, MIN, MAX, COUNT, IF, ROUND, ABS, PMT, PV, FV, NPV, IRR.

**Phase 2** replaces this with IronCalc (Apache 2.0, pure Rust, NLnet + European Commission funded) invoked through a Tauri IPC command. The public API of the engine module — `setCell`, `getCell`, `evaluateAll`, `evaluateFormula`, `formatValue` — is designed to be stable across the transition so grid and toolbar code does not change.

**Rationale for Phase 1 staging:**
- Time to market: IronCalc integration requires Rust engineering depth; the JS engine ships today and validates the product concept immediately
- The JS engine is sovereign (EUPL-1.2 licensed alongside the application, forkable independently)
- The function set covers ~90% of real institutional proforma workloads
- Known limitations are documented in [`docs/engine.md`](./docs/engine.md) so users understand what is and is not supported

**Rationale for IronCalc as Phase 2 target:**
- Apache 2.0 licensed, pure Rust, minimal dependencies
- Funded by NLnet and the European Commission's Next Generation Internet programme — aligned with EU sovereignty posture
- 200+ Excel-compatible functions at 1.0, targeting near-full Excel compatibility over time
- Embeddable from Rust via `cargo add ironcalc`; WebAssembly-capable for future browser deployment
- Alternative Phase 2 option: Formualizer (MIT, Apache Arrow-backed, 320+ functions) — decision deferred pending engineering spike against representative workloads

**Alternative considered and rejected:** shipping the application without a formula engine at all and exporting to Excel for evaluation. Rejected because it would hard-couple users to Excel's continued existence, defeating the sovereignty thesis.

---

### ADR-006: Minimal IPC surface

**Decision:** Expose exactly three IPC commands from Rust to JavaScript: `open_file`, `save_file`, `get_app_data_dir`.

**Rationale:**
- Every IPC command is a potential attack surface
- A spreadsheet editor in Phase 1 needs file open, file save, and app-data directory access. Nothing else.
- No shell access, no arbitrary file system traversal, no network commands
- CSP set to `default-src 'self'; connect-src 'none'` — no outbound connections from the WebView

**Phase 2 additions:** IronCalc integration may add two IPC commands: `evaluate_workbook` (full-workbook evaluation on demand) and `parse_formula` (syntax validation and AST inspection). These will keep the overall surface below six commands.

**Inheritance:** Same discipline as memo ADR-006. Memo exposes four commands (the fourth is `read_font_file`, which proforma does not need for MVP).

---

### ADR-007: SHA-256 audit chain

**Decision:** Every save computes a SHA-256 digest of the canonical document and writes it into `document.audit.sha256`. On first save after opening a file, the previous digest is written to `audit.parent_sha256`.

**Rationale:**
- The audit chain makes tampering mathematically detectable — any change to a saved file invalidates the embedded digest
- The chain forms an append-only record of document history, suitable for fiduciary archiving
- Uses the browser's native `crypto.subtle.digest` API — no external dependency
- Canonical serialisation (2-space JSON, stable key order) ensures the digest is reproducible on any platform

**Phase 2:** When archive integration (F12 commit) is added, the chain extends to reference prior committed versions by parent SHA-256, enabling full end-to-end chain-of-custody verification without requiring a central database.

---

### ADR-008: Self-hosted Forgejo for code hosting

**Decision:** The canonical repository is on a self-hosted Forgejo instance on European infrastructure. GitHub is used as a mirror only.

**Rationale:**
- GitHub is owned by Microsoft (US hyperscaler, subject to CLOUD Act)
- Forgejo is GPL-licensed, self-hostable, EU-governed (Codeberg e.V., Berlin)
- The monorepo currently mirrors to GitHub for convenience — this is the secondary location
- The primary location must be sovereign infrastructure

**Inheritance:** Same decision as memo ADR-008. The PointSav monorepo mirroring policy applies to all `app-workplace-*` applications uniformly.

---

## Dependency Sovereignty Audit

| Dependency | Licence | Controller | Linux sovereign? | macOS sovereign? |
|---|---|---|---|---|
| Rust language | MIT + Apache 2.0 | Rust Foundation (US non-profit) | ✅ | ✅ |
| Tauri v1/v2 | MIT + Apache 2.0 | Commons Conservancy (NL) | ✅ | ✅ |
| wry (WebView crate) | Apache 2.0 | Commons Conservancy (NL) | ✅ | ✅ |
| tao (windowing crate) | Apache 2.0 | Commons Conservancy (NL) | ✅ | ✅ |
| WebKitGTK (Linux WebView) | LGPL 2.1 | GNOME Foundation | ✅ | N/A |
| WKWebView (macOS WebView) | Proprietary | Apple | N/A | ✗ Accept |
| serde | MIT + Apache 2.0 | serde-rs community | ✅ | ✅ |
| serde_json | MIT + Apache 2.0 | serde-rs community | ✅ | ✅ |
| Phase 1 formula engine | EUPL-1.2 | PointSav Digital Systems | ✅ | ✅ |
| Phase 2 IronCalc *(planned)* | Apache 2.0 | NLnet + EC (EU-funded) | ✅ | ✅ |

On Linux, the only proprietary components in the entire running binary are closed-source firmware in the OS kernel (Wi-Fi drivers etc.) — which are present regardless of which application stack is chosen.

The Phase 2 IronCalc integration strengthens this posture further: the formula engine moves from being inside the application binary to being an EU-funded upstream dependency governed by NLnet. This is an explicit coordination point with the European Commission's Next Generation Internet programme.

---

## Security Model

The application's security posture is intentionally minimal-surface:

**CSP (Content Security Policy)**
```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
connect-src 'none';
object-src 'none';
frame-ancestors 'none';
```

`connect-src: 'none'` is the most important rule. The spreadsheet makes **zero outbound network connections** at runtime.

**File system access** is scoped to:
- User-selected file paths via native OS dialogue (open/save)
- The application data directory (`~/.local/share/workplace-proforma/` on Linux)

Path traversal attacks are blocked in the Rust IPC handlers using `canonicalize()` + prefix validation before any file operation.

**JSON validation** happens at two points: the Rust side validates that input and output are parseable JSON before any file operation (refusing to read non-JSON or write corrupted state); the JavaScript side validates the document against the schema shape defined in `schema.js`.

**IPC encryption** (Tauri v2 on Linux): The Isolation Pattern is enabled. All IPC messages are encrypted with SubtleCrypto before crossing the WebView boundary. Keys are regenerated on each application launch.

---

## Phase 2 and beyond

See `CHANGELOG.md` and the platform roadmap. Proforma-specific Phase 2 items:

- **IronCalc integration** via Rust IPC, replacing the JS formula engine with minimal UI impact
- **XLSX export** via `rust_xlsxwriter` (Apache 2.0, pure Rust) for legacy recipient compatibility
- **Schema-compliant backward compatibility tests** ensuring every Phase 1 file opens unchanged in Phase 2+
- **Archive integration** with `os-totebox` (via WorkplaceOS capability system) activating the F12 commit path and `ARCHIVE.*` formula namespace
- **AI sidebar** following the same three-paths pattern as the platform `service-slm` (no AI / user-provided AI / archive-integrated AI)

The first two are standalone-application improvements. The last two activate the platform integration layer documented in the PointSav Project Instructions.
