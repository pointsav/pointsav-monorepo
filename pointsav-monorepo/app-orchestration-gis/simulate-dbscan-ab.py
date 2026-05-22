#!/usr/bin/env python3
"""
simulate-dbscan-ab.py — Alberta DBSCAN triangulation simulation.

Compares the current anchor-centric ring algorithm against the proposed
DBSCAN proximity graph for Alberta clusters. Writes a single GeoJSON to
the live deployment's data/ dir that the map loads as a toggleable overlay.

Output: /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/sim-ab-dbscan.geojson

Usage:
    python3 simulate-dbscan-ab.py            # full Alberta
    python3 simulate-dbscan-ab.py edmonton   # Edmonton metro only
"""
import json
import math
import sys
from collections import defaultdict
from pathlib import Path

# ── PATHS ─────────────────────────────────────────────────────────────────────
BUSINESS_DIR = Path("/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business")
OUT_PATH     = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/sim-ab-dbscan.geojson")

# ── BOUNDING BOXES ────────────────────────────────────────────────────────────
BBOXES = {
    "alberta":  (49.0, 60.0, -120.0, -110.0),
    "edmonton": (53.20, 53.80, -114.0, -113.0),
}
scope = sys.argv[1].lower() if len(sys.argv) > 1 else "alberta"
BBOX  = BBOXES.get(scope, BBOXES["alberta"])
print(f"Scope: {scope}  bbox={BBOX}")

# ── ALGORITHM THRESHOLDS ──────────────────────────────────────────────────────
TAU_TIGHT_KM = 1.0
TAU_LOOSE_KM = 3.0

# ── CHAIN TAXONOMY (CA region, self-contained — no config.py import) ──────────
# class: hypermarket | hardware | warehouse | lifestyle
# role:  anchor (can seed old-algo cluster) | secondary | both
CHAIN_META = {
    "walmart-ca":                  ("hypermarket", "anchor",    "walmart-inc"),
    "real-canadian-superstore-ca": ("hypermarket", "anchor",    "loblaws-companies"),
    "ikea-ca":                     ("lifestyle",   "anchor",    "ingka-group"),
    "home-depot-ca":               ("hardware",    "both",      "home-depot-inc"),
    "costco-ca":                   ("warehouse",   "both",      "costco-wholesale"),
    "lowes-ca":                    ("hardware",    "secondary", "lowes-companies"),
    "canadian-tire-ca":            ("hardware",    "secondary", "canadian-tire-corp"),
}
CA_ANCHORS   = {k for k, (_, role, _) in CHAIN_META.items() if role in ("anchor", "both")}
CA_HARDWARE  = {k for k, (cls, _, _) in CHAIN_META.items() if cls == "hardware"}
CA_WAREHOUSE = {k for k, (cls, _, _) in CHAIN_META.items() if cls == "warehouse"}
# All qualifying chains for the graph (both seed and secondary)
CA_ALL       = set(CHAIN_META.keys())

CLASS_RANK = {"hypermarket": 4, "warehouse": 3, "lifestyle": 3, "hardware": 2}

# ── HELPERS ───────────────────────────────────────────────────────────────────
def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    ph1, ph2 = math.radians(lat1), math.radians(lat2)
    a = (math.sin(math.radians(lat2 - lat1) / 2) ** 2
         + math.cos(ph1) * math.cos(ph2)
         * math.sin(math.radians(lon2 - lon1) / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def build_grid(recs, size=0.1):
    g = {}
    for idx, r in enumerate(recs):
        cell = (int(float(r["latitude"]) / size), int(float(r["longitude"]) / size))
        g.setdefault(cell, []).append(idx)
    return g


def neighbours_within(lat, lon, recs, grid, r_km, size=0.1):
    deg = (r_km + 0.5) / 111.0
    result = []
    for la in range(int((lat - deg) / size), int((lat + deg) / size) + 1):
        for lo in range(int((lon - deg) / size), int((lon + deg) / size) + 1):
            for idx in grid.get((la, lo), []):
                d = haversine_km(lat, lon,
                                 float(recs[idx]["latitude"]),
                                 float(recs[idx]["longitude"]))
                if d <= r_km:
                    result.append((idx, d))
    return result


# ── LOAD DATA ─────────────────────────────────────────────────────────────────
def load_records():
    records = []
    lat_min, lat_max, lon_min, lon_max = BBOX
    for chain_id, (cls, role, owner) in CHAIN_META.items():
        path = BUSINESS_DIR / f"{chain_id}.jsonl"
        if not path.exists():
            print(f"  MISSING {path.name}")
            continue
        n = 0
        with open(path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                    lat = float(r["latitude"])
                    lon = float(r["longitude"])
                    if lat_min <= lat <= lat_max and lon_min <= lon <= lon_max:
                        r["chain_id"] = chain_id
                        r["cls"]      = cls
                        r["owner"]    = owner
                        r["role"]     = role
                        records.append(r)
                        n += 1
                except Exception:
                    pass
        print(f"  {chain_id}: {n}")
    print(f"  Total: {len(records)} records")
    return records


# ── OLD ALGORITHM: anchor-centric ring scan ────────────────────────────────────
def run_old(records):
    anchors = [r for r in records if r["role"] in ("anchor", "both")]
    hw_recs = [r for r in records if r["chain_id"] in CA_HARDWARE]
    wh_recs = [r for r in records if r["chain_id"] in CA_WAREHOUSE]
    hw_grid = build_grid(hw_recs)
    wh_grid = build_grid(wh_recs)
    all_grid = build_grid(records)

    clusters = []
    for anc in anchors:
        lat, lon = float(anc["latitude"]), float(anc["longitude"])
        cid = anc["chain_id"]

        hw_near = [hw_recs[i] for i, _ in neighbours_within(lat, lon, hw_recs, hw_grid, TAU_LOOSE_KM)
                   if hw_recs[i]["chain_id"] != cid]
        wh_near = [wh_recs[i] for i, _ in neighbours_within(lat, lon, wh_recs, wh_grid, TAU_LOOSE_KM)
                   if wh_recs[i]["chain_id"] != cid]

        has_hw = len(hw_near) > 0
        has_wh = len(wh_near) > 0
        if has_hw and has_wh:
            tier = 3
        elif has_hw or has_wh:
            tier = 2
        else:
            continue  # no secondary — not a cluster

        nearby = [records[i] for i, _ in neighbours_within(lat, lon, records, all_grid, TAU_LOOSE_KM)]
        chains  = sorted({r["chain_id"] for r in nearby} | {cid})
        classes = sorted({r["cls"] for r in nearby} | {anc["cls"]})

        clusters.append({
            "lat": lat, "lon": lon,
            "is_centroid": False,
            "tier": tier,
            "anchor": cid,
            "chains": chains,
            "classes": classes,
        })
    return clusters


# ── NEW ALGORITHM: DBSCAN proximity graph ─────────────────────────────────────
def union_find(n, edges):
    parent = list(range(n))
    rank   = [0] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        ra, rb = find(a), find(b)
        if ra == rb:
            return
        if rank[ra] < rank[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        if rank[ra] == rank[rb]:
            rank[ra] += 1

    for a, b in edges:
        union(a, b)
    comps = defaultdict(list)
    for i in range(n):
        comps[find(i)].append(i)
    return list(comps.values())


def component_diameter(indices, records):
    max_d = 0.0
    for i in range(len(indices)):
        for j in range(i + 1, len(indices)):
            a, b = records[indices[i]], records[indices[j]]
            d = haversine_km(float(a["latitude"]), float(a["longitude"]),
                             float(b["latitude"]), float(b["longitude"]))
            if d > max_d:
                max_d = d
    return max_d


def split_greedy(indices, records, max_d):
    """Greedy clique-like partition: every member within max_d of every other member."""
    remaining = sorted(indices,
                       key=lambda i: (-CLASS_RANK.get(records[i]["cls"], 1),
                                      records[i]["chain_id"]))
    groups = []
    while remaining:
        seed = remaining.pop(0)
        group = [seed]
        still_remaining = []
        for candidate in remaining:
            fits = all(
                haversine_km(float(records[candidate]["latitude"]),
                             float(records[candidate]["longitude"]),
                             float(records[g]["latitude"]),
                             float(records[g]["longitude"])) <= max_d
                for g in group
            )
            if fits:
                group.append(candidate)
            else:
                still_remaining.append(candidate)
        remaining = still_remaining
        groups.append(group)
    return groups


def tight_intact(indices, records):
    for i in range(len(indices)):
        for j in range(i + 1, len(indices)):
            a, b = records[indices[i]], records[indices[j]]
            if haversine_km(float(a["latitude"]), float(a["longitude"]),
                            float(b["latitude"]), float(b["longitude"])) > TAU_TIGHT_KM:
                return False
    return True


def evaluate_tier_new(indices, records):
    members = [records[i] for i in indices]
    chains  = {m["chain_id"] for m in members}
    classes = {m["cls"]      for m in members}

    has_hyper = "hypermarket" in classes
    has_hw    = "hardware"    in classes
    has_wh    = "warehouse"   in classes   # warehouse club (Costco)
    has_life  = "lifestyle"   in classes   # lifestyle / furniture (IKEA)

    ti = tight_intact(indices, records)    # geometric flag — NOT a tier input

    # Tier is retailer-category COMPOSITION only (BRIEF-VARIABLE-DISTANCE §1).
    # Tightness/distance is a separate rank axis and never gates the tier.

    # T1 Regional: Hypermarket ∧ Hardware ∧ (Warehouse club ∨ Lifestyle)
    if has_hyper and has_hw and (has_wh or has_life):
        return 1, ti

    # T2 District: Hypermarket ∧ at least one major co-anchor
    if has_hyper and (has_hw or has_wh or has_life):
        return 2, ti

    # T3 Local: any co-location of ≥2 distinct chains
    if len(chains) >= 2:
        return 3, ti

    return None, ti  # singleton → not a co-location


def split_greedy_tight(indices, records, max_d, atom_of):
    """Greedy clique partition that never separates a frozen tight nucleus.
    Atoms (Pass-1 tight components) move as units; an atom joins a group only
    if every one of its members is within max_d of every member already in
    the group."""
    atoms = {}
    for idx in indices:
        atoms.setdefault(atom_of[idx], []).append(idx)
    # seed with the largest nuclei first; atom id breaks ties deterministically
    remaining = sorted(atoms.keys(), key=lambda a: (-len(atoms[a]), a))
    groups = []
    while remaining:
        seed = remaining.pop(0)
        group = list(atoms[seed])
        still = []
        for cand in remaining:
            fits = all(
                haversine_km(float(records[c]["latitude"]), float(records[c]["longitude"]),
                             float(records[g]["latitude"]), float(records[g]["longitude"])) <= max_d
                for c in atoms[cand] for g in group
            )
            if fits:
                group.extend(atoms[cand])
            else:
                still.append(cand)
        remaining = still
        groups.append(group)
    return groups


def run_new(records):
    """Two-pass tight-first DBSCAN. Pass 1 freezes tight (≤TAU_TIGHT) nuclei;
    Pass 2 groups at TAU_LOOSE with those nuclei moving as atomic units, so a
    tight pairing is never dissolved into a looser blob by seed order."""
    n    = len(records)
    grid = build_grid(records)

    def graph_edges(radius):
        e = set()
        for i, r in enumerate(records):
            lat, lon = float(r["latitude"]), float(r["longitude"])
            for j, d in neighbours_within(lat, lon, records, grid, radius):
                if j != i:
                    e.add((min(i, j), max(i, j)))
        return e

    # ── Pass 1 — TIGHT graph: freeze co-location nuclei at TAU_TIGHT_KM ──
    tight_comps = union_find(n, graph_edges(TAU_TIGHT_KM))
    atom_of = {}
    for atom_id, comp in enumerate(tight_comps):
        for idx in comp:
            atom_of[idx] = atom_id

    # ── Pass 2 — LOOSE graph at TAU_LOOSE_KM; nuclei never split ──
    loose_comps = union_find(n, graph_edges(TAU_LOOSE_KM))

    clusters   = []
    singletons = 0
    for comp in loose_comps:
        # Split only if the component exceeds the 3 km membership cap
        if component_diameter(comp, records) > TAU_LOOSE_KM:
            groups = split_greedy_tight(comp, records, TAU_LOOSE_KM, atom_of)
        else:
            groups = [comp]

        for group in groups:
            members = [records[i] for i in group]
            lats = [float(m["latitude"])  for m in members]
            lons = [float(m["longitude"]) for m in members]

            tier, ti = evaluate_tier_new(group, records)
            if tier is None:
                singletons += 1
                continue

            chains  = sorted({m["chain_id"] for m in members})
            classes = sorted({m["cls"]      for m in members})
            owners  = sorted({m["owner"]    for m in members})

            clusters.append({
                "lat": sum(lats) / len(lats),
                "lon": sum(lons) / len(lons),
                "is_centroid": True,
                "tier": tier,
                "tight": ti,
                "span_km": round(component_diameter(group, records), 3),
                "n_chains": len(chains),
                "n_owners": len(owners),
                "chains": chains,
                "classes": classes,
                "member_count": len(members),
                "members_detail": [
                    {"chain": m["chain_id"], "cls": m["cls"],
                     "lat": float(m["latitude"]), "lon": float(m["longitude"])}
                    for m in members
                ],
            })

    print(f"  {len(clusters)} tiered clusters  |  {singletons} ring-layer singletons")
    return clusters


def assign_distance_ranks(clusters):
    """Stage-1 geometric rank: inverted percentile of span_km, computed WITHIN
    tier. Alberta sim = one country / one continent, so the pool is per-tier
    (production blends Country + continent percentiles — see BRIEF §3)."""
    by_tier = defaultdict(list)
    for c in clusters:
        by_tier[c["tier"]].append(c)
    for tier, group in by_tier.items():
        ordered = sorted(group, key=lambda c: (c["span_km"], c["lat"], c["lon"]))
        m = len(ordered)
        for i, c in enumerate(ordered):
            frac = i / (m - 1) if m > 1 else 0.0          # 0.0 = tightest in tier
            c["dist_rank_in_tier"] = round(frac, 3)
            c["dist_pctile"]       = int(round(100 * (1 - frac)))  # 100 = tightest


# ── DELTA MATCHING ─────────────────────────────────────────────────────────────
def match_delta(old, new):
    DELTA_COLOUR = {
        "upgraded":   "#22C55E",  # bright green
        "unchanged":  "#94A3B8",  # neutral slate — not blue (avoids co-location ring conflict)
        "downgraded": "#F43F5E",  # rose-red (shifted from pure red for deuteranopia safety)
        "new":        "#A855F7",  # violet
    }
    for nc in new:
        best_d, best_oc = 99999, None
        for oc in old:
            d = haversine_km(nc["lat"], nc["lon"], oc["lat"], oc["lon"])
            if d < best_d:
                best_d, best_oc = d, oc

        if best_oc is None or best_d > 5.0:
            nc["delta"] = "new"
            nc["old_tier"] = None
            nc["shift_km"] = None
        else:
            nc["shift_km"] = round(best_d, 2)
            nc["old_tier"] = best_oc["tier"]
            ot, nt = best_oc["tier"], nc["tier"]
            nc["delta"] = "upgraded" if nt < ot else "downgraded" if nt > ot else "unchanged"

        nc["colour"] = DELTA_COLOUR[nc["delta"]]
    return new


# ── BUILD GEOJSON ─────────────────────────────────────────────────────────────
TIER_LABEL = {1: "Regional", 2: "District", 3: "Local"}


def build_geojson(old, new, scope_label):
    features = []

    for oc in old:
        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [oc["lon"], oc["lat"]]},
            "properties": {
                "kind":       "old",
                "tier":       oc["tier"],
                "tier_label": TIER_LABEL.get(oc["tier"], "?"),
                "anchor":     oc["anchor"],
                "chains":     ", ".join(oc["chains"]),
                "classes":    ", ".join(oc["classes"]),
                "label":      f"OLD · T{oc['tier']} · {oc['anchor']}",
                "colour":     "#b45309",  # amber — old anchor-pin clusters
            },
        })

    for nc in new:
        prox = "tight ≤1km" if nc.get("tight") else "loose ≤3km"
        delta_str = nc.get("delta", "new")
        old_t = nc.get("old_tier")
        arrow = (f" (was T{old_t})" if old_t and old_t != nc["tier"] else
                 f" (shift {nc['shift_km']}km)" if nc.get("shift_km", 0) and nc.get("shift_km", 0) > 0.05 else "")
        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [nc["lon"], nc["lat"]]},
            "properties": {
                "kind":         "new",
                "tier":         nc["tier"],
                "tier_label":   TIER_LABEL.get(nc["tier"], "?"),
                "tight":        nc.get("tight", False),
                "tight_intact": nc.get("tight", False),
                "prox":         prox,
                "span_km":      nc.get("span_km", 0.0),
                "dist_rank_in_tier": nc.get("dist_rank_in_tier", 0.0),
                "dist_pctile":  nc.get("dist_pctile", 0),
                "n_chains":     nc["n_chains"],
                "n_owners":     nc["n_owners"],
                "chains":       ", ".join(nc["chains"]),
                "classes":      ", ".join(nc["classes"]),
                "member_count": nc["member_count"],
                "members_detail": nc.get("members_detail", []),
                "delta":        delta_str,
                "old_tier":     old_t,
                "shift_km":     nc.get("shift_km"),
                "colour":       nc.get("colour", "#94A3B8"),
                "label":        f"NEW · T{nc['tier']} · {prox}{arrow} · {', '.join(nc['chains'])}",
            },
        })

    return {
        "type": "FeatureCollection",
        "metadata": {
            "scope":         scope_label,
            "generated":     "2026-05-20",
            "tau_tight_km":  TAU_TIGHT_KM,
            "tau_loose_km":  TAU_LOOSE_KM,
            "old_count":     len(old),
            "new_count":     len(new),
        },
        "features": features,
    }


# ── MAIN ──────────────────────────────────────────────────────────────────────
def main():
    print("\n=== Alberta DBSCAN Simulation ===\n")
    print("Loading records...")
    records = load_records()
    print()

    print("OLD algorithm (anchor-centric rings)...")
    old = run_old(records)
    print(f"  {len(old)} clusters")
    dist = defaultdict(int)
    for c in old:
        dist[c["tier"]] += 1
    for t in sorted(dist):
        print(f"    T{t}: {dist[t]}")
    print()

    print("NEW algorithm (DBSCAN proximity graph)...")
    new = run_new(records)
    dist2 = defaultdict(int)
    for c in new:
        dist2[c["tier"]] += 1
    for t in sorted(dist2):
        print(f"    T{t}: {dist2[t]}")
    print()

    print("Stage-1 geometric rank (span_km percentile within tier)...")
    assign_distance_ranks(new)
    print()

    print("Matching deltas...")
    new = match_delta(old, new)
    delta_dist = defaultdict(int)
    for c in new:
        delta_dist[c.get("delta", "?")] += 1
    for d in sorted(delta_dist):
        print(f"    {d}: {delta_dist[d]}")
    print()

    gj = build_geojson(old, new, scope)
    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(OUT_PATH, "w") as f:
        json.dump(gj, f, separators=(",", ":"))
    old_n = sum(1 for ft in gj["features"] if ft["properties"]["kind"] == "old")
    new_n = sum(1 for ft in gj["features"] if ft["properties"]["kind"] == "new")
    print(f"Written: {OUT_PATH}")
    print(f"  {old_n} OLD features  +  {new_n} NEW features  =  {old_n + new_n} total")
    print()
    print("Load on map: toggle 'DBSCAN Sim (AB)' in the Layers panel.")


if __name__ == "__main__":
    main()
