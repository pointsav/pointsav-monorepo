# Workplace✦Presentation

A sovereign, offline-first desktop presentation editor. Part of the PointSav workplace family alongside Workplace✦Memo and Workplace✦Proforma.

*Un editor de presentaciones de escritorio soberano y offline. Parte de la familia PointSav Workplace junto a Workplace✦Memo y Workplace✦Proforma.*

---

## What it is — in one line

PowerPoint, replaced. The file is a single `.html` document, yours forever, readable in fifty years by any browser on any computer.

*PowerPoint, reemplazado. El archivo es un único documento `.html`, tuyo para siempre, legible en cincuenta años por cualquier navegador en cualquier ordenador.*

---

## The file is the product

Every presentation you save is a single `.html` file. That file contains everything:

- The slides themselves as plain HTML sections
- All fonts embedded as base64 inside the CSS
- A small JavaScript runtime that makes the file runnable as a slideshow in any browser
- A SHA-256 cryptographic seal that lets anyone verify the file has not been tampered with
- Metadata in a single `<meta>` tag — no proprietary wrapper

Open the file in Firefox. Arrow keys navigate. F goes fullscreen. Escape exits. No application required. No account. No internet connection. No vendor relationship.

*Cada presentación que guardas es un único archivo `.html`. Ese archivo contiene todo: las diapositivas como secciones HTML planas, las fuentes incrustadas en base64 dentro del CSS, un pequeño runtime de JavaScript que permite reproducirlo como presentación en cualquier navegador, un sello criptográfico SHA-256 que permite verificar que el archivo no ha sido alterado, y metadatos en una única etiqueta `<meta>` — sin envoltorio propietario.*

---

## Why this exists

Modern office suites keep your presentations inside software you do not own. The vendor owns the format. The vendor owns the access. If the vendor raises prices, deprecates an API, or goes out of business, the file you created last year may stop opening.

Workplace✦Presentation rejects that arrangement. Your file is HTML. Your fonts are Open Font Licence. The framework is EU-governed (Tauri, Netherlands). The licence is the European Commission's own (EUPL-1.2). On Linux, every layer is open source and forkable.

*Las suites ofimáticas modernas mantienen tus presentaciones dentro de software que no posees. El proveedor controla el formato. El proveedor controla el acceso. Si el proveedor sube precios, deprecia una API, o desaparece, el archivo que creaste el año pasado puede dejar de abrirse.*

*Workplace✦Presentation rechaza ese arreglo. Tu archivo es HTML. Tus fuentes son de Open Font Licence. El framework está gobernado en la UE (Tauri, Países Bajos). La licencia es la propia de la Comisión Europea (EUPL-1.2). En Linux, cada capa es código abierto y puede ser bifurcada.*

---

## Design principles

| | English | Español |
|---|---|---|
| 1 | The file is the product. | El archivo es el producto. |
| 2 | No account, no cloud, no kill switch. | Sin cuenta, sin nube, sin interruptor remoto. |
| 3 | Familiar to a PowerPoint user within 30 seconds. | Familiar para un usuario de PowerPoint en 30 segundos. |
| 4 | Every byte in a saved file can be inspected and understood. | Cada byte en un archivo guardado puede ser inspeccionado y comprendido. |
| 5 | The split-screen code view lets you see your file as it really is. | La vista de código en pantalla dividida te permite ver tu archivo tal como realmente es. |

---

## Stack

Rust 1.95 · Tauri 1.7 (Netherlands) · Paged.js · SIL Open Font Licence families · EUPL-1.2.

No network calls. No telemetry. No auto-updater reaching external servers.

*Sin llamadas de red. Sin telemetría. Sin actualizador automático que contacte servidores externos.*

---

## Status

Active development. See `ROADMAP.md` for the seven-phase plan. See `NEXT.md` for what's happening right now. See `CLEANUP_LOG.md` for known deferred work.

*Desarrollo activo. Consulta `ROADMAP.md` para el plan de siete fases. Consulta `NEXT.md` para saber qué está ocurriendo ahora. Consulta `CLEANUP_LOG.md` para el trabajo diferido conocido.*

---

## Sibling apps — the workplace family

- **Workplace✦Memo** — document editor. Output: single-file `.html`.
- **Workplace✦Proforma** — spreadsheet editor. Output: `.json` with cryptographic audit chain.
- **Workplace✦Presentation** — this app.

All three share chrome tokens, IPC patterns, and EUPL-1.2 licensing. UX evolves independently per app.

*Las tres aplicaciones comparten tokens de interfaz, patrones IPC y licencia EUPL-1.2. La UX evoluciona de forma independiente por aplicación.*

---

## Licence

EUPL v1.2 — European Union Public Licence. Full text in `LICENCE`.

The European Commission's own open-source licence. GPL-compatible, explicitly EU-jurisdiction.

*Licencia EUPL v1.2 — Licencia Pública de la Unión Europea. Texto completo en `LICENCE`. La licencia de código abierto de la propia Comisión Europea. Compatible con GPL, jurisdicción UE explícita.*
