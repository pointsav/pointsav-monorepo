# os-totebox integration

service-slm is a prototype component of os-totebox, the eventual
PointSav archive appliance. This document summarises what that means
for this service. The authoritative integration spec lives in the
os-totebox project docs; we reference relevant rows from
[SLM-STACK §6](../../specs/SLM-STACK.md) and
[ADR-0004](../adr/0004-flat-binary-no-mesh.md).

## What os-totebox needs from service-slm

- **One systemd unit.** Not a pod, not a set of sidecars. See
  [ADR-0004](../adr/0004-flat-binary-no-mesh.md).
- **Predictable startup.** Static binary, no interpreter warm-up.
  Target: sub-second to "listening on unix socket."
- **Deterministic memory.** Bounded by configuration, not by
  workload. Totebox Laptop-A has ~550 MB headroom; service-slm must
  fit with its inference runtime.
- **Signed updates.** Sigstore keyless signing, SLSA attestation, SBOM.
  See [SECURITY.md](../../SECURITY.md).
- **Cross-compile to aarch64.** Toteboxes ship on ARM silicon;
  x86_64 is for development only.

## What service-slm needs from os-totebox

- A writable data directory for `outbound/`, `inbound/`, `log/`,
  `compute/`, `memory/`, `ledger/`.
- A path to its configuration file (TOML).
- Read access to the Secret Manager equivalent for API keys.
- Network access to: the Cloud Run GPU node, the Claude API,
  LadybugDB, Mooncake master.
- A cgroup-enforced RAM ceiling so the process fails closed rather
  than causing host OOM.

## How it ships

- Container image for Cloud Run GPU node.
- Native binary, cross-compiled to ARM and x86_64, for Totebox hosts.
- systemd unit file (to be added in Phase 3).
- Update mechanism via signed artefacts per [SECURITY.md](../../SECURITY.md).

## Phases

- Phase 2 (current): Rust rewrite running standalone. Not yet
  os-totebox integrated.
- Phase 3: cross-compile, systemd unit, signed release pipeline
  exercised, fit verification on Laptop-A.
- Phase 4: optional open-source release.

See [ROADMAP.md](../../ROADMAP.md) for the milestone list.
