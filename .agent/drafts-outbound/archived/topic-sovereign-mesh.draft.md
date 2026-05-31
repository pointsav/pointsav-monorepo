---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: infrastructure/
target_filename: sovereign-mesh.md
audience: technical operators and engineers working on PPN deployment and administration
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - infrastructure/sovereign-mesh.md
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - architecture/diode-standard.md
  - architecture/machine-based-auth.md
notes_for_editor: >
  Expands the one-sentence stub at infrastructure/sovereign-mesh.md.
  Content synthesised from the five source documents listed above — no new
  architecture is introduced; this topic names and describes the mesh layer
  that infrastructure-os and os-network-admin reference by implication.
  Two operator decisions are pending at originating cluster (canonical PPN subnet
  and GCP static IP); planned/intended language is used throughout for IP
  assignments. Companion Spanish draft is staged alongside this file.
  Article frontmatter to add on commit: title "Sovereign Mesh",
  category "infrastructure", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, diode-standard, machine-based-auth].
research_done_count: 5
research_suggested_count: 2
open_questions_count: 2
research_provenance: >
  Five source documents read directly from the current working tree:
  (1) infrastructure/sovereign-mesh.md stub,
  (2) systems/infrastructure-os.md,
  (3) systems/os-network-admin.md,
  (4) architecture/diode-standard.md,
  (5) architecture/machine-based-auth.md.
  No external research performed. Architecture synthesised from these documents.
research_inline: true
---

# Sovereign Mesh

The **sovereign mesh** is the application-level network overlay that connects every PointSav Private Network (PPN) fleet node. It runs over WireGuard cryptographic tunnels on a dedicated `ppn0` interface and carries signed binary commands without relying on a centralised message broker. Each node communicates directly with its authorised peers; the mesh layer enforces the same authority hierarchy as the [[Diode Standard]] as a structural property, not a configuration option.

## Hub-and-spoke topology

The mesh uses a hub-and-spoke arrangement. The cloud relay node sits at the centre and relays packets between spoke nodes that may not have a direct path to each other.

| Role | Node | Planned address | Crate |
|---|---|---|---|
| Hub | Cloud relay (GCP) | `10.50.0.1` | `app-infrastructure-cloud` |
| Spoke | On-premises node | `10.50.0.2` | `app-infrastructure-onprem` |
| Spoke | Leased node | `10.50.0.3` | `app-infrastructure-leased` |

The `10.50.0.0/24` subnet is the intended PPN address range. All mesh traffic is encapsulated inside WireGuard before leaving a node; the underlying transport — public internet, private LAN, or GCP internal network — is irrelevant to the mesh layer.

## WireGuard overlay

Each node brings up a `ppn0` WireGuard interface as part of its boot sequence. WireGuard provides:

- **Key agreement** — Noise Protocol IK handshake; each node's long-term keypair is generated and stored at first mesh join by `os-network-admin` for the control-plane node, or via the Genesis Protocol for bare-metal edge nodes
- **Encryption and integrity** — ChaCha20-Poly1305 per packet; no plaintext mesh traffic ever leaves a node
- **Peer reachability** — the cloud relay is the only statically-addressed peer; on-premises and leased nodes resolve each other through the relay until a direct routed path becomes available

WireGuard configuration for each node is held in `~/Foundry/deployments/<instance>/` (local-only, gitignored). Keypairs are never stored in any repository.

## Command protocol

All mesh commands use a 16-byte binary packet format delivered over UDP on port 8090. The compact size is deliberate: the packet carries an intent token, a target selector, a nonce, and a truncated authority signature — sufficient to identify the command, verify its provenance, and detect replay attacks without requiring a full TLS session per command.

The command flow from operator to target node is:

```
Operator intent (plain language)
      ↓
F8 Terminal  —  os-network-admin  HTTP :8085
      ↓
service-slm semantic router
      ↓
16-byte binary command (authorised and signed)
      ↓
service-udp broadcast  →  ppn0  →  WireGuard tunnel
      ↓
Target node  —  UDP port 8090
```

Commands flow in one direction only — from `os-network-admin` outward to the mesh — a constraint enforced by `service-pointsav-link` at the application layer. See [[Diode Standard]] for the full authority hierarchy.

## Node roles in the mesh

### os-infrastructure — edge anchor

The bare-metal `os-infrastructure` node is a mesh peer, not a mesh controller. It listens on port 8090 for signed binary commands addressed to it and executes them; it does not initiate commands. The node's Broadcom 14e4:16b4 NIC carries mesh traffic via the `ppn0` interface once the Genesis Protocol join sequence completes.

### os-network-admin — control plane

`os-network-admin` owns command authority for the mesh. The F8 Terminal — a plain-language command surface on HTTP port 8085 — accepts operator intent and routes it through `service-slm` to produce a signed 16-byte binary command. The command is then broadcast over `service-udp` on port 8090 to one or more mesh peers. `os-network-admin` also hosts the pairing registry and manages new-node admission via the [[machine-based auth]] handshake.

### Cloud relay — hub

The GCP cloud relay node relays WireGuard-encapsulated packets between spoke nodes. It does not interpret mesh commands; it is a transport layer only. The relay's fixed public IP and static WireGuard configuration make it the anchor point that allows on-premises and leased nodes to find each other without DNS or DHCP dependency.

## Genesis Protocol integration

A bare-metal node joins the mesh through the [[infrastructure-os#genesis-protocol|Genesis Protocol]] rather than manual WireGuard provisioning. At first boot:

1. seL4 generates an entropy-seeded keypair from hardware sources
2. The node enters blind-boot mode — ignoring all DHCP and DNS — and scans for the `os-network-admin` beacon on port 8090
3. If the beacon is found, `os-network-admin` guides the node through the mesh-join handshake: WireGuard peer registration, IP assignment, and keypair binding to the pairing registry
4. If no beacon is found within the scan window, the node self-geneses: it writes its keypair to UEFI Secure Variable storage and enters a holding pattern on port 9443, awaiting an admin claim

This mechanism ensures that no node ever joins the mesh without a verified authority handshake. Manual `wg genkey` workflows apply during initial fleet provisioning only; they are not the runtime join path for production nodes.

## Relationship to the Diode Standard

The [[Diode Standard]] defines three mesh traffic categories: authority commands, telemetry, and inter-node sync. All three flow through the sovereign mesh, but only authority commands use the 16-byte binary format on port 8090. Telemetry and sync traffic use WireGuard-encapsulated TCP or UDP on other ports.

The Diode Standard's unidirectionality constraint — authority commands flow from `os-network-admin` to nodes, never the reverse — is implemented at the mesh layer by `service-pointsav-link`, a hot-pluggable adapter that enforces the flow direction without requiring WireGuard policy changes.

## See also

- [[infrastructure-os]] — deployment postures, Genesis Protocol sequence, Broadcom NIC substrate
- [[os-network-admin]] — F8 Terminal, service-slm integration, mesh policy ownership
- [[diode-standard]] — authority hierarchy and traffic category definitions
- [[machine-based-auth]] — Noise Protocol keypair management and pairing types

---

## Research trail

### Done

1. Read `infrastructure/sovereign-mesh.md` stub — confirmed one-sentence content; identified full scope of missing sections
2. Read `systems/infrastructure-os.md` — Genesis Protocol five-step sequence, three deployment postures, Broadcom 14e4:16b4 NIC role, hub-spoke topology
3. Read `systems/os-network-admin.md` — F8 Terminal, service-slm → 16-byte binary → service-udp → port 8090 command flow, control-plane role
4. Read `architecture/diode-standard.md` — three traffic categories, service-pointsav-link adapter, unidirectionality enforcement mechanism
5. Read `architecture/machine-based-auth.md` — Noise Protocol IK handshake, four pairing types (ADMIN/INPUT/USER/INTERFACE), pairing registry ownership

### Suggested (not done this session)

1. Read `service-vpn/` crate source to confirm WireGuard tooling, key rotation, and `ppn0` bring-up sequence
2. Read `os-network-admin/scripts/mesh_status.sh` to verify IP assignments, interface names, and port references match this topic

### Open questions

1. **Canonical PPN subnet** — production code hardcodes `10.50.0.1/2/3`; guides use `10.x.x.x/24` (range unspecified). This topic uses `10.50.0.0/24` and `.1/.2/.3` as the planned addresses with "planned/intended" language. Operator must ratify before this topic is published.
2. **Genesis Protocol implementation state** — TOPICs describe the full 5-step seL4 protocol; current `os-infrastructure/src/main.rs` contains EAPOL monitor-mode code, not Genesis Protocol code. This topic describes the intended architecture per the TOPICs. Code/TOPIC alignment is a separate track gated on the operator's EAPOL-vs-Genesis decision.
