# .agent/briefs/ — Durable project briefs

Git-tracked, permanent artifacts. Never delete; supersede by setting `status: archived`
or moving to `briefs/archive/`. All engines read this index at session start (AGENT.md step 7).

---

## Active briefs

| File | Purpose |
|---|---|
| [BRIEF-flow-restructure.md](BRIEF-flow-restructure.md) | **PRIMARY PLAN OF RECORD** — $7-node fleet architecture; per-repo to-do (§8) + execution order (§9); Phase 0 done; resume at §9 step 1 |
| [BRIEF-sovereign-routing-comprehensive.md](BRIEF-sovereign-routing-comprehensive.md) | Comprehensive sovereign routing plan; Sprint 0a built (http.rs:1214); superseded in part by flow-restructure |
| [BRIEF-universal-ai-gateway.md](BRIEF-universal-ai-gateway.md) | Universal AI gateway — Anthropic shim, Yo-Yo fleet, app-console-slm end-state |
| [BRIEF-service-slm-architecture.md](BRIEF-service-slm-architecture.md) | service-slm architecture reference (2026) |
| [BRIEF-service-content-architecture.md](BRIEF-service-content-architecture.md) | service-content architecture reference (2026) |
| [BRIEF-tier-architecture.md](BRIEF-tier-architecture.md) | Tier 0–3 ladder architecture reference |
| [BRIEF-service-slm-hardening.md](BRIEF-service-slm-hardening.md) | service-slm hardening plan (2026-05-18) |
| [BRIEF-service-audit.md](BRIEF-service-audit.md) | Service audit findings (2026-05-16) |
| [BRIEF-lbug-build-blocker.md](BRIEF-lbug-build-blocker.md) | LadybugDB build blocker history; MemoryMax raised to 6G; shared-lib path chosen |
| [BRIEF-olmo-performance-tuning.md](BRIEF-olmo-performance-tuning.md) | OLMo performance tuning notes |
| [BRIEF-learning-loop-master-plan.md](BRIEF-learning-loop-master-plan.md) | Learning loop master plan (2026-05-18) |
| [BRIEF-flow-bottleneck-strategic-review.md](BRIEF-flow-bottleneck-strategic-review.md) | Flow bottleneck strategic review (2026-05-21) |
| [BRIEF-claim-authoring-convention.md](BRIEF-claim-authoring-convention.md) | Claim-authoring convention proposal |
| [BRIEF-KNOWLEDGE-PLATFORM-PLAN.md](BRIEF-KNOWLEDGE-PLATFORM-PLAN.md) | Knowledge platform 8-phase execution plan |
| [BRIEF-KNOWLEDGE-PLATFORM-VISION.md](BRIEF-KNOWLEDGE-PLATFORM-VISION.md) | Knowledge platform upstream vision + architecture |
| [BRIEF-MASTER-PLAN-2026.md](BRIEF-MASTER-PLAN-2026.md) | Master plan 2026 |
| [BRIEF-phase-3c-service-content-loRA-stub.md](BRIEF-phase-3c-service-content-loRA-stub.md) | Phase 3c — service-content LoRA stub (from workspace) |
| [BRIEF-layer3-compliance-report.md](BRIEF-layer3-compliance-report.md) | Layer 3 compliance report (from workspace) |

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
