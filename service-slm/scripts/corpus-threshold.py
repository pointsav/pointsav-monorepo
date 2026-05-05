#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR MIT
"""
Phase 3 corpus threshold watcher and training trigger.

Counts JSONL tuples per adapter bucket against per-adapter thresholds.
When a threshold is reached (or --force is supplied), writes a
training-pending marker and, if SLM_YOYO_TRAINER_ENDPOINT is set,
dispatches to the Yo-Yo Trainer (Leapfrog 2030 Yo-Yo #1).

Run modes:
  On-demand:     corpus-threshold.py [--dry-run] [--adapter NAME]
  Sunday cron:   corpus-threshold.py --force          (via systemd timer)

Adapter buckets and thresholds:
  engineering-pointsav    engineering/**/*.jsonl   threshold=50  SFT
  apprenticeship-pointsav apprenticeship/**/*.jsonl threshold=50  DPO

Quality gate (post-D4): ≥60% validation acceptance rate blocks adapter
promotion. Pre-D4 the gate always returns False (safe default).
"""

import argparse
import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

FOUNDRY_ROOT = Path(os.environ.get("FOUNDRY_ROOT", Path.home() / "Foundry"))
CORPUS_ROOT = FOUNDRY_ROOT / "data" / "training-corpus"
PENDING_DIR = FOUNDRY_ROOT / "data" / "training-pending"

QUALITY_GATE_THRESHOLD = 0.60  # ≥60% acceptance rate required before promotion

ADAPTER_SPECS: dict = {
    "engineering-pointsav": {
        "glob": "engineering/**/*.jsonl",
        "threshold": 50,
        "method": "sft",
        "description": "Cross-cluster engineering edit tuples for the pointsav vendor adapter",
    },
    "apprenticeship-pointsav": {
        "glob": "apprenticeship/**/*.jsonl",
        "threshold": 50,
        "method": "dpo",
        "description": "Apprenticeship shadow + verdict tuples; DPO requires signed verdicts",
    },
}


def count_files(glob_pattern: str) -> list:
    return list(CORPUS_ROOT.glob(glob_pattern))


def write_pending_marker(adapter_name: str, files: list, dry_run: bool = False) -> Path:
    timestamp = datetime.now(timezone.utc).isoformat()
    spec = ADAPTER_SPECS[adapter_name]
    marker = {
        "adapter": adapter_name,
        "tuple_count": len(files),
        "training_method": spec["method"],
        "triggered_at": timestamp,
        "d4_ready": bool(os.environ.get("SLM_YOYO_TRAINER_ENDPOINT")),
        "quality_gate_threshold": QUALITY_GATE_THRESHOLD,
        "sample_files": [f.name for f in files[:5]],
    }
    PENDING_DIR.mkdir(parents=True, exist_ok=True)
    marker_path = PENDING_DIR / f"{adapter_name}-{timestamp[:10]}.json"
    if dry_run:
        print(f"    [DRY-RUN] Would write marker: {marker_path}")
        print(f"    [DRY-RUN] {json.dumps(marker, indent=6)}")
        return marker_path
    with open(marker_path, "w") as f:
        json.dump(marker, f, indent=2)
    return marker_path


def check_quality_gate(adapter_name: str, dry_run: bool = False) -> bool:
    """
    Run held-out validation and check acceptance rate >= QUALITY_GATE_THRESHOLD.

    Pre-D4: always returns False (blocks promotion safely).
    Post-D4: POST to SLM_YOYO_TRAINER_ENDPOINT to run held-out inference,
    compute acceptance rate, and return True only if rate >= threshold.
    """
    yoyo_endpoint = os.environ.get("SLM_YOYO_TRAINER_ENDPOINT", "")
    if not yoyo_endpoint:
        print(f"    [QUALITY] SLM_YOYO_TRAINER_ENDPOINT not set (D4 pending) — gate blocks promotion.")
        return False
    if dry_run:
        print(f"    [QUALITY] [DRY-RUN] Would validate against {yoyo_endpoint}.")
        return False
    # TODO(post-D4): implement validation API call to Yo-Yo #1.
    # POST {yoyo_endpoint}/v1/validate with adapter_name + held-out corpus files.
    # Response: { acceptance_rate: float, total_evaluated: int }.
    # Promote if acceptance_rate >= QUALITY_GATE_THRESHOLD.
    print(f"    [QUALITY] Yo-Yo validation not yet wired (post-D4 task). Gate blocks promotion.")
    return False


def trigger_training_cycle(adapter_name: str, files: list, dry_run: bool = False) -> bool:
    """Invoke a training cycle. Returns True if training was successfully dispatched."""
    yoyo_endpoint = os.environ.get("SLM_YOYO_TRAINER_ENDPOINT", "")

    marker_path = write_pending_marker(adapter_name, files, dry_run=dry_run)
    if not dry_run:
        print(f"    [MARKER] Written: {marker_path}")

    if not yoyo_endpoint:
        print(f"    [TRAIN] D4 not configured — marker written for operator pickup.")
        return False

    if dry_run:
        print(f"    [DRY-RUN] Would dispatch training to {yoyo_endpoint}.")
        return False

    # TODO(post-D4): POST to Yo-Yo #1 training endpoint.
    # The Leapfrog 2030 Yo-Yo #1 (g2-standard-4 / L4 / OLMo 3 32B-Think)
    # accepts a training job with the corpus files and returns a job_id.
    print(f"    [TRAIN] D4 endpoint configured but training API not yet wired (post-D4 task).")
    print(f"    [TRAIN] Endpoint: {yoyo_endpoint}")
    return False


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Phase 3 corpus threshold watcher and Yo-Yo training trigger"
    )
    parser.add_argument(
        "--force", action="store_true",
        help="Trigger training regardless of threshold (Sunday 02:00 UTC cron mode)"
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Report counts and decisions without writing markers or calling APIs"
    )
    parser.add_argument(
        "--adapter", metavar="NAME",
        choices=list(ADAPTER_SPECS.keys()),
        help="Check only this adapter (default: all adapters)"
    )
    parser.add_argument(
        "--skip-quality-gate", action="store_true",
        help="Promote adapter without running quality gate (testing only)"
    )
    args = parser.parse_args()

    now = datetime.now(timezone.utc).isoformat()
    flags = []
    if args.force:
        flags.append("FORCE")
    if args.dry_run:
        flags.append("DRY-RUN")
    flag_str = f" [{', '.join(flags)}]" if flags else ""
    print(f"[{now}] Phase 3 corpus threshold check{flag_str}")
    print(f"  FOUNDRY_ROOT    = {FOUNDRY_ROOT}")
    print(f"  CORPUS_ROOT     = {CORPUS_ROOT}")
    yoyo = os.environ.get("SLM_YOYO_TRAINER_ENDPOINT", "(not set — D4 pending)")
    print(f"  TRAINER_ENDPOINT= {yoyo}")
    print()

    any_triggered = False

    for adapter_name, spec in ADAPTER_SPECS.items():
        if args.adapter and adapter_name != args.adapter:
            continue

        files = count_files(spec["glob"])
        count = len(files)
        threshold = spec["threshold"]
        at_threshold = count >= threshold

        print(f"  [{adapter_name}]")
        print(f"    tuples:      {count} / {threshold} threshold")
        print(f"    method:      {spec['method']}")
        print(f"    description: {spec['description']}")

        trigger = at_threshold or args.force
        if not trigger:
            print(f"    status:      accumulating — {threshold - count} more tuples needed")
            print()
            continue

        any_triggered = True
        reason = "threshold reached" if at_threshold else "forced (Sunday cron)"
        print(f"    status:      READY — {reason}")

        dispatched = trigger_training_cycle(adapter_name, files, dry_run=args.dry_run)

        if dispatched or args.skip_quality_gate:
            gate_pass = args.skip_quality_gate or check_quality_gate(
                adapter_name, dry_run=args.dry_run
            )
            if gate_pass:
                print(f"    [PROMOTE]    Quality gate passed (>={QUALITY_GATE_THRESHOLD:.0%}). Adapter ready.")
            else:
                print(f"    [PROMOTE]    Quality gate not met — adapter not promoted.")
        print()

    if not any_triggered:
        print("  No adapters at threshold. No training triggered.")

    sys.exit(0)


if __name__ == "__main__":
    main()
