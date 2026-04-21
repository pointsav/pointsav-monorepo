# Testing

Testing strategy for service-slm. Short version: write property tests
for invariants, integration tests for protocols, smoke tests for
compile-time guarantees.

## The four levels

### 1. Unit tests

In-crate, `#[cfg(test)]` modules. Scope: a single function or type.
Must be hermetic — no network, no filesystem writes outside a tempdir.

### 2. Property tests

Using `proptest` (see [`Cargo.toml`](../../Cargo.toml) workspace deps).
Use these wherever a contract can be expressed as a universally
quantified statement — for example:

- `sanitise` and `rehydrate` compose to the identity on any valid
  payload.
- `ModuleId::new` accepts every string it produces via its
  `Display` implementation.
- Ledger CSV round-trip is loss-free.

Property tests are not optional for any type or function that embodies
an invariant. If the invariant is "the protocol never leaks data,"
write the property test first.

### 3. Integration tests

In `crates/<name>/tests/`. Scope: cross-module within a crate. Allowed
to use the filesystem (tempdir), spawn subprocesses, and bind local
ports. Never hit the real network.

### 4. End-to-end tests

In `tests/` at the workspace root (not yet populated). Scope:
cross-crate, full binary invocations. Allowed to spawn a local
Mooncake-mock sidecar, a local `httpbin`, or a fake Cloud Run endpoint.
Gated behind a `--ignored` attribute or a feature flag; must not run
in the default `cargo test` invocation if they take more than a few
seconds.

## What CI runs

CI runs `cargo test --workspace --all-features` on every PR, across
Linux and macOS. This covers levels 1–3. Level 4 is gated.

## What CI does not run (yet)

- Fuzz tests. Recommended for `slm-doorman` once sanitisation rules
  stabilise; add via `cargo-fuzz`.
- Mutation tests (`cargo-mutants`). Useful for verifying test
  coverage on the ledger writer and doorman protocol.
- Benchmarks. We will add `criterion` benchmarks for the KV cache
  hash and the ledger writer when either approaches a hot path.

## Writing a good property test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn sanitise_then_rehydrate_is_identity(
        payload in any::<RawPayload>(),
    ) {
        let (sanitised, table) = sanitise(&payload).unwrap();
        let rehydrated = rehydrate(&sanitised, &table).unwrap();
        prop_assert_eq!(payload, rehydrated);
    }
}
```

Note: the generator `any::<RawPayload>()` must produce the full
domain of valid payloads. If it produces only trivial inputs, the test
is worth nothing. Use `proptest`'s `strategy` combinators to describe
the real shape of inputs.

## What not to test

- Serde round-trips of types that are purely mechanical (pure struct
  of primitives) — the derive handles it.
- Third-party crates. Their tests are not our tests.
- Every combination of feature flags — pick one meaningful combination
  per platform and test that.
