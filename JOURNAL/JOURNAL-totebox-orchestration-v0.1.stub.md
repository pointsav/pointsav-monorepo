---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.1"
title: "TODO — working title: Capability-Secured Session Orchestration: A Runtime Architecture for Multi-Tenant AI Workload Isolation"
target_journal: "MLSys (ACM Conference on Machine Learning and Systems)"
target_publisher: "ACM"
impact_factor: ""
acceptance_rate: "22% (2025)"
alternate_venue: "ASPLOS (ACM SIGARCH, 19.4% AR); OSDI (USENIX, ~20-25% AR)"
authors:
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Software
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
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
corresponding_author: corporate.secretary@woodfinegroup.com
word_count_body: 0
word_count_target: 9500
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
preprint_posted: true
preprint_posted_date: 2026-05-28
doi: ""
license: "CC BY 4.0"
cite_as: "Woodfine, Mathew, Woodfine, Peter M., & Woodfine, Jennifer M. (2026). Capability-Secured Session Orchestration. Working Paper v0.1, 28 May 2026. Woodfine Management Corp., New York, NY."
revision_history:
  - version: "0.1"
    date: "2026-05-28"
    changes: "Frontmatter and skeleton only; HOLD pending J2 submission; preprint notice added; public posting"
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

> **Working Paper · Version 0.1 · 2026-05-28 · CC BY 4.0**
> This manuscript is a working draft. It has not been peer reviewed. Findings are preliminary and subject to revision without notice. Correspondence: corporate.secretary@woodfinegroup.com.
>
> *Cite as:* Woodfine, Mathew, Woodfine, Peter M., & Woodfine, Jennifer M. (2026). Capability-Secured Session Orchestration. Working Paper v0.1, 28 May 2026. Woodfine Management Corp., New York, NY.

> **Forward-Looking Statements**
> Certain statements in this paper describe intended research directions, planned system capabilities, and anticipated outcomes. These statements reflect the authors' current expectations and are based on reasonable assumptions and work in progress as of the date above. Actual results, measurements, and findings may differ materially. Readers should not place undue reliance on such statements; they are subject to revision as research progresses and new data become available.

# TODO: Capability-Secured Session Orchestration

**Mathew Woodfine, Peter M. Woodfine, and Jennifer M. Woodfine**  
Woodfine Management Corp., New York, NY, USA  
*Corresponding author:* corporate.secretary@woodfinegroup.com

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
