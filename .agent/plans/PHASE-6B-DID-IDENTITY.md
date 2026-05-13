# Phase 6B — Portable DID Identity (`did:web:` + WebFinger)

> Status: **gated — needs BP6 design decision from operator**
> Created: 2026-05-13 | Author: task@project-knowledge

## Context

Phase 6A shipped 2026-05-13 (slug normalisation, redirect hatnote, inject_wiki_prefixes fix).
Phase 6B is the next planned milestone per ARCHITECTURE.md §6.

## Scope (from ARCHITECTURE.md §6 / Wikipedia Parity plan)

1. **`did:web:` identity for authors** — each user account gets a `did:web:` DID
   hosted at `/.well-known/did.json` on the wiki domain. Allows cross-instance
   author attribution without a central registry.

2. **WebFinger** (`GET /.well-known/webfinger?resource=acct:user@domain`) —
   ActivityPub-compatible discovery endpoint returning the user's DID + profile URL.
   Enables federation lookups from Mastodon/ActivityPub clients.

3. **`did:` field in article JSON-LD** — `author` in `schema:TechArticle` becomes a
   `did:web:` URI rather than a plain string.

## Gate: BP6 design decision

Operator must answer before implementation starts:

| Decision | Options | Default if unasked |
|---|---|---|
| DID key type | Ed25519 (fast, small) vs P-256 (widely supported) | Ed25519 |
| Key storage | redb (existing store) vs filesystem | redb |
| DID rotation | Supported vs single static key per user | static (simpler) |
| WebFinger scope | Only wiki users vs full ActivityPub actor objects | wiki users only |
| Cross-instance federation | In Phase 6B or deferred to Phase 7 | deferred |

## Rough file scope

| File | Change |
|---|---|
| `src/server.rs` | Add `GET /.well-known/did.json` + `GET /.well-known/webfinger` routes |
| `src/users.rs` | Add `did_key` field to `User` struct; generate on account creation |
| `src/links.rs` or new `src/did.rs` | DID document generation + key management |
| `src/render.rs` → `jsonld_for_topic()` | Replace plain author string with `did:web:` URI |
| `tests/did_test.rs` | New integration tests |

## Dependencies

- No new crates needed for Ed25519 (ring is already in Cargo.toml; has Ed25519 support)
- WebFinger response format is JSON — serde_json already present
