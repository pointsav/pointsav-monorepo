# Requirements

Status values: `draft`, `approved`, `verified`. Every REQ needs at least one TRACE row before it can be `verified`.

| ID | Class | Crate | Requirement | Status |
|---|---|---|---|---|
| FOLIO-REQ-001 | determinism | all | Output bytes are a pure function of (Document, Limits, FontBlobs); no clock, environment, locale or random reads | draft |
| FOLIO-REQ-002 | determinism | all | No `HashMap`/`HashSet` iteration in any output path; use ordered collections | draft |
| FOLIO-REQ-003 | determinism | all | Debug and release builds produce byte-identical output; two hosts produce byte-identical output | draft |
| FOLIO-REQ-004 | untrusted-parse | codec, font | Every parser returns `Err` on malformed input and never panics, indexes unchecked, or loops without an iteration budget | draft |
| FOLIO-REQ-005 | untrusted-parse | codec | Inflate enforces an absolute output cap per stream; ZIP enforces entry count and total-size caps | draft |
| FOLIO-REQ-006 | untrusted-parse | core | Each `Limits` field is tested at the limit and at limit+1 | draft |
| FOLIO-REQ-007 | fidelity | core | Unit conversion uses i64 EMU with i128 intermediates and one rounding rule; no `f64` in the IR | draft |
| FOLIO-REQ-008 | fidelity | layout | A missing glyph is reported, never silently dropped (missing-glyph policy) | draft |
| FOLIO-REQ-009 | fidelity | xlsx | Formula cells carry caller-supplied cached values; absence is an error | draft |
| FOLIO-REQ-010 | fidelity | codec | SHA-256 matches FIPS 180-4 vectors including 55, 56 and 64-byte padding and the million-a vector | draft |
| FOLIO-REQ-011 | process | all | Zero third-party crates in every manifest and lockfile | draft |
| FOLIO-REQ-012 | process | all | `#![forbid(unsafe_code)]`, no `build.rs`, no proc-macros, no `macro_rules!` except const table generators | draft |
| FOLIO-REQ-013 | process | all | Each crate builds standalone with only its family path dependencies | draft |
| FOLIO-REQ-014 | process | conform | Golden corpus contains only synthetic data | draft |
