---
artifact: topic
schema: foundry-draft-v1
title: "The Three-Binary Architecture: os-console, os-totebox, os-orchestration"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, claim-43, claim-49, claim-23, claim-52]
research_trail:
  sources: [BRIEF-OS-FAMILY.md, BRIEF-sovereign-os-family-master-plan.md, BRIEF-os-console-hypervisor.md, conventions/architecture-layer-catalog.md]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: agent-research + DOCTRINE review + project-registry cross-check
---

# The Three-Binary Architecture: os-console, os-totebox, os-orchestration

Totebox Orchestration is delivered through three distinct binary operating environments.
Each has a distinct role, a distinct deployment target, and a distinct set of hosted
applications. Together they form a complete system for sovereign data management.

---

## Overview

```
                    ┌──────────────────────────────────────┐
                    │           HOST MACHINE               │
                    │                                      │
                    │         os-console                   │
                    │   Operator Terminal Surface          │
                    │   cartridges: app-console-* (F2–F12)│
                    │   keyboard-native TUI                │
                    │   machine-authorized via F11         │
                    └──────────────┬───────────────────────┘
                                   │  machine capability token
                                   │  (authorized at F11 pairing)
                                   ▼
                    ┌──────────────────────────────────────┐
                    │       CUSTOMER HARDWARE / VM         │
                    │                                      │
                    │         os-totebox                   │
                    │   Sovereign WORM Data Vault          │
                    │   services: service-* (Ring 1+2)    │
                    │   no shell; no root process          │
                    │   WORM append-only ledger            │
                    │   Tier A local AI (service-slm)      │
                    └──────────────┬───────────────────────┘
                                   │  capability-gated federation
                                   │  (Totebox grants derived caps)
                                   ▼
                    ┌──────────────────────────────────────┐
                    │        VENDOR INFRASTRUCTURE         │
                    │                                      │
                    │      os-orchestration                │
                    │   Stateless Aggregation Layer        │
                    │   apps: app-orchestration-*          │
                    │   Yo-Yo GPU fleet (Tier B)           │
                    │   multi-Totebox commercial flows     │
                    │   holds no archive keys              │
                    └──────────────────────────────────────┘
```

---

## os-totebox: Sovereign WORM Data Vault

os-totebox is the customer-side deployment. It runs on hardware under the customer's
physical control — a NUC-class machine, a GCP VM, or a private server. Its function is
to host the Ring 1 and Ring 2 services:

**Ring 1 — Boundary Ingest:**
- `service-fs` — WORM append-only filesystem; every write is a ledger entry
- `service-input` — structured input ingestion from os-console operators
- `service-extraction` — entity extraction pipeline from email and document payloads
- `service-egress` — controlled outbound data flow

**Ring 2 — Deterministic Processing:**
- `service-content` — DataGraph; knowledge graph over all ingested entities
- `service-people` — personnel and identity ledger
- `service-email` — email archive bridge (Microsoft Exchange → Maildir)
- `service-slm` — Doorman inference gateway (Tier A local AI on OLMo 7B)

Ring 3 (optional AI, Tier B+ inference) is hosted by os-orchestration, not os-totebox.
A Totebox ships with Rings 1 and 2. Ring 3 is an opt-in paid tier.

**Intended final form (Phase H2, planned):** os-totebox boots bare metal or as a VM
into a seL4 Microkit image. Each service runs as a seL4 Protection Domain. No Linux
shell. No package manager. No init system. The service-fs PD holds the WORM storage
device capability; no other PD can reach the physical storage directly.

---

## os-console: Operator Terminal Surface

os-console runs on the operator's personal machine. It is the interface through which
the operator interacts with Totebox services. It does not store data — it renders views
of data that lives in os-totebox.

os-console is deployed as a node instance (naming prefix: `node-console-*`). The
operator boots it or runs it as an application. It connects outbound to the Totebox,
authenticates via machine pairing (F11), and presents the authorized cartridge set.

**Cartridges (app-console-*) are the rendering unit:**

| Key | Cartridge | Backend |
|---|---|---|
| F2 | People | service-people :9091 |
| F3 | Email | service-email |
| F4 | Content | service-content :9081 |
| F6 | Bookkeeper | service-bookkeeper |
| F9 | SLM | service-slm :9080 |
| F11 | System / Pairing | pairing-server |
| F12 | Input / Audit | service-input |

**Intended final form (Phase H2, planned):** os-console boots as a seL4 Microkit image.
Each cartridge runs as a seL4 Protection Domain (the "browser tab" isolation model).
moonshot-hypervisor provides the host-side VMM on Linux (KVM) and macOS
(Hypervisor.framework).

---

## os-orchestration: Stateless Aggregation Layer

os-orchestration (legacy name: os-interface, rename in progress) is the vendor-side paid
tier. It aggregates across multiple customer Toteboxes and provides commercial services
that require cross-Totebox data access.

os-orchestration is **stateless**: it holds no archive keys, writes no data, owns no
ledger. It computes over capability-granted views of Totebox data.

**Hosted applications (app-orchestration-*):**

| App | Role | Port |
|---|---|---|
| app-orchestration-slm | Yo-Yo GPU broker (Tier B); multi-Totebox SLM federation | :9180 |
| app-orchestration-market | Data marketplace UI (planned, Doctrine #52) | — |
| app-orchestration-exchange | Ad exchange gateway (planned, Doctrine #52) | — |
| app-orchestration-gis | Continental-scale GIS processing | deployed |
| app-orchestration-bim | Multi-archive BIM federation | — |

**Commercial model (Doctrine #23):** A single Totebox runs Rings 1–2 at no licensing
cost. Connecting to os-orchestration for Tier B inference, data marketplace, or BIM
aggregation is the paid tier.

**Intended final form (Phase H2, planned):** os-orchestration boots as a seL4 Microkit
image. A capability-broker Protection Domain holds all cross-Totebox endpoint
capabilities. No app-orchestration-* PD can directly contact a Totebox service without
routing through the capability-broker PD.

---

## Geometric Protection Across All Three

The Geometric Protection model applies at each layer with the same mechanism but a
different scope:

| Layer | PD boundary | What it bounds |
|---|---|---|
| os-console | Per cartridge (F2, F3, F4...) | One cartridge cannot read another's data |
| os-totebox | Per service (service-people, service-slm...) | One service cannot escalate to another |
| os-orchestration | Per app + capability-broker chokepoint | One org's data cannot reach another org |

The seL4 kernel provides the same formal proof at each layer. The capability substrate
(system-core, system-ledger) provides the Rust-language interface to these proofs.
moonshot-sel4-vmm provides the PD runtime that makes Rust code run inside each
Protection Domain.

The three-binary architecture is one system — one seL4 proof, one capability substrate,
one moonshot-toolkit build pipeline — deployed at three distinct tiers.

---

## Deployment Topology

```
TOTEBOX ORCHESTRATION LAYER
gateway-orchestration-command-1   [os-orchestration hub]
  │
  ├── cluster-totebox-*     (os-totebox)     outbound capability grant
  ├── node-console-*        (os-console)     outbound machine pairing
  └── gateway-orchestration-[gis/bim/slm]-N  outbound federation
```

Each `cluster-totebox-*` is one customer's Sovereign WORM Data Vault. Each
`node-console-*` is one operator's terminal. `gateway-orchestration-*` instances are
named deployments of specific os-orchestration app-orchestration-* applications.

---

## What This Means for the SMB Operator

A small business with no IT department runs:
- One `cluster-totebox-1` — on a NUC under the server room desk or a $7/month GCP VM
- One or more `node-console-*` — one per staff member who needs Totebox access
- Optional: connection to `gateway-orchestration-slm-1` for Tier B AI capability

The operator does not manage servers. They do not configure firewalls. They boot the
Totebox, boot os-console, scan a QR code, and the system is running. If they cancel the
optional Tier B subscription, their Totebox (and all their data) continues operating
independently. There is no mandatory cloud dependency.
