# BENCHMARKS — system-ledger

Published numbers from the Phase 1A.5 clean benchmark run.

**Run date:** 2026-04-27  
**Crate versions:** system-core 0.2.0, system-ledger 0.2.1  
**Hardware:** Intel Xeon 2.20 GHz, 4 vCPUs (GCP n2-class); load moderate-to-heavy during run  
**Profile:** release (`opt-level = 3`)

Full run report at `.agent/BENCH-v0.2.0.md` in the cluster working
directory. Numbers below are from `benches/consult.rs` (10 benchmarks).

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

**Log scaling:** bench #8 (1024-leaf, 10-hash path) vs bench #7 (8-leaf, 3-hash
path): 17.74 µs vs 5.37 µs — a 3.3× difference for a 128× deeper tree.
Confirms O(log n) growth.

---

## Hardware note

These numbers are specific to the Intel Xeon 2.20 GHz single-threaded context
on the GCP n2-class VM. Ed25519 operations (benches #2, 3, 6, 9, 10) are
hardware-bound; ARM embedded targets will be 10–50× slower per curve25519-dalek
performance data. SHA-256 operations (benches #7, 8) and cache operations
(benches #4, 5) are not hardware-portable in the same way.

For publication, always qualify with "Intel Xeon 2.20 GHz single-thread."
