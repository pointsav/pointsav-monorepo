#!/usr/bin/env python3
"""
run-dpo-training.py — LoRA DPO fine-tuning for the apprenticeship adapter.

Reads DPO feedback pairs from a local directory (or GCS-synced path),
fine-tunes OLMo 7B Instruct with LoRA using HuggingFace TRL DPOTrainer,
and saves the adapter to the output path (optionally uploading to GCS).

Requirements (on trainer VM):
  pip install trl>=0.8 peft>=0.10 transformers>=4.40 datasets bitsandbytes

Usage:
  python3 run-dpo-training.py --corpus /path/to/feedback/ --base-model allenai/OLMo-2-1124-7B-Instruct
  python3 run-dpo-training.py --dry-run   # inspect corpus without training

GCS variant (Yo-Yo trainer VM, picking up marker):
  SLM_YOYO_WEIGHTS_GCS_BUCKET=woodfine-node-gcp-free-foundry-substrate \\
    python3 run-dpo-training.py --from-gcs --adapter apprenticeship-pointsav

Notes:
  - adapter output goes to ./adapters/<adapter_name>-<date>/
  - GCS upload: gs://<bucket>/adapters/<adapter_name>-<date>/
  - OLMo-only policy: never substitute a non-OLMo base model
"""

import argparse
import glob
import json
import os
import subprocess
import sys
import time

from pathlib import Path


FOUNDRY_ROOT = os.environ.get("FOUNDRY_ROOT", "/srv/foundry")
GCS_BUCKET = os.environ.get("SLM_YOYO_WEIGHTS_GCS_BUCKET", "")

# LoRA hyperparameters for OLMo 7B — conservative defaults
LORA_R = 16
LORA_ALPHA = 32
LORA_DROPOUT = 0.05
LORA_TARGET_MODULES = ["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"]
MAX_PROMPT_LENGTH = 512
MAX_LENGTH = 1024
BATCH_SIZE = 2
GRAD_ACCUM = 4
LEARNING_RATE = 1e-5   # LoRA-DPO: higher than full-FT DPO (1e-7..1e-6); 47 steps needs signal
NUM_EPOCHS = 1
BETA = 0.1  # DPO default. Prior 0.5 justification (empty-"[]" rejected) is obsolete — those pairs are now filtered.


def load_feedback_files(corpus_path: str) -> list[dict]:
    """Load DPO pairs from corpus_path.

    Reads all pair files:
    - apprenticeship-*.jsonl: git-commit shadow captures (chosen=operator diff, rejected=OLMo diff)
    - enrichment-*.jsonl: DataGraph disagreement pairs (chosen=Tier B, rejected=Tier A)

    Filters applied:
    - Skips pairs where rejected is empty ("[]") — length-bias degenerate pairs.
    - Skips pairs where auto_verdict=False (explicitly rejected by operator or verdict pipeline).
      Pairs with no auto_verdict field (most) are accepted.
    """
    files = []
    for pat in ["apprenticeship-*.jsonl", "enrichment-*.jsonl"]:
        files.extend(glob.glob(os.path.join(corpus_path, pat)))
    files = sorted(set(files))
    print(f"[corpus] found {len(files)} pair files in {corpus_path}")
    records = []
    skipped = 0
    skipped_empty_rejected = 0
    skipped_verdict = 0
    for f in files:
        try:
            d = json.load(open(f))
        except Exception as e:
            print(f"[WARN] skip {f}: {e}", file=sys.stderr)
            skipped += 1
            continue
        if not d.get("prompt") or not d.get("chosen"):
            skipped += 1
            continue
        rejected = d.get("rejected", "")
        # Skip degenerate pairs where Tier A returned nothing — not genuine preference signal
        if not rejected or rejected == "[]":
            skipped_empty_rejected += 1
            continue
        # Skip pairs explicitly rejected by verdict pipeline
        verdict = d.get("auto_verdict")
        if verdict is not None and verdict is not True:
            skipped_verdict += 1
            continue
        # Conversational format: TRL applies OLMo-2 chat template correctly, avoiding
        # the "Mismatch between tokenized prompt and start of tokenized prompt+chosen"
        # warnings that fire with raw-string format (EOS token handling in standalone vs
        # concatenated tokenization differs, breaking DPO loss boundary detection).
        records.append({
            "prompt":   [{"role": "user",      "content": d["prompt"]}],
            "chosen":   [{"role": "assistant", "content": d["chosen"]}],
            "rejected": [{"role": "assistant", "content": rejected}],
        })
    print(f"[corpus] loaded {len(records)} DPO pairs ({skipped} format-skipped, {skipped_empty_rejected} empty-rejected filtered, {skipped_verdict} verdict-rejected filtered)")
    return records


def sync_from_gcs(adapter_name: str, local_corpus: str) -> str:
    """Sync apprenticeship corpus from GCS to local path. Returns local corpus path."""
    if not GCS_BUCKET:
        print("[ERROR] SLM_YOYO_WEIGHTS_GCS_BUCKET not set", file=sys.stderr)
        sys.exit(1)
    feedback_path = os.path.join(local_corpus, "feedback")
    os.makedirs(feedback_path, exist_ok=True)
    print(f"[gcs] syncing corpus from gs://{GCS_BUCKET}/training-corpus/apprenticeship/ ...")
    subprocess.run(
        ["gcloud", "storage", "cp", "-r",
         f"gs://{GCS_BUCKET}/training-corpus/apprenticeship/",
         os.path.join(local_corpus, "apprenticeship/")],
        check=True,
    )
    # Feedback pairs are under apprenticeship/../feedback/ on GCS
    subprocess.run(
        ["gcloud", "storage", "cp", "-r",
         f"gs://{GCS_BUCKET}/training-corpus/feedback/",
         feedback_path + "/"],
        check=True,
    )
    return feedback_path


def upload_adapter_to_gcs(adapter_path: str, adapter_name: str) -> None:
    """Upload adapter to GCS.

    yoyo-batch VMs do not have GCS write permissions — only the workspace VM
    holds ADC with cloud-platform scope. The startup script on yoyo-batch
    must rsync the adapter to the workspace VM which then runs this upload,
    OR the workspace pulls the adapter via rsync and uploads directly.

    When run directly on the workspace VM (e.g. during testing), this works
    without any special setup.
    """
    if not GCS_BUCKET:
        return
    gcs_dest = f"gs://{GCS_BUCKET}/adapters/{os.path.basename(adapter_path)}/"
    print(f"[gcs] uploading adapter → {gcs_dest}")
    try:
        subprocess.run(
            ["gcloud", "storage", "rsync", adapter_path + "/", gcs_dest],
            check=True,
        )
        print(f"[gcs] adapter uploaded: {gcs_dest}")
    except subprocess.CalledProcessError as e:
        print(f"[gcs] upload failed (likely no ADC on this VM): {e}")
        print(f"[gcs] adapter saved locally at: {adapter_path}")
        print(f"[gcs] pull from workspace: rsync -a yoyo-batch:{adapter_path}/ /tmp/adapter/ then upload")


def run_training(records: list[dict], base_model: str, output_dir: str, dry_run: bool,
                 max_runtime_seconds: int = 0, resume: bool = False) -> None:
    """Fine-tune base_model with DPO on records, save adapter to output_dir."""
    print(f"[train] base model: {base_model}")
    print(f"[train] output dir: {output_dir}")
    print(f"[train] DPO pairs:  {len(records)}")
    print(f"[train] LoRA r={LORA_R} alpha={LORA_ALPHA} beta={BETA}")
    if max_runtime_seconds:
        print(f"[train] runtime cap: {max_runtime_seconds}s ({max_runtime_seconds // 3600}h {(max_runtime_seconds % 3600) // 60}m)")
    if resume:
        import glob as _glob
        checkpoints = sorted(_glob.glob(os.path.join(output_dir, "checkpoint-*")))
        if checkpoints:
            print(f"[train] resuming from checkpoint: {checkpoints[-1]}")
        else:
            print(f"[train] --resume set but no checkpoint found in {output_dir} — starting fresh")

    if dry_run:
        print("[train] DRY-RUN — skipping actual training")
        return

    # Import training libraries (only needed at training time)
    try:
        import torch
        from datasets import Dataset
        from peft import LoraConfig, TaskType, get_peft_model
        from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig, TrainerCallback
        from trl import DPOConfig, DPOTrainer
    except ImportError as e:
        print(f"[ERROR] Missing training library: {e}", file=sys.stderr)
        print("Install: pip install trl peft transformers datasets bitsandbytes", file=sys.stderr)
        sys.exit(1)

    os.makedirs(output_dir, exist_ok=True)

    print(f"[train] CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"[train] GPU: {torch.cuda.get_device_name(0)}")

    # 4-bit quantization for memory efficiency
    bnb_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype=torch.bfloat16,
        bnb_4bit_use_double_quant=True,
    )

    print("[train] loading tokenizer ...")
    tokenizer = AutoTokenizer.from_pretrained(base_model, trust_remote_code=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    tokenizer.padding_side = "right"  # prevents causal mask misalignment with OLMo-2 DPO batches

    print("[train] loading model (4-bit) ...")
    model = AutoModelForCausalLM.from_pretrained(
        base_model,
        quantization_config=bnb_config,
        device_map="auto",
        trust_remote_code=True,
    )
    model.config.use_cache = False

    peft_config = LoraConfig(
        task_type=TaskType.CAUSAL_LM,
        r=LORA_R,
        lora_alpha=LORA_ALPHA,
        lora_dropout=LORA_DROPOUT,
        target_modules=LORA_TARGET_MODULES,
        bias="none",
    )

    # Runtime cap callback — saves checkpoint and stops training cleanly if wall-clock limit hit
    class RuntimeCapCallback(TrainerCallback):
        def __init__(self, max_seconds: int, output_dir: str) -> None:
            self._start = time.monotonic()
            self._max = max_seconds
            self._out = output_dir

        def on_step_end(self, args, state, control, **kwargs):
            if self._max and (time.monotonic() - self._start) >= self._max:
                elapsed = int(time.monotonic() - self._start)
                print(f"[train] runtime cap reached ({elapsed}s >= {self._max}s) — saving checkpoint and stopping")
                control.should_save = True
                control.should_training_stop = True

    dataset = Dataset.from_list(records)
    split = dataset.train_test_split(test_size=0.1, seed=42)

    # 32B memory guardrails: gradient_checkpointing + smaller batch + shorter sequences
    is_32b = "32B" in base_model or "32b" in base_model
    _batch_size = 1 if is_32b else BATCH_SIZE
    _grad_accum = 4 if is_32b else GRAD_ACCUM
    _max_length = 512 if is_32b else MAX_LENGTH
    if is_32b:
        print(f"[train] 32B memory mode: batch=1, grad_ckpt=True, max_len={_max_length}, grad_accum={_grad_accum}")

    training_args = DPOConfig(
        output_dir=output_dir,
        num_train_epochs=NUM_EPOCHS,
        per_device_train_batch_size=_batch_size,
        gradient_accumulation_steps=_grad_accum,
        gradient_checkpointing=is_32b,
        learning_rate=LEARNING_RATE,
        beta=BETA,
        max_length=_max_length,
        logging_steps=5,
        save_steps=5,        # checkpoint every 5 steps (corpus is small; 50 was never reached in 1 epoch)
        save_total_limit=2,  # keep only 2 most recent; avoids disk fill on spot VM across days
        eval_strategy="no",           # eval needs 2× VRAM (ref+trained); disabled on L4 24 GB
        report_to="none",
        bf16=torch.cuda.is_available(),
        remove_unused_columns=False,
    )

    # expandable_segments avoids fragmentation-caused OOM on CUDA
    os.environ.setdefault("PYTORCH_CUDA_ALLOC_CONF", "expandable_segments:True")

    callbacks = []
    if max_runtime_seconds:
        callbacks.append(RuntimeCapCallback(max_runtime_seconds, output_dir))

    trainer = DPOTrainer(
        model=model,
        ref_model=None,  # uses implicit reference (PEFT base model)
        args=training_args,
        train_dataset=split["train"],
        processing_class=tokenizer,
        peft_config=peft_config,
        callbacks=callbacks or None,
    )

    print(f"[train] starting DPO training on {len(split['train'])} pairs ...")
    resume_ckpt = None
    if resume:
        checkpoints = sorted(glob.glob(os.path.join(output_dir, "checkpoint-*")))
        if checkpoints:
            resume_ckpt = checkpoints[-1]
            print(f"[train] resuming from checkpoint: {resume_ckpt}")
        else:
            print(f"[train] no checkpoint in {output_dir} — starting fresh")
    trainer.train(resume_from_checkpoint=resume_ckpt)

    print(f"[train] saving adapter to {output_dir}")
    trainer.save_model(output_dir)
    tokenizer.save_pretrained(output_dir)
    print("[train] done")


def main() -> None:
    parser = argparse.ArgumentParser(description="LoRA DPO training for apprenticeship adapter")
    parser.add_argument("--corpus", default=os.path.join(FOUNDRY_ROOT, "data", "training-corpus", "feedback"),
                        help="Path to feedback/ directory containing apprenticeship-*.jsonl files")
    parser.add_argument("--base-model", default="allenai/OLMo-2-1124-7B-Instruct",
                        help="HuggingFace model ID for the OLMo base model (OLMo-only policy)")
    parser.add_argument("--adapter-name", default="apprenticeship-pointsav",
                        help="Name for the output adapter")
    parser.add_argument("--output-dir", default=None,
                        help="Override output directory (default: ./adapters/<name>-<date>)")
    parser.add_argument("--from-gcs", action="store_true",
                        help="Sync corpus from GCS before training")
    parser.add_argument("--upload-gcs", action="store_true",
                        help="Upload trained adapter to GCS after training")
    parser.add_argument("--dry-run", action="store_true",
                        help="Load corpus and report counts without training")
    parser.add_argument("--max-runtime-seconds", type=int, default=7200,
                        help="Wall-clock training limit in seconds (default: 7200 = 2h). "
                             "Saves checkpoint and exits cleanly when reached. 0 = no cap.")
    parser.add_argument("--resume", action="store_true",
                        help="Resume training from the latest checkpoint in output_dir. "
                             "Pass on every daily run to accumulate training incrementally.")
    args = parser.parse_args()

    corpus_path = args.corpus
    if args.from_gcs:
        local_staging = "/tmp/foundry-training-corpus"
        corpus_path = sync_from_gcs(args.adapter_name, local_staging)

    records = load_feedback_files(corpus_path)
    if not records:
        print("[ERROR] No valid DPO pairs found — check corpus path and field names", file=sys.stderr)
        sys.exit(1)

    # Use a fixed -wip suffix so --resume finds the same checkpoint directory each day.
    # Only rename to a dated path when promoting the adapter to the registry.
    output_dir = args.output_dir or f"./adapters/{args.adapter_name}-wip"

    run_training(records, args.base_model, output_dir, dry_run=args.dry_run,
                 max_runtime_seconds=args.max_runtime_seconds,
                 resume=args.resume)

    if args.upload_gcs and not args.dry_run:
        upload_adapter_to_gcs(output_dir, args.adapter_name)

    if not args.dry_run:
        print(f"\n[done] adapter at: {output_dir}")
        if GCS_BUCKET and args.upload_gcs:
            print(f"[done] GCS: gs://{GCS_BUCKET}/adapters/{os.path.basename(output_dir)}/")


if __name__ == "__main__":
    main()
