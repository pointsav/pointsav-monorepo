# RESEARCH — moonshot-schema-validator

**Status:** Research phase. No replacement code written.
**Registered:** 2026-06-14
**Priority:** HIGH
**"We Own It" target:** Replaces `jsonschema` Rust crate (server) + `ajv` JS library (client)

---

## Dependencies replaced

### jsonschema Rust crate
- **Version borrowed:** 0.20+ (will pin in app-privategit-bim Cargo.toml)
- **Licence:** MIT
- **SLOC:** ~15,000 lines (validator core + referencing + meta-schema)
- **What it does:** Validates a `serde_json::Value` against a JSON Schema 2020-12 document.
  Used in `app-privategit-bim/src/schema/validator.rs` as the server-side gate on every
  DTCG schema write (POST /edit/:slug).
- **Why borrowed:** Ship speed. JSON Schema 2020-12 is a complex spec; the crate handles
  $ref resolution, dialect negotiation, and keyword cross-references correctly.
- **Replacement complexity:** Medium. PBS-1 uses a narrow subset of JSON Schema 2020-12
  keywords. A purpose-built PBS-1 validator would be ~2,000 lines of Rust.

### ajv (client-side JSON Schema)
- **Version borrowed:** 8.x (vendored ESM, ~120KB minified)
- **Licence:** MIT
- **SLOC:** ~18,000 lines (TypeScript source)
- **What it does:** Real-time JSON validation in the CodeMirror browser pane.
  Provides linting feedback as the operator edits DTCG JSON.
- **Why borrowed:** No WASM JSON Schema validator exists at sufficient maturity.
- **Replacement path:** Compile `moonshot-schema-validator` to `wasm32-unknown-unknown`
  target → load as `<script type="module">` → replace ajv entirely. Single codebase,
  server and browser both validate identically.

---

## Technical approach (planned)

PBS-1 validation scope (JSON Schema 2020-12 keywords required):
- `type`: "object", "string", "number", "boolean", "array"
- `properties`: object property definitions
- `required`: array of required property names
- `additionalProperties`: boolean (false for strict PBS-1 objects)
- `enum`: allowed string values (IFC class codes, compliance keys)
- `$ref`: internal cross-references between schema files
- `allOf`: composition (BIM entity inheriting from base object type)
- Custom vocabulary: `bim.entity` dialect annotation extension

WASM compilation path:
```toml
[lib]
crate-type = ["cdylib", "rlib"]  # cdylib for WASM, rlib for native

[target.wasm32-unknown-unknown.dependencies]
wasm-bindgen = "0.2"
```

---

## Prior art surveyed

| Library | Licence | Status | Why not used |
|---|---|---|---|
| `jsonschema` Rust 0.20 | MIT | Active, stable | Borrowed now; full 2020-12 more than needed |
| `boon` Rust | MIT | Active | Smaller scope; worth evaluating |
| `ajv` JS v8 | MIT | Active, industry standard | Borrowed now (client side) |
| `@cfworker/json-schema` | MIT | CF Workers targeted | WASM-compatible but CF-specific |

`boon` (https://github.com/nicholasgasior/boon) is the candidate for the native side.
`wasm-bindgen` exports will expose `validate(schema_str, doc_str) -> bool | error`.

---

## Integration notes

When this crate ships, the migration is:
1. Cargo.toml: remove `jsonschema` dependency, add `moonshot-schema-validator`
2. `app-privategit-bim/src/schema/validator.rs`: swap call site (~5 lines)
3. Build with `--target wasm32-unknown-unknown`, copy `.wasm` to assets/
4. `bim.js`: replace `ajv` import with WASM module load

`bim.woodfinegroup.com` stays live throughout — the API surface is identical.

---

## Related

- `moonshot-bim-parser` — higher-priority Rust IFC parser replacement
- BRIEF-bim-objects-system.md §E — dependency sovereignty map
- `app-privategit-bim/src/schema/validator.rs` — current borrower
