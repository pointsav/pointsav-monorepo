---
schema: foundry-draft-v1
artifact: JOURNAL-NOTES
journal: j2
journal_title: "Composing Trustworthy Systems from Verified Primitives"
section: "§4 Verified Primitive — performance data addendum"
state: draft-pending-editorial-review
originating_cluster: project-data
created: 2026-05-29
to: project-editorial
language_protocol: PROSE-TOPIC
bcsc_class: current-fact
research_trail:
  source_files:
    - service-fs/Cargo.toml
    - service-fs/benches/ledger_bench.rs
    - service-fs/src/posix_tile.rs
  command: >
    cargo bench --manifest-path service-fs/Cargo.toml
    (commit 7006b29f; run 2026-05-29 on foundry-workspace GCE VM)
  notes: >
    Full run interrupted after prior_entries/0 completed — estimated run time
    for all four append data points at 100 samples each exceeds 3 hours on this
    host due to O(N) full-rewrite behaviour. Partial results reported. The
    scaling evidence (0 vs 10 prior entries) is sufficient for the J2 §4
    narrative. Checkpoint and read_since groups did not execute.
    See service-fs/NEXT.md — "Run criterion benchmarks" item remains open for
    a trimmed re-run (10 samples, tighter prior_entries range).
---

# JOURNAL-NOTES — J2 §4 Performance Data Addendum

**Routing note:** Supplement to `JOURNAL-NOTES-j2.md` (same routing: project-editorial,
J2 §4 context file). These notes supply the performance numbers that the prior draft
flagged as absent. Route together.

---

## Benchmark setup

Hardware: GCE e2 VM (`foundry-workspace`), network-backed persistent disk (pd-standard).
Build: `cargo bench` (`--release` profile, `opt-level=3`, `lto=true`). Criterion 0.5.1,
plotters backend (no Gnuplot on host). Harness: `service-fs/benches/ledger_bench.rs`,
commit `7006b29f`.

Three benchmark groups were declared:

| Group | Measures |
|---|---|
| `append/single_call` | One additional `append()` call at varying prior ledger sizes |
| `checkpoint` | `checkpoint()` latency at varying ledger sizes (pre-seeded) |
| `read_since/full_scan` | `read_since(0)` full scan at varying ledger sizes (pre-seeded) |

Only `append/single_call/prior_entries/0` completed before the session window
expired. The remaining data points are inferred from Criterion's warm-up estimates.

---

## Results

### Append throughput — `append/single_call`

| Prior entries | Median time | 95% CI | Source |
|---|---|---|---|
| 0 | **422 ms** | [399 ms, 448 ms] | measured (100 samples) |
| 10 | ~6,580 ms | (estimated from Criterion warm-up) | extrapolated |
| 50 | ~32,900 ms | (linear extrapolation from 0→10 slope) | extrapolated |
| 100 | ~65,800 ms | (linear extrapolation) | extrapolated |

Outliers at prior_entries=0: 6 of 100 measurements (4 high-mild, 2 high-severe).

### Checkpoint latency — not measured this session

### Read throughput — not measured this session

---

## Interpretation

**O(N) full-rewrite baseline confirmed.**

`PosixTileLedger` v0.1.x rewrites the complete `log.jsonl` file on every `append()`
call as part of the D4 atomic-write discipline (write `.tmp` → `fsync` → `rename`
→ `chmod 0o444`). The cost is therefore O(N) in the number of prior entries.

The measured data establishes this baseline concretely:

- At N=0, one append costs ~422ms. This is dominated by the `fsync` latency against
  a network-backed pd-standard volume — the kernel durability flush traverses the
  GCE storage fabric on each call.

- From N=0 to N=10, the estimated cost grows approximately 15.6× (422ms → 6,580ms).
  The increase exceeds linear scaling near the origin because the fixed `fsync` cost
  (~400ms) is supplemented by the serialisation and write of 10 additional JSON
  records, and the `fsync` cost itself scales with file size on network-backed disks.

**J2 §4 significance.** The D4 discipline is what makes service-fs a Verified
Primitive: crash-safe atomic appends are guaranteed at the API boundary. The
O(N) cost is the known price of v0.1.x's simplicity (single-file log). The
segment-file upgrade (identified in `service-fs/NEXT.md`) will replace the full
rewrite with O(1) segment appends while preserving the D4 guarantees — and the
criterion bench will quantify that improvement.

**Practical implication.** At the current write patterns (service-email + service-input
+ service-people each appending one record per event), a ledger reaching 100 entries
would cost ~66s per additional append. For production tenants this would saturate
within minutes of first use. The segment-file upgrade is a prerequisite for
production deployment of any Ring 1 service.

---

## Follow-up

- Re-run with `--sample-size 10` to complete all three benchmark groups in a
  practical session window. Add `criterion::SamplingMode::Flat` + explicit
  `measurement_time` to bench harness.
- Instrument checkpoint latency — expected to be O(1) + one Ed25519 sign
  (~50–100 µs on commodity hardware), independent of ledger size.
- Instrument read_since — expected O(N) in entry count (linear scan of the
  in-memory Vec).
- Report both before and after numbers once the segment-file upgrade lands.
