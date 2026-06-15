---
artifact: brief
status: archived
archived: 2026-05-27
synthesised_into:
  - BRIEF-workplace-software-suite.md
  - BRIEF-workplace-desktop-environment.md
topic: app-workplace-architecture-decision
created: 2026-05-23
agents: 8 research + 1 synthesis
model: opus
recommended-option: j
consensus: strong
veto-applied: b (browser leg cannot prove connect-src 'none'), c (browser-only — same), e (unified + browser leg), f (unified browser-only), h (PWA service worker incompatible with connect-src 'none'), i (browser leg breaks sovereignty proof)
next-action: move apps to project-development per priority order; ship launcher + Memo first; connectivity topology ratified in §9
---

# BRIEF — `app-workplace-*` Delivery Architecture Decision

## 1. Executive Summary

**Recommendation: Option (j) — launcher shell + seven separate desktop Tauri v2 processes (Steam / JetBrains Toolbox model).** Consensus across 8 independent Opus research agents is **strong**: 7 of 8 agents nominate (j) as their first recommendation; the 8th (Software Architect) names (j) as primary while logging an explicit dissent that option (a) is defensible. The single most decisive finding is that **every option carrying a browser-delivered leg is vetoed by `connect-src 'none'` sovereignty enforcement** — the browser leg's DNS resolution, TLS handshake, OCSP/CT logging, and vendor telemetry sit outside any page CSP and cannot be proved offline. Once browser delivery is removed from the option set, the choice collapses to (a) vs (g) vs (j); (j) wins on every dimension except sheer engineering simplicity, and the marginal cost of the launcher (~25–30 engineer-days over option a) is repaid by suite-identity, seL4-aligned process isolation, and per-app crash containment that no unified-process design can deliver.

## 2. Veto Decisions

| Option | Veto reason | Agent(s) who flagged it |
|---|---|---|
| **b** — separate apps, desktop + browser | Browser leg cannot maintain `connect-src 'none'`; delivery channel (DNS + TLS + OCSP + vendor telemetry) sits outside page CSP; dual-CSP fork doubles maintenance ~2.8×; GDPR Art. 25 "by default" not provable | Security/Sovereignty (8), Software Architect (1), Systems Designer (5), Economist (6), Product Manager (4) |
| **c** — separate apps, browser-only | Same CSP/delivery-channel veto; abandons sovereign-desktop positioning; no native filesystem; structurally incoherent with `connect-src 'none'` | Security/Sovereignty (8), Software Architect (1), UX Researcher (3), Systems Designer (5) |
| **e** — unified workbench, desktop + browser | Inherits the browser-leg veto from (b); compounded by 7-of-7 blast radius if any leg is exploited; worst-of-both-worlds maintenance burden | Security/Sovereignty (8), Software Architect (1), Economist (6), Systems Designer (5) |
| **f** — unified workbench, browser-only | Same CSP veto plus single-point-of-failure across all 7 apps; full WebKitGTK CVE attack surface with no Tauri IPC allowlist | Security/Sovereignty (8), UI/UX Engineer (2), Software Architect (1), Systems Designer (5) |
| **h** — PWA, separate apps | Service-worker install/update channel requires `connect-src` relaxation by definition; PWA installability is patchy on AArch64 Linux outside Chromium; browser-vendor telemetry operates outside page CSP | Security/Sovereignty (8), Software Architect (1), UX Researcher (3), Systems Designer (5) |
| **i** — hybrid (heavy desktop / light browser) | The browser leg for Memo/PDF carries the same CSP/delivery-channel exposure as (b); two-stack-forever cost without a clean engineering boundary (pdfium-render is just as native-friendly as web-ifc is webview-bound) | Security/Sovereignty (8), UI/UX Engineer (2), UX Researcher (3), Software Architect (1) |
| **d** — duplicate of (a) | Not a veto — but the option label adds no information. Retire to avoid future ambiguity. | Software Architect (1) explicit; all other agents implicit |

Three options survive the veto cleanly: **a**, **g**, **j**.

## 3. Per-Option Scorecard

Scores aggregated from research-agent outputs. Where multiple agents scored the same dimension, the cell shows the mean with the range. Weights: Security/Sovereignty 25 %, Systems Integration 20 %, SMB User Fit 20 % (Agents 3+4+7 averaged), Technical Debt 15 % (Agent 1 risk inverted as 10−risk), Economic 10 %, UI/UX 10 %.

| Option | Security/Sov. (25%) | Sys Integration (20%) | SMB User Fit (20%) | Tech Debt (15%) | Economic (10%) | UI/UX (10%) | Weighted Total |
|---|---|---|---|---|---|---|---|
| **a** — separate apps, desktop-only | 9.00 | 8.75 | 8.25 (range 7–9) | 7.00 | 8.00 | 7.50 | **8.25** |
| **b** — separate apps, desktop + browser | VETOED — browser leg breaks `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **c** — separate apps, browser-only | VETOED — browser-only fails `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **d** — identical to (a) | — | — | — | — | — | — | **= 8.25 (no distinction)** |
| **e** — unified workbench, desktop + browser | VETOED — browser leg breaks `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **f** — unified workbench, browser-only | VETOED — browser-only fails `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **g** — unified workbench, desktop-only | 6.00 | 4.00 | 5.83 (range 4.5–7) | 4.00 | 4.00 | 6.25 | **5.09** |
| **h** — PWA, separate apps | VETOED — service worker incompatible with `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **i** — hybrid (Memo/PDF browser) | VETOED — browser leg breaks `connect-src 'none'` | VETOED | VETOED | VETOED | VETOED | VETOED | **VETOED** |
| **j** — launcher + 7 separate processes | 9.50 | 8.75 | 8.25 (range 8–8.75) | 6.00 | 9.00 | 8.50 | **8.43** |
| **k1** (Systems Designer addition) — per-app binaries + thin shared `.desktop` manifest (no parent supervisor) | 9.00* | 8.75 | 8.00 (est.) | 7.00 | 8.50 | 7.50 | **8.18 (est.)** |

*k1 sovereignty score estimated from a-tier; identical isolation properties to (a).

**Arithmetic shown for the three surviving options:**

- **Option a:** 0.25 × 9.00 + 0.20 × 8.75 + 0.20 × 8.25 + 0.15 × 7.00 + 0.10 × 8.00 + 0.10 × 7.50
  = 2.250 + 1.750 + 1.650 + 1.050 + 0.800 + 0.750 = **8.250**
- **Option g:** 0.25 × 6.00 + 0.20 × 4.00 + 0.20 × 5.83 + 0.15 × 4.00 + 0.10 × 4.00 + 0.10 × 6.25
  = 1.500 + 0.800 + 1.167 + 0.600 + 0.400 + 0.625 = **5.092**
- **Option j:** 0.25 × 9.50 + 0.20 × 8.75 + 0.20 × 8.25 + 0.15 × 6.00 + 0.10 × 9.00 + 0.10 × 8.50
  = 2.375 + 1.750 + 1.650 + 0.900 + 0.900 + 0.850 = **8.425**

Option j wins by 0.175 over option a; option g trails by 3.33. The j-vs-a margin is narrow on the headline number but widens materially on the Security/Sovereignty (+0.50), Economic (+1.00), and UI/UX (+1.00) sub-dimensions where the launcher specifically adds value.

## 4. Leapfrog 2030 Fit Analysis

Doctrine claim text retrieved from `/srv/foundry/DOCTRINE.md`:

- **Claim #1** — "True air-gap operation. Connectivity-independent."
- **Claim #2** — "100-year readability. Format-stable in plain text."
- **Claim #34** — "The Two-Bottoms Sovereign Substrate. The substrate has two bottoms: native (seL4 today, moonshot-kernel tomorrow) on AArch64-first hardware where formal verification is meaningful, and compatibility (NetBSD with Veriexec verified-image boot + `build.sh` offline reproducibility + rump kernels for IT/OT bridge) for boot-anywhere on commodity hardware where seL4 cannot reach bare-metal."
- **Claim #43** — "Single-Boundary Compute Discipline. The Doorman (`service-slm`) is the single boundary point for all AI inference compute in every Foundry deployment. No process, session, or service accesses an inference tier (local, Yo-Yo, or external API) except through the Doorman."

### Option j (launcher + 7 desktop processes)

- **Claim #1 — true air-gap.** Each child Tauri binary ships with `connect-src 'none'` enforced at the binary level (Tauri v2 CSP, no relaxation). The launcher itself holds the station's MBA WireGuard socket; children can either inherit the FD via `SCM_RIGHTS` (per Systems Designer §"Service Integration") or exclusively talk to the launcher via Unix domain socket, never directly to the network. A packet capture of any child process during normal operation shows zero outbound traffic — the air-gap claim is *provable*, not aspirational.
- **Claim #2 — 100-year readability.** Independent per-app processes do not couple file formats. Each app owns its `data_dir` and produces plain-text/standard-format outputs (IFC-SPF for BIM per `.agent/rules/bim-product-family.md`, .md for Memo, .xlsx/.ods or similar for Proforma, .pdf for PDF, .key/.pptx/native for Presentation). No suite-wide binary blob exists that could become unreadable if the launcher dies; the launcher is purely a discovery/lifecycle shell.
- **Claim #34 — two-bottoms substrate.** Tauri v2 binaries compile cleanly for AArch64 on either native (seL4 + moonshot-kernel + thin shim) or compatibility (NetBSD + Veriexec) bottoms. The launcher's Unix-domain-socket IPC is portable across both substrates; the seL4 capability model maps directly onto per-process isolation (one CNode set per child), which is the textbook least-authority composition the kernel was designed for. Option g voluntarily collapses this to one shared CNode set — wasting the kernel's primary value.
- **Claim #43 — single-boundary compute.** When any of the 7 apps needs AI inference (e.g. Memo grammar check, Proforma formula suggestion, BIM IFC-text annotation), the launcher proxies the request to `service-slm` Doorman over the station's WireGuard tunnel — a single boundary point per station. Per-app TLS client certs let the gateway audit log record `station + app + user` granularity. Option g would force the unified binary itself to become the Doorman client, blurring the per-app audit identity to a single station-wide identifier and weakening claim #43's audit guarantee.

### Option a (separate apps, no launcher)

- **Claim #1.** Same per-app `connect-src 'none'` posture as (j). Equally provable.
- **Claim #2.** Identical to (j) — 7 independent file-format domains, no suite-wide coupling.
- **Claim #34.** Identical to (j) on isolation; the difference is whether `os-workplace` provides the launcher cohesion layer at OS level. If `os-workplace` ships a competent native app-grid (the Software Architect's dissent), (a) and (j) are equivalent on substrate fit.
- **Claim #43.** Slightly weaker than (j) in the absence of a launcher to centralise MBA socket ownership — sub-variant (a2) in Systems Designer §"Service Integration" must add a station-local MBA-proxy daemon to play the role the launcher would otherwise play. Net cost is comparable; the architecture is just split between `os-workplace` and a sidecar daemon instead of unified in the launcher.

### Option g (unified workbench, desktop-only)

- **Claim #1.** Single Tauri binary, single WebView, `connect-src 'none'` enforced — survives the network claim.
- **Claim #2.** A unified binary tends to grow shared serialisation surfaces (cross-app clipboard, project hub, undo stack) that become hard to keep plain-text-stable over decades.
- **Claim #34 — major fit failure.** seL4's primary value is *formally-verified inter-process capability isolation*. Option g collapses 7 capability domains (BIM IP, proforma financial records, GIS tiles, memo docs, PDFs, presentations, glossaries) into one CNode set. A WebKitGTK 0-day inside any one app context reaches all 7 data classes. The Security/Sovereignty agent's verdict is the strongest: "Option g voluntarily collapses the isolation seL4 makes available." For a sovereign-OS strategy that names AArch64 + seL4 as its substrate, this is leaving the marquee property on the table.
- **Claim #43.** Audit identity at the gateway collapses to "station-workplace-jennifer-1 connected" — opaque about which of the 7 contexts is acting. Recovering per-app audit requires the unified binary to inject and self-sign an `X-Workplace-App` header that the gateway must trust — a procedural rather than cryptographic audit guarantee, contrary to claim #43's intent.

**Verdict:** Options (a) and (j) both fit the leapfrog architecture cleanly; (g) violates the spirit of claims #34 and #43 even though it technically satisfies #1. Between (a) and (j), the launcher in (j) adds a tangible cohesion + audit-identity surface that improves leapfrog fit at modest cost.

## 5. Agent Consensus Map

| Agent role | Recommended option | Confidence | Key reasoning (one sentence) |
|---|---|---|---|
| 01 — Software Architect | j (primary), a (dissent) | High | WebView is non-negotiable for BIM/GIS via ThatOpen, Tauri's single-Core blast radius mandates cross-app OS-process isolation, and launcher cost (~25–30 engineer-days) buys suite cohesion that would otherwise be built into each of 7 apps separately. |
| 02 — UI/UX Engineer | j | High | JetBrains Toolbox + Steam parallel: suite identity at launcher level, isolated keymaps at app level; 7 disjoint shortcut namespaces (Excel/Word/PowerPoint/Revit/QGIS/Acrobat/IDE) cannot coexist in one window without breaking Revit + Excel power-user muscle memory. |
| 03 — UX Researcher | a/j (treated as UX-equivalent) | High | Affinity v3 unification backlash + Autodesk open-letter pattern + LibreOffice sovereignty-community prior all converge on separate-windows; launcher is invisible after first run and does not change any per-app interaction. |
| 04 — Product Manager | j | High | Only shape that simultaneously preserves PLG land-and-expand, fits Affinity-vacated SMB perpetual-license slot, accommodates 2–4-engineer velocity ceiling via shared substrate + per-app SKUs, and matches Steam-normalised "install the apps you need" cultural muscle memory. |
| 05 — Systems Designer | j (primary), a/k1 (near-ties) | High | 7 OS processes = 7 seL4 CNode sets = textbook least-authority composition; launcher passes MBA WireGuard FD via `SCM_RIGHTS` enabling per-app TLS identity to gateway; Steam, VS Code, and GNOME Shell all chose process-per-app over single-process at scale. |
| 06 — Economist | j | High | Captures ~95 % of option a's incremental-shipping advantage, recovers ~3.6 em/yr of plumbing maintenance via shared launcher infrastructure, preserves regression isolation, and aligns with the operator's already-ratified $19 FSL per-app SKU pricing decision (2026-05-22). |
| 07 — SMB Survey | a/j (top tier) | High | LibreOffice StartCenter is literally option (j) — separate `.desktop` launchers reachable from a shell — and it underwrites 200 M users; ONLYOFFICE unified shell scales to 5 editorial apps, never to 7 apps spanning BIM/GIS/IDE. |
| 08 — Security/Sovereignty | j | Very high | Maximum seL4 capability isolation, minimum WebKitGTK 0-day blast radius (1/7 vs 7/7), smallest supply-chain blast radius per CVE (one binary's deps, not seven's), provable GDPR Art. 25 "by default" posture, aligns with ISO 19650-5 leakage-prevention for BIM IP. |

**Consensus level:** strong. 7/8 agents name (j) as primary. The 8th (UX Researcher) treats (a) and (j) as UX-equivalent and recommends "ship whichever the team can deliver faster," explicitly endorsing (j) as well.

## 6. Recommended Architecture

### Delivery model

**`app-workplace-launcher` (suite identity shell) + 8 desktop apps — including `app-workplace-workbench` (coding IDE) — all Tauri v2 AArch64 native, all `connect-src 'none'`, all EUPL 1.2.** No browser variant. No PWA variant. No unified-binary variant.

### Per-app table

| App | Priority | Rationale | Shared infrastructure it needs |
|---|---|---|---|
| `app-workplace-launcher` | **1 — ships first** | Suite-identity layer, app discovery grid, first-run "1 app installed, 7 available" surface, app lifecycle management (spawn/reap child processes), cross-app file picker, recent-documents memory, crash-recovery banners, license activation. Thin binary (~5 MB Rust GUI, egui or GTK4). Carries the launcher-shell role previously documented under `app-workplace-workbench`. | MBA pairing module (one outbound WireGuard tunnel to gateway-orchestration-command-1 per station); shared theming substrate from `pointsav-design-system`; license-verification helper; Unix-domain-socket IPC schema (length-prefixed CBOR or MessagePack). |
| `app-workplace-workbench` (coding IDE) | **Priority TBD — parallel track** | Native desktop coding environment running at localhost — analogous to VS Code but as a Tauri v2 binary. File tree, code editor (CodeMirror 6), integrated terminal, git integration, language server protocol support. Distinct from `app-privategit-workbench` (the browser-based three-column developer tool for Totebox Orchestration community users — different product, different persona). User-tier application; lifecycle independent of the launcher; can crash and restart without affecting the suite. Estimated binary ~150 MB (Tauri v2 + WebView + LSP subprocess). | Tauri v2 + WebView; CodeMirror 6 vendored bundle (reusable from `app-mediakit-knowledge`); LSP subprocess support; shared theming from `pointsav-design-system`. Registry action required: create crate directory + row. |
| `app-workplace-memo` | **2 — first feature app** | Light app, fastest to ship after launcher is wired. Word/Pages muscle memory. Validates the launcher↔child IPC end-to-end with the lowest engineering risk. Currently Scaffold-coded (47 files) per project-registry.md. | Shared text engine (markdown render + sanitisation), CodeMirror 6 vendored bundle reusable from `app-mediakit-knowledge`, shared theme tokens. |
| `app-workplace-pdf` (new — not yet in registry) | **3** | PDF reader + annotator. Native `pdfium-render` (MIT) parses PDFs in the Rust core; WebView surfaces the rendered pages. Adobe Acrobat / Okular muscle memory. Stirling-PDF precedent confirms SMB acceptance of standalone PDF tooling. | `pdfium-render` native crate, shared `service-fs` access for recent docs. **Registry action required:** create row + scaffold. |
| `app-workplace-proforma` | **4** | Active (45 files, Phase pending). Excel muscle memory: F2/F4/F9/Ctrl+;/Ctrl+T plus formula-bar conventions. Heavier than Memo; spreadsheet engine is the load-bearing substrate. Currently CLAUDE.md marked "local-only" — conformance pending per registry. | Native table/cell engine (Rust); formula evaluator; CSV/XLSX/ODS importers; chart rendering via SVG. Must clear "local-only" CLAUDE.md status before Active state holds. |
| `app-workplace-presentation` | **5** | Active (52 files; Phase 5 per registry). PowerPoint/Keynote muscle memory: F5 present, Ctrl+M new slide, Tab demote. Layout templating from `pointsav-design-system`. | Slide-layout engine, shared theming, export to PDF/PPTX, presenter-display surface. |
| `app-workplace-gis` (new — separate from `gateway-orchestration-gis-1`) | **6** | Native QGIS muscle memory for power users; `maplibre-rs` is a credible native renderer, but for v0 ship the WebView with MapLibre GL JS + OpenFreeMap PMTiles per Economist §"GIS Ecosystem Licensing." Hosts editing-side workflows that the public `gis.woodfinegroup.com` Mediakit surface does not. **Registry action required:** create row under `app-workplace-*`. | OpenFreeMap PMTiles (~$0/mo); native projection libs (GeoRust); shared MBA path to `cluster-totebox-personnel-1` for layer pulls. |
| `app-workplace-bim` | **7 — ships last** | Reserved-folder per registry; research phase only. Heaviest app and the load-bearing reason WebView stays in the stack at all (`@thatopen/components` is JS-only, no native Rust port exists for BIM widget catalogue). Ship after Creoox xeokit-license quote is in hand (Economist §"BIM/GIS Ecosystem Licensing"); v0 can ship with MPL-only stack (web-ifc + @thatopen/components, no xeokit) at reduced rendering features to preserve optionality. Must follow `.agent/rules/bim-product-family.md` data contract (IFC-SPF canonical, BCF 3.0 issues, IDS 1.0 validation, IfcOpenShell via subprocess for LGPL compliance). | web-ifc WASM (MPL-2.0), @thatopen/components (MIT), IfcOpenShell subprocess (LGPL — clean via dynamic invocation per BIM family rules), MBA path to `cluster-totebox-property-1` for canonical IFC pull. |

### IPC model between `app-workplace-launcher` and child apps

Per Software Architect §"Launcher Shell Model Feasibility" and Systems Designer §"Launcher Shell Implementation Analysis":

- **Transport:** Unix domain socket at `$XDG_RUNTIME_DIR/workplace-launcher.sock`. Crates: `interprocess` and/or `tokio-unix-ipc` (both support `SCM_RIGHTS` FD-passing across Unix-domain sockets, which is the critical primitive for MBA socket sharing).
- **Wire format:** length-prefixed CBOR or MessagePack. Versioned; the protocol contract is the load-bearing artefact — design it correctly once or eat versioning pain forever.
- **Lifecycle:** launcher spawns each child via `tokio::process::Command`, retains the `Child` handle, awaits `wait()` to reap on exit per Tokio process-management discipline (zombie/orphan protection per Tauri sidecar warnings).
- **Minimum-viable message set:**
  - `Hello { app_id, pid, version }` — child → launcher on startup
  - `OpenDocument { path }` — launcher → child (file-open routing from shared picker)
  - `RecentDocument { path, app_id }` — child → launcher (local-only telemetry)
  - `Quit` — launcher → child (graceful shutdown)
  - `Heartbeat` — both directions, for crash detection
  - `MbaSocketFd { fd }` — launcher → child at spawn time via `SCM_RIGHTS`, granting the child direct access to the WireGuard tunnel for per-app TLS to the gateway (preserves per-app audit identity)
- **Crash recovery:** if a child dies, the launcher surfaces a banner ("app-workplace-bim exited unexpectedly — restart?"). The other 6 apps are unaffected. This is the structural win over options (e) and (g).
- **Supervisor:** ~200 lines of bespoke Rust on `tokio::process` rather than adopting `super-visor` / `task-supervisor` (neither mature enough per Systems Designer §"Launcher Shell Implementation Analysis").

### Migration path from current Tauri v1 scaffold state

Per Software Architect §"Tauri v1 → v2 Migration Analysis" and per project-registry.md current states:

1. **Sprint 0 — Tauri v1 → v2 migration of all currently-active workplace apps** (`app-workplace-memo`, `app-workplace-proforma`, `app-workplace-presentation`). Use the `tauri migrate` CLI for the config (top-level `tauri` key → `app`; `allowlist` block → `src-tauri/capabilities/`). Reconcile the no-bundler ↔ plugin-script split manually. Budget: 10–20 engineer-days total. CSP reconciliation: keep `connect-src 'none'` everywhere except internal `ipc:` pseudo-origin which Tauri v2 requires (this is local-only, never crosses a network interface — not a relaxation of the doctrine).
2. **Sprint 1 — Launcher binary.** Build `app-workplace-launcher` as a fresh crate at `vendor/pointsav-monorepo/app-workplace-launcher/`. Minimal Rust GUI (egui chosen for AArch64 maturity and ~5 MB binary size; GTK4 is the fallback if egui's table widgets are insufficient for app-grid UX). Define the Unix-domain-socket IPC contract end-to-end with versioning. Budget: 10–15 engineer-days. **`app-workplace-workbench` (coding IDE)** is provisioned as a separate crate (`vendor/pointsav-monorepo/app-workplace-workbench/`, Tauri v2 + WebView, ~150 MB) on a parallel track independent of this sprint; its first ship does not block Sprint 2 Memo validation.
3. **Sprint 2 — Memo first ship.** Wire `app-workplace-memo` to handshake with launcher (`Hello` + `MbaSocketFd` + `OpenDocument` + `Heartbeat`). Ship as the first end-to-end demonstration of the suite. Budget: 5 engineer-days plus app-feature work.
4. **Sprint 3 — PDF + Proforma.** Add registry row for `app-workplace-pdf`; scaffold; ship. Promote `app-workplace-proforma` to Active by resolving its "local-only" CLAUDE.md status (registry note).
5. **Sprint 4 — Presentation + GIS.** Promote `app-workplace-presentation` from Phase 5 to v1.0 ship. Add registry row for `app-workplace-gis`; scaffold + ship.
6. **Sprint 5 — BIM, gated on xeokit quote.** Promote `app-workplace-bim` from Reserved-folder to Scaffold-coded → Active. Must follow `.agent/rules/bim-product-family.md` end-to-end. If the Creoox xeokit quote is unfavorable, ship the MPL-only stack (web-ifc + @thatopen/components) with reduced rendering features.

Total: ~25–30 wall-clock weeks for a pair of engineers (per Economist §"Delivery timeline estimates"), with the first user-visible ship at month 3–4 (launcher + Memo).

### What `os-workplace` must provide at the OS level

Per the `BRIEF-sovereign-os-family-master-plan.md` §D "Workplace OS" architecture and the Systems Designer §"AArch64 Linux Desktop Systems Status":

- **AArch64 native bootable image** with NetBSD compatibility personality (Doctrine claim #34 two-bottoms substrate); GUI stack is Wayland-clean GTK + WebKitGTK 4.1 to match Tauri v2's Linux WebView requirement.
- **A station-scoped WireGuard PPN identity at `10.42.20.x`** per the master plan §B address plan, allowing the launcher's MBA socket to terminate inside the station.
- **`pairings.yaml` entry per station** so that each child app's TLS connection to `gateway-orchestration-command-1` is audit-rooted (the launcher does not need to be in `pairings.yaml` itself — the children's per-app certs are the audit identities the gateway logs).
- **Wayland compositor + freedesktop `.desktop` integration** so the launcher and each of the 7 child apps appear in the OS app grid AND can be launched independently (the launcher does not replace `os-workplace`'s shell — it complements it). This is the Software Architect's dissent honoured: the OS provides the grid, the launcher provides the suite-cohesion layer (recent projects, shared file picker, app switcher, license activation).
- **Per-app data-directory isolation** at the filesystem level mapped to per-app seL4 CNode capability sets when running on the native bottom (claim #34): `~/.local/share/workplace/bim/`, `~/.local/share/workplace/proforma/`, etc., each granted only to the app that owns it.
- **`service-slm` Doorman endpoint** reachable via the station's WireGuard tunnel — the single boundary point for AI inference (claim #43). The launcher does not embed the Doorman; it forwards inference requests from children to the Doorman over the MBA socket.
- **Optional but recommended:** a system-level `xdg-open`-style MIME-handler registration so double-clicking a `.ifc` file routes to `app-workplace-bim` automatically without going through the launcher's app grid. This is the "Steam users double-click `quarterly-numbers.xlsx`" dissent honoured.

## 7. Dissenting Views

**Software Architect (Agent 1) — dissent toward (a) over (j).** The strongest argument against (j) is YAGNI applied to the launcher: if `os-workplace` itself provides a competent app launcher (which any sovereign desktop OS will need to anyway), then building `app-workplace-workbench` as a separate Rust binary is duplicative effort. Under that reading, (a) is correct: ship 7 independent Tauri v2 apps with strong per-app crash isolation, and let the OS provide the cohesion layer. The 25–30 days saved go into making `os-workplace` itself better. The risk is that `os-workplace` is currently Scaffold-coded (4 files, pre-engineering); if that programme slips, the suite ships without a launcher and feels discontinuous. A defensible compromise — and the synthesis position taken in §6 above — is to ship (j) *now* and treat the launcher (`app-workplace-launcher`) as a deliverable that can be retired into `os-workplace` later, once the OS-level shell is mature. (Note: this retirement path applies to the launcher, not to `app-workplace-workbench` — the coding IDE is a user-tier application and is not retirable into the OS.)

**UX Researcher (Agent 3) — partial dissent on (j) vs (a) UX-equivalence.** Affinity v3 is a confounded natural experiment (consolidation + Canva acquisition + forced sign-in + AI paywall hit users simultaneously); attributing the backlash specifically to unification is not clean. In a hypothetical clean experiment, the verdict on unified workbench might be different. The UX Researcher also notes that option (j) "launcher shell" is barely distinguishable from (a) at runtime, but adds engineering complexity (the shell, the inter-app contract, the icon-cohesion discipline) without paying for itself in UX. This dissent does not change the synthesis recommendation because the launcher's value (per Economist + Product Manager + Systems Designer) is non-UX: it pays for itself in shared MBA-socket ownership, per-app audit identity, suite-pricing surface, and shared plumbing maintenance — none of which are visible to UX.

**Economist (Agent 6) — dissent toward (g) on suite-identity premium.** A unified workbench is the only path that produces a coherent "sovereign productivity suite" brand identity. Option (j) ships 7 separate apps that happen to share a launcher — that is not a brand, it is a portfolio. Adobe Creative Cloud, Microsoft 365, and Affinity Suite (pre-v3) extract higher per-customer LTV than sum-of-parts because customers buy into the suite as identity. The 8–10 engineer-month delay before first ship is paying for an irreplaceable brand asset. **Synthesis response:** the operator's already-ratified $19 FSL per-app SKU pricing decision (2026-05-22, per MEMORY.md `project_software_distribution_substrate.md`) is structurally incompatible with the suite-identity thesis; the launcher (j) is the better fit for ratified pricing.

**Systems Designer (Agent 5) — dissent toward (g) on first-party threat model.** The seven workplace apps are first-party PointSav code, all written by the same small team, all signed by the same release process, all delivered as one `os-workplace` image. There is no reason to apply browser-style site-isolation between co-developed first-party apps with the same maintainer trust level. A unified workbench is dramatically simpler operationally: one update channel, one capability file to audit, one settings store, cross-app clipboard and drag-drop come for free. **Synthesis response:** the threat model *does* include hostile input via web-ifc parsing of contractor-supplied IFC files (the BIM-vs-proforma scenario), and "same maintainer trust level" collapses under supply-chain attack against any of web-ifc, @thatopen/components, leaflet, pdf.js, or transitive npm. Process-per-app makes that compromise containable.

**Security/Sovereignty (Agent 8) — operational dissent on update cadence.** Seven separate binaries means seven separate update channels, seven separate signature verification flows, seven separate binary-ledger entries. A regulated SMB IT operator managing seven update flows is statistically more likely to skip an update, mis-configure one, or run a stale version than an operator managing a single workbench binary. **Synthesis response:** the launcher in (j) is the load-bearing operational mitigation for this concern — a well-designed launcher mediates updates centrally ("7 apps installed, 2 updates available") while preserving per-app process isolation at runtime, exactly as F-Droid, SteamOS, and GNOME Software have validated at scale.

**Systems Designer (Agent 5) — alternative recommendation k1 (per-app binaries + thin `.desktop` manifest, no parent process).** Scored 8.75 in Systems Integration (tied with j) and offers the simplest possible composition with seL4 — zero supervisor process, zero shared address space. Trade-off vs (j): no cross-app coordination layer, "open in proforma from BIM" must go through the shell's MIME-handler or the gateway, not a launcher RPC. More Unix-philosophy, less Steam-philosophy. **Synthesis response:** k1 is a credible alternative that should be revisited if the launcher engineering proves heavier than budgeted; on the current weighted scorecard it loses to (j) by 0.25 because the launcher's audit-identity / file-picker / recent-docs surface is worth the marginal engineering cost.

**Moonshot integration note — launcher absorption into os-workplace (operator directive 2026-05-24).** Shipping `app-workplace-launcher` as a standalone Rust binary is the correct Day 1 answer. Its long-term architectural home is as a **native system service inside os-workplace** — the launcher becomes part of the OS, not an application running on top of it. This transition is tracked as a `moonshot-*` milestone (the same register as `moonshot-kernel`, `moonshot-hypervisor`, etc.). Concretely: the IPC contract (`workplace-launcher.sock`, the `Hello`/`OpenDocument` message set, the Unix-domain-socket transport) must be designed as a stable OS-level ABI from the start — because once the launcher is absorbed into `os-workplace`, every `app-workplace-*` binary will talk to the OS service using the same contract. The Software Architect dissent (§7 ¶1) is correct as a long-range observation. **`app-workplace-workbench` (coding IDE) is a user-tier application and is not a candidate for OS absorption** — IDEs are not system services; this note does not apply to it.

## 8. Next Steps

Priority ordering for moving the 7 apps (plus the new launcher) into `project-development`. All registry state transitions must follow the project-registry.md template at `~/Foundry/templates/project-CLAUDE.md.tmpl` + `project-NEXT.md.tmpl`. Directory + registry row land in the same commit per `~/Foundry/CLAUDE.md` §9.

### Priority 1 — `app-workplace-launcher` — NEW

- **Current registry state:** does not exist; not in project-registry.md.
- **Must happen before Active:** (1) create directory `vendor/pointsav-monorepo/app-workplace-launcher/` with `Cargo.toml`, `README.md` + `README.es.md`, `CLAUDE.md`, `NEXT.md`; (2) add registry row under `app-workplace-*`; (3) choose Rust GUI substrate (egui v0.30+ recommended; GTK4-rs fallback); (4) author IPC contract spec at `app-workplace-launcher/docs/IPC-CONTRACT.md` (versioned); (5) wire child process spawn/reap; (6) ship first-run UX (1 app installed, 7 available).
- **Dependencies:** Tauri v2 migration sprint must complete on Memo/Proforma/Presentation before the launcher's `Hello` handshake can be validated against a real child; PPN WireGuard Part A must be live per master-plan §J for the MBA socket-passing to make sense.
- **Transitions to Active when:** launcher + 1 child (Memo) handshake end-to-end, child opens a document via launcher's file picker. Estimate: 10–15 engineer-days post-migration.

**Note on `app-workplace-workbench` (coding IDE) — NEW, parallel track.** Workbench is a user-tier coding environment (VS Code analogue, Tauri v2 + WebView, ~150 MB) provisioned on a parallel track independent of the launcher delivery sequence. It is not a dependency for Memo validation. Priority and scheduling to be assigned in project-development. Must happen before Active: (1) create directory `vendor/pointsav-monorepo/app-workplace-workbench/` with `Cargo.toml`, `README.md` + `README.es.md`, `CLAUDE.md`, `NEXT.md`; (2) add registry row; (3) define IDE substrate (file tree, CodeMirror 6 editor, integrated terminal, LSP subprocess); (4) bilingual READMEs. Distinct from `app-privategit-workbench` (browser-based three-column developer tool for Totebox Orchestration community users — separate product, separate persona, separate crate).

### Priority 2 — `app-workplace-memo`

- **Current registry state:** Scaffold-coded; 47 files; running on Linux Mint per sibling's doc; CLAUDE.md + NEXT.md pending for Active.
- **Must happen before Active:** (1) Tauri v1 → v2 migration (2–5 engineer-days per app per Software Architect §"Migration effort"); (2) add CLAUDE.md + NEXT.md from templates; (3) implement launcher `Hello` handshake + accept MBA socket FD; (4) ship Markdown edit + autosave + autosave-to-canonical pattern matching `app-mediakit-knowledge`'s atomic-write discipline.
- **Dependencies:** launcher (Priority 1). Reuse CodeMirror 6 vendored bundle from `app-mediakit-knowledge/static/vendor/` per cleanup-log 2026-04-26 entry.
- **Transitions to Active when:** launcher-mediated first run, Memo opens/edits/saves a document, autosave verified, AArch64 build clean.

### Priority 3 — `app-workplace-pdf` — NEW

- **Current registry state:** does not exist; not in project-registry.md.
- **Must happen before Active:** (1) create directory + Cargo.toml + README pair + CLAUDE.md + NEXT.md; (2) add registry row under `app-workplace-*`; (3) integrate `pdfium-render` (MIT) for native PDF parsing; (4) implement annotation surface (highlights, sticky notes, freehand) per Acrobat muscle memory; (5) bilingual READMEs per workspace §6.
- **Dependencies:** launcher (Priority 1); shared `service-fs` access for recent-docs memory.
- **Transitions to Active when:** open + view + annotate + save round-trip clean on AArch64; launcher integration verified.

### Priority 4 — `app-workplace-proforma`

- **Current registry state:** Active; 45 files; CLAUDE.md present but marked "local-only"; conformance pending.
- **Must happen before Active state holds:** (1) resolve "local-only" CLAUDE.md status — promote to a workspace-conformant CLAUDE.md per template; (2) Tauri v1 → v2 migration; (3) implement launcher handshake; (4) native Rust table/formula engine (do NOT route formula evaluation through `service-slm` — SYS-ADR-07 hard rule applies to structured spreadsheet data per `~/Foundry/AGENT.md` § Hard rules); (5) XLSX + ODS import/export.
- **Dependencies:** launcher (Priority 1).
- **Transitions to fully Active when:** Tauri v2 migration complete, CLAUDE.md conformance, launcher integration verified, formula engine round-trips against a reference Excel file.

### Priority 4.5 — `app-workplace-schedule` — NEW

- **Current registry state:** does not exist; not in project-registry.md.
- **Must happen before Active:** (1) create directory `vendor/pointsav-monorepo/app-workplace-schedule/` with `Cargo.toml`, bilingual `README.md` + `README.es.md`, `CLAUDE.md`, `NEXT.md`, `CHANGELOG.md` per template; (2) add registry row under `app-workplace-*`; (3) adopt **TaskJuggler-style human-readable text DSL** (`.wpsched` or `.tjp`-extended) as the canonical on-disk format, paired with a TOML side-file for per-project metadata — git-diffable schedule history, baselines as git-tagged immutable snapshots, plain UTF-8 (no MPXJ binary parsing burden); (4) build native **egui Gantt widget** for v0 using egui `Painter` primitives — rectangles for bars, orthogonal polylines for FS dependency arrows, `resvg` for PDF/PNG export (no WebView dependency, consistent with `connect-src 'none'` + EUPL 1.2 + sovereign-binary distribution); (5) ship the Day-1 ten-feature surface (see §9 below): Gantt view, WBS indent/outdent, FS dependencies with lag/lead, drag-to-reschedule, PDF export, MPX + PMXML import + PMXML export, WBS-based CPM, baseline snapshot, print-to-paper (11×17 clean), resource = subcontractor named-only assignment; (6) MS Project key bindings as default (Alt+Shift+Right/Left indent, Ctrl+F2 link, Shift+F2 task info, Insert add row, Ctrl+Del delete, F5 go-to); P6 binding profile as selectable preference (Insert add sibling, F9 reschedule, F5 refresh); (7) launcher handshake (`Hello` / `OpenDocument` / `MbaSocketFd`).
- **Dependencies:** launcher (Priority 1); shared `pointsav-design-system` tokens for bar colours (blue normal / red critical-path / dark-grey summary / diamond milestone); MBA path to gateway for future multi-project portfolio view; no `service-slm` involvement for the scheduling engine (SYS-ADR-07 hard rule — schedule data is structured and must not transit AI inference).
- **Transitions to Active when:** launcher integration verified, `.wpsched` round-trips through git as plain-text diff, CPM calculation produces same critical-path output as a reference MS Project / P6 file on a contractor's test schedule, MPX + PMXML import/export round-trip clean, PDF export prints landscape 11×17 with bars + dependency arrows + WBS column intact, baseline snapshot survives a git tag + checkout cycle.

### Priority 5 — `app-workplace-presentation`

- **Current registry state:** Active; 52 files; CLAUDE.md present; Phase 5.
- **Must happen before v1.0 ship:** (1) Tauri v1 → v2 migration; (2) launcher handshake; (3) presenter-display surface (multi-window per Tauri v2 multi-window discipline — note GTK main-thread bug #11312 mitigated by per-app process isolation); (4) export to PDF + PPTX.
- **Dependencies:** launcher (Priority 1); shared design-system tokens for slide layouts.
- **Transitions to v1.0 ship when:** Phase 5 closes, launcher integration verified, two-window presenter mode verified.

### Priority 6 — `app-workplace-gis` — NEW

- **Current registry state:** does not exist; not in project-registry.md. (Distinct from `gateway-orchestration-gis-1` which serves the public read-only `gis.woodfinegroup.com` surface.)
- **Must happen before Active:** (1) create directory + Cargo.toml + README pair + CLAUDE.md + NEXT.md; (2) add registry row under `app-workplace-*`; (3) WebView-based MapLibre GL JS surface (native `maplibre-rs` deferred to v2 — too immature for v0 power-user GIS per Software Architect §"BIM/GIS WebView Constraint Analysis"); (4) integrate OpenFreeMap PMTiles for global vector tiles (~$0/mo); (5) MBA path to `cluster-totebox-personnel-1` for editable layer pulls; (6) attribute table + projection dialogs to match QGIS muscle memory; (7) DESIGN-RESEARCH outbound for ring/catchment/co-location UX per `vendor/pointsav-monorepo/.agent/rules/artifact-registry.md` items B6–B10.
- **Dependencies:** launcher (Priority 1); `app-mediakit-knowledge` style of WebView vendoring; gateway access already proven via `gateway-orchestration-gis-1`.
- **Transitions to Active when:** launcher integration verified, native projection round-trips (EPSG:4326 ↔ EPSG:3857), editable layer saves to `cluster-totebox-personnel-1`.

### Priority 7 — `app-workplace-bim` (ships last)

- **Current registry state:** Reserved-folder; 2 files (CLAUDE.md + RESEARCH.md); research phase; directory created 2026-04-23.
- **Must happen before Scaffold-coded → Active:** (1) operator requests **Creoox xeokit commercial license quote** per Economist §"BIM/GIS Ecosystem Licensing Costs" — this is the only unbounded cost variable in the architecture; (2) decide xeokit-or-not based on quote (acceptable: ship v0 with MPL-only stack at reduced rendering features); (3) author Cargo.toml + bilingual README + NEXT.md + CHANGELOG.md per template; (4) Tauri v2 scaffold; (5) WebView + web-ifc 0.77 (MPL-2.0) + @thatopen/components (MIT); (6) IfcOpenShell invoked via Python subprocess per `.agent/rules/bim-product-family.md` LGPL compliance (dynamic invocation, never static link); (7) follow flat-file BIM archive data contract in `cluster-totebox-property/service-bim/` (canonical/, elements/, drawings/, issues/, requirements/, ingestion/); (8) AutoCAD-phase muscle memory (Phase 1: L/PL/C/M/CO/TR/O/F/LA aliases, F3/F8/F10 toggles) before Navisworks-phase or Revit-phase scope is added; (9) the F12-equivalent commit-to-canonical action is operator-explicit per SYS-ADR-10.
- **Dependencies:** launcher (Priority 1); xeokit quote (operator action, parallel track); MBA path to `cluster-totebox-property-1`; `.agent/rules/bim-product-family.md` rules engagement.
- **Transitions to Active when:** launcher integration verified, IFC ingestion pipeline operates end-to-end against a real model, BCF 3.0 issue surface lands, IDS 1.0 validation gate green.

### Cross-cutting registry-state transitions to record this session

| App | Current state | After this BRIEF |
|---|---|---|
| `app-workplace-launcher` | Not in registry | Add row → Reserved-folder pending directory creation |
| `app-workplace-workbench` (coding IDE) | Not in registry | Add row → Reserved-folder pending directory creation (parallel track, independent of launcher delivery sequence) |
| `app-workplace-memo` | Scaffold-coded | Hold Scaffold-coded; queue Active promotion after launcher + Tauri v2 + CLAUDE.md/NEXT.md |
| `app-workplace-pdf` | Not in registry | Add row → Reserved-folder pending directory creation |
| `app-workplace-proforma` | Active (local-only CLAUDE.md) | Hold Active; queue conformance pass |
| `app-workplace-presentation` | Active (Phase 5) | Hold Active; queue Tauri v2 + launcher handshake |
| `app-workplace-gis` | Not in registry | Add row → Reserved-folder pending directory creation |
| `app-workplace-bim` | Reserved-folder | Hold Reserved-folder pending xeokit quote + Scaffold-coded scaffold |

### Self-check — questions a developer reading only this BRIEF can answer

- **(a) Which delivery model?** `app-workplace-launcher` (suite identity shell, ~5 MB, egui) + 8 desktop apps including `app-workplace-workbench` (coding IDE, ~150 MB, Tauri v2 + WebView), all AArch64-native, EUPL 1.2, `connect-src 'none'`. No browser. No PWA. No unified binary.
- **(b) Which app ships first?** Launcher + `app-workplace-memo` together, in that order. Launcher must be wireable end-to-end before Memo's handshake counts as a ship.
- **(c) What is the IPC model?** Unix domain socket at `$XDG_RUNTIME_DIR/workplace-launcher.sock`; length-prefixed CBOR or MessagePack frames; minimum message set `Hello` / `OpenDocument` / `RecentDocument` / `Quit` / `Heartbeat` / `MbaSocketFd` (the last via `SCM_RIGHTS` FD-passing). Crate baseline: `interprocess` and/or `tokio-unix-ipc`. Bespoke ~200-line supervisor on `tokio::process::Command`.
- **(d) Is browser delivery in scope?** No. All browser-leg options (b, c, e, f, h, i) are vetoed by the `connect-src 'none'` sovereignty constraint. Off-`os-workplace` browser delivery on generic hardware is a separate product with its own threat model and is not within this architecture.
- **(e) What must `os-workplace` provide?** AArch64-native bootable image (NetBSD compatibility today, seL4 native at moonshot per Doctrine claim #34); Wayland + GTK + WebKitGTK 4.1 stack; per-station WireGuard PPN identity at `10.42.20.x`; `pairings.yaml` entries enabling per-app TLS audit identity to `gateway-orchestration-command-1`; per-app data-directory isolation mapped to per-app seL4 CNode sets on the native bottom; freedesktop `.desktop` integration so launcher and each app appear in the OS app grid; reachable `service-slm` Doorman endpoint over WireGuard per claim #43.

## 9. app-workplace-schedule — Product Scope

Synthesis of the WP-SCHED-01 SMB survey (research date 2026-05-24). `app-workplace-schedule` slots in as Priority 4.5 in the `app-workplace-*` rollout — a Gantt / critical-path / resource-scheduling app for construction PMs, general contractors, and SMB project managers. Out of scope: calendaring (separate service), task management (Kanban-style, covered elsewhere), site/field BIM (`app-workplace-bim`).

### 9.1 Comparables verdict

| Tool | Verdict | Why not for us |
|---|---|---|
| **Primavera P6 Professional / EPPM** (Oracle) | Disqualified | $2,500–$3,520/user perpetual + $500–$800/yr support; cloud at $250–$350/user/mo; no native AArch64 / Linux; Windows + Oracle Cloud only. SMB-unaffordable; sovereignty-incompatible. But XER + PMXML are the de-facto contract-delivery formats — import/export must support them. |
| **ASTA Powerproject 2026** (Elecosoft) | Disqualified | Windows-only; Mac via SaaS; no Linux build. UK construction muscle-memory relevant but delivery-channel incompatible. |
| **Microsoft Project (Plan 3 / Plan 5)** | Disqualified | Windows desktop only; subscription $10–$55/user/mo; Plan 5 end-of-sale **2026-05-01**, Project Online retires **2026-09-30**. File format (MPP) + shortcut grammar are the universal SMB muscle-memory targets. |
| **GanttProject 3.4** | Reference, not vendor | GPL v3 Java/JavaFX desktop; no native AArch64 binary (runs via Liberica JDK 17+). Java runtime incompatible with sovereign-Rust os-workplace ethos. Useful as feature-surface reference; active (3.4 Beta IV May 2026). |
| **ProjectLibre 1.9.8** | Reference, not vendor | CPAL 1.0 Java MS-Project clone; CPAL attribution clauses unwelcome in EUPL substrate; modern-Java compatibility is a sustained pain point. Closest open MS-Project lookalike for UX study only. |
| **OpenProject Community 14.x** | Strong reference for `service-schedule` backend | GPL v3 Ruby on Rails web stack; official ARM64 Docker images. UI is a browser app, not a desktop Gantt — cannot ship as desktop binary. Useful as backend data-model reference if multi-user collab is wanted in v1. |
| **Planner (GNOME)** | Reference only | GPL v2 GTK desktop; slow-cadence development (~6 mo between commits); GTK2 vintage. Not a forward platform. |
| **TaskJuggler 3.x** | Strong reference for the engine layer | GPL v2 Ruby CLI + text DSL; pure Ruby; "schedule everything from a `.tjp` text file" pattern is exactly the sovereign-friendly storage model we should adopt internally. Adopted as the native-format model (see §9.2). |
| **Linear** | Out of category | Issue tracker w/ optional roadmap timeline; not a Gantt or CPM tool. SaaS-only. |
| **Notion** | Out of category | Knowledge base with database timeline views; not real Gantt; no dependencies, levelling, or baselines. SaaS-only. |
| **Basecamp** | Out of category | Team comms + flat task lists; "Hill Chart" only; DHH's stated philosophy is explicitly anti-Gantt. |
| **GanttPRO / TeamGantt / Smartsheet** | Disqualified | SaaS-only sovereignty disqualifier ($10–$25/user/mo); useful as UX competitive reference for non-construction SMB feature parity. |

**Bottom line:** there is no current native desktop Gantt for AArch64 Linux that is both actively maintained and FOSS-licensed in an EUPL-compatible way. This is the gap `app-workplace-schedule` targets.

### 9.2 Native format decision

**Adopt a TaskJuggler-style human-readable text DSL** (`.wpsched`, or extend TJ3's `.tjp` directly) as the canonical on-disk format, paired with a TOML/YAML side-file for per-project metadata.

Justification:

- **Git-diffability** — schedule history becomes a `git log` of the `.wpsched` file; baseline comparison is a `git diff` against a tagged commit. The text-DSL format is the load-bearing primitive that makes the FIDIC §20.1 contemporaneous-record requirement (see §9.7) work without bolt-on audit infrastructure.
- **EUPL 1.2 compatibility** — TaskJuggler itself is GPL v2 Ruby; we are adopting the *format* and DSL conventions, not the implementation. The format is unencumbered text — no proprietary spec, no licence-encumbered parser library required. A native Rust parser sits cleanly under EUPL 1.2 with no upstream copyleft contamination.
- **100-year readability** (Doctrine claim #2) — plain UTF-8; no MPXJ-style binary-OLE-compound-document parsing burden; no derived state to corrupt; the on-disk file is also the scheduling-engine input (single source of truth).
- **Cryptographic integrity** — trivial to SHA-256 anchor via `local-fs-anchoring`; Ed25519-signable for baselines (v1).
- **Sovereignty** — no proprietary spec; no licence-encumbered library; reads cleanly on any Unix in 2126 with `cat`.

Formats explicitly *not* adopted as native:

- **JSON** — no native commenting, hostile to git diffs once nested arrays grow. Used only for IPC payloads to/from the renderer.
- **iCal VTODO** — RFC 5545 has no concept of dependencies, lag/lead, CPM, resources, or baselines. Export-only, never native.
- **OASIS UOF** — never gained adoption; effectively dead.

### 9.3 v0 feature set (Day-1 launch)

Ten table-stakes features, exact list:

1. **Gantt chart view** — tasks as horizontal bars on a date axis.
2. **WBS hierarchy** with indent/outdent (Alt+Shift+Right/Left).
3. **Finish-to-start dependency** with lag/lead time (numeric days).
4. **Drag-to-reschedule** task bars (start + finish), with cascade through dependencies.
5. **PDF export** — landscape, configurable date range, fits-to-page; non-negotiable for contract delivery to GC/owner.
6. **MPX import + PMXML import + PMXML export** — SMB subs receive and return schedules from prime contractors on MS Project or P6.
7. **WBS-based critical-path calculation** (longest dependency chain to project finish). Required for delay litigation; standard scheduling method referenced by FIDIC and SCL Delay & Disruption Protocol.
8. **Baseline snapshot** — save current schedule as baseline; show planned-vs-actual bars stacked.
9. **Print to paper** — clean 11×17 print (the sticky-notes-on-wall replacement only works if the print is clean).
10. **Resource = subcontractor list** — even if levelling is deferred, assigning "Acme Drywall" to a bar is required by the contractor workflow.

### 9.4 v1 feature set (deferred)

- **Resource levelling** (automatic over-allocation detection + automatic delay of lower-priority parallel tasks). MS Project's algorithm is the muscle-memory benchmark.
- **Earned Value Management** (BCWS, BCWP, ACWP, CPI, SPI). Not statutorily required for SMB Canadian/EU GC work; expected on owner-lender-financed projects > ~$5M CAD.
- **MPP import** (binary, via MPXJ called as a JVM sidecar — LGPL-clean by IPC isolation; native Rust replacement in v1+).
- **XER import** (native Rust parser; existing Python references `xerparser`, `xer-reader`, `PyP6Xer` document the structure cleanly — port is a few weeks of work).
- **All four dependency types** (SS, FF, SF in addition to FS).
- **Multiple baselines** (MS Project supports 11; v1 needs at least 3).
- **Multi-project / portfolio view** — many SMB GCs run 3–8 concurrent jobs.
- **Resource histogram** — per-subcontractor over-allocation visualisation.
- **Calendar exceptions** — site closure, weather days, regional holidays (Canadian provincial + EU member-state holiday tables).

**Future (post-v1):** 4D BIM link (phase IFC geometry by task bar — handoff into `app-workplace-bim`), time-distance / linear-schedule view (road, rail, tunnel, high-rise), Monte Carlo schedule risk simulation, schedule-of-record signing (Ed25519 + WORM-ledger anchor for delay-arbitration citation), web/mobile read-only viewer served from a future `service-schedule`.

### 9.5 Rendering decision

**Native egui Gantt widget for v0.** Build it ourselves using egui's `Painter` primitives (rectangles for bars, orthogonal polylines for FS dependency arrows, hit-test for drag-to-resize, stacked rectangles for resource histogram). Budget: 4–8 engineer-weeks for Day-1-quality bar + dependency renderer; another 4 weeks for printing/PDF.

Reasoning vs the WebView-vendored frappe-gantt fallback:

1. **Toolkit consistency** — the other `app-workplace-*` apps are egui-based; one consistent toolkit avoids two-renderer drift across the suite.
2. **Sovereignty** — `connect-src 'none'` + "no browser delivery" + EUPL 1.2 + sovereign-binary distribution all point against a WebView dependency that has to be vendored, patched for security, and explained to auditors.
3. **Renderer complexity ceiling is low** — Gantt at construction-SMB level is genuinely *simpler* than the comparable BIM or GIS canvas: bars are rectangles, dependency arrows are 90° elbows, drag-to-resize is hit-test on bar edges, critical-path highlight is colour-the-bars-with-total-float-zero. The hard part is the *scheduler* (CPM, levelling, calendar exceptions), which is identical regardless of renderer — pure Rust, the renderer consumes the resulting layout.
4. **PDF/PNG export via `resvg`** — pure-Rust SVG-to-raster pipeline, no Chromium dependency.

**Fallback only if v0 schedule pressure forces it:** vendored **frappe-gantt** (MIT, 50 KB, SVG, no runtime CDN — `connect-src 'none'`-compatible if locally vendored). Replace with native egui in v1. **Never** DHTMLX (commercial from $699/seat, closed-source — breaks EUPL substrate). **Never** Bryntum (most feature-complete but commercial and very expensive — breaks sovereignty story).

### 9.6 Muscle-memory conventions

**MS Project key bindings as default.** P6 binding profile as a selectable preference toggle for users coming from the construction-scheduler track.

MS Project default bindings:

| Action | Shortcut |
|---|---|
| Indent task (make child) | **Alt+Shift+Right** |
| Outdent task (promote) | **Alt+Shift+Left** |
| Expand summary | **Alt+Shift+Plus** |
| Collapse summary | **Alt+Shift+Minus** |
| Go to task by ID | **F5** (opens "Go To" dialog) |
| Insert new task | **Insert** (adds row above current selection) |
| Delete task | **Ctrl+Del** (bare Del clears cell — preserve MS Project distinction) |
| Link selected tasks (FS dependency) | **Ctrl+F2** |
| Unlink tasks | **Ctrl+Shift+F2** |
| Task Information dialog | **Shift+F2** |
| New project | **Ctrl+N** |
| Open | **Ctrl+O** |
| Save | **Ctrl+S** |
| Print | **Ctrl+P** |
| Find | **Ctrl+F** |
| Replace | **Ctrl+H** |

P6 alt-binding profile (selectable):

| Action | Shortcut |
|---|---|
| Add sibling activity | **Insert** |
| Add sibling WBS | **Ctrl+Alt+A** |
| Reschedule (recalculate dates) | **F9** (P6 schedulers reflexively press F9 — must be supported even if our scheduler auto-recalculates) |
| Refresh view | **F5** (different meaning than MS Project's "Go To" — honour in P6 profile) |
| Schedule dialog | **F9** |

UI conventions to copy verbatim (universal across MS Project, P6, GanttProject, ProjectLibre, OpenProject, Powerproject, every SaaS Gantt — deviating loses every existing user):

- Two-pane layout: tasks-grid left, Gantt bars right, vertically synchronised scroll, draggable splitter.
- First grid column = WBS / outline ID, computed not entered (1, 1.1, 1.1.1...).
- Default columns visible: ID, Task Name, Duration, Start, Finish, Predecessors, Resource Names (MS Project default).
- Today line as a vertical highlight in the Gantt panel.
- Bar colours: blue normal, red critical-path, dark grey summary rollup, diamond milestone (MS Project default since 1995).
- Right-click bar: Task Information / Link / Unlink / Mark % Complete / Set as Milestone / Show on Timeline.
- Dependency drag-create: hover predecessor bar → grab handle at edges → drag dotted line to successor → release on bar → FS link with 0-day lag (GanttPRO / TeamGantt / MS Project convention).
- Date arithmetic input in duration/lag: accept `5d`, `2w`, `3h`, `0d` (milestone), `15ed` (elapsed days, ignoring calendar) — MS Project syntax.
- Predecessors column grammar: `4FS+2d` = predecessor task ID 4, finish-to-start, plus 2 days lag — MS Project syntax; universal across tools.

Conventions to consciously break and surface as preferences: MS Project's auto-scheduling vs manually-scheduled per-task toggle (default auto, hide manual behind advanced preference); P6's three-layer Project/WBS/Activity model with explicit codes (flatten to one "task with optional WBS rollup" model).

### 9.7 BCSC / regulatory note

**FIDIC §20.1 / SCL Delay & Disruption Protocol contemporaneous-schedule requirement.** FIDIC 1999/2017 Red, Yellow, and Silver Books are the most cited international construction contract templates. §20.1 requires notice of any delay-causing event within **28 days** of the contractor becoming aware, with quantified claim to the engineer within **42 days**; sub-clauses 6.10, 4.21, and 8.3 require **contemporaneous records** — daily logs, correspondence, progress reports, photos, schedule updates — sufficient to substantiate the event and its impact on the critical path. FIDIC's own guidance references the SCL Delay & Disruption Protocol (2nd ed.) as the increasingly-adopted international methodology; SCL's preferred delay-analysis method is **windows analysis** — the project timeline is sliced into monthly windows; within each window the actual progress vs baseline + the then-current critical path are recorded. This is a **schedule-integrity requirement, not just a documentation requirement**: the contractor must demonstrate which baseline was current at each window and that the as-built schedule was captured without retroactive edits.

Implications for `app-workplace-schedule`:

- **Baselines as git-tagged immutable snapshots** — the text-DSL native format makes this natural: every baselined schedule is a tagged commit in the project's own git history; checkout of the tag reproduces the schedule as it stood. Windows-analysis as-of-date snapshots come for free.
- **No retroactive baseline edits** — UI must require an explicit "new baseline" action; never silently overwrite. Display a warning if a baseline tag has been moved.
- **Optional Ed25519 signing** (v1) — baselines signed by the contractor's identity and (optionally) anchored to the workspace's local WORM ledger, making the baseline citable in arbitration as "cryptographically established to have existed at time T". Same pattern `data/binary-ledger/` uses for software-unit deployments.
- **Schedule-update audit trail** — every change to dates/dependencies recorded with timestamp + user via git commit metadata; all writes go through a commit hook so the ledger is never bypassed.

Canadian / EU specifics: no statutory schedule-audit requirement for SMB private construction in Canada (CCDC contract templates require submission but no methodology); EU national rules vary (German VOB/B §5 has FIDIC-analogous delay-notification provisions; UK JCT contracts reference SCL Protocol explicitly). EVM is statutorily required only on US federal contracts > $20M — deferred to v1.

**BCSC posture for marketing copy** (per `feedback_bcsc_disclosure.md` + workspace §6 BCSC continuous-disclosure rule): use "planned/intended" language. Examples — "Schedule-of-record signing planned for v1"; never "delivers FIDIC-compliant audit trail today" before the signing pipeline ships. Avoid implying any single tool is sufficient for FIDIC claims — the tool provides the record; the claim itself is a legal artefact requiring contemporaneous records *plus* expert delay analysis.

### 9.8 MS Project retirement note — market window

Microsoft has set hard end-of-sale and retirement dates that open a structural market window:

- **Microsoft Project Plan 5 end-of-sale: 2026-05-01** (already past as of this BRIEF's date).
- **Project Online retirement: 2026-09-30** (~4 months from this BRIEF's date).

The MS Project file format (`.mpp`) and shortcut grammar are the universal SMB muscle-memory target. With Microsoft itself retiring the SaaS surface and forcing customers onto Plan 3 / Plan 1 (lighter SKUs) or Project for the web (browser-only), a meaningful population of construction SMB users will be evaluating alternatives in the window between 2026-05 and 2026-12. `app-workplace-schedule` shipping with **MPX/PMXML round-trip + MS Project key bindings + native AArch64 desktop delivery + EUPL 1.2 + perpetual $19 SKU** (per the ratified software distribution substrate 2026-05-22) is exactly the structurally-positioned alternative for that population. Use "planned/intended" disclosure language for any compliance or feature-readiness claim until the v0 ship lands.

---

## 10. os-workplace Desktop Architecture

> Synthesis of WP-DESK-01..08 (software architect, UX engineer, UX researcher,
> product manager, systems designer, economist, SMB survey, desktop integration).
> Appended 2026-05-24 by synthesis-agent (Command Session, claude-opus-4-7).
> This section governs the desktop layer of `os-workplace`; the per-app
> delivery model in §1–§8 and the `app-workplace-schedule` product scope in
> §9 remain canonical.
>
> **Second deployment shape — TUI Desktop:** A complementary sovereignty-maximal
> deployment shape (GPU-accelerated terminal emulator → `pointsav-tui-shell`
> multiplexer-WM → ratatui apps with Kitty graphics) is specified separately in
> `BRIEF-tui-desktop-architecture.md`. The niri-fork + Tauri v2 graphical shape
> documented in this section remains canonical for apps requiring WebView
> (app-workplace-bim, app-workplace-gis) and full-desktop customers. Both shapes
> share the BFS attribute store, substrate-handle broker, and
> `software.pointsav.com` distribution model.

### 10.1 Agent Consensus Map

| Agent | Core recommendation | Key finding (one sentence) |
|---|---|---|
| **WP-DESK-01 Software Architect** | Fork niri (GPL-3.0, Smithay-based, Rust, AArch64-clean); relicense the workplace shell layer as EUPL 1.2 on top | Bespoke Smithay-from-scratch compositor costs 9–14 EM to v1 (18–24 EM to niri parity) — niri at 2–4 EM is the honest fork target. |
| **WP-DESK-02 UX Engineer** | Synthesise Acme plumbing + Zellij mode-bar discoverability + NixOS-style declarative config; ship a 50-line universal F-row keyboard vocabulary | A configuration-via-CLI `os-workplace config` surface backed by three TOML layers (L0 image / L1 policy / L2 user) replaces the "system preferences" panel without losing accessibility. |
| **WP-DESK-03 UX Researcher** | "Tiling-capable, not tiling-default" — stacking-mode day-1 with a discoverable "Discipline Track" promotion to tiling | Pure tiling-default fails the 2-week onboarding gate for bookkeepers/PMs/architects; realistic coder-first ceiling is 20–35% of the cohort over 12 months. |
| **WP-DESK-04 Product Manager** | Lead with construction/BIM beachhead vertical; deliver "OS IS the app suite" as the product reality but not the headline | EU BIM mandate (≥€1M public projects since Jan 2025) is a now-binding regulatory clock that creates a forced refresh; $39 Compliance Bundle is the default SKU, not per-app à la carte. |
| **WP-DESK-05 Systems Designer** | Smithay-based compositor on NetBSD bottom for v1; LionsOS framebuffer-passthrough pattern for stage-1 seL4; Nitpicker-modelled native compositor as a moonshot | sDDF 0.6.0 has no graphics device class; LionsOS itself reuses the Linux graphics driver — seL4-native display is genuinely 4–8 PY, not v1 work. |
| **WP-DESK-06 Economist** | Ship Sway unmodified + an `os-workplace-shell` integration shim for v1; defer bespoke compositor to v3+ post-revenue | A 30× cost gap (~$85k Sway-unmodified vs ~$1.0M direct + $2.0M opportunity for greenfield Smithay) is dispositive for a 2-engineer pre-revenue team. |
| **WP-DESK-07 SMB Survey** | Dual-mode default (stacking + opt-in tiling); position as "the workstation that finishes Schleswig-Holstein's pilot"; refuse the "BIM-author on Linux" claim | Zero published case studies of non-developer SMBs running a tiling WM as daily-driver in any sector — os-workplace is establishing the first reference site. |
| **WP-DESK-08 Desktop Integration** | BeOS BFS-attribute model as substrate; typed `Super+Shift+s` Send palette replaces Share Sheet; one persistent notification tile (not a drawer) | The "OS IS the app suite" is the only thing no hyperscaler can ship — each has a structural P&L blocker (Google browser thesis, Apple App Store, Microsoft Win32 ISV). |

### 10.2 Compositor Decision

**Recommendation: fork `niri` (GPL-3.0, Smithay-based, Rust, AArch64-clean) for v1.** Rebrand to `os-workplace-compositor`; replace KDL config with a hard-coded workplace policy file driven via IPC from `app-workplace-launcher`; add a private `workplace-shell-v1` Wayland protocol that gates client identity ("every visible surface MUST come from an `app-workplace-*` binary"). Defer the bespoke Smithay-from-scratch decision to v0.5 ratification with an explicit trip-wire (>3 patches that cannot be upstreamed → escalate).

#### Weighted reasoning matrix

| Criterion | Weight | Sway-unmodified (Path E) | Fork niri | Fork Sway | Bespoke Smithay | Rationale |
|---|---:|---:|---:|---:|---:|---|
| Engineering cost to v1 (lower is better) | 25% | 9 | 7 | 8 | 2 | Sway-unmodified ~0.5–1.5 EM (Agent 06); niri-fork 2–4 EM (Agent 01); Sway-fork 2–4 EM; bespoke 9–14 EM (Agent 01). |
| AArch64 maturity | 15% | 9 | 9 | 9 | 6 | All three forks ship AArch64 today; bespoke must re-prove the matrix. |
| Rust-language coherence with app stack | 15% | 3 | 9 | 3 | 9 | niri and Smithay are Rust-through; Sway is C with no current Rust bindings. PointSav's apps are Rust. |
| EUPL 1.2 outbound compatibility | 10% | 9 (MIT in) | 7 (GPL-3.0 boundary, separable) | 9 (MIT in) | 9 (MIT Smithay in) | GPL-3.0 contamination is manageable as a separable binary component, not a fork blocker. |
| Workplace-policy invariance (compositor can enforce "no non-`app-workplace-*` clients") | 15% | 3 | 7 | 5 | 10 | Bespoke owns this absolutely; niri's IPC + private protocol can carry it; Sway-unmodified cannot. |
| Onboarding-cost story (tiling-capable, not tiling-default) | 10% | 6 | 8 | 6 | 8 | niri's scrolling-tile + horizontal-strip is a mouse-friendly extension of i3 muscle memory (Agent 01). |
| seL4 retargeting cost (preserve Wayland-protocol abstraction boundary) | 10% | 6 | 8 | 6 | 9 | Owning the source tree makes the eventual port a known cost; Sway-unmodified inherits the upstream cadence. |
| **Weighted total (1–10)** | **100%** | **6.6** | **7.85** | **6.65** | **6.75** | niri-fork wins on the composite. |

**Why niri beats Sway-unmodified despite the 30× cost gap:** the Agent 06 economist analysis assumes the compositor is undifferentiated infrastructure — a thesis that holds for v1 ARR capture but fails on the leapfrog-2030 axis. Workplace-policy invariance (compositor enforces "only `app-workplace-*` clients") and Rust-coherence with the app stack are structural moats the agent ranks below revenue velocity. The synthesis position: niri-fork at 2–4 EM is the *smallest* premium over Sway-unmodified that buys both invariants. Sway-unmodified remains the explicit fallback if the v0.1 milestone slips by >2 EM.

**License analysis (per Agent 01).** GPL-3.0 is in the EUPL 1.2 outbound compatibility appendix — the `os-workplace` stack (EUPL 1.2 in vendor) can lawfully consume and ship niri-derived code as part of the OS image, provided distribution of that component is GPL-3.0. The compositor lives as a separable GPL-3.0 binary; `app-workplace-*` apps over Wayland IPC remain EUPL 1.2. Direction matters: GPL→EUPL does *not* work, so the workplace shell layer is published as a new EUPL-1.2 component, not as a relicense of the niri fork itself.

**Engineering cost and timeline (one full-time senior Rust engineer, AArch64-fluent, Sonnet/Opus-augmented).**
- **v0.1 (rebrand + KDL strip + `workplace-shell-v1` protocol):** 2–4 EM, 3 calendar months
- **v0.3 (launcher IPC wired, F-row vocabulary enforced, mode-bar live):** +3 EM, 6 calendar months cumulative
- **v0.5 (`os-workplace config` CLI + L0/L1/L2 TOML reload + accessibility-first `app-workplace-settings` panel):** +3 EM, 9 calendar months cumulative
- **Maintenance steady-state:** ~3 EM/year (upstream rebase + AArch64 QA across three SoC families).

**AArch64 GPU bet.** Two-target initial QA matrix, both supported by Agents 01 and 05:
- **Primary: Qualcomm Snapdragon X-series (Adreno) via freedreno + Turnip** — Mesa 25.1's default AArch64 Vulkan; production-grade.
- **Secondary: ARM Mali Valhall (G610-class) via Panfrost + PanVK** — Vulkan 1.1 conformant as of April 2025; the strongest open-driver story (Agent 05).
- Tertiary eval-only: Rockchip RK3588 boards (Panfrost) for low-cost SMB hardware; Apple M1/M2 (Asahi) for Mac-resident developers.
- Explicitly out of scope for v1: Apple M3/M4/M5 (Asahi GPU work in progress); NVIDIA Jetson; M-series Apple Silicon for customer-facing deployments.

### 10.3 "Tiling-capable, not tiling-default" — the onboarding model

The single sharpest finding across the eight briefs: **Agent 03 (UX Researcher) and Agent 07 (SMB Survey) independently converge on the conclusion that pure tiling-default fails the SMB onboarding gate.** The realistic ceiling for "coder-first" daily posture is 20–35% of the cohort over 12 months even with Regolith-style scaffolding; the remaining 65–80% retreat to mouse-first habits within hours unless mouse-first is the default. Zero published case studies show a non-developer SMB team running a tiling WM as their daily driver.

**Proposed model: dual-mode default with a discoverable "Discipline Track" upgrade.**

| Phase | Default mode | Trigger | What changes |
|---|---|---|---|
| **Day 0** | Stacking layout, mouse-first defaults, persistent status row | Boot | Arrow keys work; single-click focuses; `Super+space` opens command palette; mode-bar shows "NORMAL — `Super+?` for help"; `Super+T` flips to tiling |
| **Week 2** | Stacking + focus-promotion hints | `os-workplace promote --next` graduates the user to using `Super+hjkl` for focus | Mode-bar adds "tiling-ready — `Super+T` to lock in"; achievements badge appears in `app-workplace-settings`. |
| **Week 4** | Tiling layout, keyboard-first | User runs `os-workplace promote --next` or operator policy locks at L1 | Compositor switches default to scroll-strip tiling; mouse drag-to-resize remains; status row shows current mode (NORMAL/MOVE/RESIZE/PLUMB) |
| **Month 3** | Tiling + plumb-by-selection | User runs `os-workplace promote --next` | Acme-style Button-3 plumb-send activates; `Super+Shift+s` Send palette becomes muscle memory |

Each promotion is opt-in, reversible (`os-workplace promote --rollback`), and surfaced in the mode-bar. L1 policy can lock a user at any phase. This honours Agent 03's "successful leapfrog tactic" finding (progressive disclosure beats demanding mastery on day 1) and Agent 07's "Schleswig-Holstein phased substitution playbook" — both empirically successful templates.

**Universal keyboard vocabulary (50 lines, from Agent 02).** Every `app-workplace-*` MUST honour this vocabulary; deviations require a doctrine amendment. Filed at `vendor/pointsav-os-workplace/conventions/keyboard-vocabulary.md`:

```
F-row (single key, no modifier):
  F1  = help (overlay; ESC dismisses)              [universal: Excel/Revit/QGIS all use F1]
  F2  = rename / edit-in-place                     [universal: Windows Explorer, Excel cell edit]
  F3  = find-next                                  [universal: browsers, editors]
  F4  = repeat / fill-down                         [Excel formula toggle, Revit "repeat last"]
  F5  = refresh / recalc                           [universal: browsers, QGIS, debuggers]
  F6  = focus next pane                            [browsers address-bar; reuse here]
  F7  = check / validate (spell, lint, schema)
  F8  = run / build / step                         [IDE conventional]
  F9  = recalc all (proforma) / set breakpoint
  F10 = focus menu / mode switch
  F11 = full-screen toggle                         [universal: browsers]
  F12 = save-as / export                           [Word, Excel — universal; SYS-ADR-10 mandatory]

Ctrl row (data verbs):
  C-n new   C-o open   C-s save   C-w close-pane   C-q quit-app
  C-x cut   C-c copy   C-v paste  C-z undo         C-y redo
  C-f find  C-g find-next-prompted   C-h replace
  C-Home top-of-document   C-End bottom-of-document
  C-PgUp/PgDn previous/next tab or sheet           [Excel sheet nav — universal]

Super (WM verbs — never collide with apps):
  S-Enter      = launch terminal
  S-d          = launcher (dmenu-style fuzzy)
  S-h/j/k/l    = focus left/down/up/right pane
  S-H/J/K/L    = move pane left/down/up/right
  S-1..9       = workspace 1..9
  S-S-1..9     = move to workspace 1..9
  S-Tab        = next workspace
  S-Space      = mode switch (NORMAL → MOVE → RESIZE → PLUMB → LAUNCH)
  S-p          = plumb selection (replaces Open-With dialog)
  S-`          = toggle scratchpad / special workspace
  S-,          = open user.toml in editor
  S-?          = show all bindings active in current mode

Alt row (reserved for app-internal verbs; apps may define):
  Apps SHOULD use Alt + letter for app-specific verbs.
  Apps MUST NOT bind any unmodified F-key, Ctrl combo above, or Super combo.

Mouse:
  Button 1 (left):   select / focus
  Button 2 (middle): execute selection (Acme-chord style)
  Button 3 (right):  context menu (text apps) / plumb selection (Acme apps)
  B1+B2 chord:       cut    B1+B3 chord: paste
```

**"No system preferences": the three-layer config model (from Agent 02).**

| Layer | File | Owner | Lifecycle | Examples |
|---|---|---|---|---|
| **L0 — Image defaults** | `/etc/os-workplace/defaults.toml` | PointSav (read-only, signed) | Locked at image build | Default font, theme, keyboard layout list, WM mode definitions, plumbing baseline rules |
| **L1 — Deployment policy** | `/etc/os-workplace/policy.toml` | Customer admin (signed via `factory-release-engineering` identity) | Updated on Stage 5 / customer cut | Workgroup printers, mail server, F-row vocabulary overrides, allowlisted `app-workplace-*` set, promotion-track lock |
| **L2 — User preferences** | `~/.config/os-workplace/user.toml` | The user | Live-reloaded; version-controlled in `~/.os-workplace-state/.git` | Personal keymaps, accent colour, scratchpad app list, custom plumbing rules |

Precedence L2 → L1 → L0 by override; `inotify`-driven live-reload in <200 ms for >95% of preferences. The single canonical CLI surface:

```
os-workplace config get <key>
os-workplace config set <key> <value>
os-workplace config diff           # local vs L1 vs L0
os-workplace config validate
os-workplace config rollback       # to previous generation
os-workplace config generations    # list rollback targets
os-workplace config edit           # opens user.toml in $EDITOR
os-workplace config explain <key>  # man-page-style help — the panel-replacement verb
```

Accessibility concession (Agent 02's dissent): `app-workplace-settings` ships as a screen-reader-first read/write *renderer* of the L0/L1/L2 TOML files. The panel is a view, not the source of truth — this satisfies WCAG 2.2 + BCSC audit-disclosure requirements without re-introducing the GNOME-Settings sprawl.

### 10.4 GTM and pricing synthesis

**Best beachhead vertical: construction/BIM (per Agent 04).** Ranked decision matrix from Agent 04:

| Vertical | Sovereignty pull | Procurement pain | Seat density | Halo |
|---|---|---|---|---|
| **Construction (BIM)** | High (EU procurement BIM mandate ≥€1M projects since Jan 2025) | Acute (Revit LT $545/yr + Primavera P6 $2,500–3,500 perpetual + MS Project $30/user/mo) | High (project teams 20–60 seats) | Strong (project consortia spread tooling) |
| Legal | Highest (DE §203 StGB criminal-grade secrecy; Quebec Law 25) | High (MS Cloud unacceptable for many privileged matters) | Medium (20–80 seats) | Moderate (referrals + bar associations) |
| Engineering | Medium-high (ITAR-adjacent; EU dual-use export controls) | Medium (Autodesk hegemony tolerated) | Medium | Moderate |

**Sequence: Construction (Yr 1) → Legal (Yr 2) → Engineering (Yr 3).** The EU BIM mandate is a now-binding regulatory clock; the existing `project-bim` cluster (woodfine-design-bim → bim.woodfinegroup.com) gives internal feed-forward; project teams are the highest-seat-density beachhead.

**Pricing — reconciling per-app FSL with bundle economics.** The ratified pricing (per `project_software_distribution_substrate.md` in memory) is $1 Apache 2.0 / $19 FSL per-app per-seat perpetual. Agent 04 and Agent 06 (economist) both recommend raising the bundled price; the synthesis position keeps the ratified floor and adds bundle SKUs above it:

| SKU | Price | Includes | Audience |
|---|---|---|---|
| **os-workplace Foundation** (EUPL 1.2) | $0 | Bare OS; no `app-workplace-*` installed; community support | Developers, hobbyists, evaluators |
| **app-workplace-{x} Apache 2.0** | $1/app/seat/mo perpetual | Single app, FOSS terms, community channels | Hobbyists, OSS purists, transparency mandate |
| **app-workplace-{x} FSL** | $19/app/seat/mo perpetual | Single app, commercial channels, support eligibility | À la carte buyers |
| **Workplace Compliance Bundle** (new) | $39/seat/mo perpetual | All `app-workplace-*`, priority support, EU/Canada data-residency attestations, BCSC-compatible audit pack | **Default SMB SKU; construction/legal/engineering beachhead** |
| **Workplace Sovereign Tier** (new) | $79/seat/mo perpetual | Bundle + dedicated tenant OR self-hosted with vendor cosign + 4h SLA + named TAM | Public-sector-adjacent SMBs; Quebec Law 25 / GDPR Art. 32 buyers |
| **Compliance Assurance** (optional, per Agent 06) | +$19/seat/yr | Audit-log retention, security update SLA, DPIA template kit | Subscription line item on top of FSL |

The $39 bundle sits 53% below ONLYOFFICE Workspace, 48% below Nextcloud Enterprise + add-ons, and 96% below Microsoft 365 E5 — preserving the sovereignty-at-disruptive-price narrative while not signalling "consumer-grade" via $19 per-app. Per-app FSL remains catalogued and clickable for OSS-friendly customers.

**"Lead-with-vertical, deliver-as-OS" GTM framing (Agent 04).** Public-facing positioning is **"Sovereign BIM workstation for EU construction SMBs — single vendor, EU-resident, perpetual licence, BIM-mandate-compliant out of the box."** "Powered by os-workplace" goes beneath the fold. The architectural truth — the OS IS the app suite — is a *product proof point* that becomes visible during use (single update plane, no driver glue, no install ladder). Buyers procure capabilities; they do not procure operating systems.

**MSP-first channel (per Agent 04).** SMBs do not have IT staff to absorb a desktop swap; MSPs do. 30% perpetual revenue share + free reference deployment in MSP's own office. Target MSPs in Quebec (Law 25 panic-buyers), Bavaria/NRW (Schleswig-Holstein halo), Île-de-France (gendarmerie halo).

**Market sizing (per Agent 04).** TAM (EU-27 + Canada regulated SMB, 10–200 employees, sovereignty-relevant verticals): ~3.5M firms × 35 seats avg = ~123M seats; at $30/seat/mo blended ≈ **TAM ARR $44B**. SAM (5-year window, acute-sovereignty-pressure geographies): ~620k firms × 35 × 60% IT-mature = ~13M seats ≈ **SAM ARR $4.7B**. SOM (3-year credible capture, no installed base, no channel yet, calibrated against Nextcloud's 2M new seats in 2025): **40k–130k seats ≈ $14M–$47M ARR by month 36**.

### 10.5 The "OS IS the app suite" integration model

**BeOS attribute model / typed dataflow bus (per Agent 08).** Every artifact in `os-workplace` is a BFS-equivalent object with a stable inode-id and a typed attribute set. The OS exposes a kernel interface (`/dev/attr` or equivalent) so any `app-workplace-*` can `query` (`SELECT inode WHERE MAIL:thread == X AND BIM:project == Y`), `subscribe` (notify-on-change watchers), and `transfer` (hand a substrate handle to another subsystem). Apps query attributes; they do not maintain their own catalogues. This collapses six layers — file manager, search index, recents, tags, smart folders, app-internal catalogues — into one substrate, which is the precondition for "no file manager." Implementation choice (open question from Agent 08): Haiku BFS port vs Btrfs/ZFS extended attributes + indexed query daemon. Decision deferred to v0.5 ratification; v0.1 ships with Btrfs+xattrs+sqlite-index pragmatic substitute.

**Jennifer's 30-minute journey (concrete step-by-step, from Agent 08).**

- **T+0:00 — Boot.** UEFI → verified boot via signed initramfs → seat manager → `app-workplace-launcher` paints the first frame. No GRUB menu, no display manager chooser.
- **T+0:08 — Login.** Single-field PIN/passphrase tied to device-local identity keychain. No "sign in to Apple/Google/Microsoft." Login unlocks (i) BFS-equivalent attribute store, (ii) Totebox mount, (iii) local SLM (`service-slm`) for offline assistance.
- **T+0:15 — Launcher appears.** Tiling-capable compositor (default in stacking mode for week 1; promotes per §9.3). Status bar across top: left = current Totebox + branch; centre = mailbox + chat unread counts (the notification *tile*, not a drawer); right = clock + battery + offline/online + SLM readiness. App grid shows installed `app-workplace-*` apps; first-run surface shows suite discovery ("1 app installed, 7 available"). No desktop icons, no dock, no Start menu. `app-workplace-workbench` (coding IDE) is available in the grid but not auto-launched at boot.
- **T+0:30 — Check email.** `Super+E` → `app-workplace-email` paints in active tile. Email is *not* a separate app reading IMAP; it is a view onto the attribute store (`MAIL:from`, `MAIL:subject`, `MAIL:thread`, `MAIL:read` indexed attributes). Inbox view is a live query. Search is instantaneous because the index *is* the filesystem.
- **T+1:00 — Open a proforma.** `Super+R` opens `app-workplace-proforma` into a second tile. Jennifer types the property address; launcher resolves to an existing proforma via attribute query (`PROFORMA:address ~= "Lot 7"`). No file picker, no Finder, no app-maintained "Recent files" list — the OS already maintains it as a query.
- **T+5:00 — Share the PDF via email.** `Super+Shift+s` → launcher paints the Send palette over the active tile (keyboard modal, not a drawer). Type `e` → `email`; auto-completes to two options (`compose new` / `reply to current thread`). `<Enter>` → launcher RPCs `app-workplace-email.compose(attachments=[inode-12345], prefill={...})`. The PDF is never copied — its content-addressed substrate handle is passed; the two apps are two views of the same object.
- **T+8:00 — Join a meeting.** Status bar's centre region has been showing the meeting countdown since login. At T-2 min the notification tile goes amber. `Super+J` joins it — `app-workplace-conference` opens; meeting context is the calendar entry's attributes including its WireGuard-resolved peer list. No browser tab. No "open in Zoom" prompt.

**Cross-app sharing: `Super+Shift+s` Send palette (per Agent 08).** Replaces both the iOS Share Sheet and the Android Intent chooser. One keystroke; typed verb palette; bounded chooser (the eight `app-workplace-*` apps are a closed set, not a third-party plugin point). Options listed by per-document frequency:

```
send → email (compose new)         [enter]
send → email (reply to thread X)   [r]
send → chat (jennifer-and-peter)   [c]
send → schedule (attach to event)  [s]
send → bim (link to model)         [b]
```

Selecting `email/compose` hands the file's content-addressed substrate handle (BFS inode + attributes) to `app-workplace-email`; the message is composed with the attachment line already filled; SMTP MIME-encodes from the BFS inode at send time. The file is *referenced* via its substrate identity, not "shared" in the iOS sense.

**Communication services as OS-native tiles.** Email, chat, calendar, and conferencing are not third-party apps over the OS — they are first-class `app-workplace-*` subsystems backed by the BFS attribute store. Each app is a view onto typed attributes (`MAIL:*`, `CHAT:*`, `CAL:*`, `MEET:*`); the cross-app substrate handle model (above) means a calendar event can carry a chat-room reference, a meeting can carry the PDF being discussed, and the notification tile aggregates all four into one always-visible surface (`Super+N` opens the unified queue pane).

**FOSS communication stack (cross-referencing WP-NET-08).** The four canonical components, each chosen for AArch64 viability, EUPL/AGPL-compatible licensing, and absence of vendor lock-in:

- **Stalwart** (mail server) — Rust-native SMTP/IMAP/JMAP server; AGPL-3.0; ships with built-in JMAP support that pairs naturally with the BFS attribute model.
- **Pimalaya** (email client library) — Rust crates (`himalaya`, `email-lib`); MIT/Apache-2.0; provides the substrate beneath `app-workplace-email` so the UI is a thin Tauri view over typed `MAIL:*` attributes.
- **matrix-rust-sdk** (chat) — Rust SDK for the Matrix protocol; Apache-2.0; gives `app-workplace-chat` end-to-end-encrypted federated chat without a hyperscaler dependency.
- **Galène** (conferencing) — Go-language SFU; MIT; lightweight WebRTC selective-forwarding unit suitable for an `app-workplace-conference` tile without pulling Chromium-grade media engines into the OS image.

These four components are the FOSS substrate that makes the "communication services as OS-native tiles" claim deliverable; they are not part of the v0.1 compositor scope (which is the niri fork) but are the v0.5–v1.0 communication track.

### 10.6 seL4 display architecture — two phases

**Phase 1 (v1): NetBSD bottom, Smithay-based compositor (niri fork), Linux AArch64 DRM/KMS.** Per Agent 05:

```
Tauri v2 app (WebKitGTK 4.1, GTK4 Wayland client)
  ↓ Wayland protocol (xdg-shell, xdg-decoration, linux-dmabuf-v1, wlr-layer-shell-v1)
os-workplace-compositor (niri fork, Smithay 0.7, Rust)
  ↓ libdrm / KMS atomic ioctls
NetBSD kernel: drm(4) + rkdrm/sunxidrm/tegradrm/AMDGPU
  ↓ Mesa + Panfrost (Mali) / Freedreno (Adreno)
AArch64 SoC: GPU + display controller + HDMI/DP PHY
```

This is the v1 ship path. Smithay is mature (Smithay 0.7.0, Jun 2025); WebKitGTK 4.1 speaks gdk-wayland natively; NetBSD has live drm(4) with rkdrm/sunxidrm/tegradrm for ARM SoCs and an active Wayland port (BSDCan 2025).

**Phase 2 (moonshot, post-v1): seL4 native display, LionsOS framebuffer pattern, Nitpicker capability model.** Per Agent 05's two-stage roadmap:

- **Stage 1 — LionsOS pattern (v1+1, ~1–2 PY):** NetBSD-in-CAmkES-VM owns DRM/KMS; seL4 holds device-frame capabilities and arbitrates which "bottom" is on screen via `wp_drm_lease_v1`. The compositor (same Smithay/niri-fork) runs *inside* the NetBSD VM; apps unchanged. This is exactly what LionsOS — UNSW Trustworthy Systems' reference seL4 OS — does today.
- **Stage 2 — Native seL4 compositor (moonshot, ~4–8 PY):** Nitpicker-modelled `nitpicker-rs` (Rust, `no_std`), a `wayland-shim` translation component, user-level `panfrost-rs` + `kms-driver-rs` on sDDF. **This is a genuine moonshot, not a near-term goal.** sDDF 0.6.0 has no graphics device class as of March 2025; LionsOS maintainers themselves use Linux framebuffer passthrough. If UNSW Trustworthy Systems with full institutional seL4 knowledge chooses passthrough, a 2-engineer team should not do better in v1.

**Doctrine claim #34 ("two bottoms") survives because the app does not know which bottom it is on.** The compositor surface is Wayland either way; what changes is whose kernel owns the GPU IRQ — a capability-distribution question, not an app question. The honest claim for v1 is "NetBSD compatibility bottom with niri-derived compositor"; the seL4 native bottom is announced and architected (§9.6 Phase 2) but not built.

### 10.7 SMB readiness and market reality check

**The findings the synthesis must internalise honestly:**

**Agent 07: Windows owns ~85–90% of construction/legal/engineering SMB endpoints.** The revenue-bearing software is Windows-anchored (Revit 63.5% US architecture firms; AutoCAD 73%; Bluebeam Revu; PCLaw; AbacusLaw; QuickBooks Desktop; Sage 50). A construction PM, an architect, a paralegal, and a bookkeeper at the same SMB will hit Day-1 blockers if forced through a Windows-only file-format path. **Zero published case studies** show non-developer SMBs running a tiling WM as their daily-driver desktop — in any sector. The closest analog (Regolith Linux) has no enterprise references.

**Agent 03: "coder-first, discipline-second" has a realistic ceiling of 20–35% of the cohort over 12 months.** 5–10% can be moved to pure tiling-default within 6 months; the remaining 65–80% will retreat to mouse-first habits within hours of being given the option. This is the AccountingWEB pattern: "accounting departments are notably conservative compared to other business groups, and even significant system migrations are considered radical and meet with resistance."

**Agent 07's "dual-mode default" recommendation.** Ship a stacking-mode day-1 default with `Super+T` flipping to tiling. Make tiling the *discoverable upgrade* within a default stacking layout. Operator/owner can lock the upgrade behind a per-user opt-in (the L1 policy layer in §9.3). This honours both the radical-2030 architectural ambition and the empirical onboarding evidence.

**The Gendarmerie is the only existence-proof at scale.** 103,164 workstations (97% of the French national gendarmerie's computing estate) on GendBuntu by June 2024 — and they run **GNOME** (Unity-derived during rollout era), not a tiling WM. Schleswig-Holstein's documented Linux desktop pilot covers only ~150 users (including the digital minister); the headline "30,000 PCs migrating to Linux" refers to LibreOffice on Windows. Italian MoD (LibreDifesa) is office-suite-only on Windows endpoints. Denmark (2025) is office-suite-only. There is no Canadian peer reference at all.

**Honest assessment:** `os-workplace` is establishing the first reference site in this category. There are no playbooks; the first 3–5 customers must be partners in the experiment, not buyers expecting a vetted product. The Schleswig-Holstein public-sector halo + the EU BIM mandate clock + the Gendarmerie 103k-seat existence proof are the credibility anchors; the dual-mode default + MSP-first channel + Windows-VM-for-Revit/AutoCAD escape hatches are the operational concessions that make the bet survivable.

### 10.8 What no hyperscaler is doing — the radical gap

**From Agent 08, each hyperscaler's structural blocker:**

- **Google (ChromeOS)** has bet the surface on the browser. Every ChromeOS improvement reinforces the browser-IS-the-app thesis. Going native would invalidate a decade of platform investment and admin tooling. Their economic model (Workspace subscriptions + ad-adjacent attention) requires the browser layer to mediate.
- **Apple (iOS/macOS)** ships the most polished "OS IS the app suite" precedent on iOS, but their distribution model (App Store 30% rake) and their hardware moat make a *standalone OS deployment for non-Apple AArch64 hardware* impossible. They will not surface a real terminal as a first-class tile — their power-user story is Xcode-on-macOS, not a sovereign business workstation.
- **Microsoft (Windows)** has bet on backward-compatibility + WSL + Teams. They cannot ship a clean-sheet workplace OS without cannibalising the Win32 ISV ecosystem they spent thirty years ratifying. Teams integration also lives at the SaaS layer, not the OS layer.
- **All three** depend on telemetry / cloud sign-in / app-store economy. None can offer perpetual-license sovereignty as a structural product feature without restructuring their P&L.

**PointSav's unmatchable position.** Small, vertically-integrated, SMB-focused, no app-store P&L to protect, no advertising business to defend, no enterprise ISV ecosystem to placate. The `app-workplace-*` apps are the entire surface — designed *as* the OS, not as guests. The radical claim, intended for v1+1 onward:

> `os-workplace` is planned to be the first AArch64-native, BFS-attribute-grounded, keyboard-capable business desktop with bundled email / chat / calendar / conference + proforma + pdf + bim + schedule as OS subsystems under one perpetual license, with no app store, no browser-as-shell, no cloud sign-in requirement, and a formal-verification substrate (seL4) at the moonshot horizon. No hyperscaler will match that combination because each element conflicts with the rest of their business model.

**BCSC posture (per §6 of `CLAUDE.md`):** the sovereignty + formal-verification claim is forward-looking; it MUST be presented in "planned / intended / may / target" language until delivered. The compositor is fork-niri-on-NetBSD today (v1); the seL4 native bottom is a moonshot (v1+1 stage 1, v3+ stage 2); the BFS attribute substrate is a v0.5 ratification decision; the eight bundled `app-workplace-*` apps ship on the per-app roadmap in §1–§8. Public communications use the radical-gap framing **above** for category positioning but anchor every shipped capability claim in the present.

### 10.9 Dissenting views

**Agent 03 (UX Researcher): terminal-first is not achievable for bookkeepers in 2026.** The cohort the os-workplace vision targets most explicitly — bookkeepers — is empirically the most software-conservative of the three personas. AccountingWEB and Starkman data anchor on operator comfort as the gating factor; the plain-text-accounting community's own SMB-adoption discussion confirms this. Realistic time-to-baseline on a pure tiling desktop for a bookkeeper is 6–12 weeks with material attrition risk over the first 2 weeks; in the same period a stacking-DE configuration with the sovereign substrate underneath reaches productivity in <2 weeks. The "coder-first, discipline-second" framing is a *retention* story (the 20–35% who graduate are deeply locked-in), not an *acquisition* story — the remaining 65–80% must be retained on a conventional stacking-DE configuration of the same OS, same Doorman, same sovereign substrate, same audit ledger, or the customer loses them.

**Agent 06/07 (Economist + SMB Survey): adopt Sway unmodified for v1, defer bespoke compositor to v2+.** The 30× cost gap between Sway-unmodified (~$85k, 0.5–1.5 EM) and the niri fork (~$300k, 3–6 EM) is dispositive for a 2-engineer pre-revenue team. The customer cannot distinguish path E (Sway unmodified) from path C (niri fork) from the seat of a 20-person Hamburg architecture firm — both render the same Revit-replacement and the same email client. Schleswig-Holstein's 30,000-seat success runs on KDE Plasma, not bespoke; the Gendarmerie 103,164 seats run GNOME. Every engineer-month on compositor work is a month not closing the application long-tail (the actual Munich-vs-Schleswig-Holstein lesson). The economist position: ship Sway in v1, capture the BIM-mandate + GDPR demand wave, earn the ARR, then fund the bespoke compositor on the back of customer revenue and a 4–6 person engineering team. The synthesis position (§9.2 niri fork) rejects this in favour of workplace-policy invariance and Rust-coherence, but the dissent should be tracked as a v0.1-slip fallback.

**Agent 07 (SMB Survey): a sovereign-substrate browser appliance is the right 2026 product, not Hyprland.** The stronger version of the SMB-survey dissent: the right product for the regulated-SMB market in 2026 is not `os-workplace` as specified — it is a Schleswig-Holstein-grade managed Linux + Firefox + Thunderbird + LibreOffice + Nextcloud that runs Procore/Clio/Xero/Buildertrend in the browser, with a hidden terminal/tiling power-mode for the owner-operator. That product is Gendarmerie + Chromebook, not Hyprland. The construction PM's workflow is already decomposing as "iPad in the field, browser in the office, Revit on a beefy workstation" (Rogers-O'Brien: 190 iPads, $1.8M/year saved) — `os-workplace` may be solving a problem that has already been routed around. The synthesis position (§9.7 dual-mode default + Windows-VM-for-Revit) concedes the dissent's accurate parts (heterogeneity is the killer; revenue-bearing software is Windows-anchored) while preserving the sovereignty thesis for the 70–80% of SMB workflow where Linux + LibreOffice + Firefox + sovereign substrate already wins.

**Agent 08 (Desktop Integration): curated AArch64 Linux distro for v1, defer BeOS-style data model to v2.** The BeOS distribution problem is unsolved at the OS layer: a sovereign OS with no third-party apps means *every* workflow a customer has today (QuickBooks, Sage, AutoCAD, Bluebeam, Revit, Procore, M365) has to be re-implemented inside the eight `app-workplace-*` apps. AArch64 native is a 5-year story, not a 2026 story (Apple's ARM transition took 2020–2025 with Rosetta as a bridge — PointSav cannot ship a Rosetta equivalent). The right v1 is a Linux distribution + curated app suite — ship `app-workplace-*` as native AArch64 binaries on a stripped-down Debian/Ubuntu base, iterate with real customers, take the BeOS-substrate bet for v2 once 1,000 paying SMBs have validated the eight apps. The synthesis position (§9.5 BFS attribute model as the v0.5 substrate decision, with Btrfs+xattrs+sqlite as the v0.1 pragmatic substitute) rejects this in favour of architectural decisions that cannot be retrofitted onto a Debian remix — but the dissent's counterclaim that the BeOS substrate is a v2+ investment (not v1) is folded into the §9.5 v0.5 deferral.

## 11. MBA Connectivity Topology Decision

> *Section numbering note:* the dispatch brief expected this WP-NET synthesis to land as §9 (after §8 Next Steps). Sections 9 (app-workplace-schedule — Product Scope) and 10 (os-workplace Desktop Architecture) were added to this BRIEF by other synthesis agents between dispatch and write, so this section is appended as §11 to preserve their content. The dispatch heading "MBA Connectivity Topology Decision" is retained verbatim.

This section synthesises 8 independent WP-NET research agents (security, systems-IPC, software-architecture, UX, economics, connectivity, OS-architecture, communication-services) on the question: **where does the MBA socket live, and what is the trust boundary between the station, the launcher, os-console, and the gateway?** The synthesis introduces a fourth topology (D) that emerged independently from three agents and was not in the original A/B/C question set; D is included in the scorecard and wins on the weighted score.

### 11.1 Agent consensus map

| Agent | Recommended topology | Confidence | Key reasoning (one sentence) |
|---|---|---|---|
| WP-NET-01 — Security/Sovereignty | **B** | High | seL4-shaped capability composition: launcher holds the tunnel cap, children receive a duplicated FD restricted to that one tunnel, `SO_PEERCRED` preserves per-app audit identity at the gateway, and Workplace stays independent of Console. |
| WP-NET-02 — Systems Designer (IPC) | **D** (C-transport + `system-mba` broker) | High | SCM_RIGHTS on Linux has no `revoke(2)`, so B leaks an unrevokable tunnel cap; bare C loses per-app identity; a small `system-mba` daemon with `SO_PEERCRED` + Ed25519 audit-header signing is the only design that preserves per-app identity *and* survives launcher/console lifecycle independently. |
| WP-NET-03 — Software Architect | **C** (with B-as-fallback acknowledged) | High | Crate-quality matters: `sendfd`/`tokio-send-fd` are stale, one-maintainer crates; kernel WireGuard is the most-audited primitive in scope; per-app Ed25519 signing already piggybacks on the SOFT- license-key infrastructure already ratified; IPC contract survives the moonshot only if "network is not in the contract." |
| WP-NET-04 — UX/Product Manager | **B** | High | Topology A reproduces Google-Play-Services-style "required-but-secondary" coupling that breaks SMB trust; C exposes WireGuard tunnel state as the user-visible failure surface (Jennifer cannot recover from `EHOSTUNREACH`); B preserves Console as a *valuable* surface rather than a *gating* one and supports the single-installer-two-checkboxes GTM. |
| WP-NET-05 — Economist | **C** | High | $19/seat/app perpetual pricing collapses if support burden scales linearly with seat count; B doubles pairings.yaml entries (100 vs 50 for a 50-seat firm) and doubles key-rotation events; C piggybacks on the WireGuard key the customer already rotates for PPN, marginal cost ≈ 0; Console-crash trust erosion under A carries a 20× lifetime-value multiplier. |
| WP-NET-06 — Connectivity Systems Designer | **D** (C-transport + selective P2P short-circuits) | High | Hub-and-spoke routing penalises Vancouver→europe-west4-a operators by 2–8× RTT; the Tailscale/Headscale model (control plane hub-and-spoke, data plane direct-P2P-when-NAT-allows, gateway-relayed otherwise) is the known-good pattern; MBA socket lifecycle must be decoupled from any user-process — that is a layering question, not a connectivity question. |
| WP-NET-07 — OS Architect | **D** | High | `system-mba` is the canonical Unix pattern (dbus-broker / polkitd / systemd-resolved / NetworkManager analogue); smallest TCB exposed to user-facing apps; cleanest failure-recovery path (socket activation + re-handshake-on-restart); cleanest moonshot absorption path (already an OS-level ABI on day 1). |
| WP-NET-08 — Email/Chat/Cal/Conf | **C** (persistent-connection services need user-systemd sync daemons independent of any UI) | Very high | IMAP IDLE, Matrix sliding-sync, and CalDAV polling must outlive any foregrounded UI process; only C lets `service-email-sync.service` / `service-chat-sync.service` / `service-calendar-sync.service` run as user-systemd units that survive launcher/console restarts; "Jennifer-without-Console" is the operator's stated UX bar and only C meets it without architectural caveats. |

**Vote tally:**
- Topology A — 0 votes
- Topology B — 2 votes (01, 04)
- Topology C — 3 votes (03, 05, 08)
- Topology D — 3 votes (02, 06, 07)

**Crucial observation:** Topology D *uses* Topology C as its transport layer; D = C + a small `system-mba` daemon that owns the WireGuard endpoint capability and signs per-app audit headers. Aggregating C-and-D votes as "kernel-WireGuard family" yields **6 of 8 agents** in that family; only Agents 01 and 04 favour a user-process-owned MBA socket (B), and both explicitly accept "B with hardened launcher converges on D" as a defensible reading of their position.

### 11.2 Weighted scorecard

Weights per dispatch instruction: Security/Sovereignty 30%, Systems Integration 25%, Operational Simplicity 20%, SMB Standalone Fit (Jennifer-without-Console) 15%, Economic 10%.

Per-topology scores are computed by averaging the matching dimensions across the relevant agents:
- Security/Sovereignty: Agent 01 combined column (50/50 sec+sov weighting from that agent)
- Systems Integration: Agents 02, 06, 07 (systems-IPC, connectivity, OS-architecture)
- Operational Simplicity: Agents 05, 07 (economist op-simplicity column, OS-architect boot-reliability)
- SMB Standalone Fit (Jennifer-without-Console): Agents 01, 04, 06, 07, 08 — binary pass/fail rendered on a 1–10 scale where pass = 9, partial-pass = 6, fail = 2
- Economic: Agent 05

| Topology | Security/Sov (30%) | Sys Integration (25%) | Op Simplicity (20%) | Jennifer-no-Console (15%) | Economic (10%) | Weighted Total |
|---|---|---|---|---|---|---|
| **A** — os-console owns MBA socket | 3.5 | mean(4, 4, 4) = 4.00 | mean(4, 3) = 3.5 | 2.0 (fail) | 5.0 | **3.55** |
| **B** — Launcher owns socket; SCM_RIGHTS to children | 8.5 | mean(6, 5, 6) = 5.67 | mean(3, 6) = 4.5 | 6.0 (partial: works only while launcher running) | 4.0 | **6.17** |
| **C** — WireGuard kernel = auth | 6.5 | mean(8, 8, 9) = 8.33 | mean(9, 8) = 8.5 | 9.0 (pass) | 8.0 | **7.88** |
| **D** — `system-mba` OS daemon (C-transport + broker) | 7.5 | mean(8, 9, 10) = 9.00 | mean(7.5, 9) = 8.25 | 9.0 (pass) | 7.5 | **8.25** |

**Arithmetic shown for each topology:**

- **Topology A:** 0.30 × 3.5 + 0.25 × 4.00 + 0.20 × 3.5 + 0.15 × 2.0 + 0.10 × 5.0
  = 1.050 + 1.000 + 0.700 + 0.300 + 0.500 = **3.550**

- **Topology B:** 0.30 × 8.5 + 0.25 × 5.67 + 0.20 × 4.5 + 0.15 × 6.0 + 0.10 × 4.0
  = 2.550 + 1.4175 + 0.900 + 0.900 + 0.400 = **6.1675** ≈ **6.17**

- **Topology C:** 0.30 × 6.5 + 0.25 × 8.33 + 0.20 × 8.5 + 0.15 × 9.0 + 0.10 × 8.0
  = 1.950 + 2.0825 + 1.700 + 1.350 + 0.800 = **7.8825** ≈ **7.88**

- **Topology D:** 0.30 × 7.5 + 0.25 × 9.00 + 0.20 × 8.25 + 0.15 × 9.0 + 0.10 × 7.5
  = 2.250 + 2.250 + 1.650 + 1.350 + 0.750 = **8.250**

**Ranking:** D (8.25) > C (7.88) > B (6.17) > A (3.55).

D wins by 0.37 over C and by 2.08 over B. The margin is meaningful: D's lead over C is concentrated in Systems Integration (+0.67) and Security/Sovereignty (+1.0) where the `system-mba` broker provides what bare C lacks — kernel-attested per-app audit identity via `SO_PEERCRED` on a local UDS, combined with the cryptokey-routing transport benefits.

### 11.3 Topology D evaluation

Topology D — `system-mba` as a dedicated OS-tier daemon owning the MBA socket — was nominated independently by Agents 02 (Systems Designer / IPC), 06 (Connectivity Systems Designer), and 07 (OS Architect). Each agent arrived at D from a different starting point:

- **Agent 02** reached D by reasoning forward from the IPC contract: SCM_RIGHTS on Linux has no `revoke(2)`, so B's "capability passed at spawn" claim collapses operationally; bare C loses per-app identity at the Doorman; the synthesis is C-transport plus a small UDS-broker daemon that uses `SO_PEERCRED` + Ed25519 audit-header signing.
- **Agent 06** reached D by reasoning from layering: a *transport* concern (which Totebox does this request go to) should not be owned by a *GUI* binary; binding MBA lifecycle to a UI process couples two layers that should be independent.
- **Agent 07** reached D by reasoning from canonical Unix patterns: `systemd-resolved`, `dbus-broker`, `polkitd`, `NetworkManager`, `system-auth-helper` — every comparable broker concern in Linux has converged on a dedicated daemon with socket-activated systemd lifecycle. `system-mba` is the OS architect's "least surprise" choice.

**Convergent properties that distinguish D from C:**

1. **Per-app audit identity preserved without app-level signing keys.** `system-mba` reads `SO_PEERCRED` on each local UDS connection, resolves the PID to a binary path via `/proc/<pid>/exe`, and signs an Ed25519 audit header `(station, app, user, timestamp, request-hash)` before forwarding over `wg0`. The Doorman validates against the station's public key. Apps never hold a signing key; the daemon does.
2. **MBA lifecycle decoupled from any UI.** systemd socket activation means the daemon starts on first `connect()` and survives launcher/console crashes. Apps reconnect on EPIPE; the new daemon instance re-derives authority deterministically. None of these failure modes touch the WireGuard kernel interface.
3. **Smallest TCB exposed to user-facing `app-workplace-*` processes.** Apps hold only a connected `AF_UNIX` endpoint to `/run/foundry/system-mba.sock`. They do not hold `CAP_NET_ADMIN`, `NETLINK_ROUTE`, the WireGuard control FD, or any networking capability beyond "talk to the local broker."
4. **Composable with Doorman audit-routing.** `system-mba` writes a local audit journal (`/var/log/foundry/system-mba/<YYYY-MM>.jsonl`) before each request hits the wire, providing an audit trail even when the data path bypasses the gateway via direct-P2P (the Tailscale/Headscale fallback Agent 06 proposes). The journal ships out-of-band to the gateway for ledger reconciliation.
5. **Moonshot absorption already complete.** The launcher's IPC contract (`workplace-launcher.sock`) covers app-lifecycle and UI concerns *only*; `system-mba`'s UDS at `/run/foundry/system-mba.sock` covers transport identity. The two contracts are versioned independently. When the launcher is absorbed into os-workplace as a system service, the MBA contract is unaffected — it is already an OS-tier ABI on day 1.

**Comparison to A/B/C on the load-bearing axes:**

| Axis | A | B | C | D |
|---|---|---|---|---|
| Per-app audit identity at gateway | Strong (broker stamps) | Weak (FD opaque post-pass) | Forgeable (no signer) | Strong (`SO_PEERCRED` + Ed25519) |
| Capability revocation | Trivial | Impossible on Linux (no `revoke(2)`) | Trivial (interface down) | Trivial (restart daemon) |
| Survives Console crash | No | n/a (Console not in path) | Yes | Yes |
| Survives Launcher crash | n/a | No (FDs orphaned) | Yes | Yes |
| WireGuard kernel substrate dependency | Userspace tunnel | Userspace tunnel | Kernel | Kernel |
| `pairings.yaml` entries per station | 1 | 2 | 1 | 1 |
| Per-station key rotations per year (50-seat firm) | 4/year | 8/year | 0 incremental | 0 incremental |
| seL4 capability mapping cleanliness | Anti-pattern (broker-as-ACL) | Best on paper, worst in practice on Linux | Clean | Cleanest (one daemon, one cap, IPC composition) |
| Moonshot absorption cost | Re-architecture | Promotion of ad-hoc protocol | n/a (nothing to absorb) | Already done (day-1 OS ABI) |

D is strictly better than C on per-app audit identity and seL4 capability mapping; strictly better than B on revocation, lifecycle decoupling, and ops cost; and dominates A on every axis.

### 11.4 Verdict

**Winning topology: D (`system-mba` OS-tier daemon, owning the MBA socket; both Console and Launcher consume it; transport is kernel WireGuard).**

**Consensus level: strong.** No agent named A. Two agents (01, 04) named B as primary; both acknowledge B converges on D once the launcher is hardened into a daemon-with-tray. Three agents (03, 05, 08) named C; D *is* C plus a small broker daemon, so this is not a contradiction — these agents prioritise the transport substrate (kernel WireGuard) which D retains intact. Three agents (02, 06, 07) named D directly.

**Single most decisive finding:** *the MBA socket lifecycle must be decoupled from any user-process lifecycle.* That single principle eliminates A (console-process-owned) and B (launcher-process-owned) on layering grounds, leaves C and D as the only candidates whose data plane survives console/launcher crash, and selects D over C because D preserves per-app audit identity (`SO_PEERCRED` + Ed25519) that C cannot provide without re-introducing a broker daemon under a different name.

### 11.5 The Jennifer-without-Console test

This was the dispositive scenario across all 8 agents. *Jennifer is using `app-workplace-memo` on her station. os-console is NOT running. Can she save a document to her Totebox?*

| Topology | Outcome | Why |
|---|---|---|
| **A** — os-console owns MBA socket | **FAIL** | Console holds the WireGuard tunnel; without console running, no process holds the tunnel capability. Memo's save fails with "gateway unreachable." Product-marketing failure: Workplace is sold as the user surface, Console as an admin/cartridge surface — coupling Workplace's availability to Console's process state is wrong. |
| **B** — Launcher owns MBA socket | **PASS (conditional)** | Launcher holds tunnel; Memo received its socket FD at spawn. Console state irrelevant. *But:* if Jennifer's launcher crashes or is restarted, all child apps lose their FDs and the test inverts to "Jennifer-without-Launcher" failure. |
| **C** — WireGuard kernel = auth | **PASS** | Tunnel established at boot by `wg-quick` (or `systemd-networkd`) before any app starts. Memo opens a socket to the gateway directly. Console and Launcher are both irrelevant. |
| **D** — `system-mba` OS daemon | **PASS** | `system-mba.service` is a systemd unit running whether or not any UI is up. Apps connect via local UDS; daemon signs and forwards. Identity preserved; Console irrelevant; Launcher irrelevant. |

**Deciding test:** This scenario is the single binary filter that eliminates Topology A from the option set. Any topology in which "Jennifer cannot read or send email when Console is not running" is a product-marketing failure that violates the operator's stated UX bar ("OS IS the app suite"). Topology B passes the literal Jennifer-without-Console test but fails its sibling test (Jennifer-without-Launcher) once SCM_RIGHTS-orphaned children are considered. Only C and D pass cleanly under all combinations of UI-process absence; D additionally preserves per-app audit identity at the gateway, which C does not.

### 11.6 Communication services conclusion

Per Agent 08, the recommended FOSS stack and its integration with the winning topology:

**Server-side (on Totebox VMs):**

| Service | Recommended FOSS | License | Notes |
|---|---|---|---|
| Email + JMAP + CalDAV + CardDAV + WebDAV + SMTP | **Stalwart Mail Server** (single Rust binary) | AGPLv3 | Replaces Postfix + Dovecot + Rspamd + Radicale; runs on $7/mo node; AArch64 native |
| Chat (Matrix homeserver) | **Synapse** or **Dendrite** or **Conduit** | Apache 2.0 | Conduit is Rust-native and runs lightest |
| Conferencing | **Galène** (Go, Pion WebRTC) | MIT | Runs on $7/mo node; admin API; WHIP ingress; AArch64 verified |
| Voice-only | **Mumble** (Murmur server) | BSD | Companion to conferencing for low-latency voice |

**Client-side (on os-workplace stations):**

| App | Backing library / pattern | License |
|---|---|---|
| `app-workplace-mail` | Pimalaya `email-lib` (Rust): IMAP + JMAP + Maildir + Notmuch + IDLE + OAuth + PGP | MIT |
| `app-workplace-chat` | `matrix-rust-sdk` (Rust): E2EE, sliding sync, threads, spaces | Apache 2.0 |
| `app-workplace-calendar` | Evolution Data Server over D-Bus (v1) → `minicaldav` / `caldav-rs` (v2) | GPL / MIT-Apache |
| `app-workplace-meet` | WebView with Galène JS client (v1) → `webrtc-rs` native (v2) | MIT |

**Per-app user-systemd sync daemons:**

```
~/.config/systemd/user/
├── service-email-sync.service       (himalaya/email-lib + IDLE)
├── service-chat-sync.service        (matrix-rust-sdk sliding sync)
├── service-calendar-sync.service    (vdirsyncer / EDS)
├── service-meet-presence.service    (Galène signalling keepalive)
└── service-comms-monitor.service    (aggregator; status to Console F3 + Workbench status bar)
```

**Connection under Topology D:**

Each sync daemon opens a local UDS to `/run/foundry/system-mba.sock`. `system-mba` authenticates the daemon via `SO_PEERCRED` (UID/PID/binary-path), signs the audit header, and forwards the request over `wg0` to the gateway. The gateway routes to the appropriate Totebox service (Stalwart for mail/calendar/contacts, Synapse/Dendrite/Conduit for Matrix, Galène for conferencing signalling). The data path for media (Galène WebRTC) flows over UDP through the same `wg0` tunnel — outside CSP scope (`media-src`/`webrtc` are separate from `connect-src`).

**The BeOS Mail Kit / Evolution Data Server / iOS MailCore precedent:** data lives on disk in standard formats (`~/Mail/` Maildir, `~/Calendar/` iCalendar vdir, `~/Contacts/` vCards, `~/.local/share/matrix/` sled DB). Multiple UIs can present it. Console F3 cartridges become *read-only summary lenses* over the same stores that the dedicated apps own. Replacing any app does not lose state. **This is what "OS IS the app suite" means in practice.**

### 11.7 IPC contract implications

Topology D introduces a **second OS-level IPC contract** distinct from the launcher's:

| Contract | Socket path | Owner | Scope |
|---|---|---|---|
| Launcher contract (`workplace-launcher.sock`) | `$XDG_RUNTIME_DIR/workplace-launcher.sock` | `app-workplace-launcher` (today) → `os-workplace` system service (post-moonshot) | App lifecycle, file picker, recent docs, focus, UI handoffs |
| MBA contract (`system-mba.sock`) | `/run/foundry/system-mba.sock` | `system-mba.service` (systemd unit; OS-tier; never user-tier) | Per-app audit identity, WireGuard egress, Doorman routing |

**Effect on the existing launcher CBOR contract from §6:**

The `MbaSocketFd { fd }` message in the minimum-viable message set (§6 "IPC model between `app-workplace-launcher` and child apps") is **removed**. The launcher no longer passes a WireGuard FD to children via `SCM_RIGHTS`. Instead, children open a UDS connection to `/run/foundry/system-mba.sock` themselves at startup. The launcher's IPC contract therefore shrinks and is *purely* about being-a-workplace-app — no networking primitives in scope. This is the contract shape Agent 03 called "the only choice that preserves the IPC contract as an OS-level ABI."

**Revised minimum-viable launcher message set:**

- `Hello { app_id, pid, version }` — child → launcher on startup
- `OpenDocument { path }` — launcher → child (file-open routing from shared picker)
- `RecentDocument { path, app_id }` — child → launcher (local-only telemetry)
- `Quit` — launcher → child (graceful shutdown)
- `Heartbeat` — both directions, for crash detection
- ~~`MbaSocketFd { fd }`~~ — **removed**; MBA is now a separate OS-tier contract owned by `system-mba`

**Minimum-viable `system-mba` UDS contract (new):**

- `OpenStream { target_service, optional_app_hint }` — app → daemon; daemon returns a connected stream
- `SignedAuditHeader` — daemon-internal; constructed per request via `SO_PEERCRED` + Ed25519, attached to outbound traffic
- `Health` — both directions; daemon reports tunnel state + last-handshake timestamp
- Wire format: length-prefixed CBOR with a `Version: u8` byte at frame start; protocol contract documented at `vendor/pointsav-monorepo/system-mba/docs/UDS-CONTRACT.md`

**Effect on the moonshot absorption path (§7 ¶ "Moonshot integration note"):**

Topology D *completes* the moonshot absorption for the MBA layer before the launcher absorption begins. `system-mba` is already an OS-tier daemon on day 1; nothing about it changes when the launcher is later absorbed into os-workplace. The two absorptions are independent:

1. **MBA absorption** — *complete on day 1.* `system-mba.service` is a system-tier systemd unit (`/run/foundry/system-mba.sock`, no user-tier surface). Co-versioned with the WireGuard configuration package.
2. **Launcher absorption** — *future moonshot milestone.* `app-workplace-launcher` ships as a user-tier Rust binary today; eventually moves into `os-workplace` as a system service with the same `workplace-launcher.sock` contract. The contract is the load-bearing artefact and is unaffected by the move. (`app-workplace-workbench` is a coding IDE and is not a candidate for absorption — see §7 moonshot note.)

This means the moonshot register adds one named entry (`moonshot-launcher-absorption`) and *removes* one that would otherwise have appeared (`moonshot-mba-extraction`) — D ships pre-absorbed.

### 11.8 Dissenting views

**Agent 01 (Security/Sovereignty) — argues for B over D.** The dissent: D's MBA-handshake layer is *additional* code on top of WireGuard's formally-verified protocol; every line of that broker is unverified relative to Tamarin/CryptoVerif proofs (Lipp/Blanchet/Bhargavan EuroS&P 2019). The launcher already exists as a trusted ~200-line supervisor with `SO_PEERCRED` discipline; adding `system-mba` doubles the OS-tier daemons that must be flawless. **Synthesis response:** the launcher cannot also be the broker without coupling network identity to UI lifecycle — the same coupling that disqualifies A. The launcher's restart cycle is a *normal user action* (operator restarts dock); network identity must not bounce when the operator restarts the dock. That single fact eliminates "B with hardened launcher" as a stable design. D's broker daemon adds ~500 lines of Rust to the trusted chain, which is auditable.

**Agent 04 (UX/Product Manager) — argues for B over D.** The dissent: D introduces a daemon Jennifer cannot see, cannot kill, and cannot reason about; her mental model collapses to "WireGuard is up or down" with no actionable lever. **Synthesis response:** Jennifer does not need to reason about `system-mba` — apps surface app-layer auth errors only, never socket paths or daemon state. The legibility surface for D is the `service-comms-monitor.service` aggregator (Agent 08) which presents a single status indicator in Console F3 and the Workbench status bar. The daemon is invisible by design, exactly as `systemd-resolved` and `pulseaudio` are invisible.

**Agent 03 (Software Architect) — argues for C without `system-mba`.** The dissent: per-app Ed25519 signing has a well-known scope-creep pattern (replay protection → nonce → per-user identity → key-derivation hierarchy) that silently expands the build cost; if signing drifts away from WireGuard kernel identity, C collapses into B's complexity profile without B's clean FD-passing abstraction. **Synthesis response:** D *centralises* the signing key in one daemon (`system-mba`) rather than distributing it per-app. Scope creep is bounded because there is one signer, one rotation policy, one audit-header schema. The dissent's concern is real if per-app keys live in each app — D explicitly avoids that.

**Agent 05 (Economist) — argues for C without `system-mba`, with A as a ship-now bridge.** The dissent: A is the lowest-build-cost defensible choice for v0.0.1; ship A, prove product-market fit on 5–10 customers, port to C in v0.2 when the team has 4 engineers. **Synthesis response:** the trust-erosion economics (20× LTV multiplier on each Console crash) and the Jennifer-without-Console product-marketing failure both compound from day one; migrating network topology under live customers is far more expensive than building it right at v0.0.1. The economist's own primary recommendation (C) is preserved in D — D adds ~3 engineer-weeks for the `system-mba` daemon on top of C's 6 engineer-weeks (Agent 05's estimate), for a total of ~9 engineer-weeks. This remains under A's 10 and B's 12.

**Agent 06 (Connectivity Systems Designer) — partial dissent on ship-cadence within D.** The dissent: *start with B with a hardened launcher, design the launcher's IPC contract as if `system-mba` already exists, leave room to extract it later* — this is how `systemd-resolved` happened (started inside NetworkManager, got extracted). **Synthesis response:** accepted as the implementation path for the first shipping milestone. The launcher's IPC contract (§11.7) already excludes `MbaSocketFd`; the placeholder for `system-mba` is the day-1 commitment; whether `system-mba` ships as a separate daemon on day 1 or is extracted from the launcher on day 30 is an engineering scheduling question, not an architectural one. The architectural commitment is "MBA is OS-tier, not user-tier, by the time we charge customers."
