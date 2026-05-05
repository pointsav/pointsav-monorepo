#!/usr/bin/env python3
"""
ingest-jennifer.py — load cluster-totebox-jennifer entities into service-content graph.

Reads people.csv (persons + companies) and corporate.csv (domain terms) from
the jennifer deployment directory, then POSTs them in batches to service-content's
POST /v1/graph/mutate endpoint.

Bypasses the Doorman/LLM extraction step entirely — entities are pre-extracted.

Usage:
  python3 ingest-jennifer.py [--endpoint URL] [--jennifer-dir PATH] [--dry-run]

Defaults:
  --endpoint     http://127.0.0.1:9081
  --jennifer-dir /srv/foundry/deployments/cluster-totebox-jennifer
"""

import argparse
import csv
import json
import re
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone


PEOPLE_CSV_RELPATH = "service-people/people.csv"
CORPORATE_CSV_RELPATH = "service-content/domains/corporate.csv"
MODULE_ID = "woodfine"
BATCH_SIZE = 100
CORPORATE_LIMIT = 424  # all rows


def normalize_id(module_id: str, name: str) -> str:
    clean = name.lower()
    clean = re.sub(r"[^a-z0-9 ]", "", clean)
    clean = clean.strip().replace(" ", "_")
    clean = re.sub(r"_+", "_", clean)
    return f"{module_id}__{clean[:60]}"


def make_entity(name: str, classification: str, confidence: float, source: str = "") -> dict:
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    return {
        "id": normalize_id(MODULE_ID, name),
        "entity_name": name,
        "classification": classification,
        "role_vector": source[:120] if source else "",
        "location_vector": "",
        "contact_vector": "",
        "module_id": MODULE_ID,
        "confidence": confidence,
        "created_at": now,
    }


def post_batch(endpoint: str, entities: list, batch_num: int, total_batches: int, dry_run: bool) -> bool:
    payload = {"module_id": MODULE_ID, "entities": entities}
    body = json.dumps(payload).encode("utf-8")
    label = f"Batch {batch_num}/{total_batches}: {len(entities)} entities"

    if dry_run:
        print(f"  [dry-run] {label} — would POST to {endpoint}/v1/graph/mutate")
        return True

    req = urllib.request.Request(
        f"{endpoint}/v1/graph/mutate",
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            status = resp.status
            print(f"  {label} → HTTP {status}")
            return status in (200, 201, 204)
    except urllib.error.HTTPError as e:
        print(f"  {label} → HTTP ERROR {e.code}: {e.read().decode('utf-8', errors='replace')[:200]}", file=sys.stderr)
        return False
    except urllib.error.URLError as e:
        print(f"  {label} → CONNECTION ERROR: {e.reason}", file=sys.stderr)
        return False


def load_people(jennifer_dir: str) -> list:
    path = f"{jennifer_dir}/{PEOPLE_CSV_RELPATH}"
    entities = []
    skipped = 0
    with open(path, newline="", encoding="utf-8") as f:
        reader = csv.reader(f, delimiter="|")
        next(reader)  # skip header
        for row in reader:
            if len(row) < 2:
                skipped += 1
                continue
            name = row[0].strip()
            entity_type = row[1].strip() if len(row) > 1 else ""
            source = row[2].strip() if len(row) > 2 else ""
            if not name:
                skipped += 1
                continue
            if entity_type == "Person":
                classification, confidence = "person", 0.90
            elif entity_type == "Company":
                classification, confidence = "company", 0.85
            else:
                classification, confidence = "organization", 0.80
            entities.append(make_entity(name, classification, confidence, source))
    if skipped:
        print(f"  (skipped {skipped} malformed rows in people.csv)")
    return entities


def load_corporate(jennifer_dir: str) -> list:
    path = f"{jennifer_dir}/{CORPORATE_CSV_RELPATH}"
    entities = []
    skipped = 0
    with open(path, newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for i, row in enumerate(reader):
            if i >= CORPORATE_LIMIT:
                break
            name = (row.get("Term_EN") or "").strip()
            definition = (row.get("Definition") or "").strip()
            if not name:
                skipped += 1
                continue
            entities.append(make_entity(name, "domain-term", 0.75, definition[:120]))
    if skipped:
        print(f"  (skipped {skipped} malformed rows in corporate.csv)")
    return entities


def run_ingest(endpoint: str, jennifer_dir: str, dry_run: bool) -> None:
    print(f"ingest-jennifer: source={jennifer_dir}")
    print(f"ingest-jennifer: endpoint={endpoint}  module_id={MODULE_ID}  dry_run={dry_run}")
    print()

    # Load entities from both sources
    print("Loading people.csv ...")
    people = load_people(jennifer_dir)
    print(f"  → {len(people)} entities (person/company/organization)")

    print("Loading corporate.csv ...")
    corporate = load_corporate(jennifer_dir)
    print(f"  → {len(corporate)} entities (domain-term)")

    all_entities = people + corporate
    print(f"\nTotal: {len(all_entities)} entities to upsert\n")

    # Batch and POST
    batches = [all_entities[i:i + BATCH_SIZE] for i in range(0, len(all_entities), BATCH_SIZE)]
    total_batches = len(batches)
    errors = 0

    print(f"Sending {total_batches} batches of up to {BATCH_SIZE} ...")
    for i, batch in enumerate(batches, 1):
        ok = post_batch(endpoint, batch, i, total_batches, dry_run)
        if not ok:
            errors += 1

    # Summary
    print()
    by_class: dict[str, int] = {}
    for e in all_entities:
        by_class[e["classification"]] = by_class.get(e["classification"], 0) + 1

    print("=== Summary ===")
    for cls, count in sorted(by_class.items()):
        print(f"  {cls}: {count}")
    print(f"  TOTAL: {len(all_entities)}")
    if errors:
        print(f"  ERRORS: {errors} batches failed", file=sys.stderr)
        sys.exit(1)
    else:
        print("  STATUS: all batches succeeded" if not dry_run else "  STATUS: dry-run complete")


def main() -> None:
    parser = argparse.ArgumentParser(description="Load cluster-totebox-jennifer entities into service-content graph")
    parser.add_argument("--endpoint", default="http://127.0.0.1:9081", help="service-content HTTP endpoint")
    parser.add_argument("--jennifer-dir", default="/srv/foundry/deployments/cluster-totebox-jennifer")
    parser.add_argument("--dry-run", action="store_true", help="print batches without sending")
    args = parser.parse_args()
    run_ingest(args.endpoint, args.jennifer_dir, args.dry_run)


if __name__ == "__main__":
    main()
