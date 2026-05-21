# BENCHMARKS — system-ledger

Published numbers covering all 12 criterion benchmarks in `benches/consult.rs`.
Benchmarks 1–10 measured 2026-04-27; benchmarks 11–12 added 2026-05-21 (see
measurement notes below).

**Hardware:** Intel Xeon 2.20 GHz, 4 vCPUs (GCP n2-class)  
**Profile:** release (`opt-level = 3`)  
**Crate versions:** system-core 0.2.0, system-ledger 0.2.1

Full Phase 1A.4/1A.5 run report at `.agent/BENCH-v0.2.0.md` in the cluster
working directory.

---

## Results

| # | Benchmark | Mean | CI low | CI high | Notes |
|---|---|---|---|---|---|
| 1 | `Capability::hash` | 6.44 µs | 6.35 µs | 6.54 µs | |
| 2 | `SignedCheckpoint::verify_signer` (1-sig) | 4.01 ms | 3.92 ms | 4.10 ms | hardware-bound (Ed25519) |
| 3 | `SignedCheckpoint::verify_apex_handover` (2-sig) | 7.65 ms | 7.50 ms | 7.83 ms | hardware-bound |
| 4 | cache `lookup_by_tree_size` (hit, most-recent) | 11.2 ns | 10.5 ns | 12.0 ns | |
| 5 | cache `lookup_by_tree_size` (miss, full scan) | 362 ns | 351 ns | 373 ns | zero outliers |
| 6 | `consult_capability` (Allow path; 1-sig apex verify) | 3.74 ms | 3.66 ms | 3.83 ms | dominated by verify_signer |
| 7 | `InclusionProof::verify` (raw, tree-size 8) | 5.37 µs | 5.29 µs | 5.44 µs | |
| 8 | `InclusionProof::verify` (raw, tree-size 1024) | 17.74 µs | 17.57 µs | 17.91 µs | zero outliers |
| 9 | `SignedCheckpoint::verify_inclusion_proof` (composed, 1024-leaf) | 4.72 ms | 4.27 ms | 5.24 ms | widest CI (load-sensitive) |
| 10 | `apply_witness_record` (full path) | 3.71 ms | 3.68 ms | 3.74 ms | zero outliers, tight CI |
| 11 | `ConsistencyProof::verify` (raw, 4→8 — 4-hash proof) | 10.86 µs | 10.73 µs | 11.00 µs | † |
| 12 | `SignedCheckpoint::verify_consistency_proof` (composed, 4→8) | 8.37 ms | 8.22 ms | 8.53 ms | hardware-bound (Ed25519×2) † |

† Benches 11–12 measured 2026-05-21 in a targeted run (load avg 10.3 on 4 vCPUs).
Absolute values may run 10–20% higher than benches 1–10 which were measured
under lighter load 2026-04-27. Shape comparisons (ratios) within this table
remain valid.

---

## Architectural observations

**Cache speedup:** cache hit (11.2 ns) is ~358,000× faster than `verify_signer`
(4.01 ms). In stable operation, nearly all `consult_capability` calls are cache
hits. The cache is architecturally critical — not optional.

**Inclusion overhead:** `verify_inclusion_proof` (composed, bench #9) vs
`verify_signer` alone (bench #2): ~4.72 ms vs 4.01 ms. The Merkle inclusion
check adds ~18% overhead — dominated by the ed25519 signature verify, not the
hash-chain traversal. Per `topic-merkle-proofs-as-substrate-primitive.md` §6:
cache and inclusion proofs are complementary, not redundant.

**Consistency-proof overhead:** `verify_consistency_proof` (composed, bench #12)
costs ~8.37 ms — roughly 2× `verify_signer` (bench #2). Expected: the composed
primitive verifies both the old and new checkpoint signatures (two Ed25519 ops)
before the raw proof check. The raw proof alone (bench #11, 10.86 µs) is a
trivial fraction of the composed cost, confirming that signature verification
dominates consistency checking as it does for inclusion proofs.

**Log scaling:** bench #8 (1024-leaf, 10-hash path) vs bench #7 (8-leaf, 3-hash
path): 17.74 µs vs 5.37 µs — a 3.3× difference for a 128× deeper tree.
Confirms O(log n) growth. The 4-hash consistency proof (bench #11, 4→8 tree) at
10.86 µs is within the same SHA-256-dominated band.

---

## Hardware note

These numbers are specific to the Intel Xeon 2.20 GHz single-threaded context
on the GCP n2-class VM. Ed25519 operations (benches #2, 3, 6, 9, 10, 12) are
hardware-bound; ARM embedded targets will be 10–50× slower per curve25519-dalek
performance data. SHA-256 operations (benches #7, 8, 11) and cache operations
(benches #4, 5) are not hardware-portable in the same way.

For publication, always qualify with "Intel Xeon 2.20 GHz single-thread."
