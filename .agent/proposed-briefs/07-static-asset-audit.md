# Brief: static/ housekeeping audit — app-mediakit-knowledge

**target**: Read-only scan of `app-mediakit-knowledge/static/` for unused CSS classes, unreferenced JS files, dead font references, and stale vendored assets — pre-Phase-4 housekeeping.
**target_files** (read-only):
- `app-mediakit-knowledge/static/style.css` (625 lines)
- `app-mediakit-knowledge/static/wiki.js` (116 lines)
- `app-mediakit-knowledge/static/saa-init.js` (415 lines)
- `app-mediakit-knowledge/static/vendor/{cm-saa,cm-collab}.bundle.js` (601 KB + 303 KB)
- `app-mediakit-knowledge/src/{server,edit,render,assets}.rs` (template + class sources)
- `app-mediakit-knowledge/vendor-js/{package,package-lock}.json`
- `app-mediakit-knowledge/vendor-js/node_modules/{package}/package.json`
**expected_output**: Single markdown report with four sections (A/B/C/D) returning findings only; no writes; cap 80 lines
**max_response_lines**: 80
**model_tier**: haiku — mechanical pattern-match against bounded directory; no architectural judgment
**parallelisable**: yes (read-only; all 4 audit tasks are independent reads)
**confidence_gate_passes**: ≥90% — pattern-matching across bounded static/ + known src/ corpus
**layer_scope**: task
**anti_slop_check**: dead-asset findings become NEXT.md line items for the Task to close before Phase 4 begins; lockfile-vs-installed mismatches flag bundle-rebuild risk
**dependencies**: none

## Specification

Templates are generated at compile time via `maud` macros in `src/server.rs` and `src/edit.rs` — there is no separate `.html` template directory. Static assets are embedded via `rust-embed` from `static/` (`src/assets.rs` declares `#[folder = "static/"]`). No font files are present; fonts are system-stack only.

### Task A — Unused CSS classes

CSS defines 52 selectors. Cross-reference each against `src/*.rs` and `static/*.js` for usage. Flag UNUSED with line-number-in-style.css for any class with zero references. Specific candidates pre-flagged:
- `wikilink` / `wikilink-redlink` — confirm comrak's `wikilinks_title_after_pipe = true` emits `class="wikilink"` on generated anchors, or flag if the styled HTML is never produced
- `toc-level-{2..6}` — server.rs emits `"toc-level-" (level)` dynamically; confirm levels 2–6 reachable; `toc-level-1` missing (likely intentional)
- `toc-collapsed` — wiki.js applies dynamically via classList; confirm matching `#wiki-toc` id
- `site-*` classes — verify each emitted in maud templates (may predate Phase 1.1 rename)
- `fli-notice`, `ivc-band-text`, `lede`, `md` — low-frequency; verify each appears in at least one maud template or is JS-applied

### Task B — Unreferenced JS files

Three first-party JS files; verify each is referenced:
- `wiki.js` — `script src="/static/wiki.js"` in server.rs ~line 500
- `saa-init.js` — `script src="/static/saa-init.js"` in edit.rs ~line 131
- `vendor/cm-saa.bundle.js` — `script src="/static/vendor/cm-saa.bundle.js"` in edit.rs ~line 130
- `vendor/cm-collab.bundle.js` — loaded dynamically in saa-init.js ~line 282 (`s.src = '/static/vendor/cm-collab.bundle.js'`), guarded by `window.WIKI_COLLAB_ENABLED` (intentional default-off)

Flag any JS file in `static/` with no reference in src/ or other static files.

### Task C — Dead font references

Confirm:
1. No `@font-face` rules in style.css
2. No `url()` references in style.css
3. No `<link rel="preload" as="font">` in server.rs or edit.rs

If any such reference exists, flag DEAD FONT REF with file + line.

### Task D — Vendored asset staleness

`vendor-js/package.json` declares versions; `node_modules/` contains installed; `static/vendor/` contains pre-built bundles. Pre-computed declared-vs-installed table:

| Package | Declared | Installed |
|---|---|---|
| @codemirror/view | ^6.36.0 | 6.41.1 |
| @codemirror/autocomplete | ^6.18.0 | 6.20.1 |
| @codemirror/commands | ^6.8.0 | 6.10.3 |
| @codemirror/lang-markdown | ^6.3.0 | 6.5.0 |
| yjs | ^13.6.0 | 13.6.30 |
| y-codemirror.next | ^0.3.0 | 0.3.5 |
| y-websocket | ^2.0.0 | 2.1.0 |
| esbuild | ^0.25.0 | 0.25.12 |

All installed satisfy declared semver `^` ranges. Bundles were built 2026-04-27.

Confirm: do `node_modules/` versions match the bundle build date (`npm install` before `node build.mjs` same day)? Check `package-lock.json` pins vs installed — if pin differs from installed, committed bundles may not match a reproducible rebuild from lockfile. Flag STALE BUNDLE if so.

## Output shape

```
## A — CSS classes
[table: class | status (USED / UNUSED / PARTIAL) | evidence or line ref]

## B — JS file references
[table: file | referenced-from | status (REFERENCED / UNREFERENCED)]

## C — Font references
[one line per finding, or "No dead font references found."]

## D — Vendored asset staleness
[table: bundle | built | lockfile-pin vs installed | CURRENT / STALE]
```

Cap 80 lines total. No prose padding.

## Acceptance criteria

- All four sections (A/B/C/D) present
- CSS classes exhaustively cross-referenced (52 selectors)
- All 4 first-party JS files audited
- Font references confirmed absent (or flagged if found)
- Vendored bundle staleness reported with pin-vs-installed comparison
- Output ≤80 lines

## Risks / unknowns

- Some CSS classes may be applied via wiki.js's `classList.add/remove` rather than templated; sub-agent should grep both sources before flagging UNUSED
- comrak's anchor-generation behaviour for wikilinks may differ between versions; sub-agent should grep render.rs for the actual class application rather than assume
- Lockfile may be present-but-empty or not present; sub-agent should report "no lockfile" as WARN not FAIL if so
