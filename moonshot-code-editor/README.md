<div align="center">

# moonshot-code-editor

[ Leer en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Moonshot Initiative — `moonshot-*` family
**Version:** 0.1.0
**Status:** Reserved-folder — research phase
**Cluster:** `cluster/project-bim` per workspace `PROJECT-CLONES.md`
**Priority:** LOW

---

## What this replaces

This stub tracks the long-horizon intention to replace **CodeMirror 6**
(vendored JS, ~100KB minified, MIT licence) with an internal code editing
component purpose-built for the DTCG/PBS-1 schema editing workflow.

CodeMirror 6 is used in `app-privategit-bim` as the "Code" pane of the
dual-mode DTCG editor (Visual / Code toggle). It provides JSON syntax
highlighting, bracket matching, and linting display.

## Why LOW priority

CodeMirror 6 is a Mozilla-foundation-scale project (~120,000 lines of
TypeScript, 11 years of development). The gap between what we need
(JSON editing with PBS-1 linting) and what an internal replacement must
provide is large. Registering the stub satisfies the "We Own It" principle
while being honest about the horizon.

A purpose-built "DTCG schema editor" web component would be ~2,000 lines
of vanilla JS. It would not have the breadth of CodeMirror but would be
lighter, owned, and tailored to PBS-1 vocabulary.

## Architecture (long-term vision)

```
moonshot-code-editor (web component)
  ├── <pbs1-editor> — custom element, vanilla JS + CSS
  │    ├── Syntax highlighting for DTCG/PBS-1 JSON (hand-rolled tokeniser)
  │    ├── Lint API: receives errors from moonshot-schema-validator WASM
  │    └── Gutter markers, bracket matching, line numbers
  └── Compiled: single JS file, no build tooling dependency
```

No React, no build pipeline, no npm. A `<script type="module">` import and
a `<pbs1-editor>` element — same deployment model as Carbon Web Components.

## Timeline

**Long horizon (2029+).** Do not begin implementation until:
1. `app-privategit-bim` v1 ships with CodeMirror 6
2. `moonshot-schema-validator` ships (WASM lint integration is the prerequisite)
3. The specific CodeMirror features actually used are audited — the replacement
   scope is bounded by what PBS-1 editing requires, not full CodeMirror coverage

## Cross-references

- `app-privategit-bim/src/assets/codemirror.bundle.js` — current borrower
- `moonshot-schema-validator` — prerequisite (WASM lint integration)
- BRIEF-bim-objects-system.md §E — dependency sovereignty map

---

*© 2026 PointSav Digital Systems™.*
