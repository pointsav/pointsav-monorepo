---
schema: foundry-cluster-manifest-v1
cluster: project-infrastructure
cluster_name: project-infrastructure
cluster_branch: main
created: 2026-06-14
state: active
slm_endpoint: http://localhost:9180
module_id: infrastructure

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        VM fleet controller stack — service-vm-fleet (:9203 advisory placement),
        service-vm-host (:9220 per-node QEMU agent), service-vm-tenant (:9221 tenant proxy);
        app-orchestration-slm (:9180 Yo-Yo broker chassis, DOCTRINE #23 Tier B);
        system-vm-fleet-types (shared wire types); all deployed on vault-privategit-source-1.
      status: active (commits 5e851ecc / 2717fbce / a71e89f0 / 49bd534d; Stage 6 pending)
  customer:
    - status: leg-pending
      note: >
        No customer-tier deliverable defined yet. The VM fleet is internal infrastructure.
        A woodfine-fleet-deployment catalog entry will be added when a customer-facing
        deployment guide is written.
  deployment:
    - status: active
      note: >
        Four services on vault-privategit-source-1:
          service-vm-fleet    → port 9203 (local-vm-fleet.service)
          service-vm-host     → port 9220 (local-vm-host.service)
          service-vm-tenant   → port 9221 (local-vm-tenant.service)
          orchestration-slm   → port 9180 (local-orchestration-slm.service)
        Fleet nodes: laptop-a-1 (:9220 via WireGuard), laptop-b-1 (:9220 via WireGuard).
  wiki:
    - status: leg-pending
      note: >
        TOPIC and GUIDE drafts for the VM stack not yet written.
        Planned: TOPIC on PPN VM resource pool architecture; GUIDE on fleet operations.
