---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: genesis-protocol.md
audience: technical operators and engineers evaluating or deploying PointSav fleets
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - architecture/machine-based-auth.md
  - architecture/diode-standard.md
  - infrastructure/sovereign-mesh.md
notes_for_editor: >
  New topic — no existing file at this path. The Genesis Protocol is referenced
  briefly in infrastructure-os.md §"The Genesis Protocol" (5-step summary) and
  in sovereign-mesh.md. This topic provides the dedicated deep-dive. Content is
  derived entirely from the four listed sources; no new architecture is introduced.
  Article frontmatter to add on commit: title "The Genesis Protocol",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, machine-based-auth, sovereign-mesh].
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance: >
  Four source documents read directly from the current working tree:
  (1) systems/infrastructure-os.md — 5-step Genesis Protocol sequence, deferred fleet
  assembly pattern, PPN formation; (2) architecture/machine-based-auth.md — fiduciary
  keypair mechanics, ADMIN pairing type, Noise Protocol and WireGuard-style keys;
  (3) architecture/diode-standard.md — authority hierarchy context;
  (4) infrastructure/sovereign-mesh.md draft — mesh join sequence post-claim.
research_inline: true
---

# The Genesis Protocol

The Genesis Protocol is the fleet-bootstrapping sequence used by every `os-infrastructure` node at first boot. It allows a node to become operational on isolated hardware — with no prior configuration, no connection to any control plane, and no knowledge of the eventual fleet it will join — and to remain in a secure, claimable state until an administrator is ready to bring it under management. The protocol inverts the conventional assumption that a control plane must exist before compute can be added to it.

## The problem it solves

Conventional fleet management requires a sequencing dependency: the control plane must be configured, the network must be routed, and the node must be enrolled before the node becomes useful. For an operator shipping hardware to a distant location, this creates a coordination problem — the hardware arrives at a remote site before the fleet management layer is ready, or the reverse.

The Genesis Protocol removes the sequencing dependency. A node can ship to any location, boot in any network environment, and reach a secure, self-contained state without any pre-provisioning. The administrator claims it whenever they are ready.

## The five steps

### 1 — Blind boot

On first boot, the seL4 kernel generates a Tier-1 fiduciary keypair from hardware entropy. The node then enters blind-boot mode: it deliberately ignores DHCP and DNS, refusing to acquire a network address through conventional mechanisms. This prevents the node from being reached by, or reaching out to, any infrastructure it has not already verified.

### 2 — Scan

The node scans the local network for an `os-network-admin` beacon on the PPN mesh port. The scan uses the node's fiduciary public key as the identity it presents — so only a legitimate `os-network-admin` instance holding the corresponding administrative key material can respond authoritatively.

### 3 — Genesis fork

If the scan finds no `os-network-admin` beacon within the scan window, the node forms a Private Network of One. It seals all external ports except a single, hardened endpoint. It does not attempt to contact any external service. It does not fail. It holds.

A node that has genesis-forked is fully operational: it has its keypair, it has a sealed network perimeter, and it is waiting for a claim. It is not a broken node — it is a fleet-ready node that has not yet been claimed.

### 4 — Holding pattern

The single open endpoint is a hardened WebSocket interface. It accepts only one message class: an administrative claim request presenting a valid fiduciary keypair. Any other connection attempt is silently dropped. The node emits no identifying information to the network; to an external observer, the endpoint is opaque.

### 5 — Claim

When an administrator boots `os-network-admin` and presents the administrative fiduciary key, the holding-pattern endpoint verifies the key against the node's locally stored keypair. If the pair verifies:

1. The node binds to the fleet, accepting the `os-network-admin` instance as its authority
2. The node receives its WireGuard mesh configuration and joins the [[sovereign-mesh|sovereign mesh]]
3. The node's fiduciary keypair is registered in `os-network-admin`'s pairing registry as an ADMIN pairing
4. The sealed external ports open according to the fleet's Diode Standard policy

If the key does not verify, the claim is silently rejected. The node remains in its holding pattern and emits no error response.

## Deferred fleet assembly

The Genesis Protocol is designed for the case where hardware ships before the administrator is ready to manage it. An operator can ship fifty servers to fifty edge locations. Each arrives, boots, and forms a one-node PPN. Whenever the administrator is ready — days, weeks, or months later — `os-network-admin` claims all fifty in sequence. Each verifies the claim and joins the fleet. The operator never needs to physically touch the nodes after initial shipment.

This pattern works because the holding pattern is indefinitely stable. A node in the holding pattern draws minimal power, maintains its sealed perimeter, and presents no attack surface beyond the hardened claim endpoint.

## Relationship to machine-based authorization

The Genesis Protocol is the provisioning phase of [[machine-based-auth|machine-based authorization]]. The fiduciary keypair generated at first boot becomes the ADMIN pairing entry for that node in the fleet's pairing registry. After the claim:

- The node's keypair material is the sole basis for all subsequent authentication — no password is ever transmitted
- The pairing can be revoked by severing the ADMIN entry in the registry; the node becomes ungovernable by the fleet (though it continues to run its workloads standalone)
- The administrative fiduciary key authority resides physically with the administrator — it is never delegated to a cloud service

## See also

- [[infrastructure-os]] — the compute substrate that runs the Genesis Protocol at first boot
- [[os-network-admin]] — the control plane that executes the claim sequence
- [[sovereign-mesh]] — the WireGuard overlay the node joins after a successful claim
- [[machine-based-auth]] — the fiduciary keypair system the protocol relies on
- [[diode-standard]] — the authority hierarchy that governs the fleet after claim

---

## Research trail

### Done

1. Read `systems/infrastructure-os.md` — 5-step Genesis Protocol sequence, blind boot rationale, deferred fleet assembly pattern ("fifty servers to fifty locations"), PPN formation
2. Read `architecture/machine-based-auth.md` — ADMIN pairing type, Noise Protocol and WireGuard-style keys, hardware-binding of private keys, revocation mechanism
3. Read `architecture/diode-standard.md` — authority hierarchy context; os-infrastructure is a Subject, os-network-admin is the governing Authority
4. Read `infrastructure/sovereign-mesh.md` (draft) — mesh join sequence post-claim, WireGuard peer registration, IP assignment from pairing registry

### Suggested (not done this session)

1. Read `fleet-infrastructure-onprem/guide-genesis-protocol.md` if it exists — operational runbook may contain additional detail about the claim sequence and key ceremony
2. Read `os-infrastructure/src/main.rs` — current prototype code; understand gap between current EAPOL implementation and the Genesis Protocol target described here

### Open questions

1. **EAPOL vs Genesis Protocol** — current `os-infrastructure/src/main.rs` implements EAPOL monitor-mode, not the Genesis Protocol described in this topic. This topic describes the intended architecture. The code alignment is a pending operator decision. No correction needed to this topic — the intended architecture is what the TOPIC should describe.
