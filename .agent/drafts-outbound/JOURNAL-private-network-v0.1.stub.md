---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.1"
title: "TODO — working title: Zero-Trust Private Network Architecture for Distributed Operational Systems: A Customer-Rooted Mesh Design"
target_journal: "IEEE Journal on Selected Areas in Communications"
target_publisher: "IEEE"
impact_factor: "13.8"
alternate_venue: "IEEE/ACM Transactions on Networking; USENIX NSDI"
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
  - customer-sovereign networking
bcsc_class: no-disclosure-implication
ai_tool_used: ""
corresponding_author: jmwoodfine@gmail.com
word_count_body: 0
word_count_target: 9000
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
notes_for_editor: |
  STUB — no body content yet. Research material needed before writing pass.

  Suggested research sources to gather before writing:
    - app-network-* reserved-folder directories for architecture intent
    - service-vpn scaffold code (11 files — WireGuard-based)
    - system-network-interface scaffold code (6 files)
    - conventions/three-ring-architecture.md — three-ring topology doctrine
    - conventions/system-substrate-doctrine.md §6 (network substrate)
    - DOCTRINE.md §III (network topology claims)

  Contribution angle (hypothesis):
    A customer-rooted private network using WireGuard tunnels, a centralised
    key-distribution service, and a declarative mesh configuration achieves
    equivalent isolation guarantees to commercial ZTA products (Zscaler,
    Cloudflare Zero Trust, Palo Alto Prisma) without vendor-held routing or
    policy keys — measurable via: rekey latency, tunnel establishment time,
    policy-change propagation time, and failure-mode behaviour under node loss.

  Target special issue: IEEE JSAC "Zero Trust for Next-Generation Networking"
  (check current CFP — special issues rotate annually).

  Pre-writing checklist:
    1. Gather architecture material from service-vpn and system-network-interface
    2. Draft §1 gap statement: no production ZTA architecture publishes customer-held
       routing keys + transparent audit log of mesh state changes
    3. Define H₁ (isolation equivalence) + falsification programme before writing §3+
    4. forbidden_terms_cleared: set to true after language pass
    5. ORCID IDs for all three authors
---

# TODO: Zero-Trust Private Network Architecture for Distributed Operational Systems

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

---

## Abstract

TODO

---

## 1. Introduction

TODO — gap statement: commercial ZTA products route traffic through vendor-controlled infrastructure; customer-held routing keys and transparent mesh-state audit logs are unavailable. State three contributions.

---

## 2. Background and Related Work

TODO

---

## 3. Architecture

TODO

---

## 4. Implementation

TODO

---

## 5. Evaluation

TODO

---

## 6. Discussion

### 6.1 Formal Hypotheses

TODO — H₁, H₀, H₂

### 6.2 Falsification Programme

TODO

### 6.3 Limitations

TODO

---

## 7. Conclusion

TODO

---

## AI Use Disclosure

TODO

## CRediT Contributor Roles

TODO

## Conflict of Interest

The authors declare no conflict of interest.

## Funding

No external funding received.

## Data Availability

TODO

## References

TODO

---

*Version 0.1 stub — 2026-05-27*
*All section bodies are TODO — stub only*
