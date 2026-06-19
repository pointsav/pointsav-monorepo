---
artifact: topic
schema: foundry-draft-v1
title: "Geometric Protection: seL4 Capability Authorization in Totebox Orchestration"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, claim-43, claim-49, SYS-ADR-10]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, BRIEF-OS-FAMILY.md, seL4-reference-manual-v1.4, system-core-v1.0.0, system-ledger-v1.0.0]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: agent-research + system-core/system-ledger source review
---

# Geometric Protection: seL4 Capability Authorization in Totebox Orchestration

**Geometric Protection™** is a PointSav term for the application of the seL4 capability
model to Totebox authorization. It is not a product feature name or marketing claim. It
describes a mathematically distinct approach to access control that changes the structure
of authorization rather than adding strength to an existing model.

---

## Why Layered Security Fails

The standard response to a security breach is to add another layer.

```
Firewall → WAF → IAM → VPN → TLS → 2FA → SIEM → EDR → CASB → Zero Trust
```

Each new layer is a new attack surface. An adversary who learns to operate within the
newest layer can reach whatever the layer was intended to protect. More importantly, each
layer makes the same underlying assumption: **authenticate, and you have access**. The
access model — a subject presenting credentials to reach an object — does not change.
The geometry stays the same. A determined adversary is learning the maze, not losing the
ability to traverse it.

Layered security increases the cost and time of an attack. It does not change the attack
surface's logical structure.

---

## The seL4 Capability Model: Authorization as a Formal DAG

The seL4 microkernel implements a different model. Access to any resource — a memory
region, a network endpoint, a storage device — requires holding an unforgeable capability
token for that resource. Not a credential. Not a session token. A mathematical object
issued by the kernel itself.

The set of all capabilities in a running seL4 system forms a **directed acyclic graph**:
each edge is a capability; each node is a resource or a capability container (CNode).
To reach a resource, a process must hold a capability that forms a path to it in this
graph. There is no other path.

Key properties of this model:

**Unforgeability.** A capability cannot be constructed from random bits. It is a kernel
object. A process cannot guess a capability it was not given.

**No ambient authority.** In a conventional OS, a root process can reach any resource.
In seL4, even a privileged process can only reach resources for which it holds explicit
capabilities. There is no "root" that bypasses the graph.

**Formal proof.** The seL4 kernel has been formally verified using the Isabelle/HOL proof
assistant. The proofs establish that the capability model is correctly enforced by the
hardware MMU at all times. This is not an engineering assertion — it is a machine-checked
mathematical proof.

**Revocation propagates.** Revoking a capability removes an edge from the graph. The
proof guarantees that this propagation is complete: no descendant capability remains usable
after revocation.

---

## Geometric Protection Defined

Geometric Protection is the condition where the authorization model is a formally proven
bounded DAG of unforgeable tokens, rather than a mutable access control list checked at
runtime.

In conventional security, the "geometry" of access — which subjects can reach which
objects — is a mutable runtime state. An adversary who corrupts state changes the
geometry. In the seL4 capability model, the geometry is a kernel-enforced invariant.
The kernel's correctness proof means the geometry is not mutable by software running
below the kernel boundary.

An adversary who fully compromises a single Protection Domain can only access what the
seL4 proofs say that PD can reach. The topology of access is a mathematical object, not
a policy enforced by software that can be subverted.

---

## PointSav Implementation: system-core and system-ledger

The capability substrate in Totebox Orchestration is implemented in two Rust crates:

**system-core v1.0.0** defines the capability type system:

```rust
pub struct Capability {
    pub cap_type: CapabilityType,   // Endpoint | Memory | Irq | Notification | CNode
    pub rights: Vec<Right>,          // Read | Write | Invoke | Grant | Revoke
    pub expiry_t: Option<u64>,
    pub witness_pubkey: Option<String>,
    pub ledger_anchor: LedgerAnchor,
}
```

A `Capability` is not a session token. It is a typed, rights-bounded, optionally time-limited
object anchored to the WORM audit ledger.

**system-ledger v1.0.0** provides the verdict function:

```rust
pub enum Verdict {
    Allow,
    Refuse(RefuseReason),
    ExtendThenAllow { new_expiry_t: u64 },
}
```

`consult_capability()` on `InMemoryLedger` evaluates a capability invocation against the
current ledger state and returns a `Verdict`. The ledger is append-only (WORM) and anchored
via RFC 9162 Merkle proof chains. The audit trail for F12 (SYS-ADR-10) routes through
this verdict function.

---

## Machine Pairing as Capability Minting

F11 machine pairing in os-console is the intended capability minting ceremony for Totebox
access (planned; Phase H3 of the os-console substrate roadmap):

1. The Totebox pairing authority holds a `CapabilityType::CNode` — the root of its
   capability namespace.
2. When a host machine pairs via F11, the pairing authority derives and grants
   `CapabilityType::Endpoint` tokens for each authorized cartridge service.
3. The host machine's os-console instance holds these tokens. They are stored in the
   seL4 guest VM's CNode — not on the host filesystem.
4. At any point, the Totebox operator revokes a token. The `apply_revocation()` call on
   `system-ledger` propagates the revocation. The next IPC attempt from that os-console
   instance returns `Verdict::Refuse`.

The host machine is authorized. Not the user account — the machine. This is the
machine-level identity anchor for Totebox Orchestration.

---

## Contrast with Conventional IAM

| Conventional IAM / ACL | Geometric Protection (seL4) |
|---|---|
| Subject presents credential | Subject holds unforgeable capability token |
| Credential checked against policy at runtime | Kernel enforces capability ownership; no runtime policy |
| Policy is mutable state | Capability graph is kernel-enforced invariant |
| Escalation possible if policy state is corrupt | No escalation path without a capability edge |
| Revocation requires policy propagation (may lag) | Revocation removes kernel-level edge; proof ensures propagation |
| Adding security = adding policy layers | Adding security = removing capability edges |
| Security is a property of the enforcement software | Security is a property proven of the enforcement model |

---

## Leapfrog 2030 Alignment

Geometric Protection is the Totebox security model intended to be in place by the
Leapfrog 2030 milestone. The seL4 Microkit substrate (Doctrine claim #34, Two-Bottoms
Sovereign Substrate) provides the kernel layer. system-core and system-ledger provide
the Rust-language capability substrate above it. The three-binary architecture
(os-console, os-totebox, os-orchestration) implements Geometric Protection at each
layer: per-cartridge PD on os-console, per-service PD on os-totebox, and a
capability-broker PD on os-orchestration that holds cross-Totebox endpoint capabilities.

The model is intended to be in production on Totebox hardware before any hyperscaler
can replicate formally verified capability-based isolation at the SMB price point.
