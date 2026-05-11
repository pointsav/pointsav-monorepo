---
schema: foundry-cluster-manifest-v1
cluster_name: project-system
cluster_branch: cluster/project-system
created: 2026-04-26
state: active

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: 28 projects in scope — 14 system-* (system-substrate, system-core, system-security, system-interface, system-substrate-broadcom, system-substrate-freebsd, system-substrate-wifi, system-network-interface, system-udp, system-resolution, system-audit, system-verification, system-slm, system-gateway-mba); 9 moonshot-* (moonshot-kernel, moonshot-hypervisor, moonshot-sel4-vmm, moonshot-toolkit, moonshot-database, moonshot-index, moonshot-network, moonshot-protocol, moonshot-gpu); 5 vendor-quarantine (vendor-sel4-kernel, vendor-virtio, vendor-gpu-drivers, vendor-linux-systemd, vendor-wireguard). Codifies Doctrine claims #33 (The Capability Ledger Substrate) + #34 (The Two-Bottoms Sovereign Substrate).
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolders:
        - fleet-infrastructure-onprem/
        - fleet-infrastructure-cloud/
        - fleet-infrastructure-leased/
      tenant: pointsav
      purpose: vendor-side-showcase-public-facing-Customer-and-Community-Members; receives GUIDE-substrate-rollout-{onprem,cloud,leased}.md drafted Task-side; public bundle per Doctrine §VIII
      status: leg-pending — Task drafts GUIDEs in workspace-root staging; Master rehomes to catalog
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolders:
        - fleet-infrastructure-onprem/
        - fleet-infrastructure-cloud/
        - fleet-infrastructure-leased/
      tenant: woodfine
      purpose: customer-tier-operational-mirror; substrate updates flow here as Customer-side variant content
      status: leg-pending — Task drafts as cluster work; Master coordinates §11 cross-repo rehoming
  deployment:
    - shape: informational-all-instances
      tenant: all
      purpose: substrate-shaped-cluster — substrate touches every numbered runtime under ~/Foundry/deployments/. Real-time feedback loop for every cluster's Task when substrate breaks.
      currently_running:
        - ~/Foundry/deployments/cluster-totebox-corporate-1/
        - ~/Foundry/deployments/media-knowledge-documentation-1/
      future: every new instance provisioned by any cluster
      status: doctrine-§IV.d-sub-rule-applied (see RESEARCH-system-substrate.md §1.2 + DOCTRINE.md claim #34)
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-system/.claude/drafts-outbound/
      gateway: project-language Task
      planned_topics:
        - topic-merkle-proofs-as-substrate-primitive.md  # Phase 1A.4 + 1A.5 milestone (inclusion + consistency); RFC 9162 §2 grounding
        - topic-capability-ledger-substrate.md           # Doctrine claim #33 architecture decision; Phase 1A structurally complete
        - topic-two-bottoms-sovereign-substrate.md       # Doctrine claim #34 (seL4-native + NetBSD-compat composition); future-leaning as Phase 2 lands
      status: leg-pending — first skeleton (topic-merkle-proofs-as-substrate-primitive) staged in drafts-outbound/ this commit; substantive bulk follows in milestone N+1; project-language Task is the editorial gateway per cluster-wiki-draft-pipeline.md

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: system-* + moonshot-* + vendor-quarantine projects
  - repo: pointsav-fleet-deployment
    role: sibling
    path: pointsav-fleet-deployment/
    upstream: vendor/pointsav-fleet-deployment
    focus: Vendor-side fleet-infrastructure-* showcase catalog
  - repo: woodfine-fleet-deployment
    role: sibling
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
    focus: Customer-tier fleet-infrastructure-* mirror

trajectory_capture: enabled (L1 capture-edit hook installed in all three sub-clones at provisioning)

adapter_routing:
  trains:
    - cluster-project-system     # own cluster adapter (substrate-authoring skill — kernel + system layer + reproducible-build harness + capability-ledger primitives)
    - engineering-pointsav       # Vendor engineering corpus
    # NOTE: no tenant-* adapter — system-* is platform substrate, not per-tenant content
  consumes:
    - constitutional-doctrine    # always
    - engineering-pointsav       # always — Vendor knowledge
    - cluster-project-system     # own cluster context
    - role-task                  # current role
---

# Cluster manifest — project-system

Multi-clone N=3 cluster (third multi-clone cluster authored under
Doctrine v0.0.2 §IV.c). Three sub-clones in one cluster directory;
one Task session writes to one `.git/index` at a time.

## Scope

The substrate beneath every `os-*` operating system family, every
`service-*` and `app-*` deployment, and every Customer Totebox
runtime in Foundry. Codifies Doctrine claims #33 (The Capability
Ledger Substrate) + #34 (The Two-Bottoms Sovereign Substrate).

### Native bottom: seL4 (today) → moonshot-kernel (future)
- AArch64-first hardware
- seL4 v15.0.0 (31 March 2026); Microkit 2.2.0; rust-sel4 4.0.0
- moonshot-kernel: long-horizon no_std Rust port; AArch64-first

### Compat bottom: NetBSD
- Veriexec verified-image boot; `build.sh` offline reproducibility;
  rump kernels for IT/OT bridge
- BSD 2-clause; independent foundation; zero hyperscaler entanglement
- 57-port hardware breadth

### Linux NOT in doctrine
- Unsupported community-tier explore-it-anywhere fallback only
- Not a Foundry substrate; not in trust chain; not in any showcase
  GUIDE

## Branch

`cluster/project-system` in each sub-clone (created 2026-04-26
from local upstream `main`).

## Remotes (within each sub-clone)

### pointsav-monorepo + pointsav-fleet-deployment (engineering-tier)

- `origin` — canonical via admin SSH alias
  (`github.com-pointsav-administrator`)
- `origin-staging-j` — Jennifer's staging-tier mirror
  (`github.com-jwoodfine`)
- `origin-staging-p` — Peter's staging-tier mirror
  (`github.com-pwoodfine`)

### woodfine-fleet-deployment (customer-tier)

- `origin` — canonical via admin SSH alias
  (`github.com-woodfine-administrator`)
- No staging mirrors (customer-tier flow is Vendor → Customer; no
  pre-canonical staging tier on Customer side).

Push policy: staging-tier only for engineering sub-clones; no
push to `origin` (canonical) — Stage 6 promotion is the canonical-
tier path. customer-tier sub-clone receives propagation from
factory-release-engineering, not direct pushes from Task.

## Required Phase 1 reading

`~/Foundry/RESEARCH-system-substrate.md` (workspace-root staging).
Read end-to-end before Phase 1A / Phase 1B work. The synthesis
(§§1-7) carries the strategic narrative; the appendices (§8) carry
verbatim research with sources to cite when writing technical
documentation.

## Trajectory capture

Enabled. L1 capture hook installed in all three sub-clones; every
commit on `cluster/project-system` enters
`~/Foundry/data/training-corpus/engineering/project-system/` for
future cluster-adapter training.

## Cross-cluster coordination

- **project-slm Task**: may eventually need substrate APIs in
  `slm-doorman` for capability-ledger integration (apprenticeship
  verdict ledger → capability ledger primitive). Surface to Master
  via outbox when capability-ledger primitive APIs stabilize.
- **project-data Task**: WORM-ledger substrate APIs are this
  cluster's responsibility; existing `worm-ledger-design.md`
  convention is the contract; service-fs may consume the
  Foundry-canonical primitive once Phase 1A lands.
- **project-knowledge + project-orgcharts Task sessions**: consume
  `os-*` runtime; substrate updates from this cluster flow to
  their deployment instances per the all-instances deployment leg.
  Coordination automatic (substrate updates land for everyone);
  surface only if a substrate change breaks consumer assumptions.

## Mailbox

- Inbox: `~/Foundry/clones/project-system/.claude/inbox.md`
- Outbox: `~/Foundry/clones/project-system/.claude/outbox.md`
- Trajectory log: `~/Foundry/clones/project-system/.claude/trajectory-log.md`
  (created on first capture)

## State at provisioning (2026-04-26)

| Item | State |
|---|---|
| Cluster directory | Created |
| Three sub-clones | Cloned from local upstream (~553 MB total) |
| `cluster/project-system` branch | Created in each sub-clone |
| Remotes (admin alias + staging where applicable) | Configured |
| L1 capture hook | Installed in each sub-clone |
| First-session Task brief | Written (this commit's `inbox.md`) |
| Phase 1 status | Pending — Task picks up next session |

---

*Provisioned 2026-04-26 in workspace v0.1.19 / Doctrine v0.0.8.*
