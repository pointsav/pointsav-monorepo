#!/usr/bin/env python3
"""
generate-figures-f1-f5.py — Produce JoEG manuscript figures F1–F5 from Phase 22 data.

Outputs to work/figures/:
  F1-decision-tree.svg / .png  — Tier classification decision tree
  F2-dbscan-schematic.png       — Two-pass DBSCAN algorithm diagram
  F3-continental-map.png        — NA + EU cluster dot map (equal-area projections)
  F4-country-bars.png           — Per-country T1 share + count
  F5-span-violin.png            — Span_km distribution by tier (Kruskal-Wallis)

F6 (OLS coefficient forest plot) requires kontur population join — separate script.
"""
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import matplotlib.lines as mlines
import matplotlib.patheffects as pe
import numpy as np
import seaborn as sns
from scipy import stats

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

SOURCE     = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
FIG_DIR    = WORK_DIR / "figures"
FIG_DIR.mkdir(parents=True, exist_ok=True)

# Brand palette
C_T1  = "#164679"
C_T2  = "#4a90d9"
C_T3  = "#a8c8f0"
C_BG  = "#f8f9fc"
C_MUTED = "#6b7280"

DPI = 300

# ── Load data ─────────────────────────────────────────────────────────────────

def load_clusters():
    with open(SOURCE) as f:
        raw = json.load(f)
    return raw if isinstance(raw, list) else list(raw.values())

# ── F1: Decision Tree ──────────────────────────────────────────────────────────

def figure_f1(clusters):
    print("F1: Decision tree ...")
    tier_counts = Counter(c.get("t") for c in clusters)
    t1, t2, t3 = tier_counts[1], tier_counts[2], tier_counts[3]

    fig, ax = plt.subplots(figsize=(4.72, 5.5))  # ~120mm wide
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.axis("off")
    fig.patch.set_facecolor("white")

    def box(x, y, text, color, width=2.8, height=0.65, fontsize=8.5, bold=False):
        rect = mpatches.FancyBboxPatch(
            (x - width/2, y - height/2), width, height,
            boxstyle="round,pad=0.08", linewidth=0.8,
            edgecolor="#cccccc", facecolor=color
        )
        ax.add_patch(rect)
        weight = "bold" if bold else "normal"
        ax.text(x, y, text, ha="center", va="center",
                fontsize=fontsize, fontweight=weight, wrap=True)

    def diamond(x, y, text, width=3.2, height=0.9, fontsize=8):
        pts = np.array([(x, y+height/2), (x+width/2, y), (x, y-height/2), (x-width/2, y)])
        patch = plt.Polygon(pts, closed=True, linewidth=0.8,
                             edgecolor="#888888", facecolor="#f0f4fa")
        ax.add_patch(patch)
        ax.text(x, y, text, ha="center", va="center", fontsize=fontsize,
                style="italic", color="#374151")

    def arrow(x1, y1, x2, y2, label="", label_side="right"):
        ax.annotate("", xy=(x2, y2), xytext=(x1, y1),
                    arrowprops=dict(arrowstyle="-|>", color="#555555", lw=0.8))
        mx, my = (x1+x2)/2, (y1+y2)/2
        if label:
            dx = 0.18 if label_side == "right" else -0.18
            ax.text(mx+dx, my, label, fontsize=7, color="#555555",
                    ha="left" if label_side == "right" else "right", va="center")

    # Root node
    diamond(5, 9.2, "Warehouse-club\npresent?")
    diamond(5, 7.2, "Hypermarket\npresent?")
    diamond(5, 5.2, "Hardware\npresent?")

    arrow(5, 8.75, 5, 7.65, "No", "right")
    arrow(5, 6.75, 5, 5.65, "No", "right")

    # T1 from warehouse-club path
    box(2.2, 7.8, f"→ check hypermarket", "#e8f0fb", width=2.8, height=0.55, fontsize=7.5)
    arrow(3.6, 9.2, 2.6, 8.08, "Yes", "left")
    box(2.2, 6.9, f"T1.a  (N={t1:,})", C_T1, width=2.0, height=0.6, fontsize=8.5, bold=True)
    ax.text(2.2, 6.9, f"\nHyper+HW+PC", ha="center", va="center", fontsize=6.5, color="white")
    ax.patches[-1].set_facecolor(C_T1)
    # Simpler: just leaf boxes
    # Redo with cleaner layout
    ax.cla()
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.axis("off")

    # Nodes (top-down)
    # Q1: Price Club present?
    diamond(5, 9.0, "Price club\npresent?")
    # Q2: Hypermarket present? (follows "no" from Q1)
    diamond(5, 6.8, "Hypermarket\n+ Hardware?")
    # Q3: ≥3 categories? (follows "no" from Q2)
    diamond(5, 4.6, "≥ 3 anchor\ncategories?")

    # Arrows: Q1→Q2 (No), Q2→Q3 (No)
    arrow(5, 8.55, 5, 7.25, "No")
    arrow(5, 6.35, 5, 5.05, "No")

    # T1 leaf (Yes from Q1 + hypermarket branch)
    box(2.0, 8.0, f"T1   N={t1:,}", C_T1, width=2.2, height=0.75, fontsize=9, bold=True)
    arrow(3.6, 9.0, 3.1, 8.0, "Yes", "left")

    # T2 leaf (No from Q1, Yes from Q2)
    box(7.8, 6.8, f"T2   N={t2:,}", C_T2, width=2.2, height=0.75, fontsize=9, bold=True)
    arrow(6.6, 6.8, 6.9, 6.8, "Yes")

    # T1 from Q3 Yes
    box(2.0, 4.6, f"T1   N={t1:,}", C_T1, width=2.2, height=0.75, fontsize=9, bold=True)
    arrow(3.6, 4.6, 3.1, 4.6, "Yes", "left")

    # T3 leaf (No from Q3)
    box(5.0, 2.5, f"T3   N={t3:,}", C_T3, width=2.2, height=0.75, fontsize=9, bold=True)
    arrow(5, 4.15, 5, 2.88, "No")

    # Legend note
    ax.text(5, 1.4, "T1 = Tripartite anchor composition  ·  T2 = Hypermarket + Hardware  ·  T3 = Other co-location",
            ha="center", va="center", fontsize=6.5, color=C_MUTED, style="italic")
    ax.text(5, 0.9, f"Phase 22 data  ·  N={len(clusters):,} clusters  ·  13 countries",
            ha="center", va="center", fontsize=6.5, color=C_MUTED)

    fig.tight_layout(pad=0.3)
    out_svg = FIG_DIR / "F1-decision-tree.svg"
    out_png = FIG_DIR / "F1-decision-tree.png"
    fig.savefig(out_svg, format="svg", bbox_inches="tight")
    fig.savefig(out_png, dpi=DPI, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print(f"  → {out_png}")


# ── F2: DBSCAN Schematic ───────────────────────────────────────────────────────

def figure_f2(clusters):
    print("F2: DBSCAN schematic ...")
    # Find a T1 tight cluster in Alberta (lat 51-54, lon -115 to -112)
    example = None
    for c in clusters:
        if (c.get("t") == 1 and c.get("tight") == 1
                and 51 < float(c.get("lat", 0)) < 54
                and -115 < float(c.get("lon", 0)) < -112):
            example = c
            break
    if not example:
        # Fall back: any T1 tight cluster
        example = next((c for c in clusters if c.get("t") == 1 and c.get("tight") == 1), clusters[0])

    fig, axes = plt.subplots(1, 2, figsize=(7.48, 3.5))  # 190mm wide
    fig.patch.set_facecolor("white")

    # ── Left: abstract DBSCAN diagram ──
    ax = axes[0]
    ax.set_facecolor(C_BG)
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.set_aspect("equal")
    ax.axis("off")
    ax.set_title("Pass 1: DBSCAN core-point concept", fontsize=8, pad=6, color="#374151")

    # Core points (filled blue)
    cores = [(5, 5), (4.2, 6.1), (5.8, 5.9), (5.1, 4.0)]
    # Border points (hollow)
    borders = [(3.0, 5.5), (6.8, 6.5), (5.9, 3.2)]
    # Noise (×)
    noise = [(1.5, 2.0), (8.5, 8.5), (1.0, 8.0), (9.0, 1.5)]

    # Draw ε radius circles around first core point
    for r, alpha, lw in [(1.8, 0.12, 0.8), (3.0, 0.07, 0.6)]:
        circle = plt.Circle(cores[0], r, color=C_T2, fill=True, alpha=alpha, linewidth=lw, linestyle="--", edgecolor=C_T2)
        ax.add_patch(circle)

    ax.text(cores[0][0]+1.85, cores[0][1]+0.1, "ε₁=1 km", fontsize=6.5, color=C_T2)
    ax.text(cores[0][0]+3.0, cores[0][1]-0.2, "ε₂=3 km", fontsize=6.5, color="#888888")

    for p in cores:
        ax.plot(*p, "o", color=C_T1, markersize=9, zorder=5)
    for p in borders:
        ax.plot(*p, "o", color=C_T2, markersize=8, markerfacecolor="white",
                markeredgecolor=C_T2, markeredgewidth=1.4, zorder=5)
    for p in noise:
        ax.plot(*p, "x", color="#9ca3af", markersize=8, markeredgewidth=1.8, zorder=5)

    # Legend
    leg_items = [
        mlines.Line2D([], [], marker="o", color=C_T1, linestyle="None", markersize=7, label="Core point (anchor)"),
        mlines.Line2D([], [], marker="o", color=C_T2, linestyle="None", markersize=7,
                      markerfacecolor="white", markeredgecolor=C_T2, markeredgewidth=1.4, label="Border point"),
        mlines.Line2D([], [], marker="x", color="#9ca3af", linestyle="None", markersize=7,
                      markeredgewidth=1.5, label="Noise (isolated store)"),
    ]
    ax.legend(handles=leg_items, loc="lower right", fontsize=6, framealpha=0.85,
              edgecolor="#cccccc")

    # ── Right: real cluster example ──
    ax2 = axes[1]
    ax2.set_facecolor(C_BG)
    ax2.set_aspect("equal")

    members = example.get("members", [])
    lats = [m["lat"] for m in members]
    lons = [m["lon"] for m in members]
    cats  = [m.get("category", "other") for m in members]
    names = [m.get("name", "") for m in members]
    span  = float(example.get("span", 0))

    cat_color = {
        "hypermarket": C_T1, "hardware": C_T2, "price_club": "#f59e0b",
        "lifestyle": "#10b981", "electronics": "#8b5cf6",
        "sport": "#ef4444", "medical": "#6b7280", "education": "#6b7280",
    }

    for lat, lon, cat, name in zip(lats, lons, cats, names):
        color = cat_color.get(cat, "#9ca3af")
        ax2.plot(lon, lat, "o", color=color, markersize=10, zorder=5)
        ax2.text(lon, lat + 0.003, name, fontsize=5.5, ha="center", va="bottom",
                 color="#374151", zorder=6)

    # span_km annotation arrow (longest pair)
    if len(lats) >= 2:
        # find furthest pair
        best_d, best_pair = 0, (0, 1)
        for i in range(len(lats)):
            for j in range(i+1, len(lats)):
                d = math.hypot(lats[i]-lats[j], lons[i]-lons[j])
                if d > best_d:
                    best_d, best_pair = d, (i, j)
        i, j = best_pair
        ax2.annotate("", xy=(lons[j], lats[j]), xytext=(lons[i], lats[i]),
                     arrowprops=dict(arrowstyle="<->", color="#555555", lw=1.0))
        mx = (lons[i]+lons[j])/2 + 0.003
        my = (lats[i]+lats[j])/2
        ax2.text(mx, my, f"span = {span:.2f} km", fontsize=6.5, color="#374151",
                 ha="left", va="center")

    pad_lat = max((max(lats)-min(lats))*0.4, 0.01)
    pad_lon = max((max(lons)-min(lons))*0.4, 0.01)
    ax2.set_xlim(min(lons)-pad_lon, max(lons)+pad_lon)
    ax2.set_ylim(min(lats)-pad_lat, max(lats)+pad_lat)
    ax2.set_xlabel("Longitude", fontsize=7, color=C_MUTED)
    ax2.set_ylabel("Latitude",  fontsize=7, color=C_MUTED)
    ax2.tick_params(labelsize=6)
    loc = example.get("mkt", example.get("rm", ""))
    ax2.set_title(f"Pass 2: T1 cluster — {loc}", fontsize=8, pad=6, color="#374151")

    # Anchor category legend
    cat_seen = set(cats) - {"medical", "education"}
    handles = [mpatches.Patch(color=cat_color.get(c, "#9ca3af"), label=c.replace("_", " ").title())
               for c in sorted(cat_seen)]
    ax2.legend(handles=handles, loc="lower right", fontsize=6, framealpha=0.85,
               edgecolor="#cccccc")

    fig.suptitle("Two-Pass DBSCAN: Algorithm concept (left) and worked example (right)",
                 fontsize=9, y=1.01, color="#1f2937")
    fig.tight_layout(pad=0.5)
    out = FIG_DIR / "F2-dbscan-schematic.png"
    fig.savefig(out, dpi=DPI, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print(f"  → {out}")


# ── F3: Continental Cluster Map ────────────────────────────────────────────────

def figure_f3(clusters):
    print("F3: Continental map (downloading Natural Earth boundaries) ...")
    import geopandas as gpd
    from shapely.geometry import Point

    # Load world boundaries (110m adequate for overview)
    NE_URL = "https://naciscdn.org/naturalearth/110m/cultural/ne_110m_admin_0_countries.zip"
    try:
        world = gpd.read_file(NE_URL)
    except Exception as e:
        print(f"  Natural Earth download failed: {e}")
        print("  Skipping F3.")
        return

    # Build GeoDataFrame from clusters
    tier_map = {1: "T1", 2: "T2", 3: "T3"}
    points = []
    for c in clusters:
        tier = int(c.get("t", 0))
        cont = c.get("cont", "")
        if cont not in ("NA", "EU"):
            continue
        points.append({
            "geometry": Point(float(c["lon"]), float(c["lat"])),
            "tier": tier_map.get(tier, "T3"),
            "span": float(c.get("span", 1.0)),
            "cont": cont,
        })
    gdf = gpd.GeoDataFrame(points, crs="EPSG:4326")

    tier_color = {"T1": C_T1, "T2": C_T2, "T3": C_T3}
    tier_order = ["T1", "T2", "T3"]

    fig, axes = plt.subplots(1, 2, figsize=(7.48, 3.6))
    fig.patch.set_facecolor("white")

    # NA panel — Albers Equal Area Conic (EPSG:5070)
    na_crs = "EPSG:5070"
    ax_na = axes[0]
    na_world = world.to_crs(na_crs)
    na_world[na_world["CONTINENT"].isin(["North America"])].plot(
        ax=ax_na, color="#e8eef4", edgecolor="#aaaaaa", linewidth=0.3)
    na_pts = gdf[gdf["cont"] == "NA"].to_crs(na_crs)
    for tier in tier_order:
        sub = na_pts[na_pts["tier"] == tier]
        sizes = np.clip(sub["span"].values * 1.5, 2, 10)
        ax_na.scatter(sub.geometry.x, sub.geometry.y, c=tier_color[tier],
                      s=sizes, alpha=0.6, linewidths=0, label=tier, zorder=5)
    ax_na.set_xlim(-3.2e6, 3.0e6)
    ax_na.set_ylim(-0.5e6, 4.5e6)
    ax_na.set_title("North America  (Albers Equal Area Conic)", fontsize=8, pad=5)
    ax_na.set_axis_off()

    # EU panel — ETRS89-LAEA (EPSG:3035)
    eu_crs = "EPSG:3035"
    ax_eu = axes[1]
    eu_world = world.to_crs(eu_crs)
    eu_world[eu_world["CONTINENT"] == "Europe"].plot(
        ax=ax_eu, color="#e8eef4", edgecolor="#aaaaaa", linewidth=0.3)
    eu_pts = gdf[gdf["cont"] == "EU"].to_crs(eu_crs)
    for tier in tier_order:
        sub = eu_pts[eu_pts["tier"] == tier]
        sizes = np.clip(sub["span"].values * 1.5, 2, 10)
        ax_eu.scatter(sub.geometry.x, sub.geometry.y, c=tier_color[tier],
                      s=sizes, alpha=0.6, linewidths=0, label=tier, zorder=5)
    ax_eu.set_xlim(2.5e6, 6.5e6)
    ax_eu.set_ylim(1.3e6, 5.5e6)
    ax_eu.set_title("Europe  (ETRS89 Lambert Azimuthal Equal Area)", fontsize=8, pad=5)
    ax_eu.set_axis_off()

    # Shared legend
    handles = [mpatches.Patch(color=tier_color[t], label=t) for t in tier_order]
    fig.legend(handles=handles, loc="lower center", ncol=3, fontsize=8,
               title="Tier (dot size ∝ span_km)", title_fontsize=7,
               framealpha=0.9, edgecolor="#cccccc", bbox_to_anchor=(0.5, -0.04))

    fig.suptitle(
        f"Retail anchor co-location clusters — {len(clusters):,} clusters, 13 countries",
        fontsize=9, y=1.01, color="#1f2937"
    )
    fig.tight_layout(pad=0.3)
    out = FIG_DIR / "F3-continental-map.png"
    fig.savefig(out, dpi=DPI, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print(f"  → {out}")


# ── F4: Per-Country T1 Share + Count ──────────────────────────────────────────

def figure_f4(clusters):
    print("F4: Country bars ...")
    NA = {"US", "CA", "MX"}

    by_country: dict = defaultdict(lambda: {"T1": 0, "T2": 0, "T3": 0, "total": 0})
    for c in clusters:
        iso  = c.get("iso", "")
        tier = {1: "T1", 2: "T2", 3: "T3"}.get(int(c.get("t", 0)), "T3")
        by_country[iso][tier]   += 1
        by_country[iso]["total"] += 1

    rows = []
    for iso, d in by_country.items():
        tot = d["total"]
        if tot == 0:
            continue
        rows.append({
            "iso":    iso,
            "t1":     d["T1"],
            "share":  d["T1"] / tot * 100,
            "region": "NA" if iso in NA else "EU",
        })

    rows.sort(key=lambda r: r["share"], reverse=True)
    isos   = [r["iso"] for r in rows]
    counts = [r["t1"] for r in rows]
    shares = [r["share"] for r in rows]
    colors = [C_T1 if r["region"] == "NA" else C_T2 for r in rows]

    na_mean_count = sum(r["t1"]   for r in rows if r["region"]=="NA") / max(sum(1 for r in rows if r["region"]=="NA"), 1)
    eu_mean_count = sum(r["t1"]   for r in rows if r["region"]=="EU") / max(sum(1 for r in rows if r["region"]=="EU"), 1)
    na_mean_share = sum(r["share"] for r in rows if r["region"]=="NA") / max(sum(1 for r in rows if r["region"]=="NA"), 1)
    eu_mean_share = sum(r["share"] for r in rows if r["region"]=="EU") / max(sum(1 for r in rows if r["region"]=="EU"), 1)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(7.48, 4.5))
    fig.patch.set_facecolor("white")
    y = np.arange(len(isos))

    ax1.barh(y, counts, color=colors, alpha=0.85, edgecolor="none", height=0.7)
    ax1.axvline(na_mean_count, color=C_T1, linestyle="--", linewidth=1.0, alpha=0.7, label=f"NA mean ({na_mean_count:.0f})")
    ax1.axvline(eu_mean_count, color=C_T2, linestyle="--", linewidth=1.0, alpha=0.7, label=f"EU mean ({eu_mean_count:.0f})")
    ax1.set_yticks(y); ax1.set_yticklabels(isos, fontsize=8)
    ax1.set_xlabel("T1 cluster count", fontsize=8)
    ax1.set_title("T1 Count by Country", fontsize=9, pad=6)
    ax1.legend(fontsize=7, framealpha=0.85)
    ax1.tick_params(axis="x", labelsize=7)
    ax1.spines[["top","right"]].set_visible(False)

    ax2.barh(y, shares, color=colors, alpha=0.85, edgecolor="none", height=0.7)
    ax2.axvline(na_mean_share, color=C_T1, linestyle="--", linewidth=1.0, alpha=0.7, label=f"NA mean ({na_mean_share:.1f}%)")
    ax2.axvline(eu_mean_share, color=C_T2, linestyle="--", linewidth=1.0, alpha=0.7, label=f"EU mean ({eu_mean_share:.1f}%)")
    ax2.set_yticks(y); ax2.set_yticklabels([], fontsize=8)
    ax2.set_xlabel("T1 share of country total (%)", fontsize=8)
    ax2.set_title("T1 Share by Country", fontsize=9, pad=6)
    ax2.legend(fontsize=7, framealpha=0.85)
    ax2.tick_params(axis="x", labelsize=7)
    ax2.spines[["top","right"]].set_visible(False)

    # Region legend
    na_patch = mpatches.Patch(color=C_T1, alpha=0.85, label="North America")
    eu_patch = mpatches.Patch(color=C_T2, alpha=0.85, label="Europe")
    fig.legend(handles=[na_patch, eu_patch], loc="lower center", ncol=2, fontsize=8,
               framealpha=0.9, edgecolor="#cccccc", bbox_to_anchor=(0.5, -0.03))

    fig.suptitle("T1 Co-location Cluster Distribution by Country — Phase 22",
                 fontsize=9, y=1.01, color="#1f2937")
    fig.tight_layout(pad=0.5)
    out = FIG_DIR / "F4-country-bars.png"
    fig.savefig(out, dpi=DPI, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print(f"  → {out}")


# ── F5: Span_km Violin ────────────────────────────────────────────────────────

def figure_f5(clusters):
    print("F5: Span violin ...")

    spans_by_tier: dict = {"T1": [], "T2": [], "T3": []}
    for c in clusters:
        tier = {1: "T1", 2: "T2", 3: "T3"}.get(int(c.get("t", 0)), "T3")
        s = float(c.get("span", 0))
        if s > 0:
            spans_by_tier[tier].append(s)

    t1s = np.array(spans_by_tier["T1"])
    t2s = np.array(spans_by_tier["T2"])
    t3s = np.array(spans_by_tier["T3"])

    kw_stat, kw_p = stats.kruskal(t1s, t2s, t3s)
    print(f"  Kruskal-Wallis H={kw_stat:.2f}, p={kw_p:.2e}")

    import pandas as pd
    records = []
    for tier, spans in spans_by_tier.items():
        for s in spans:
            records.append({"Tier": tier, "span_km": s})
    df = pd.DataFrame(records)

    fig, ax = plt.subplots(figsize=(4.5, 4.5))
    fig.patch.set_facecolor("white")
    ax.set_facecolor(C_BG)

    palette = {"T1": C_T1, "T2": C_T2, "T3": C_T3}
    sns.violinplot(data=df, x="Tier", y="span_km", palette=palette,
                   inner=None, cut=0, linewidth=0.8, ax=ax, order=["T1","T2","T3"])
    sns.stripplot(data=df.sample(min(800, len(df)), random_state=42),
                  x="Tier", y="span_km", palette=palette,
                  size=1.5, alpha=0.25, jitter=True, ax=ax, order=["T1","T2","T3"])
    sns.boxplot(data=df, x="Tier", y="span_km", palette=palette,
                width=0.12, showcaps=True, boxprops=dict(alpha=0.7),
                whiskerprops=dict(linewidth=0.8), medianprops=dict(color="white", linewidth=1.5),
                fliersize=0, ax=ax, order=["T1","T2","T3"])

    ax.set_yscale("log")
    ax.set_ylabel("Cluster span (km, log scale)", fontsize=9)
    ax.set_xlabel("Tier", fontsize=9)
    ax.tick_params(labelsize=8)
    ax.spines[["top","right"]].set_visible(False)

    p_fmt = f"{kw_p:.2e}" if kw_p < 0.001 else f"{kw_p:.4f}"
    ax.set_title(
        f"Span Distribution by Tier\n"
        f"Kruskal–Wallis  H = {kw_stat:.1f},  p = {p_fmt}  (N={len(df):,})",
        fontsize=8.5, pad=8, color="#1f2937"
    )
    ax.text(0.01, 0.01,
            f"Medians:  T1={np.median(t1s):.2f} km  ·  T2={np.median(t2s):.2f} km  ·  T3={np.median(t3s):.2f} km",
            transform=ax.transAxes, fontsize=6.5, color=C_MUTED, va="bottom")

    fig.tight_layout(pad=0.5)
    out = FIG_DIR / "F5-span-violin.png"
    fig.savefig(out, dpi=DPI, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print(f"  → {out}")
    print(f"  Caption: Kruskal-Wallis H={kw_stat:.2f}, p={kw_p:.3e}")
    print(f"  Medians: T1={np.median(t1s):.2f} km / T2={np.median(t2s):.2f} km / T3={np.median(t3s):.2f} km")


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    print(f"Loading {SOURCE} ...")
    clusters = load_clusters()
    print(f"  {len(clusters):,} clusters\n")

    figure_f1(clusters)
    figure_f2(clusters)
    figure_f3(clusters)
    figure_f4(clusters)
    figure_f5(clusters)

    print(f"\nAll figures written to {FIG_DIR}/")


if __name__ == "__main__":
    main()
