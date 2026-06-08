---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workplace Desktop Suite
status: archived
archived: 2026-05-27
superseded_by:
  - BRIEF-workplace-desktop-environment.md
  - BRIEF-workplace-software-suite.md
created: 2026-05-27
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
---

# BRIEF — Workplace Desktop Suite

## Mission

`project-workplace` builds the native Tauri desktop surface for `os-workplace`.
Every `app-workplace-*` app targets macOS 10.13 High Sierra via Tauri v1.7
and connects to Foundry services running on localhost or over WireGuard PPN.
Binaries distribute through `project-software` (`binary-targets.yaml`,
platform `x86_64-apple-darwin`).

## App inventory

| App | Wave | Description |
|---|---|---|
| `app-workplace-workbench` | 1 — active | Tauri WebView shell loading the privategit workbench HTTP server at a configurable localhost port |
| `app-workplace-memo` | 1 — active | Sovereign offline document editor; Word/Pages muscle memory; self-contained HTML output |
| `app-workplace-presentation` | 1 — active | Sovereign offline presentation tool; PowerPoint/Keynote muscle memory |
| `app-workplace-proforma` | 2 — foundation only | Sovereign offline spreadsheet; Excel muscle memory; canonical JSON fiduciary record |
| `app-workplace-pdf` | 2 — foundation only | PDF viewer and print tool; pdfium-render crate (Apache 2.0 — Google PDFium) |
| `app-workplace-gis` | 2 — foundation only | Desktop GIS viewer; connects to gis.woodfinegroup.com or local tile server |
| `app-workplace-bim` | 3 — reserved | BIM editor; AutoCAD/Revit muscle memory; research phase per bim-product-family rules |

## macOS 10.13 build pipeline

Tauri v1.7 + tauri-build v1.5 are confirmed compatible with macOS 10.13 High Sierra.
Every `tauri.conf.json` declares `"macOS": { "minimumSystemVersion": "10.13" }`.

Build host: macOS (Intel or Apple Silicon). Not cross-compiled from Linux.
Cross-compile target: `x86_64-apple-darwin`.
Signing: Apple Developer certificate + notarization required for distribution.
Local development builds skip signing.

## Connectivity

All apps use a configurable endpoint (stored in Tauri app data `config.json`).
Default endpoint: `http://10.8.0.9` (WireGuard PPN VM address).
Local development: `http://127.0.0.1`.

Consumed services and default ports:
- Proofreader: `9097` (proofreader.pointsav.com in production)
- Doorman (SLM): `9092`
- MBA gateway / pairing-server: TBD (system-gateway-mba; port pending deployment)

## app-workplace-workbench

`app-workplace-workbench` is a Tauri WebView shell. It loads the privategit
workbench HTTP server (app-privategit-workbench, managed by project-development)
at a configurable port — default `3000`.

Architecture decision: no code is forked from the privategit workbench.
The HTTP server runs as an independent process. The Tauri app is a thin shell
that opens a window pointed at `http://127.0.0.1:<port>`.

Transition path: when the Tauri shell reaches stable, the privategit HTTP server
remains running independently — it is not removed or made optional. The two
apps serve complementary contexts (browser access vs. desktop native window).

CSP: `connect-src http://127.0.0.1:*` is intentionally broad for localhost.
When the production endpoint is known, narrow the CSP to that origin.

## app-workplace-pdf

Uses `pdfium-render` crate (Apache 2.0 — wraps Google PDFium via FFI).
PDFium is the same engine that powers Chrome's PDF viewer.
Static linking of the PDFium binary is required for macOS distribution.
Wave 2 scope: open PDF, navigate pages, print via OS print dialogue.

## Binary distribution (project-software)

Wave 1 entries to add to `binary-targets.yaml` in project-software:
- `app-workplace-memo` — platforms: `[x86_64-apple-darwin]`
- `app-workplace-presentation` — platforms: `[x86_64-apple-darwin]`
- `app-workplace-workbench` — platforms: `[x86_64-apple-darwin]`

Wave 2 entries (after foundation work matures):
- `app-workplace-proforma`, `app-workplace-pdf`, `app-workplace-gis`

## Wave 1 exit criteria

1. All three Wave 1 apps build clean on macOS 10.13 High Sierra
2. `app-workplace-workbench` opens and loads the privategit workbench in a native window
3. `app-workplace-memo` opens, creates a document, saves, and re-opens correctly
4. `app-workplace-presentation` opens and creates a presentation
5. Smoke test: connect each app to the WireGuard PPN endpoint; services respond

## Open questions

- WireGuard PPN vs configurable endpoint: resolved as configurable with PPN as default
- Pairing-server port for system-gateway-mba: TBD — check pairing-server deployment
- app-workplace-workbench default port: `3000` in foundation; confirm actual workbench port
