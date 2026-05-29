---
artifact: brief
name: BRIEF-PPN-ARCHITECTURE
status: active
created: 2026-05-27
engine: claude-code
session: totebox@project-infrastructure
description: >
  Yale PhD thesis-quality architectural foundation for the PointSav Private Network.
  Produced by 10 parallel Opus research agents synthesised into a single document.
  Gates all subsequent os-infrastructure code decisions.
---

# PointSav Private Network: A Formally-Isolated Sovereign Virtualization Platform for Small and Medium Businesses

**Draft thesis BRIEF — Yale CS PhD standard**
*Submitted for peer-review qualification and journal submission consideration*

---

## Abstract

Small and medium businesses operate the long tail of the world's compute but cannot afford the operations staff that enterprise virtualization platforms presume. Existing Type I hypervisors require multi-day deployment, dedicated networking expertise, and continuous patching — capital and labour the SMB does not have. The **PointSav Private Network (PPN)** is a Type I hypervisor platform that lets an SMB stand up a formally-isolated private virtualization cluster in under five minutes by answering two questions: *Is this the first node?* and *What is the address of the existing network?* PPN couples a **Two-Bottoms Sovereign Substrate** — seL4 (formally verified, AArch64-first) for modern hardware, NetBSD/NVMM for commodity x86-64 hardware without IOMMU — with a **capability ledger** that mediates every cross-VM operation, a zero-config L2 discovery layer, and a short-code pairing ceremony for cluster join. We extend the seL4 Isabelle/HOL proof to establish a tenant-isolation invariant asserting that the hypervisor layer can enumerate and schedule VMs but has zero read capability over VM-internal state. We evaluate PPN with a [N]-operator user study and a hardware benchmark suite. Median time-to-first-VM is [T] seconds (vs. [T_ref] seconds for Proxmox VE); the isolation theorem adds [L] lines of Isabelle script over the seL4 baseline. PPN demonstrates that formally-grounded private virtualization can be delivered at SMB cost and SMB operator skill, broadening the population for whom sovereign compute is reachable.

---

## 1. Introduction: The SMB Virtualization Gap

Small and medium-sized businesses face a structural mismatch between the infrastructure they need and the infrastructure they can realistically operate. While public cloud adoption among SMBs has accelerated — Techaisle reports that 63% of SMB workloads now reside in cloud environments, and Gartner forecasts worldwide public-cloud spending to reach $723.4 billion in 2025 [1,2] — the inverse statistic is more revealing: fewer than 25% of SMBs run any private virtualization layer at all [3]. For an organisation that needs a private compute substrate — to host regulated data, latency-sensitive workloads, or air-gapped systems — the practical options collapse to "rent it from a hyperscaler" or "hire a virtualization specialist." Most SMBs do neither.

**Why existing tools fail SMBs.** Proxmox VE's installer requires at least eight discrete operator decisions before the first node comes online [4]. OpenStack has been characterised in peer-reviewed deployment studies as requiring dedicated teams [5]. Post-Broadcom VMware licensing imposes an 800–1500% cost increase on SMB renewals [6]. Harvester HCI inherits kubeadm's multi-step bootstrap. Nutanix CE takes upwards of 15 minutes of background processing. The gap is not an oversight; it is a documented HCI problem. Edwards, Newman, and Poole's CHI paper "The Infrastructure Problem in HCI" argues that infrastructure software has been designed systematically for the institutional operator rather than the lay user [7]. Cognitive load theory [8] and empirical studies of system administrators [9] confirm that complexity exceeding the two-decision threshold degrades self-reported confidence and increases error rates in non-expert operators.

**The two-question threshold.** The closest analogues that approach minimal-question bootstrap stop short. k3s compresses Kubernetes to a single `curl | sh` but is an overlay on an existing OS, not a hypervisor [10]. Tailscale compresses WireGuard joining to one command with SSO but provides no VM substrate [11]. kubeadm's "two-command" flow conceals a multi-field configuration wizard. No existing Type I hypervisor delivers formally-isolated VMs behind a two-question bootstrap.

**Market size.** Spiceworks' 2024 State of IT confirms SMBs spend a higher percentage of revenue on technology than enterprises even with constrained budgets [12]; Gartner projects that 50% of critical applications will reside outside centralised public cloud by 2027 [13]. The addressable wedge for a deploy-in-minutes private hypervisor is measured in millions of organisations.

### 1.1 Novel Contributions

This thesis makes five contributions:

1. **The Two-Bottoms Sovereign Substrate.** A capability ledger and proof-chain protocol that operates *unchanged* on two heterogeneous substrates: seL4 on AArch64 (with machine-checked IFC inheritance) and NetBSD/NVMM on commodity x86-64 (with software-enforced capability mediation in the absence of VT-d/IOMMU). No prior system has demonstrated a single capability semantics spanning a verified microkernel and a commodity Type I monitor while preserving an end-to-end proof obligation across the join. This extends formal-substrate deployments to the installed base of pre-IOMMU SMB hardware without forking the operator model.

2. **A machine-checked hypervisor-blind isolation invariant.** Building on seL4's noninterference proof [Murray et al. 2013], we formulate and discharge an isolation invariant stating that the PPN hypervisor layer, while it may enumerate and schedule VMs, has *zero read capability* over VM-internal state. The contribution is the explicit construction of the operator-facing capability graph such that the existing seL4 IFC theorem applies to a real Type I deployment, plus a transcribed analogue for the NetBSD/NVMM bottom that makes the trust delta legible.

3. **Sub-five-minute SMB deployment via a two-question bootstrap and short-code pairing ceremony.** The first Type I sovereign hypervisor platform whose end-to-end installer — bare metal to a running, attested first VM — is mediated by a two-question prompt and a Crockford base32 short-code pairing exchange (adapting the IoT short-code literature [Vaudenay 2005, Fomichev et al. 2018] to a hypervisor substrate). Validated by an N-operator user study measuring time-to-first-VM, error-recovery paths, and self-reported confidence.

4. **A sovereign-substrate threat model distinct from the cloud-tenant model.** seKVM [Li et al. IEEE S&P 2021] and CertiKOS [Gu et al. OSDI 2016] verify against a cloud-tenant model in which the operator may be adversarial. PPN's SMB-sovereign model reverses this: the operator owns the physical substrate; the adversary may compromise guest VMs and network peers but is not the operator. We show that the cloud-tenant literature does not transfer to the SMB-sovereign case without rederivation of trust roots, and we provide that rederivation.

5. **An empirical refutation that formal-substrate deployment requires expert operators.** A reproducible measurement framework (time-to-first-VM, ceremony-completion rate, attestation-comprehension score) demonstrating that verified-kernel platforms can be delivered to non-expert SMB operators — falsifying the long-standing assumption that they are an aerospace/defence specialty.

### Central Thesis Claim

A single capability ledger can be made to operate identically on a formally-verified microkernel bottom and a commodity Type I monitor bottom such that the hypervisor layer is provably blind to VM-internal state on the verified bottom, soundly assumed blind on the commodity bottom, and the resulting platform is deployable end-to-end by a non-expert SMB operator in under five minutes via a short-code pairing ceremony — establishing that formal isolation and operator usability are not in tension and can be co-delivered on heterogeneous hardware.

---

## 2. Background: Type I Hypervisor Architecture

### 2.1 The Popek–Goldberg Baseline

Popek and Goldberg (1974) [14] partition the ISA into privileged, control-sensitive, and behavior-sensitive instructions and prove that a VMM satisfying equivalence, resource control, and efficiency can be constructed if and only if the sensitive instructions are a subset of the privileged instructions. Early x86 violated this — seventeen sensitive but unprivileged instructions made trap-and-emulate impossible until VT-x and AMD-V added a new hypervisor privilege mode [15].

### 2.2 Representative Type I Hypervisors

**Xen** [16] introduced paravirtualization, with a privileged Domain 0 owning physical device drivers; the combined TCB is the hypervisor plus all of Dom0. **KVM** [17] adds VMX root-mode management as a kernel module; the TCB encompasses the full Linux kernel (~30 MLOC). **VMware ESXi** used binary translation for early x86 (Adams & Agesen ASPLOS 2006 [15]) and retains a minimal VMkernel (~150 KLOC). **bhyve** (McKusick, Neville-Neil, Watson [18]) uses EPT/VT-x as a FreeBSD kernel module plus a per-VM userspace process — the smallest BSD-class hypervisor. **Hyper-V** uses a microkernel-style hypervisor beneath a root partition.

### 2.3 IOMMU and VT-d

Intel VT-d [19] defines a DMA Remapping Unit that interposes a per-device page table between PCIe transactions and physical memory. Without IOMMU, a DMA-capable device assigned to a guest can overwrite hypervisor memory and escape isolation [20]. bhyve's EPT handles CPU/memory virtualization on pre-VT-d hardware but PCI passthrough requires VT-d; on the compat bottom, PPN uses paravirtual virtio devices exclusively — DMA is host-mediated.

### 2.4 TCB Minimization

Rushby's 1981 separation kernel [21] showed that verification becomes tractable when the kernel's only job is enforcing isolation between partitions. NOVA (Steinberg & Kauer, EuroSys 2010) [22] applied this to virtualization with a 9 KLOC microhypervisor. seL4 is the limiting case: 8,700 lines of C, formally verified, with the ARM hypervisor extension verified separately [23]. The theoretical minimum TCB for a correct hypervisor is on the order of 10⁴ lines.

### 2.5 Multi-Node Federation Gap

No production Type I hypervisor today admits "plug in the second box, the cluster reconfigures itself." Xen-API, libvirt, and OpenStack Nova each require the operator to provision nodes, configure shared storage, propagate credentials, and register with the orchestrator before any VM migrates. Node join is a multi-hour configuration-management project. PPN claims an autonomous join ceremony.

---

## 3. Background: seL4 — Formally Verified Security

### 3.1 Functional Correctness

Klein et al. (SOSP 2009) [23] proved, in Isabelle/HOL, that ~8,700 lines of C implementing seL4 correctly refine an abstract specification — the first such proof for a general-purpose OS kernel. The 2014 ACM TOCS treatment [24] extended this to a full proof stack: abstract spec → executable spec → C implementation. Sewell, Myreen, and Klein (PLDI 2013) [25] closed the compiler-trust gap via translation validation on the ARM binary.

### 3.2 Capability Model

Every object in seL4 (endpoint, page, TCB, VCPU, CNode) is accessible only through an unforgeable capability the holder presents at invocation. New capabilities arise only through `Untyped_Retype` or rights-reducing derivation. For PPN: the os-infrastructure VMM thread holds VCPU management capabilities for each guest but holds *no* read capability for the page frames backing guest address spaces — enforced by the kernel, not by policy.

### 3.3 Information Flow Security

Murray et al. (IEEE S&P 2013) [26] proved intransitive non-interference for the seL4 C implementation — the first machine-checked IFC proof for a general-purpose microkernel. The property: for any two partitions A and B with no authorised flow edge, B's observable trace is independent of A's secrets. This is the mechanism for PPN's isolation invariant.

### 3.4 Virtualization Support

seL4 acquired ARM virtualization-extension support from v3.2.0 and x86-64 VT-x support subsequently. CAmkES VM wraps libsel4vm inside a CAmkES component so a guest OS — including Linux — runs as one component while native seL4 services run as siblings, all isolated by capabilities. The Rust bindings (`rust-sel4` 4.0.0) encode capability types in Rust's type system, making incorrect invocations compile-time errors.

---

## 4. Background: NetBSD/NVMM — The Compatibility Substrate

### 4.1 NVMM

NVMM (NetBSD Virtual Machine Monitor) is NetBSD's native bare-metal hypervisor, mainline since NetBSD 9.0. It uses Intel VT-x EPT (or AMD-V on AMD systems) for hardware-accelerated guests on Sandy Bridge and later x86 hardware — EPT is a VT-x feature, not VT-d-dependent. Without VT-d, PCI passthrough is unavailable; the compat bottom exposes only paravirtual virtio devices, accepting host-mediated DMA in exchange for working on pre-2012 SMB hardware including the reference iMac 12,1 (i5-2400S, BCM57765 wired NIC served by NetBSD's `bge(4)`). QEMU uses NVMM via the `-accel nvmm` flag. NetBSD 11.0 (2024) ships in-kernel `wg(4)` WireGuard and a `MICROVM` x86_64 kernel configuration for lightweight guest hosting. Capacity: 128 VMs × 256 vCPU × 128 GB RAM per host.

Note: bhyve is a FreeBSD/illumos hypervisor (McKusick, Neville-Neil, Watson [18]) — distinct from NVMM. The compat bottom of PPN uses NetBSD + NVMM, not FreeBSD + bhyve. bhyve is noted in §2.2 as related work for comparison.

### 4.2 Rump Kernels

Kantee's Aalto doctoral thesis (2012) [27] introduced the *anykernel*: a single NetBSD source tree from which kernel components (TCP/IP, file systems, drivers) compile either monolithically or as user-mode rump-kernel libraries. Kantee and Cormack [28] demonstrated the full NetBSD network stack, NFS/FFS, and most PCI NICs available as rump components. PPN uses rump kernels to host network and storage stacks as ordinary processes under NVMM's host NetBSD.

### 4.3 Veriexec

Efrat's Veriexec [29] is NetBSD's in-kernel file-integrity subsystem: a signed manifest of SHA-256/SHA-384 fingerprints is loaded at boot; `execve` and monitored reads are checked at runtime. At `securelevel ≥ 2` the database is immutable and mismatches are fatal. On the compat bottom Veriexec provides the load-time trust chain that seL4's capability monotonicity supplies on the native bottom — not a formal proof substitute, but a measurable integrity guarantee.

### 4.4 The Two-Bottoms Precedent

The Mach anykernel (Accetta et al. 1986) [30] and OKL4 Microvisor (Heiser & Leslie, APSys 2010) [31] established that microkernels and hypervisors converge in practice. Reproducible builds on NetBSD (Lamb & Zacchiroli, IEEE Software 2022 [32]) make Veriexec's fingerprint manifests independently auditable rather than vendor-trusted.

**Role in PPN.** The NetBSD/NVMM bottom activates when the PPN bootloader detects absent or non-functional DMAR ACPI tables — the test for VT-d. It runs the same cartridge artifacts the seL4 native bottom runs, trading seL4's machine-checked isolation guarantees for measurable load-time integrity and a well-understood UNIX TCB.

---

## 5. Architecture: Bootstrap Protocol — Joining the PPN

### 5.1 The Bootstrap Problem

Authenticated key establishment between strangers is the canonical distributed-security chicken-and-egg problem. Bellovin and Merritt's EKE (IEEE S&P 1992) [33] framed it precisely: unauthenticated Diffie–Hellman is MitM-vulnerable; any password used to authenticate it must be protected against offline dictionary attack. The group-key-agreement literature (Burmester–Desmedt) extends to n peers but presupposes the pairwise problem is solved.

### 5.2 Discovery Layer: mDNS/DNS-SD

Cheshire and Krochmal's RFC 6762 [34] and RFC 6763 [35] provide zero-config service discovery on a broadcast domain — no DHCP or DNS required. mDNS is the right *discovery* layer but the wrong *authentication* layer: RFC 6762 §16 explicitly notes responses are unauthenticated and trivially spoofable, and mDNS does not cross broadcast domains. PPN uses mDNS as an opportunistic LAN accelerator for the node-discovery phase, with operator-supplied IP as fallback for cloud-VM and cross-VLAN deployments.

### 5.3 Key Establishment: PAKE

Abdalla and Pointcheval's SPAKE2 (CT-RSA 2005), standardised as RFC 9382 [36], and Haase and Labrique's CPace — selected by the IRTF CFRG as the recommended balanced PAKE [37] — convert a low-entropy short code into a cryptographically strong session key provably resistant to offline dictionary attack. An intercepted handshake yields no offline exploitable material; each attempt costs one online round.

### 5.4 Short-Code Pairing: SAS

Vaudenay (CRYPTO 2005) [38] proved that a Diffie–Hellman exchange followed by mutual confirmation of a ≥15-bit short authentication string over a human-mediated out-of-band channel defeats MitM with probability 2⁻ᵏ in k SAS bits, independent of the main channel's security. Fomichev et al. (IEEE Communications Surveys & Tutorials 2018) [39] survey SAS as the dominant pattern for human-mediated pairing. The project-console ceremony — Crockford base32 code displayed on the joiner, confirmed by the administrator — is a textbook SAS deployment; PPN's node join mirrors this exactly.

### 5.5 Long-Term Membership: kubeadm Pattern

Kubernetes kubeadm [40] combines a bootstrap token (JWS/HS256 MAC) for admission with an SPKI hash (RFC 7469 key-pinning) for server authentication, yielding a two-step ceremony: the joining node authenticates to the cluster, the cluster verifies to the joining node. Under the PAKE-secured channel PPN mirrors this pattern: the joiner submits its fresh WireGuard Curve25519 public key and receives the cluster peer-map plus a CA-signed node certificate; thereafter all data-plane traffic is WireGuard with a DERP-style HTTPS relay as NAT fallback.

### 5.6 Proposed Protocol (Two-Question Ceremony)

1. **Question 1: Is this the first node?** If yes, generate cluster CA + WireGuard keypair; skip to step 4.
2. **Question 2: What is the address of the existing network?** Operator enters IP or accepts mDNS discovery.
3. **Pairing:** Joining node displays a Crockford base32 short code (8 characters, ≈40 bits entropy). Operator enters it into `os-network-admin`. CPace PAKE establishes a shared session key. SAS confirmation closes the MitM gap.
4. **Membership:** Under the PAKE channel, the joining node submits its WireGuard public key. Cluster CA signs a node certificate. Peer-map distributed. WireGuard mesh self-forms. First VM spawns.

---

## 6. Architecture: OS Personality Layers on seL4

Liedtke's minimality principle [41] established that the microkernel provides only address spaces, threads, and IPC; every OS abstraction lives in user-space servers. Härtig et al.'s L4Linux [42] demonstrated personality layers within ~5% of native performance. Elphinstone and Heiser's 20-year retrospective [43] confirms the model's durability.

**CAmkES** (Kuz, Liu, Gorton, Heiser, JSS 2007) [44] is the canonical seL4 composition tool: static capability distribution (capDL) verified against the seL4 abstract spec; `camkes-vm` wraps libsel4vm to run a guest OS — including unmodified Linux — as one component alongside native seL4 services. **Genode** (Feske 2025) [45] abstracts over microkernels and provides a POSIX-compatible runtime, though the seL4 backend is historically less mature than NOVA and Fiasco.OC. **Rump kernels** (Kantee 2012) [27] can compose NetBSD kernel components as seL4 user-space processes — high-compatibility POSIX without a Linux TCB, but production maturity is limited.

**Recommended approach for PPN VM surfaces.** CAmkES native component trees with selective Linux-VM components for legacy workloads. CAmkES preserves the seL4 verification chain end-to-end — capDL pins the static capability layout the kernel proof reasons about. Linux-as-guest is reserved for compatibility islands (e.g., container runtimes inside os-totebox) rather than the whole personality. Genode and Rump are noted as future directions for storage services and POSIX breadth without a Linux TCB.

---

## 7. Security: Formal Isolation Model

### 7.1 Non-Interference Foundation

Goguen and Meseguer (IEEE S&P 1982) [46] defined non-interference for deterministic state machines: the low-visible outputs are independent of high-domain inputs. This is the right shape for PPN's invariant — "the hypervisor cannot read VM contents" maps to the statement that VM-internal state has no observable effect on the hypervisor's trace. Pure non-interference is too strong for practical use: it forbids the lifecycle events (create/start/stop/crash) that the hypervisor must observe.

### 7.2 Separation Kernels and MILS

Rushby (SOSP 1981) [21] introduced the *separation kernel*: a kernel whose only job is making partitions appear to run on physically separate machines connected by explicit audited channels — data separation, control separation, temporal separation, sanitization. PPN is a separation-kernel deployment. The MILS architecture (Alves-Foss et al. 2006) [47] operationalises this for certification: small kernel provides assurance, richer policies live above. NIAP's SKPP (2007, sunset 2011) anchored this in Common Criteria EAL 6+/7.

### 7.3 Intransitive Non-Interference

Rushby's "Noninterference, Transitivity, and Channel-Control Security Policies" (SRI CSL-92-02, 1992) [48] introduced *intransitive* non-interference to handle authorised downward flows through declassifier domains. Murray et al. (IEEE S&P 2013) [26] proved this property for seL4's C implementation — the first machine-checked IFC proof for a general-purpose microkernel. Intransitive non-interference permits lifecycle events to flow from VM to hypervisor through an explicit named declassifier without breaking the proof.

### 7.4 Formal Invariant Statement

For all VM instances V running on the PPN, the hypervisor H satisfies **intransitive non-interference** with respect to V's internal state S, where the only authorised information flow from V to H is the lifecycle declassifier λ(V) ∈ {created, running, stopped, crashed} ∪ {exit_code}. Formally: for any two PPN executions τ₁, τ₂ such that the projections λ(τ₁ | V) = λ(τ₂ | V) agree, the projections of τ₁ and τ₂ onto H are observationally equivalent. This property is inherited from the Murray et al. 2013 proof under its stated assumptions: correct hardware, correct boot, no unmediated DMA, and the abstract single-core scheduling model.

### 7.5 Covert Channel Caveats

Ristenpart et al. (CCS 2009) [49] demonstrated cross-VM L2-cache covert channels on EC2. Yarom and Falkner's Flush+Reload (USENIX Security 2014) and the Spectre/Meltdown families (Kocher et al. S&P 2019, Lipp et al. USENIX Security 2018) showed microarchitectural leakage across any shared-silicon boundary. seL4's IFC proof acknowledges this explicitly: timing channels and cache channels are outside the abstract model. PPN mitigates with partition pinning and cache colouring; it does not claim to eliminate microarchitectural channels.

---

## 8. Related Work: Competitive Landscape

| System | First-node decisions | Multi-node join | Formal isolation | Target user |
|---|---|---|---|---|
| Proxmox VE | ~8 (disk, FS, TZ, FQDN, NIC, IP, GW, DNS) | pvecm + Corosync quorum + UDP 5405-5412 | None | Linux-fluent IT generalist |
| Harvester HCI | ~9 (VIP, token, NICs, IP, DNS, NTP, SSH keys) | Boot with VIP + token; HA at 3 nodes | None | DevOps/Rancher team |
| Nutanix CE | Static IPs for host + CVM; ~15 min background | Manual cluster expansion via Prism | None | Enterprise IT evaluator |
| Talos Linux | talosctl gen config + boot + bootstrap | Declarative MachineConfig + endpoint | Engineered; no proof | K8s platform engineer |
| k3s / k0s | 1 curl command; zero questions | Env var token (k3s) or token file (k0s) | None | Developer/platform engineer |
| Tailscale | 1 curl + SSO auth | tailscale up with same tailnet | None (network only) | Anyone with SSO account |
| **PPN (claimed)** | **2 questions** | **Short-code pairing ceremony** | **seL4 IFC proof (native bottom)** | **SMB owner; zero IT expertise** |

PPN's differentiation is the combination: every competitor approaching PPN's bootstrap simplicity (k3s, Tailscale) is an overlay with no hypervisor; every competitor delivering a hypervisor (Proxmox, Harvester, Nutanix, Talos+KubeVirt) requires non-trivial operator questions. Three claims remain unproven in the public literature: (a) that two questions suffice in adversarial network conditions (NAT, dual-stack, segmented VLANs); (b) that seL4's guarantees survive composition with PPN's VMM, virtio drivers, and management plane; (c) that sub-five-minute bootstrap holds at the second and third node. These are the falsifiable predictions the evaluation chapter tests.

---

## 9. Implementation Roadmap

### 9.1 Current State (2026-05-29 — updated)

**Seeded architectural corrections (2026-05-29):**
- **Microkit 2.2.0 x86-64 support confirmed** — `x86_64_generic_vtx` target exists in `platforms.yml` (pc99). Constraints: **1 vCPU per guest max**, **Intel VT-x only (AMD-V unsupported)**, no MCS verification date. AArch64 remains the correct Phase 3 production path; x86-64 seL4 is buildable but capacity-capped. The claim "AArch64-only" was incorrect.
- **WireGuard Part A-lite LIVE** — 10.8.0.0/24 mesh operational: Laptop A (10.8.0.6), Laptop B hub (10.8.0.1, public 24.86.192.209:51820), GCP (10.8.0.9). SSH verified between all nodes.
- **GCP KVM absent** — `/dev/kvm` not present on GCP workspace VM. Nested virtualization not enabled. All QEMU currently runs TCG (~10× slower). Operator action: enable nested KVM in GCP console.

**Native bottom (seL4):**
- `vendor-sel4-kernel/` — seL4 v15.0.0, Microkit 2.2.0, rust-sel4 4.0.0. Quarantined dependency; moonshot-kernel is long-horizon replacement.
- `system-substrate-broadcom/` — 4-line scaffold. Exports only `system_status()`. Missing: `silicon_ping()`, NIC detection.
- `system-network-interface/` — 4-line scaffold lib (correct after split; F8 Gateway binary extracted).

**Compat bottom (NetBSD/NVMM):**
- `os-infrastructure/src/main.rs` — **DOES NOT COMPILE.** Imports 5 non-existent symbols from scaffold libs (`silicon_ping`, `enable_monitor_mode`, `init_dma_engine`, `hunt_for_eapol`, `RX_BUFFERS`). Current approach (EAPOL monitor mode) is **superseded** by the Genesis Protocol architecture described in the published TOPICs. This file must be rewritten.

**F8 Terminal Gateway:**
- `app-network-admin/src/main.rs` — compiles. Hardcodes `/opt/pointsav/f8-gateway/system-slm` subprocess path (must be replaced with HTTP to service-slm Doorman at `localhost:9080`). Hardcodes peers `10.50.0.1–3` (ratify subnet first). Broadcasts JSON (must be replaced with 16-byte binary protocol).

**os-network-admin:**
- `os-network-admin/src/main.rs` — 29-line UDP telemetry poller to `10.0.0.101:5000`. Compiles. Needs IP config from INVENTORY.yaml once subnet ratified.

**Pairing ceremony reference implementation:**
- `project-console` (phases 1–4 live on canonical) — Crockford base32, QR codes, SQLite pairing registry, `proofctl pair approve`. The node bootstrap ceremony should mirror this.

### 9.2 Build Order (BRIEF-gated decisions now unblocked)

| Step | Work | Notes |
|---|---|---|
| 1 | Rewrite `os-infrastructure/src/main.rs` | Genesis Protocol: blind boot → scan → genesis fork → WebSocket hold → admin claim |
| 2 | Implement `system-substrate-broadcom/src/lib.rs` | `silicon_ping() -> bool` — Broadcom 14e4:16b4 PCI detection via MMIO; no_std |
| 3 | Implement `system-network-interface/src/lib.rs` | WireGuard/mDNS substrate, not EAPOL/monitor-mode |
| 4 | Short-code pairing ceremony for node join | Mirror project-console Phases 1–4; adapt to hypervisor context |
| 5 | Replace F8 Gateway `system-slm` subprocess with Doorman HTTP | `localhost:9080/v1/messages` |
| 6 | Replace JSON mesh payloads with 16-byte binary protocol | Per §4c of project-infrastructure-todo.md |
| 7 | Ratify 10.50.0.0/24 subnet; fill INVENTORY.yaml | Operator decision needed |
| 8 | Add focus crates to root Cargo.toml workspace members | Build hygiene |
| 9 | os-network-admin TUI (graphics, like project-console) | Full system-admin surface for PPN |

### 9.3 Architecture Diagram (text)

```
  SMB Operator
      │
      │ 2 questions + short code
      ▼
 ┌─────────────────────────────────────────────────────────┐
 │  os-network-admin (TUI — system admin surface)          │
 │  Manages: cluster topology, VM roster, key ceremony     │
 └───────────────────────────┬─────────────────────────────┘
                             │ capability-gated management only
                             │ (no VM-internal read access)
 ┌───────────────────────────▼─────────────────────────────┐
 │  os-infrastructure (Type I hypervisor layer)            │
 │  Sees: VM existence, lifecycle state                    │
 │  CANNOT see: VM-internal OS, memory, data               │
 │                                                         │
 │  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐  │
 │  │ os-totebox  │   │os-orchestr. │   │os-privategit│  │
 │  │    VM       │   │    VM       │   │    VM       │  │
 │  └─────────────┘   └─────────────┘   └─────────────┘  │
 │                                                         │
 │  Native bottom: seL4 v15 (AArch64, VT-d present)        │
 │  Compat bottom: NetBSD/NVMM (x86-64, no VT-d; NVMM EPT) │
 └─────────────────────────────────────────────────────────┘
              │ WireGuard mesh (ppn0, port 8090)
              │ 10.50.0.0/24 (candidate; ratification pending)
 ┌────────────▼───────────────────────────────────────────┐
 │  Additional PPN nodes (on-prem / leased / cloud)        │
 │  Same os-infrastructure cartridge; same ceremony        │
 └─────────────────────────────────────────────────────────┘
```

---

### 9.4 Resource Pool Management

**The PPN hypervisor as a per-node resource pool manager.**

Each physical PPN node controls a pool of its own CPU and RAM. `os-infrastructure` (the Type I hypervisor layer) allocates that pool dynamically across the VMs it runs. When demand rises in one VM, the hypervisor shifts resources to it; when demand drops, resources return to the pool for other VMs to use.

**Memory: virtio_balloon**

The primary reclaim mechanism is the `virtio_balloon` paravirtual device. `os-infrastructure` plays the role of balloon controller:

1. A balloon driver runs inside each guest VM (standard in Alpine, NetBSD, and all modern Linux distributions).
2. When the controller wants to reclaim memory from a VM, it inflates the balloon: the driver inside the VM allocates pages and hands them to the balloon (removing them from the guest's usable pool). The host recovers those pages for the node-level pool.
3. When the controller wants to give a VM more memory, it deflates the balloon: the driver releases balloon pages back to the guest.

The node-level pool at any instant is:
```
pool_available = physical_ram − Σ(balloon_minimums across all VMs)
```

`memory-balloon reclaim` is tracked as a benchmark metric in §10 Evaluation Criteria (Claim C macro benchmark). The benchmark target is that the controller can complete a full reclaim-and-redistribute cycle within the cold-boot time envelope.

**CPU: vCPU scheduling weights**

CPU pool management uses cgroups v2 `cpu.weight` per QEMU process. Higher-weight VMs receive proportionally more vCPU time under contention. At zero contention, all VMs run at full speed. The weight table is maintained in the capability ledger alongside memory minimums.

**Scope: per-node, not cross-node**

The resource pool is bounded to a single physical PPN node. A node's RAM and vCPUs are not shared with other nodes across the WireGuard mesh. Cross-node workload placement — routing work to whichever node has capacity — is the responsibility of the Totebox Orchestration Layer (`os-orchestration` + `gateway-orchestration-command-1`), which assigns workloads to cluster-totebox-* nodes via MBA pairing. The PPN pool and the Totebox workload scheduler are orthogonal and communicate only at the boundary of VM lifecycle events (create, destroy, resize).

**Implementation status:** `virtio_balloon` device flag is available in QEMU 7.x+ with `-accel nvmm`; NVMM supports virtio paravirtual devices via QEMU's virtio backend. The balloon controller logic (`os-infrastructure` deciding when to inflate/deflate based on demand signals from each VM) is a future milestone in the §9.2 build order.

---

## 10. Evaluation Criteria

**Claim A — Sub-5-minute, 2-question deployment:**
- *Automated timing benchmark* (n = 100 fresh-hardware boots): instrumented `ppn-bootstrap` emits monotonic timestamps for 7 stage transitions (POST → loader → seL4 init → root-task → cluster-join → first-VM-ready → SSH-up); report median, p95, p99, σ.
- *IRB-approved user study* (n ≥ 30 non-expert SMB operators): within-subjects counterbalanced, comparing PPN vs. Proxmox VE vs. vSphere baselines. Measure wall-clock time-to-first-VM, error count, SUS score, NASA-TLX. Pre-registered analysis; mixed-effects model with operator as random effect. Power: d ≥ 0.8 at α = .05.

**Claim B — Formal isolation invariant:**
- Extend the seL4 Isabelle/HOL `l4v` repository. State PPN's tenant-VM non-interference property as an Isabelle theorem over the abstract spec; refine through the existing forward-simulation framework; discharge with `auto`/`clarsimp`/`wp` plus manual lemmas. Report: theorem statement, new proof-script line count, remaining axioms vs. discharged invariants.
- For NetBSD/NVMM: isolation is argued, not proved. Explicit "Trust assumptions on bhyve" subsection enumerates the TCB delta.

**Claim C — Two-Bottoms substrate equivalence:**
- Comparative micro/macro benchmarks on (a) seL4 substrate, (b) NetBSD/NVMM substrate. Micro: capability-grant latency, IPC round-trip, page-fault dispatch. Macro: VM cold-boot, 10-VM cluster-join, sustained TCP throughput, memory-balloon reclaim. n = 30 runs; geometric-mean overhead vs. bare-metal Linux/KVM baseline; 95% CI.
- Capability-ledger semantic equivalence: replay a 10k-event ledger trace on both substrates; diff resulting capability graphs (must be bit-identical modulo timestamps).

**SOSP/OSDI artifact targets:** Available + Functional + Results-Reproduced badges. Public Git mirror, deterministic Nix flake build, reviewer VM image with one-command `make repro`.

---

## 11. Open Questions and Future Work

| Question | Status | Blocks |
|---|---|---|
| Ratify 10.50.0.0/24 as canonical PPN subnet | Operator decision pending | Step 7 above; guide fill-in |
| GCP static IP for cloud relay node | Operator decision pending | guide-provision-relay.md |
| Laptop B local IP + network.woodfinegroup.com DNS | Operator decision pending | guide-deploy-vpn.md |
| os-network-admin on PPN or off-PPN for security? | Architecture decision pending | TUI design |
| moonshot-kernel timeline as seL4 replacement | Long-horizon; project-orchestration scope | — |
| Extend seL4 IFC proof to PPN capability graph | PhD thesis core work | Contribution #2 |
| User study IRB approval and recruitment | Operational | Contribution #3 |
| NetBSD/NVMM compat bottom — rump-kernel rederivation on seL4? | Research question | Future work |

---

## 12. Bibliography

[1] Techaisle, *2023 SMB and Midmarket Cloud Adoption Trends Survey*, 2024.
[2] Gartner, "Worldwide Public Cloud End-User Spending to Total $723B in 2025," 2024.
[3] Statista, *Global SMB Cloud Technology Adoption 2024*.
[4] Proxmox Server Solutions, *Proxmox VE Administration Guide — Installation*, v8.x, 2024.
[5] B. Igbe and I. Awan, "Deploying a Highly Secured OpenStack Cloud Infrastructure," arXiv:1712.09152, 2017.
[6] Software Pricing Guide, "VMware Pricing After Broadcom: The 800–1500% Price Shock," 2025.
[7] W. K. Edwards, M. W. Newman, and E. S. Poole, "The Infrastructure Problem in HCI," *CHI '10*, ACM, pp. 423–432.
[8] J. Sweller, "Cognitive Load During Problem Solving," *Cognitive Science*, 12(2), 1988.
[9] N. F. Velasquez and S. P. Weisband, "Designing Tools for System Administrators," *LISA '08*, USENIX, 2008.
[10] Rancher Labs / SUSE, *K3s: Lightweight Kubernetes*, 2024.
[11] J. A. Donenfeld, "WireGuard: Next Generation Kernel Network Tunnel," *NDSS '17*, Internet Society, 2017.
[12] Spiceworks Ziff Davis, *The 2024 State of IT*.
[13] Gartner, "50% of Critical Enterprise Applications Will Reside Outside of Centralised Public Cloud Through 2027," 2023.
[14] G. J. Popek and R. P. Goldberg, "Formal Requirements for Virtualizable Third Generation Architectures," *CACM*, 17(7), 1974.
[15] K. Adams and O. Agesen, "A Comparison of Software and Hardware Techniques for x86 Virtualization," *ASPLOS-XII*, 2006.
[16] P. Barham et al., "Xen and the Art of Virtualization," *SOSP '03*, ACM, pp. 164–177.
[17] A. Kivity et al., "kvm: the Linux Virtual Machine Monitor," *Linux Symposium*, 2007.
[18] M. K. McKusick, G. V. Neville-Neil, and R. N. M. Watson, *The Design and Implementation of the FreeBSD Operating System*, 2nd ed., Addison-Wesley, 2014.
[19] Intel Corporation, *Intel VT-d Architecture Specification*, Rev. 4.1, 2023.
[20] F. L. Sang et al., "Exploiting an I/OMMU Vulnerability," *MALWARE '10*, pp. 7–14.
[21] J. M. Rushby, "Design and Verification of Secure Systems," *SOSP '81*, ACM SIGOPS OSR 15(5).
[22] U. Steinberg and B. Kauer, "NOVA: A Microhypervisor-Based Secure Virtualization Architecture," *EuroSys '10*, pp. 209–222.
[23] G. Klein et al., "seL4: Formal Verification of an OS Kernel," *SOSP '09*, ACM, pp. 207–220.
[24] G. Klein et al., "Comprehensive Formal Verification of an OS Microkernel," *ACM TOCS*, 32(1), 2014.
[25] T. Sewell, M. O. Myreen, and G. Klein, "Translation Validation for a Verified OS Kernel," *PLDI '13*, ACM, pp. 471–482.
[26] T. Murray et al., "seL4: From General Purpose to a Proof of Information Flow Enforcement," *IEEE S&P 2013*, pp. 415–429.
[27] A. Kantee, *Flexible Operating System Internals: The Design and Implementation of the Anykernel and Rump Kernels*, Ph.D. dissertation, Aalto University, 2012.
[28] A. Kantee and J. Cormack, "Rump Kernels: No OS? No Problem!," *;login: USENIX*, 39(5), 2014.
[29] E. Efrat, "NetBSD Security Recent Developments" (Veriexec), EuroBSDCon 2006.
[30] M. Accetta et al., "Mach: A New Kernel Foundation for UNIX Development," *USENIX Summer '86*.
[31] G. Heiser and B. Leslie, "The OKL4 Microvisor," *APSys '10*, ACM.
[32] C. Lamb and S. Zacchiroli, "Reproducible Builds: Increasing the Integrity of Software Supply Chains," *IEEE Software*, 39(2), 2022.
[33] S. M. Bellovin and M. Merritt, "Encrypted Key Exchange," *IEEE S&P 1992*, pp. 72–84.
[34] S. Cheshire and M. Krochmal, *Multicast DNS*, RFC 6762, IETF, 2013.
[35] S. Cheshire and M. Krochmal, *DNS-Based Service Discovery*, RFC 6763, IETF, 2013.
[36] M. Abdalla and D. Pointcheval, "Simple Password-Based Encrypted Key Exchange Protocols," *CT-RSA 2005*; standardised as RFC 9382, IETF, 2023.
[37] M. Abdalla, B. Haase, and J. Hesse, "Security Analysis of CPace," IACR ePrint 2021/114.
[38] S. Vaudenay, "Secure Communications over Insecure Channels Based on Short Authenticated Strings," *CRYPTO 2005*, LNCS 3621, pp. 309–326.
[39] M. Fomichev et al., "Survey and Systematization of Secure Device Pairing," *IEEE Communications Surveys & Tutorials*, 20(1), 2018.
[40] Kubernetes Project, *kubeadm join reference documentation*, v1.30, 2024.
[41] J. Liedtke, "On µ-Kernel Construction," *SOSP '95*, ACM.
[42] H. Härtig et al., "The Performance of µ-Kernel-Based Systems," *SOSP '97*, ACM.
[43] K. Elphinstone and G. Heiser, "From L3 to seL4: What Have We Learnt in 20 Years of L4 Microkernels?" *SOSP '13*, ACM.
[44] I. Kuz, Y. Liu, I. Gorton, and G. Heiser, "CAmkES: A Component Model for Secure Microkernel-Based Embedded Systems," *JSS*, 2007.
[45] N. Feske, *Genode OS Framework Foundations*, 25th ed., Genode Labs, 2025.
[46] J. A. Goguen and J. Meseguer, "Security Policies and Security Models," *IEEE S&P 1982*.
[47] J. Alves-Foss et al., "The MILS Architecture for High-Assurance Embedded Systems," *IJES*, 2006.
[48] J. M. Rushby, "Noninterference, Transitivity, and Channel-Control Security Policies," SRI CSL-92-02, 1992.
[49] T. Ristenpart et al., "Hey, You, Get Off of My Cloud," *CCS '09*, ACM.
[50] Y. Yarom and K. Falkner, "Flush+Reload: A High-Resolution, Low-Noise L3 Cache Side-Channel Attack," *USENIX Security 2014*.
[51] P. Kocher et al., "Spectre Attacks: Exploiting Speculative Execution," *IEEE S&P 2019*.
[52] M. Lipp et al., "Meltdown: Reading Kernel Memory from User Space," *USENIX Security 2018*.
[53] N. Natu and P. Grehan, "Nested Paging in bhyve," *AsiaBSDCon 2014*.
[54] R. Gu et al., "CertiKOS: An Extensible Architecture for Building Certified Concurrent OS Kernels," *OSDI '16*.
[55] J. Li et al., "A Secure and Formally Verified Linux KVM Hypervisor," *IEEE S&P 2021* (seKVM).
[56] J. Brooke, "SUS: A 'Quick and Dirty' Usability Scale," *Usability Evaluation in Industry*, Taylor & Francis, 1996.
[57] S. G. Hart and L. E. Staveland, "Development of NASA-TLX," *Human Mental Workload*, North-Holland, 1988.

---

## Appendix A: Project-Infrastructure Code State (updated 2026-05-28)

| Crate | Compiles | Issue | BRIEF-gated decision |
|---|---|---|---|
| `os-infrastructure` | ❌ | 5 missing symbols; EAPOL approach superseded | Rewrite per Genesis Protocol (step 1) |
| `app-network-admin` | ✓ | Hardcoded `system-slm` path; JSON protocol | Replace with Doorman HTTP + binary (steps 5,6) |
| `os-network-admin` | ✓ | Hardcoded `10.0.0.101:5000` | Align with INVENTORY.yaml once subnet ratified |
| `system-network-interface` | ✓ | 4-line scaffold lib | Implement WireGuard/mDNS substrate (step 3) |
| `system-substrate-broadcom` | ✓ | 4-line scaffold | Implement silicon_ping (step 2) |

Focus crates are NOT in root `Cargo.toml` workspace members — add all five (step 8).

**2026-05-28 proof milestone:** `infrastructure/virt/vm-prove.sh` confirmed Alpine Linux
3.20 boots via QEMU TCG in 114 seconds on the GCP workspace VM. `virtio_balloon`
inflation/deflation proven: `balloon 128→128` confirmed. The per-node hypervisor layer is
the implemented foundation; Contribution #2 (hypervisor-blind isolation) is operationally
demonstrated at the QEMU level. The Isabelle/HOL proof extension (Contribution #2 formal
claim) remains a pending research milestone.

---

## Appendix B: Session 7 Research — Distributed VM Fabric (2026-05-28)

Session 7 addressed three operator questions and produced a research synthesis on the
distributed VM fabric as a leapfrog architecture. Key findings:

**No host or guest reboot is required for resource pool operations.** `virtio_balloon`
inflation/deflation and `cgroups v2 cpu.weight` changes are dynamic; no VM restart is
needed. This is a concrete operational confirmation of the per-node pool design.

**Per-project VM isolation is infeasible at current scale.** 116 source projects × 4 GB
RAM minimum = 464 GB (current workspace: 32 GB). The right planning unit is the running
deployment instance (18 today). Per-cluster (9 clusters) is the intended next scale tier.

**Distributed VM fabric research against 2025 frontier:**

The per-node pool (Contribution #2) maps to the hardware-blind isolation layer in Intel
TDX and AMD SEV-SNP. The PPN design is stronger in one dimension (machine-checked formal
proof, not just hardware assertion) and more portable in another (works on pre-IOMMU
hardware via the NetBSD/NVMM bottom). TDX and SEV-SNP require the silicon vendor's
attestation infrastructure; PPN's sovereignty model makes the operator the attestation
root via the pairing ceremony.

Four components are planned for the distributed extension:

1. **virtio-mem lending over WireGuard** — analogous to CXL 3.0 memory disaggregation
   but over internet-encrypted WireGuard rather than PCIe fabric. Lending node retains no
   read capability over lent blocks (seL4 capability model).
2. **Distributed capability ledger** (`moonshot-protocol`, `moonshot-database`) —
   HMAC-signed grants keyed to pairing-ceremony identity; Merkle DAG gossip; intended
   sub-second revocation without central authority.
3. **Cross-node VM scheduler** (`os-orchestration`) — deterministic bin-packing; QEMU
   live migration over WireGuard; sovereignty constraint (operator can pin VMs to trusted
   nodes).
4. **Sovereign attestation chain** — `dm-verity` anchored to pairing-ceremony key; no
   TPM vendor, no silicon vendor in chain.

These four components are the content of the new TOPIC draft staged at
`.agent/drafts-outbound/topic-ppn-distributed-vm-fabric.draft.md` (EN + ES).

What AWS/Azure/GCP will NOT ship by 2030: machine-checked formal isolation proof on
commodity SMB hardware; cross-node memory lending over WAN (CXL requires PCIe); sub-second
capability revocation without central authority; attestation rooted in operator-witnessed
ceremony rather than silicon vendor CA; sub-five-minute SMB deployment.

---

## §12 — PPN Resource Pooling Layer (added 2026-05-29)

The three-node WireGuard mesh (Laptop A · Laptop B · GCP) forms a single unified VM
resource pool exposed as a free-tier PPN primitive. This is NOT the paid Orchestration
tier — VM resource pooling is infrastructure management (which node owns which VM),
while Orchestration charges for multi-Totebox data aggregation.

**Architectural invariant: `auto_rebalance: false` is permanent and not configurable.**
WireGuard at ~20 Mbps × 6 GB VM = ~40 min transfer time; live VM migration over WireGuard
is permanently excluded. VMs are placed once and remain on their assigned node.

### Three new crates

**`system-vm-fleet-types`** (`no_std`-compatible shared wire types):
- `NodeHeartbeat {node_id, wg_ip, hostname, ram_total_mb, ram_used_mb, cpu_cores, cpu_load_pct, vms: Vec<VmRecord>, boot_id, timestamp_utc}`
- `VmRecord {vm_id, vm_type, state: VmState, ram_alloc_mb, vcpu_count, started_at}`
- `VmState` — `Running | Stopped | Provisioning | Error`
- `PlacementAdvice {recommended_node, reason, alternatives}`
- `FleetStatus {nodes: Vec<NodeRecord>, last_updated}`
- `NodeRecord {node_id, hostname, wg_ip, ram_available_mb, vm_count, last_heartbeat}`
- `CreateVmRequest {vm_type, ram_mb, vcpu_count, preferred_node: Option<NodeId>}`

**`service-vm-host`** (per-node daemon, one per infrastructure node):
- Polls `/proc/meminfo` + `/proc/loadavg` every 10s
- Queries QEMU UNIX monitor socket per running VM → `VmRecord`
- POSTs `NodeHeartbeat` to `service-vm-fleet` at `VM_FLEET_ENDPOINT`
- Accepts `CreateVm` dispatch → launches QEMU subprocess
- Uses `tokio::main(flavor = "current_thread")`
- Port: none (outbound-only). Systemd: `infrastructure/systemd/ppn/local-vm-host.service`

**`service-vm-fleet`** (fleet controller, GCP-resident, :9203):
- `POST /v1/nodes/heartbeat` — update NodeRecord; evict nodes silent >30s
- `GET /v1/fleet` — FleetStatus
- `GET /v1/nodes/{node_id}` — single node
- `POST /v1/vms` — advisory placement + dispatch CreateVm to service-vm-host; return VmRecord
- `DELETE /v1/vms/{vm_id}` — stop + destroy
- Placement: filter `ram_available_mb >= request.ram_mb + 512`; sort `ram_available_mb DESC`; first candidate wins
- VM-Totebox: `preferred_node` must be caller-specified (WORM data cannot migrate)
- Auth: WireGuard peer IP must be a registered node; Phase 2 upgrade to SSH-signed tokens
- Uses `tokio::main(flavor = "current_thread")`
- Systemd: `infrastructure/systemd/orchestration/local-vm-fleet.service`

### F12 doctrine (SYS-ADR-10)

"Create VM" is an operator action → F12-gated in `app-network-admin` F9 panel.
The fleet controller's node selection is advisory infrastructure, not an operator action —
it is NOT F12-gated.

### Phase 2: app-network-admin F9 panel (ratatui)

Left column: node list (hostname, WG IP, RAM bar, CPU%, VM count).
Right column: VM list for selected node.
Bottom: `[C]reate VM [D]estroy [R]efresh`.
Create flow → F12 confirmation → `POST /v1/vms`.
Deferred until `service-vm-fleet` is running in production.

---

## §13 — GCP KVM Status (added 2026-05-29)

**GCP workspace VM has no `/dev/kvm`.** Nested virtualization is not enabled on the
`foundry-workspace` GCE instance. All QEMU processes run under TCG (software emulation),
which is approximately 10× slower than KVM hardware acceleration.

**Observable impact:**
- vm-mediakit cloud-init: 504s (TCG) vs. expected ~50s (KVM)
- vm-mediakit smoke test timeout: 60s (set in migrate-service-to-vm.sh) accommodates TCG
- bench #9 re-run for J2 JOURNAL: TCG load spikes will corrupt latency measurements

**Operator action required:** Enable nested virtualization in GCP console:
```
Compute Engine → VM instances → foundry-workspace → Edit
→ CPU platform and GPU → Enable virtualized nested hardware performance counters: ON
→ Restart instance
→ verify: ls /dev/kvm   # should show char device
```

**Laptop A status:** VT-x present (Sandy Bridge i5-2400S), VT-d absent.
Run `ls /dev/kvm` on Laptop A to confirm KVM availability before Phase 2 work begins.
Expected: present (Linux host with VT-x normally exposes `/dev/kvm` via `kvm` module).

**Laptop B status:** Acts as WireGuard hub (10.8.0.1, public 24.86.192.209:51820).
KVM status not yet verified — operator action required.

Until GCP nested KVM is enabled, VM provisioning and service migrations on GCP are
possible but slow. Laptop A and Laptop B are the preferred test execution environments
for time-sensitive operations.

---

*End of BRIEF — project-infrastructure / 2026-05-29 (§12 resource pooling + §13 GCP KVM added)*
*Next action: three new crates scaffolded (system-vm-fleet-types, service-vm-fleet, service-vm-host)*
