# 🧠 TOOL-COGNITIVE-FORGE
**Vendor:** PointSav Digital Systems™
**Standard:** Knowledge Distillation
**Tier:** 3 (Operational Tool)

## I. ARCHITECTURAL MANDATE
This tool executes locally on the Tier-1 Foundry (iMac). It is a manual script used to train microscopic models (0.5B parameters) to mimic massive models (7B parameters). 

## II. EXECUTION MECHANICS
1. **The Teacher:** Boots a heavy, MIT-licensed model (e.g., Phi-4-Mini) locally. 
2. **The Ingestion:** Feeds the Teacher thousands of raw emails from the Cloud Spool and demands perfect JSON routing outputs.
3. **The Ledger:** Writes these perfect inputs/outputs to `training_dataset.jsonl`.
4. **The Lobotomy:** Uses this dataset to run a LoRA fine-tuning sequence against a 0.5B model, stripping it of conversational noise and forcing it into a purely deterministic routing engine for the $5/month Cloud Shield.
