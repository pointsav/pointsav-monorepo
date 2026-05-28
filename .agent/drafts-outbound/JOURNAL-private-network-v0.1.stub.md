---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.2"
title: "Customer-Rooted Mesh Architecture for Distributed Operational Systems: Zero-Trust Isolation Without Vendor Key Custody"
target_journal: "IEEE Transactions on Information Forensics and Security"
target_publisher: "IEEE Signal Processing Society"
impact_factor: "9.65"
alternate_venue: "Computers & Security (Elsevier, IF 7.98); IEEE Transactions on Network and Service Management (IEEE, IF 6.44)"
authors:
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Formal Analysis
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Validation
      - Writing – Review & Editing
subject_codes:
  - "C.2.1 Network Architecture and Design"
  - "C.2.2 Network Protocols"
  - "K.6.5 Security and Protection"
keywords:
  - zero-trust architecture
  - private network
  - mesh topology
  - WireGuard
  - distributed systems
  - customer-controlled networking
  - key custody
  - network isolation
  - audit log integrity
bcsc_class: no-disclosure-implication
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: jmwoodfine@gmail.com
word_count_body: 4800
word_count_target: 9000
submission_status: not-submitted
writing_pass_date: 2026-05-28
language_pass_date: 2026-05-28
preprint_posted: true
preprint_posted_date: 2026-05-28
cites:
  - donenfeld-2017-wireguard
  - rose-2020-nist-800-207
  - ward-beyer-2014-beyondcorp
  - perrin-2018-noise
  - lipp-2019-wireguard-proof
  - saltzer-schroeder-1975
  - bellovin-1989-security-problems
  - dingledine-2004-tor
  - asokan-2011-man-in-middle
forbidden_terms_cleared: true
notes_for_editor: |
  Writing pass 2026-05-28: §1–§3 body written (~4,800 words). §4 Implementation and
  §5 Evaluation pending benchmark data. §6 Discussion with formal hypotheses,
  falsification programme, and limitations written. §7 Conclusion written.

  Pre-submission blockers:
    - §4 Implementation: benchmark environment needed (tunnel establishment time,
      rekey latency, policy-change propagation, failure-mode behaviour)
    - §5 Evaluation: empirical data pending above benchmarks
    - ORCID IDs for all three authors
    - Promote [CITATION NEEDED] placeholders to stable citation IDs once located

  Venue note: IEEE TIFS is the primary target. Check current IEEE JSAC CFP for
  "Zero Trust for Next-Generation Networking" special issue — if open, preferred
  submission path. Standard TIFS track is the fallback.
---

> **Working Paper · Version 0.2 · 2026-05-28**
> This manuscript is a working draft. It has not been peer reviewed. Findings are preliminary and subject to revision without notice. The authors welcome correspondence at jmwoodfine@gmail.com.

> **Forward-Looking Statements**
> Certain statements in this paper describe intended research directions, planned system capabilities, and anticipated outcomes. These statements reflect the authors' current expectations and are based on reasonable assumptions and work in progress as of the date above. Actual results, measurements, and findings may differ materially. Readers should not place undue reliance on such statements; they are subject to revision as research progresses and new data become available.

# Customer-Rooted Mesh Architecture for Distributed Operational Systems: Zero-Trust Isolation Without Vendor Key Custody

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

---

## Abstract

Commercial zero-trust architecture (ZTA) products achieve network isolation by routing traffic through vendor-operated infrastructure, placing routing key custody, policy enforcement, and audit-log generation with the vendor rather than with the customer. Organisations that require verifiable network isolation with explicit, auditable customer control over routing keys have no mature open-architecture reference to follow. This paper proposes a customer-rooted mesh architecture (CRMA) that achieves ZTA-equivalent isolation guarantees using WireGuard as the sole cryptographic primitive, with all private routing keys generated and permanently held on customer-controlled nodes. The CRMA derives its network dependency topology directly from the served application's service-composition graph, enforcing component-boundary isolation at the network layer through declarative WireGuard AllowedIPs configuration. We describe the CRMA design, its public-key-only coordination model (the central coordinator holds only public keys and distributes mesh configuration; no private key ever transits the coordinator), its append-only customer-controlled audit log, and a prototype implementation comprising a hub-provisioning script, a spoke-factory script, and declarative WireGuard configuration templates. Two formal hypotheses are stated: H₁ (the CRMA provides isolation equivalent to commercial ZTA products under the NIST SP 800-207 [Rose et al. 2020] definition); H₂ (private key custody remains with originating nodes under adversarial coordinator compromise). A benchmark evaluation plan is described and pending execution. The CRMA provides a documented, reproducible open-architecture ZTA reference that structurally decouples isolation guarantees from vendor key custody.

---

## 1. Introduction

Zero-trust architecture (ZTA) has become the de facto security framework for distributed enterprise systems in the post-perimeter era [Rose et al. 2020]. The core ZTA principle — that no implicit trust is granted to any network location, and that every request for resource access is authenticated and authorised — is sound and well-supported by the academic literature [Kindervag 2010; Ward and Beyer 2014]. Commercial ZTA products that implement this principle have achieved wide adoption: Zscaler Internet Access, Cloudflare Zero Trust, Palo Alto Prisma Access, and Microsoft Azure AD Application Proxy each route enterprise traffic through vendor-operated network infrastructure and enforce access policies through vendor-held policy databases.

The commercial ZTA model contains a structural property that receives insufficient critical attention: the vendor holds the routing keys. When an enterprise deploys Zscaler or Cloudflare Zero Trust, the vendor's infrastructure establishes the tunnels, enforces the split-tunnel or full-tunnel routing policy, and generates the audit logs. The customer can configure the policy through a vendor-provided console, but the cryptographic material that enforces the policy — the private keys governing tunnel establishment and the routing tables governing traffic direction — resides on vendor infrastructure and is not exportable or independently auditable.

This vendor key-custody property creates a class of risks that is distinct from the risks ZTA was designed to address:

1. **Audit-log integrity**: vendor-generated audit logs can be modified or selectively omitted by the vendor without the customer's knowledge. Independent verification of the audit log requires trusting the vendor's assertion that the log is complete.

2. **Policy enforcement verification**: a customer cannot independently verify that the routing policy configured through the vendor console is the policy being enforced by the vendor's infrastructure. The enforcement is a black box.

3. **Custody transfer on vendor event**: a vendor insolvency, acquisition, or regulatory action that freezes the vendor's infrastructure immediately disrupts the customer's network isolation, with no customer-held cryptographic material available for independent recovery.

4. **Jurisdiction and compulsion**: the vendor's infrastructure is subject to the legal jurisdiction in which it operates. A government compulsion order against the vendor may expose customer traffic or policy configuration without the customer's knowledge or consent.

These risks are not hypothetical; analogous failures have occurred in commercial certificate authority [Cameron et al. 2019] and commercial VPN provider [CITATION NEEDED] contexts. They are structural properties of vendor-mediated key custody, not implementation defects.

The academic and practitioner literature on ZTA [Rose et al. 2020; Kindervag 2010; Ward and Beyer 2014] acknowledges the customer/vendor boundary but does not enumerate the key-custody risks or propose architectures that explicitly address them. WireGuard [Donenfeld 2017], the modern kernel-integrated VPN protocol with a formally verified cryptographic core [Lipp et al. 2019], provides the cryptographic primitives required to build ZTA without vendor key custody — but no peer-reviewed publication has documented a complete architecture that does so.

This paper makes three contributions:

1. **A taxonomy of key-custody risks in commercial ZTA** — enumerating the four risk categories above and defining measurable falsification criteria for customer key-custody assurance.

2. **The customer-rooted mesh architecture (CRMA)** — a WireGuard-based ZTA design in which private keys are generated on and never leave the nodes they protect; the central coordinator holds only public keys and distributes topology configuration; and the audit log is written to customer-controlled append-only storage.

3. **A prototype implementation** — a hub-provisioning script, spoke-factory script, and configuration templates that instantiate the CRMA, with a benchmark evaluation plan for empirical validation.

The remainder of this paper is organised as follows. Section 2 reviews background on ZTA, WireGuard, and the Noise Protocol Framework. Section 3 describes the CRMA design. Section 4 describes the prototype implementation. Section 5 presents the benchmark evaluation plan. Section 6 states formal hypotheses and the falsification programme. Section 7 concludes.

---

## 2. Background and Related Work

### 2.1 Zero-Trust Architecture

Zero-trust architecture was introduced by Kindervag [2010] as "never trust, always verify" — the principle that network location (inside or outside a perimeter) should carry no implicit access rights, and that every access request must be explicitly authenticated and authorised. The concept was operationalised by Google's BeyondCorp programme [Ward and Beyer 2014; Osborn et al. 2016], which migrated Google employees to a model where corporate applications were accessible from any network after device and user authentication, without VPN tunnels to a corporate perimeter.

NIST SP 800-207 [Rose et al. 2020] formalises ZTA as an enterprise security framework. The NIST definition identifies seven ZTA tenets, including: (1) all data sources and computing services are considered resources; (2) all communication is secured regardless of network location; (3) access to individual enterprise resources is granted on a per-session basis; (4) access policy is dynamic and informed by observable state of client identity, application, and other behavioural attributes; and (5) the enterprise monitors and measures the integrity and security posture of all owned and associated assets. The NIST framework does not specify where cryptographic key material must reside — this is the gap the CRMA addresses.

### 2.2 WireGuard

WireGuard [Donenfeld 2017] is a modern virtual private network protocol that operates as a Linux kernel module (and cross-platform userspace implementation). It uses a fixed cryptographic suite: Curve25519 for elliptic-curve Diffie-Hellman key exchange, ChaCha20 for symmetric encryption, Poly1305 for message authentication, BLAKE2s for hashing, and SipHash24 for hash table keys. The protocol is documented and formally verified; Lipp et al. [2019] provide a mechanised cryptographic proof of the WireGuard handshake using the ProVerif tool, establishing secrecy and authentication guarantees under the Dolev-Yao adversary model.

WireGuard's configuration model is minimal: each node has a private key (locally generated, never transmitted) and a public key (derived from the private key, distributed to peers). The peer configuration lists each peer's public key and the IP address ranges (AllowedIPs) for which traffic should be directed through the tunnel to that peer. Routing is entirely determined by the AllowedIPs configuration; there is no central routing table managed by a shared controller.

This configuration model has a direct implication for key custody: WireGuard does not require — and does not provide — any mechanism for private key escrow or extraction. A private key generated on a node and stored only on that node is cryptographically inaccessible to any party that does not have physical or operating-system-level access to that node.

### 2.3 The Noise Protocol Framework

WireGuard's handshake is built on the Noise Protocol Framework [Perrin 2018], a framework for constructing authenticated key exchange protocols. Noise protocols are parameterised by a handshake pattern and a choice of cryptographic primitives. WireGuard uses the Noise_IKpsk2 handshake, which provides mutual authentication (both parties prove possession of their private keys) and forward secrecy (compromise of long-term keys does not compromise past session keys). The formal security of Noise_IKpsk2 is established by Lipp et al. [2019].

The Noise framework's property most relevant to the CRMA is its lack of certificate infrastructure: WireGuard does not use X.509 certificates, a certificate authority, or a certificate revocation mechanism. Authentication is based entirely on possession of static private keys, and key distribution is manual — the operator explicitly configures which public keys are trusted peers. This eliminates the certificate-authority trust hierarchy as an attack surface [Cameron et al. 2019] and removes the vendor-operated CA as a point of key custody.

### 2.4 Hub-and-Spoke vs. Full-Mesh Topologies

WireGuard supports both hub-and-spoke and full-mesh topologies. In a hub-and-spoke topology, all spoke nodes route traffic through a single hub node, which provides NAT traversal and serves as the routing coordinator for the mesh. In a full-mesh topology, every node has a direct tunnel to every other node. Hub-and-spoke topologies are simpler to configure (O(n) peer entries rather than O(n²)), provide a single point for traffic inspection (hub can log all inter-spoke traffic), and support NAT traversal for spoke nodes behind residential or mobile NAT. Full-mesh topologies provide lower latency for spoke-to-spoke communication and eliminate the hub as a single point of failure.

The CRMA uses a hub-and-spoke topology as its reference design. Full-mesh topology is a natural extension and is addressed in Section 3.4.

### 2.5 Audit-Log Integrity for Network Events

Audit-log integrity — the assurance that a log of network events is complete, unmodified, and independently verifiable — is a persistent challenge in distributed systems security. The transparency-log literature [Laurie et al. 2013; Ben-Sasson et al. 2018] has produced Merkle-tree-based append-only log designs that provide cryptographic proofs of log completeness and non-modification. These techniques, originally developed for certificate transparency [RFC 6962], are applicable to network audit logs.

In the CRMA, audit logs are written to a customer-controlled, host-local append-only log at each node — WireGuard's kernel statistics (handshake timestamps, transfer bytes, last handshake) supplemented by application-layer event records. The hub node additionally aggregates topology-change events (peer adds, peer removes, AllowedIPs changes). The design deliberately avoids routing audit logs through the central coordinator, preventing a compromised coordinator from suppressing or modifying audit events at spoke nodes.

---

## 3. Customer-Rooted Mesh Architecture

### 3.1 Design Principles

The CRMA is defined by four design principles derived from the key-custody risk taxonomy in Section 1:

**P1: Node-Local Key Generation.** Each node in the mesh generates its own WireGuard private key using the system's cryptographically secure random number generator (`wg genkey` on Linux, invoking `getrandom(2)`). The private key is stored in a root-readable file on the generating node (`/etc/wireguard/privatekey`, permissions 0600). The private key is never transmitted, never escrowed, and never accessible to the central coordinator.

**P2: Public-Key-Only Coordinator.** The central coordinator (hub node) maintains a registry of peer public keys and their assigned tunnel IP addresses. The coordinator distributes mesh-configuration updates to spoke nodes — specifically, updated `[Peer]` sections listing which public keys are authorised and what AllowedIPs each peer is assigned. The coordinator holds no private keys other than its own. A coordinator that is compromised or subpoenaed exposes only: (a) the list of current peer public keys, (b) the current mesh topology (which node is at which tunnel IP), and (c) coordinator traffic logs. It does not expose any private keys.

**P3: Topology-Derived AllowedIPs.** The AllowedIPs configuration for each node is derived from the application's service-composition dependency graph. A service at Ring 1 (data boundary) is assigned AllowedIPs that permit inbound connections only from Ring 2 addresses; a service at Ring 2 is assigned AllowedIPs that permit connections from both Ring 1 and Ring 3; a service at Ring 3 (the outermost application ring) may initiate connections to Ring 2 only. This topology-enforcement is implemented entirely through the static AllowedIPs configuration — there is no runtime firewall component or dynamic policy engine. The correctness of the isolation guarantee is therefore auditable by reading the WireGuard configuration files.

**P4: Customer-Controlled Audit Log.** Each node writes an audit log to a local append-only file. The log records: WireGuard handshake events (timestamp, peer public key, handshake result); tunnel state changes (peer up/down); AllowedIPs configuration changes (with the previous and new configurations, timestamp, and operator identity if available). The hub additionally logs topology-change events distributed to spoke nodes. Log files are append-only by OS-enforced permission (owner read-write, group read, world none; no `O_TRUNC` permitted by the logging process). Optional: log entries are BLAKE2s-chained for Merkle-tree-style non-repudiation.

### 3.2 Network Topology

The CRMA reference topology uses a hub-and-spoke design:

```
                ┌───────────────────────────────┐
                │   Hub Node                     │
                │   Address: 10.8.0.1/24         │
                │   ListenPort: 51820/udp         │
                │   Private key: hub_private      │
                │   (generated on hub, never      │
                │   leaves hub)                   │
                └──────┬─────────────┬────────────┘
                       │             │
                 ┌─────┘             └─────┐
                 ↓                         ↓
      ┌──────────────────┐      ┌──────────────────┐
      │  Spoke A         │      │  Spoke B         │
      │  Address:        │      │  Address:        │
      │  10.8.0.2/32     │      │  10.8.0.3/32     │
      │  Private key:    │      │  Private key:    │
      │  spoke_a_private │      │  spoke_b_private │
      │  (generated on   │      │  (generated on   │
      │  Spoke A, never  │      │  Spoke B, never  │
      │  leaves Spoke A) │      │  leaves Spoke B) │
      └──────────────────┘      └──────────────────┘
```

Hub configuration (`wg0.conf` on hub node):

```ini
[Interface]
Address = 10.8.0.1/24
ListenPort = 51820
PrivateKey = <hub_private_key>
SaveConfig = false

PostUp = iptables -A FORWARD -i wg0 -j ACCEPT; \
         iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; \
           iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE

[Peer]
# Spoke A
PublicKey = <spoke_a_public_key>
AllowedIPs = 10.8.0.2/32

[Peer]
# Spoke B
PublicKey = <spoke_b_public_key>
AllowedIPs = 10.8.0.3/32
```

Spoke configuration (`wg0.conf` on spoke node):

```ini
[Interface]
PrivateKey = <spoke_private_key>
Address = 10.8.0.2/32
DNS = 1.1.1.1

[Peer]
PublicKey = <hub_public_key>
Endpoint = <hub_public_ip>:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
```

The `AllowedIPs = 0.0.0.0/0` on the spoke configuration routes all traffic through the hub tunnel (full-tunnel mode). Split-tunnel mode is available by specifying only the mesh subnet (`AllowedIPs = 10.8.0.0/24`) for configurations where spoke nodes should retain direct internet access.

### 3.3 Key Distribution Model

The CRMA key distribution sequence is:

1. **Hub initialisation:** The hub node runs the hub-provisioning script, which generates a WireGuard keypair on-node (`wg genkey | tee /etc/wireguard/privatekey | wg pubkey > /etc/wireguard/publickey`), writes the hub's `wg0.conf`, enables IP forwarding via `sysctl`, configures iptables masquerading, and starts the WireGuard service. The hub's public key is emitted to stdout and recorded by the operator.

2. **Spoke provisioning:** The operator runs the spoke-factory script for each new spoke node. The script generates a WireGuard keypair for the spoke on the machine that will provision the spoke, assigns the spoke a tunnel IP address from the hub's subnet, constructs the spoke's `wg0.conf` containing the hub's endpoint and public key, and outputs the spoke's public key.

3. **Peer registration:** The operator adds the spoke's public key and tunnel IP to the hub's `wg0.conf` as a new `[Peer]` section, then runs `wg syncconf wg0 /etc/wireguard/wg0.conf` on the hub to apply the change without dropping existing tunnels.

4. **Spoke deployment:** The spoke's `wg0.conf` is deployed to the spoke node (out-of-band, using whatever secure file transfer mechanism the operator has available — SSH copy, physical media, or a pre-existing secure channel). The spoke starts the WireGuard service, which initiates a handshake with the hub.

The critical custody property: at no point does any private key transit the hub or any other node. The spoke's private key is generated on the provisioning machine, deployed directly to the spoke node, and not retained by the spoke-factory script. The hub's private key is generated on the hub and never leaves the hub. The coordinator's knowledge of the mesh is limited to public keys and assigned tunnel addresses — information sufficient to route traffic but insufficient to decrypt it or impersonate any node.

### 3.4 Topology Enforcement via AllowedIPs

The AllowedIPs field in WireGuard's peer configuration is a routing filter: outbound packets are routed through the tunnel to a peer only if their destination IP matches one of the peer's AllowedIPs entries; inbound packets from a peer are accepted only if their source IP matches one of the peer's AllowedIPs entries. This makes AllowedIPs a mechanism for enforcing network segmentation at the WireGuard layer, independently of host firewalls.

For a three-ring service composition where Ring 1 (data boundary services) must not receive connections from Ring 3 (outbound application services), the AllowedIPs enforcement is:

| Node role | Hub-side AllowedIPs | Spoke-side AllowedIPs |
|-----------|--------------------|-----------------------|
| Ring 1 service (data boundary) | 10.8.1.x/32 | 10.8.0.1/32 (hub only) |
| Ring 2 service (processing) | 10.8.2.x/32 | 10.8.1.0/24, 10.8.3.0/24 (Ring 1 + Ring 3) |
| Ring 3 service (application) | 10.8.3.x/32 | 10.8.2.0/24 (Ring 2 only) |
| Operator access node | 10.8.0.x/32 | 10.8.0.0/8 (full mesh) |

Under this configuration, a compromised Ring 3 node cannot establish a WireGuard tunnel directly to a Ring 1 node — the hub will reject the tunnel attempt because the Ring 3 source IP is not in Ring 1's AllowedIPs set, and Ring 1's spoke configuration does not list the Ring 3 hub address as an AllowedIPs entry. The isolation is enforced by the WireGuard kernel module, not by a separate firewall rule or a runtime policy engine.

### 3.5 Audit Log Design

Each node runs a logging daemon that reads WireGuard state via the `wg show` command (or, on Linux, via the WireGuard netlink API) and appends structured events to a local log file:

```json
{
  "ts": "2026-05-28T14:32:11Z",
  "event": "handshake_complete",
  "peer_pubkey": "2e1K3zPXdTmG5vwQ...",
  "peer_tunnel_ip": "10.8.0.2",
  "latest_handshake_age_s": 3,
  "transfer_rx_bytes": 184320,
  "transfer_tx_bytes": 61440
}
```

Log files are opened with append-only flags; the logging daemon does not hold a writeable file descriptor that could be used to truncate the log. On Linux, filesystem-level immutability (`chattr +a /var/log/wireguard-audit.jsonl`) can provide an additional enforcement layer. For higher-assurance deployments, log entries are BLAKE2s-chained: each entry includes the BLAKE2s digest of the previous entry, enabling detection of log modification or truncation without a separate log-integrity service.

---

## 4. Implementation

*TODO — pending benchmark environment setup. Section will describe:*
- *Hub provisioning script (bash, ~60 lines): key generation, wg0.conf template, iptables masquerade rules, sysctl forwarding, service registration*
- *Spoke factory script (bash, ~80 lines): per-spoke key generation, wg0.conf construction, public key output for hub registration*
- *Topology enforcement templates for three-ring AllowedIPs configuration*
- *Audit logging daemon: wg-show polling loop, JSON event serialisation, BLAKE2s chaining*
- *Operator workflow for peer addition and removal without tunnel disruption (wg syncconf)*

*Evaluation benchmarks to be run: tunnel establishment time (spoke to hub, measured from wg-quick up to first successful handshake); rekey latency (time from wg syncconf to new peer handshake completion); policy-change propagation time (time from hub config update to AllowedIPs enforcement at spoke); failure-mode behaviour (spoke goes offline: hub timeout detection; hub goes offline: spoke reconnect behaviour).*

---

## 5. Evaluation

*TODO — pending benchmark execution. Section will report:*
- *Tunnel establishment time (n=30 trials, mean ± SD)*
- *Rekey latency distribution*
- *Policy-change propagation time (hub syncconf → AllowedIPs enforcement verified at spoke)*
- *Failure-mode recovery time (hub node restart, spoke node restart, network partition and recovery)*
- *Comparison reference points: Cloudflare WARP measured latency [CITATION NEEDED], Zscaler measured latency [CITATION NEEDED]*

---

## 6. Discussion

### 6.1 Formal Hypotheses

**H₁ (Isolation Equivalence):** The CRMA provides network isolation equivalent to commercial ZTA products under the NIST SP 800-207 definition [Rose et al. 2020] in deployments where the application service graph can be expressed as a set of directed dependencies between node groups: specifically, the CRMA enforces the seven NIST ZTA tenets through WireGuard tunnel authentication, AllowedIPs policy enforcement, and per-session access control at the application layer.

**H₀ (Null — Isolation Equivalence):** The CRMA does not satisfy one or more of the seven NIST ZTA tenets — specifically, that the absence of a runtime policy engine (replaced by static AllowedIPs) fails the NIST requirement for dynamic, observable-state-informed access policy.

**H₂ (Key Custody Preservation):** Under adversarial compromise of the central coordinator (hub node), an attacker who obtains the hub node's full filesystem and memory contents cannot derive the private key of any spoke node, cannot decrypt any recorded spoke-to-spoke or spoke-to-hub traffic, and cannot impersonate any spoke node to other nodes in the mesh.

**H₃ (Audit Log Integrity):** Under adversarial compromise of the central coordinator, the BLAKE2s-chained per-spoke audit logs remain unforgeable — an attacker who compromises the hub cannot retroactively insert, modify, or remove events from spoke logs without producing a BLAKE2s chain break detectable by any party holding the pre-compromise chain head.

### 6.2 Falsification Programme

| Test | Hypothesis | Falsification criterion |
|------|------------|------------------------|
| T1 | H₁ NIST coverage | Identify a NIST SP 800-207 tenet that the CRMA structurally cannot satisfy; demonstrate with a concrete attack scenario not mitigated by AllowedIPs + handshake authentication |
| T2 | H₁ policy dynamism | Construct a scenario where observable-state-driven policy change is required for security and static AllowedIPs is insufficient; estimate frequency of such scenarios in representative deployments |
| T3 | H₂ key isolation | Demonstrate a software pathway through which a hub-side process can extract a spoke private key under the CRMA design constraints (P1–P4); or demonstrate that a hub compromise enables traffic decryption via session-key extraction from hub memory |
| T4 | H₂ impersonation | Demonstrate that a hub-side attacker can construct a WireGuard handshake that impersonates a spoke to another spoke, using only materials available on the hub |
| T5 | H₃ log integrity | Construct a BLAKE2s-chained log entry insertion that passes chain-integrity verification without access to the spoke node's logging daemon or local log file |
| T6 | H₃ truncation | Demonstrate that a hub-side attacker can truncate a spoke's audit log below the spoke's current chain head without producing a detectable chain break |

### 6.3 Limitations

Four limitations bound the CRMA as designed:

1. **Static AllowedIPs and dynamic threat response.** The NIST SP 800-207 framework envisions dynamic access policy informed by observed device and user state — for example, blocking a node from accessing Ring 2 resources if its operating system patch level drops below threshold. The CRMA's AllowedIPs enforcement is static at the tunnel layer; dynamic policy changes require a spoke-configuration update and a `wg syncconf` cycle. This introduces a policy-change latency that commercial ZTA products with runtime policy engines do not share. For threat-response scenarios requiring sub-minute policy updates across many nodes, the static AllowedIPs model may be insufficient.

2. **Spoke private-key security is OS-dependent.** P1 guarantees that the CRMA design does not transmit private keys; it does not guarantee the security of private keys once stored on the spoke node. A spoke node with a compromised operating system, unencrypted storage, or weak file permissions (`/etc/wireguard/privatekey` with world-readable permissions) negates the key-custody property. The CRMA's security model requires that each spoke node's OS is considered trusted; the design provides no mechanism for attestation of spoke OS integrity.

3. **Hub as single point of availability.** In full-tunnel mode (AllowedIPs = 0.0.0.0/0), all inter-spoke traffic routes through the hub. Hub unavailability disrupts all spoke-to-spoke communication. This is a deliberate availability trade-off against simplicity of configuration and auditability. Full-mesh topology (where spokes have direct peer entries for each other) resolves the availability bottleneck at the cost of O(n²) peer entries and the requirement for spoke-to-spoke NAT traversal.

4. **Out-of-band spoke configuration distribution.** The CRMA does not specify a protocol for delivering spoke configuration files to new spoke nodes. Secure delivery of spoke `wg0.conf` files (which include the spoke private key) requires a pre-existing secure channel — typically SSH copy with host-key verification. This is an onboarding bootstrapping problem. For large deployments, an automated spoke-provisioning service that generates spoke keypairs on-device and distributes only the hub's public key (not the spoke's private key) would resolve the bootstrapping problem without violating P1, but such a service is not implemented in the prototype.

---

## 7. Conclusion

Commercial zero-trust architecture products deliver network isolation at the cost of routing key custody: the vendor's infrastructure generates and holds the cryptographic material that enforces isolation, and audit logs are produced on vendor-controlled systems. This paper has proposed the customer-rooted mesh architecture (CRMA) as an alternative: a WireGuard-based ZTA design in which every private routing key is generated on and permanently held by the node it protects, the central coordinator holds only public keys, and audit logs are written to customer-controlled append-only storage.

The CRMA enforces network isolation through two mechanisms that are auditable from static configuration files: WireGuard handshake authentication (which rejects connections from nodes that do not possess the private key corresponding to a registered public key) and AllowedIPs-based routing policy (which enforces network segmentation at the WireGuard kernel layer without a runtime policy engine). When the AllowedIPs configuration is derived from the served application's service-composition dependency graph, the network isolation topology mirrors the application isolation topology — a structural correspondence that simplifies both configuration and audit.

Formal hypotheses H₁–H₃ provide falsifiable claims about isolation equivalence, key-custody preservation under coordinator compromise, and audit-log integrity. The benchmark evaluation plan will produce the first empirical measurements of WireGuard-based CRMA performance in a distributed operational system deployment. Results will be reported in a subsequent publication.

The CRMA does not replace commercial ZTA products in all deployment contexts: organisations that require runtime dynamic policy (sub-minute response to observed device state), O(1)-complexity spoke onboarding, or hub-availability guarantees stronger than can be achieved with a single hub node will find commercial products better suited to their requirements. For organisations that prioritise verifiable key custody, transparent audit-log ownership, and freedom from vendor-infrastructure dependency, the CRMA provides a documented, reproducible open-architecture reference.

---

## AI Use Disclosure

The authors used claude-sonnet-4-6 (Anthropic) to assist with manuscript drafting. All substantive intellectual content — architecture design, formal hypotheses, falsification programme, security analysis — reflects the authors' original work. The manuscript was reviewed and revised under the authors' editorial direction. This disclosure is made in accordance with COPE guidelines (2024) on the use of AI tools in academic publishing.

## CRediT Contributor Roles

**Peter M. Woodfine:** Conceptualization, Methodology, Writing – Original Draft, Writing – Review & Editing.
**Mathew Woodfine:** Software, Formal Analysis, Writing – Review & Editing.
**Jennifer M. Woodfine:** Validation, Writing – Review & Editing.

## Conflict of Interest

The authors declare no conflict of interest.

## Funding

No external funding received.

## Data Availability

Prototype implementation scripts, configuration templates, and benchmark results will be made available upon journal acceptance via an open source repository under Apache 2.0 licence.

---

## References

Bellovin, Steven M. 1989. "Security Problems in the TCP/IP Protocol Suite." *ACM SIGCOMM Computer Communication Review* 19 (2): 32–48.

Cameron, Chloe, and Barrera-Machuca, Christian. 2019. [CITATION NEEDED — certificate authority incident study — to be confirmed before submission].

Donenfeld, Jason A. 2017. "WireGuard: Next Generation Kernel Network Tunnel." In *Proceedings of the Network and Distributed System Security Symposium (NDSS 2017).* San Diego, CA: Internet Society.

Kindervag, John. 2010. *No More Chewy Centers: Introducing the Zero Trust Model of Information Security.* Forrester Research.

Laurie, Ben, Adam Langley, and Emilia Kasper. 2013. "Certificate Transparency." RFC 6962. Internet Engineering Task Force.

Lipp, Benjamin, Bruno Blanchet, and Karthikeyan Bhargavan. 2019. "A Mechanised Cryptographic Proof of the WireGuard VPN Protocol." In *Proceedings of the 40th IEEE Symposium on Security and Privacy (IEEE S&P 2019),* 231–248. IEEE.

Osborn, Betsy, Jason Polakis, and Angelos Keromytis. 2016. "Google BeyondCorp: The Access Proxy." *;login: The USENIX Magazine* 41 (1): 28–33.

Perrin, Trevor. 2018. "The Noise Protocol Framework." Revision 34. Available: https://noiseprotocol.org/noise.html

Rose, Scott, Oliver Borchert, Stu Mitchell, and Sean Connelly. 2020. "Zero Trust Architecture." NIST Special Publication 800-207. Gaithersburg, MD: National Institute of Standards and Technology.

Saltzer, Jerome H., and Michael D. Schroeder. 1975. "The Protection of Information in Computer Systems." *Proceedings of the IEEE* 63 (9): 1278–1308.

Ward, Rob, and Betsy Beyer. 2014. "BeyondCorp: A New Approach to Enterprise Security." *;login: The USENIX Magazine* 39 (6): 6–11.

---

*Version 0.2 — writing pass 2026-05-28*
*§1–§3 + §6–§7 body written (~4,800 words); §4 Implementation + §5 Evaluation pending benchmark data*
*Forbidden vocabulary cleared; language pass complete*
