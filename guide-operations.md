# GUIDE: TERMINAL OPERATIONS & PARAMETRIC COMPUTE

## THE PARAMETRIC COMPUTE MATRIX
The Command Ledger chassis eliminates non-deterministic text generation ("prompt engineering"). Compute is executed strictly via structural parameters defined by the Operator.

### LINGUISTIC DOMAINS (LAW)
The system enforces active corporate law via deterministic protocols:
* **COMM:** Outbound communication standardization.
* **MEMO:** Structural logic alignment (Minto Pyramid Principle).
* **LEGAL:** Market claim redaction and fact extraction.
* **TRANSLATE:** Cross-border parity, retaining frozen nomenclature (e.g., "direct-hold solutions").

### EXECUTION DEPTHS
Operators apply compute at specific depths to optimize resource allocation between local edge compute and hyperscaler APIs:
* **[L1] SYNTAX:** Low-compute typographical correction.
* **[L2] STRUCTURE:** Medium-compute logical realignment.
* **[L3] FULL PROTOCOL:** High-compute execution of the Linguistic Domain.

## MULTI-YO-YO DEPLOYMENT & BATCH EXTRACTION

### OPERATIONAL POSTURE
The Doorman supports multiple remote Tier B instances. These are grouped into specialized roles to optimize the cost-to-intelligence ratio.

#### NODE ALPHA: THE TRAINER (L4 SPOT)
- **Deployment:** Continuous Spot instance.
- **Workflow:** SFT / LoRA adapter training from shadow-brief tuples.
- **Maintenance:** Automatically monitored by `mistralrs-idle.timer`. Ensure `SLM_YOYO_TRAINER_ENDPOINT` is configured in the environment.

#### NODE BETA: THE GRAPH EXTRACTOR (H100 DEDICATED)
- **Deployment:** On-demand batch instance.
- **Workflow:** Massive parallel extraction of LadybugDB datagraph relationships.
- **Maintenance:** Spin up manually for archives exceeding 10M tokens. Destroy immediately upon completion of extraction phase to stop high-cost billing. Ensure `SLM_YOYO_GRAPH_ENDPOINT` is active during the batch run.
