#!/usr/bin/env python3
"""
score-regional-markets.py — Composite ranking for Top 400 NA and EU Regional Markets.

Scoring formula:
  score = tier_score × civic_multiplier × metro_multiplier × confidence_factor

  tier_score        = (t1 × 4) + (t2 × 2) + (t3 × 1)
  civic_multiplier  = 1.5 if any cluster member has a civic anchor category, else 1.0
  metro_multiplier  = clamp(dist_km_to_nearest_metro / 50.0, 0.5, 2.0)
  confidence_factor = 1.0 if mkt_conf == 'high' else 0.7

Outputs:
  work/top400-na.json  — top 400 NA Regional Markets ranked by score
  work/top400-eu.json  — top 400 EU Regional Markets ranked by score

Run from:  app-orchestration-gis/
  python3 score-regional-markets.py
"""

import json
import math
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
WORK_DIR = SCRIPT_DIR / 'work'
RM_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/regional-markets.json')
CLUSTERS_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')
AEC_PATH = WORK_DIR / 'DATA-aec-clusters.csv'

OUT_NA = WORK_DIR / 'top400-na.json'
OUT_EU = WORK_DIR / 'top400-eu.json'

CIVIC_CATEGORIES = {
    'healthcare', 'higher_education', 'medical', 'education',
    'hospital', 'university', 'college', 'clinic',
}

# Major NA metro centroids (lat, lon, name)
NA_METROS = [
    (40.7128, -74.0060, 'New York'),
    (34.0522, -118.2437, 'Los Angeles'),
    (41.8781, -87.6298, 'Chicago'),
    (29.7604, -95.3698, 'Houston'),
    (33.4484, -112.0740, 'Phoenix'),
    (39.9526, -75.1652, 'Philadelphia'),
    (32.7767, -96.7970, 'Dallas'),
    (29.4241, -98.4936, 'San Antonio'),
    (32.7157, -117.1611, 'San Diego'),
    (37.3382, -121.8863, 'San Jose'),
    (30.2672, -97.7431, 'Austin'),
    (30.3322, -81.6557, 'Jacksonville'),
    (32.7555, -97.3308, 'Fort Worth'),
    (39.9612, -82.9988, 'Columbus'),
    (35.2271, -80.8431, 'Charlotte'),
    (39.7684, -86.1581, 'Indianapolis'),
    (37.7749, -122.4194, 'San Francisco'),
    (47.6062, -122.3321, 'Seattle'),
    (39.7392, -104.9903, 'Denver'),
    (36.1627, -86.7816, 'Nashville'),
    (35.4676, -97.5164, 'Oklahoma City'),
    (36.1699, -115.1398, 'Las Vegas'),
    (45.5051, -122.6750, 'Portland'),
    (39.2904, -76.6122, 'Baltimore'),
    (33.7490, -84.3880, 'Atlanta'),
    (44.9778, -93.2650, 'Minneapolis'),
    (27.9506, -82.4572, 'Tampa'),
    (25.7617, -80.1918, 'Miami'),
    (35.7796, -78.6382, 'Raleigh'),
    (38.5816, -121.4944, 'Sacramento'),
    (38.6270, -90.1994, 'St. Louis'),
    (40.4406, -79.9959, 'Pittsburgh'),
    (39.1031, -84.5120, 'Cincinnati'),
    (41.4993, -81.6944, 'Cleveland'),
    (39.0997, -94.5786, 'Kansas City'),
    (28.5383, -81.3792, 'Orlando'),
    (40.7608, -111.8910, 'Salt Lake City'),
    (33.5207, -86.8025, 'Birmingham'),
    (37.5407, -77.4360, 'Richmond'),
    (42.8864, -78.8784, 'Buffalo'),
    (41.7658, -72.6851, 'Hartford'),
    (41.8240, -71.4128, 'Providence'),
    (35.1495, -90.0490, 'Memphis'),
    (43.0389, -76.1422, 'Syracuse'),
    (32.2226, -110.9747, 'Tucson'),
    (36.7378, -119.7871, 'Fresno'),
    (35.0853, -106.6056, 'Albuquerque'),
    (42.3601, -71.0589, 'Boston'),
    (42.3314, -83.0458, 'Detroit'),
    (43.6532, -79.3832, 'Toronto'),
    (45.5017, -73.5673, 'Montreal'),
    (49.2827, -123.1207, 'Vancouver'),
    (51.0447, -114.0719, 'Calgary'),
    (53.5461, -113.4938, 'Edmonton'),
    (45.4215, -75.6972, 'Ottawa'),
    (49.8951, -97.1384, 'Winnipeg'),
    (46.8139, -71.2080, 'Quebec City'),
    (19.4326, -99.1332, 'Mexico City'),
    (20.6597, -103.3496, 'Guadalajara'),
    (25.6866, -100.3161, 'Monterrey'),
    (19.0413, -98.2062, 'Puebla'),
]

# Major EU metro centroids (lat, lon, name)
EU_METROS = [
    (51.5074, -0.1278, 'London'),
    (48.8566, 2.3522, 'Paris'),
    (52.5200, 13.4050, 'Berlin'),
    (40.4168, -3.7038, 'Madrid'),
    (41.9028, 12.4964, 'Rome'),
    (44.4268, 26.1025, 'Bucharest'),
    (48.2082, 16.3738, 'Vienna'),
    (53.5753, 10.0153, 'Hamburg'),
    (47.4979, 19.0402, 'Budapest'),
    (52.2297, 21.0122, 'Warsaw'),
    (41.3851, 2.1734, 'Barcelona'),
    (48.1351, 11.5820, 'Munich'),
    (45.4654, 9.1859, 'Milan'),
    (50.0755, 14.4378, 'Prague'),
    (42.6977, 23.3219, 'Sofia'),
    (50.9333, 6.9500, 'Cologne'),
    (59.3293, 18.0686, 'Stockholm'),
    (40.8518, 14.2681, 'Naples'),
    (45.0703, 7.6869, 'Turin'),
    (52.3676, 4.9041, 'Amsterdam'),
    (43.2965, 5.3698, 'Marseille'),
    (39.4699, -0.3763, 'Valencia'),
    (50.0647, 19.9450, 'Kraków'),
    (50.1109, 8.6821, 'Frankfurt'),
    (37.3891, -5.9845, 'Seville'),
    (51.9225, 4.4792, 'Rotterdam'),
    (60.1699, 24.9384, 'Helsinki'),
    (37.9838, 23.7275, 'Athens'),
    (51.2217, 6.7762, 'Düsseldorf'),
    (48.7758, 9.1829, 'Stuttgart'),
    (51.5136, 7.4653, 'Dortmund'),
    (51.1079, 17.0385, 'Wrocław'),
    (38.7223, -9.1393, 'Lisbon'),
    (51.3397, 12.3731, 'Leipzig'),
    (50.8503, 4.3517, 'Brussels'),
    (51.4508, 7.0131, 'Essen'),
    (50.2649, 19.0238, 'Katowice'),
    (38.1157, 13.3615, 'Palermo'),
    (45.7640, 4.8357, 'Lyon'),
    (43.6047, 1.4442, 'Toulouse'),
    (43.2965, -0.3698, 'Nice'),
    (47.2184, -1.5536, 'Nantes'),
    (48.5734, 7.7521, 'Strasbourg'),
    (43.6108, 3.8767, 'Montpellier'),
    (44.8378, -0.5792, 'Bordeaux'),
    (50.6292, 3.0573, 'Lille'),
    (48.1173, -1.6778, 'Rennes'),
    (49.2583, 4.0317, 'Reims'),
    (49.4938, 0.1077, 'Le Havre'),
    (45.4338, 4.3903, 'Saint-Étienne'),
    (43.1242, 5.9280, 'Toulon'),
    (45.1885, 5.7245, 'Grenoble'),
    (47.3220, 5.0415, 'Dijon'),
    (47.4784, -0.5632, 'Angers'),
    (43.8367, 4.3601, 'Nîmes'),
    (43.5297, 5.4474, 'Aix-en-Provence'),
    (48.3905, -4.4860, 'Brest'),
    (47.9960, 0.1966, 'Le Mans'),
    (52.3758, 4.8975, 'The Hague'),
    (51.4416, 5.4697, 'Eindhoven'),
    (52.0907, 5.1214, 'Utrecht'),
]

NA_ISOS = {'US', 'CA', 'MX'}
EU_ISOS = {'GB', 'DE', 'FR', 'ES', 'IT', 'PL', 'NL', 'AT', 'PT', 'GR', 'IS', 'SE', 'DK', 'FI', 'NO'}


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = math.sin(dlat/2)**2 + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2)) * math.sin(dlon/2)**2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat, lon, metros):
    best_dist = float('inf')
    best_name = ''
    for mlat, mlon, name in metros:
        d = haversine(lat, lon, mlat, mlon)
        if d < best_dist:
            best_dist = d
            best_name = name
    return best_dist, best_name


def metro_multiplier(dist_km):
    return max(0.5, min(2.0, dist_km / 50.0))


def load_aec_data():
    aec = {}
    if not AEC_PATH.exists():
        return aec
    import csv
    with open(AEC_PATH) as f:
        for row in csv.DictReader(f):
            aec[row['cluster_id']] = {
                'ashrae_zone':     row.get('ashrae_zone', ''),
                'koppen_class':    row.get('koppen_class', ''),
                'eu_climate_zone': row.get('eu_climate_zone', ''),
                'ecoregion_name':  row.get('ecoregion_name', ''),
                'ecoregion_biome': row.get('ecoregion_biome', ''),
            }
    return aec


# ── Load data ──────────────────────────────────────────────────────────────────

print("Loading regional-markets.json...", flush=True)
with open(RM_PATH) as f:
    rms_raw = json.load(f)
print(f"  {len(rms_raw)} Regional Markets", flush=True)

print("Loading clusters-meta.json...", flush=True)
with open(CLUSTERS_PATH) as f:
    clusters_raw = json.load(f)
print(f"  {len(clusters_raw)} clusters", flush=True)

print("Loading AEC data...", flush=True)
aec_data = load_aec_data()
print(f"  {len(aec_data)} AEC records", flush=True)

# Build rm → cluster list
rm_clusters = {}
for c in clusters_raw:
    rm = c.get('rm')
    if rm:
        rm_clusters.setdefault(rm, []).append(c)

# ── Score each RM ──────────────────────────────────────────────────────────────

print("\nScoring Regional Markets...", flush=True)

scored = []
for rm in rms_raw:
    rm_id   = rm['rm_id']
    iso     = rm.get('iso', '')
    cont    = rm.get('continent', rm.get('cont', ''))

    # Determine continent from ISO if missing
    if not cont:
        cont = 'NA' if iso in NA_ISOS else 'EU' if iso in EU_ISOS else 'OTHER'

    if cont not in ('NA', 'EU'):
        continue

    cl = rm_clusters.get(rm_id, [])
    if not cl:
        continue

    # Tier counts
    t1 = sum(1 for c in cl if c['t'] == 1)
    t2 = sum(1 for c in cl if c['t'] == 2)
    t3 = sum(1 for c in cl if c['t'] == 3)
    tier_score = t1 * 4 + t2 * 2 + t3 * 1
    if tier_score == 0:
        continue

    # Civic anchor
    civic = False
    for c in cl:
        for m in c.get('members', []):
            if m.get('category', '').lower() in CIVIC_CATEGORIES:
                civic = True
                break
        if civic:
            break
    civ_mult = 1.5 if civic else 1.0

    # Centroid
    centroid = rm.get('centroid', {})
    if isinstance(centroid, dict):
        lat = centroid.get('lat', 0)
        lon = centroid.get('lon', 0)
    elif isinstance(centroid, list) and len(centroid) == 2:
        lon, lat = centroid[0], centroid[1]
    else:
        lat = lon = 0

    # Metro distance
    metros = NA_METROS if cont == 'NA' else EU_METROS
    dist_km, nearest = nearest_metro(lat, lon, metros)
    met_mult = metro_multiplier(dist_km)

    # Confidence
    mkt_conf = rm.get('mkt_conf', 'high')
    conf_factor = 1.0 if mkt_conf == 'high' else 0.7

    score = tier_score * civ_mult * met_mult * conf_factor

    # AEC summary for clusters in this RM
    cluster_ids = [c['id'] for c in cl]
    aec_summary = {}
    for cid in cluster_ids:
        if cid in aec_data:
            for k, v in aec_data[cid].items():
                if v and k not in aec_summary:
                    aec_summary[k] = v

    # Cluster details
    cluster_details = []
    for c in cl:
        cluster_details.append({
            'cluster_id': c['id'],
            'tier': c['t'],
            'tier_desc': c.get('td', ''),
            'span_km': c.get('span', 0),
            'members': [
                {'chain_id': m.get('chain_id', ''), 'name': m.get('name', ''),
                 'category': m.get('category', '')}
                for m in c.get('members', [])
            ],
            'aec': aec_data.get(c['id'], {}),
        })

    scored.append({
        'rm_id':             rm_id,
        'market':            rm.get('market', ''),
        'iso':               iso,
        'continent':         cont,
        'region':            rm.get('region', ''),
        'centroid':          {'lat': lat, 'lon': lon},
        't1': t1, 't2': t2, 't3': t3,
        'tier_score':        tier_score,
        'civic':             civic,
        'civic_multiplier':  civ_mult,
        'nearest_metro':     nearest,
        'dist_km':           round(dist_km, 1),
        'metro_multiplier':  round(met_mult, 3),
        'mkt_conf':          mkt_conf,
        'confidence_factor': conf_factor,
        'score':             round(score, 3),
        'cluster_ids':       cluster_ids,
        'clusters':          cluster_details,
        'aec_summary':       aec_summary,
    })

print(f"  Scored {len(scored)} Regional Markets", flush=True)

# ── Split and rank ─────────────────────────────────────────────────────────────

na_scored = sorted([r for r in scored if r['continent'] == 'NA'], key=lambda r: r['score'], reverse=True)
eu_scored = sorted([r for r in scored if r['continent'] == 'EU'], key=lambda r: r['score'], reverse=True)

# Add rank
for i, r in enumerate(na_scored, 1):
    r['rank'] = i
for i, r in enumerate(eu_scored, 1):
    r['rank'] = i

top400_na = na_scored[:400]
top400_eu = eu_scored[:400]

# ── Write output ───────────────────────────────────────────────────────────────

with open(OUT_NA, 'w') as f:
    json.dump(top400_na, f, indent=2)
print(f"\nWrote {OUT_NA} ({len(top400_na)} entries)", flush=True)

with open(OUT_EU, 'w') as f:
    json.dump(top400_eu, f, indent=2)
print(f"Wrote {OUT_EU} ({len(top400_eu)} entries)", flush=True)

# ── Print top 20 NA and EU for agent context ───────────────────────────────────

print("\n── TOP 20 NA Regional Markets ──")
print(f"{'Rank':>4}  {'Market':<40}  {'ISO':>3}  {'T1':>3}  {'T2':>3}  {'T3':>3}  {'Civic':>5}  {'Metro dist':>10}  {'Score':>7}")
for r in top400_na[:20]:
    print(f"{r['rank']:>4}  {r['market']:<40}  {r['iso']:>3}  {r['t1']:>3}  {r['t2']:>3}  {r['t3']:>3}  {'Yes' if r['civic'] else 'No':>5}  {r['nearest_metro'][:8]:>10}  {r['score']:>7.2f}")

print("\n── TOP 20 EU Regional Markets ──")
print(f"{'Rank':>4}  {'Market':<40}  {'ISO':>3}  {'T1':>3}  {'T2':>3}  {'T3':>3}  {'Civic':>5}  {'Metro dist':>10}  {'Score':>7}")
for r in top400_eu[:20]:
    print(f"{r['rank']:>4}  {r['market']:<40}  {r['iso']:>3}  {r['t1']:>3}  {r['t2']:>3}  {r['t3']:>3}  {'Yes' if r['civic'] else 'No':>5}  {r['nearest_metro'][:8]:>10}  {r['score']:>7.2f}")

# ── Identify 3 test regions for TOPIC articles ──────────────────────────────────

print("\n── 3 TEST REGIONS FOR TOPIC ARTICLES ──")
print("  (top-scoring NA non-metro, second NA, top-scoring EU)")

def first_non_metro(ranked, metro_threshold_km=40):
    for r in ranked:
        if r['dist_km'] >= metro_threshold_km:
            return r
    return ranked[0]

test1 = first_non_metro(top400_na, 40)
test2 = next((r for r in top400_na if r['rm_id'] != test1['rm_id'] and r['dist_km'] >= 40), top400_na[1])
test3 = first_non_metro(top400_eu, 40)

for i, r in enumerate([test1, test2, test3], 1):
    print(f"\n  Test {i}: {r['market']} ({r['iso']}) rank={r['rank']} score={r['score']:.2f}")
    print(f"    dist_from_{r['nearest_metro']}={r['dist_km']}km  T1={r['t1']} T2={r['t2']} T3={r['t3']}  civic={r['civic']}")
    print(f"    centroid: lat={r['centroid']['lat']} lon={r['centroid']['lon']}")
    print(f"    clusters: {r['cluster_ids']}")

print("\n── Score Complete ──")
