---
artifact: foundry-draft-v1
type: TOPIC
slug: topic-sel4-capability-topology
title: "seL4 Capability Topology — The Formal Security Model"
status: draft
created: 2026-06-29
author: totebox@project-infrastructure
route_to: project-editorial
language_protocol: PROSE-*
research_source: geometric-capabilities-research-2026-06-29
research_claim: "Capability graph topology determines all permitted information flow — formally proved invariant"
research_method: seL4 Foundation documentation + Miller (2000) + Drossopoulou (2016) + Fuchsia docs + Murray et al. (2013)
research_verification: cross-checked against seL4 proof repository + published verification papers
language: en
---

# seL4 Capability Topology — The Formal Security Model

## What capabilities are

In the seL4 microkernel, every object access is mediated by a **capability** — an
unforgeable token that grants a specific right to a specific object. A process that
does not hold a capability to an object cannot observe it, modify it, or call it.
There is no ambient authority. There is no root privilege at the kernel level.

Capabilities are stored in **CSpaces** — per-process capability space tables managed by
the seL4 kernel. The CSpace is a bounded directed graph: capability pointers are edges;
kernel objects (threads, memory regions, IPC endpoints, notification objects) are nodes.

## The topology invariant

The collection of all CSpaces in a running seL4 system forms a directed graph called the
**capability topology**. The seL4 kernel enforces one invariant above all others:

> *Only connectivity begets connectivity.*
> — Jonathan Shapiro and Mark Miller, 2000

In formal terms: if component A has no capability to reach component B (directly or
transitively through any chain of intermediate capabilities), then A cannot obtain
information from B, cannot modify B's state, and cannot cause B to take any action.

This invariant is **machine-checked**. The seL4 team at the seL4 Foundation has published
formal proofs in the Isabelle/HOL proof assistant covering:

1. **Functional correctness** (C-level, all architectures): the kernel implementation
   matches its abstract specification.
2. **Integrity** (AArch64 EL2, April 2025): no process can modify kernel objects or
   another process's capabilities without holding a grant right to the relevant capability.
3. **Confidentiality** (RISC-V64, published; AArch64 in progress): no process can read
   another process's data without holding a read right.

## What "topology determines security" means

Security in an seL4 system is not determined by policies, ACLs, or software checks.
It is determined by the shape of the capability graph — the topology.

An architect who draws the capability topology of a system has drawn its security
boundary. Two components that share no path in the capability graph have no information
channel, no matter what software runs inside them. A compromised component cannot
"escape" its partition because escape would require obtaining a capability it does not
hold — and the kernel proves that capability creation is monotone: you cannot create a
capability you do not already have (or cannot derive from what you have).

## Industrial prior art — Fuchsia "component topology"

Google's Fuchsia OS uses the same capability model and the same vocabulary. Fuchsia's
documentation refers to the "component topology" to describe the tree of component
capability relationships that determines what system calls and interfaces are reachable.
This is direct evidence that "topology" is the established term for this architectural concept.

The analogous PointSav PPN architecture: the seL4 capability graph is the outer security
boundary. What happens inside a component (a Linux VM, a WireGuard PD, a Doorman process)
is bounded by what the capability graph permits it to reach.

## Terminology

Use **topology** for this concept. Not "geometry." Precedents:

- Miller, Jonathan, Shapiro (2000): "Robust Composition: Towards a Unified Approach to
  Access Control and Concurrency Control" — introduces the "only connectivity begets
  connectivity" principle; uses "object graph" which subsequent literature formalises as
  "capability topology."
- Drossopoulou et al. (2016): "Holistic Specifications for Robust Programs" — formal
  treatment of capability safety using graph topology terminology.
- Murray et al. (2013): seL4 information-flow proof (USENIX Security 2013).
- Fuchsia documentation: "component topology" used throughout.

No peer-reviewed paper uses "geometry" for this concept.

## Architecture coverage by seL4 target

| Architecture | Formal security claim | Notes |
|---|---|---|
| AArch64 EL2 | Functional correctness + integrity | Only verified hypervisor mode as of mid-2026 |
| RISC-V64 | Functional correctness + integrity + confidentiality | Deepest proofs; HiFive Unleashed only |
| x86-64 | Functional correctness only | No integrity or confidentiality proof |

**Critical:** "Topology determines security" is a valid claim only for AArch64 EL2 and
RISC-V64 deployments. Do not apply this claim to x86-64 deployments.

## PPN application

The PointSav Private Network is planned/intended to use seL4 as the hypervisor layer
on AArch64 nodes. In that configuration, the seL4 capability topology governs what
components can communicate across the mesh. The WireGuard mesh interface, the pairing
ceremony server, and the fleet management service each occupy distinct seL4 protection
domains with explicitly granted capability channels. A component without a capability
to the WireGuard PD cannot modify peer tables — regardless of whether it is compromised.

This is the formal basis for the PPN security model.
