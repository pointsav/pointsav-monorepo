# RESEARCH — moonshot-code-editor

**Status:** Research phase. No replacement code written.
**Registered:** 2026-06-14
**Priority:** LOW
**"We Own It" target:** Replaces CodeMirror 6 (vendored JS, ~100KB, MIT)

---

## Dependency replaced

### CodeMirror 6
- **Version borrowed:** 6.x (vendored bundle, pinned)
- **Licence:** MIT
- **SLOC:** ~120,000 lines TypeScript (core + extensions + language packages)
- **Bundle size:** ~100KB minified (JSON mode + lint + basic setup)
- **What it does:** In-browser code editor used as the "Code" pane in the dual-mode
  DTCG editor in `app-privategit-bim`. Provides: JSON syntax highlighting, bracket
  matching, line numbers, gutter error markers (linting), keyboard shortcuts.
- **Why borrowed:** CodeMirror 6 has 11 years of browser compatibility work, accessibility
  (WCAG 2.1 AA keyboard navigation), and mobile support baked in. Replacing it from
  scratch is a large investment with minimal return while the platform is maturing.
- **Replacement complexity:** HIGH for full CodeMirror parity. LOW for the narrow
  PBS-1 editing subset.

---

## What PBS-1 editing actually needs (scoped replacement)

When auditing what features of CodeMirror 6 the DTCG editor uses:

| Feature | Used? | Why |
|---|---|---|
| JSON syntax highlighting | Yes | Visual DTCG editing |
| Bracket/brace matching | Yes | JSON structure navigation |
| Line numbers | Yes | Error location reference |
| Lint gutter markers | Yes | moonshot-schema-validator errors |
| Debounced onChange | Yes | Bidirectional SchemaState sync |
| Basic keyboard shortcuts | Yes | Ctrl+Z undo, Ctrl+A select-all |
| Vim/Emacs bindings | No | Not needed for BIM editing |
| Multi-cursor | No | PBS-1 editing is single-focus |
| Autocomplete | Possible | bSDD property name completion (Phase 2) |
| Diff view | No | Git-level; not in editor |
| Fold regions | Possible | Collapsing large IFC Psets |

The viable internal replacement is a purpose-built `<pbs1-editor>` web component
implementing only the "Used? Yes" rows above. ~2,000–3,000 lines of vanilla JS.

---

## Prior art surveyed

| Library | Licence | SLOC | Notes |
|---|---|---|---|
| CodeMirror 6 | MIT | ~120k TS | Current borrow; industry standard |
| Monaco Editor (VSCode) | MIT | ~700k TS | Much larger; not appropriate for web component embed |
| Ace Editor | BSD | ~70k JS | Older architecture; less accessible |
| `<textarea>` + highlight.js | Various | — | Zero-JS fallback; viable for v1 if CodeMirror adds complexity |
| hand-rolled tokeniser | Own | ~2–3k JS | The target for moonshot-code-editor |

## Approach when the time comes

1. Audit the exact CodeMirror API surface used in `app-privategit-bim/src/assets/bim.js`
2. Write a minimal tokeniser for DTCG/PBS-1 JSON (not full JSON — just the keywords
   that appear in `$type: "bim.entity"` schemas)
3. Wrap in a `<pbs1-editor>` custom element with the same external API as the
   CodeMirror integration (`.setValue()`, `.getValue()`, `onChange` callback,
   `.setDiagnostics()` for lint)
4. Replace the CodeMirror import in bim.js with the new component — no other changes

The external API contract is the leverage point: keep it stable and the swap is trivial.

---

## Related

- `app-privategit-bim/src/assets/codemirror.bundle.js` — current borrower
- `app-privategit-bim/src/assets/bim.js` — integration point (onChange, setDiagnostics)
- `moonshot-schema-validator` — provides lint diagnostics to the editor (prerequisite)
- BRIEF-bim-objects-system.md §E — dependency sovereignty map
