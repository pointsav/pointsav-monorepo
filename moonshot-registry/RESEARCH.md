# RESEARCH — moonshot-registry

**Status:** Research phase. No replacement code written.
**Registered:** 2026-06-14
**Priority:** MEDIUM
**"We Own It" target:** OCI Distribution Spec v1.1.0 registry client + server

---

## Dependency replaced

### oci-client Rust crate
- **Version to borrow (Phase 2):** 0.12+ (pinned when added in Phase 2)
- **Licence:** Apache 2.0
- **SLOC:** ~8,000 lines
- **What it does:** OCI Distribution Spec client — pulls BIM object blobs from
  the registry endpoint in `app-privategit-bim` by content digest. Handles
  manifest fetching, layer pulling, and digest verification.
- **Why borrowed:** Phase 2 timeline. Phase 1 ships without OCI distribution;
  Phase 2 adds it. Borrowing `oci-client` gets Phase 2 to production; owning
  the full client+server unifies the stack and removes a dependency boundary.
- **Replacement complexity:** Medium. OCI Distribution Spec v1.1.0 is a well-defined
  HTTP API (~10 routes). The server side is written directly in Axum (app-privategit-bim);
  the client library is the piece that benefits from `moonshot-registry`.

---

## OCI Distribution Spec v1.1.0 route coverage (server side)

The server is written natively in Axum inside `app-privategit-bim` (Phase 2).
`moonshot-registry` provides the shared client library and eventually a standalone
registry binary for isolated BIM object vaults.

| Route | Method | Purpose |
|---|---|---|
| `/v2/` | GET | API version check (registry check) |
| `/v2/<name>/manifests/<reference>` | GET/HEAD | Pull manifest by tag or digest |
| `/v2/<name>/manifests/<reference>` | PUT | Push manifest |
| `/v2/<name>/blobs/<digest>` | GET/HEAD | Pull blob layer |
| `/v2/<name>/blobs/uploads/` | POST | Initiate push (chunked upload) |
| `/v2/<name>/blobs/uploads/<uuid>` | PUT | Complete push |
| `/v2/<name>/tags/list` | GET | List tags for a repository |
| `/v2/<name>/referrers/<digest>` | GET | OCI 1.1 referrer API (attestations) |

All routes are Axum handlers in `app-privategit-bim/src/routes/api.rs` (Phase 2).
`moonshot-registry` will extract and generalize these as a reusable crate.

---

## BIM object OCI layout

```
Image manifest (application/vnd.oci.image.manifest.v1+json):
  annotations:
    org.opencontainers.image.title: "Private Office Small"
    org.bim.pbs-1.version: "1.0"
    org.bim.ifc.class: "IfcSpace"
    org.bim.category: "key-plan"
  layers:
    - mediaType: application/vnd.bim.ifc+octet-stream
      digest: sha256:abc...
      annotations: { title: "model.ifc" }
    - mediaType: application/vnd.bim.dxf+octet-stream
      digest: sha256:def...
      annotations: { title: "model.dxf" }
    - mediaType: image/svg+xml
      digest: sha256:ghi...
      annotations: { title: "thumbnail.svg" }
    - mediaType: application/vnd.bim.pbs1+json
      digest: sha256:jkl...
      annotations: { title: "object.json" }
```

OCI 1.1 referrer API is used to attach IDS validation results as attestation
artifacts pointing to the subject manifest digest.

---

## Prior art surveyed

| Library | Licence | What it does | Notes |
|---|---|---|---|
| `oci-client` (rust) | Apache 2.0 | OCI registry client | Current borrow candidate (Phase 2) |
| `oci-spec` (rust) | Apache 2.0 | OCI type definitions only | Could use for types even if writing own client |
| `dkregistry` (rust) | Apache 2.0 | Docker Registry v2 client | Older; OCI 1.1 support partial |
| `crane` (Go) | Apache 2.0 | OCI registry CLI tool | Not Rust; reference implementation |

The `oci-spec` crate for type definitions is worth retaining even when writing
the custom client — the manifest and descriptor structs are well-tested.

---

## Related

- BRIEF-bim-objects-system.md §C — PBS-1 framework, distribution protocol
- `app-privategit-bim/src/routes/api.rs` — Phase 2 OCI server routes
- `tool-keyplan` — the build pipeline that pushes BIM objects to the registry
- `moonshot-bim-parser` — provides IFC layer content for registry push
