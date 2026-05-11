---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-intelligence
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-yo-yo-lora-training-pipeline.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-11
authored_by: command@claude-code (session 2026-05-11)
authored_with: claude-sonnet-4-6
references:
  - service-slm/scripts/nightly-run.sh
  - service-slm/compute/packer/scripts/lora-training.sh
  - service-slm/scripts/corpus-threshold.py
  - service-slm/CLAUDE.md
  - conventions/apprenticeship-substrate.md §12 (CPT Trigger Discipline)
  - conventions/four-tier-slm-substrate.md
notes_for_editor: |
  This TOPIC covers the nightly LoRA adapter training pipeline on Yo-Yo #1.
  The code is complete and committed; the Packer image rebuild and
  lora-training.service activation are the next operator actions (intended).
  Use "intended" / "will be enabled" language for those two steps only.
  The CPT section must remain clearly distinct — no automated CPT, no
  implied timeline other than Q1 2027 first-cut target.
  Cross-reference with topic-jennifer-datagraph-rebuild.md (DataGraph phase
  precedes training phase in the same nightly window).
---

# TOPIC — Yo-Yo #1 Nightly LoRA Training Pipeline

## Overview

Yo-Yo #1 is a g2-standard-4 Google Cloud spot instance equipped with a
single NVIDIA L4 GPU (24 GB VRAM). Each night it runs a two-phase, four-hour
pipeline that produces fine-tuned adapter weights for the workspace language
model. Phase 1 extracts structured business entities from the jennifer data
corpus and writes them to a property graph. Phase 2 reads accumulated
engineering and apprenticeship training tuples, checks whether the corpus
has crossed a minimum threshold, and runs a parameter-efficient training
pass against the base model. The two phases are mandatory and sequential —
they cannot overlap because both require exclusive access to the L4 GPU.

## Why the Phases Are Separate

The L4 GPU serves two incompatible workloads within the nightly window.
During Phase 1, vLLM loads OLMo 3 32B Think (4-bit quantised) to run entity
extraction inference. During Phase 2, the QLoRA training loop loads OLMo 3
7B Think safetensors for gradient computation. A GPU cannot serve an active
vLLM inference process and a PyTorch training loop simultaneously — memory
addresses conflict and context switching between CUDA kernels at this scale
is not supported. `nightly-run.sh` enforces the boundary explicitly:
Phase 1 ends with `stop-yoyo.sh`, which drains the vLLM process and frees
the GPU before Phase 2 begins. Each phase has a configurable budget,
defaulting to 7200 seconds (two hours) each.

## Phase 1 — DataGraph Rebuild

At the start of the nightly window, `start-yoyo.sh` boots the Yo-Yo #1 VM
and waits up to 90 minutes for vLLM to signal readiness. Once the inference
server is live, `jennifer-datagraph-rebuild.sh` processes three document
streams from the jennifer deployment: meeting transcript markdown files,
agent research YAML and markdown files, and contact source JSON records.
For each document, the script calls `POST :9080/v1/chat/completions` through
the Doorman, which routes the payload to the 32B Think model on the Yo-Yo VM.
The model returns a structured JSON array of named entities — people,
companies, projects, accounts, and locations — constrained by a JSON Schema
grammar so the output is machine-parseable without post-processing. The
script then calls `POST :9081/v1/graph/mutate` on service-content to write
those entities into LadybugDB. A local ledger of processed document hashes
ensures each document is processed exactly once across multiple nightly runs.

At the end of Phase 1, vLLM stops and the GPU is released.

## Phase 2 — Adapter Training

`corpus-threshold.py` runs at the start of Phase 2. It counts JSONL tuples
in two corpus buckets — `engineering-pointsav` (SFT tuples drawn from
cross-cluster engineering commits) and `apprenticeship-pointsav` (DPO pairs
drawn from the apprenticeship routing substrate). When either bucket reaches
50 tuples, the script writes a training-pending marker file and, if the
`SLM_YOYO_WEIGHTS_GCS_BUCKET` environment variable is set, syncs the
relevant corpus directory to the configured GCS bucket.

On the Yo-Yo VM, `lora-training.sh` polls the training-pending directory
every 30 seconds. When a marker appears, it claims the marker with an atomic
rename (appending `.claimed`), pulls the corpus from GCS, and runs QLoRA
using the peft, bitsandbytes, and trl libraries.

## What QLoRA Is

QLoRA (Quantised Low-Rank Adaptation) is a parameter-efficient fine-tuning
method that loads a base model in 4-bit NF4 quantisation and trains a small
set of additional weight matrices — called an adapter — rather than updating
the full model. For a 7B-parameter model like OLMo 3 7B Think, 4-bit
quantisation reduces the GPU footprint from roughly 14 GB (in bfloat16) to
approximately 6 GB, leaving adequate headroom on the 24 GB L4 for the
training loop itself. The adapter targets seven linear projection layers:
`q_proj`, `v_proj`, `k_proj`, `o_proj`, `gate_proj`, `up_proj`, and
`down_proj`. Training runs for two epochs with rank 16 (`r=16`), alpha 32
(`lora_alpha=32`), a maximum sequence length of 512 tokens, and gradient
checkpointing enabled to manage activation memory.

The training configuration is intentionally conservative. The goal is to
shift the base model toward the vocabulary, formatting patterns, and
structural conventions that appear in the engineering and apprenticeship
corpora — not to retrain the model on a general task. Two epochs over
hundreds of tuples is sufficient for this narrow shift.

## The Two Corpus Streams

**Engineering tuples** are SFT (supervised fine-tuning) pairs drawn from
actual commit diffs, commit messages, and review briefs across all clusters
in the workspace. They teach the model the precise technical vocabulary and
structural patterns used in the engineering workflow: how diffs are described,
how review comments are phrased, and how implementation decisions are
documented.

**Apprenticeship tuples** are DPO (direct preference optimisation) pairs
produced by the apprenticeship routing substrate. Each pair consists of a
shadow response (the model's unguided output) and a verdict response (the
preferred formulation confirmed by the operator). DPO training on these pairs
moves the model toward the preferred response distribution without requiring
explicit labels for every token.

## Adapter Output and Publication

When training completes, the adapter is saved to
`/data/weights/adapters/<tenant>/<role>/v<N>/` on the Yo-Yo VM. The adapter
directory contains the LoRA weight files and tokenizer configuration — total
size is typically 1 to 3 GB. `lora-training.sh` then signals
`adapter-publish.service`, which uploads the adapter directory to the
configured GCS bucket. The adapter is subsequently available to the workspace
Doorman for loading as an inference-time weight overlay on the base model.
The marker file is renamed to `.completed` when all steps succeed.

## Adapter Training Versus Continued Pre-Training

The nightly LoRA process is adapter training. It produces a weight delta —
a few gigabytes of parameters — that the base model loads at inference time.
It runs in approximately two hours on a single L4 GPU and operates over
hundreds to low thousands of training tuples. The base model itself is not
modified.

Continued pre-training (CPT) is a distinct operation at a fundamentally
different scale. CPT would produce a new base model checkpoint by training
on 50 to 100 billion tokens across 8 to 32 H100-class GPUs for one to four
weeks. The cost per CPT cycle runs to tens of thousands of dollars. CPT is
operator-triggered, never automated, and never scheduled as part of the
nightly pipeline. The first-cut CPT target is Q1 2027, contingent on
corpus volume and operator decision. Until that decision is made, all
nightly training is adapter-only.

## Current Status

The nightly pipeline code is complete. The workspace language model service
passes 177 of 177 tests. The Packer image rebuild that bakes the training
Python stack (peft, bitsandbytes, trl) into the Yo-Yo VM is the next
intended operator action. Once that image is deployed, `lora-training.service`
on the Yo-Yo VM will be enabled with `systemctl enable --now lora-training.service`.
Until the image is rebuilt, the training phase runs in marker-only mode:
`corpus-threshold.py` writes and dispatches the GCS marker, but
`lora-training.sh` is not yet active on the runtime VM image.
