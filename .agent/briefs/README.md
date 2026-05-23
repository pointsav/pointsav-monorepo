# .agent/briefs/ — Durable project briefs

Git-tracked, permanent artifacts. Never delete; supersede by setting `status: archived`
or moving to `briefs/archive/`. All engines read this index at session start (AGENT.md step 7).

---

## Active briefs

| File | Purpose |
|---|---|
| [BRIEF-flow-restructure.md](BRIEF-flow-restructure.md) | **PRIMARY PLAN OF RECORD** — $7-node fleet architecture; node-class ladder; Phases 0–5 DONE; Phase 6 deferred |
| [BRIEF-vm-hardening-and-consolidation.md](BRIEF-vm-hardening-and-consolidation.md) | **ACTIVE TODO** — remove 7B model from VM; deploy Phase 4/5 binaries; BRIEF consolidation + conflict resolutions (operator approval needed); Stage 6 |
| [BRIEF-sovereign-routing-comprehensive.md](BRIEF-sovereign-routing-comprehensive.md) | EXTENDS primary — legal research, Sprint 0–5 engineering detail, training data format, LoRA hyperparameters |
| [BRIEF-universal-ai-gateway.md](BRIEF-universal-ai-gateway.md) | EXTENDS primary — exact LOC/file Sprint breakdown, pricing/customer transition milestones |
| [BRIEF-learning-loop-master-plan.md](BRIEF-learning-loop-master-plan.md) | EXTENDS primary — corpus quality gate, eval harness, DPO pairs, Phase 1.1–1.10 specifics |
| [BRIEF-tier-architecture.md](BRIEF-tier-architecture.md) | EXTENDS primary — model family ratification, BCSC-permissible families, 7 gap priorities |
| [BRIEF-service-content-architecture.md](BRIEF-service-content-architecture.md) | EXTENDS primary + CONFLICT — Ring 2/3 coupling violation (graph halts without Doorman); 5-sprint PUSH inversion |
| [BRIEF-service-slm-hardening.md](BRIEF-service-slm-hardening.md) | EXTENDS primary — post-crash recovery state, Task 2–5 immediate next steps |
| [BRIEF-phase-3c-service-content-loRA-stub.md](BRIEF-phase-3c-service-content-loRA-stub.md) | EXTENDS primary — deferred Phase 3.6–3.10 (draft generation, citation linkage, LoRA scheduler) |
| [BRIEF-lbug-build-blocker.md](BRIEF-lbug-build-blocker.md) | CONFLICT with primary — cargo build blocked (static link); detailed blocker record; keep active |
| [BRIEF-claim-authoring-convention.md](BRIEF-claim-authoring-convention.md) | UNRELATED — knowledge platform editorial convention (pending Command ratification) |
| [BRIEF-KNOWLEDGE-PLATFORM-PLAN.md](BRIEF-KNOWLEDGE-PLATFORM-PLAN.md) | UNRELATED — knowledge platform 8-phase execution plan |
| [BRIEF-KNOWLEDGE-PLATFORM-VISION.md](BRIEF-KNOWLEDGE-PLATFORM-VISION.md) | UNRELATED — knowledge platform upstream vision + architecture |
| [BRIEF-layer3-compliance-report.md](BRIEF-layer3-compliance-report.md) | UNRELATED — security/WireGuard compliance (URGENT — separate track) |
| [BRIEF-service-audit.md](BRIEF-service-audit.md) | ABSORBED by primary — defect list pre-Phase-4; pending operator approval to archive |
| [BRIEF-service-slm-architecture.md](BRIEF-service-slm-architecture.md) | ABSORBED by primary — Sprint 0a prerequisite check, executed; pending operator approval to archive |
| [BRIEF-MASTER-PLAN-2026.md](BRIEF-MASTER-PLAN-2026.md) | ABSORBED by primary — entry-point index superseded; pending operator approval to archive |
| [BRIEF-olmo-performance-tuning.md](BRIEF-olmo-performance-tuning.md) | ABSORBED — 7B model measurements (model being removed); pending operator approval to archive |
| [BRIEF-flow-bottleneck-strategic-review.md](BRIEF-flow-bottleneck-strategic-review.md) | Self-declared superseded — mark archived |

## Archived briefs

| File | Purpose |
|---|---|
| [archive/BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md](archive/BRIEF-WIKIPEDIA-PARITY-MASTER-PLAN.md) | Wikipedia parity master plan (superseded) |
| [archive/BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md](archive/BRIEF-WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md) | Wikipedia parity functional index (superseded) |
| [archive/BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md](archive/BRIEF-WIKIPEDIA-PARITY-RESEARCH-LOG.md) | Wikipedia parity research log (superseded) |

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
