---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: pointsav/content-wiki-documentation
target_path: ./
target_filename: topic-pointsav-private-network.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: true
bilingual_pair: topic-pointsav-private-network.es.md
references:
  - woodfine-fleet-deployment/README.md (lines 64-102)
  - woodfine-fleet-deployment/route-network-admin/guide-mesh-orchestration.md
  - woodfine-fleet-deployment/fleet-infrastructure-cloud/guide-provision-relay.md
  - woodfine-fleet-deployment/fleet-infrastructure-onprem/guide-provision-onprem.md
  - woodfine-fleet-deployment/guide-mesh-execution.md
  - woodfine-fleet-deployment/route-network-admin/README.md
research_trail:
  source_commits:
    - "guide-mesh-orchestration.md — WireGuard hub-and-spoke topology, Curve25519 keys"
    - "route-network-admin/README.md — NODE-IMAC-12 master key custody"
    - "guide-mesh-execution.md — F8 Terminal, Two-step HITL protocol, Zero-Broker UDP"
    - "fleet-infrastructure-onprem/guide-provision-onprem.md — Mesh Fusion terminology"
  prior_drafts: []
  citations: []
  operator_inputs:
    - "PPN is infrastructure; it carries packets but has no access to os-* application layer (2026-05-20)"
    - "Even PointSav as vendor cannot access customer os-* data through the PPN (2026-05-20)"
    - "MBA is peer-to-peer above the PPN, completely independent security layer (2026-05-20)"
    - "PPN is deliberately designed so Totebox Orchestration sits on top of it (2026-05-20)"
  related_files:
    - .agent/drafts-outbound/topic-machine-based-authorization.md
    - .agent/plans/os-console-platform.md
notes_for_editor: |
  Comprehensive first draft. This TOPIC covers PPN as infrastructure, separate from MBA.
  The key architectural message — vendor cannot access customer data through infrastructure
  ownership — is the most important claim and must be stated precisely.

  Refinement priorities:
  - Verify "PPN" as canonical abbreviation vs. full form "PointSav Private Network"
  - Register citation IDs for WireGuard design decisions
  - Generate bilingual .es.md pair
  - Apply Bloomberg-article register
  - Confirm fleet node inventory (cloud relay, onprem iMac, leased laptop) is current
  - "Zero-Broker UDP" and "Mesh Fusion" terms — confirm canonical status with operator
  - BCSC note: forward-looking claims (WireGuard fabric ratification) need "planned/intended" language
  - Target length: ~1000-1300 words English
---

# PointSav Private Network

## What the PPN is

The PointSav Private Network (PPN) is the private WireGuard mesh that connects
Woodfine's fleet nodes. It is an infrastructure layer: it provisions the virtual
machines that run `os-*` services, routes encrypted traffic between them, and provides
a static topology that Totebox Orchestration runs on top of.

The PPN is not an authorization system. It grants no access to application data.
Being on the PPN means a machine can reach the network; it does not mean a machine can
open any door. Authorization is handled at the application layer by Machine-Based
Authorization (MBA), which operates independently of and above the PPN.

## The hub-and-spoke topology

The PPN uses a hub-and-spoke topology:

| Node | Role | WireGuard position |
|---|---|---|
| `fleet-infrastructure-cloud` | GCP compute instance; static public IP | Hub — accepts all spoke connections |
| `fleet-infrastructure-onprem` | iMac 12.1 (NODE-IMAC-12); on-premises | Spoke — dials OUT to cloud hub |
| `fleet-infrastructure-leased` | Laptop endpoints (Laptop A, Laptop B) | Spoke — dials OUT to cloud hub |

All traffic initiates from the spokes. The hub never dials in to nodes. The public
internet cannot initiate connections to spoke nodes — they dial out only.

**Cloud relay role:** The cloud hub provides a stable, publicly-addressable endpoint
for WireGuard peer discovery. It relays encrypted WireGuard packets between spokes.
It does not decrypt application-layer traffic; it cannot read `os-*` service data.

## NODE-IMAC-12 and master key custody

The on-premises spoke — NODE-IMAC-12, an iMac 12.1 running Linux Mint — holds the
master cryptographic keys for the entire network. This is a deliberate physical custody
decision: the machine that holds the WireGuard master keys sits on the executive's desk,
under physical custody of Woodfine Management Corp.

If the cloud relay is destroyed, NODE-IMAC-12 retains the master keys and can
immediately provision a new relay by dialing into a new cloud IP. The WireGuard key
material that defines the network topology is in physical possession of the customer,
not the vendor.

NODE-IMAC-12 is also the primary deployment target for `os-console`. It is
the machine from which the executive operates the Totebox Archive interface.

## WireGuard cryptographic fabric

WireGuard uses Curve25519 elliptic-curve key pairs: one private/public pair per node.
The private key never leaves its node. Public keys are distributed to peers and
recorded in WireGuard configuration on the hub.

Key management follows the principle of least exposure:
- Private keys stored on-device only, never transmitted
- `route-network-admin` holds the authoritative peer key registry and subnet assignments
- IP range: `10.x.x.x/24` (exact ranges ratified when the fabric moves to Active state)

The PPN uses a Zero-Broker UDP broadcast matrix: fleet health commands are broadcast
as UDP signals across the mesh to all active nodes simultaneously, without routing
through any central broker. Every node that is online replies.

## Mesh Fusion: joining the network

Binding a new physical node to the PPN is called Mesh Fusion. The procedure:

1. Install the host operating system on the target hardware
2. Generate a WireGuard Curve25519 key pair on the new node
3. Register the public key with `route-network-admin`
4. Configure the WireGuard interface on the new node with the hub endpoint and subnet IP
5. Establish the encrypted tunnel: the node dials the cloud relay
6. Verify connectivity: the hub can see the new spoke; the spoke can reach other nodes

Mesh Fusion is infrastructure-layer provisioning only. It does not establish any
`os-*` application pairings. After Mesh Fusion, the node is on the network; MBA
pairing ceremonies are performed separately to grant application-layer access.

## The F8 Terminal: mesh management interface

The PPN is managed through `os-network-admin`, exposed as the F8 Terminal — a
keyboard-driven interface at the `route-network-admin` node. Operators submit network
management commands in natural English. The interface uses a two-step execution
protocol to ensure human-in-the-loop oversight of all network changes:

1. **Submit Intent:** The operator types a command in natural English
   (e.g., `Lock down the laptop node`)
2. **Verify Translation:** The interface halts and displays the machine-translated
   action payload proposed by the SLM
   (e.g., `[PROPOSED] ACTION: ISOLATE, TARGET: NODE-LAPTOP-A`)
3. **Authorize Execution:** The operator visually confirms the translation and
   explicitly clicks EXECUTE to broadcast the command

No network action executes without the operator's visual confirmation of the
machine-translated payload. The SLM translates intent; the human authorizes execution.

## Deliberate isolation from the application layer

The most important architectural property of the PPN: it is **deliberately isolated**
from the `os-*` application layer.

The PPN was designed to carry packets and provision VMs. It was not designed to grant
access to what runs inside those VMs. This isolation means:

- A machine on the PPN cannot read `os-totebox` data without an MBA pairing
- PointSav Digital Systems, as vendor and PPN operator, has no application-layer access
  to customer Totebox Archives through the network infrastructure
- Compromising the network relay does not compromise customer data
- The vendor owns the pipes. The customer owns the doors.

This is the foundation of the Sovereign Data property: the customer's data sovereignty
is preserved even against the vendor who built and operates the underlying infrastructure.

## PPN and MBA: two independent security layers

The PPN and MBA are designed to be independent. Neither depends on the other for
its security guarantees:

| Layer | What it protects | Who manages it | What it does NOT do |
|---|---|---|---|
| PPN (WireGuard) | Network topology; encrypts transit traffic | PointSav Digital Systems (vendor) | Does not grant os-* application access |
| MBA (peer-to-peer) | os-* application access | Machine's operator (customer) | Does not depend on PPN topology |

An attacker who fully compromises the PPN gains encrypted WireGuard traffic between
VMs and nothing more — they cannot read application data without MBA pairings. An
operator who revokes an MBA pairing blocks application access even if the machine
remains on the PPN.

The two layers provide defense in depth. They also enforce the sovereignty boundary:
the vendor controls the infrastructure layer; the customer controls the authorization
layer.

## Cold Storage Entanglement

Heavy archive data — architectural drawings, IoT logs, extensive media assets — is
managed through cryptographic splitting and physical egress. External drives are
mathematically locked to specific archives using a key derived from the archive's
identity. Those drives are unreadable on any other system. The core operating
environment remains lightweight while overflow data stays under physical custody
of the customer.

Cold Storage Entanglement operates outside the PPN — it is a physical data custody
mechanism, not a network mechanism. The drives are encrypted with keys that live on
the archive node, not in the WireGuard mesh.

## See also

- TOPIC: Machine-Based Authorization — the application-layer authorization that
  operates above the PPN
- TOPIC: os-console and app-console-keys — the platform that connects to os-* via MBA
- `woodfine-fleet-deployment/route-network-admin/guide-mesh-orchestration.md` — WireGuard
  key generation and subnet assignment
- `woodfine-fleet-deployment/guide-mesh-execution.md` — F8 Terminal operations
- `woodfine-fleet-deployment/fleet-infrastructure-cloud/guide-provision-relay.md` — cloud
  relay provisioning
