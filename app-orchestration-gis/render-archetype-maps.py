#!/usr/bin/env python3
"""Render S1 (VWH) and S2 (PKS) archetype map figures for practitioner summaries.

Output: work/figure-s1-vwh-map.png (VWH, 6,368 clusters, T1/T2/T3 colour-coded)
        work/figure-s2-pks-map.png (PKS, 7,045 clusters, T1/T2/T3 colour-coded)

Colours match gis.woodfinegroup.com tier palette: T1=blue, T2=green, T3=orange.
Basemap: Natural Earth 110m (bundled in pyogrio test fixtures).
"""

import json
import os
import sys

import geopandas as gpd
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np
from shapely.geometry import Point

BASEMAP_SHP = (
    "/home/mathew/.local/lib/python3.12/site-packages"
    "/pyogrio/tests/fixtures/naturalearth_lowres/naturalearth_lowres.shp"
)

DATA_DIR = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data"
OUT_DIR = "/srv/foundry/clones/project-gis/work/figures"

TIER_COLORS = {
    "T1": "#2563EB",   # blue-600  — highest tier
    "T2": "#16A34A",   # green-600
    "T3": "#EA580C",   # orange-600
}
TIER_SIZES = {"T1": 14, "T2": 6, "T3": 2}
TIER_ALPHA  = {"T1": 0.9, "T2": 0.7, "T3": 0.4}

# Study geography bounding box (18 countries NA+EU, excludes AU/AS)
XLIM = (-130, 35)
YLIM = (14, 72)


def load_world():
    return gpd.read_file(BASEMAP_SHP)


def load_clusters(geojson_path, tier_field):
    with open(geojson_path) as f:
        data = json.load(f)
    records = []
    for feat in data["features"]:
        p = feat["properties"]
        tier_raw = p.get(tier_field, "")
        tier = f"T{tier_raw}" if str(tier_raw).isdigit() else str(tier_raw)
        if tier not in TIER_COLORS:
            continue
        records.append({"lon": p["lon"], "lat": p["lat"], "tier": tier})
    return records


def render(clusters, tier_field_label, title, subtitle, out_path):
    world = load_world()

    fig, ax = plt.subplots(figsize=(14, 7), dpi=150)
    fig.patch.set_facecolor("#F8FAFC")
    ax.set_facecolor("#E8F4FD")

    world.plot(
        ax=ax,
        color="#EFF6FF",
        edgecolor="#94A3B8",
        linewidth=0.3,
    )

    # Plot tiers in reverse order so T1 renders on top
    for tier in ["T3", "T2", "T1"]:
        pts = [(c["lon"], c["lat"]) for c in clusters if c["tier"] == tier]
        if not pts:
            continue
        lons, lats = zip(*pts)
        ax.scatter(
            lons, lats,
            s=TIER_SIZES[tier],
            c=TIER_COLORS[tier],
            alpha=TIER_ALPHA[tier],
            linewidths=0,
            zorder=3 if tier == "T1" else (2 if tier == "T2" else 1),
        )

    ax.set_xlim(XLIM)
    ax.set_ylim(YLIM)
    ax.set_aspect("equal")
    ax.axis("off")

    # Legend
    t1_n = sum(1 for c in clusters if c["tier"] == "T1")
    t2_n = sum(1 for c in clusters if c["tier"] == "T2")
    t3_n = sum(1 for c in clusters if c["tier"] == "T3")
    legend_handles = [
        mpatches.Patch(color=TIER_COLORS["T1"], label=f"Tier 1 (n={t1_n:,})"),
        mpatches.Patch(color=TIER_COLORS["T2"], label=f"Tier 2 (n={t2_n:,})"),
        mpatches.Patch(color=TIER_COLORS["T3"], label=f"Tier 3 (n={t3_n:,})"),
    ]
    legend = ax.legend(
        handles=legend_handles,
        loc="lower left",
        framealpha=0.9,
        fontsize=8,
        title="Co-location tier",
        title_fontsize=8,
    )

    # Title block
    total = len(clusters)
    ax.set_title(
        f"{title}\n{subtitle}  ·  {total:,} clusters  ·  18 countries",
        fontsize=10,
        fontweight="bold",
        pad=8,
        color="#1E293B",
    )

    plt.tight_layout(pad=0.5)
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    fig.savefig(out_path, dpi=150, bbox_inches="tight", facecolor=fig.get_facecolor())
    plt.close(fig)
    print(f"  Saved: {out_path} ({os.path.getsize(out_path) // 1024} KB)")


def main():
    print("Rendering S1 — VWH Urban Fringe archetype map...")
    vwh = load_clusters(
        os.path.join(DATA_DIR, "archetype-vwh.geojson"),
        tier_field="vwh_tier",
    )
    render(
        vwh,
        tier_field_label="vwh_tier",
        title="Urban Fringe (VWH) Co-location Archetype",
        subtitle="Industrial trade-services supply clusters in the metropolitan ring",
        out_path=os.path.join(OUT_DIR, "figure-s1-vwh-map.png"),
    )

    print("Rendering S2 — PKS Commuter archetype map...")
    pks = load_clusters(
        os.path.join(DATA_DIR, "archetype-pks.geojson"),
        tier_field="commuter_tier",
    )
    render(
        pks,
        tier_field_label="commuter_tier",
        title="Commuter (PKS) Co-location Archetype",
        subtitle="Transit-adjacent commercial clusters at rail stations and regional airports",
        out_path=os.path.join(OUT_DIR, "figure-s2-pks-map.png"),
    )

    print("Done.")


if __name__ == "__main__":
    main()
