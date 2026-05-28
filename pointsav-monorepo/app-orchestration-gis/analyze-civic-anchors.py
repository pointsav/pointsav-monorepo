#!/usr/bin/env python3
"""
analyze-civic-anchors.py — Hospital + university anchor analysis for tier classification.

Sections:
  1. Current civic co-presence by tier × continent
  2. Rule simulations (A: civic required for T1; B: civic unlocks new T2; C: hybrid)
  3. Data quality breakdown of civic members in source JSONL

Input:  clusters-meta.json (deployed gateway copy)
        cleansed-civic-osm.jsonl (deployment service-places)
Output: work/civic-anchor-analysis.txt
"""
import json
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

CLUSTERS_SRC = Path(
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)
CIVIC_SRC = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1/service-places/cleansed-civic-osm.jsonl"
)
OUT = WORK_DIR / "civic-anchor-analysis.txt"

RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "electronics", "sport"}


def has_cat(cluster, cat):
    return any(m.get("category") == cat for m in cluster.get("members", []))


def has_civic(cluster):
    return has_cat(cluster, "medical") or has_cat(cluster, "education")


def retail_cats_of(cluster):
    return {m.get("category") for m in cluster.get("members", [])} & RETAIL_CATS


def main():
    print(f"Loading {CLUSTERS_SRC} ...")
    raw = json.loads(CLUSTERS_SRC.read_text())
    clusters = raw if isinstance(raw, list) else list(raw.values())
    print(f"  {len(clusters):,} clusters")

    lines = []

    def pr(s=""):
        print(s)
        lines.append(s)

    t1_total = sum(1 for c in clusters if c.get("t") == 1)
    t2_total = sum(1 for c in clusters if c.get("t") == 2)
    t3_total = sum(1 for c in clusters if c.get("t") == 3)

    pr("=" * 72)
    pr("CIVIC ANCHOR ANALYSIS — Hospitals + Universities in T1/T2/T3")
    pr("=" * 72)
    pr(f"Clusters loaded: {len(clusters):,}  (T1={t1_total:,}  T2={t2_total:,}  T3={t3_total:,})")
    pr()

    # ------------------------------------------------------------------
    pr("SECTION 1 — Civic co-presence by tier × continent")
    pr("-" * 72)
    pr("(% of clusters in that tier/continent that have ≥1 nearby civic POI)")
    pr()

    for cont in ("NA", "EU", "ALL"):
        cset = clusters if cont == "ALL" else [c for c in clusters if c.get("cont") == cont]
        if not cset:
            continue
        pr(f"  {cont}  ({len(cset):,} clusters)")
        pr(f"  {'Tier':<5} {'N':>6}  {'hospital%':>9}  {'university%':>11}  {'both%':>6}  {'either%':>7}")
        pr(f"  {'-'*4}  {'-'*5}  {'-'*9}  {'-'*11}  {'-'*6}  {'-'*7}")
        for tier in (1, 2, 3):
            ts = [c for c in cset if c.get("t") == tier]
            if not ts:
                continue
            n = len(ts)
            med = sum(1 for c in ts if has_cat(c, "medical"))
            edu = sum(1 for c in ts if has_cat(c, "education"))
            both = sum(1 for c in ts if has_cat(c, "medical") and has_cat(c, "education"))
            either = sum(1 for c in ts if has_civic(c))
            pr(
                f"  T{tier}    {n:>6}  {med/n*100:>8.1f}%  {edu/n*100:>10.1f}%  "
                f"{both/n*100:>5.1f}%  {either/n*100:>6.1f}%"
            )
        pr()

    # ------------------------------------------------------------------
    pr("=" * 72)
    pr("SECTION 2 — Rule simulations")
    pr("=" * 72)

    # RULE A — Civic required for T1 (restrictive)
    pr()
    pr("RULE A — Civic proximity REQUIRED for T1 (restrictive)")
    pr(
        "  A cluster keeps T1 only if it has ≥1 hospital or university within 5 km.\n"
        "  T1 clusters without civic proximity → demoted to T2."
    )
    pr()
    demoted = [c for c in clusters if c.get("t") == 1 and not has_civic(c)]
    t1_a = t1_total - len(demoted)
    t2_a = t2_total + len(demoted)
    t3_a = t3_total

    pr(f"  T1 clusters demoted: {len(demoted):,} of {t1_total:,} ({len(demoted)/t1_total*100:.1f}%)")
    pr()
    pr("  Demotions by country (Top 15):")
    for iso, n in Counter(c.get("iso", "?") for c in demoted).most_common(15):
        total_iso = sum(1 for c in clusters if c.get("iso") == iso and c.get("t") == 1)
        pr(f"    {iso:>3}: {n:>4}  of {total_iso:>4} T1  ({n/total_iso*100:.0f}%)")
    pr()
    pr(f"  New distribution: T1={t1_a:,}  T2={t2_a:,}  T3={t3_a:,}")
    pr(f"  Delta:            T1{t1_a-t1_total:+,}  T2{t2_a-t2_total:+,}  T3{t3_a-t3_total:+,}")

    # RULE B — Civic as new T2 pathway (expansive)
    pr()
    pr("-" * 72)
    pr("RULE B — Civic as new T2 PATHWAY (expansive)")
    pr(
        "  New T2 path: hypermarket + (hospital OR university), even without hardware.\n"
        "  Targets current T3 clusters that have hypermarket + civic but no hardware."
    )
    pr()
    t3_promoted = [
        c for c in clusters
        if c.get("t") == 3
        and "hypermarket" in retail_cats_of(c)
        and "hardware" not in retail_cats_of(c)
        and has_civic(c)
    ]
    t1_b = t1_total
    t2_b = t2_total + len(t3_promoted)
    t3_b = t3_total - len(t3_promoted)

    pr(
        f"  T3 → T2 promotions: {len(t3_promoted):,} of {t3_total:,} "
        f"({len(t3_promoted)/t3_total*100:.1f}%)"
    )
    pr()
    pr("  Promotions by country (Top 15):")
    for iso, n in Counter(c.get("iso", "?") for c in t3_promoted).most_common(15):
        total_iso = sum(1 for c in clusters if c.get("iso") == iso and c.get("t") == 3)
        pr(f"    {iso:>3}: {n:>4}  of {total_iso:>4} T3  ({n/total_iso*100:.0f}%)")
    pr()
    pr("  Promotions by continent:")
    for cont in ("NA", "EU"):
        subset = [c for c in t3_promoted if c.get("cont") == cont]
        t3_cont = sum(1 for c in clusters if c.get("cont") == cont and c.get("t") == 3)
        pr(f"    {cont}: {len(subset):,} of {t3_cont:,} T3 promoted ({len(subset)/max(1,t3_cont)*100:.1f}%)")
    pr()
    # T2 clusters that have civic + hypermarket — if civic as third anchor, some could rise to T1
    t2_civic_hyper_hw = [
        c for c in clusters
        if c.get("t") == 2 and has_civic(c)
    ]
    pr(
        f"  Note: {len(t2_civic_hyper_hw):,} current T2 clusters have civic proximity.\n"
        f"  If civic counted as a tertiary anchor (= T1.a rule), some could qualify T1."
    )
    pr()
    pr(f"  New distribution: T1={t1_b:,}  T2={t2_b:,}  T3={t3_b:,}")
    pr(f"  Delta:            T1{t1_b-t1_total:+,}  T2{t2_b-t2_total:+,}  T3{t3_b-t3_total:+,}")

    # RULE C — Hybrid
    pr()
    pr("-" * 72)
    pr("RULE C — Hybrid: Rule A (T1 requires civic) + Rule B (civic unlocks T2)")
    pr()
    t1_c = t1_a
    t2_c = t2_total + len(demoted) + len(t3_promoted)
    t3_c = t3_b
    pr(f"  New distribution: T1={t1_c:,}  T2={t2_c:,}  T3={t3_c:,}")
    pr(f"  Delta:            T1{t1_c-t1_total:+,}  T2{t2_c-t2_total:+,}  T3{t3_c-t3_total:+,}")

    # ------------------------------------------------------------------
    pr()
    pr("=" * 72)
    pr("SECTION 3 — Civic source data quality (cleansed-civic-osm.jsonl)")
    pr("-" * 72)

    if not CIVIC_SRC.exists():
        pr(f"  WARNING: {CIVIC_SRC} not found — skipping quality section")
    else:
        print(f"Loading {CIVIC_SRC} ...")
        recs = []
        with open(CIVIC_SRC) as f:
            for line in f:
                line = line.strip()
                if line:
                    try:
                        recs.append(json.loads(line))
                    except Exception:
                        pass
        print(f"  {len(recs):,} records")

        hosp = [r for r in recs if r.get("category_id") == "hospital"]
        univ = [r for r in recs if r.get("category_id") == "university"]

        pr(f"  Total records: {len(recs):,}  (hospitals={len(hosp):,}  universities={len(univ):,})")
        pr()

        # Hospital quality breakdown
        pr("  HOSPITALS:")
        h_beds_emerg = sum(
            1 for r in hosp
            if (r.get("bed_count") or 0) >= 150 and r.get("emergency") == "yes"
        )
        h_emerg_only = sum(
            1 for r in hosp
            if r.get("emergency") == "yes" and (r.get("bed_count") or 0) < 150
        )
        h_beds_only = sum(
            1 for r in hosp
            if (r.get("bed_count") or 0) >= 150 and r.get("emergency") != "yes"
        )
        h_regional = sum(1 for r in hosp if r.get("hospital_tier") == "regional")
        h_district = sum(1 for r in hosp if r.get("hospital_tier") == "district")
        h_wikidata = sum(1 for r in hosp if r.get("wikidata_id"))
        h_beds_any = sum(1 for r in hosp if r.get("bed_count"))

        pr(f"    Regional tier:       {h_regional:>6,}  ({h_regional/len(hosp)*100:.1f}%)")
        pr(f"    District tier:       {h_district:>6,}  ({h_district/len(hosp)*100:.1f}%)")
        pr(f"    Has bed count:       {h_beds_any:>6,}  ({h_beds_any/len(hosp)*100:.1f}%)")
        pr(f"    Has emergency=yes:   {h_beds_any+h_emerg_only:>6,}")  # recount properly
        h_emerg_yes = sum(1 for r in hosp if r.get("emergency") == "yes")
        pr(f"    Has emergency=yes:   {h_emerg_yes:>6,}  ({h_emerg_yes/len(hosp)*100:.1f}%)")
        pr(f"    Has Wikidata ID:     {h_wikidata:>6,}  ({h_wikidata/len(hosp)*100:.1f}%)")
        pr(f"    High conf (beds≥150 + emergency=yes): {h_beds_emerg:>6,}  ({h_beds_emerg/len(hosp)*100:.1f}%)")
        pr(f"    Med conf  (emergency=yes only):        {h_emerg_only:>6,}  ({h_emerg_only/len(hosp)*100:.1f}%)")
        pr(f"    Med conf  (beds≥150 only):             {h_beds_only:>6,}  ({h_beds_only/len(hosp)*100:.1f}%)")
        pr(
            f"    Low conf  (name-heuristic only):       "
            f"{len(hosp)-h_beds_emerg-h_emerg_only-h_beds_only:>6,}"
        )
        pr()

        # University quality breakdown
        pr("  UNIVERSITIES:")
        u_regional = sum(1 for r in univ if r.get("university_tier") == "regional")
        u_small = sum(1 for r in univ if r.get("university_tier") == "small")
        u_wikidata = sum(1 for r in univ if r.get("wikidata_id"))
        u_ipeds = sum(
            1 for r in univ
            if r.get("iso_country_code") == "US" and r.get("confidence", 0) >= 0.95
        )

        pr(f"    Regional tier:       {u_regional:>6,}  ({u_regional/len(univ)*100:.1f}%)")
        pr(f"    Small tier:          {u_small:>6,}  ({u_small/len(univ)*100:.1f}%)")
        pr(f"    Has Wikidata ID:     {u_wikidata:>6,}  ({u_wikidata/len(univ)*100:.1f}%)")
        pr(f"    US high-conf:        {u_ipeds:>6,}  (confidence≥0.95; proxy for IPEDS enrichment)")
        pr()

        # Country breakdown
        pr("  HOSPITALS by country (Top 15):")
        for iso, n in Counter(r.get("iso_country_code", "?") for r in hosp).most_common(15):
            h_reg = sum(
                1 for r in hosp
                if r.get("iso_country_code") == iso and r.get("hospital_tier") == "regional"
            )
            pr(f"    {iso:>3}: {n:>5,}  regional={h_reg:>4,} ({h_reg/n*100:.0f}%)")
        pr()

        pr("  UNIVERSITIES by country (Top 15):")
        for iso, n in Counter(r.get("iso_country_code", "?") for r in univ).most_common(15):
            u_reg = sum(
                1 for r in univ
                if r.get("iso_country_code") == iso and r.get("university_tier") == "regional"
            )
            pr(f"    {iso:>3}: {n:>5,}  regional={u_reg:>4,} ({u_reg/n*100:.0f}%)")

    # ------------------------------------------------------------------
    pr()
    pr("=" * 72)
    pr("SUMMARY TABLE")
    pr("=" * 72)
    pr()
    pr(f"  {'Scenario':<40} {'T1':>6}  {'T2':>6}  {'T3':>6}")
    pr(f"  {'-'*39}  {'-'*5}  {'-'*5}  {'-'*5}")
    pr(f"  {'Current (Phase 22)':<40} {t1_total:>6,}  {t2_total:>6,}  {t3_total:>6,}")
    pr(f"  {'Rule A  (civic required for T1)':<40} {t1_a:>6,}  {t2_a:>6,}  {t3_a:>6,}")
    pr(f"  {'Rule B  (civic unlocks new T2 path)':<40} {t1_b:>6,}  {t2_b:>6,}  {t3_b:>6,}")
    pr(f"  {'Rule C  (A + B hybrid)':<40} {t1_c:>6,}  {t2_c:>6,}  {t3_c:>6,}")
    pr()
    pr("  Notes:")
    pr("  - Rule A makes T1 more exclusive (suburban retail parks without civic drop)")
    pr("  - Rule B targets NA T3 gap: hypermarket-anchored clusters near hospitals/universities")
    pr("  - Data quality is weakest for district hospitals (minimal OSM metadata)")
    pr("  - EU university quality gap: no enrollment data equivalent to US IPEDS")
    pr("  - Run crosscheck-civic-quality.py for OSM vs Overture source comparison")

    WORK_DIR.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n")
    print(f"\nWrote: {OUT}")


if __name__ == "__main__":
    main()
