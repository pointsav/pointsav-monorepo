---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.1"
title: "TODO — working title: Capability-Secured Session Orchestration: A Runtime Architecture for Multi-Tenant AI Workload Isolation"
target_journal: "TODO — OSDI / EuroSys / MLSys"
target_publisher: "USENIX / ACM / MLSys"
impact_factor: ""
alternate_venue: "ACM SOSP; IEEE TPDS"
authors:
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Software
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Formal Analysis
      - Writing – Review & Editing
subject_codes:
  - "D.4.1 Process Management"
  - "D.4.6 Security and Protection"
  - "I.2.11 Distributed Artificial Intelligence"
keywords:
  - session orchestration
  - capability-based security
  - AI workload isolation
  - multi-tenant runtime
  - reproducible execution
  - transparency logs
bcsc_class: no-disclosure-implication
ai_tool_used: ""
corresponding_author: jmwoodfine@gmail.com
word_count_body: 0
word_count_target: 9500
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
notes_for_editor: |
  STUB — no body content yet. Research material needed before writing pass.

  Suggested research sources to gather before writing:
    - service-slm scaffold + BRIEF-slm-substrate-master.md (Yo-Yo VM, OLMo-3, DataGraph)
    - app-console-* for session management architecture
    - conventions/apprenticeship-substrate.md
    - conventions/trajectory-substrate.md
    - DOCTRINE.md claims #28, #29 (substrate substitution)
    - JOURNAL-trustworthy-systems-v0.1.draft.md §3 (capability ledger substrate)
      as the base layer this paper builds on top of

  Contribution angle (hypothesis):
    A session-orchestration runtime that gates each AI inference invocation against
    a customer-rooted capability ledger (built on the substrate in
    JOURNAL-trustworthy-systems) achieves measurable multi-tenant isolation
    guarantees — auditable, transferable, and vendor-independent — while adding
    <N ms overhead to inference latency compared to an unaudited baseline.

  This paper is downstream of JOURNAL-trustworthy-systems (cites it as prior work).
  Write JOURNAL-trustworthy-systems first; this paper's §2 cites it.

  Pre-writing checklist:
    1. Gather service-slm implementation material
    2. Define isolation metric (what does "isolated" mean here, precisely?)
    3. Define H₁ (isolation with audit) + falsification tests before writing §3+
    4. forbidden_terms_cleared: set to true after language pass
    5. ORCID IDs for all three authors
    6. Confirm target venue (OSDI vs. MLSys vs. EuroSys) based on system vs. ML angle
---

# TODO: Capability-Secured Session Orchestration

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

---

## Abstract

TODO

---

## 1. Introduction

TODO — gap statement: existing AI runtime frameworks (vLLM, Triton, TensorRT-LLM) do not publish per-session capability state to a customer-rooted audit log. State three contributions.

---

## 2. Background and Related Work

TODO — Cite JOURNAL-trustworthy-systems (the substrate this builds on). Cover vLLM, Triton, existing multi-tenant isolation work, seL4-based isolation.

---

## 3. Session Orchestration Architecture

TODO

---

## 4. Implementation

TODO

---

## 5. Evaluation

TODO — Latency overhead of capability check per inference invocation. Audit log throughput. Isolation test under concurrent tenant workloads.

---

## 6. Discussion

### 6.1 Formal Hypotheses

TODO — H₁ (isolation with bounded overhead), H₀, H₂

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

TODO — will cite JOURNAL-trustworthy-systems-v0.1 as prior work when published.

---

*Version 0.1 stub — 2026-05-27*
*All section bodies are TODO — stub only*
*Downstream of JOURNAL-trustworthy-systems; write that paper first*
