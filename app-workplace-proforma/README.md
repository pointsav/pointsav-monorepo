# Workplace✦Proforma

> A sovereign, offline-first spreadsheet for institutional analysis.
> Produces self-contained `.json` files with full formula fidelity.
> No cloud. No account. No kill switch.

---

## What it is

Workplace✦Proforma is a desktop spreadsheet built with the muscle memory of
Excel, Numbers, and Google Sheets — same menu bar, same formula bar, same
cell-grid-with-sheet-tabs feel — rebuilt from the ground up around one
principle: **your proforma is a file on your machine and belongs entirely to
you.**

The output is a self-contained `.json` file with all formulas, formatting,
metadata, and audit chain embedded. It opens in any JSON-aware tool on any
device, now or in twenty years. It prints to a flawless PDF via the OS print
dialogue. It never touches a server.

This is not a web app. It is a native desktop application that happens to use
JSON as its document format.

---

## What it is not

It is not a business intelligence platform.
It is not a database front-end.
It is not a cloud collaboration tool.
It is not a general-purpose scripting engine.

It is a spreadsheet for people who build proformas, underwrites, sensitivity
analyses, and institutional financial models — and who want those models to
be permanent, auditable, and completely theirs.

---

## Key features

- **Excel-shaped chrome** — the menu bar, formula bar, grid, and sheet tabs
  every spreadsheet user already knows. No relearning required.
- **Full keyboard parity** — arrow keys navigate cells, F2 edits, Enter
  commits and moves down, Tab commits and moves right, Ctrl+S saves. The
  muscle memory of decades of Excel use is preserved exactly.
- **Sovereign formula engine** — a pure JavaScript formula engine covering
  arithmetic, cell references, ranges, and the common functions institutional
  users rely on: SUM, AVERAGE, MIN, MAX, COUNT, IF, ROUND, ABS, PMT, PV, FV,
  NPV, IRR.
- **Canonical JSON format** — one file per proforma. Schema-versioned.
  Backward-compatible forever. Fully self-describing — any LLM or human
  reader understands the model without a parser.
- **Cryptographic audit chain** — every save computes a SHA-256 digest of
  the canonical document and embeds it in the file. The chain makes
  tampering mathematically detectable.
- **Print / PDF** — native OS print dialogue; `@media print` CSS mirrors the
  grid geometry exactly; what you see is what prints.
- **Zero network calls** — no outbound connections after install.

---

## Sovereignty position

Workplace✦Proforma is designed to meet the requirements of European
government deployments — specifically the profile established by France's
DINUM (La Suite Numérique, April 2026 Linux directive) and Germany's ZenDiS
(openDesk). It fills the gap those platforms acknowledge: a **local-first,
offline, single-user desktop spreadsheet** producing a permanent, open-format
file.

| Concern | Answer |
|---|---|
| Licence | EUPL v1.2 — European Commission copyleft, EU jurisdiction |
| Framework | Tauri v2 — MIT/Apache 2.0, Commons Conservancy (Netherlands) |
| Language | Rust — MIT/Apache 2.0 |
| Formula engine (Phase 1) | Pure JavaScript, embedded, EUPL-1.2 |
| Formula engine (Phase 2) | IronCalc — Apache 2.0, pure Rust, NLnet-funded |
| Primary target OS | Linux (sovereign target) |
| Secondary OS | macOS, Windows (proprietary WebView — accepted) |
| Code hosting | pointsav-monorepo (Forgejo primary; GitHub mirror) |

On Linux, every layer of the stack except the application code itself is
either LGPL or permissively-licensed open-source infrastructure that can be
fully forked and rewritten independently.

---

## Relationship to Workplace✦Memo

Workplace✦Memo and Workplace✦Proforma are **two standalone applications**
that share a philosophical and architectural lineage — the same licensing
posture, the same Tauri + Rust sovereignty stack, the same security
discipline, the same file-is-the-product commitment — but **not a shared UI
codebase.**

Each application evolves independently toward category-leading UX for its
own users. Memo preserves Word muscle memory; Proforma preserves Excel
muscle memory. The neutral starting point gives users from the Microsoft
universe zero friction on adoption. Beyond that, each application is free
to diverge in whatever direction best serves its category.

There is no `workplace-shared/` chassis. Each application vendors its own
dependencies, each ships its own binary, each has its own roadmap.

---

## Repository location

```
pointsav-monorepo/
└── app-workplace-proforma/      ← this folder
```

Part of the PointSav Digital Systems monorepo. Canonical location is
self-hosted Forgejo on European infrastructure; mirrored to GitHub for
convenience.

---

## Quick start

Full instructions: [DEVELOPMENT.md](./DEVELOPMENT.md)

```bash
npm install
npm run tauri dev      # development
npm run tauri build    # production binary
```

Prerequisites: Rust 1.78+, Node.js 20+, platform WebView (see DEVELOPMENT.md)

---

## Project structure

```
app-workplace-proforma/
├── README.md
├── DEVELOPMENT.md
├── ARCHITECTURE.md
├── LICENCE                    EUPL v1.2
├── CHANGELOG.md
├── Makefile
├── package.json
├── src/                       Frontend — vanilla JS, no framework
│   ├── index.html
│   ├── styles/
│   │   └── app.css            Application chrome + grid styles
│   └── js/
│       ├── schema.js          JSON schema defaults and validation
│       ├── engine.js          MVP formula engine
│       ├── grid.js            Grid rendering, cell selection, editing
│       ├── formula-bar.js     Formula bar wiring
│       ├── toolbar.js         Formatting and toolbar actions
│       ├── export.js          Print/PDF path and XLSX stub
│       └── app.js             State, file I/O, menus, shortcuts
├── src-tauri/                 Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/
│       └── main.rs            Minimal IPC surface (three commands)
└── docs/
    ├── schema.md              JSON schema specification
    ├── engine.md              Formula engine reference
    └── print-pipeline.md      Print/PDF technical reference
```

---

## The file is the product

Every proforma is one `.json` file. The file carries the complete model:
formulas in Excel-compatible syntax, formatting metadata, named ranges,
conditional highlighting rules, anchor identifiers, and a SHA-256 checksum.
Two people opening the same file on different machines see exactly the same
thing.

Three extensions, each with one role:

| Extension | Role |
|---|---|
| `.json` | Canonical file. The fiduciary record. Permanent. |
| `.pdf` | Print exhaust. Generated on demand for distribution. Cryptographically sealed. |
| `.xlsx` | Legacy exhaust. Generated on demand for recipients whose systems require Excel. Never authored directly. |

No CSV. No hidden sidecar files. No application state outside the `.json`
itself. See [`docs/schema.md`](./docs/schema.md) for the full specification.

---

## Licence

Copyright © 2026 PointSav Digital Systems

Licensed under the European Union Public Licence v1.2 (EUPL-1.2).
See [LICENCE](./LICENCE) for the full text in English.

The EUPL is a copyleft licence designed by the European Commission,
available in all 23 EU official languages, recognised under EU law.
Disputes are subject to the law of the EU member state where the licensor
is based.
