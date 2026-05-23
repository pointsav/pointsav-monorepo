# .agent/briefs/ — Durable project briefs

Git-tracked, permanent artifacts. Never delete; supersede by setting `status: archived`
or moving to `briefs/archive/`. All engines read this index at session start (AGENT.md step 7).

---

## Active briefs

| File | Purpose |
|---|---|
| [BRIEF-flow-restructure.md](BRIEF-flow-restructure.md) | **PRIMARY PLAN OF RECORD** — $7-node fleet architecture; node-class ladder; Phases 0–5 DONE; Phase 6 next; §12 = Command Session transition checklist |
| [BRIEF-service-content-architecture.md](BRIEF-service-content-architecture.md) | EXTENDS primary — Ring 2/3 coupling defect; 5-sprint PUSH inversion; Sprint 1 (~30 LOC) is the immediate next code item |
| [BRIEF-lbug-build-blocker.md](BRIEF-lbug-build-blocker.md) | CONFLICT — cargo build blocked (static link); Options A/B/C to try; keep active until resolved |
| [BRIEF-sovereign-routing-comprehensive.md](BRIEF-sovereign-routing-comprehensive.md) | EXTENDS primary — legal research; Sprint 0–5 detail; Sprint 0a done; training data format; LoRA hyperparameters |
| [BRIEF-universal-ai-gateway.md](BRIEF-universal-ai-gateway.md) | EXTENDS primary — exact LOC/file Sprint breakdown; pricing/customer transition milestones |
| [BRIEF-tier-architecture.md](BRIEF-tier-architecture.md) | EXTENDS primary — model family policy; BCSC-permissible families; §2 corrected 2026-05-23 (1B not 7B for Tier A) |
| [BRIEF-learning-loop-master-plan.md](BRIEF-learning-loop-master-plan.md) | EXTENDS primary — corpus quality gate; eval harness; DPO pairs; **deferred to Phase 7** (7B premise corrected) |
| [BRIEF-phase-3c-service-content-loRA-stub.md](BRIEF-phase-3c-service-content-loRA-stub.md) | EXTENDS primary — deferred Phase 3.6–3.10 (draft generation, citation linkage, LoRA scheduler) |
| [BRIEF-claim-authoring-convention.md](BRIEF-claim-authoring-convention.md) | UNRELATED — knowledge platform editorial convention (pending Command ratification) |
| [BRIEF-KNOWLEDGE-PLATFORM-PLAN.md](BRIEF-KNOWLEDGE-PLATFORM-PLAN.md) | UNRELATED — knowledge platform 8-phase execution plan |
| [BRIEF-KNOWLEDGE-PLATFORM-VISION.md](BRIEF-KNOWLEDGE-PLATFORM-VISION.md) | UNRELATED — knowledge platform upstream vision + architecture |
| [BRIEF-layer3-compliance-report.md](BRIEF-layer3-compliance-report.md) | UNRELATED — security/WireGuard compliance (URGENT — separate track) |

## Archived briefs

| File | Archived | Reason |
|---|---|---|
| [BRIEF-service-slm-hardening.md](BRIEF-service-slm-hardening.md) | 2026-05-23 | Pre-Phase-4 state; Tasks 2+3 moot (local Tier A removed); all relevant items in flow-restructure |
| [BRIEF-vm-hardening-and-consolidation.md](BRIEF-vm-hardening-and-consolidation.md) | 2026-05-23 | §3A conflicts resolved in flow-restructure; ops checklist → §12; nothing unique remains |
| [BRIEF-MASTER-PLAN-2026.md](BRIEF-MASTER-PLAN-2026.md) | 2026-05-23 | Absorbed by flow-restructure (PRIMARY PLAN) |
| [BRIEF-olmo-performance-tuning.md](BRIEF-olmo-performance-tuning.md) | 2026-05-23 | Measures the 7B model removed from VM |
| [BRIEF-service-audit.md](BRIEF-service-audit.md) | 2026-05-23 | Absorbed by flow-restructure §8 |
| [BRIEF-service-slm-architecture.md](BRIEF-service-slm-architecture.md) | 2026-05-23 | Sprint 0a executed; absorbed by flow-restructure §8.C |
| [BRIEF-flow-bottleneck-strategic-review.md](BRIEF-flow-bottleneck-strategic-review.md) | 2026-05-23 | Self-declared superseded by flow-restructure |
| [archive/BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md](archive/BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md) | prior | Wikipedia parity master plan (superseded) |
| [archive/BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md](archive/BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md) | prior | Wikipedia parity functional index (superseded) |
| [archive/BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md](archive/BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md) | prior | Wikipedia parity research log (superseded) |

---

## Artifact routing (at milestone)

| Artifact type | Gateway project | Destination |
|---|---|---|
| TOPIC-* | project-editorial | content-wiki-documentation / content-wiki-projects / content-wiki-corporate |
| GUIDE-* | project-editorial | woodfine-fleet-deployment/\<cluster\>/ |
| COMMS-*, LEGAL-*, TRANSLATE-* | project-editorial | varies |
| DESIGN-COMPONENT, DESIGN-RESEARCH | project-design | pointsav-design-system |
| DESIGN-TOKEN-* | project-design | pointsav-design-system (requires master_cosign) |
| BIM-* | project-bim | woodfine-design-bim |
| CODE-* / SCRIPT-* / CONFIG-* / DATA-* | self-contained | direct commit |
