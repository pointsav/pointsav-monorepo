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
from datetime import datetime, timezone
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
LEARNING_RATE = 1e-4
NUM_EPOCHS = 1
BETA = 0.1  # DPO beta — higher = closer to reference model


def load_feedback_files(corpus_path: str) -> list[dict]:
    """Load all apprenticeship DPO feedback pairs from corpus_path."""
    pattern = os.path.join(corpus_path, "apprenticeship-*.jsonl")
    files = sorted(glob.glob(pattern))
    records = []
    skipped = 0
    for f in files:
        try:
            d = json.load(open(f))
        except Exception as e:
            print(f"[WARN] skip {f}: {e}", file=sys.stderr)
            skipped += 1
            continue
        # Require TRL fields
        if not d.get("prompt") or not d.get("chosen") or not d.get("rejected"):
            skipped += 1
            continue
        records.append({
            "prompt": d["prompt"],
            "chosen": d["chosen"],
            "rejected": d["rejected"],
        })
    print(f"[corpus] loaded {len(records)} DPO pairs ({skipped} skipped)")
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
    if not GCS_BUCKET:
        return
    gcs_dest = f"gs://{GCS_BUCKET}/adapters/{os.path.basename(adapter_path)}/"
    print(f"[gcs] uploading adapter → {gcs_dest}")
    subprocess.run(
        ["gcloud", "storage", "cp", "-r", adapter_path + "/", gcs_dest],
        check=True,
    )
    print(f"[gcs] adapter uploaded: {gcs_dest}")


def run_training(records: list[dict], base_model: str, output_dir: str, dry_run: bool,
                 max_runtime_seconds: int = 0) -> None:
    """Fine-tune base_model with DPO on records, save adapter to output_dir."""
    print(f"[train] base model: {base_model}")
    print(f"[train] output dir: {output_dir}")
    print(f"[train] DPO pairs:  {len(records)}")
    print(f"[train] LoRA r={LORA_R} alpha={LORA_ALPHA} beta={BETA}")
    if max_runtime_seconds:
        print(f"[train] runtime cap: {max_runtime_seconds}s ({max_runtime_seconds // 3600}h {(max_runtime_seconds % 3600) // 60}m)")

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

    training_args = DPOConfig(
        output_dir=output_dir,
        num_train_epochs=NUM_EPOCHS,
        per_device_train_batch_size=BATCH_SIZE,
        gradient_accumulation_steps=GRAD_ACCUM,
        learning_rate=LEARNING_RATE,
        beta=BETA,
        max_length=MAX_LENGTH,
        logging_steps=10,
        save_steps=50,
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
    trainer.train()

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
    args = parser.parse_args()

    corpus_path = args.corpus
    if args.from_gcs:
        local_staging = "/tmp/foundry-training-corpus"
        corpus_path = sync_from_gcs(args.adapter_name, local_staging)

    records = load_feedback_files(corpus_path)
    if not records:
        print("[ERROR] No valid DPO pairs found — check corpus path and field names", file=sys.stderr)
        sys.exit(1)

    date_str = datetime.now(timezone.utc).strftime("%Y%m%d")
    output_dir = args.output_dir or f"./adapters/{args.adapter_name}-{date_str}"

    run_training(records, args.base_model, output_dir, dry_run=args.dry_run,
                 max_runtime_seconds=args.max_runtime_seconds)

    if args.upload_gcs and not args.dry_run:
        upload_adapter_to_gcs(output_dir, args.adapter_name)

    if not args.dry_run:
        print(f"\n[done] adapter at: {output_dir}")
        if GCS_BUCKET and args.upload_gcs:
            print(f"[done] GCS: gs://{GCS_BUCKET}/adapters/{os.path.basename(output_dir)}/")


if __name__ == "__main__":
    main()
