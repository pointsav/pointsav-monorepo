# Workplace✦Memo

> A sovereign, offline-first document editor for office workers.  
> Produces self-contained `.html` files that print to flawless PDFs.  
> No cloud. No account. No kill switch.

---

## What it is

Workplace✦Memo is a desktop document editor built with the muscle memory of
Word, Pages, and Google Docs — same canvas, same toolbar, same
page-on-a-grey-desktop feel — rebuilt from the ground up around one principle:
**your document is a file on your machine and belongs entirely to you.**

The output is a self-contained `.html` file with all fonts embedded as base64.
It opens in any browser on any device, now or in twenty years. It prints to a
flawless PDF via the OS print dialogue. It never touches a server.

This is not a web app. It is a native desktop application that happens to use
HTML as its document format.

---

## What it is not

It is not a coding tool.  
It is not a CMS.  
It is not a web page builder.  
It is not a cloud collaboration platform.

It is a document editor for people who write memos, briefs, reports, proposals,
and specs — and who want those documents to be beautiful, permanent, and
completely theirs.

---

## Key features

- **Word-like canvas** — white page at true paper dimensions (A4/Letter),
  centred on a warm grey desktop, realistic page shadow, visible page breaks
- **Live pagination** — Paged.js renders real page breaks as you type; what
  you see on screen is what prints
- **Full toolbar** — font family, size, bold, italic, underline, strikethrough,
  colour, highlight, alignment, lists, indent, line spacing
- **Ruler** — live margin handles, draggable, actual mm or inch values
- **Embedded fonts** — 8 curated OFL font families embedded at build time; a
  Fonts panel allows downloading and embedding additional families
- **Templates** — named CSS + page geometry bundles; switching reflows instantly
- **HTML export** — self-contained file, all fonts base64-encoded, no external
  dependencies
- **Print / PDF** — native OS print dialogue; `@media print` CSS mirrors the
  canvas geometry exactly; what you see is what prints
- **Track changes** *(Phase 3)* — `<ins>`/`<del>` HTML with author +
  timestamp; Accept/Reject toolbar
- **Zero network calls** — no outbound connections after install

---

## Sovereignty position

Workplace✦Memo is designed to meet the requirements of European government
deployments — specifically the profile established by France's DINUM (La Suite
Numérique, April 2026 Linux directive) and Germany's ZenDiS (openDesk). It fills
the gap those platforms acknowledge: a **local-first, offline, single-user
desktop document editor** producing a permanent, open-format file.

| Concern | Answer |
|---|---|
| Licence | EUPL v1.2 — European Commission copyleft, EU jurisdiction |
| Framework | Tauri v2 — MIT/Apache 2.0, Commons Conservancy (Netherlands) |
| Language | Rust — MIT/Apache 2.0 |
| Pagination | Paged.js — MIT |
| Fonts | SIL Open Font Licence (all embedded families) |
| Primary target OS | Linux (sovereign target) |
| Secondary OS | macOS, Windows (proprietary WebView — accepted) |
| Code hosting | pointsav-monorepo (GitHub mirror); self-hosted Forgejo planned |

On Linux, every layer of the stack except the application code itself is either
LGPL or permissively-licensed open-source infrastructure that can be fully
forked and rewritten independently.

---

## Repository location

```
pointsav-monorepo/
└── app-workplace-memo/        ← this folder
```

Part of the PointSav Digital Systems monorepo. Mirrored to GitHub.

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
app-workplace-memo/
├── README.md
├── DEVELOPMENT.md
├── ARCHITECTURE.md
├── LICENCE                    EUPL v1.2
├── CHANGELOG.md
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── src/                       Frontend — TypeScript, no framework
│   ├── main.ts
│   ├── editor/
│   │   ├── canvas.ts          Contenteditable document canvas
│   │   ├── toolbar.ts         Formatting controls
│   │   ├── ruler.ts           Margin ruler with drag handles
│   │   └── pagination.ts      Paged.js integration
│   ├── core/
│   │   ├── document.ts        Document state and serialisation
│   │   ├── templates.ts       Template definitions (CSS + geometry)
│   │   └── fonts.ts           Font management and base64 embedding
│   ├── ipc/
│   │   └── bridge.ts          Tauri IPC — open / save / export / print
│   └── styles/
│       ├── app.css            Application chrome
│       └── canvas.css         Document canvas and page styles
├── src-tauri/                 Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json       Minimal permission surface
│   ├── fonts/                 Embedded OFL font binaries
│   └── src/
│       ├── main.rs
│       └── commands/
│           ├── mod.rs
│           ├── file_ops.rs    open / save / export HTML
│           └── print.rs       print / silent PDF
└── docs/
    ├── fonts.md               Font licences and sources
    ├── templates.md           Template format spec
    └── print-pipeline.md      Print/PDF technical reference
```

---

## Licence

Copyright © 2026 PointSav Digital Systems

Licensed under the European Union Public Licence v1.2 (EUPL-1.2).  
See [LICENCE](./LICENCE) for the full text in English.

The EUPL is a copyleft licence designed by the European Commission, available
in all 23 EU official languages, recognised under EU law. Disputes are subject
to the law of the EU member state where the licensor is based.
