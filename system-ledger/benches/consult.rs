//! criterion benchmarks for the kernel-side ledger consultation
//! latency budget. Master 4b deliverable from
//! `~/Foundry/clones/project-system/.claude/inbox-archive.md`.
//!
//! Run with: `cargo bench -p system-ledger`. Numbers surface in
//! `target/criterion/<bench>/report/`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ed25519_dalek::{Signer, SigningKey};
use system_core::{
    Capability, CapabilityType, Checkpoint, LedgerAnchor, NoteSignature, Right,
    SignedCheckpoint,
};
use system_ledger::{InMemoryLedger, LedgerConsumer};

fn keypair(seed: u8) -> (SigningKey, [u8; 32]) {
    let sk = SigningKey::from_bytes(&[seed; 32]);
    let pk = sk.verifying_key().to_bytes();
    (sk, pk)
}

fn fixture_capability() -> Capability {
    Capability {
        cap_type: CapabilityType::Endpoint,
        rights: vec![Right::Invoke, Right::Read],
        expiry_t: None,
        witness_pubkey: None,
        ledger_anchor: LedgerAnchor {
            origin: "foundry.bench.cap-ledger".to_string(),
            tree_size: 1,
            root_hash: [0xAA; 32],
        },
    }
}

fn signed_checkpoint(
    tree_size: u64,
    root_byte: u8,
    signers: &[(&str, &SigningKey)],
) -> SignedCheckpoint {
    let cp = Checkpoint {
        origin: "foundry.bench.cap-ledger".to_string(),
        tree_size,
        root_hash: [root_byte; 32],
        extensions: vec![],
    };
    let body = cp.body_bytes();
    let signatures = signers
        .iter()
        .map(|(name, sk)| {
            let pk = sk.verifying_key().to_bytes();
            let key_hash = NoteSignature::derive_key_hash(name, &pk);
            let sig = sk.sign(&body).to_bytes();
            NoteSignature {
                signer_name: name.to_string(),
                key_hash,
                signature: sig,
            }
        })
        .collect();
    SignedCheckpoint {
        checkpoint: cp,
        signatures,
    }
}

fn bench_capability_hash(c: &mut Criterion) {
    let cap = fixture_capability();
    c.bench_function("Capability::hash", |b| {
        b.iter(|| black_box(cap.hash()))
    });
}

fn bench_verify_signer_single(c: &mut Criterion) {
    let (sk, pk) = keypair(0x11);
    let signed = signed_checkpoint(100, 0xAA, &[("apex", &sk)]);
    c.bench_function("SignedCheckpoint::verify_signer (1-sig)", |b| {
        b.iter(|| black_box(signed.verify_signer("apex", &pk).unwrap()))
    });
}

fn bench_verify_apex_handover(c: &mut Criterion) {
    let (sk_old, pk_old) = keypair(0x11);
    let (sk_new, pk_new) = keypair(0x22);
    let signed = signed_checkpoint(
        100,
        0xCD,
        &[("apex-old", &sk_old), ("apex-new", &sk_new)],
    );
    c.bench_function("SignedCheckpoint::verify_apex_handover (2-sig)", |b| {
        b.iter(|| {
            black_box(
                signed
                    .verify_apex_handover("apex-old", &pk_old, "apex-new", &pk_new)
                    .unwrap(),
            )
        })
    });
}

fn bench_cache_hit(c: &mut Criterion) {
    let (sk, _pk) = keypair(0x11);
    let mut ledger = InMemoryLedger::new();
    // Fill cache with 64 entries; the target lookup is the LAST one
    // inserted (most-recent — best cache-hit case).
    for h in 0..64u64 {
        ledger.cache.insert(signed_checkpoint(h, h as u8, &[("apex", &sk)]));
    }
    c.bench_function("cache lookup_by_tree_size (hit, most-recent)", |b| {
        b.iter(|| black_box(ledger.cache.lookup_by_tree_size("foundry.bench.cap-ledger", 63)))
    });
}

fn bench_cache_miss(c: &mut Criterion) {
    let (sk, _pk) = keypair(0x11);
    let mut ledger = InMemoryLedger::new();
    for h in 0..64u64 {
        ledger.cache.insert(signed_checkpoint(h, h as u8, &[("apex", &sk)]));
    }
    c.bench_function("cache lookup_by_tree_size (miss, full scan)", |b| {
        b.iter(|| black_box(ledger.cache.lookup_by_tree_size("foundry.bench.cap-ledger", 99999)))
    });
}

fn bench_consult_capability_allow(c: &mut Criterion) {
    let (sk, pk) = keypair(0x11);
    let mut ledger = InMemoryLedger::new();
    ledger.apex.record_genesis("apex", pk, 0).unwrap();
    let cap = fixture_capability();
    let root = signed_checkpoint(5, 0xAA, &[("apex", &sk)]);
    c.bench_function("consult_capability (Allow path; 1-sig apex verify)", |b| {
        b.iter(|| {
            black_box(
                ledger
                    .consult_capability(&cap, &root, 1000, None)
                    .unwrap(),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_capability_hash,
    bench_verify_signer_single,
    bench_verify_apex_handover,
    bench_cache_hit,
    bench_cache_miss,
    bench_consult_capability_allow,
);
criterion_main!(benches);
