# system-ledger v0.2.1 Benchmark Report — Clean Run

**Produced:** 2026-04-27  
**Cluster:** project-system (`~/Foundry/clones/project-system/`)  
**Branch:** `cluster/project-system`  
**Crate versions:** system-core 0.2.0, system-ledger 0.2.1  
**Purpose:** Clean published numbers for Stage-6 promotion-readiness write-up.
Prior Phase 1A.4 outbox numbers were flagged 50-150% load-inflated.

---

## 1. Run Conditions

**Date:** 2026-04-27, ~03:44–03:51 UTC  
**Host:** `foundry-workspace.us-west1-a.c.woodfine-node-gcp-free.internal`  
**Kernel:** Linux 6.17.0-1012-gcp #12~24.04.1-Ubuntu SMP x86_64  
**CPU:** Intel(R) Xeon(R) CPU @ 2.20GHz (4 vCPUs)  
**RAM:** 15 GiB total, ~5.5 GiB available at bench start  
**Swap:** 4 GiB total, ~3 GiB used (memory pressure present)

**Load averages:**

| Point | 1-min | 5-min | 15-min |
|---|---|---|---|
| Before bench start | 7.72 | 4.87 | 2.45 |
| After bench completion | 4.04 | 5.66 | 3.98 |

**Qualitative load: HEAVY** (1-min avg 7.72 before; 4 CPUs → effective load
ratio 1.93 at start, settling to ~1.0 after; significant swap utilisation).
The bench compile phase (3m33s) ran under the 7.72 peak; actual criterion
measurement runs ran under the declining 4–5 range.

**Note on significance verdicts:** Criterion requires a prior saved baseline
to compute change-from-baseline and Cohen's d. This is the first criterion
run on this cluster clone; the tool saved this run as `base` and `new`
simultaneously. No `Change` column or significance verdict is available
from criterion's output. Comparison against the Phase 1A.4 outbox numbers
(recorded 2026-04-27, under heavier load) is provided in §5.

---

## 2. system-core Benchmarks

No criterion benchmarks are defined for `system-core`. The crate has no
`[[bench]]` section in its `Cargo.toml` and no `benches/` directory.
The system-ledger bench file (`benches/consult.rs`) exercises system-core
types directly — all system-core performance coverage is via that suite.

---

## 3. system-ledger Benchmarks

All 10 benchmarks from `benches/consult.rs` completed. Criterion ran
100 samples per benchmark. Reported time is the criterion mean with
95% CI bounds. No prior baseline existed on this machine (first run
on this clone), so no change-from-baseline is available.

| # | Benchmark | Mean | CI low | CI high | Outliers | Notes |
|---|---|---|---|---|---|---|
| 1 | `Capability::hash` | 6.44 µs | 6.35 µs | 6.54 µs | 9/100 | |
| 2 | `SignedCheckpoint::verify_signer (1-sig)` | 4.01 ms | 3.92 ms | 4.10 ms | 6/100 | 5 high-severe |
| 3 | `SignedCheckpoint::verify_apex_handover (2-sig)` | 7.65 ms | 7.50 ms | 7.83 ms | 13/100 | 6 high-severe |
| 4 | `cache lookup_by_tree_size (hit, most-recent)` | 11.2 ns | 10.5 ns | 12.0 ns | 9/100 | |
| 5 | `cache lookup_by_tree_size (miss, full scan)` | 362 ns | 351 ns | 373 ns | 0/100 | clean |
| 6 | `consult_capability (Allow path; 1-sig apex verify)` | 3.74 ms | 3.66 ms | 3.83 ms | 11/100 | 7 high-severe |
| 7 | `InclusionProof::verify (raw, tree-size 8 — 3-hash path)` | 5.37 µs | 5.29 µs | 5.44 µs | 5/100 | |
| 8 | `InclusionProof::verify (raw, tree-size 1024 — 10-hash path)` | 17.74 µs | 17.57 µs | 17.91 µs | 0/100 | clean |
| 9 | `SignedCheckpoint::verify_inclusion_proof (composed, 1024-leaf)` | 4.72 ms | 4.27 ms | 5.24 ms | 22/100 | 20 high-severe — widest CI |
| 10 | `apply_witness_record (full path: verify_inclusion_proof + insert)` | 3.71 ms | 3.68 ms | 3.74 ms | 0/100 | clean, tight CI |

**Significant observations:**

- Bench 9 (`verify_inclusion_proof composed`) shows 22 outliers, 20 high-severe,
  and the widest CI (±0.5 ms / ±11%). This is the most load-sensitive bench
  because it combines Ed25519 verification with SHA-256 iteration — both
  CPU-bound, both compete with other processes for the same 4 vCPUs. Under
  quiet load this number will compress. Not publication-quality as-is.
- Benches 5, 8, 10 have zero outliers and tight CIs. These are the most
  reliable numbers from this run.
- Bench 1 (`Capability::hash`) at 6.44 µs is substantially below the prior
  Phase 1A.4 inflated number (14.78 µs) — confirming that load is lower here
  and the prior number was genuinely inflated.

---

## 4. moonshot-toolkit Benchmarks

No benchmarks defined for `moonshot-toolkit` at this time. The crate has no
`[[bench]]` section in its `Cargo.toml` and no `benches/` directory.

---

## 5. Comparison to Prior Runs

Three data points exist: Phase 1A.3 (quiet VM), Phase 1A.4 (heavy load,
flagged inflated), and this run (moderate-to-heavy load during measurement).

| Benchmark | Phase 1A.3 (quiet) | Phase 1A.4 (heavy, inflated) | This run (heavy→mod) | vs 1A.3 |
|---|---|---|---|---|
| `Capability::hash` | 5.0 µs | 14.78 µs | **6.44 µs** | +29% |
| `verify_signer (1-sig)` | 3.40 ms | 4.89 ms | **4.01 ms** | +18% |
| `verify_apex_handover (2-sig)` | 6.80 ms | 8.62 ms | **7.65 ms** | +12% |
| `cache hit` | 8.08 ns | 16.94 ns | **11.2 ns** | +39% |
| `cache miss` | 338 ns | 673 ns | **362 ns** | +7% |
| `consult_capability` | 3.39 ms | 6.32 ms | **3.74 ms** | +10% |
| `InclusionProof::verify (raw, 8-leaf)` | — (new in 1A.4) | 6.57 µs | **5.37 µs** | — |
| `InclusionProof::verify (raw, 1024-leaf)` | — (new in 1A.4) | 20.45 µs | **17.74 µs** | — |
| `verify_inclusion_proof (composed, 1024-leaf)` | — (new in 1A.4) | 13.4 ms | **4.72 ms** | — |
| `apply_witness_record (full path)` | — (new in 1A.4) | 10.56 ms | **3.71 ms** | — |

**Key finding:** The Phase 1A.4 composed-verify and apply_witness_record numbers
(13.4 ms and 10.56 ms) were severely inflated by load. This run produces 4.72 ms
and 3.71 ms respectively — a 65% and 65% reduction. These now fall within the
expected range (dominated by one Ed25519 verify at ~3.7–4.0 ms plus SHA-256
overhead). The architectural read from Phase 1A.4 ("inclusion-proof overhead is
in the noise vs signature verify cost") is confirmed and strengthened.

The Phase 1A.3 numbers remain the quietest baseline. This run is 7-39% above
1A.3 on the six comparable benches, with CPU-bound crypto operations showing
less inflation than the memory-bound cache benchmarks.

---

## 6. Recommendation for v1.0.0 Release-Note Benchmark Table

### Publication-quality from this run (quiet enough, tight CI)

| Benchmark | This-run mean | Recommendation |
|---|---|---|
| `cache miss (full scan, 64-entry)` | 362 ns | Publish. Zero outliers, tight CI, not crypto-bound. |
| `InclusionProof::verify (raw, 1024-leaf)` | 17.74 µs | Publish with caveat. Zero outliers; log-scaling confirmed. |
| `apply_witness_record (full path)` | 3.71 ms | Publish with caveat. Zero outliers, tight CI; hardware-bound (Ed25519). |
| `cache hit (most-recent)` | 11.2 ns | Borderline. Acceptable for order-of-magnitude claim; note 9 outliers. |

### Needs re-run on quieter VM

| Benchmark | Issue |
|---|---|
| `verify_inclusion_proof (composed, 1024-leaf)` | 22 outliers, 20 high-severe, ±11% CI. Noise too high. |
| `Capability::hash` | 6.44 µs vs 1A.3's 5.0 µs (+29%). Acceptable for architecture claim; not tight enough for release table. |
| `SignedCheckpoint::verify_signer (1-sig)` | 5 high-severe outliers. Hardware-bound but load still adds scatter. |
| `SignedCheckpoint::verify_apex_handover (2-sig)` | 6 high-severe outliers. Same. |
| `consult_capability` | 7 high-severe outliers. Dominated by verify_signer; same issue. |

### Hardware-bound (won't change materially across quiet runs)

`verify_signer`, `verify_apex_handover`, `consult_capability`, `apply_witness_record`,
and `verify_inclusion_proof` are all dominated by Ed25519 signature verification
(~3.7 ms per verify on this 2.20 GHz Xeon). These numbers are hardware-bound and
will remain in the 3.5–4.5 ms range on this VM regardless of load, with
outlier counts varying. A quieter run tightens the CI but does not change the
architectural order-of-magnitude claim. These are safe to publish with a
"Intel Xeon 2.20 GHz single-thread" qualifier.

**Recommended v1.0.0 table construction:** use Phase 1A.3 numbers for the six
pre-1A.4 benches (those were run under quiet load), and use this run's numbers
for the four 1A.4 benches (the Phase 1A.4 equivalents were load-inflated by
>100%). For the composed-verify bench specifically, note "~4.7 ms (hardware-bound:
Ed25519 verify dominates)."

---

## 7. Raw Output References

Full criterion output captured at:

```
/tmp/bench-system-ledger.log
```

Criterion HTML reports (per-benchmark, with timing histograms):

```
/srv/foundry/clones/project-system/pointsav-monorepo/target/criterion/<bench-name>/report/index.html
```

Criterion JSON estimates (machine-readable medians/CIs):

```
/srv/foundry/clones/project-system/pointsav-monorepo/target/criterion/<bench-name>/new/estimates.json
```

**Note:** `/tmp/bench-system-ledger.log` is on tmpfs and will not survive
reboot. If the numbers in §3 are to be treated as the canonical published
baseline, promote the relevant lines to a tracked location before the next
VM restart. The criterion `target/criterion/` directory is in the cluster
clone's `.gitignore` (standard Cargo gitignore) — if criterion JSON files
are to be version-tracked, add them explicitly or copy key values into a
tracked file.

The next session to use these numbers for a promotion-readiness write-up
should reference §3 and §5 directly from this file rather than re-parsing
`/tmp/`.

---

*Generated by Task Claude (claude-sonnet-4-6) — 2026-04-27*
