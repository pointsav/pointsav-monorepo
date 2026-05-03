# Architecture — Workplace\*Memo

Architectural decisions, dependency sovereignty audit, and technical rationale.

---

## Guiding Principle

Every layer of this stack must pass a single test: **can we fork it, rewrite it, and own it completely, with no dependency on a US hyperscaler's goodwill, infrastructure, or legal jurisdiction?**

On Linux — the production sovereign platform — the answer is yes at every layer.

---

## Decision Record

### ADR-001: Tauri over Electron

**Decision:** Use Tauri (Rust + OS WebView) rather than Electron (Chromium bundled).

**Rationale:**
- Tauri bundles are 8–15 MB vs Electron's 150+ MB
- Tauri uses the OS WebView which receives security patches from OS maintainers, not from application release cycles
- Tauri is governed by the Tauri Foundation within the Dutch non-profit Commons Conservancy (EU jurisdiction, MIT + Apache 2.0)
- Electron bundles a specific Chromium version controlled by Google

**Trade-off:** WebView behaviour differs across OS (WKWebView/WebView2/WebKitGTK). CSS `@page` support varies. Mitigated by Linux-first development and a test matrix.

---

### ADR-002: Tauri v1 for macOS 10.13, Tauri v2 for Linux

**Decision:** Use Tauri v1.x for the macOS 10.13 development build. Use Tauri v2.x for the Linux production build.

**Rationale:**
- Tauri v2 requires macOS 10.15+. The current development machine runs 10.13 (High Sierra)
- The application code (HTML/CSS/JS frontend, Rust IPC commands) is identical between v1 and v2
- Only the `src-tauri/Cargo.toml` dependency versions and `tauri.conf.json` schema differ
- The `package.json` `tauri:macos` and `tauri:linux` scripts select the correct version

**Migration path:** When the development machine is upgraded to macOS 10.15+, update `src-tauri/Cargo.toml` to Tauri v2 and remove the macOS-specific workarounds documented in `DEVELOPMENT.md`.

---

### ADR-003: Paged.js for live pagination

**Decision:** Use Paged.js (MIT, Pagedmedia.org) for the document canvas pagination engine.

**Rationale:**
- Paged.js is a JavaScript polyfill for the W3C Paged Media specification
- It renders each page as a discrete `div` in the DOM — giving the page-on-grey-desktop feel without server-side rendering
- It computes live page breaks as the user types, reflecting true print output
- It is the pagination engine used by several European government document tools
- MIT licensed, small codebase, can be forked and maintained internally if upstream becomes unmaintained

**Risk:** Pagedmedia.org is a small organisation. Mitigation: Paged.js is vendored into `src/js/vendor/paged.min.js` at a pinned version. Upgrades are manual and deliberate.

---

### ADR-004: EUPL v1.2 application licence

**Decision:** Licence Workplace*Memo under the European Union Public Licence v1.2.

**Rationale:**
- Designed by the European Commission for European public sector software
- Available in all 23 EU official languages
- Copyleft (like GPL) — prevents a hyperscaler from taking the code and distributing it as a proprietary product
- Explicitly compatible with GPL v2/v3 and AGPL v3
- Jurisdiction clause: disputes governed by the law of the EU member state where the licensor is established
- Used by openDesk (Germany) and several La Suite Numérique (France) components

---

### ADR-005: Font embedding via base64 at export time

**Decision:** At export time, all fonts referenced in the document are base64-encoded and embedded as `@font-face` data URIs in the exported HTML file.

**Rationale:**
- CDN font links (Google Fonts etc.) are a hyperscaler dependency and fail in offline/print contexts
- Base64 embedding makes the document fully self-contained — readable and printable anywhere, forever
- Fonts used in a document are guaranteed to appear correctly in the PDF regardless of what fonts are installed on the receiving machine
- Each font family at three weights adds ~200–400 KB to the file — acceptable for permanent archived documents

**Implementation:** `scripts/embed-fonts.sh` pre-processes all `.woff2` files in `fonts/` into `src/js/font-data.js` at build time. This file is `.gitignore`d. Font binary files (`.woff2`) are tracked in git under `fonts/`.

---

### ADR-006: Minimal IPC surface

**Decision:** Expose exactly four IPC commands from Rust to JavaScript: `open_file`, `save_file`, `get_app_data_dir`, `read_font_file`.

**Rationale:**
- Every IPC command is a potential attack surface
- A document editor needs file open, file save, and font data access. Nothing else.
- No shell access, no arbitrary file system traversal, no network commands
- CSP set to `default-src 'self'; connect-src 'none'` — no outbound connections from the WebView

---

### ADR-007: HTML as the native document format

**Decision:** The native save format is a self-contained `.html` file.

**Rationale:**
- HTML is an open standard readable by any browser in 2026 or 2046
- No proprietary application required to read the document
- Fonts are embedded — no external dependencies
- Track changes (`<ins>`/`<del>`) are semantic HTML — auditable by anyone
- Diff-friendly — standard `git diff` shows meaningful document changes
- `.docx` is opaque XML requiring Microsoft's schema interpretation

**Compatibility:** `.docx` import via `mammoth.js` (MIT) is planned for Phase 3. This converts `.docx` → clean HTML which the editor renders natively.

---

### ADR-008: Self-hosted Forgejo for code hosting

**Decision:** The canonical repository is on a self-hosted Forgejo instance on European infrastructure. GitHub is used as a mirror only.

**Rationale:**
- GitHub is owned by Microsoft (US hyperscaler, subject to CLOUD Act)
- Forgejo is GPL-licensed, self-hostable, EU-governed (Codeberg e.V., Berlin)
- The monorepo currently mirrors to GitHub for convenience — this is the secondary location
- The primary location must be sovereign infrastructure

**Note for Phase 1:** Until a self-hosted Forgejo instance is provisioned, GitHub serves as the working repository. The migration path is documented in `docs/FORGEJO_MIGRATION.md`.

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
| Paged.js | MIT | Pagedmedia.org | ✅ | ✅ |
| EB Garamond | SIL OFL | Georg Mayr-Duffner | ✅ | ✅ |
| Source Serif 4 | SIL OFL | Frank Grießhammer / Adobe | ✅ | ✅ |
| Lora | SIL OFL | Cyreal | ✅ | ✅ |
| Playfair Display | SIL OFL | Claus Eggers Sørensen | ✅ | ✅ |
| Fraunces | SIL OFL | Undercase Type | ✅ | ✅ |
| DM Sans | SIL OFL | Colophon Foundry | ✅ | ✅ |
| IBM Plex Sans | SIL OFL | IBM / Bold Monday | ✅ | ✅ |
| Source Code Pro | SIL OFL | Paul D. Hunt / Adobe | ✅ | ✅ |

On Linux, the only proprietary components in the entire running binary are closed-source firmware in the OS kernel (Wi-Fi drivers etc.) — which are present regardless of which application stack is chosen.

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

`connect-src: 'none'` is the most important rule. The document editor makes **zero outbound network connections** at runtime.

**File system access** is scoped to:
- User-selected file paths via native OS dialogue (open/save)
- The application data directory (`~/.local/share/workplace-memo/` on Linux)

Path traversal attacks are blocked in the Rust IPC handlers using `canonicalize()` + prefix validation before any file operation.

**IPC encryption** (Tauri v2 on Linux): The Isolation Pattern is enabled. All IPC messages are encrypted with SubtleCrypto before crossing the WebView boundary. Keys are regenerated on each application launch.
