# gen/ - data generators for moonshot-folio-text

## make_tables.pl -> src/tables.rs

Generates the Line_Break property table. Provenance: Perl 5.38.2 `Unicode::UCD`
(`prop_invmap`) on the VM, which bundles **Unicode 15.0.0**. Properties used: Line_Break,
General_Category, East_Asian_Width, Extended_Pictographic. The generator folds them into one
sorted `(first code point, class)` table of 3589 entries:

- SA resolved by LB1 (Mn/Mc to CM, otherwise AL);
- QU split into `QU_PI`/`QU_PF`/`QU` by General_Category Pi/Pf (LB15a, LB15b);
- OP/CP split into `OP_EA`/`CP_EA` when East_Asian_Width is F, W or H (LB30);
- unassigned Extended_Pictographic code points become `ID_PICT` (LB30b).

Regenerate: `perl gen/make_tables.pl > src/tables.rs && cargo fmt -p moonshot-folio-text`.
Perl 15.0.0 is older than the ICU oracle (15.1); characters whose class changed in 15.1
(for example new Brahmic classes) are therefore not covered by the oracle corpus.

## tests/gen/make_corpus.py -> tests/data/icu_line_breaks.txt

Builds a deterministic corpus (all ordered pairs of representative code points of every
Line_Break class, triples around 32 context classes, ~100 hand-picked strings, 30000 seeded
random mixes) and records the break offsets of ICU 74.2 (`PyICU`, root locale; identical to
`en@lb=strict`) as an independent oracle.
Regenerate: `python3 -I tests/gen/make_corpus.py root tests/data/icu_line_breaks.txt` (run from the
crate directory). Dictionary scripts (Thai, Lao, Khmer, Myanmar) are excluded because ICU
segments them with a dictionary that UAX#14 does not specify.
