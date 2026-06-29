---
artifact: foundry-draft-v1
type: TEXT
slug: text-ppn-topology-security-pitch
title: "PPN Topology Security — Product Page Copy"
status: draft
created: 2026-06-29
author: totebox@project-infrastructure
route_to: project-editorial
language_protocol: COMMS-*
research_source: geometric-capabilities-research-2026-06-29
research_claim: "seL4 capability topology as the PPN security model — product copy"
research_method: seL4 formal verification research + competitor positioning
research_verification: claims verified against AArch64 EL2 integrity proof status (April 2025)
language: en
audience: SMB decision-maker; no computer science background required
word_target: 300
---

# PPN Topology Security — Product Page Copy

**Target:** software.pointsav.com product description section for os-infrastructure.

**BCSC note:** Forward-looking claims use "planned/intended" language.

---

## Draft copy

### Security built into the architecture

Most operating systems treat security as a layer — a firewall here, an access control
list there, a policy that can be misconfigured or overridden.

os-infrastructure takes a different approach. Security is built into the structure of
the software itself.

Every component in os-infrastructure runs in a seL4 protection domain. The seL4
microkernel is formally verified — its security properties are machine-checked
mathematical proofs, not engineering judgments.

The security model is based on one principle: **only connectivity begets connectivity.**

If component A has no capability pointer to component B, then A cannot observe B,
cannot modify B, and cannot call B. Not because of a policy rule. Not because of a
firewall. Because the mathematical proof says so.

The shape of those capability connections — the topology — is what determines security.
You can draw it. You can audit it. You can prove it.

### What this means for your infrastructure

When you run os-infrastructure on a node, the WireGuard mesh interface, the fleet
management service, and the inference layer each run in separate seL4 partitions with
explicitly granted channels between them.

A compromised service cannot escape its partition and cannot reach another service's
data — because no seL4 capability connects them. The topology prevents it.

This is what "sovereign compute" means at the infrastructure level: the security
boundary is a mathematical proof, not a configuration file.

### Available today

os-infrastructure is available for x86-64 hardware at software.pointsav.com for $19 USDC.

The AArch64 version — the target of the April 2025 UK NCSC-funded formal verification
work — is planned/intended as the next release target, intended to carry the full formal
security claim as proofs are published.

Source code is open and available on GitHub. The binary distribution is $19 USDC for
the pre-built, Ed25519-signed release.

---

## Editorial notes

- "topology" terminology is correct and verified against academic literature (Miller 2000,
  Fuchsia docs). Do not change to "geometry."
- "machine-checked mathematical proofs" is factually accurate for AArch64 EL2 integrity
  (April 2025). Do not generalize to x86-64.
- "planned/intended" language is required for AArch64 release timing (BCSC posture).
- The "only connectivity begets connectivity" quote is from Miller & Shapiro (2000) —
  can be attributed or paraphrased; confirm with editorial.
