# Hazards

| ID | Hazard | Mitigation | REQ |
|---|---|---|---|
| H-01 | Zip or deflate bomb exhausts memory | Absolute output cap, entry and size caps in `Limits` | 005, 006 |
| H-02 | Malformed font crashes or loops the parser | Bounds-checked reads, iteration budget, `Err` on bad tables | 004 |
| H-03 | Nondeterministic output breaks audit tarballs | No clock, env or hash-order dependence; debug/release and cross-host compare | 001, 002, 003 |
| H-04 | Silent text drop when a glyph is missing | Missing-glyph policy reports every miss | 008 |
| H-05 | Silent fidelity loss through float rounding | i64 EMU, one rounding rule, no `f64` in the IR | 007 |
| H-06 | Stale cached values in XLSX shown as truth | Cached values mandatory and caller-supplied; recalculation flagged on open | 009 |
| H-07 | Real business data enters the golden corpus | Synthetic-only corpus, `purity` gate | 014 |
| H-08 | Third-party code enters the build | Zero-dependency rule, lockfile scan | 011, 013 |
| H-09 | Recursion on adversarial nesting overflows the stack | No recursion; explicit stacks capped by `Limits::max_depth` | 004 |
