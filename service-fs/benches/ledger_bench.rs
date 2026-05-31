// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Criterion benchmarks for service-fs PosixTileLedger.
//
// ADR-07: zero AI in Ring 1. These benchmarks exercise deterministic
// primitives only: SHA-256 hash chain + Ed25519 checkpoint signing.
//
// Run:
//   cargo bench --manifest-path service-fs/Cargo.toml
//
// Results in target/criterion/. HTML report at
//   target/criterion/report/index.html
//
// Key findings (J2 §4 data — append throughput + checkpoint latency):
// see JOURNAL-NOTES-j2-benchmarks.md staged in .agent/drafts-outbound/.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use service_fs::ledger::LedgerBackend;
use service_fs::posix_tile::PosixTileLedger;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn bench_dir(tag: &str) -> PathBuf {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    std::env::temp_dir().join(format!("sfs-bench-{}-{}", tag, id))
}

/// Open a PosixTileLedger with `n` entries already appended.
/// Uses a fixed payload so results are reproducible.
fn seed_ledger(n: u64, root: &std::path::Path) -> PosixTileLedger {
    let ledger =
        PosixTileLedger::open(root, "bench", None::<&std::path::Path>).unwrap();
    let payload = serde_json::json!({
        "data": "benchmark-payload-fixed-32-bytes"
    });
    for i in 0..n {
        ledger
            .append(&format!("bench-{}", i), &payload)
            .unwrap();
    }
    ledger
}

fn cleanup(root: &std::path::Path) {
    let _ = std::fs::remove_dir_all(root);
}

// ---------------------------------------------------------------------------
// Append throughput — measures one additional append at varying prior sizes.
//
// Background: PosixTileLedger v0.1.x rewrites the full log.jsonl on each
// append (D4 atomic-write discipline). Append cost is therefore O(N) in the
// number of prior entries. This bench documents that baseline so the segment-
// file upgrade can quantify the improvement.
// ---------------------------------------------------------------------------
fn bench_append(c: &mut Criterion) {
    let payload = serde_json::json!({
        "data": "benchmark-payload-fixed-32-bytes"
    });
    let mut group = c.benchmark_group("append/single_call");

    for &prior in &[0u64, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::new("prior_entries", prior),
            &prior,
            |b, &prior| {
                b.iter_batched(
                    || {
                        let root = bench_dir(&format!("app-{}", prior));
                        let ledger = seed_ledger(prior, &root);
                        (root, ledger)
                    },
                    |(root, ledger)| {
                        ledger.append("bench-final", &payload).unwrap();
                        cleanup(&root);
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Checkpoint latency — measures checkpoint() at varying log sizes.
//
// checkpoint() reads the chain tip and (optionally) produces an Ed25519
// signed-note. Without a signing key it is effectively O(1): no iteration
// over prior entries. With a signing key the cost is dominated by one
// Ed25519 signing operation (~50–100 µs on commodity hardware).
//
// The ledger is pre-seeded ONCE before the inner loop so setup cost
// is not included in the measurement.
// ---------------------------------------------------------------------------
fn bench_checkpoint(c: &mut Criterion) {
    let mut group = c.benchmark_group("checkpoint");

    for &size in &[1u64, 100, 1_000] {
        let root = bench_dir(&format!("cp-{}", size));
        let ledger = seed_ledger(size, &root);

        group.bench_with_input(
            BenchmarkId::new("entries", size),
            &size,
            |b, _| {
                b.iter(|| ledger.checkpoint().unwrap());
            },
        );

        cleanup(&root);
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Read throughput — measures read_since(0) at varying log sizes.
//
// read_since returns all entries with cursor > since. At since=0 this is a
// full scan. Cost is O(N) in the number of entries (each is a clone from the
// in-memory Vec).
// ---------------------------------------------------------------------------
fn bench_read_since(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_since/full_scan");

    for &size in &[10u64, 100, 1_000] {
        let root = bench_dir(&format!("rs-{}", size));
        let ledger = seed_ledger(size, &root);

        group.bench_with_input(
            BenchmarkId::new("entries", size),
            &size,
            |b, _| {
                b.iter(|| ledger.read_since(0).unwrap());
            },
        );

        cleanup(&root);
    }
    group.finish();
}

criterion_group!(benches, bench_append, bench_checkpoint, bench_read_since);
criterion_main!(benches);
