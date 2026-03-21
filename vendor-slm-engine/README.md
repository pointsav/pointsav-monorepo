# Vendor SLM Engine (PointSav Digital Systems)

This repository holds the Single Source of Truth for all Small Language Model (SLM) configurations used within the Woodfine/PointSav ecosystem.

## Asset Ledger
* **pointsav-nano (135M):** The core Semantic Router for edge-node data extraction. Built on Apache 2.0 SmolLM2.

## Operational Deployment
The physical weights are NOT stored in this repository. To instantiate the model on a Tier-2 or Tier-3 node, execute:
`ollama create pointsav-nano -f Modelfile.pointsav-nano`

© 2026 PointSav Digital Systems
