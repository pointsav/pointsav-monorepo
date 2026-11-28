# moonshot-folio assurance

Lightweight, in-repo assurance process (Power of Ten adapted to Rust plus NASA-STD-8739.8 practice).
Checked by `cargo xtask` (task F0.4). No external tooling.

| File | Purpose |
|---|---|
| `REQUIREMENTS.md` | Numbered `FOLIO-REQ-nnn`, each with a safety class and owning crate |
| `TRACE.tsv` | REQ to test names; `cargo xtask trace` fails on an orphan REQ or an unknown tag |
| `VERIFICATION.md` | REQ by method by level matrix |
| `REVIEW-LOG.md` | Independent review per merged phase (reviewer is not the author) |
| `HAZARDS.md` | Hazard list with mitigations and the REQs that cover them |

Test tagging: a test names the requirement it covers with a doc line `// REQ: FOLIO-REQ-nnn` directly above `#[test]`.
Safety classes: `untrusted-parse`, `determinism`, `fidelity`, `process`.
Operator approval is needed for: re-baselining goldens, licence changes, promotion to canonical, graduation of each consumer.

Fuzzing: `cargo xtask fuzz [--iters N] [--seed S]` mutates the seed files in `assurance/fuzz-corpus/<kind>/` (kinds: zip, inflate, zlib, png, jpeg) with an in-house xorshift PRNG and fails on any panic, printing a hex reproducer. Add a seed file when a parser is added. A robustness loop never replaces a positive-decode test.
