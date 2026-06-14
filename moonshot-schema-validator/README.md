<div align="center">

# moonshot-schema-validator

[ Leer en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Moonshot Initiative — `moonshot-*` family
**Version:** 0.1.0
**Status:** Reserved-folder — research phase
**Cluster:** `cluster/project-bim` per workspace `PROJECT-CLONES.md`
**Priority:** HIGH

---

## What this replaces

This crate is the planned internal replacement for two borrowed dependencies
used in `app-privategit-bim`:

1. **`jsonschema` Rust crate v0.20+** (MIT) — server-side DTCG JSON Schema
   validation in the POST /edit/:slug handler
2. **`ajv` v8.x** (MIT, vendored JS, ~120KB) — client-side real-time JSON
   validation in the CodeMirror editor browser pane

Both validate JSON documents against JSON Schema 2020-12. The current split
(Rust server + JS client) is a temporary arrangement. The goal is a single
Rust implementation compiled once, deployed twice: as a native binary on
the server and as a WASM module loaded in the browser.

## Why this matters

The BIM platform validates the PBS-1 schema (PointSav BIM Object Schema v1)
at every save. Validating our own schema format with a 3rd-party library is
a structural debt. When we publish PBS-1 as a specification for the AEC
community, the canonical validator should be ours.

## Architecture (planned)

```
moonshot-schema-validator (Rust)
  ├── native target  → linked into app-privategit-bim server binary
  └── wasm32 target → compiled to .wasm → loaded by browser via <script>
       replaces ajv client-side dependency entirely
```

JSON Schema 2020-12 is an open specification (IETF, no licence restrictions).
The implementation scope is narrowed to what PBS-1 requires:
`type`, `properties`, `required`, `additionalProperties`, `enum`, `$ref`,
`allOf`, and the `bim.entity` vocabulary extension.

## Timeline

**Medium horizon (2027–2028).** The jsonschema Rust crate covers server-side
validation adequately and is MIT-licensed. WASM compilation of the Rust
crate to replace ajv is the medium-term goal.

Prerequisite: `app-privategit-bim` v1 must ship and validate PBS-1 correctly
with the borrowed crates before investing in the owned replacement.

## Cross-references

- `app-privategit-bim/src/schema/validator.rs` — current borrower (jsonschema crate)
- `app-privategit-bim/src/assets/bim.js` — client-side validation (ajv)
- `moonshot-bim-parser` — the higher-priority Rust replacement for IfcOpenShell
- BRIEF-bim-objects-system.md §E — dependency sovereignty map

---

*© 2026 PointSav Digital Systems™.*
