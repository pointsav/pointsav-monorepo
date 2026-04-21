# slm-memory-kv

Ring 2 of the three-ring memory model: working memory that survives
teardown.

Mooncake Store is the state-of-the-art distributed KV cache pool. This
crate is the Rust client that speaks its wire protocol. **We never link
Mooncake as a library.** It runs as a sidecar; we talk to it over HTTP
(metadata) and RDMA/TCP (data). This is the correct architectural
boundary for a C++ dependency.

Specification: [YOYO-COMPUTE §3](../../specs/YOYO-COMPUTE.md).

## What lives here

- The Mooncake Store metadata client.
- The LMCache config generator.
- The KV block hash computation (must be deterministic across processes
  — `PYTHONHASHSEED` equivalent per YOYO-COMPUTE §3).
- `moduleId` namespace isolation.
- The stats collector writing to `memory/kv/stats.csv`.

## What does not live here

- Any Mooncake internals. We depend on the wire protocol only.
- The Python LMCache bridge used in Phase 1. Phase 2 calls mistral.rs
  directly with the KV connector config.
