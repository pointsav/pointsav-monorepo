---
schema: foundry-draft-v1
state: draft-pending
originating_cluster: project-proofreader
target_repo: content-wiki-documentation
target_path: ./
target_filename: guide-proofreader-distillation.md
audience: operator
bcsc_class: operational
language_protocol: prose-guide
authored: 2026-05-04
authored_by: task-project-proofreader
---

# GUIDE: Executing Proofreader Training Distillation

This guide outlines the operational steps required to distill the Proofreader's apprenticeship corpus into a training dataset for the SLM.

## Prerequisites
- The `app-console-proofreader` has been accumulating events in the `apprenticeship/prose-edit` JSONL corpus.
- The `service-slm` teacher-student distillation environment (`ignite_teacher.sh`) is prepared to ingest a `training_dataset.jsonl`.

## Running the Distillation Tool
The pure-Rust distillation tool processes the raw JSONL events to extract verified input/output pairs based on operator verdicts (accepted/edited).

1. Navigate to the `pointsav-monorepo` root.
2. Execute the distillation target:
   ```bash
   cargo run --bin tool-proofreader-trainer -- --corpus-dir /srv/foundry/data/training-corpus/apprenticeship/prose-edit --output ../datasets/training_dataset.jsonl
   ```
3. Verify the output dataset contains valid JSONL pairs with `instruction` and `output` keys.

## Triggering the SLM Training
Once the dataset is distilled:
1. Navigate to `service-slm/router-trainer/scripts/`.
2. Ensure the teacher model is inactive if you are only running the student fine-tuning, or integrate the dataset into the broader knowledge distillation run.
3. The generated pairs are now ready for the standard LoRA fine-tuning sequence to update the per-tenant adapter.
