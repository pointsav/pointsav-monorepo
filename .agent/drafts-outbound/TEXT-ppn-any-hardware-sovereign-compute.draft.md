---
schema: foundry-draft-v1
artifact: TEXT
draft-id: project-infrastructure-text-ppn-any-hardware
title: "Any Hardware, Sovereign Compute"
status: staged
owner: project-infrastructure
created: 2026-06-14
updated: 2026-06-14
route: project-editorial
destination: pointsav.com product page (planned)
language_protocol: EN-only
bcsc_class: internal-draft
gaps: []
research_trail:
  methodology: "Derived from BRIEF-ppn-infrastructure-reference.md §15 (PPN Install Model)"
  sources_consulted: ["BRIEF-ppn-infrastructure-reference.md §15"]
  claims_verified: true
  forbidden_terms_cleared: false
  notes: "Needs Bloomberg vocabulary check before publication; no Sovereign Data Foundation or forward-looking product claims as fact"
---

# Any Hardware, Sovereign Compute

Your compute costs should not require a long-term commitment to a single cloud provider. A
physical laptop sitting in a closet has processing power, memory, and fast-boot virtualization
capabilities that would cost hundreds of dollars a month to rent from a cloud service — if you
rented them at all. PointSav Private Network (PPN) puts that hardware to work.

**How it works.** Installing the PPN node software on any machine — an old laptop, a leased
data-center server, or a cloud VM — adds it to a shared, encrypted resource pool. Each node
advertises its available memory, processor type, and virtualization support. When a workload
needs to run, PPN selects the most suitable node automatically, starts a virtual machine in
seconds, and routes the request there. All traffic between nodes passes through an encrypted
tunnel; your internet provider and cloud vendor see only encrypted data, not the workloads
running inside.

**What you keep.** Adding cloud VMs to the pool does not hand the cloud provider authority
over your applications. The PPN mesh treats a cloud VM the same as any other node: a block of
compute with a WireGuard address. Your license keys, configuration, and data remain under your
control. Any node can be removed from the pool at any time.

**The economics.** A small relay instance on a cloud provider (approximately $15 per month)
serves as the coordination point for the mesh. Old laptops or desktops that would otherwise
sit unused contribute virtual-machine-accelerated compute to the pool at no additional cost.
Three heterogeneous machines — a cloud VM, a MacBook Pro, and a MacBook Air — have been
tested forming a live resource pool. The setup on each new machine takes approximately ten
minutes.

**Current state.** The PPN resource pool is in active operation. The planned single-ISO
install experience — boot the machine, answer three questions, done — is in development.

---

*For technical detail on the architecture, isolation model, and planned seL4 formally-verified
isolation layer, see the [PPN VM Resource Pool Architecture] topic.*
