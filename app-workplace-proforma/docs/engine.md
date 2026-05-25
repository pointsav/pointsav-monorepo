# Formula Engine — Workplace\*Proforma

**Phase:** 1 (MVP)
**Implementation:** `src/js/engine.js` — pure JavaScript, EUPL-1.2

---

## Overview

The Phase 1 formula engine is a minimal JavaScript evaluator sufficient for the MVP scope. It supports the arithmetic, references, ranges, and functions that cover the overwhelming majority of institutional proforma workloads.

Phase 2 replaces this module with IronCalc (Apache 2.0, pure Rust, NLnet + European Commission funded) invoked through a Tauri IPC command. The public API is designed to remain stable across that transition.

---

## Supported features

### Operators

| Operator | Meaning |
|---|---|
| `+` | Addition |
| `-` | Subtraction (binary) or negation (unary) |
| `*` | Multiplication |
| `/` | Division |
| `^` | Exponentiation |
| `(`, `)` | Grouping |
| `=` | Equality (returns 1 or 0) |
| `<>` | Inequality |
| `<`, `>`, `<=`, `>=` | Comparison |

### References

**A1-notation:** `A1`, `B5`, `$C$10`, `AA100`

**Range:** `A1:C10` — returns all cells in the rectangular region as a flat array

**Semantic (preferred):** `pgi`, `revenue_pgi`, `assumption_rent_growth` — the `id` of any line or assumption in the document. Semantic references are resolved via the document's `idToCoord` map and are recommended because they survive row reordering and are legible to AI readers.

**Dotted path:** `pgi.y3` — reference a specific year column of a line's semantic id.

### Functions

#### Aggregation

- `SUM(range_or_values)` — Sum of all numeric values
- `AVERAGE(range_or_values)` — Arithmetic mean
- `MIN(range_or_values)` — Smallest value
- `MAX(range_or_values)` — Largest value
- `COUNT(range_or_values)` — Count of numeric values

#### Math

- `ABS(number)` — Absolute value
- `ROUND(number, places)` — Round to `places` decimal places
- `NEG(number)` — Negation (equivalent to unary minus)

#### Logic

- `IF(condition, true_value, false_value)` — Return `true_value` if `condition` is truthy, else `false_value`

#### Financial

- `PMT(rate, nper, pv)` — Periodic loan payment. Rate per period; number of periods; present value. Returns negative (cash outflow).
- `PV(rate, nper, pmt)` — Present value of a stream of payments.
- `FV(rate, nper, pmt, pv)` — Future value of an investment.
- `NPV(rate, values...)` — Net present value. First argument is discount rate; remaining arguments are period cashflows.
- `IRR(values)` — Internal rate of return via Newton-Raphson iteration.

---

## Error values

The engine returns error markers when evaluation fails:

| Error | Meaning |
|---|---|
| `#DIV/0!` | Division by zero |
| `#NAME?` | Unresolved identifier — typically a semantic id that does not exist in the document |
| `#CIRC!` | Circular reference — a cell's formula depends on its own value |
| `#NUM!` | Numerical failure — e.g. IRR did not converge |
| `#ERR!` | Parse or evaluation error — typically malformed formula syntax |

---

## Evaluation model

The engine uses a memoised recursive evaluator with cycle detection:

1. `evaluateAll()` walks every cell in the current sheet's cell store.
2. For each formula cell, `evalCellByRef()` tokenises, parses, and evaluates.
3. Memoisation via the `computed` field prevents re-evaluation within a single pass.
4. Cycle detection via a `visited` set prevents infinite recursion on circular references.

Typical institutional proforma workloads (up to ~2000 formula cells) evaluate in under 10ms on commodity hardware.

---

## Known limitations (Phase 1)

Known to not be supported until Phase 2 IronCalc replaces this engine:

- **Array formulas and dynamic arrays** — Excel 365's SPILL behaviour
- **Advanced financial functions** — XIRR, XNPV, IPMT, PPMT, CUMPRINC, CUMIPMT, NPER, RATE
- **Lookup functions** — VLOOKUP, HLOOKUP, INDEX, MATCH, XLOOKUP, CHOOSE
- **Statistical functions** — STDEV, VAR, CORREL, LINEST, forecast functions
- **Text functions** — CONCATENATE, LEFT, RIGHT, MID, FIND, SUBSTITUTE
- **Date/time functions** — DATE, TODAY, NOW, YEAR, MONTH, WORKDAY
- **Conditional aggregation** — SUMIF, COUNTIF, AVERAGEIF, SUMIFS
- **Cross-sheet references** — `Sheet2!A1` syntax
- **Error-handling wrappers** — IFERROR, IFNA
- **Array constants** — `{1,2,3}` literal syntax

Documents that require these functions can be authored in Phase 1 using simpler formula structures; Phase 2 adds support without requiring any file-format changes.

---

## Phase 2 migration

When IronCalc integration ships, this module's public API remains:

```javascript
WorkplaceEngine.setCell(ref, raw)
WorkplaceEngine.getCell(ref)
WorkplaceEngine.getValue(ref)
WorkplaceEngine.setCells(cells)
WorkplaceEngine.getAllCells()
WorkplaceEngine.setIdMap(map)
WorkplaceEngine.setYearColumns(cols)
WorkplaceEngine.evaluateAll()
WorkplaceEngine.evaluateFormula(formula, contextCoord)
WorkplaceEngine.formatValue(value, format)
WorkplaceEngine.parseRef(ref)
WorkplaceEngine.coord(col, row)
WorkplaceEngine.colIndexToLetter(idx)
WorkplaceEngine.colLetterToIndex(letters)
```

Phase 2 implementations route these calls through Tauri IPC to a Rust-side IronCalc engine. `evaluateAll()` becomes `tauriInvoke('evaluate_workbook', { cells })`. The rest of the grid and toolbar code requires no changes.

---

## Inspection and debugging

In the dev WebView console:

```javascript
WorkplaceEngine.getAllCells()                    // all cells on active sheet
WorkplaceEngine.getCell("C5")                     // single cell detail
WorkplaceEngine.evaluateFormula("=SUM(C2:C10)")   // standalone eval
WorkplaceEngine.coord(5, 10)                      // A1-notation for col=5 row=10
```

All formula cells expose their parsed `formula`, stored `raw` text, memoised `computed` value, and resolved final `value` for debugging.
