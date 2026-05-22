"""
taxonomy.py — Declarative co-location taxonomy.

Single source of truth replacing the scattered ALPHA_* sets in config.py,
CHAIN_FAMILY in build-tiles.py, and CHAIN_META in simulate-dbscan-ab.py.

Import:
    from taxonomy import CATEGORIES, BRAND_FILL, THRESHOLDS, DISPLAY_COUNTRIES
    from taxonomy import category_of, tier_of, slots_for
"""

# ── 6 CANONICAL CATEGORIES ────────────────────────────────────────────────────
# 4 retail anchors + 2 civic anchors.  Civic never gate the tier.
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
}

# Retail-only set used in tier_of()
_RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle"}

# ── CIVIC SCALE THRESHOLDS ────────────────────────────────────────────────────
THRESHOLDS = {
    "hospital_min_beds":          150,   # CBRE threshold; OSM bed_count gate
    "university_min_enrolment":   1000,  # students; ETER/HESA/IPEDS
}

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
        ],
        "DE": [
            "kaufland-de",
            "ecenter-de",
            "marktkauf-de",
        ],
        "ES": [
            "mercadona-es",
        ],
        "IT": [
            "esselunga-it",
        ],
        "GR": [
            "sklavenitis-gr",
        ],
        "PL": [],    # no ingested hypermarket anchor in PL yet
        "AT": [
            "billa-plus-at",
        ],
        "NL": [
            "albert-heijn-xl-nl",
        ],
        "PT": [
            "continente-pt",
        ],
        "SE": [
            "coop-forum-se",
        ],
        "DK": [
            "bilka-dk",
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
        ],
        "FR": [
            "leroy-merlin-fr",
            "castorama-fr",
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
            "clas-ohlson-se",
        ],
        "DK": [
            "imerco-dk",
            "silvan-dk",
        ],
        "NO": [
            "obs-bygg-no",
        ],
        "FI": [
            "k-rauta-fi",
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
            "makro-es",
        ],
        "IT": [],
        "GR": [],
        "PL": [
            "makro-pl",
        ],
        "AT": [],
        "NL": [
            "makro-nl",
        ],
        "PT": [],
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
        "FR": ["ikea-fr"],
        "DE": ["ikea-de"],
        "ES": ["ikea-es"],
        "IT": ["ikea-it"],
        "GR": ["ikea-gr"],
        "PL": ["ikea-pl"],
        "AT": ["ikea-at"],
        "NL": ["ikea-nl"],
        "PT": ["ikea-pt"],
        "SE": ["ikea-se"],
        "DK": ["ikea-dk"],
        "NO": ["ikea-no"],
        "FI": ["ikea-fi"],
        "IS": [],       # no IKEA in Iceland
    },

    # Civic categories have no BRAND_FILL — detected from OSM civic JSONL
    "medical":   {},
    "education": {},
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
    "leclerc-fr": "E.Leclerc",
    "carrefour-hypermarket-fr": "Carrefour",
    "auchan-fr": "Auchan",
    "kaufland-de": "Kaufland",
    "ecenter-de": "E center",
    "marktkauf-de": "Marktkauf",
    "mercadona-es": "Mercadona",
    "esselunga-it": "Esselunga",
    "sklavenitis-gr": "Sklavenitis",
    "billa-plus-at": "Billa Plus",
    "albert-heijn-xl-nl": "Albert Heijn XL",
    "continente-pt": "Continente",
    "coop-forum-se": "Coop Forum / Stora Coop",
    "bilka-dk": "Bilka",
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
    "bq-uk": "B&Q",
    "leroy-merlin-fr": "Leroy Merlin",
    "castorama-fr": "Castorama",
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
    "praxis-nl": "Praxis",
    "gamma-nl": "Gamma",
    "karwei-nl": "Karwei",
    "leroy-merlin-pt": "Leroy Merlin",
    "bauhaus-se": "Bauhaus",
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
    "makro-es": "Makro", "makro-nl": "Makro", "makro-pl": "Makro",
    # Lifestyle
    "ikea-us": "IKEA", "ikea-ca": "IKEA", "ikea-mx": "IKEA",
    "ikea-uk": "IKEA", "ikea-fr": "IKEA", "ikea-de": "IKEA",
    "ikea-es": "IKEA", "ikea-it": "IKEA", "ikea-gr": "IKEA",
    "ikea-pl": "IKEA", "ikea-at": "IKEA", "ikea-nl": "IKEA",
    "ikea-pt": "IKEA", "ikea-se": "IKEA", "ikea-dk": "IKEA",
    "ikea-no": "IKEA", "ikea-fi": "IKEA",
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


def tier_of(cats: set[str]) -> int | None:
    """
    Composition-only tier rule.  cats = set of retail category keys present.
    Civic categories (medical, education) are ignored — they never gate the tier.

    T1 Regional:  hypermarket ∧ hardware ∧ (price_club ∨ lifestyle)
    T2 District:  hypermarket ∧ at least one other retail category
    T3 Local:     ≥ 2 distinct retail categories
    None:         singleton — not a co-location
    """
    retail = cats & _RETAIL_CATS
    n = len(retail)
    if n < 2:
        return None

    has_hyper = "hypermarket" in retail
    has_hw    = "hardware"    in retail
    has_pc    = "price_club"  in retail
    has_life  = "lifestyle"   in retail

    if has_hyper and has_hw and (has_pc or has_life):
        return 1
    if has_hyper and n >= 2:
        return 2
    return 3


def slots_for(iso: str, category: str) -> list[str]:
    """Return the list of chain_ids for (iso, category). Empty list if not present."""
    return BRAND_FILL.get(category, {}).get(iso, [])


def all_chains_for_iso(iso: str) -> dict[str, str]:
    """Return {chain_id: category} for all retail chains declared for iso."""
    result = {}
    for cat in ("hypermarket", "hardware", "price_club", "lifestyle"):
        for cid in slots_for(iso, cat):
            result[cid] = cat
    return result


def ring_radius_km(tier: int) -> float:
    """Canonical ring radius: T1-tight → 1.0 km, all others → 3.0 km.
    Caller passes the `tight_intact` flag separately for the tight test."""
    return 1.0 if tier == 1 else 3.0
