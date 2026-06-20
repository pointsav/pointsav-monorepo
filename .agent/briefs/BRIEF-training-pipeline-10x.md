---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-training-pipeline-10x
title: Training + DataGraph 10x — SFT-first pivot and pipeline repairs
status: active
owner: project-intelligence
created: 2026-06-19
updated: 2026-06-19
---

# Training Pipeline 10x — SFT-first pivot

## Context

Operator goal: "10x the performance and quality of the training and datagraph."
Session 26c ran a four-agent adversarial Opus audit (one lens each: prompt/DPO
corpus, inference slot saturation, DataGraph entity quality, training
architecture). Two independent agents converged on the same root cause.

## Root cause (two independent agents converged)

The DPO training task is **unlearnable as framed.** Live corpus
(`/srv/foundry/data/corpus/dpo/2026-06-19.jsonl`, 1,021 pairs, 100% git-commit):

| field | contains | median |
|---|---|---|
| `prompt` | bare commit subject; **no file context**; empty acceptance test | 131 chars |
| `chosen` | the **entire multi-file repo diff** | 4,025 chars |
| `rejected` | OLMo's stripped fragment | 220 chars |

Mean chosen/rejected ratio **93x**; only ~168 pairs survive all training filters.
Asking OLMo 7B to reproduce a 4KB multi-file diff from a one-line message with
zero file context is impossible. Worse, `chosen`/`rejected` live in different
output spaces (chosen = raw `diff --git`, rejected = stripped hunk), so DPO
learns "emit `diff --git` and be long" — surface form, not correctness.

## Decisions locked

1. **SFT-first pivot** (operator-approved 2026-06-19, AskUserQuestion "full
   pivot"). Teach format + edit skill with SFT on the gold senior diffs before
   any preference optimisation. On-policy DPO/SimPO comes later, once the model
   emits valid diffs >90% of the time.
2. **Per-file split.** One training pair per file (split multi-file `actual_diff`
   on `diff --git` boundaries) collapses the 93x ratio to ~1-3x and turns 1,021
   commits into thousands of tractable targets.
3. **Canonical envelope both sides.** Completions are wrapped in the exact
   inference format (YAML frontmatter + `## Reasoning` + fenced `## Diff`).
4. **OLMo-only** retained (see [[feedback_olmo_only]]).

## Work log

### 2026-06-19 (Session 26c) — DONE
- `fix(apprenticeship)` commit `0506d359`: removed redundant "## Required
  response shape" block from `apprentice_prompt()` (user prompt). Was causing
  OLMo to echo `<unified diff, OR empty if escalate=true>` literally (55% of DPO
  rejected fields). System prompt already shows the format.
- **`run-dpo-training.py`**: added `--mode sft` (SFTTrainer, LR 2e-5, 2 epochs,
  `SFT_SYSTEM_PROMPT` system turn, `load_sft_files()`); added `run_sft_training()`.
  Fixed the `_max_length=512` truncation bug (silently chopped every diff >512
  tokens mid-hunk) → per-mode: SFT 2048, SimPO 7B 1024, DPO 7B 512, 32B 1024.
- **`export-sft.py`** (new): builds SFT corpus from shadow tuples' gold
  `actual_diff` — per-file split + canonical envelope, `--task-type git-commit`
  default (other shadow types carry capture-boilerplate bodies). Validated,
  git-commit only: **2,228 clean SFT records from 1,299 tuples** (vs 168 trainable
  DPO pairs = 13x), each a tractable single-file task with a real commit-message
  prompt in the on-policy envelope; 0 malformed, all within the 2048-token window.
  Written to `data/corpus/sft/sft-2026-06-20.jsonl`.
- **`2f9bd612`**: size guard `_MAX_SEGMENT_CHARS=8000` drops 425 generated/vendored
  diffs (Cargo.lock, minified) that would truncate past SFT max_length.

## Decisions open / follow-up

### Training (this archive — Totebox)
- **True file-grounded prompts (capture-path change).** Shadow tuples never
  captured pre-edit file contents (`brief.scope.files` empty; `brief_id` is NOT
  the git SHA). The git post-commit hook must record the commit SHA + pre-edit
  blobs (`git show <sha>^:<path>`) so SFT/DPO prompts contain real file context.
  Until then prompts carry commit-subject + path only. Rust/hook change.
- **DPO format fix** (for the later preference phase): reconstruct BOTH `chosen`
  and `rejected` in the canonical envelope in `verdict.rs::write_dpo_pair`;
  add a `chosen` length floor in `corpus_gate.rs` (empty-chosen inverted pairs);
  fix stop sequence `"```\n\n"` → `"\n```\n"` in apprenticeship.rs (rarely fires);
  prefill `"---\nself_confidence: "` instead of `"---\n"`; raise Tier A
  max_tokens 1024 → 1536; change the system-prompt diff EXAMPLE to unmistakable
  placeholders so verbatim echoes are detectable (TEMPLATE_ECHO can't catch
  `path/to/file`/`old line` echoes today).
- **lora-update.sh / nightly**: add an SFT stage before the preference stage;
  gate preference on "SFT adapter exists + emits valid diffs".

### Inference slot saturation (mix of Totebox + Command)
- **`CONTENT_DRAIN_THREADS=1`** when Tier B down — frees an OLMo slot for
  interactive `/v1/extract` (Command: service-content env). The `defer:timeout`
  is the Doorman's 120s deadline firing while blocked behind drain, NOT a 503.
- Skip redundant `/v1/extract` Tier-B call in service-content when circuit known
  open (halves Tier A load during outage).
- Detect llama-server 503 → new `DoormanError::TierABusy` fast-fail (local.rs).
- Global Tier-A admission semaphore = `--parallel` count (Doorman).
- **`ExpressLane::decide()` is fully built but never wired** into any HTTP
  handler — the fairness logic that would fix this permanently is dead code.

### DataGraph entity quality (Command + Totebox)
- **NULL vectors root cause**: `service-content/src/main.rs:55` extraction prompt
  says "exactly two fields" while the schema (main.rs:869-885) declares five
  (incl. the 3 vectors). The prompt actively forbids what the schema asks. Fix:
  add vectors to prompt + 2-3 few-shot examples, OR delete them from schema.
- **No entity resolution**: `Woodfine Management Corp.`/`Corp`, `Peter`/`Peter M.`
  split into distinct nodes (graph.rs:120-124). Add normalization before id
  construction. Inflates the 11,873 count with duplicates.
- Context probe `break`s on first matching word (router.rs:225-230) → multi-entity
  prompts get partial grounding.
- `confidence` field is a tier tag (0.75/0.95), not a confidence; query unordered.

## Carry-forward
- Stage 6 for `0506d359` + the SFT script commits (outbox msg-id
  project-intelligence-20260620-session26c-stage6-prompt-fix).
- Re-audit corpus quality after SFT adapter trains on yoyo-batch.
- DataGraph + DPO format fixes are separate sprints; see open list above.

Related: [[project_opus_audit_findings]] [[project_drain_dispatch_fix]]
[[feedback_olmo_only]] [[project_datagraph_enrichment_cascade]]
