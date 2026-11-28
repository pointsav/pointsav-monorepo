# Verification matrix

Methods: T = unit/integration test, G = golden comparison, F = fuzz/watchdog, I = inspection.
Levels: L1 text, L2 geometry, L3 XML skeleton, L4 pixel.

| REQ | Method | Level | Gate |
|---|---|---|---|
| 001 | T, G | L1 | `determinism` |
| 002 | I, T | n/a | clippy deny set, review |
| 003 | T | bytes | `determinism` (debug vs release, cross-host) |
| 004 | T, F, I | n/a | tests, `cargo xtask fuzz` (codec parsers done; font and PDF reader to add), REVIEW-LOG |
| 005 | T | n/a | tests at limit and limit+1 |
| 006 | T | n/a | tests |
| 007 | T | n/a | tests, clippy `arithmetic_side_effects` |
| 008 | T, G | L1, L2 | golden |
| 009 | T | L3 | tests |
| 010 | T | n/a | known-answer vectors |
| 011 | I | n/a | `vendor-check`, lockfile scan |
| 012 | I | n/a | compiler, grep gate |
| 013 | T | n/a | `vendor-check` |
| 014 | I | n/a | `purity` |
| 015 | T | n/a | known-answer, reference and split tests |
| 016 | T | n/a | `tests/zip.rs` determinism and name tests |
| 017 | T | n/a | `tests/zip.rs` truncation, corruption, limit tests; independent review pending (REVIEW-LOG) |
| 018 | T | n/a | `tests/inflate.rs` vectors from Python zlib, caps at limit and limit+1, truncation and corruption; independent review pending |
| 019 | T | n/a | `tests/png.rs` 17 vectors (independent Python encoder, PIL cross-check), limits, bomb, truncation, corruption; independent review pending |
| 020 | T | n/a | `tests/jpeg.rs` hand-built headers, limits, truncation, corruption; independent review pending |
