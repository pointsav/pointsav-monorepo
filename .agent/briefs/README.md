# .agent/briefs/ — Durable project briefs

Git-tracked, permanent artifacts. Never delete; supersede by setting `status: archived`
or moving to `briefs/archive/`. All engines read this index at session start (AGENT.md step 7).

---

## Active briefs

| File | Purpose |
|---|---|
| [BRIEF-slm-substrate-master.md](BRIEF-slm-substrate-master.md) | **PRIMARY PLAN OF RECORD** — SLM substrate (Yo-Yo + DataGraph + Learning Loop); replaces BRIEF-flow-restructure.md (lost 2026-05-22 rebase); §5/§6 = immediate + pending work; §2 = Yo-Yo permanent VM design |
| [BRIEF-claim-authoring-convention.md](BRIEF-claim-authoring-convention.md) | UNRELATED — knowledge platform editorial convention (pending Command ratification) |
| [BRIEF-KNOWLEDGE-PLATFORM-PLAN.md](BRIEF-KNOWLEDGE-PLATFORM-PLAN.md) | UNRELATED — knowledge platform 8-phase execution plan |
| [BRIEF-KNOWLEDGE-PLATFORM-VISION.md](BRIEF-KNOWLEDGE-PLATFORM-VISION.md) | UNRELATED — knowledge platform upstream vision + architecture |
| [BRIEF-layer3-compliance-report.md](BRIEF-layer3-compliance-report.md) | UNRELATED — security/WireGuard compliance (URGENT — separate track) |

**Note:** The following BRIEFs listed above were referenced in README but are no longer on disk
(lost in Stage-6 rebase contamination 2026-05-22). Their open items are absorbed into
`BRIEF-slm-substrate-master.md §6`:
`BRIEF-flow-restructure.md`, `BRIEF-service-content-architecture.md`, `BRIEF-lbug-build-blocker.md`,
`BRIEF-sovereign-routing-comprehensive.md`, `BRIEF-universal-ai-gateway.md`,
`BRIEF-tier-architecture.md`, `BRIEF-learning-loop-master-plan.md`, `BRIEF-phase-3c-service-content-loRA-stub.md`

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
