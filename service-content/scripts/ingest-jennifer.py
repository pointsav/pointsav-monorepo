#!/usr/bin/env python3
"""
ingest-jennifer.py — load cluster-totebox-jennifer entities into service-content graph.

Sources:
  1. service-people/people.csv                → person / company / organization entities
  2. service-content/domains/corporate.csv    → domain-term entities (EN terms + definitions)
  3. service-research/ledger/*.yaml           → research-document entities (article metadata)
  4. Markdown document sources (multiple dirs) → research-document, corporate-document,
     regulatory-document, architecture-reference, technical-reference entities

Bypasses the Doorman/LLM extraction step — entities are pre-extracted or derived
directly from structured data. POSTs in batches to POST /v1/graph/mutate.

Usage:
  python3 ingest-jennifer.py [--endpoint URL] [--jennifer-dir PATH] [--dry-run]
                             [--skip-research] [--skip-people] [--skip-corporate]
                             [--skip-documents]

Defaults:
  --endpoint     http://127.0.0.1:9081
  --jennifer-dir /srv/foundry/deployments/cluster-totebox-jennifer
"""

import argparse
import csv
import json
import os
import re
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone


PEOPLE_CSV_RELPATH = "service-people/people.csv"
CORPORATE_CSV_RELPATH = "service-content/domains/corporate.csv"
RESEARCH_LEDGER_RELPATH = "service-research/ledger"
MODULE_ID = "woodfine"
BATCH_SIZE = 100
CORPORATE_LIMIT = 424  # all rows

# Markdown document sources: (relative_path, classification, confidence)
# research-document entries MERGE with YAML ledger entries (same IDs) — enriches role_vector
MARKDOWN_SOURCES = [
    ("service-research/assets",                                      "research-document",    0.80),
    ("service-minutebook/assets",                                    "corporate-document",   0.85),
    ("service-study/corporate-bloomberg-language",                   "corporate-document",   0.85),
    ("service-study/study-private-dealer/assets",                    "regulatory-document",  0.80),
    ("service-study/projects-architecture/assets",                   "architecture-reference", 0.75),
    ("service-study/documentation-general/assets",                   "technical-reference",  0.70),
    ("service-study/documentation-design-slides-response/assets",    "corporate-document",   0.80),
    ("service-agents/david-johnston/assets",                         "corporate-document",   0.80),
    ("pointsav-design-system/tokens/linguistic",                     "corporate-document",   0.80),
]


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


def _title_from_filename(filename: str) -> str:
    """
    Extract a human-readable title from a research YAML filename.

    Handles patterns like:
      RESEARCH_CORPORATE_2024_11_17_Some Article Title_Bloomberg copy 2.yaml
      RESEARCH_PROJECTS_2024_02_03_Some Title_Bloomberg copy.yaml
      21_RESEARCH_CORPORATE_2024_12_15_Title_Bloomberg copy.yaml
      RESEARCH CORPORATE 2026-03-02 AI Threat Prompts UBS...
      RESEARCH CORPORATE Blackstone (BX) Profit Misses...
      COLOUR_PORTRAIT_RESEARCH_CORPORATE_2024_06_07_...
      Some Plain Title copy 2.yaml
    """
    stem = re.sub(r"\.(yaml|md)$", "", filename, flags=re.IGNORECASE)
    # Strip trailing copy markers first (exposes .pdf and source suffixes)
    stem = re.sub(r"\s+copy\s*\d*$", "", stem, flags=re.IGNORECASE).strip()
    # Strip .pdf suffix (some PDF titles saved as .yaml; may appear after copy strip)
    stem = re.sub(r"\.pdf$", "", stem, flags=re.IGNORECASE).strip()
    # Strip known source suffixes: underscore or " - " variants
    _pubs = r"Bloomberg|ZeroHedge|Reuters|FT|WSJ|Globe and Mail|ArchDaily|Urban Omnibus|RENX.*"
    stem = re.sub(rf"\s*[-–]\s*(?:{_pubs})$", "", stem, flags=re.IGNORECASE).strip()
    stem = re.sub(rf"[_ ](?:{_pubs})$", "", stem, flags=re.IGNORECASE).strip()
    # Strip leading numeric prefix: "21_"
    stem = re.sub(r"^\d+_", "", stem)
    # Strip COLOUR_PORTRAIT_ prefix (before RESEARCH strip — may be combined)
    stem = re.sub(r"^COLOUR_PORTRAIT_", "", stem)
    # Strip RESEARCH/PUBLISHED [TYPE] [DATE] prefix in all delimiter variants
    # Order: most specific first
    _sep = r"[_ ]"
    _date_full = r"\d{4}[_\- ]\d{2}[_\- ]\d{2}"
    _date_year = r"\d{4}"
    stem = re.sub(rf"^(?:RESEARCH|PUBLISHED){_sep}[A-Z]+{_sep}(?:{_date_full}|{_date_year}){_sep}", "", stem)
    stem = re.sub(rf"^(?:RESEARCH|PUBLISHED){_sep}[A-Z]+{_sep}", "", stem)
    stem = re.sub(rf"^(?:RESEARCH|PUBLISHED){_sep}", "", stem).strip()
    # Clean up underscores left behind
    stem = stem.replace("_", " ").strip()
    return stem[:120] if stem else filename[:80]


def _extract_source_date_from_filename(filename: str) -> tuple:
    """Return (source, date_str) derived from filename pattern."""
    source = ""
    date_str = ""
    # Date: YYYY_MM_DD anywhere in filename
    m = re.search(r"(\d{4})_(\d{2})_(\d{2})", filename)
    if m:
        date_str = f"{m.group(1)}-{m.group(2)}-{m.group(3)}"
    # Source: known publisher names
    for pub in ("Bloomberg", "ZeroHedge", "Reuters", "FT", "WSJ", "Globe and Mail", "ArchDaily", "Urban Omnibus"):
        if pub.lower() in filename.lower():
            source = pub
            break
    return source, date_str


def _extract_title(doc: dict, filename: str) -> str:
    """Normalize title from any of the 6 known field locations, falling back to filename."""
    title = (
        doc.get("title")
        or doc.get("document_title")
        or (doc.get("article") or {}).get("title")
        or (doc.get("article_metadata") or {}).get("title")
        or (doc.get("document_metadata") or {}).get("title")
        or doc.get("project_name")
    )
    if title:
        return str(title).strip()
    return _title_from_filename(filename)


def _extract_metrics_text(metrics) -> str:
    """Format top-3 metrics from either dict-list or flat-dict form."""
    if not metrics:
        return ""
    parts = []
    if isinstance(metrics, list):
        for m in metrics[:3]:
            if not isinstance(m, dict):
                continue
            name = m.get("name") or m.get("metric") or m.get("type") or ""
            value = m.get("value") or m.get("equivalent_usd") or ""
            if name and value:
                parts.append(f"{name}: {value}")
    elif isinstance(metrics, dict):
        for key, val in list(metrics.items())[:3]:
            if val:
                parts.append(f"{key}: {val}")
    return "; ".join(parts)


def _extract_role_vector(doc: dict, filename: str) -> str:
    """Build role_vector: summary > metrics > source/date > filename stem."""
    summary = str(doc.get("summary") or "").strip()
    if summary:
        return summary[:220]

    metrics_text = _extract_metrics_text(doc.get("metrics"))
    if metrics_text:
        return metrics_text[:220]

    source = (
        str(doc.get("source") or (doc.get("article") or {}).get("source") or "").strip()
    )
    date = str(
        doc.get("date") or doc.get("document_date") or doc.get("publication_date")
        or (doc.get("article") or {}).get("date_published") or ""
    ).strip()[:10]

    parts = [p for p in [source, date] if p]
    if parts:
        return " | ".join(parts)

    # Last resort: derive from filename
    stem = re.sub(r"\.(yaml|md)$", "", filename, flags=re.IGNORECASE)
    stem = stem.replace(" copy 2", "").replace(" copy", "")
    return stem[:120]


def _extract_md_role_vector(filepath: str) -> str:
    """Extract first meaningful paragraph from a markdown file."""
    try:
        with open(filepath, encoding="utf-8", errors="replace") as f:
            content = f.read()
    except Exception:
        return ""
    # Strip YAML frontmatter
    content = re.sub(r"^---.*?---\s*", "", content, flags=re.DOTALL)
    parts = []
    for line in content.splitlines():
        line = line.strip()
        if not line:
            continue
        if line.startswith("http") or "https://" in line:
            continue
        if re.match(r"^\d{1,2}/\d{1,2}/\d{4}", line):
            continue
        if re.match(r"^\s*[©®]", line) or "All Rights Reserved" in line:
            continue
        if re.match(r"^Share feedback", line, re.IGNORECASE) or re.match(r"^Advertisement", line, re.IGNORECASE):
            continue
        # Strip markdown headers to plain text
        line = re.sub(r"^#{1,6}\s+", "", line)
        if len(line) < 15:
            continue
        parts.append(line)
        if len(" ".join(parts)) >= 220:
            break
    result = " ".join(parts)
    return result[:220].strip() if result else ""


def load_markdown_dir(dirpath: str, classification: str, confidence: float) -> tuple:
    """Load *.md files from a single directory as graph entities. Returns (entities, skipped)."""
    if not os.path.isdir(dirpath):
        return [], 0
    entities = []
    skipped = 0
    for filename in sorted(os.listdir(dirpath)):
        if not filename.lower().endswith(".md"):
            continue
        filepath = os.path.join(dirpath, filename)
        title = _title_from_filename(filename)
        if not title or len(title) < 4:
            skipped += 1
            continue
        role_vector = _extract_md_role_vector(filepath)
        entities.append(make_entity(title, classification, confidence, role_vector))
    return entities, skipped


def load_documents(jennifer_dir: str) -> list:
    """Load all markdown document sources defined in MARKDOWN_SOURCES."""
    all_entities = []
    total_skipped = 0
    counts: dict[str, int] = {}
    for relpath, classification, confidence in MARKDOWN_SOURCES:
        dirpath = os.path.join(jennifer_dir, relpath)
        entities, skipped = load_markdown_dir(dirpath, classification, confidence)
        total_skipped += skipped
        if entities:
            counts[classification] = counts.get(classification, 0) + len(entities)
            all_entities.extend(entities)
    for cls, n in sorted(counts.items()):
        print(f"    {cls}: {n}")
    if total_skipped:
        print(f"  (skipped {total_skipped} short/empty filenames)")
    return all_entities


def load_research(jennifer_dir: str) -> list:
    """Load service-research/ledger/*.yaml as research-document entities."""
    ledger_path = os.path.join(jennifer_dir, RESEARCH_LEDGER_RELPATH)
    if not os.path.isdir(ledger_path):
        print(f"  (research ledger not found at {ledger_path} — skipping)")
        return []

    try:
        import yaml as _yaml  # only needed here; stdlib fallback below
        _load = _yaml.safe_load
    except ImportError:
        _load = None

    entities = []
    skipped = 0

    for filename in sorted(os.listdir(ledger_path)):
        if not filename.endswith(".yaml"):
            continue
        filepath = os.path.join(ledger_path, filename)
        try:
            with open(filepath, encoding="utf-8", errors="replace") as fh:
                raw = fh.read()
            if _load is not None:
                doc = _load(raw) or {}
            else:
                doc = _minimal_yaml_parse(raw)
        except Exception:
            skipped += 1
            continue

        if not isinstance(doc, dict):
            skipped += 1
            continue

        title = _extract_title(doc, filename)
        if not title or len(title) < 4:
            skipped += 1
            continue

        role_vector = _extract_role_vector(doc, filename)
        entities.append(make_entity(title, "research-document", 0.80, role_vector))

    if skipped:
        print(f"  (skipped {skipped} unparseable/empty YAML files)")
    return entities


def _minimal_yaml_parse(raw: str) -> dict:
    """Ultra-minimal YAML parser for simple key: value lines (no PyYAML fallback)."""
    result = {}
    for line in raw.splitlines():
        m = re.match(r"^(\w[\w_]*):\s*(.+)$", line)
        if m:
            key, val = m.group(1), m.group(2).strip().strip('"').strip("'")
            if key not in result:
                result[key] = val
    return result


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


def run_ingest(
    endpoint: str,
    jennifer_dir: str,
    dry_run: bool,
    skip_people: bool = False,
    skip_corporate: bool = False,
    skip_research: bool = False,
    skip_documents: bool = False,
) -> None:
    print(f"ingest-jennifer: source={jennifer_dir}")
    print(f"ingest-jennifer: endpoint={endpoint}  module_id={MODULE_ID}  dry_run={dry_run}")
    print()

    all_entities: list = []

    if not skip_people:
        print("Loading people.csv ...")
        people = load_people(jennifer_dir)
        print(f"  → {len(people)} entities (person/company/organization)")
        all_entities.extend(people)

    if not skip_corporate:
        print("Loading corporate.csv ...")
        corporate = load_corporate(jennifer_dir)
        print(f"  → {len(corporate)} entities (domain-term)")
        all_entities.extend(corporate)

    if not skip_research:
        print("Loading service-research/ledger/*.yaml ...")
        research = load_research(jennifer_dir)
        print(f"  → {len(research)} entities (research-document)")
        all_entities.extend(research)

    if not skip_documents:
        print("Loading markdown document sources ...")
        docs = load_documents(jennifer_dir)
        print(f"  → {len(docs)} entities (documents)")
        all_entities.extend(docs)

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
    parser.add_argument("--skip-people", action="store_true", help="skip people.csv (person/company)")
    parser.add_argument("--skip-corporate", action="store_true", help="skip corporate.csv (domain-term)")
    parser.add_argument("--skip-research", action="store_true", help="skip service-research/ledger YAMLs")
    parser.add_argument("--skip-documents", action="store_true", help="skip markdown document sources")
    args = parser.parse_args()
    run_ingest(
        args.endpoint,
        args.jennifer_dir,
        args.dry_run,
        skip_people=args.skip_people,
        skip_corporate=args.skip_corporate,
        skip_research=args.skip_research,
        skip_documents=args.skip_documents,
    )


if __name__ == "__main__":
    main()
