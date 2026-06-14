"""
taxonomy.py — Declarative co-location taxonomy.

Single source of truth replacing the scattered ALPHA_* sets in config.py,
CHAIN_FAMILY in build-tiles.py, and CHAIN_META in simulate-dbscan-ab.py.

Import:
    from taxonomy import CATEGORIES, BRAND_FILL, THRESHOLDS, DISPLAY_COUNTRIES
    from taxonomy import category_of, tier_of, slots_for
"""

# ── 7 CANONICAL CATEGORIES ────────────────────────────────────────────────────
# 5 retail anchors + 2 civic anchors.  Civic never gate the tier.
CATEGORIES = {
    "hypermarket": {
        "label": "Hypermarket",
        "naics": "452210",
        "description": "General-merchandise + grocery, large-format (Walmart-class).",
    },
    "hardware": {
        "label": "Hardware / Home Improvement",
        "naics": "444110",
        "description": "Home-improvement anchor (Home Depot class).",
    },
    "price_club": {
        "label": "Price Club",
        "naics": "452910",
        "description": "Membership warehouse-club format (Costco class).",
    },
    "lifestyle": {
        "label": "Lifestyle Anchor",
        "naics": "442110",
        "description": "Destination furniture / home goods (IKEA only by design).",
    },
    "sport": {
        "label": "Sport / Outdoor Anchor",
        "naics": "451110",
        "description": "Destination sporting-goods anchor (Decathlon-class, ≥3,000 sqm).",
    },
    "electronics": {
        "naics": "443142",
        "label": "Electronics",
        "description": "Large-format consumer electronics anchor (MediaMarkt-class, ≥3,000 sqm). EU Phase 21; NA (Best Buy) deferred to Phase 22.",
    },
    "medical": {
        "label": "Medical — Regional Hospital",
        "naics": "622110",
        "description": "Acute in-patient, 24/7 ED, scale-filtered.",
    },
    "education": {
        "label": "Education — University",
        "naics": "611310",
        "description": "Degree-granting, enrolment-filtered.",
    },
    # ── VWH archetype enrichment categories (do NOT gate PRO tier) ────────────
    "auto_parts": {
        "label": "Automotive Parts & Accessories",
        "naics": "441310",
        "description": "Auto-parts retail — VWH archetype signal. Never gates T1/T2/T3 tier.",
    },
    "paint": {
        "label": "Paint / Coatings",
        "naics": "444120",
        "description": "Paint-store retail — VWH archetype signal. Never gates T1/T2/T3 tier.",
    },
    # ── VWH Tier A enrichment categories ─────────────────────────────────────────
    "mro_industrial": {
        "label": "Industrial MRO Supply",
        "naics": "423840",
        "description": "MRO distributor (Würth/Fastenal class) — VWH signal. Never gates tier.",
    },
    "flooring": {
        "label": "Flooring & Tile Supply",
        "naics": "442210",
        "description": "Contractor flooring warehouse (Floor & Decor class) — VWH signal. Never gates tier.",
    },
    "tool_rental": {
        "label": "Tool & Equipment Rental",
        "naics": "532412",
        "description": "Equipment rental branch — VWH signal; deliberate hardware adjacency. Never gates tier.",
    },
    "lumber": {
        "label": "Lumber & Building Materials",
        "naics": "444190",
        "description": "Lumber yard or building materials dealer — VWH signal. Never gates tier.",
    },
    # ── VWH Tier B enrichment categories ─────────────────────────────────────────
    "plumbing": {
        "label": "Plumbing & HVAC Supply",
        "naics": "423720",
        "description": "Plumbing/HVAC trade counter — VWH Tier B signal. Never gates tier.",
    },
    "electrical": {
        "label": "Electrical Supply",
        "naics": "423610",
        "description": "Electrical trade counter — VWH Tier B signal. Never gates tier.",
    },
    "welding": {
        "label": "Welding & Industrial Gas",
        "naics": "424690",
        "description": "Welding supply / industrial gas — VWH Tier B signal. Never gates tier.",
    },
    # ── PKS archetype commercial signal ──────────────────────────────────────────
    "car_rental": {
        "label": "Car Rental",
        "naics": "532111",
        "description": "Car rental branch — PKS defining commercial signal. Never gates tier.",
    },
}

# Retail-only set used in tier_of()
# VWH/PKS enrichment categories are intentionally excluded — they never gate PRO tier logic.
_RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "sport", "electronics"}

# ── CIVIC SCALE THRESHOLDS ────────────────────────────────────────────────────
THRESHOLDS = {
    "hospital_min_beds":          150,   # CBRE threshold; OSM bed_count gate
    "university_min_enrolment":   1000,  # students; ETER/HESA/IPEDS
}

# ── GEOMETRIC TIER GATES ──────────────────────────────────────────────────────
# T2 District requires the anchor pair to form a unified retail node.
# Above this span the two stores likely serve separate trade areas (separate trips),
# so the cluster downgrades to T3 Local regardless of anchor composition.
# Calibrated so that NA T3 approaches EU parity (~24 % vs ~26 %).
# Change B: introduced 2026-05-28.
SPAN_T2_MAX_KM: float = 2.5

# ── 17 DISPLAY COUNTRIES (grouped) ───────────────────────────────────────────
DISPLAY_COUNTRIES = {
    "NA":          ["US", "CA", "MX"],
    "UK":          ["GB"],
    "NORDICS":     ["SE", "DK", "NO", "FI", "IS"],
    "CONTINENTAL": ["FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT"],
}

# Flat ordered list for iteration
ALL_DISPLAY_ISO = [
    iso for group in DISPLAY_COUNTRIES.values() for iso in group
]

# ── BRAND FILL ────────────────────────────────────────────────────────────────
# BRAND_FILL[category][iso] = [chain_id, ...]
# First entry = flagship (#1 slot).  All entries contribute to category detection.
# chain_id must match a .jsonl file in service-fs/service-business/.

BRAND_FILL: dict[str, dict[str, list[str]]] = {

    "hypermarket": {
        "US": [
            "walmart-us",           # Fortune #1 — canonical NA hypermarket
            "target-us",            # Mass-merchandise
            "whole-foods-us",       # Premium grocery (~528 stores)
            "heb-us",               # TX/MX regional (~340 stores)
            "wegmans-us",           # NE US large-format (~110 stores)
            "winco-foods-us",       # Bulk-style grocery W/SW US
            "sprouts-us",           # Natural grocery Sun Belt
            "fred-meyer-us",        # PNW-regional (Kroger sub)
        ],
        "CA": [
            "walmart-ca",
            "real-canadian-superstore-ca",
        ],
        "MX": [
            "walmart-mx",
            "soriana-mx",
            "chedraui-mx",
        ],
        "GB": [
            "tesco-uk",
            "sainsburys-uk",
            "asda-uk",
            "morrisons-uk",
        ],
        "FR": [
            "leclerc-fr",
            "carrefour-hypermarket-fr",
            "auchan-fr",
            "geant-casino-fr",       # 10 records, Q2901839 (Casino Group) — Phase 18 2026-05-22
            "intermarche-hyper-fr",  # 56 records, Q2029154 (Les Mousquetaires) — Phase 18 2026-05-22
        ],
        "DE": [
            "kaufland-de",
            "ecenter-de",
            "marktkauf-de",
            "globus-de",  # 125 records, Q528681 (Globus Holding) — Phase 18 2026-05-22
        ],
        "ES": [
            "mercadona-es",
            "carrefour-hypermarket-es",  # 326 records, Q217599 — Phase 17 2026-05-22
            "alcampo-es",               # 323 records, Q2832081 (Auchan ES) — Phase 17 2026-05-22
            "leclerc-es",               # 220 records — Phase 17 2026-05-22
        ],
        "IT": [
            "esselunga-it",
            "carrefour-hypermarket-it",  # 215 records, Q217599 — Phase 17 2026-05-22
            "famila-it",                # 215 records (Selex group) — Phase 17 2026-05-22
            "ipercoop-it",             # 101 records (Coop Italia hyper) — Phase 17 2026-05-22
        ],
        "GR": [
            "sklavenitis-gr",
        ],
        "PL": [
            "auchan-pl",               # 120 records, Q758603 — Phase 17 2026-05-22
            "carrefour-hypermarket-pl", # 114 records, Q217599 — Phase 17 2026-05-22
            "kaufland-pl",             # 253 records, Q685967 (Schwarz Group) — Phase 18 2026-05-22
            "leclerc-pl",              # 36 records, Q1273376 — Phase 18 2026-05-22
        ],
        "AT": [
            "billa-plus-at",
            "interspar-at",  # 85 records, Q1364056 (SPAR Austria) — Phase 18 2026-05-22
        ],
        "NL": [
            "albert-heijn-xl-nl",
            "jumbo-nl",  # 8 records, Q14716185 (Jumbo Foodmarkt large-format) — Phase 18 2026-05-22
        ],
        "PT": [
            "continente-pt",
            "auchan-pt",   # 31 stores, Q758603 — Phase 20 2026-05-24
        ],
        "SE": [
            "coop-forum-se",
            "maxi-ica-se",  # 50 records, Q104553487 (ICA Maxi Stormarknad) — Phase 17 2026-05-22
        ],
        "DK": [
            "bilka-dk",
            "foetex-dk",  # 103 records, Q3093871 (Salling Group) — Phase 18 2026-05-22
        ],
        "NO": [
            "obs-coop-no",
        ],
        "FI": [
            "k-citymarket-fi",
            "prisma-fi",
        ],
        "IS": [
            "hagkaup-is",
        ],
    },

    "hardware": {
        "US": [
            "home-depot-us",
            "lowes-us",
            "menards-us",
        ],
        "CA": [
            "home-depot-ca",
            "canadian-tire-ca",
            # lowes-ca: dropped — Lowe's exited Canada
            "peavey-mart-ca",
        ],
        "MX": [
            "home-depot-mx",
        ],
        "GB": [
            "bq-uk",
            "wickes-uk",  # 236 records, Q7998350 (Travis Perkins) — Phase 18 2026-05-22
        ],
        "FR": [
            "leroy-merlin-fr",
            "castorama-fr",
            "bricomarch-fr",  # 497 records, Q2896882 (Les Mousquetaires) — Phase 18 2026-05-22
            "brico-depot-fr", # 137 records, Q3007003 (Kingfisher) — Phase 18 2026-05-22
        ],
        "DE": [
            "hornbach-de",
            "obi-de",
            "bauhaus-de",
            "toom-baumarkt-de",
            "hagebaumarkt-de",
        ],
        "ES": [
            "leroy-merlin-es",
            "brico-depot-es",
            "bauhaus-es",
        ],
        "IT": [
            "leroy-merlin-it",
            "obi-it",
            "bricocenter-it",
        ],
        "GR": [
            "leroy-merlin-gr",
            "praktiker-gr",
        ],
        "PL": [
            "leroy-merlin-pl",
            "castorama-pl",
            "obi-pl",
        ],
        "AT": [
            "hornbach-at",
            "obi-at",      # ~50 stores, Q316004 — Phase 20 2026-05-24
            "bauhaus-at",  # ~18 stores, Q532716 — Phase 20 2026-05-24
        ],
        "NL": [
            "praxis-nl",
            "gamma-nl",
            "karwei-nl",
        ],
        "PT": [
            "leroy-merlin-pt",
        ],
        "SE": [
            "bauhaus-se",
            # clas-ohlson-se demoted 2026-05-22: small-format housewares, not home-improvement anchor
        ],
        "DK": [
            "imerco-dk",
            "silvan-dk",
            "bauhaus-dk",  # 20 records, Q532716 — Phase 18 2026-05-22
        ],
        "NO": [
            "obs-bygg-no",
            "bauhaus-no",  # 2 records (OSM sparse in NO), Q532716 — Phase 18 2026-05-22
        ],
        "FI": [
            "k-rauta-fi",
            "bauhaus-fi",  # 6 records, Q532716 — Phase 18 2026-05-22
        ],
        "IS": [
            "husasmidjan-is",
            "byko-is",
        ],
    },

    "price_club": {
        "US": [
            "costco-us",
            "sams-club-us",
            "bjs-wholesale-us",
        ],
        "CA": [
            "costco-ca",
        ],
        "MX": [
            "costco-mx",
            "sams-club-mx",
        ],
        "GB": [
            "costco-uk",
        ],
        "FR": [
            "costco-fr",
        ],
        "DE": [],    # no Costco in DE; Metro/Selgros are B2B, excluded
        "ES": [
            "costco-es",
            # makro-es demoted 2026-05-24: B2B trade-only (Metro AG parent, trade card required)
        ],
        "IT": [],
        "GR": [],
        "PL": [
            # makro-pl demoted 2026-05-24: B2B trade-only
        ],
        "AT": [],
        "NL": [
            # makro-nl demoted 2026-05-24: B2B trade-only
        ],
        "PT": [
            # makro-pt demoted 2026-05-24: B2B trade-only
        ],
        "SE": [
            "costco-se",
        ],
        "DK": [],
        "NO": [],
        "FI": [],
        "IS": [
            "costco-is",
        ],
    },

    "lifestyle": {
        "US": ["ikea-us"],
        "CA": ["ikea-ca"],
        "MX": ["ikea-mx"],
        "GB": ["ikea-uk"],
        "FR": ["ikea-fr", "xxxlutz-fr"],   # xxxlutz-fr ~5 stores — Phase 21 2026-05-24
        "DE": ["ikea-de", "xxxlutz-de", "hoeffner-de"],  # xxxlutz-de ~90, hoeffner-de ~25 — Phase 21 2026-05-24
        "ES": ["ikea-es"],
        "IT": ["ikea-it"],
        "GR": ["ikea-gr"],
        "PL": ["ikea-pl"],
        "AT": ["ikea-at", "xxxlutz-at"],   # xxxlutz-at ~47 stores — Phase 21 2026-05-24
        "NL": ["ikea-nl"],
        "PT": ["ikea-pt"],
        "SE": ["ikea-se", "xxxlutz-se"],   # xxxlutz-se ~5 stores — Phase 21 2026-05-24
        "DK": ["ikea-dk"],
        "NO": ["ikea-no"],
        "FI": ["ikea-fi"],
        "IS": [],       # no IKEA in Iceland
    },

    "sport": {
        "US": ["rei-us", "bass-pro-shops-us", "cabelas-us"],
        "CA": ["decathlon-ca"],
        "MX": ["decathlon-mx"],   # ~14 stores, Q509349 — Phase 20 2026-05-24
        "GB": ["decathlon-gb"],
        "FR": ["decathlon-fr"],
        "DE": ["decathlon-de"],
        "ES": ["decathlon-es"],
        "IT": ["decathlon-it"],
        "GR": ["decathlon-gr"],   # ~6 stores, Q509349 — Phase 20 2026-05-24
        "NL": ["decathlon-nl"],
        "PL": ["decathlon-pl"],
        "PT": ["decathlon-pt"],
        "AT": ["decathlon-at"],   # ~6 stores, Q509349 — Phase 20 2026-05-24
        "SE": ["decathlon-se", "xxl-se"],  # xxl-se ~31 stores, Q5447082 — Phase 20 2026-05-24
        "DK": ["decathlon-dk"],
        "NO": ["decathlon-no", "xxl-no"],  # xxl-no ~39 stores, Q5447082 — Phase 20 2026-05-24
        "FI": ["decathlon-fi", "xxl-fi"],  # xxl-fi ~15 stores, Q5447082 — Phase 20 2026-05-24
    },

    "electronics": {
        "US": [],   # Best Buy deferred to Phase 22 (counter-factual analysis first)
        "CA": [],
        "MX": [],
        "GB": [],   # Currys deferred to Phase 22 (UK coverage expansion)
        "FR": ["boulanger-fr", "darty-fr"],   # Mulliez big-box + large-format Darty — Phase 21 2026-05-24
        "DE": ["mediamarkt-de", "saturn-de"], # saturn-de rebranding in flight — Phase 21 2026-05-24
        "ES": ["mediamarkt-es"],              # Phase 21 2026-05-24
        "IT": ["mediaworld-it"],              # MediaMarkt branded as MediaWorld in IT — Phase 21 2026-05-24
        "GR": ["mediamarkt-gr"],              # Phase 21 2026-05-24
        "PL": ["mediamarkt-pl"],              # Phase 21 2026-05-24
        "AT": ["mediamarkt-at"],              # Phase 21 2026-05-24
        "NL": ["mediamarkt-nl"],              # Phase 21 2026-05-24
        "PT": [],
        "SE": ["mediamarkt-se"],              # Phase 21 2026-05-24
        "DK": [],
        "NO": [],
        "FI": [],
        "IS": [],
    },

    # Civic categories have no BRAND_FILL — detected from OSM civic JSONL
    "medical":   {},
    "education": {},

    # VWH enrichment categories — not gating tier; BRAND_FILL drives ingest-osm.py
    "auto_parts": {
        "US": ["autozone-us", "oreilley-auto-us", "napa-us"],
        "CA": ["napa-ca", "partsource-ca"],                    # Phase 2 2026-06-01
        "MX": ["autozone-mx"],                                  # Phase 2 2026-06-01
        "GB": ["halfords-uk"],
        # EU auto-parts deepening (multi-country EU ingests) — Phase 2 2026-06-01
        "FR": ["norauto-fr", "feuvert-fr", "euromaster-eu"],
        "DE": ["atu-de", "euromaster-eu"],
        "ES": ["norauto-fr", "feuvert-fr"],
        "IT": ["norauto-fr", "euromaster-eu"],
        "PL": ["norauto-fr"],
        "NL": ["euromaster-eu"],
        "PT": ["feuvert-fr"],
        "GR": [], "AT": ["atu-de"],
        "SE": ["euromaster-eu"], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    "paint": {
        "US": ["sherwin-williams-us"],
        "CA": [], "MX": ["comex-mx"],                           # Phase 2 2026-06-01
        "GB": [], "FR": [], "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    # ── VWH Tier A ────────────────────────────────────────────────────────────────
    "mro_industrial": {
        "US": ["fastenal-us", "grainger-us"],
        "CA": ["princess-auto-ca"],                            # Phase 2 2026-06-01
        "MX": ["truper-mx"],                                   # Phase 3 2026-06-14
        "GB": ["wurth-de"],
        "FR": ["wurth-de"], "DE": ["wurth-de"], "ES": ["wurth-de"],
        "IT": ["wurth-de"], "GR": [], "PL": ["wurth-de"],
        "AT": ["wurth-de"], "NL": ["wurth-de"], "PT": [],
        "SE": ["wurth-de"], "DK": ["wurth-de"], "NO": ["wurth-de"],
        "FI": ["wurth-de"], "IS": [],
    },
    "flooring": {
        "US": ["floor-decor-us"],
        "GB": ["topps-tiles-uk"],
        "CA": [], "MX": [],
        "FR": [], "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    "tool_rental": {
        "US": ["united-rentals-us", "sunbelt-rentals-us"],
        "CA": ["united-rentals-us"],
        "MX": [],
        "GB": ["hss-hire-uk", "speedy-hire-uk"],
        "FR": ["loxam-fr", "kiloutou-fr"],
        "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": ["boels-rental-nl"], "PT": [],
        "SE": [], "DK": [], "NO": [],
        "FI": ["ramirent-fi"], "IS": [],
    },
    "lumber": {
        "US": ["84-lumber-us", "builders-firstsource-us"],
        "CA": ["kent-building-supplies-ca"],
        "MX": [],
        "GB": [], "FR": [], "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    # ── VWH Tier B ────────────────────────────────────────────────────────────────
    "plumbing": {
        "US": ["ferguson-us"],
        "GB": ["wolseley-uk"],
        "CA": [], "MX": [],
        "FR": [], "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    "electrical": {
        "US": [],
        "GB": ["cef-uk"],
        "FR": ["rexel-fr"], "DE": ["rexel-fr"], "ES": ["rexel-fr"],
        "IT": ["rexel-fr"], "NL": ["rexel-fr"],
        "CA": [], "MX": [], "GR": [], "PL": [],
        "AT": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    "welding": {
        "GB": ["boc-uk"],
        "US": [], "CA": [], "MX": [],
        "FR": [], "DE": [], "ES": [], "IT": [], "GR": [], "PL": [],
        "AT": [], "NL": [], "PT": [],
        "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
    },
    # ── PKS commercial signal ─────────────────────────────────────────────────────
    "car_rental": {
        "US": ["enterprise-us", "hertz-us", "avis-us"],
        "CA": ["enterprise-us", "hertz-us", "avis-us"],
        "MX": [],
        "GB": ["europcar-fr", "sixt-de"],
        "FR": ["europcar-fr", "sixt-de"],
        "DE": ["sixt-de", "europcar-fr"],
        "ES": ["europcar-fr", "sixt-de"],
        "IT": ["europcar-fr", "sixt-de"],
        "GR": [], "PL": [],
        "AT": ["sixt-de"], "NL": ["europcar-fr", "sixt-de"], "PT": [],
        "SE": ["sixt-de"], "DK": ["sixt-de"], "NO": [], "FI": [], "IS": [],
    },
}

# ── CONTINENT MAPPING ─────────────────────────────────────────────────────────
ISO_TO_CONTINENT: dict[str, str] = {
    "US": "NA", "CA": "NA", "MX": "NA",
    "GB": "EU", "FR": "EU", "DE": "EU", "ES": "EU",
    "IT": "EU", "GR": "EU", "PL": "EU", "AT": "EU",
    "NL": "EU", "PT": "EU",
    "SE": "EU", "DK": "EU", "NO": "EU", "FI": "EU", "IS": "EU",
}

# ── DISPLAY NAMES ─────────────────────────────────────────────────────────────
# Flat chain_id → display name for UI
DISPLAY_NAMES: dict[str, str] = {
    # Hypermarket
    "walmart-us": "Walmart", "walmart-ca": "Walmart", "walmart-mx": "Walmart",
    "real-canadian-superstore-ca": "Real Canadian Superstore",
    "target-us": "Target",
    "whole-foods-us": "Whole Foods Market",
    "heb-us": "H-E-B",
    "wegmans-us": "Wegmans",
    "winco-foods-us": "WinCo Foods",
    "sprouts-us": "Sprouts Farmers Market",
    "fred-meyer-us": "Fred Meyer",
    "soriana-mx": "Soriana",
    "chedraui-mx": "Chedraui",
    "tesco-uk": "Tesco",
    "sainsburys-uk": "Sainsbury's",
    "asda-uk": "ASDA",
    "morrisons-uk": "Morrisons",
    "leclerc-fr": "E.Leclerc", "leclerc-pl": "E.Leclerc",
    "carrefour-hypermarket-fr": "Carrefour",
    "auchan-fr": "Auchan",
    "geant-casino-fr": "Géant Casino", "intermarche-hyper-fr": "Intermarché Hyper",
    "kaufland-de": "Kaufland", "kaufland-pl": "Kaufland",
    "ecenter-de": "E center",
    "marktkauf-de": "Marktkauf", "globus-de": "Globus",
    "mercadona-es": "Mercadona",
    "esselunga-it": "Esselunga",
    "sklavenitis-gr": "Sklavenitis",
    "billa-plus-at": "Billa Plus", "interspar-at": "Interspar",
    "albert-heijn-xl-nl": "Albert Heijn XL", "jumbo-nl": "Jumbo Foodmarkt",
    "continente-pt": "Continente", "auchan-pt": "Auchan",
    "coop-forum-se": "Coop Forum / Stora Coop",
    "bilka-dk": "Bilka", "foetex-dk": "Føtex",
    "obs-coop-no": "OBS Coop",
    "k-citymarket-fi": "K-Citymarket",
    "prisma-fi": "Prisma",
    "hagkaup-is": "Hagkaup",
    # Hardware
    "home-depot-us": "The Home Depot",
    "home-depot-ca": "The Home Depot",
    "home-depot-mx": "The Home Depot",
    "lowes-us": "Lowe's",
    "menards-us": "Menards",
    "alaska-industrial-hardware-us": "Alaska Industrial Hardware",
    "canadian-tire-ca": "Canadian Tire",
    "peavey-mart-ca": "Peavey Mart",
    "bq-uk": "B&Q", "wickes-uk": "Wickes",
    "leroy-merlin-fr": "Leroy Merlin",
    "castorama-fr": "Castorama", "bricomarch-fr": "Bricomarché", "brico-depot-fr": "Brico Dépôt",
    "hornbach-de": "Hornbach",
    "obi-de": "OBI",
    "bauhaus-de": "Bauhaus",
    "toom-baumarkt-de": "toom Baumarkt",
    "hagebaumarkt-de": "Hagebaumarkt",
    "leroy-merlin-es": "Leroy Merlin",
    "brico-depot-es": "Brico Dépôt",
    "bauhaus-es": "Bauhaus",
    "leroy-merlin-it": "Leroy Merlin",
    "obi-it": "OBI",
    "bricocenter-it": "Bricocenter",
    "leroy-merlin-gr": "Leroy Merlin",
    "praktiker-gr": "Praktiker",
    "leroy-merlin-pl": "Leroy Merlin",
    "castorama-pl": "Castorama",
    "obi-pl": "OBI",
    "hornbach-at": "Hornbach",
    "obi-at": "OBI", "bauhaus-at": "Bauhaus",
    "praxis-nl": "Praxis",
    "gamma-nl": "Gamma",
    "karwei-nl": "Karwei",
    "leroy-merlin-pt": "Leroy Merlin",
    "bauhaus-se": "Bauhaus", "bauhaus-dk": "Bauhaus", "bauhaus-no": "Bauhaus", "bauhaus-fi": "Bauhaus",
    "clas-ohlson-se": "Clas Ohlson",
    "imerco-dk": "Imerco",
    "silvan-dk": "Silvan",
    "obs-bygg-no": "OBS Bygg",
    "k-rauta-fi": "K-Rauta",
    "husasmidjan-is": "Húsasmiðjan",
    "byko-is": "Byko",
    # Price club
    "costco-us": "Costco", "costco-ca": "Costco", "costco-mx": "Costco",
    "costco-uk": "Costco", "costco-fr": "Costco", "costco-es": "Costco",
    "costco-se": "Costco", "costco-is": "Costco",
    "sams-club-us": "Sam's Club", "sams-club-mx": "Sam's Club",
    "bjs-wholesale-us": "BJ's Wholesale Club",
    "makro-es": "Makro", "makro-nl": "Makro", "makro-pl": "Makro", "makro-pt": "Makro",
    # Sport
    "decathlon-fr": "Décathlon", "decathlon-de": "Decathlon", "decathlon-gb": "Decathlon",
    "decathlon-es": "Decathlon", "decathlon-it": "Decathlon", "decathlon-nl": "Decathlon",
    "decathlon-pl": "Decathlon", "decathlon-pt": "Decathlon", "decathlon-se": "Decathlon",
    "decathlon-dk": "Decathlon", "decathlon-no": "Decathlon", "decathlon-fi": "Decathlon",
    "decathlon-ca": "Decathlon",
    "decathlon-at": "Decathlon", "decathlon-gr": "Decathlon", "decathlon-mx": "Decathlon",
    "xxl-no": "XXL", "xxl-se": "XXL", "xxl-fi": "XXL",
    "rei-us": "REI", "bass-pro-shops-us": "Bass Pro Shops", "cabelas-us": "Cabela's",
    # Lifestyle
    "ikea-us": "IKEA", "ikea-ca": "IKEA", "ikea-mx": "IKEA",
    "ikea-uk": "IKEA", "ikea-fr": "IKEA", "ikea-de": "IKEA",
    "ikea-es": "IKEA", "ikea-it": "IKEA", "ikea-gr": "IKEA",
    "ikea-pl": "IKEA", "ikea-at": "IKEA", "ikea-nl": "IKEA",
    "ikea-pt": "IKEA", "ikea-se": "IKEA", "ikea-dk": "IKEA",
    "ikea-no": "IKEA", "ikea-fi": "IKEA",
    # Electronics (Phase 21)
    "mediamarkt-de": "MediaMarkt", "saturn-de": "Saturn",
    "mediamarkt-at": "MediaMarkt", "mediamarkt-nl": "MediaMarkt",
    "mediamarkt-es": "MediaMarkt", "mediaworld-it": "MediaWorld",
    "mediamarkt-gr": "MediaMarkt", "mediamarkt-pl": "MediaMarkt",
    "mediamarkt-se": "MediaMarkt",
    "boulanger-fr": "Boulanger", "darty-fr": "Darty",
    # Lifestyle additions (Phase 21)
    "xxxlutz-at": "XXXLutz", "xxxlutz-de": "XXXLutz",
    "xxxlutz-se": "XXXLutz", "xxxlutz-fr": "XXXLutz",
    "hoeffner-de": "Höffner",
    # VWH — auto_parts (Phase 1, 2026-06-01)
    "autozone-us": "AutoZone",
    "oreilley-auto-us": "O'Reilly Auto Parts",
    "napa-us": "NAPA Auto Parts",
    "halfords-uk": "Halfords",
    # VWH — paint (Phase 1, 2026-06-01)
    "sherwin-williams-us": "Sherwin-Williams",
    # VWH — mro_industrial (Phase 1, 2026-06-01; Phase 3 MX 2026-06-14)
    "truper-mx": "Truper",
    "wurth-de": "Würth",
    "fastenal-us": "Fastenal",
    "grainger-us": "Grainger",
    "hilti-ch": "Hilti",
    # VWH — flooring (Phase 1, 2026-06-01)
    "floor-decor-us": "Floor & Decor",
    "topps-tiles-uk": "Topps Tiles",
    # VWH — tool_rental (Phase 1, 2026-06-01)
    "united-rentals-us": "United Rentals",
    "sunbelt-rentals-us": "Sunbelt Rentals",
    "loxam-fr": "Loxam",
    "kiloutou-fr": "Kiloutou",
    "hss-hire-uk": "HSS Hire",
    "speedy-hire-uk": "Speedy Hire",
    "boels-rental-nl": "Boels Rental",
    "ramirent-fi": "Ramirent",
    # VWH — lumber (Phase 1, 2026-06-01)
    "84-lumber-us": "84 Lumber",
    "builders-firstsource-us": "Builders FirstSource",
    "kent-building-supplies-ca": "Kent Building Supplies",
    # VWH — plumbing (Phase 1, 2026-06-01)
    "ferguson-us": "Ferguson",
    "wolseley-uk": "Wolseley",
    # VWH — electrical (Phase 1, 2026-06-01)
    "rexel-fr": "Rexel",
    "cef-uk": "City Electrical Factors",
    # VWH — welding (Phase 1, 2026-06-01)
    "boc-uk": "BOC",
    # PKS — car_rental (Phase 1, 2026-06-01)
    "enterprise-us": "Enterprise Rent-A-Car",
    "hertz-us": "Hertz",
    "avis-us": "Avis",
    "sixt-de": "Sixt",
    "europcar-fr": "Europcar",
}

# ── REVERSE INDEX: chain_id → category ───────────────────────────────────────
_CHAIN_TO_CAT: dict[str, str] = {}
for _cat, _by_iso in BRAND_FILL.items():
    if _cat in ("medical", "education"):
        continue
    for _iso, _chains in _by_iso.items():
        for _cid in _chains:
            _CHAIN_TO_CAT[_cid] = _cat


# ── HELPER FUNCTIONS ──────────────────────────────────────────────────────────

def category_of(chain_id: str) -> str | None:
    """Return the category key for a chain_id, or None if not in taxonomy."""
    return _CHAIN_TO_CAT.get(chain_id)


def tier_of(cats: set[str], tight: bool = False, span_km: float | None = None) -> int | None:
    """
    Composition-and-geometry tier rule.  cats = set of retail category keys present.
    Civic categories (medical, education) are ignored — they never gate the tier.

    T1 Regional:  hypermarket ∧ hardware ∧ (price_club ∨ lifestyle ∨ electronics)
                  OR tight cluster with 3+ members (H2b rule)
                  OR any 4+ retail anchor categories (destination cluster)
    T2 District:  hypermarket ∧ hardware ∧ span_km ≤ SPAN_T2_MAX_KM (2.5 km)
    T3 Local:     all remaining co-locations (including hyper+hw with span > 2.5 km)
    None:         singleton — not a co-location

    Three T1 admission paths (T1.a / T1.b / T1.c):
      T1.a — tripartite composition: hyper + hw + one of (pc, lifestyle, electronics).
             Electronics is load-bearing for EU T1: MediaMarkt/Saturn/Boulanger
             clusters that include a hypermarket and hardware anchor qualify.
      T1.b — H2b compact: tight cluster with ≥3 members, any composition.
             Addresses dense EU retail parks lacking a price_club anchor.
      T1.c — category breadth: 4+ distinct retail anchor categories.

    Change A (code-paper alignment): T2 now requires has_hyper AND has_hw.
    The prior `has_hyper and n >= 2` rule promoted hypermarket-only clusters
    to T2; that was a code bug relative to the intended compositional definition.
    Removes ~621 clusters from T2 → T3.

    Change B (geometric span gate): T2 further requires span_km ≤ SPAN_T2_MAX_KM.
    A hypermarket+hardware pair with span > 2.5 km likely serves separate trade
    areas (separate customer trips); downgraded to T3 Local.
    Effect: ~667 T2 → T3 globally; NA T3 +44 %, EU T3 +54 %.
    """
    retail = cats & _RETAIL_CATS
    n = len(retail)
    if n < 2:
        return None

    has_hyper = "hypermarket" in retail
    has_hw    = "hardware"    in retail
    has_pc    = "price_club"  in retail
    has_life  = "lifestyle"   in retail
    has_elec  = "electronics" in retail

    # T1.a — tripartite: hyper + hw + (price_club or lifestyle or electronics)
    if has_hyper and has_hw and (has_pc or has_life or has_elec):
        return 1
    # T1.b — H2b: compact multi-anchor regardless of composition
    if tight and n >= 3:
        return 1
    # T1.c — category breadth ≥4
    if n >= 4:
        return 1
    # T2 — hypermarket + hardware within span gate (Change B)
    if has_hyper and has_hw:
        if span_km is None or span_km <= SPAN_T2_MAX_KM:
            return 2
        return 3  # span > 2.5 km: stores too dispersed for a unified district node
    return 3


def slots_for(iso: str, category: str) -> list[str]:
    """Return the list of chain_ids for (iso, category). Empty list if not present."""
    return BRAND_FILL.get(category, {}).get(iso, [])


def all_chains_for_iso(iso: str) -> dict[str, str]:
    """Return {chain_id: category} for all retail chains declared for iso."""
    result = {}
    for cat in ("hypermarket", "hardware", "price_club", "lifestyle", "sport", "electronics"):
        for cid in slots_for(iso, cat):
            result[cid] = cat
    return result


def ring_radius_km(tier: int) -> float:
    """Canonical ring radius: T1-tight → 1.0 km, all others → 3.0 km.
    Caller passes the `tight_intact` flag separately for the tight test."""
    return 1.0 if tier == 1 else 3.0
