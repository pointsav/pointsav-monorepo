---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Yo-Yo Daily Enrichment Cycle"
slug: topic-yoyo-daily-enrichment-cycle
language: en
status: draft
paired_with: TOPIC-yoyo-daily-enrichment-cycle.es.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-yoyo-daily-enrichment-cycle.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
research_provenance: "BRIEF-slm-learning-loop.md §11 §13 §14; bin/yoyo-daily-cycle.sh; cycle logs at /srv/foundry/data/yoyo-cycle-logs/"
research_inline: true
created: 2026-06-09
author: totebox@project-intelligence (claude-sonnet-4-6)
---

# Yo-Yo Daily Enrichment Cycle

The Yo-Yo daily enrichment cycle is the automated batch window that runs a GPU-accelerated
inference VM once per day to enrich the DataGraph and accumulate training data for the
local language model. The cycle runs at a fixed time, enforces a hard cost cap, and
terminates the VM whether the work finishes early or reaches the cap.

## Purpose

The workspace VM runs a 7-billion-parameter language model (OLMo 2 7B) on CPU for
interactive use. This model performs adequately for short prompts but extracts entities
from documents with lower accuracy than a larger GPU-resident model. The daily cycle
addresses this gap by starting a separate GPU VM — the Yo-Yo batch node — that loads
a 32-billion-parameter model and processes a queue of documents that accumulated during
the day.

The products of each cycle are:
- Additional named entities added to the DataGraph (LadybugDB graph store)
- Direct Preference Optimisation (DPO) training pairs written to the enrichment corpus

Each DPO pair records what the 32B model extracted as the preferred output and what the
7B model extracted as the baseline, enabling the 7B model to be fine-tuned toward the
larger model's extraction quality over successive training runs.

## The eight phases

The cycle is a single Bash script (`yoyo-daily-cycle.sh`) that executes eight sequential
phases. The script writes a timestamped log file for each run.

**Phase 1 — VM start.** If the batch VM is not already running, a `gcloud instances start`
command is issued. The VM boots from a persistent disk that retains the model weights and
the inference server configuration from the previous cycle.

**Phase 2 — Inference server health.** The script polls the llama-server health endpoint
(`/health`) at ten-second intervals until it returns `{"status":"ok"}`. Startup consistently
takes approximately 170 seconds from power-on to first healthy response. If the server
does not respond within ten minutes, the cycle aborts and stops the VM.

**Phase 3 — Tier B circuit.** The Doorman (the local inference gateway) maintains a circuit
breaker for the Yo-Yo node. The script waits up to two minutes for the circuit to close,
confirming the Doorman has registered the VM as reachable. If the circuit does not close,
the cycle continues with a Tier A fallback warning logged.

**Phase 4 — Enrichment drain.** For 40 percent of the cycle budget (18 minutes at the
45-minute cap), the script waits while the Doorman processes the pending enrichment queue.
During this window, the service-content process sends document chunks to the Yo-Yo node
for entity extraction and writes DPO pairs to the enrichment corpus. Progress is logged
every 60 seconds with entity counts, enrichment pair counts, GPU utilisation, and VRAM
usage.

**Phase 5 — Corpus threshold check.** After enrichment, `corpus-threshold.py` runs to
count accumulated training-ready data. If counts exceed the configured threshold, the
script writes dated training marker files to `data/training-pending/`. These markers
are the input to Phase 6.

**Phase 6 — LoRA training trigger.** Three gates must all pass for training to run:
training markers must be present, the ML libraries must be installed in the training
virtual environment on the batch VM, and an operator-authored approval tag must exist
for the current date. If all three pass, the script stops the inference server to free
approximately 16 gigabytes of VRAM, then invokes `run-dpo-training.py` over SSH with a
45-percent budget (20 minutes at the 45-minute cap). The `--resume` flag accumulates
daily checkpoints so each run extends the previous day's training rather than starting
from scratch.

**Phase 7 — GCS sync.** If the `SLM_YOYO_WEIGHTS_GCS_BUCKET` environment variable is
set and training markers are present, the enrichment corpus is synchronised to the
configured Cloud Storage bucket. This step is currently disabled pending a future session
that configures the bucket.

**Phase 8 — Hard stop.** The inference server is stopped via SSH, the VM is stopped via
`gcloud instances stop`, and the script waits up to three minutes for the VM to reach
`TERMINATED` status. A summary line records total elapsed time, entity delta, DPO pair
delta, and VM final status.

## Budget and cost

The daily cycle operates under a 45-minute hard cap. The VM is stopped unconditionally
at the end of Phase 8 regardless of whether phases completed normally.

| Item | Value |
|---|---|
| VM type | g2-standard-4 with NVIDIA L4 24 GB |
| Zone | us-central1-a |
| Running cost | approximately $0.71 per hour |
| Cycle cost at 45-minute cap | approximately $0.53 per cycle |
| TERMINATED cost | $0.00 |
| Monthly cost (daily cycles) | approximately $16 per month |

A kill switch file (`/srv/foundry/data/yoyo-disabled`) suppresses all VM lifecycle
operations immediately. Creating the file prevents Phase 1 from issuing a start command.
Removing the file resumes normal operation on the next scheduled cycle.

An idle monitor timer checks every five minutes whether the VM has been running idle for
more than 30 minutes. If the daily cycle fails to stop the VM, the idle monitor will stop
it as a safety backstop, preventing uncapped cost accumulation.

## DPO pair format

Each enrichment DPO pair is a JSON file written to the feedback directory. The format
is compatible with the TRL DPOTrainer:

```json
{
  "prompt":      "<document chunk text>",
  "chosen":      "[{\"classification\":\"Person\",\"entity_name\":\"...\"}]",
  "rejected":    "[{\"classification\":\"Person\",\"entity_name\":\"...\"}]",
  "source_type": "datagraph-enrichment",
  "worm_id":     "<document identifier>",
  "timestamp":   "<ISO 8601>"
}
```

`chosen` is the 32B model's extraction. `rejected` is the 7B model's extraction. A pair
is only written when both models found at least one entity and the results differ after
normalisation. Pairs where the 7B model found nothing are discarded — they contain no
genuine preference signal.

## Verified test results (2026-06-09)

Three 10-minute test cycles confirmed the pipeline operates correctly end-to-end.

| Cycle | Duration | Entity delta | DPO pairs added | VM final status |
|---|---|---|---|---|
| 1 | 10 min 43 s | +7 | +6 | TERMINATED |
| 2 | 9 min 12 s | +8 | +4 | TERMINATED |
| 3 | 10 min 38 s | +22 | +8 | TERMINATED |

GPU diagnostics in cycle 3: 99% utilisation, 16,151 of 23,034 MB VRAM in use, 73°C.

## Open question

The LoRA target module names in `run-dpo-training.py` use LLaMA-style names
(`q_proj`, `gate_proj`, etc.). OLMo-2 may use different internal names. This must be
verified against the loaded model before the first training run. A silent mismatch would
cause LoRA to train zero parameters.
