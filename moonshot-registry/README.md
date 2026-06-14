<div align="center">

# moonshot-registry

[ Leer en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Moonshot Initiative — `moonshot-*` family
**Version:** 0.1.0
**Status:** Reserved-folder — research phase
**Cluster:** `cluster/project-bim` per workspace `PROJECT-CLONES.md`
**Priority:** MEDIUM

---

## What this replaces

This crate is the planned internal implementation of an OCI Distribution Spec
v1.1.0 registry server and client, replacing the borrowed `oci-client` Rust
crate used in Phase 2 of `app-privategit-bim`.

The BIM Objects distribution model (PBS-1 framework, Phase 2) uses the OCI
Distribution Spec to address BIM objects by content hash:

```
woodfine/key-plans/private-office:small@sha256:abc123
```

Any AEC developer can pull a specific BIM object version using standard OCI
tooling (`docker pull`, `skopeo`, `crane`) or via the `moonshot-registry`
client library.

## Why OCI Distribution Spec

1. **Open specification** (OCI, Apache 2.0 licence) — no vendor lock-in
2. **Content-addressed** — every object version is cryptographically identified
3. **Offline-capable** — OCI clients work without internet once content is cached
4. **Tool ecosystem** — standard `docker pull` workflow already understood by AEC firms
5. **Layer model** — IFC-SPF, DXF, SVG, and JSON manifest as separate OCI layers

## Architecture (planned)

```
moonshot-registry
  ├── server/     — Axum OCI Distribution Spec v1.1.0 endpoint (/v2/* routes)
  │                 Hosted by app-privategit-bim on os-privategit
  └── client/     — Pull/push API for BIM object blobs (replaces oci-client crate)
                    Used by tool-keyplan and external AEC developers
```

BIM object OCI layout:
```
manifest.json (OCI Image Manifest)
  └── layers:
       ├── layer 0: model.ifc    (IFC-SPF, canonical)
       ├── layer 1: model.dxf    (DXF block export)
       ├── layer 2: thumbnail.svg (zone diagram)
       └── layer 3: object.json  (PBS-1 DTCG manifest)
```

## Timeline

**Medium horizon (Phase 2 of app-privategit-bim, 2027).** Phase 1 of the
rewrite borrows `oci-client` for client-side pulls. The registry server
endpoint in `app-privategit-bim` is written in Axum directly (OCI spec is
straightforward: ~10 HTTP routes). `moonshot-registry` unifies the client
and server once the spec coverage and edge cases are fully understood.

## Cross-references

- `app-privategit-bim/src/routes/api.rs` — Phase 2 OCI /v2/* routes
- `tool-keyplan` — the build pipeline that pushes PBS-1 objects to the registry
- `moonshot-bim-parser` — provides the IFC layer content
- BRIEF-bim-objects-system.md §C — PBS-1 framework, distribution layer

---

*© 2026 PointSav Digital Systems™.*
