---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-ai-audit-baseline-2026-05
title: "AI Audit Baseline — project-intelligence Ecosystem 2026-05"
status: archived
owner: project-intelligence
created: 2026-05-31
updated: 2026-06-12
author: gemini-cli (totebox@project-intelligence)
reviewed: 2026-06-01 claude-sonnet-4-6
moved_to: project-intelligence
archived: 2026-06-12
---

# AI-Auditable Baseline Brief: project-intelligence Ecosystem
**Date:** 2026-05-31
**Context:** Audit of `service-slm`, `service-content`, and `app-orchestration-slm` against verified architectural principles (seL4-based, WORM-ready, Asset-Level Ownership).

---

## 1. Executive Summary
This brief establishes a baseline for automated auditability of the `project-intelligence` ecosystem. The codebase currently implements proprietary commercial-layer brokers on top of flat-file archives, successfully maintaining `moduleId` isolation. However, it relies on shims (transient Spot VM provisioning) for security and performance targets that are intended to be native in the long-term `moonshot-*` trajectory.

## 2. Architecture Compliance Audit
| Principle | Implementation Status | Benchmark Gap (Ref: Industry Standard) |
| :--- | :--- | :--- |
| **Asset-Level Ownership** | Enforced via `moduleId` isolation. | Highly compliant; matches benchmark. |
| **WORM Compliance** | File-system based (JSONL, LadybugDB). | **Gap:** Lack of cryptographic sealing at write-time; rely on `service-fs` stub. |
| **seL4 Foundation** | Managed via VM shims. | **Planned (Phase 2/3):** Not native microkernel execution yet; TCB is the Linux VM. This is the documented Phase 2 (NetBSD/NVMM) and Phase 3 (seL4 Microkit AArch64) roadmap — not an unplanned gap. See archived BRIEF-LEAPFROG-2030.md. |
| **Flat-File Permanence** | JSONL, YAML, LadybugDB. | Highly compliant; vendor-neutral exportable. |

## 3. Identified Gaps & Architectural Debt
- **service-slm (Routing Debt):**
  - `eur-west4-a` zone-locking: **by design** — the 256 GB weights disk is zone-bound; zone fallback is a deliberate non-policy per BRIEF-slm-substrate-master.md §2.2 (`SLM_YOYO_ALLOW_ZONE_FALLBACK=false` permanent). This is not debt.
  - Circuit breaker diagnostics are basic; lacks telemetry to distinguish transient vs. fatal state.
- **service-content (Legacy State):**
  - Relies on legacy file-watchers instead of MCP-based event triggers.
  - Significant hardcoded developer paths in `main.rs`.
- **app-orchestration-slm (Broker Stability):**
  - Metering/Registry is currently ephemeral in-memory state; requires persistence (Redb/SQLite) for production audit trails.

## 4. Actionable Remediation Roadmap
1. **[P0] Audit Ledger:** Implement mandatory cryptographic hashing (`sha256`) at point of entry in `slm-doorman/src/ledger.rs` to reach WORM baseline.
2. **[P1] Telemetry:** Augment `/readyz` to include JSON-structured `circuit_breaker_state` (e.g., `{"reason": "stockout", "zone": "eur-west4-a"}`).
3. **[P2] Decoupling:** Move hardcoded paths in `service-content` to environment-based configuration (`INFRASTRUCTURE_ROOT`, `CORPUS_ROOT`).
4. **[P3] Infrastructure:** Commit current systemd unit files to `infrastructure/` to eliminate out-of-repo configuration drift.

---
*This file is intended for future automated diffs to verify structural adherence against these benchmarks.*
