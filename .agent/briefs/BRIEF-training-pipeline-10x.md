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

### Inference slot saturation — Batch D, DEFERRED to a dedicated session
Rationale for deferral (2026-06-20): all of these touch the Doorman HOT PATH
(local.rs / http.rs extract+batch handlers / router.rs / slm-core), which was
being deployed concurrently, in a contended monorepo (see session-context:
concurrent sessions + auto-rebase). Multi-crate exhaustive-match surface + the
agent's caveat that the 503 may not even fire (llama.cpp queues internally rather
than 503-ing) make this unsuitable for the tail of a long multi-batch session.
Needs focused work + live-doorman validation.
- **IMMEDIATE mitigation (Command env, routed via outbox 2026-06-20):**
  `CONTENT_DRAIN_THREADS=1` (or `SLM_BATCH_CONCURRENCY=1`) while Tier B down —
  frees an OLMo slot for interactive `/v1/extract`. The `defer:timeout` is the
  Doorman's 120s deadline firing while blocked behind drain, NOT a 503.
- Skip redundant `/v1/extract` Tier-B call in service-content when circuit known
  open (halves Tier A load during outage).
- Detect llama-server 503 → new `DoormanError::TierABusy` fast-fail (local.rs +
  error.rs + http.rs ×2 + slm-core DeferReason + router.rs classify_error).
- Global Tier-A admission semaphore = `--parallel` count (Doorman) — the real fix
  for llama.cpp internal-queue head-of-line blocking.
- **`ExpressLane::decide()` is fully built but never wired** into any HTTP
  handler — the permanent batch-vs-interactive fairness mechanism, currently dead
  code. Wiring it is the end-state.

### Batch execution log (Session 26c cont., 2026-06-20)
- **Batch A** (`8d73757b`, on origin/main): apprenticeship.rs capture quality —
  prefill→`self_confidence: `, stop→`\n```\n`, max_tokens→1536. 193 tests.
- **Batch B** (`c1c1dcc4`+`20b7b295`, on origin/main): verdict.rs canonical-envelope
  DPO pairs (`render_canonical_response`) + corpus_gate chosen-floor & example-echo
  + run-dpo-training.py mirror. 195 tests.
- **Batch C** (`3c6faacf`, local — needs push/Stage 6): service-content
  normalize_entity_key dedup + extraction prompt/schema vector alignment. 44 tests.
  NOTE: needs a live extraction spot-check post-deploy (vectors populate when
  stated, no hallucination).
- **Batch D**: deferred (above).
- Concurrency note: this clone is shared with other sessions + an auto-rebase;
  edits twice wiped before commit. Mitigation: claimed session.lock; commit each
  batch immediately after green tests.

### DataGraph entity quality
- **[DONE — Batch C `3c6faacf`] NULL vectors root cause**: prompt/schema
  contradiction at main.rs:55 fixed — prompt now permits optional vectors when
  explicitly stated; Jennifer few-shot demonstrates role_vector. Live spot-check
  pending post-deploy.
- **[DONE — Batch C `3c6faacf`] Entity resolution**: `normalize_entity_key`
  collapses `Corp.`/`Corp` + whitespace/case variants onto one node id. (No alias
  resolution for `Peter`/`Peter M.` — still a follow-up if needed.)
- **[OPEN]** Context probe `break`s on first matching word (router.rs:225-230) →
  multi-entity prompts get partial grounding. (Doorman change.)
- **[OPEN]** `confidence` field is a tier tag (0.75/0.95), not a confidence; query
  unordered — rename or compute real signal + `ORDER BY`.

## Carry-forward
- Stage 6 for `0506d359` + the SFT script commits (outbox msg-id
  project-intelligence-20260620-session26c-stage6-prompt-fix).
- Re-audit corpus quality after SFT adapter trains on yoyo-batch.
- DataGraph + DPO format fixes are separate sprints; see open list above.

Related: [[project_opus_audit_findings]] [[project_drain_dispatch_fix]]
[[feedback_olmo_only]] [[project_datagraph_enrichment_cascade]]
