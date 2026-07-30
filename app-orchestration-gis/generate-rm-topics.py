#!/usr/bin/env python3
"""
generate-rm-topics.py

Reads clusters-meta.json, groups clusters by Regional Market (rm field),
and emits one TOPIC draft markdown file per RM to .agent/drafts-outbound/.

Usage:
    python3 generate-rm-topics.py [--min-clusters N] [--iso XX] [--limit N]

Defaults:
    --min-clusters 2   (only emit RMs with 2+ co-locations)
    --iso              (filter to one country, e.g. US, DE, GB; default: all)
    --limit            (max topics to emit; default: unlimited)

Output files follow foundry-draft-v1 frontmatter schema.
"""

import argparse
import json
import os
import re
import sys
from collections import defaultdict
from datetime import date

CLUSTERS_META = os.path.join(
    os.path.dirname(__file__),
    'www', 'data', 'clusters-meta.json',
)

DRAFTS_DIR = os.path.join(
    os.path.dirname(__file__),
    '..', '..', '..', '.agent', 'drafts-outbound',
)

TIER_NAMES = {1: 'T1 Regional', 2: 'T2 District', 3: 'T3 Local'}

COUNTRY_NAMES = {
    'US': 'United States', 'CA': 'Canada', 'MX': 'Mexico',
    'GB': 'United Kingdom', 'DE': 'Germany', 'FR': 'France',
    'ES': 'Spain', 'IT': 'Italy', 'SE': 'Sweden', 'NO': 'Norway',
    'FI': 'Finland', 'DK': 'Denmark', 'GR': 'Greece', 'AT': 'Austria',
    'NL': 'Netherlands', 'PT': 'Portugal', 'PL': 'Poland',
}

def country_name(iso):
    return COUNTRY_NAMES.get(iso, iso or 'International')

def slugify(text):
    s = text.lower()
    s = re.sub(r'[^\w\s-]', '', s)
    s = re.sub(r'[\s_]+', '-', s)
    s = re.sub(r'-+', '-', s).strip('-')
    return s[:80]

def tier_badge(t):
    if t == 1: return 'T1 Regional'
    if t == 2: return 'T2 District'
    if t == 3: return 'T3 Local'
    return f'T{t}'

def compact_num(n):
    if n >= 1_000_000: return f'{n/1_000_000:.1f}M'
    if n >= 1_000: return f'{n/1_000:.0f}K'
    return str(n)

def render_topic(rm_key, clusters):
    """Render a single TOPIC draft markdown for a Regional Market."""
    # Derive RM-level metadata from first (best-ranked) cluster
    clusters_sorted = sorted(clusters, key=lambda c: (c.get('t', 4), -c.get('dr', 0)))
    rep = clusters_sorted[0]
    mkt_name  = rep.get('mkt', rm_key)
    mrgn      = rep.get('mrgn', '')
    metro     = rep.get('metro', '')
    iso       = rep.get('iso', '')
    cont      = rep.get('cont', '')
    country   = country_name(iso)

    t1 = [c for c in clusters if c.get('t') == 1]
    t2 = [c for c in clusters if c.get('t') == 2]
    t3 = [c for c in clusters if c.get('t') == 3]
    n  = len(clusters)

    geo_parts = [mkt_name]
    if mrgn and mrgn != mkt_name: geo_parts.append(mrgn)
    if metro and metro not in (mkt_name, mrgn): geo_parts.append(metro)
    geo_parts.append(country)
    geo_line  = ' · '.join(p for p in geo_parts if p)

    # Filename
    slug      = slugify(mkt_name)
    iso_lower = iso.lower()
    filename  = f'topic-rm-{iso_lower}-{slug}.draft.md'

    today = date.today().isoformat()

    # Cluster rows
    cluster_rows = []
    for c in clusters_sorted:
        t_name = tier_badge(c.get('t', 0))
        td     = c.get('td', '').replace('Hypermarket', 'HM').replace('Hardware', 'HW').replace('Price Club', 'PC')
        span   = f"{c.get('span', 0):.2f} km" if c.get('span') is not None else '—'
        dp     = f"{int(c.get('dp', 0))}th pctile" if c.get('dp') is not None else '—'
        members = c.get('members', [])
        anchor  = next((m['name'] for m in members if m.get('category') == 'hypermarket'), '')
        anchor  = anchor or next((m['name'] for m in members if True), '')
        cluster_rows.append(f'| {t_name} | {c.get("td","")} | {span} | {dp} | {anchor} |')

    cluster_table = '\n'.join(cluster_rows)

    content = f"""---
schema: foundry-draft-v1
artifact_type: TOPIC
title: "Regional Market: {mkt_name}"
draft_id: topic-rm-{iso_lower}-{slug}
target_path: content-wiki-documentation/markets/{iso_lower}/{slug}.md
language_protocol: PROSE-TOPIC
gateway: project-editorial
created: {today}
source_cluster: project-gis
source_data: clusters-meta.json Phase 15 build
bcsc_reviewed: false
research_trail_complete: false
references:
  - clusters-meta.json
  - colocation-tier-summary.json
cites: []
---

# Regional Market: {mkt_name}

**{geo_line}** — {n} co-location cluster{"s" if n != 1 else ""} identified.

---

## Market profile

| Attribute | Value |
|---|---|
| Market | {mkt_name} |
| Metro region | {mrgn or '—'} |
| Country | {country} |
| ISO | {iso} |
| Continent | {cont} |
| Total co-locations | {n} |
| T1 Regional | {len(t1)} |
| T2 District | {len(t2)} |
| T3 Local | {len(t3)} |

---

## Co-location inventory

| Tier | Composition | Span | Compactness | Anchor |
|---|---|---|---|---|
{cluster_table}

---

## Summary

{"This market contains a T1 Regional co-location — the highest tier, indicating the convergence of a hypermarket, hardware retailer, and price club or lifestyle anchor within a tight radius." if t1 else ""}
{"This market has " + str(len(t2)) + " T2 District co-location" + ("s" if len(t2) != 1 else "") + " — strong hypermarket + hardware or price club convergences." if t2 else ""}
{"" if not t3 else f"{len(t3)} T3 Local co-location{'s' if len(t3)!=1 else ''} — emerging retail groupings with two qualifying anchor categories."}

(draft-pending — substance follows in a future editorial pass)

---

## Research trail

- **Source**: clusters-meta.json Phase 15 build (2026-05-19), project-gis pipeline
- **Method**: Two-pass tight-first DBSCAN; tier determined by retail anchor composition only
- **Cluster count**: {n} co-location{"s" if n != 1 else ""} in `{rm_key}`
- **Validation**: Pipeline-generated; editorial review pending before publication
"""
    return filename, content


def main():
    parser = argparse.ArgumentParser(description='Generate RM TOPIC drafts')
    parser.add_argument('--min-clusters', type=int, default=2, help='Min co-locations per RM')
    parser.add_argument('--iso', type=str, default=None, help='Filter to one ISO country code')
    parser.add_argument('--limit', type=int, default=None, help='Max topics to emit')
    args = parser.parse_args()

    with open(CLUSTERS_META) as f:
        data = json.load(f)

    # Group by rm key
    by_rm = defaultdict(list)
    for c in data:
        rm = c.get('rm') or f'_solo_{c["id"]}'
        by_rm[rm].append(c)

    # Filter
    rms = [(k, v) for k, v in by_rm.items()
           if len(v) >= args.min_clusters
           and not k.startswith('_solo_')
           and (args.iso is None or any(c.get('iso') == args.iso for c in v))]

    # Sort by cluster count desc, then by T1 count desc
    rms.sort(key=lambda kv: (-len(kv[1]), -sum(1 for c in kv[1] if c.get('t') == 1)))

    if args.limit:
        rms = rms[:args.limit]

    os.makedirs(DRAFTS_DIR, exist_ok=True)

    written = 0
    for rm_key, clusters in rms:
        filename, content = render_topic(rm_key, clusters)
        path = os.path.join(DRAFTS_DIR, filename)
        with open(path, 'w') as f:
            f.write(content)
        iso_sample = clusters[0].get('iso', '?')
        print(f'  {filename}  ({len(clusters)} clusters, {iso_sample})')
        written += 1

    print(f'\n{written} TOPIC drafts written to {os.path.relpath(DRAFTS_DIR)}')


if __name__ == '__main__':
    main()
