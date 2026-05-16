<div align="center">

# Sovereign Replacement Initiative | Iniciativa de Reemplazo Soberano

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

### *An active engineering initiative to replace foreign third-party architecture.*

</div>

<br/>

> [!WARNING]
> **SOVEREIGN FRAMEWORK DECLARATION**
> This repository is a reference implementation of the Sovereign Data Protocol. It enforces absolute data isolation. It contains zero active proprietary network payloads.

| Architecture Tier | Component Role | Governance Anchor |
| :--- | :--- | :--- |
| 🔴 Research | Sovereign Model Training | Sovereign Data Foundation |

## I. WHAT THIS MOONSHOT REPLACES

`moonshot-slm` targets the external AI API inference dependency — specifically the Anthropic Tier C inference path routed through `service-slm` (Doorman). All Tier C language model requests currently leave the sovereign infrastructure perimeter and are fulfilled by a third-party hosted model. This moonshot eliminates that dependency by training, evaluating, and deploying a purpose-built sovereign model that operates entirely within Foundry infrastructure.

## II. WHAT IS BEING BUILT

**Training pipeline.** An automated supervised fine-tuning (SFT) and direct preference optimisation (DPO) pipeline operating against an OLMo base model (Apache 2.0). Training data flows from `service-slm` — the `/v1/shadow` endpoint captures development session tuples — and from `service-content` — graph-validated entity extraction rows.

**Adapter lifecycle management.** Separate LoRA adapters for distinct task families:
- `coding-lora` — trained from development session tuples captured during operator workflows via post-commit hook.
- `extraction-lora` — trained from `service-content` graph-validated entity extraction rows.

Each adapter follows a SLSA-attested promotion lifecycle: train → evaluate → promote → deploy. Promotion is gated by an evaluation harness that holds out a 100-pair corpus and rejects any adapter exhibiting greater than 5% regression against the prior checkpoint.

**Continual learning.** A 20–30% replay buffer and KL-divergence anchor (β=0.1) prevent adapter updates from eroding previously learned capabilities. Adapter versions are retained as immutable ledger entries in GCS.

**Long-term objective.** A domain-specific model trained exclusively on sovereign data that handles all Tier B tasks without any external model API call. The Anthropic Tier C path becomes a fallback only, then is retired.

## III. CURRENT STATE

Research / Placeholder. The training data capture infrastructure exists in `service-slm` (apprenticeship substrate, `/v1/shadow` endpoint). The training runtime and adapter management system are the implementation target of this moonshot. No training runs have been executed.

**Implementation trigger:** ≥1,000 DPO training tuples accumulated in the service-slm apprenticeship corpus (~6–8 weeks of normal development pace after Sprint 0b activation).

## IV. CROSS-REFERENCES

| Component | Relationship |
| :--- | :--- |
| `service-slm` | Inference gateway; captures shadow training data via `/v1/shadow` |
| `service-content` | Source of extraction-lora training rows (graph-validated only) |
| `moonshot-gpu` | GPU substrate required for training runs |
| `app-console-slm` | Operator console for monitoring corpus state and adapter promotion |

---
*© 2026 PointSav Digital Systems™.*
*Public Architectural Blueprint. Governed by the Sovereign Data Protocol.*
