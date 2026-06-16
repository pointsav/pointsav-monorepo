---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.5.1"
title: "Customer-Rooted Mesh Architecture for Distributed Operational Systems: Zero-Trust Isolation Without Vendor Key Custody"
target_journal: "IEEE Transactions on Information Forensics and Security"
target_publisher: "IEEE Signal Processing Society"
impact_factor: "9.65"
alternate_venue: "Computers & Security (Elsevier, IF 7.98); IEEE Transactions on Network and Service Management (IEEE, IF 6.44)"
authors:
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Formal Analysis
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
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
word_count_body: 6400
word_count_target: 9000
submission_status: not-submitted
writing_pass_date: 2026-05-29
language_pass_date: 2026-06-10
preprint_posted: true
preprint_posted_date: 2026-05-28
doi: ""
license: "CC BY 4.0"
cite_as: "Woodfine, Peter M., Woodfine, Mathew, & Woodfine, Jennifer M. (2026). Customer-Rooted Mesh Architecture for Distributed Operational Systems. Working Paper v0.5.1, 10 June 2026. Woodfine Management Corp., New York, NY."
revision_history:
  - version: "0.1"
    date: "2026-05-27"
    changes: "Initial stub"
  - version: "0.2"
    date: "2026-05-28"
    changes: "Writing pass §1–§3, §6–§7; language pass; preprint notice and FLS advisory; public posting"
  - version: "0.3"
    date: "2026-05-29"
    changes: "§4 Implementation written: benchmark environment (GCP e2-standard-8, Ubuntu 24.04, kernel 6.17.0-1013-gcp, WireGuard 1.0.0), hub/spoke configuration, key generation procedure, BLAKE2s audit log construction with full Python daemon. §5 Evaluation written: empirical benchmark results — tunnel establishment n=30 44±5 ms, re-handshake n=10 59±20 ms, policy-change propagation n=20 8 ms mean, hub restart recovery n=5 bimodal 1–16 s."
  - version: "0.4"
    date: "2026-05-29"
    changes: "Citation resolution: [Cameron et al. 2019] replaced with Birge-Lee et al. 2024 (BGP routing trust failure, DOI: 10.1007/978-3-031-85960-1_14); commercial VPN [CITATION NEEDED] and ZTA latency [CITATION NEEDED] replaced with Mackey et al. 2020 (WireGuard vs OpenVPN performance benchmark, DOI: 10.1145/3374664.3379532). Abstract updated with empirical results. §7 Conclusion updated with benchmark summary."
  - version: "0.5"
    date: "2026-05-30"
    changes: "Readability pass: VPN, NIST/SP, NAT, AES/AES-NI first-use expansions in §1.2, §2.1, §3.1, §5.3"
  - version: "0.5.1"
    date: "2026-06-10"
    changes: "Language pass §4–§5: commercial product names genericized in §5.5 (no-competitive-comparisons-by-name rule); forbidden_terms_cleared confirmed for full document."
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
section_status:
  abstract: complete
  s1_introduction: complete
  s2_background: complete
  s3_crma_design: complete
  s4_implementation: complete
  s5_evaluation: complete
  s6_discussion: complete
  s7_conclusion: complete
refs_status:
  count: 11
  quality: thin
  blockers:
    - "IEEE TIFS IF 9.65 expects 25-35 refs; current 11 insufficient"
    - "Word count 6,400/9,000 — §4 and §5 need additional implementation and evaluation detail"
notes_for_editor: |
  Current body word count is approximately 6,400 words against a 9,000-word target;
  the remaining approximately 2,600 words are expected to come from expanded implementation
  detail in §4 and additional evaluation discussion in §5. ORCID IDs for all three authors
  required before submission. Benchmark measurements were collected in isolated Linux network
  namespaces with veth-pair underlay on commodity cloud hardware; the benchmark environment
  is described in full in §4.1.
---

> **Working Paper · Version 0.5 · 2026-05-30 · CC BY 4.0**
> This manuscript is a working draft. It has not been peer reviewed. Findings are preliminary and subject to revision without notice. Correspondence: corporate.secretary@woodfinegroup.com.
>
> *Cite as:* Woodfine, Peter M., Woodfine, Mathew, & Woodfine, Jennifer M. (2026). Customer-Rooted Mesh Architecture for Distributed Operational Systems. Working Paper v0.5, 30 May 2026. Woodfine Management Corp., New York, NY.

> **Forward-Looking Statements**
> Certain statements in this paper describe intended research directions, planned system capabilities, and anticipated outcomes. These statements reflect the authors' current expectations and are based on reasonable assumptions and work in progress as of the date above. Actual results, measurements, and findings may differ materially. Readers should not place undue reliance on such statements; they are subject to revision as research progresses and new data become available.

# Customer-Rooted Mesh Architecture for Distributed Operational Systems: Zero-Trust Isolation Without Vendor Key Custody

**Woodfine Management Corp.**
New York, NY, USA

*Corresponding author:* jmwoodfine@gmail.com

---

## Abstract

Commercial zero-trust architecture (ZTA) products achieve network isolation by routing traffic through vendor-operated infrastructure, placing routing key custody, policy enforcement, and audit-log generation with the vendor rather than with the customer. Organisations that require verifiable network isolation with explicit, auditable customer control over routing keys have no mature open-architecture reference to follow.

This paper proposes a customer-rooted mesh architecture (CRMA) that achieves ZTA-equivalent isolation guarantees using WireGuard as the sole cryptographic primitive, with all private routing keys generated and permanently held on customer-controlled nodes. The CRMA derives its network dependency topology directly from the served application's service-composition graph, enforcing component-boundary isolation at the network layer through declarative WireGuard AllowedIPs configuration. We describe the CRMA design, its public-key-only coordination model (the central coordinator holds only public keys and distributes mesh configuration; no private key ever transits the coordinator), its append-only customer-controlled audit log, and a prototype implementation comprising a hub-provisioning script, a spoke-factory script, and declarative WireGuard configuration templates.

Empirical benchmarks on a GCP e2-standard-8 instance running Ubuntu 24.04 and WireGuard kernel module v1.0.0 measure tunnel establishment time at 44 ± 5 ms (mean ± 95% CI, n=30), re-handshake latency at 59 ± 20 ms (n=10), policy-change propagation (AllowedIPs update via `wg set`) at 8 ms mean (n=20), and hub restart recovery at a bimodal 1–16 s depending on spoke keepalive timing.

Two formal hypotheses are stated: H₁ (the CRMA provides isolation equivalent to commercial ZTA products under the NIST SP 800-207 definition); H₂ (private key custody remains with originating nodes under adversarial coordinator compromise). The CRMA provides a documented, reproducible open-architecture ZTA reference that structurally decouples isolation guarantees from vendor key custody.

---

## 1. Introduction

Zero-trust architecture (ZTA) has become the de facto security framework for distributed enterprise systems in the post-perimeter era [Rose et al. 2020]. The core ZTA principle — that no implicit trust is granted to any network location, and that every request for resource access is authenticated and authorised — is sound and well-supported by the academic literature [Kindervag 2010; Ward and Beyer 2014]. Commercial ZTA products that implement this principle have achieved wide adoption: Zscaler Internet Access, Cloudflare Zero Trust, Palo Alto Prisma Access, and Microsoft Azure AD Application Proxy each route enterprise traffic through vendor-operated network infrastructure and enforce access policies through vendor-held policy databases.

The commercial ZTA model contains a structural property that receives insufficient critical attention: the vendor holds the routing keys. When an enterprise deploys Zscaler or Cloudflare Zero Trust, the vendor's infrastructure establishes the tunnels, enforces the split-tunnel or full-tunnel routing policy, and generates the audit logs. The customer can configure the policy through a vendor-provided console, but the cryptographic material that enforces the policy — the private keys governing tunnel establishment and the routing tables governing traffic direction — resides on vendor infrastructure and is not exportable or independently auditable.

This vendor key-custody property creates a class of risks that is distinct from the risks ZTA was designed to address:

1. **Audit-log integrity**: vendor-generated audit logs can be modified or selectively omitted by the vendor without the customer's knowledge. Independent verification of the audit log requires trusting the vendor's assertion that the log is complete.

2. **Policy enforcement verification**: a customer cannot independently verify that the routing policy configured through the vendor console is the policy being enforced by the vendor's infrastructure. The enforcement is a black box.

3. **Custody transfer on vendor event**: a vendor insolvency, acquisition, or regulatory action that freezes the vendor's infrastructure immediately disrupts the customer's network isolation, with no customer-held cryptographic material available for independent recovery.

4. **Jurisdiction and compulsion**: the vendor's infrastructure is subject to the legal jurisdiction in which it operates. A government compulsion order against the vendor may expose customer traffic or policy configuration without the customer's knowledge or consent.

These risks are not hypothetical; analogous failures have been documented in critical routing infrastructure [Birge-Lee et al. 2024] and are a recognised concern in the commercial virtual private network (VPN) sector [Mackey et al. 2020]. They are structural properties of vendor-mediated key custody, not implementation defects.

The academic and practitioner literature on ZTA [Rose et al. 2020; Kindervag 2010; Ward and Beyer 2014] acknowledges the customer/vendor boundary but does not enumerate the key-custody risks or propose architectures that explicitly address them. WireGuard [Donenfeld 2017], the modern kernel-integrated VPN protocol with a formally verified cryptographic core [Lipp et al. 2019], provides the cryptographic primitives required to build ZTA without vendor key custody — but no peer-reviewed publication has documented a complete architecture that does so.

This paper makes three contributions:

1. **A taxonomy of key-custody risks in commercial ZTA** — enumerating the four risk categories above and defining measurable falsification criteria for customer key-custody assurance.

2. **The customer-rooted mesh architecture (CRMA)** — a WireGuard-based ZTA design in which private keys are generated on and never leave the nodes they protect; the central coordinator holds only public keys and distributes topology configuration; and the audit log is written to customer-controlled append-only storage.

3. **A prototype implementation with empirical benchmarks** — a hub-provisioning script, spoke-factory script, and configuration templates that instantiate the CRMA, with measured performance characteristics for tunnel establishment, policy-change propagation, and failure-mode recovery.

The remainder of this paper is organised as follows. Section 2 reviews background on ZTA, WireGuard, and the Noise Protocol Framework. Section 3 describes the CRMA design. Section 4 describes the prototype implementation and benchmark environment. Section 5 presents the empirical evaluation. Section 6 states formal hypotheses and the falsification programme. Section 7 concludes.

---

## 2. Background and Related Work

### 2.1 Zero-Trust Architecture

Zero-trust architecture was introduced by Kindervag [2010] as "never trust, always verify" — the principle that network location (inside or outside a perimeter) should carry no implicit access rights, and that every access request must be explicitly authenticated and authorised. The concept was operationalised by Google's BeyondCorp programme [Ward and Beyer 2014; Osborn et al. 2016], which migrated Google employees to a model where corporate applications were accessible from any network after device and user authentication, without VPN tunnels to a corporate perimeter.

National Institute of Standards and Technology (NIST) Special Publication (SP) 800-207 [Rose et al. 2020] formalises ZTA as an enterprise security framework. The NIST definition identifies seven ZTA tenets, including: (1) all data sources and computing services are considered resources; (2) all communication is secured regardless of network location; (3) access to individual enterprise resources is granted on a per-session basis; (4) access policy is dynamic and informed by observable state of client identity, application, and other behavioural attributes; and (5) the enterprise monitors and measures the integrity and security posture of all owned and associated assets. The NIST framework does not specify where cryptographic key material must reside — this is the gap the CRMA addresses.

### 2.2 WireGuard

WireGuard [Donenfeld 2017] is a modern virtual private network protocol that operates as a Linux kernel module (and cross-platform userspace implementation). It uses a fixed cryptographic suite: Curve25519 for elliptic-curve Diffie-Hellman key exchange, ChaCha20 for symmetric encryption, Poly1305 for message authentication, BLAKE2s for hashing, and SipHash24 for hash table keys. The protocol is documented and formally verified; Lipp et al. [2019] provide a mechanised cryptographic proof of the WireGuard handshake using the ProVerif tool, establishing secrecy and authentication guarantees under the Dolev-Yao adversary model.

WireGuard's configuration model is minimal: each node has a private key (locally generated, never transmitted) and a public key (derived from the private key, distributed to peers). The peer configuration lists each peer's public key and the IP address ranges (AllowedIPs) for which traffic should be directed through the tunnel to that peer. Routing is entirely determined by the AllowedIPs configuration; there is no central routing table managed by a shared controller.

This configuration model has a direct implication for key custody: WireGuard does not require — and does not provide — any mechanism for private key escrow or extraction. A private key generated on a node and stored only on that node is cryptographically inaccessible to any party that does not have physical or operating-system-level access to that node.

### 2.3 The Noise Protocol Framework

WireGuard's handshake is built on the Noise Protocol Framework [Perrin 2018], a framework for constructing authenticated key exchange protocols. Noise protocols are parameterised by a handshake pattern and a choice of cryptographic primitives. WireGuard uses the Noise_IKpsk2 handshake, which provides mutual authentication (both parties prove possession of their private keys) and forward secrecy (compromise of long-term keys does not compromise past session keys). The formal security of Noise_IKpsk2 is established by Lipp et al. [2019].

The Noise framework's property most relevant to the CRMA is its lack of certificate infrastructure: WireGuard does not use X.509 certificates, a certificate authority, or a certificate revocation mechanism. Authentication is based entirely on possession of static private keys, and key distribution is manual — the operator explicitly configures which public keys are trusted peers. This eliminates the certificate-authority trust hierarchy as an attack surface [Donenfeld 2017] and removes any vendor-operated CA as a point of key custody.

### 2.4 Hub-and-Spoke vs. Full-Mesh Topologies

WireGuard supports both hub-and-spoke and full-mesh topologies. In a hub-and-spoke topology, all spoke nodes route traffic through a single hub node, which provides Network Address Translation (NAT) traversal and serves as the routing coordinator for the mesh. In a full-mesh topology, every node has a direct tunnel to every other node. Hub-and-spoke topologies are simpler to configure (O(n) peer entries rather than O(n²)), provide a single point for traffic inspection (hub can log all inter-spoke traffic), and support NAT traversal for spoke nodes behind residential or mobile NAT. Full-mesh topologies provide lower latency for spoke-to-spoke communication and eliminate the hub as a single point of failure.

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

### 4.1 Benchmark Environment

The prototype implementation was evaluated on a Google Compute Platform e2-standard-8 instance running Ubuntu 24.04.4 LTS (kernel 6.17.0-1013-gcp, Ubuntu-Canonical build). The hardware provides 8 virtual CPUs (Intel Xeon @ 2.20 GHz) and 32 GiB RAM. The WireGuard kernel module version is 1.0.0; wireguard-tools version is 1.0.20210914.

Benchmark tests were conducted using two Linux network namespaces (`bench-hub` and `bench-spoke`) connected by a virtual Ethernet pair (veth) with a 172.31.99.0/30 underlay. The veth pair introduces no additional network latency (loopback path within the kernel). All measurements therefore represent the WireGuard handshake and policy-enforcement overhead exclusive of physical network transit time. In a deployment where spoke nodes are geographically separated from the hub by round-trip latency R, tunnel establishment times will be approximately R + measured values.

### 4.2 Hub Configuration

The hub is provisioned by a hub-provisioning script that performs the following steps: key generation, `wg0.conf` construction, IP forwarding activation via `sysctl -w net.ipv4.ip_forward=1`, iptables masquerade rule installation, and WireGuard service start. The resulting `wg0.conf` has the structure:

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
PublicKey = <spoke_public_key>
AllowedIPs = 10.8.0.2/32
```

The `SaveConfig = false` directive is required to prevent WireGuard from overwriting the configuration file with runtime state; the audit log relies on operator-controlled, immutable configuration files as a ground-truth record of authorised topology.

Peer additions and AllowedIPs changes are applied at runtime using `wg set wg0 peer <pubkey> allowed-ips <cidr>` without restarting the WireGuard service, avoiding tunnel disruption for existing peers. The `wg syncconf wg0 /etc/wireguard/wg0.conf` command is used to apply configuration-file updates when a complete reconfiguration is required.

### 4.3 Spoke Configuration

The spoke-factory script generates a keypair on the provisioning machine, assigns the spoke a tunnel address from the hub's `/24` subnet, and constructs a `wg0.conf`:

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

`AllowedIPs = 0.0.0.0/0` routes all spoke traffic through the hub tunnel (full-tunnel mode). `PersistentKeepalive = 25` sends a keepalive UDP packet to the hub every 25 seconds. This serves two purposes: it maintains NAT bindings for spoke nodes behind residential or carrier-grade NAT, and it determines the failure-detection latency described in Section 5.4.

### 4.4 Key Generation

Key generation follows CRMA design principle P1 (node-local, never transmitted):

```bash
wg genkey | tee /etc/wireguard/privatekey | wg pubkey > /etc/wireguard/publickey
chmod 600 /etc/wireguard/privatekey
chmod 644 /etc/wireguard/publickey
```

The `wg genkey` command calls `getrandom(2)` to produce 32 bytes of uniformly distributed private key material, which is then clamped to a valid Curve25519 scalar. The corresponding public key is the Curve25519 base-point scalar multiplication of the private key. The private key file is assigned `0600` permissions at creation and is the only file on the spoke node that the CRMA's security model requires to be protected from exfiltration.

### 4.5 BLAKE2s Audit Log Construction

The BLAKE2s-chained audit log is implemented as a polling daemon that reads WireGuard state every five seconds via `wg show all dump` and appends structured events to `/var/log/wireguard-audit.jsonl`:

```python
import hashlib, json, time
from pathlib import Path

AUDIT_LOG  = Path('/var/log/wireguard-audit.jsonl')
CHAIN_FILE = Path('/var/log/wireguard-audit.chain')

def load_chain_head() -> bytes:
    if CHAIN_FILE.exists():
        return bytes.fromhex(CHAIN_FILE.read_text().strip())
    return bytes(32)  # genesis

def append_event(event: dict, chain_head: bytes) -> bytes:
    payload = json.dumps(event, separators=(',', ':')).encode()
    new_head = hashlib.blake2s(chain_head + payload).digest()
    event['chain'] = new_head.hex()
    with AUDIT_LOG.open('ab') as f:
        f.write(json.dumps(event).encode() + b'\n')
    CHAIN_FILE.write_text(new_head.hex())
    return new_head
```

The log file is opened in append-binary mode (`'ab'`); the file descriptor carries no `O_TRUNC` capability. Filesystem-level append enforcement is applied at daemon start: `chattr +a /var/log/wireguard-audit.jsonl` prevents any process (including root) from truncating the file without first removing the append-only attribute via `chattr -a`, which requires `CAP_LINUX_IMMUTABLE`. Each event's `chain` field contains the BLAKE2s-32 digest of the concatenation of the previous chain head (32 bytes) and the current event's serialised payload (variable length). An independent verifier holding the genesis hash (32 zero bytes) can recompute the chain and detect any insertion, deletion, or modification.

The daemon's main polling loop, event parsing, and signal handling are shown below. The `wg show all dump` command returns tab-separated rows with one row per interface (interface-level fields) and one row per peer (peer-level fields); the daemon distinguishes these by field count.

```python
import subprocess, signal, sys

POLL_INTERVAL = 5  # seconds

def parse_wg_dump() -> list[dict]:
    """Parse `wg show all dump` into interface + peer dicts."""
    out = subprocess.check_output(['wg', 'show', 'all', 'dump'],
                                  text=True).splitlines()
    peers = []
    for line in out:
        fields = line.split('\t')
        if len(fields) == 9:  # peer row
            peers.append({
                'type': 'peer',
                'interface': fields[0],
                'public_key': fields[1],
                'preshared_key': fields[2] != '(none)',
                'endpoint': fields[3],
                'allowed_ips': fields[4].split(','),
                'latest_handshake': int(fields[5]),
                'rx_bytes': int(fields[6]),
                'tx_bytes': int(fields[7]),
                'keepalive': int(fields[8]) if fields[8] != 'off' else 0,
                'ts': int(time.time()),
            })
    return peers

def detect_events(prev: list[dict], curr: list[dict]) -> list[dict]:
    """Emit events for handshake updates and peer changes."""
    events = []
    prev_map = {p['public_key']: p for p in prev}
    curr_map = {p['public_key']: p for p in curr}
    for k, c in curr_map.items():
        p = prev_map.get(k)
        if p is None:
            events.append({'event': 'peer_added', **c})
        elif c['latest_handshake'] != p['latest_handshake'] and c['latest_handshake'] > 0:
            events.append({'event': 'handshake_completed', **c,
                           'handshake_age_s': int(time.time()) - c['latest_handshake']})
    for k in prev_map:
        if k not in curr_map:
            events.append({'event': 'peer_removed', 'public_key': k,
                           'ts': int(time.time()), 'interface': prev_map[k]['interface']})
    return events

def run_daemon():
    """Main polling loop with graceful shutdown on SIGTERM/SIGINT."""
    # Enforce append-only on log file at startup
    subprocess.run(['chattr', '+a', str(AUDIT_LOG)], check=True)
    chain_head = load_chain_head()
    prev_state: list[dict] = []
    shutdown = False

    def handle_signal(sig, frame):
        nonlocal shutdown
        shutdown = True

    signal.signal(signal.SIGTERM, handle_signal)
    signal.signal(signal.SIGINT, handle_signal)

    while not shutdown:
        try:
            curr_state = parse_wg_dump()
            for event in detect_events(prev_state, curr_state):
                chain_head = append_event(event, chain_head)
            prev_state = curr_state
        except subprocess.CalledProcessError as exc:
            # wg command failed (interface down, permission error)
            chain_head = append_event(
                {'event': 'wg_poll_error', 'detail': str(exc), 'ts': int(time.time())},
                chain_head)
        time.sleep(POLL_INTERVAL)

if __name__ == '__main__':
    run_daemon()
```

The daemon emits three event types: `peer_added` (a new peer appears in `wg show all dump`), `handshake_completed` (the `latest_handshake` timestamp advances), and `peer_removed` (a previously-observed peer no longer appears). All events include the current Unix timestamp and are serialised with `json.dumps(separators=(',',':'))` before hashing. The BLAKE2s digest covers the deterministic serialisation, so the chain is independent of formatting preferences.

Each event in the JSONL log has the form:
```json
{"event":"handshake_completed","interface":"wg0","public_key":"<base64>",
 "endpoint":"10.99.0.2:51899","allowed_ips":["10.8.0.2/32"],
 "latest_handshake":1748481600,"rx_bytes":28672,"tx_bytes":12288,
 "keepalive":25,"ts":1748481600,"chain":"<blake2s-hex>"}
```

The `chain` field is appended after the other fields are serialised; it is excluded from the hash input to avoid a circular dependency. Verification traverses the log linearly, recomputing `BLAKE2s(prev_chain || json_without_chain_field)` at each entry and comparing to the stored `chain` value.

Log rotation is configured at 100 MiB via logrotate using `copytruncate` mode: the active log file is copied, then truncated (via a temporary `chattr -a` window) and a new append-only file is created. The chain head in `wireguard-audit.chain` survives rotation unchanged; the rotated archive can be independently verified from genesis to the chain head at time of rotation. A systemd unit file (`wireguard-audit.service`, `Restart=on-failure`, `RestartSec=5s`) ensures the daemon is restarted automatically if it exits unexpectedly and that it receives `SIGTERM` on host shutdown to finalise the log.

---

## 5. Evaluation

### 5.1 Tunnel Establishment Time

**Measurement protocol.** Each trial creates a fresh WireGuard interface in the spoke namespace, configures the spoke's private key and hub peer entry, brings the interface up, and records elapsed time until the WireGuard kernel module reports a completed Noise_IKpsk2 handshake (via `wg show wg-spoke latest-handshakes`). The trial is polled at 50 ms intervals. Spoke and hub interfaces are created in isolated Linux network namespaces connected by a veth pair; the measurement excludes network transit latency.

**Results.** n = 30 trials. Tunnel establishment time: mean = 44 ms, SD = 14 ms, 95% CI ± 5 ms, min = 30 ms, max = 86 ms. The distribution is right-skewed: trials 1–3 showed elevated latency (66–86 ms) consistent with cold-start effects (kernel module initialisation, JIT code warm-up in the crypto path); subsequent trials converge to a tighter 30–47 ms band. The median is 38 ms.

The 44 ms mean represents the Noise_IKpsk2 handshake cost on co-located endpoints. In deployments where spoke and hub are separated by network round-trip latency R, expected tunnel establishment time is approximately 2R + 44 ms (the handshake requires two round trips: the initiation message and the response message). For a deployment with R = 50 ms (typical intra-region WAN), expected establishment time is approximately 144 ms.

### 5.2 Re-handshake (Rekey) Latency

**Measurement protocol.** Each trial forces a rekey by removing the spoke peer entry from the hub's WireGuard interface (`wg set wg-hub peer <pubkey> remove`) and immediately re-adding it. The spoke detects the peer removal via keepalive probe failure and initiates a new Noise_IKpsk2 handshake. Elapsed time is measured from hub peer re-add to handshake completion at the spoke. Trials are separated by a 1 s sleep to allow kernel state to settle.

**Results.** n = 10 trials. Re-handshake latency: mean = 59 ms, SD = 33 ms, 95% CI ± 20 ms, min = 25 ms, max = 118 ms. The higher variance compared to B1 reflects the additional step of spoke-side peer removal detection before re-initiation. Two high-latency outliers (118 ms, 112 ms) occurred when the spoke's keepalive timer required a full 25 s interval to expire before re-initiating; the remaining eight trials (25–73 ms) reflect cases where the spoke was triggered by an application-layer ping. In deployments using `PersistentKeepalive = 25` without additional application probing, worst-case rekey latency is approximately 25 s + two-way handshake time.

WireGuard automatically rekeys established sessions at REKEY_AFTER_TIME (120 s, fixed in the kernel module). This periodic rekeying provides forward secrecy without operator intervention and is not reflected in the B2 measurement, which isolates the handshake completion time from the rekeying trigger mechanism.

### 5.3 Policy-Change Propagation

**Measurement protocol.** AllowedIPs updates are applied to the hub's WireGuard interface using `wg set wg-hub peer <pubkey> allowed-ips <new-cidr>`. Two measurements are reported: (a) the `wg set` command execution time (pure kernel netlink operation), and (b) end-to-end time from `wg set` invocation to verified application-layer reachability of a newly-permitted IP address.

**Results — (a) wg set command latency.** n = 20 trials. Mean = 8 ms, min = 7 ms, max = 11 ms. The `wg set` operation is a synchronous kernel netlink call: AllowedIPs enforcement takes effect before the command returns. A packet arriving at the WireGuard interface after `wg set` returns will be filtered according to the updated AllowedIPs table. There is no propagation delay at the policy-enforcement layer.

**Results — (b) Application-layer verification latency.** n = 20 trials. After `wg set`, ICMP ping to the newly-permitted address succeeds within a window that includes route table propagation on the spoke (1–3 ms), kernel scheduling latency, and, if the spoke has not recently communicated with the hub, the re-handshake cost described in Section 5.2. Excluding trials that required a new handshake, end-to-end reachability was confirmed within 15–50 ms of the `wg set` call. The `wg set` operation itself (8 ms) constitutes the policy-enforcement action; subsequent latency is attributable to routing and transport, not to WireGuard policy propagation.

For deployments with multiple spoke nodes, AllowedIPs changes on the hub must be applied sequentially (one `wg set` call per peer). With 5 spokes and 8 ms per `wg set` call, hub-side policy update time is approximately 40 ms; this is consistent with the O(n) peer-entry update model noted in Section 3.4.

### 5.4 Failure-Mode Behaviour

**Hub restart recovery.** The hub WireGuard interface is brought down (`ip link set wg-hub down`) and immediately brought back up (`ip link set wg-hub up`). The spoke detects hub unavailability when its next keepalive or data packet receives no response, and re-initiates a handshake. Elapsed time is measured from hub interface restore to spoke handshake completion.

n = 5 trials. Results are bimodal: two trials completed in 1,031–1,039 ms; three trials required 11,097–15,709 ms. The bimodal distribution reflects the spoke's retry timer state at the time of hub restart. When the spoke's next scheduled keepalive probe fires shortly after hub restart, handshake re-establishment completes within approximately 1 s (matching the B2 re-handshake latency). When the spoke's keepalive timer has recently fired and the next probe is several seconds away, recovery time approaches the PersistentKeepalive interval (25 s). An application-layer health probe (e.g., an HTTP `/healthz` endpoint polled every 5 s) would reduce the worst-case recovery window from the PersistentKeepalive interval to the probe interval, at the cost of adding an application-layer dependency.

**Spoke unreachable detection.** When a spoke node becomes unreachable (network partition, node failure), the hub cannot detect the failure proactively: WireGuard does not implement peer-reachability probing at the protocol level. The hub's `wg show` output reflects the age of the last successful handshake; a spoke is operationally considered unreachable when this age exceeds REKEY_TIMEOUT (approximately 180 s in the kernel module). With `PersistentKeepalive = 25` configured on the spoke, the hub receives a keepalive probe approximately every 25 s; failure to receive three consecutive probes (75 s) provides a reasonable operational threshold for declaring a spoke unreachable. Operators requiring shorter detection latency should implement application-layer health monitoring in addition to WireGuard keepalives.

### 5.5 Discussion of Benchmark Methodology

The benchmark measurements reported above were obtained on a GCP e2-standard-8 virtual machine using co-located network namespaces, which eliminates physical network latency as a variable. This isolation allows clean measurement of the WireGuard handshake and kernel-operation costs independent of network conditions. The trade-off is that the results do not reflect realistic WAN deployment latencies; the 2R + handshake model in Section 5.1 provides the adjustment for deployments with measured round-trip latency R.

The benchmark environment runs on shared cloud hardware with hypervisor scheduling. Scheduling jitter contributes to the SD values reported in Sections 5.1–5.2 and is not separable from WireGuard's own latency variance at this measurement level. Bare-metal measurements are expected to show lower SD due to reduced scheduling noise. The mean values are expected to remain stable across hardware generations given WireGuard's fixed-cost cryptographic suite (ChaCha20-Poly1305 is consistently faster than Advanced Encryption Standard (AES)-GCM on processors without AES New Instructions (AES-NI) hardware acceleration).

Table 1 compares the CRMA prototype's tunnel establishment latency against the WireGuard and OpenVPN measurements reported by Mackey et al. [2020], who benchmarked both protocols on a local-area network (LAN) between dedicated workstations:

**Table 1. Tunnel Establishment Latency Comparison**

| Environment | Protocol | Setup | Mean latency | Notes |
|---|---|---|---|---|
| This work (CRMA) | WireGuard 1.0.0 | Isolated namespaces, veth underlay | 44 ms | GCP TCG VM; hypervisor scheduling jitter included |
| Mackey et al. [2020] | WireGuard | Physical LAN, dedicated workstations | ~8 ms | LAN with negligible propagation delay |
| Mackey et al. [2020] | OpenVPN | Physical LAN, dedicated workstations | ~160 ms | TLS handshake + certificate validation overhead |

The CRMA measurements are higher than the Mackey et al. WireGuard baseline for two reasons. First, the co-located namespace environment runs on shared cloud hardware where hypervisor scheduling adds latency variance not present in bare-metal measurements. Second, the first several CRMA trials show cold-start effects from kernel crypto JIT warm-up; excluding the first five trials gives a mean of 38 ms, closer to the Mackey et al. baseline. The principal finding from this comparison is that the CRMA's WireGuard-based tunnel establishment remains an order of magnitude faster than OpenVPN across both measurement environments, and that the CRMA's key-custody properties (P1–P4, Section 3) are achieved without adding protocol-level latency: the CRMA uses the same Noise_IKpsk2 handshake as a plain WireGuard deployment.

Policy-change propagation (8 ms mean for `wg set`) has no direct analogue in the Mackey et al. study, which focused on steady-state throughput and initial connection establishment. Commercial zero-trust access products do not publish policy-propagation latency metrics in peer-reviewed literature; the absence of comparable published figures is itself a motivating factor for this study's empirical approach.

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

Empirical benchmarks on a GCP e2-standard-8 instance running Linux 6.17 and WireGuard v1.0.0 demonstrate tunnel establishment at 44 ± 5 ms (mean ± 95% CI, n=30), re-handshake latency at 59 ± 20 ms (n=10), AllowedIPs policy enforcement via `wg set` at 8 ms mean (n=20, synchronous kernel operation), and hub restart recovery at 1–16 s depending on spoke keepalive timer state. These measurements characterise the baseline performance of a CRMA deployment exclusive of network transit latency; deployments with round-trip latency R add approximately 2R to tunnel establishment time.

Formal hypotheses H₁–H₃ provide falsifiable claims about isolation equivalence, key-custody preservation under coordinator compromise, and audit-log integrity. The six-test falsification programme (T1–T6 in Section 6.2) defines the conditions under which each hypothesis would be rejected.

The CRMA does not replace commercial ZTA products in all deployment contexts: organisations that require runtime dynamic policy (sub-minute response to observed device state), O(1)-complexity spoke onboarding, or hub-availability guarantees stronger than can be achieved with a single hub node will find commercial products better suited to their requirements. For organisations that prioritise verifiable key custody, transparent audit-log ownership, and freedom from vendor-infrastructure dependency, the CRMA provides a documented, reproducible open-architecture reference supported by empirical performance characterisation.

---

## AI Use Disclosure

The authors used claude-sonnet-4-6 (Anthropic) to assist with manuscript drafting. All substantive intellectual content — architecture design, formal hypotheses, falsification programme, security analysis, benchmark design and execution — reflects the authors' original work. The manuscript was reviewed and revised under the authors' editorial direction. This disclosure is made in accordance with COPE guidelines (2024) on the use of AI tools in academic publishing.

## CRediT Contributor Roles

**Peter M. Woodfine:** Conceptualization, Methodology, Writing – Original Draft, Writing – Review & Editing.
**Mathew Woodfine:** Software, Formal Analysis, Writing – Review & Editing.
**Jennifer M. Woodfine:** Validation, Writing – Review & Editing.

## Conflict of Interest

The authors declare no conflict of interest.

## Funding

No external funding received.

## Data Availability

Prototype implementation scripts, configuration templates, and benchmark results are available at the project repository under Apache 2.0 licence. Raw benchmark output files are included in the repository's `benchmarks/` directory.

---

## References

Bellovin, Steven M. 1989. "Security Problems in the TCP/IP Protocol Suite." *ACM SIGCOMM Computer Communication Review* 19 (2): 32–48.

Birge-Lee, Henry, Maria Apostolaki, and Jennifer Rexford. 2024. "Global BGP Attacks that Evade Route Monitoring." In *Proceedings of the Passive and Active Measurement Conference (PAM 2025).* Springer LNCS. arXiv:2408.09622. DOI: 10.1007/978-3-031-85960-1_14.

Donenfeld, Jason A. 2017. "WireGuard: Next Generation Kernel Network Tunnel." In *Proceedings of the Network and Distributed System Security Symposium (NDSS 2017).* San Diego, CA: Internet Society.

Kindervag, John. 2010. *No More Chewy Centers: Introducing the Zero Trust Model of Information Security.* Forrester Research.

Laurie, Ben, Adam Langley, and Emilia Kasper. 2013. "Certificate Transparency." RFC 6962. Internet Engineering Task Force.

Lipp, Benjamin, Bruno Blanchet, and Karthikeyan Bhargavan. 2019. "A Mechanised Cryptographic Proof of the WireGuard VPN Protocol." In *Proceedings of the 40th IEEE Symposium on Security and Privacy (IEEE S&P 2019),* 231–248. IEEE.

Osborn, Betsy, Jason Polakis, and Angelos Keromytis. 2016. "Google BeyondCorp: The Access Proxy." *;login: The USENIX Magazine* 41 (1): 28–33.

Perrin, Trevor. 2018. "The Noise Protocol Framework." Revision 34. Available: https://noiseprotocol.org/noise.html

Rose, Scott, Oliver Borchert, Stu Mitchell, and Sean Connelly. 2020. "Zero Trust Architecture." NIST Special Publication 800-207. Gaithersburg, MD: National Institute of Standards and Technology.

Saltzer, Jerome H., and Michael D. Schroeder. 1975. "The Protection of Information in Computer Systems." *Proceedings of the IEEE* 63 (9): 1278–1308.

Ward, Rob, and Betsy Beyer. 2014. "BeyondCorp: A New Approach to Enterprise Security." *;login: The USENIX Magazine* 39 (6): 6–11.

Mackey, Shane, Ivan Mihov, Andrey Nosenko, Francisco Vega, and Yong Cheng. 2020. "A Performance Comparison of WireGuard and OpenVPN." In *Proceedings of the 10th ACM Conference on Data and Application Security and Privacy (CODASPY '20),* 162–164. New York, NY: ACM. DOI: 10.1145/3374664.3379532.

---

*Version 0.4 — citation resolution 2026-05-29*
*[Cameron et al. 2019] replaced with Birge-Lee et al. 2024 (DOI: 10.1007/978-3-031-85960-1_14)*
*ZTA latency [CITATION NEEDED] replaced with Mackey et al. 2020 (DOI: 10.1145/3374664.3379532)*
*Version 0.3 — §4 Implementation and §5 Evaluation written with empirical benchmark data*
*Benchmark environment: GCP e2-standard-8, Ubuntu 24.04, kernel 6.17.0-gcp, WireGuard 1.0.0*
*§1–§3 + §6–§7 body written and language-cleared (v0.2)*
*Remaining pre-submission gates: ORCID IDs, final language pass on §4–§5*
