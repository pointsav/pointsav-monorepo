#!/usr/bin/env python3
"""
score-regional-markets.py — Top 600 Regional Markets ranking (NA and EU).

DEFINITION: A Regional Market is a named suburb or satellite municipality within
commuting distance (15–80 km) of a major metro centre. Major metro cores are
covered extensively by institutional research (Oxford Economics, CBRE, Colliers
International). Standalone secondary cities (>80 km from any major metro) are a
separate category. This script ranks only the suburban-regional tier.

Classification by haversine distance to nearest major metro centroid:
  metro-core           dist_km < SUBURBAN_MIN_KM  → excluded
  suburban-regional    SUBURBAN_MIN_KM ≤ dist_km ≤ SUBURBAN_MAX_KM  → Top 400 pool
  standalone-secondary dist_km > SUBURBAN_MAX_KM  → excluded (separate category)

Geographic coherence check:
  Any RM whose clusters span > MAX_SPAN_KM is a name-collision aggregation and
  is excluded regardless of distance classification.

Scoring formula (within suburban-regional pool only):
  score = tier_score × civic_multiplier × confidence_factor

  tier_score        = (t1 × 4) + (t2 × 2) + (t3 × 1)
  civic_multiplier  = 1.5 if any cluster member has a civic-anchor category, else 1.0
  confidence_factor = 1.0 if mkt_conf == 'high' else 0.7

  No metro_multiplier — all markets in the pool are already in the suburban band.

Outputs:
  work/top600-regional-na.json  — top 600 suburban-regional NA markets
  work/top600-regional-eu.json  — top 600 suburban-regional EU markets
  work/top600-proforma-coverage.json — per-country proforma 3x proof
  work/classified-na.json       — all NA markets with rm_type (reference)
  work/classified-eu.json       — all EU markets with rm_type (reference)

Run from:  app-orchestration-gis/
  python3 score-regional-markets.py
"""

import csv
import json
import math
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
WORK_DIR   = SCRIPT_DIR / 'work'

RM_PATH       = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/regional-markets.json')
CLUSTERS_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')
AEC_PATH      = WORK_DIR / 'DATA-aec-clusters.csv'

OUT_NA       = WORK_DIR / 'top600-regional-na.json'
OUT_EU       = WORK_DIR / 'top600-regional-eu.json'
OUT_CLASS_NA = WORK_DIR / 'classified-na.json'
OUT_CLASS_EU = WORK_DIR / 'classified-eu.json'
OUT_COVERAGE = WORK_DIR / 'top600-proforma-coverage.json'

# Geometric isolation thresholds (replaces fixed metro-distance approach)
# INNER_KM: below this = metro-adjacent (too close to be an independent suburban market)
# OUTER_KM: above this = standalone (not connected to the suburban ring of any major metro)
# Both are derived from co-location data geometry, not from a metro reference list.
INNER_KM      = 3     # only excludes near-duplicate co-located markets (same hub as a
                      # neighbour). Well-known dense-metro suburbs (Plano, Brampton) stay
                      # IN and are ranked DOWN by the isolation gradient, not excluded.
OUTER_KM      = 120   # replaces SUBURBAN_MAX_KM=80; wider range opens low-density countries
METRO_CORE_KM = 12    # a market centroid within this of a major metro centroid IS that
                      # metro (already a "Metro Market") → excluded. Exclusion-only; does
                      # NOT define the suburban band. Catches localized names (Roma, Genova).

# Market confidence overrides — where naming resolution returns administrative names
# instead of colloquial names, confidence is downgraded in regional-markets.json.
# These markets are real; upgrade confidence here rather than editing the source data.
MKT_CONF_OVERRIDES = {
    'rm_ca_strathcona_county': 'high',  # Sherwood Park / Strathcona County, AB — real T1 market
}
MAX_SPAN_KM   = 200   # cluster bounding-box span limit (name-collision check)

TOP_N = 600  # candidates per continent (3x proforma for the largest target market)

# Nordic countries treated as one combined market for normalization and proforma coverage.
NORDIC_ISOS = {'DK', 'SE', 'NO', 'FI'}

# New Europe group: Central/Eastern Europe + Balkans + Greece.
# GR is a member; the standalone GR proforma slot is replaced by this group.
NEW_EUROPE_ISOS = {'CZ', 'HU', 'RO', 'BG', 'SK', 'HR', 'GR'}

# Per-country proforma targets from Woodfine Buildings Portfolio Proforma V2.
# CA/US/ES/MX confirmed from proforma; Spain template applied to remaining markets.
# GR replaced by NEW_EUROPE group (2026-06-29): CZ/HU/RO/BG/SK/HR/GR treated as one market.
PROFORMA_TARGETS = {
    'CA': 22, 'US': 44, 'MX': 22,
    'ES': 22, 'PL': 22, 'NORDICS': 22,
    'GB': 22, 'IT': 22, 'NEW_EUROPE': 22,
}


def norm_group(iso):
    """Map a country ISO to its normalization group."""
    if iso in NORDIC_ISOS:
        return 'NORDICS'
    if iso in NEW_EUROPE_ISOS:
        return 'NEW_EUROPE'
    return iso


CIVIC_CATEGORIES = {
    'healthcare', 'higher_education', 'medical', 'education',
    'hospital', 'university', 'college', 'clinic',
}

# ── Metro reference lists ──────────────────────────────────────────────────────
# These define both metro-core exclusion zones and suburban distance reference.
# Covers ~100 NA + ~120 EU major metro centres.

NA_METROS = [
    # United States
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
    (43.0481, -76.1474, 'Syracuse'),
    (32.2226, -110.9747, 'Tucson'),
    (36.7378, -119.7871, 'Fresno'),
    (35.0853, -106.6056, 'Albuquerque'),
    (42.3601, -71.0589, 'Boston'),
    (42.3314, -83.0458, 'Detroit'),
    (43.0389, -87.9065, 'Milwaukee'),
    (31.7619, -106.4850, 'El Paso'),
    (37.6879, -97.3442, 'Wichita'),
    (38.8339, -104.8214, 'Colorado Springs'),
    (41.2565, -95.9345, 'Omaha'),
    (41.5868, -93.6250, 'Des Moines'),
    (36.1540, -95.9928, 'Tulsa'),
    (35.9606, -83.9207, 'Knoxville'),
    (36.0726, -79.7920, 'Greensboro'),
    (36.0999, -80.2442, 'Winston-Salem'),
    (35.9940, -78.8986, 'Durham'),
    (36.8529, -75.9780, 'Virginia Beach'),
    (33.9988, -81.0450, 'Columbia SC'),
    (34.8526, -82.3940, 'Greenville SC'),
    (43.5978, -116.2024, 'Boise'),
    (46.8772, -96.7898, 'Fargo'),
    (43.5460, -96.7313, 'Sioux Falls'),
    (44.5133, -88.0133, 'Green Bay'),
    (42.6526, -73.7562, 'Albany'),
    (42.2626, -71.8023, 'Worcester'),
    (41.3083, -72.9279, 'New Haven'),
    (34.7465, -92.2896, 'Little Rock'),
    (30.6954, -88.0399, 'Pensacola'),
    (30.4158, -91.1800, 'Baton Rouge'),
    (29.9511, -90.0715, 'New Orleans'),
    (34.2000, -119.1800, 'Oxnard'),
    (33.9958, -117.3564, 'Riverside'),
    (37.9577, -121.2908, 'Stockton'),
    (37.6391, -120.9969, 'Modesto'),
    (35.3733, -119.0187, 'Bakersfield'),
    (47.2529, -122.4443, 'Tacoma'),
    (47.0379, -122.9007, 'Olympia'),
    (44.0805, -103.2310, 'Rapid City'),
    (40.7989, -77.8600, 'State College'),
    (40.0379, -76.3055, 'Lancaster'),
    (39.9576, -75.6052, 'Wilmington DE'),
    (30.3960, -88.8853, 'Mobile'),
    (32.3668, -86.2999, 'Montgomery'),
    (33.9519, -84.5499, 'Kennesaw'),
    (39.7817, -89.6501, 'Springfield IL'),
    # Canada
    (43.6532, -79.3832, 'Toronto'),
    (45.5017, -73.5673, 'Montreal'),
    (49.2827, -123.1207, 'Vancouver'),
    (51.0447, -114.0719, 'Calgary'),
    (53.5461, -113.4938, 'Edmonton'),
    (45.4215, -75.6972, 'Ottawa'),
    (49.8951, -97.1384, 'Winnipeg'),
    (46.8139, -71.2080, 'Quebec City'),
    (43.2557, -79.8711, 'Hamilton'),
    (43.1594, -79.2469, 'St. Catharines'),
    (43.3501, -80.3161, 'Kitchener'),
    (46.1548, -60.1942, 'Cape Breton'),
    (44.6476, -63.5728, 'Halifax'),
    (46.0878, -64.7782, 'Moncton'),
    (47.5615, -52.7126, 'St. John\'s'),
    # Mexico
    (19.4326, -99.1332, 'Mexico City'),
    (20.6597, -103.3496, 'Guadalajara'),
    (25.6866, -100.3161, 'Monterrey'),
    (19.0413, -98.2062, 'Puebla'),
    (21.1619, -86.8515, 'Cancún'),
    (20.9674, -89.5926, 'Mérida'),
    (32.5027, -117.0087, 'Tijuana'),
    (29.0729, -110.9559, 'Hermosillo'),
    (28.6353, -106.0889, 'Chihuahua'),
    (26.9353, -101.4252, 'Saltillo'),
    (22.1565, -100.9855, 'San Luis Potosí'),
    (21.0190, -101.2574, 'León'),
    (20.5888, -100.3899, 'Querétaro'),
    (19.7010, -101.1844, 'Morelia'),
    (20.1011, -98.7591, 'Pachuca'),
    (18.9242, -99.2216, 'Cuernavaca'),
    (21.8853, -102.2916, 'Aguascalientes'),
    (20.9670, -100.7440, 'Irapuato'),
    (19.8301, -90.5349, 'Campeche'),
    (17.0732, -96.7266, 'Oaxaca'),
]

EU_METROS = [
    # United Kingdom
    (51.5074, -0.1278, 'London'),
    (53.4808, -2.2426, 'Manchester'),
    (52.4862, -1.8904, 'Birmingham'),
    (53.8008, -1.5491, 'Leeds'),
    (55.8642, -4.2518, 'Glasgow'),
    (55.9533, -3.1883, 'Edinburgh'),
    (51.4816, -3.1791, 'Cardiff'),
    (51.4545, -2.5879, 'Bristol'),
    (52.9548, -1.1581, 'Nottingham'),
    (53.3811, -1.4701, 'Sheffield'),
    (52.6369, -1.1398, 'Leicester'),
    (50.8229, -0.1363, 'Brighton'),
    (53.7632, -2.7044, 'Preston'),
    (54.9783, -1.6178, 'Newcastle'),
    (51.8787, -0.4200, 'Luton'),
    (51.4400, -1.0000, 'Reading'),
    (52.2053,  0.1218, 'Cambridge'),
    (51.7520, -1.2577, 'Oxford'),
    (52.6333, -1.1333, 'Coventry'),
    (53.5453, -2.6318, 'Wigan'),
    (53.7974, -1.7631, 'Bradford'),
    (50.9097, -1.4044, 'Southampton'),
    (50.7192, -1.8808, 'Bournemouth'),
    (51.3781, -2.3597, 'Bath'),
    (51.6214, -3.9436, 'Swansea'),
    (54.5967, -5.9301, 'Belfast'),
    (57.1499, -2.0942, 'Aberdeen'),
    (56.4620, -2.9707, 'Dundee'),
    # Germany
    (52.5200, 13.4050, 'Berlin'),
    (53.5753, 10.0153, 'Hamburg'),
    (48.1351, 11.5820, 'Munich'),
    (50.9333,  6.9500, 'Cologne'),
    (50.1109,  8.6821, 'Frankfurt'),
    (48.7758,  9.1829, 'Stuttgart'),
    (51.5136,  7.4653, 'Dortmund'),
    (51.2217,  6.7762, 'Düsseldorf'),
    (51.4508,  7.0131, 'Essen'),
    (53.0793,  8.8017, 'Bremen'),
    (52.3759,  9.7320, 'Hannover'),
    (51.9607,  7.6261, 'Münster'),
    (49.4521, 11.0767, 'Nürnberg'),
    (51.0504, 13.7373, 'Dresden'),
    (51.3397, 12.3731, 'Leipzig'),
    (52.1205, 11.6276, 'Magdeburg'),
    (49.0069,  8.4037, 'Karlsruhe'),
    (49.4875,  8.4660, 'Mannheim'),
    (48.3705, 10.8978, 'Augsburg'),
    (50.7753,  6.0839, 'Aachen'),
    (51.4779,  7.2148, 'Bochum'),
    (52.2689, 10.5268, 'Braunschweig'),
    (52.0302,  8.5325, 'Bielefeld'),
    (47.9990,  7.8421, 'Freiburg'),
    (48.1374,  9.3730, 'Ulm'),
    (54.0924, 12.0991, 'Rostock'),
    (53.8655, 10.6866, 'Lübeck'),
    (53.0758, 13.9907, 'Cottbus'),
    (51.4818, 11.9697, 'Halle'),
    (49.0000, 12.1000, 'Regensburg'),
    (50.0000,  8.2700, 'Wiesbaden'),
    (50.1106,  8.6821, 'Darmstadt'),
    # France
    (48.8566,  2.3522, 'Paris'),
    (43.2965,  5.3698, 'Marseille'),
    (45.7640,  4.8357, 'Lyon'),
    (43.6047,  1.4442, 'Toulouse'),
    (43.7102,  7.2620, 'Nice'),
    (47.2184, -1.5536, 'Nantes'),
    (48.5734,  7.7521, 'Strasbourg'),
    (43.6108,  3.8767, 'Montpellier'),
    (44.8378, -0.5792, 'Bordeaux'),
    (50.6292,  3.0573, 'Lille'),
    (48.1173, -1.6778, 'Rennes'),
    (45.4338,  4.3903, 'Saint-Étienne'),
    (43.1242,  5.9280, 'Toulon'),
    (45.1885,  5.7245, 'Grenoble'),
    (49.4938,  0.1077, 'Le Havre'),
    (47.3220,  5.0415, 'Dijon'),
    (49.2583,  4.0317, 'Reims'),
    (48.7833,  2.0000, 'Versailles'),
    (47.9784,  7.7649, 'Colmar'),
    (43.8384,  4.3601, 'Nîmes'),
    (43.5297,  5.4474, 'Aix-en-Provence'),
    (48.3905, -4.4860, 'Brest'),
    (47.9960,  0.1966, 'Le Mans'),
    (50.3672,  3.5236, 'Valenciennes'),
    (49.8941,  2.2958, 'Amiens'),
    # Poland
    (52.2297, 21.0122, 'Warsaw'),
    (50.0647, 19.9450, 'Kraków'),
    (51.1079, 17.0385, 'Wrocław'),
    (51.7592, 19.4560, 'Łódź'),
    (53.4285, 14.5528, 'Szczecin'),
    (54.3520, 18.6466, 'Gdańsk'),
    (53.1325, 23.1688, 'Białystok'),
    (51.2465, 22.5684, 'Lublin'),
    (53.7784, 20.4801, 'Olsztyn'),
    (53.1138, 18.0084, 'Bydgoszcz'),
    (52.4064, 16.9252, 'Poznań'),
    (50.8661, 20.6286, 'Kielce'),
    (50.2649, 19.0238, 'Katowice'),
    (50.3000, 18.6667, 'Gliwice'),
    (50.6751, 17.9213, 'Opole'),
    (53.4289, 21.5690, 'Łomża'),
    # Spain
    (40.4168, -3.7038, 'Madrid'),
    (41.3851,  2.1734, 'Barcelona'),
    (39.4699, -0.3763, 'Valencia'),
    (37.3891, -5.9845, 'Seville'),
    (37.1773, -3.5986, 'Granada'),
    (38.3452, -0.4815, 'Alicante'),
    (39.5693,  2.6502, 'Palma'),
    (36.7213, -4.4214, 'Málaga'),
    (43.2627, -2.9253, 'Bilbao'),
    (43.3623, -8.4115, 'A Coruña'),
    (41.6488, -0.8891, 'Zaragoza'),
    (43.3614, -5.8593, 'Oviedo'),
    (39.8628, -4.0273, 'Toledo'),
    (41.6561, -4.7244, 'Valladolid'),
    (37.8882, -4.7794, 'Córdoba'),
    (36.5271, -6.2886, 'Cádiz'),
    # Italy
    (41.9028, 12.4964, 'Rome'),
    (45.4654,  9.1859, 'Milan'),
    (40.8518, 14.2681, 'Naples'),
    (45.0703,  7.6869, 'Turin'),
    (38.1157, 13.3615, 'Palermo'),
    (44.4056,  8.9463, 'Genoa'),
    (43.7696, 11.2558, 'Florence'),
    (44.4949, 11.3426, 'Bologna'),
    (45.4409, 12.3155, 'Venice'),
    (45.4384, 10.9916, 'Verona'),
    (37.5019, 15.0875, 'Catania'),
    (38.1111, 15.6617, 'Messina'),
    (45.6495, 13.7768, 'Trieste'),
    (43.3167, 13.3167, 'Ancona'),
    (41.1171, 16.8719, 'Bari'),
    (40.6401, 15.8051, 'Potenza'),
    (37.9333, 15.5500, 'Reggio Calabria'),
    # Netherlands
    (52.3676,  4.9041, 'Amsterdam'),
    (51.9225,  4.4792, 'Rotterdam'),
    (52.3758,  4.8975, 'The Hague'),
    (52.0907,  5.1214, 'Utrecht'),
    (51.4416,  5.4697, 'Eindhoven'),
    (51.8126,  5.8372, 'Nijmegen'),
    (53.2194,  6.5665, 'Groningen'),
    # Belgium
    (50.8503,  4.3517, 'Brussels'),
    (51.2213,  4.3997, 'Antwerp'),
    (51.0543,  3.7174, 'Ghent'),
    (50.6451,  5.5734, 'Liège'),
    (50.4109,  4.4441, 'Charleroi'),
    (50.8280,  3.2614, 'Kortrijk'),
    # Austria
    (48.2082, 16.3738, 'Vienna'),
    (47.8095, 13.0550, 'Salzburg'),
    (47.2692, 11.4041, 'Innsbruck'),
    (47.0707, 15.4395, 'Graz'),
    (48.3069, 14.2858, 'Linz'),
    # Scandinavia
    (59.3293, 18.0686, 'Stockholm'),
    (57.7089, 11.9746, 'Gothenburg'),
    (55.6761, 12.5683, 'Copenhagen'),
    (56.1572, 10.2107, 'Aarhus'),
    (55.3959, 10.3883, 'Odense'),
    (55.0607, 14.9057, 'Bornholm'),
    (57.0488, 9.9187, 'Aalborg'),
    (59.9139, 10.7522, 'Oslo'),
    (58.9700,  5.7331, 'Stavanger'),
    (63.4305, 10.3951, 'Trondheim'),
    (60.3913,  5.3221, 'Bergen'),
    (60.1699, 24.9384, 'Helsinki'),
    (61.4978, 23.7610, 'Tampere'),
    (65.0121, 25.4651, 'Oulu'),
    (60.4518, 22.2666, 'Turku'),
    # Greece
    (37.9838, 23.7275, 'Athens'),
    (40.6401, 22.9444, 'Thessaloniki'),
    (38.2466, 21.7346, 'Patras'),
    (35.3387, 25.1442, 'Heraklion'),
    # Portugal
    (38.7223, -9.1393, 'Lisbon'),
    (41.1579, -8.6291, 'Porto'),
    (41.5503, -8.4200, 'Braga'),
    (40.2033, -8.4103, 'Coimbra'),
    # Czech Republic / Slovakia
    (50.0755, 14.4378, 'Prague'),
    (49.1951, 16.6068, 'Brno'),
    (49.7384, 13.3736, 'Plzeň'),
    (48.1486, 17.1077, 'Bratislava'),
    (48.6971, 21.2611, 'Košice'),
    # Hungary / Romania / Bulgaria
    (47.4979, 19.0402, 'Budapest'),
    (47.1585, 27.6014, 'Iași'),
    (44.4268, 26.1025, 'Bucharest'),
    (46.7712, 23.6236, 'Cluj-Napoca'),
    (45.7489, 21.2087, 'Timișoara'),
    (45.6427, 25.5887, 'Brașov'),
    (44.1598, 28.6348, 'Constanța'),
    (42.6977, 23.3219, 'Sofia'),
    (42.1354, 24.7453, 'Plovdiv'),
    (43.2048, 27.9101, 'Varna'),
    # Iceland / others
    (64.1265, -21.8174, 'Reykjavik'),
]

NA_ISOS = {'US', 'CA', 'MX'}
EU_ISOS = {
    'GB', 'DE', 'FR', 'ES', 'IT', 'PL', 'NL', 'AT', 'PT', 'GR',
    'IS', 'SE', 'DK', 'FI', 'NO', 'BE', 'CZ', 'SK', 'HU', 'RO', 'BG',
}


# ── Utility functions ──────────────────────────────────────────────────────────

def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat, lon, metros):
    best_dist, best_name = float('inf'), ''
    for mlat, mlon, name in metros:
        d = haversine(lat, lon, mlat, mlon)
        if d < best_dist:
            best_dist, best_name = d, name
    return best_dist, best_name


def classify_rm_geometric(anchor_d):
    """Classify using anchor_d — distance to nearest T1 cluster outside this RM.
    No fixed metro reference needed; classification emerges from co-location topology."""
    if anchor_d < INNER_KM:
        return 'metro-adjacent'       # too close to a larger hub
    elif anchor_d <= OUTER_KM:
        return 'suburban-regional'    # eligible for Top 400
    else:
        return 'standalone'           # genuinely isolated city


def rm_regional_score(clusters, anchor_d, civic, mkt_conf):
    """Regional-market DISCOVERY score.

    Philosophy: a Regional Market's value is being the retail anchor for an
    UNDERSERVED region — a place people don't already associate with a major
    metro. Large, well-known suburbs (Mississauga, Plano) are effectively metro
    markets already; they belong in the Top 400 but should never rank #1.

    ISOLATION (distance to the nearest other major hub) is the primary signal.
    Retail VOLUME is dampened (sqrt of depth beyond the anchor floor) so a place
    with many clusters cannot dominate just by being a big retail agglomeration.
    The #1 slots go to isolated, sufficient regional hubs — the genuine discoveries.

    Returns (score, tier_score, quality_base).
    """
    if not clusters:
        return 0.0, 0.0, 0.0

    # T1.b scoring correction: tight_intact ∧ mc ≥ 3 should classify as T1 per the
    # taxonomy definition, but some builds miss this path and emit t=2. Apply correction
    # at score time so these markets compete as T1 without requiring a full rebuild.
    def effective_tier(c):
        if c.get('t') == 2 and c.get('tight') and c.get('mc', 0) >= 3:
            return 1  # T1.b: compact multi-anchor (tight, ≥3 members)
        return c.get('t', 3)

    # Tier depth. For close-in suburbs (anchor_d ≤ 40 km), use a depth_bonus floor of 2
    # so single T1 clusters (tier_score=4) earn a quality bonus over single T2 (tier_score=2).
    # This lets composition distinguish Airdrie/Sherwood Park from T2 markets nearby.
    # For isolated markets (anchor_d > 40 km), keep floor=4 — they score high from isolation
    # and don't need an additional composition boost that would flood the top ranks with
    # small-town T1.b reclassifications far from any competing T1.
    depth_floor = 2 if anchor_d <= 40.0 else 4
    tier_score  = sum(4 if effective_tier(c) == 1 else 2 if effective_tier(c) == 2 else 1 for c in clusters)
    depth_bonus = 0.30 * math.sqrt(max(tier_score - depth_floor, 0))
    tight_avg   = sum(1.1 if c.get('tight') else 1.0 for c in clusters) / len(clusters)
    quality     = (1.0 + depth_bonus) * tight_avg

    # Isolation — discovery signal. Linear 0.65→1.0 over 0–90 km, then plateau.
    # Floor raised from 0.30 to 0.65 so close-in suburbs (Sherwood Park, Airdrie)
    # compete on composition rather than being suppressed by proximity alone.
    iso_norm  = min(anchor_d, 90.0) / 90.0
    isolation = 0.65 + 0.35 * iso_norm

    # Civic (hospital/university) — modest community-vitality signal
    civic_mult  = 1.15 if civic else 1.0
    # Confidence penalty reduced from 0.70 to 0.90 — medium confidence reflects naming
    # uncertainty, not market quality; a 10% penalty is sufficient signal.
    conf_factor = 1.0 if mkt_conf == 'high' else 0.90

    score = quality * isolation * civic_mult * conf_factor
    return round(score, 4), round(tier_score, 4), round(quality, 4)


def cluster_lat_lon(c):
    """Extract lat/lon from cluster object; falls back to parsing cluster ID."""
    lat = c.get('lat')
    lon = c.get('lon')
    if lat is not None and lon is not None:
        return float(lat), float(lon)
    # Parse from ID format: co_{iso}_{n|s}{lat×100000}_{e|w}{lon×100000}
    cid = c.get('id', '')
    parts = cid.split('_')
    if len(parts) >= 4:
        try:
            lp = parts[-2]
            op = parts[-1]
            lat_sign = 1 if lp[0] == 'n' else -1
            lon_sign = 1 if op[0] == 'e' else -1
            return lat_sign * int(lp[1:]) / 100000, lon_sign * int(op[1:]) / 100000
        except (ValueError, IndexError):
            pass
    return None, None


def is_geographically_coherent(cluster_list):
    """Return False if cluster bounding box spans more than MAX_SPAN_KM."""
    if len(cluster_list) < 2:
        return True
    coords = []
    for c in cluster_list:
        lat, lon = cluster_lat_lon(c)
        if lat is not None:
            coords.append((lat, lon))
    if len(coords) < 2:
        return True
    lats = [c[0] for c in coords]
    lons = [c[1] for c in coords]
    span = haversine(min(lats), min(lons), max(lats), max(lons))
    return span <= MAX_SPAN_KM


def load_aec_data():
    aec = {}
    if not AEC_PATH.exists():
        return aec
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

# Build rm_id → cluster list
rm_clusters = {}
for c in clusters_raw:
    rm = c.get('rm')
    if rm:
        rm_clusters.setdefault(rm, []).append(c)

# ── T1 spatial index for geometric isolation classifier ────────────────────────
# Pre-compute positions of all T1 clusters for fast anchor_d lookup.
# anchor_d = distance to the nearest T1 cluster NOT in this RM.
t1_positions = []  # [(lat, lon, cluster_id, rm_id), ...]
for c in clusters_raw:
    if c.get('t') == 1:
        clat, clon = cluster_lat_lon(c)
        if clat is not None:
            t1_positions.append((clat, clon, c['id'], c.get('rm', '')))

print(f"  T1 spatial index: {len(t1_positions)} T1 clusters", flush=True)


def nearest_external_t1(rm_id, lat, lon):
    """Haversine distance to nearest T1 cluster NOT belonging to this RM (km).
    Uses a ~180 km bounding-box pre-filter for speed."""
    best = float('inf')
    lat_tol = 1.65   # ~180 km in latitude degrees
    lon_tol = 2.5    # ~180 km in longitude at mid-latitudes
    for clat, clon, cid, crm in t1_positions:
        if crm == rm_id:
            continue
        if abs(clat - lat) > lat_tol or abs(clon - lon) > lon_tol:
            continue
        d = haversine(lat, lon, clat, clon)
        if d < best:
            best = d
    # If bounding box found nothing, do a full scan (handles sparse areas)
    if best == float('inf'):
        for clat, clon, cid, crm in t1_positions:
            if crm == rm_id:
                continue
            d = haversine(lat, lon, clat, clon)
            if d < best:
                best = d
    return best if best < float('inf') else 999.0


# ── Score and classify ─────────────────────────────────────────────────────────

print("\nScoring and classifying Regional Markets...", flush=True)

all_scored = []
skip_counts = {'no_clusters': 0, 'no_tier': 0, 'incoherent': 0}

for rm in rms_raw:
    rm_id = rm['rm_id']
    iso   = rm.get('iso', '')
    cont  = rm.get('continent', rm.get('cont', ''))

    if not cont:
        cont = 'NA' if iso in NA_ISOS else 'EU' if iso in EU_ISOS else 'OTHER'
    if cont not in ('NA', 'EU'):
        continue

    cl = rm_clusters.get(rm_id, [])
    if not cl:
        skip_counts['no_clusters'] += 1
        continue

    # Geographic coherence — exclude name-collision aggregations
    if not is_geographically_coherent(cl):
        skip_counts['incoherent'] += 1
        continue

    # Tier counts (kept for output metadata and comparison)
    t1 = sum(1 for c in cl if c['t'] == 1)
    t2 = sum(1 for c in cl if c['t'] == 2)
    t3 = sum(1 for c in cl if c['t'] == 3)
    tier_score = t1 * 4 + t2 * 2 + t3 * 1
    if tier_score == 0:
        skip_counts['no_tier'] += 1
        continue

    # Civic anchor
    civic = any(
        m.get('category', '').lower() in CIVIC_CATEGORIES
        for c in cl for m in c.get('members', [])
    )

    # Centroid
    centroid = rm.get('centroid', {})
    if isinstance(centroid, dict):
        lat = centroid.get('lat', 0)
        lon = centroid.get('lon', 0)
    elif isinstance(centroid, list) and len(centroid) == 2:
        lon, lat = centroid[0], centroid[1]
    else:
        lat = lon = 0

    # Geometric classification — anchor_d replaces fixed distance thresholds.
    # anchor_d = nearest T1 cluster outside this RM; derived from co-location data.
    anchor_d = nearest_external_t1(rm_id, lat, lon)
    rm_type  = classify_rm_geometric(anchor_d)

    # Metro distance used for (a) naming/display and (b) metro-core exclusion below.
    metros             = NA_METROS if cont == 'NA' else EU_METROS
    dist_km, suburb_of = nearest_metro(lat, lon, metros)

    # Metro-core exclusion: a market sitting essentially ON a major metro centroid
    # IS that metro (already covered as a Metro Market) → exclude. Language-agnostic;
    # catches localized spellings the name match misses (Roma=Rome, Genova=Genoa).
    if dist_km < METRO_CORE_KM:
        rm_type = 'metro-adjacent'

    # Name-match override: also force metro-adjacent when the market name IS the metro
    # (compound names like "Indianapolis city (balance)" whose centroid may be offset).
    market_base    = rm.get('market', '').split(',')[0].strip().lower()
    metro_names_lc = {name.lower() for _, _, name in metros}
    if (market_base in metro_names_lc
            or any(market_base.startswith(m) for m in metro_names_lc if len(m) >= 5)):
        rm_type = 'metro-adjacent'

    # Confidence (with override for markets where naming resolution lowered confidence)
    mkt_conf = MKT_CONF_OVERRIDES.get(rm_id, rm.get('mkt_conf', 'high'))

    # Regional discovery score (isolation-primary, volume-dampened)
    score, geo_quality_raw, normalized_quality = rm_regional_score(cl, anchor_d, civic, mkt_conf)

    # AEC summary
    cluster_ids = [c['id'] for c in cl]
    aec_summary = {}
    for cid in cluster_ids:
        if cid in aec_data:
            for k, v in aec_data[cid].items():
                if v and k not in aec_summary:
                    aec_summary[k] = v

    # Cluster detail
    cluster_details = []
    for c in cl:
        clat, clon = cluster_lat_lon(c)
        cluster_details.append({
            'cluster_id': c['id'],
            'tier':       c['t'],
            'tier_desc':  c.get('td', ''),
            'span_km':    c.get('span', 0),
            'lat':        clat,
            'lon':        clon,
            'members': [
                {'chain_id': m.get('chain_id', ''), 'name': m.get('name', ''),
                 'category': m.get('category', '')}
                for m in c.get('members', [])
            ],
            'aec': aec_data.get(c['id'], {}),
        })

    all_scored.append({
        'rm_id':              rm_id,
        'market':             rm.get('market', ''),
        'rm_type':            rm_type,
        'suburb_of':          suburb_of,
        'dist_km':            round(dist_km, 1),      # metro distance (display only)
        'anchor_d':           round(anchor_d, 1),     # geometric isolation distance
        'iso':                iso,
        'continent':          cont,
        'region':             rm.get('region', ''),
        'centroid':           {'lat': lat, 'lon': lon},
        't1': t1, 't2': t2, 't3': t3,
        'tier_score':         tier_score,             # raw tier count (metadata)
        'geo_quality_raw':    geo_quality_raw,        # sum of per-cluster quality
        'normalized_quality': normalized_quality,     # geo_quality_raw / cluster_count
        'civic':              civic,
        'mkt_conf':           mkt_conf,
        'score':              round(score, 3),
        'cluster_ids':        cluster_ids,
        'clusters':           cluster_details,
        'aec_summary':        aec_summary,
    })

print(f"  Scored {len(all_scored)} Regional Markets", flush=True)
print(f"  Skipped — no clusters: {skip_counts['no_clusters']} | "
      f"no tier: {skip_counts['no_tier']} | "
      f"incoherent: {skip_counts['incoherent']}", flush=True)

# ── Classification summary ─────────────────────────────────────────────────────

for cont in ('NA', 'EU'):
    subset     = [r for r in all_scored if r['continent'] == cont]
    adjacent   = [r for r in subset if r['rm_type'] == 'metro-adjacent']
    suburban   = [r for r in subset if r['rm_type'] == 'suburban-regional']
    standalone = [r for r in subset if r['rm_type'] == 'standalone']
    print(f"  {cont}: {len(adjacent)} metro-adjacent | "
          f"{len(suburban)} suburban-regional | "
          f"{len(standalone)} standalone", flush=True)

# ── Filter and rank ────────────────────────────────────────────────────────────

na_regional = sorted(
    [r for r in all_scored if r['continent'] == 'NA' and r['rm_type'] == 'suburban-regional'],
    key=lambda r: r['score'], reverse=True,
)
eu_regional = sorted(
    [r for r in all_scored if r['continent'] == 'EU' and r['rm_type'] == 'suburban-regional'],
    key=lambda r: r['score'], reverse=True,
)

na_all = sorted([r for r in all_scored if r['continent'] == 'NA'], key=lambda r: r['score'], reverse=True)
eu_all = sorted([r for r in all_scored if r['continent'] == 'EU'], key=lambda r: r['score'], reverse=True)


def apply_country_norm(regional_list):
    """Add norm_score (0–1) by min-max normalizing raw scores within each country/group.

    Nordics (DK/SE/NO/FI) are treated as one group so the 66 proforma candidate slots
    are drawn from the best sites across all four countries, not separately.
    Best site in every group gets 1.0; worst gets 0.0. Groups with one site get 1.0.
    Re-sorts the list by norm_score DESC.
    """
    groups = {}
    for r in regional_list:
        g = norm_group(r['iso'])
        groups.setdefault(g, []).append(r)
    for g, members in groups.items():
        scores = [m['score'] for m in members]
        g_min, g_max = min(scores), max(scores)
        span = g_max - g_min
        for m in members:
            m['norm_score'] = round(1.0 if span == 0 else (m['score'] - g_min) / span, 4)
    return sorted(regional_list, key=lambda r: r['norm_score'], reverse=True)


na_regional = apply_country_norm(na_regional)
eu_regional = apply_country_norm(eu_regional)

for i, r in enumerate(na_regional, 1):
    r['rank'] = i
for i, r in enumerate(eu_regional, 1):
    r['rank'] = i

top_na = na_regional[:TOP_N]
top_eu = eu_regional[:TOP_N]

# ── Write output ───────────────────────────────────────────────────────────────

WORK_DIR.mkdir(exist_ok=True)

# rm-top600.json — flat dict keyed by rm_id for O(1) lookup in the map JS.
# Deployed to www/data/ so the browser can fetch it alongside clusters-meta.json.
# 'score' = norm_score × 100 (0–100 Regional Market Index, country-normalized);
# 'raw' = original discovery score before normalization.
OUT_RM_TOP600 = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/rm-top600.json')

rm_top600_dict = {}
for r in top_na:
    rm_top600_dict[r['rm_id']] = {
        'rank': r['rank'], 'score': round(100 * r['norm_score'], 1), 'raw': r['score'],
        'name': r['market'], 'metro': r['suburb_of'],
        'dist_km': r['anchor_d'],   # anchor_d = geometric isolation distance
        'lat': r['centroid']['lat'], 'lon': r['centroid']['lon'],
        't1': r['t1'], 't2': r['t2'], 't3': r['t3'],
        'iso': r['iso'],
        'cont': 'NA',
    }
for r in top_eu:
    rm_top600_dict[r['rm_id']] = {
        'rank': r['rank'], 'score': round(100 * r['norm_score'], 1), 'raw': r['score'],
        'name': r['market'], 'metro': r['suburb_of'],
        'dist_km': r['anchor_d'],   # anchor_d = geometric isolation distance
        'lat': r['centroid']['lat'], 'lon': r['centroid']['lon'],
        't1': r['t1'], 't2': r['t2'], 't3': r['t3'],
        'iso': r['iso'],
        'cont': 'EU',
    }
with open(OUT_RM_TOP600, 'w') as f:
    json.dump(rm_top600_dict, f, separators=(',', ':'))
print(f"Wrote {OUT_RM_TOP600} ({len(rm_top600_dict)} entries)", flush=True)

# Proforma coverage proof: per-country count of candidates in TOP_N vs. 3x target.
all_top = top_na + top_eu
coverage_rows = []
for grp, proforma_target in PROFORMA_TARGETS.items():
    if grp == 'NORDICS':
        count = sum(1 for r in all_top if r['iso'] in NORDIC_ISOS)
    elif grp == 'NEW_EUROPE':
        count = sum(1 for r in all_top if r['iso'] in NEW_EUROPE_ISOS)
    else:
        count = sum(1 for r in all_top if r['iso'] == grp)
    needed = proforma_target * 3
    coverage_rows.append({
        'country': grp,
        'proforma_target': proforma_target,
        'candidates_needed': needed,
        'candidates_in_top600': count,
        'meets_3x': count >= needed,
        'gap': max(0, needed - count),
    })
with open(OUT_COVERAGE, 'w') as f:
    json.dump(coverage_rows, f, indent=2)
print(f"Wrote {OUT_COVERAGE} ({len(coverage_rows)} country entries)", flush=True)
print("\n── Proforma 3x Coverage ──")
for row in coverage_rows:
    flag = '✓' if row['meets_3x'] else f'✗ gap={row["gap"]}'
    print(f"  {row['country']:>8}  proforma={row['proforma_target']:>2}  needed={row['candidates_needed']:>3}  "
          f"have={row['candidates_in_top600']:>3}  {flag}")

with open(OUT_NA, 'w') as f:
    json.dump(top_na, f, indent=2)
print(f"\nWrote {OUT_NA} ({len(top_na)} suburban-regional NA entries)", flush=True)

with open(OUT_EU, 'w') as f:
    json.dump(top_eu, f, indent=2)
print(f"Wrote {OUT_EU} ({len(top_eu)} suburban-regional EU entries)", flush=True)

with open(OUT_CLASS_NA, 'w') as f:
    json.dump(na_all, f, indent=2)
print(f"Wrote {OUT_CLASS_NA} ({len(na_all)} total NA with rm_type)", flush=True)

with open(OUT_CLASS_EU, 'w') as f:
    json.dump(eu_all, f, indent=2)
print(f"Wrote {OUT_CLASS_EU} ({len(eu_all)} total EU with rm_type)", flush=True)

# ── Console summary ────────────────────────────────────────────────────────────

print("\n── TOP 20 NA Regional Markets (suburban-regional, country-normalized) ──")
hdr = f"{'Rk':>3}  {'Market':<30}  {'ISO':>3}  {'Suburb of':<16}  {'km':>5}  T1  T2  T3  Civ  Norm  Raw"
print(hdr)
for r in top_na[:20]:
    print(f"{r['rank']:>3}  {r['market']:<30}  {r['iso']:>3}  "
          f"{r['suburb_of']:<16}  {r['anchor_d']:>5.1f}  "
          f"{r['t1']:>2}  {r['t2']:>2}  {r['t3']:>2}  "
          f"{'Y' if r['civic'] else 'N':>3}  {r['norm_score']:>5.3f}  {r['score']:>5.3f}")

print("\n── TOP 20 EU Regional Markets (suburban-regional, country-normalized) ──")
print(hdr)
for r in top_eu[:20]:
    print(f"{r['rank']:>3}  {r['market']:<30}  {r['iso']:>3}  "
          f"{r['suburb_of']:<16}  {r['anchor_d']:>5.1f}  "
          f"{r['t1']:>2}  {r['t2']:>2}  {r['t3']:>2}  "
          f"{'Y' if r['civic'] else 'N':>3}  {r['norm_score']:>5.3f}  {r['score']:>5.3f}")

# ── 3 candidate test markets for TOPIC articles ────────────────────────────────

print("\n── 3 CANDIDATE TEST REGIONS FOR TOPIC ARTICLES ──")

test1 = top_na[0] if top_na else None
test2 = next(
    (r for r in top_na if test1 and r['suburb_of'] != test1['suburb_of']),
    top_na[1] if len(top_na) > 1 else None
)
test3 = top_eu[0] if top_eu else None

for i, r in enumerate([test1, test2, test3], 1):
    if r:
        print(f"\n  Test {i}: {r['market']} ({r['iso']}), rank={r['rank']}, score={r['score']:.2f}")
        print(f"    suburb_of={r['suburb_of']}  dist={r['dist_km']} km  "
              f"T1={r['t1']} T2={r['t2']} T3={r['t3']}  civic={r['civic']}")
        print(f"    centroid: lat={r['centroid']['lat']} lon={r['centroid']['lon']}")
        print(f"    rm_id: {r['rm_id']}")

print("\n── Score Complete ──")
print(f"\nDefinition: suburban-regional = anchor_d in [{INNER_KM}–{OUTER_KM}] km from nearest external T1 cluster.")
print(f"Metro-adjacent (<{INNER_KM} km) and standalone (>{OUTER_KM} km)")
print(f"are classified and written to classified-*.json but excluded from the Top 400.")
