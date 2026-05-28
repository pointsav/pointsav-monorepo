#!/usr/bin/env python3
"""
ingest-gis-graph.py — load GIS retail cluster entities into service-content graph.

Reads clusters-meta.json produced by the GIS pipeline and upserts each cluster as
a retail-cluster GraphEntity in the DataGraph (LadybugDB via service-content).

This bridges the GIS pipeline output to the Doorman context-injection path: once
clusters are in the graph, every inference call through the Doorman receives
geospatial retail context injected into the system prompt.

Source: /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json
Target: service-content POST /v1/graph/mutate (or via Doorman POST /v1/graph/mutate)

Usage:
  python3 ingest-gis-graph.py [--endpoint URL] [--clusters-json PATH]
                               [--dry-run] [--limit N] [--tier {1,2,3}]
                               [--via-doorman]

Defaults:
  --endpoint       http://127.0.0.1:9081   (direct to service-content)
  --clusters-json  /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json
  --via-doorman    POST to Doorman at :9080 instead (adds audit logging; requires X-Foundry-Module-ID)
"""

import argparse
import json
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone

CLUSTERS_JSON = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
DEFAULT_ENDPOINT = "http://127.0.0.1:9081"
DOORMAN_ENDPOINT = "http://127.0.0.1:9080"
MODULE_ID = "woodfine"
BATCH_SIZE = 100

TIER_CONFIDENCE = {1: 0.95, 2: 0.85, 3: 0.75}

CONF_MAP = {"high": 0.92, "med": 0.78, "low": 0.65}


def cluster_to_entity(c: dict) -> dict:
    cluster_id = c.get("id", "")
    tier = c.get("t", 3)
    tier_desc = c.get("td", f"T{tier} Cluster")
    mkt = (c.get("mkt") or "").strip()
    iso = (c.get("iso") or "").strip()
    mrgn = (c.get("mrgn") or "").strip()
    cont = (c.get("cont") or "").strip()
    lat = c.get("lat", 0.0)
    lon = c.get("lon", 0.0)
    catchment_km = c.get("rr", 0.0)
    member_count = c.get("mc", 0)
    members = c.get("members", [])
    conf_str = c.get("conf", "")
    ashrae = (c.get("ashrae_zone") or "").strip()

    # entity_name: "<tier_desc> — <market>, <iso>" or fall back to cluster id
    if mkt:
        entity_name = f"{tier_desc} — {mkt}, {iso}"
    else:
        entity_name = f"T{tier} {tier_desc} Cluster [{iso}]"
    entity_name = entity_name[:120]

    # role_vector: tier, anchor chains, catchment, confidence (≤220 chars)
    chain_names = [m.get("name", "") for m in members[:5] if m.get("name")]
    chains_str = ", ".join(chain_names) if chain_names else "mixed"
    role_parts = [
        f"T{tier} cluster",
        f"{member_count} anchor(s): {chains_str}",
        f"catchment {catchment_km:.0f}km",
    ]
    if conf_str:
        role_parts.append(f"{conf_str} conf")
    if ashrae:
        role_parts.append(f"climate:{ashrae}")
    role_vector = "; ".join(role_parts)[:220]

    # location_vector: lat/lon, country, continent, regional market (≤220 chars)
    loc_parts = [f"lat:{lat:.4f},lon:{lon:.4f}", iso]
    if cont:
        loc_parts.append(cont)
    if mrgn:
        loc_parts.append(mrgn[:80])
    location_vector = "; ".join(loc_parts)[:220]

    # confidence: prefer explicit conf field, fall back to tier default
    confidence = CONF_MAP.get(conf_str, TIER_CONFIDENCE.get(tier, 0.75))

    # stable deterministic id
    safe_id = cluster_id.replace("/", "_").replace(".", "_")[:60]
    entity_id = f"{MODULE_ID}__gis_{safe_id}"

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    return {
        "id": entity_id,
        "entity_name": entity_name,
        "classification": "retail-cluster",
        "role_vector": role_vector,
        "location_vector": location_vector,
        "contact_vector": "",
        "module_id": MODULE_ID,
        "confidence": confidence,
        "created_at": now,
    }


def post_batch(
    endpoint: str,
    entities: list,
    batch_num: int,
    total_batches: int,
    dry_run: bool,
    via_doorman: bool,
) -> bool:
    payload = {"module_id": MODULE_ID, "entities": entities}
    body = json.dumps(payload).encode("utf-8")
    label = f"Batch {batch_num}/{total_batches}: {len(entities)} entities"

    if dry_run:
        sample = entities[0]["entity_name"] if entities else "—"
        print(f"  [dry-run] {label} — first: {sample!r}")
        return True

    url = f"{endpoint}/v1/graph/mutate"
    headers = {"Content-Type": "application/json"}
    if via_doorman:
        headers["X-Foundry-Module-ID"] = MODULE_ID

    req = urllib.request.Request(url, data=body, headers=headers, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            print(f"  {label} → HTTP {resp.status}")
            return resp.status in (200, 201, 204)
    except urllib.error.HTTPError as e:
        body_text = e.read().decode("utf-8", errors="replace")[:200]
        print(f"  {label} → HTTP ERROR {e.code}: {body_text}", file=sys.stderr)
        return False
    except urllib.error.URLError as e:
        print(f"  {label} → CONNECTION ERROR: {e.reason}", file=sys.stderr)
        return False


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Load GIS retail cluster entities into service-content DataGraph"
    )
    parser.add_argument("--endpoint", default=DEFAULT_ENDPOINT)
    parser.add_argument("--clusters-json", default=CLUSTERS_JSON, metavar="PATH")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--limit", type=int, default=0, help="max clusters to process (0=all)")
    parser.add_argument("--tier", type=int, choices=[1, 2, 3], default=0, help="filter to one tier (0=all)")
    parser.add_argument(
        "--via-doorman", action="store_true",
        help=f"POST via Doorman ({DOORMAN_ENDPOINT}) for audit logging instead of direct"
    )
    args = parser.parse_args()

    endpoint = DOORMAN_ENDPOINT if args.via_doorman else args.endpoint

    print(f"ingest-gis-graph: source={args.clusters_json}")
    print(f"ingest-gis-graph: endpoint={endpoint}  module_id={MODULE_ID}  dry_run={args.dry_run}")
    if args.tier:
        print(f"ingest-gis-graph: tier filter=T{args.tier}")
    print()

    try:
        with open(args.clusters_json, encoding="utf-8") as f:
            clusters = json.load(f)
    except FileNotFoundError:
        print(f"ERROR: clusters-meta.json not found at {args.clusters_json}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"ERROR: failed to parse clusters-meta.json: {e}", file=sys.stderr)
        sys.exit(1)

    if not isinstance(clusters, list):
        print("ERROR: clusters-meta.json expected to be a JSON array", file=sys.stderr)
        sys.exit(1)

    if args.tier:
        clusters = [c for c in clusters if c.get("t") == args.tier]
    if args.limit:
        clusters = clusters[: args.limit]

    print(f"Loaded {len(clusters)} clusters from source")

    entities = [cluster_to_entity(c) for c in clusters]

    tier_counts: dict[int, int] = {}
    for c in clusters:
        t = c.get("t", 0)
        tier_counts[t] = tier_counts.get(t, 0) + 1
    for t in sorted(tier_counts):
        print(f"  T{t}: {tier_counts[t]} clusters")
    print(f"  TOTAL: {len(entities)} entities to upsert")
    print()

    batches = [entities[i : i + BATCH_SIZE] for i in range(0, len(entities), BATCH_SIZE)]
    total_batches = len(batches)
    errors = 0

    print(f"Sending {total_batches} batches of up to {BATCH_SIZE} ...")
    for i, batch in enumerate(batches, 1):
        ok = post_batch(endpoint, batch, i, total_batches, args.dry_run, args.via_doorman)
        if not ok:
            errors += 1

    print()
    print("=== Summary ===")
    print(f"  retail-cluster: {len(entities)}")
    if errors:
        print(f"  ERRORS: {errors} batches failed", file=sys.stderr)
        sys.exit(1)
    else:
        status = "dry-run complete" if args.dry_run else "all batches succeeded"
        print(f"  STATUS: {status}")


if __name__ == "__main__":
    main()
