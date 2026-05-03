# Canonical JSON Schema — Workplace\*Proforma

**Version:** 1.0 (Draft)
**Status:** Specification for the single canonical file format.
**Licence:** Public specification. Any party may implement a compatible reader or writer.

---

## 1. Position

This schema is load-bearing infrastructure. The `.json` file format defined here is the only canonical representation of a proforma in Workplace\*Proforma. Files written to this schema today must open unchanged in every future version of any application that implements it.

The schema follows five design commitments:

1. **Text-first.** The file is valid UTF-8 JSON per RFC 8259 / ISO/IEC 21778:2017.
2. **Self-describing.** Field names are human-readable and semantically meaningful.
3. **AI-native.** Structure is optimised for language models reading the file directly, not for compactness or parsing speed.
4. **Versioned.** Every file declares its schema version in the first field of the root object.
5. **Backward-compatible forever.** New versions add fields. They never remove or repurpose existing ones. Version 1.0 files open in every future version unchanged.

---

## 2. Root Object

```json
{
  "proforma_version": "1.0",
  "document_id": "uuid-v7-here",
  "anchor": null,
  "metadata": { ... },
  "template": null,
  "assumptions": [ ... ],
  "sheets": [ ... ],
  "named_ranges": { ... },
  "presentation": { ... },
  "audit": { ... }
}
```

Every field is present in every file. Optional fields are set to `null` or `{}` or `[]` when unused, never omitted. This makes the structure self-describing and the diff behaviour predictable.

---

## 3. Fields

### 3.1 `proforma_version` (required)

Semantic version string identifying the schema version the file conforms to. Required as the first field for rapid version detection by readers. Current value: `"1.0"`.

### 3.2 `document_id` (required)

UUIDv7-compatible identifier assigned on file creation. Never changes across edits. Distinct from the file path or filename. Allows a proforma to be tracked across rename, move, or archive operations.

### 3.3 `anchor` (optional, null for standalone)

Binds the proforma to a specific legal asset when archive-bound.

```json
{
  "archive_type": "PropertyArchive",
  "archive_id": "BC-LTSA-009284733",
  "asset_name": "3100 Lougheed Highway",
  "jurisdiction": "BC-CA",
  "anchor_type": "land_title_pin"
}
```

For standalone (non-archive-bound) files, `anchor` is `null`. Standalone operation is the default in Phase 1.

### 3.4 `metadata` (required)

```json
{
  "title": "Untitled Proforma",
  "created": "2026-04-17T14:18:00-07:00",
  "last_modified": "2026-04-17T14:18:00-07:00",
  "author": "",
  "description": "",
  "currency": "USD",
  "locale": "en-US",
  "schema_url": "https://schemas.pointsav.com/proforma/1.0"
}
```

### 3.5 `template` (optional)

Identifies the template the file was created from. `null` for files created as blank documents.

```json
{
  "id": "multifamily-10yr-v3",
  "name": "Multifamily — 10 Year Hold",
  "version": "3.2",
  "source": "content-wiki-proforma-templates/multifamily/10yr-v3.json"
}
```

### 3.6 `assumptions` (required)

Ordered array of input assumptions. Formulas reference these by `id`.

```json
[
  {
    "id": "rent_growth",
    "label": "Annual Rent Growth",
    "value": 0.035,
    "unit": "ratio",
    "format": "percent-1dp",
    "category": "revenue"
  }
]
```

### 3.7 `sheets` (required)

Ordered array of sheet objects. Each sheet has a `type` that determines how the grid renders it.

**Assumptions sheet** (type: `"assumptions"`) — renders the `assumptions` array as a simple label/value grid.

**Proforma sheet** (type: `"grid"`) — renders sections and line items as the full proforma grid with year columns.

**Returns sheet** (type: `"summary"`) — renders a summary grid of key metrics.

```json
{
  "id": "proforma",
  "name": "Proforma",
  "type": "grid",
  "sections": [
    {
      "id": "revenue",
      "label": "Revenue",
      "account_code_range": "4000-4999",
      "lines": [
        {
          "id": "pgi",
          "label": "Potential Gross Income",
          "account_code": "4000",
          "formula": "",
          "format": "currency-0dp"
        }
      ]
    }
  ]
}
```

Each line has:

- `id` — Semantic identifier, unique within the file. Used in formula references.
- `label` — Human-readable row label.
- `account_code` — Binding to the Chart of Accounts. Enables cross-proforma aggregation.
- `formula` — Excel-compatible formula string (without the leading `=`). References other lines by `id` (preferred) or cell coordinate.
- `format` — Display format for rendered cells.
- `emphasis` (optional) — `"subtotal"` | `"total"`. Visual treatment hint.

### 3.8 `named_ranges` (optional)

Named ranges for use in formulas. Resolved by the engine at evaluation time.

```json
{
  "NOI": "sections.noi.lines.noi.years",
  "HoldPeriod": "y1:y10"
}
```

### 3.9 `presentation` (optional)

Visual presentation metadata. Affects rendering only, not calculation.

```json
{
  "theme": "institutional-cream",
  "column_widths": { "label": 280, "code": 60, "year": 108 },
  "frozen_panes": { "columns": 2, "rows": 1 },
  "conditional_formatting": []
}
```

### 3.10 `audit` (required)

The integrity chain. Populated when the file is saved and re-verified when opened.

```json
{
  "sha256": "a81c9d2f7e5b...",
  "signed_by": null,
  "signed_at": "2026-04-17T14:18:00-07:00",
  "parent_sha256": null,
  "commit_context": null
}
```

---

## 4. Formula Language

Formulas use Excel-compatible syntax with one extension: references by semantic `id` are preferred over cell coordinates.

### 4.1 Reference Styles

**Semantic reference (preferred):**
```
=pgi * (1 - vacancy)
=rent_growth + 0.005
=noi / annual_ds
```

**A1-notation (legacy, supported):**
```
=B2*B3*12
=SUM(C2:C11)
```

The engine accepts both. Semantic references are recommended because they survive row reordering and are legible to AI readers.

### 4.2 Supported Operators

`+`, `-`, `*`, `/`, `^`, `(`, `)`, `=`, `<>`, `<`, `>`, `<=`, `>=`

### 4.3 Phase 1 Functions

**Aggregation:** `SUM`, `AVERAGE`, `MIN`, `MAX`, `COUNT`

**Math:** `ABS`, `ROUND`, `NEG`

**Logic:** `IF`

**Financial:** `PMT`, `PV`, `FV`, `NPV`, `IRR`

### 4.4 Phase 2 Functions

Phase 2 replaces the JS engine with IronCalc, adding 200+ Excel-compatible functions including `XIRR`, `XNPV`, `IPMT`, `PPMT`, `CUMPRINC`, `CUMIPMT`, `NPER`, `RATE`, `SUMIF`, `COUNTIF`, `VLOOKUP`, `INDEX/MATCH`, `CHOOSE`, date functions, text functions, and array formulas.

---

## 5. Versioning and Compatibility

### 5.1 The Backward-Compatibility Commitment

**Every version of the application must open every prior version of the schema, unchanged, forever.**

Concretely:
- A file written by Schema 1.0 opens without modification in any future version.
- A file written by Schema 1.3 is automatically interpreted by a 1.3-capable application.
- The application never silently upgrades a file. Upgrades are explicit user actions that produce a new file with a new version tag, preserving the original.

### 5.2 What Constitutes a Compatible Change

**Permitted (minor version bump, e.g. 1.0 → 1.1):**
- Add new optional fields to existing objects
- Add new sections or line categories
- Add new supported formula functions
- Add new format or presentation options

**Prohibited (forces a major version bump, e.g. 1.x → 2.0):**
- Rename any existing field
- Change the semantic meaning of any existing field
- Remove any existing field
- Change the type of any existing field
- Change formula evaluation semantics for existing functions

### 5.3 The RFC Process

Any change to the schema follows a public Request for Comment process, hosted in the pointsav-monorepo schemas directory. A 30-day public review period is required for schema changes.

---

## 6. Reference Implementation

The Phase 1 reference implementation is in this repository at `src/js/schema.js` and `src/js/engine.js`. A pure-Rust reference will be added in Phase 2 alongside the IronCalc integration.

A second-party implementer in any language can consume this specification to build a compatible reader without needing access to PointSav proprietary code.

---

## 7. Open Questions

Tracked for future resolution:

1. **Number precision and currency handling.** IEEE 754 double has limitations for large monetary values. Candidates: (a) store monetary values as strings, (b) adopt a fixed-precision decimal representation, (c) accept IEEE 754 with explicit documentation.
2. **Date representation.** ISO 8601 string is the default. Spreadsheet date serial numbers (Excel's 1900-based system) may be needed for compatibility.
3. **Unit handling.** `"sf"`, `"usd"`, `"ratio"` are informal strings. A formal unit system may be justified.
4. **Localisation.** Number formats and function names vary by locale. The specification declares en-US canonical; locale variations are future work.
