"""
app-orchestration-gis configuration.
"""
import os
from pathlib import Path

# Data Paths
TOTEBOX_DATA_PATH = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
BOUNDARIES_DIR    = TOTEBOX_DATA_PATH / "boundaries"
SERVICE_BUSINESS_CLEANSED = TOTEBOX_DATA_PATH / "service-business" / "cleansed-clusters.jsonl"
# OSM-sourced civic data (hospital + university from amenity= tags — much tighter than Overture)
SERVICE_PLACES_CLEANSED   = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-osm.jsonl"
SERVICE_PLACES_OVERTURE   = TOTEBOX_DATA_PATH / "service-places" / "cleansed-places.jsonl"
WORK_DIR          = Path(__file__).parent / "work"

# ── LEAPFROG 2030: ALPHA BRANDS — 4-CLASS TAXONOMY (2026-05-16) ──────────────
# Operator decision D5 (2026-05-16): 4 anchor classes replace the 3-set arrangement.
# IKEA exits ALPHA_ANCHORS into its own Lifestyle class.
# "Hypermarket" reserved for Walmart/Tesco/Soriana-class general-merchandise+grocery.
# Costco/Sam's/BJ's/Makro = Warehouse-Club (membership-bulk format).
# Home Depot/Leroy Merlin/B&Q etc. = Hardware (unchanged set).

# General-merchandise + grocery hypermarkets (daily-shopping format).
# Phase 5: carrefour-hypermarket-fr DONE (493 stores, Q217599, 2026-05-16).
# Phase 6: auchan-fr, leclerc-fr, ecenter-de, marktkauf-de DONE 2026-05-16.
# Phase 7: kaufland-de DONE 2026-05-16.
# Phase 8: fred-meyer-us DONE 2026-05-17 (126 records, Q5495932; PNW-regional; 63 new clusters, 0 T1).
# Phase 12: whole-foods-us promoted from GENERIC_FOOD; chedraui-mx promoted; heb-us, asda-uk, morrisons-uk ingested 2026-05-17.
# Phase 15: wegmans-us, winco-foods-us, sprouts-us ingested 2026-05-17.
# Phase 16: esselunga-it, sklavenitis-gr, billa-plus-at, continente-pt, albert-heijn-xl-nl added 2026-05-19.
ALPHA_HYPERMARKET = {
    "NA": {
        # Walmart: Fortune #1 — dominant NA hypermarket
        "walmart-us", "walmart-ca", "walmart-mx",
        # Target: Fortune-scale US mass-merchandise hypermarket
        "target-us",
        # Soriana: Mexican flagship hypermarket (~700 stores) — operator D1 Sprint 12
        "soriana-mx",
        # Fred Meyer: Kroger subsidiary; PNW-regional (92–132 stores); Phase 8 review pending
        "fred-meyer-us",
        # Whole Foods Market: premium US grocery (~528 stores, Q1758180) — Phase 12 promotion 2026-05-17
        "whole-foods-us",
        # H-E-B: Texas/MX regional hypermarket (~340 stores, Q1665088) — Phase 12 ingest 2026-05-17
        "heb-us",
        # Chedraui: Mexican hypermarket chain (~280 stores, Q2336803) — Phase 12 promotion 2026-05-17
        "chedraui-mx",
        # Wegmans: large-format NE US grocery (~110 stores, Q1182328) — Phase 15 ingest 2026-05-17
        "wegmans-us",
        # WinCo Foods: bulk/warehouse-style grocery OR/WA/ID/NV/UT/AZ/CA/TX (~140 stores, Q2584339) — Phase 15 2026-05-17
        "winco-foods-us",
        # Sprouts Farmers Market: natural grocery Sun Belt (~410 stores, Q7580917) — Phase 15 2026-05-17
        "sprouts-us",
    },
    "EU": {
        # Mercadona-ES: Spain's flagship hypermarket, 1,603 stores — operator D1 2026-05-16
        "mercadona-es",
        # Tesco-UK + Sainsbury's-UK: UK large-format hypermarkets — Phase 1 promotion 2026-05-16
        "tesco-uk", "sainsburys-uk",
        # ASDA-UK: Walmart UK subsidiary, large-format hypermarket (~600 stores, Q297410) — Phase 12 2026-05-17
        "asda-uk",
        # Morrisons-UK: UK large-format hypermarket (~500 stores, Q922344) — Phase 12 2026-05-17
        "morrisons-uk",
        # Nordic large-format hypermarkets (migrated from ALPHA_ANCHORS EU 2026-05-16)
        "bilka-dk", "obs-coop-no", "hagkaup-is", "k-citymarket-fi", "prisma-fi",
        # Carrefour FR: hypermarket-only variant (493 stores, Q217599) — Phase 5 2026-05-16
        "carrefour-hypermarket-fr",
        # Auchan FR: hypermarket-only (~130 stores, Q758603) — Phase 6 2026-05-16
        "auchan-fr",
        # E.Leclerc FR: dominant French hypermarket cooperative (~720 stores, Q1273376) — Phase 6 2026-05-16
        "leclerc-fr",
        # Edeka hypermarket formats DE: E center (~290, Q1101048) + Marktkauf (~310, Q1524300) — Phase 6 2026-05-16
        "ecenter-de", "marktkauf-de",
        # Kaufland DE: Schwarz Group hypermarket (~760 stores, Q685967) — Phase 7 2026-05-16
        "kaufland-de",
        # Phase 16 EU gap fills (2026-05-19):
        # Esselunga IT: N. Italian flagship hypermarket (~190 stores, Q1377048)
        "esselunga-it",
        # Sklavenitis GR: Greece #1 grocer + Marinopoulos acquisition (~600 stores, Q7536996)
        "sklavenitis-gr",
        # Billa Plus AT: REWE Group AT flagship hypermarket (~135 stores, Q806085)
        "billa-plus-at",
        # Continente PT: Sonae MC PT #1 hypermarket (~40 stores, Q5164541)
        "continente-pt",
        # Albert Heijn XL NL: Ahold Delhaize large-format hypermarket (~85 stores)
        "albert-heijn-xl-nl",
    }
}

# Destination furniture / home-goods (Lifestyle format).
ALPHA_LIFESTYLE = {
    "NA": {"ikea-us", "ikea-ca", "ikea-mx"},
    "EU": {
        "ikea-es", "ikea-it", "ikea-gr", "ikea-pl",
        # ikea-nordics split into per-country (pre-freeze fix 2026-05-22)
        "ikea-se", "ikea-dk", "ikea-no", "ikea-fi",
        "ikea-fr", "ikea-de", "ikea-uk",
        "ikea-at", "ikea-nl", "ikea-pt",
    },
}
ALPHA_HARDWARE = {
    # Home Depot is in BOTH ALPHA_ANCHORS (can initiate clusters) and ALPHA_HARDWARE
    # (contributes s_hw scoring when another Fortune-scale anchor is the cluster origin).
    "NA": {"home-depot-us", "home-depot-ca", "home-depot-mx", "alaska-industrial-hardware-us", "menards-us"},
    "EU": {
        # Primary large-format hardware anchors — sufficient alone for T2
        "leroy-merlin-es", "leroy-merlin-it", "leroy-merlin-gr", "leroy-merlin-pl",
        "leroy-merlin-fr", "leroy-merlin-pt",
        "castorama-pl",
        "castorama-fr",  # 118 records confirmed 2026-05-06 (promoted from GENERIC)
        "k-rauta-fi",
        "hornbach-de", "hornbach-at",
        "praxis-nl",
        "gamma-nl",      # 163 stores; Intergamma NL; pending first ingest
        "karwei-nl",     # 129 stores; Intergamma NL; pending first ingest
        "bauhaus-se",    # 40 records confirmed; primary HW anchor in SE
        "bq-uk",         # 356 records confirmed; primary HW anchor in GB
        "obs-bygg-no",   # 63 records confirmed; primary HW anchor in NO
    }
}
GENERIC_HARDWARE = {
    # lowes-ca removed 2026-05-22: Lowe's exited Canada; 1 stale record
    "NA": {"lowes-us", "canadian-tire-ca", "peavey-mart-ca"},
    "EU": {
        # ── TODO: these chains need better OSM brand:wikidata coverage ──────────
        # Each currently has 0 records. Once re-ingested with sufficient data,
        # promote the primary market chain to ALPHA_HARDWARE.
        # Suggested ingest method: query by name= tag if wikidata tag is sparse.
        #   obi-de / obi-it / obi-pl  → try name="OBI" query with country bbox
        #   bauhaus-de / bauhaus-es   → try name="Bauhaus" (bauhaus-se already promoted)
        #   castorama-fr → try name="Castorama" in FR bbox (separate from castorama-pl)
        #   brico-depot-es → try brand:wikidata=Q3007003
        #   imerco-dk / husasmidjan-is → Nordic specialty chains
        # (bq-uk promoted 2026-05-06; obs-bygg-no promoted 2026-05-06)
        # ────────────────────────────────────────────────────────────────────────
        "obi-de", "bauhaus-de",
        "obi-it", "obi-pl",
        "bauhaus-es",
        "imerco-dk", "husasmidjan-is",
        # Intentionally generic (smaller format than primary alpha chains)
        "brico-depot-es",   # Spain: second-tier vs Leroy Merlin
        "clas-ohlson-se",   # Nordics: home goods, not a full hardware anchor
        # Phase C additions — pending first ingest
        "toom-baumarkt-de",   # ~340 stores DE; REWE Group subsidiary
        "hagebaumarkt-de",    # ~350 stores DE; hagebau cooperative
        "bricocenter-it",     # ~50 stores IT; Adeo/Leroy Merlin subsidiary
        "silvan-dk",          # ~80 stores DK; Maxbo Group
        "praktiker-gr",       # ~28 stores GR; independent Greek operations
        "byko-is",            # ~12 stores IS; Iceland's primary HW chain
    }
}
ALPHA_WAREHOUSE = {
    # Membership warehouse-club format. Costco + Makro can both initiate clusters and
    # contribute s_wh scoring when another anchor is the cluster origin.
    "NA": {"costco-us", "costco-ca", "costco-mx", "sams-club-us", "sams-club-mx", "bjs-wholesale-us"},
    "EU": {
        "costco-es", "costco-se", "costco-is", "costco-uk", "costco-fr",
        "makro-es", "makro-nl", "makro-pl",
    }
}

# Generic categorization for substitution logic
GENERIC_FOOD = {"lidl-es", "safeway-ca", "biedronka-pl",
                "lidl-uk",
                "lidl-de", "lidl-fr", "lidl-nl", "lidl-at", "lidl-pt",
                "aldi-de", "aldi-uk", "aldi-nl", "aldi-pl"}
# Sprint 12 — soriana-mx promoted from Food to ALPHA_HYPERMARKET (operator decision A1).
# mercadona-es, tesco-uk, sainsburys-uk promoted to ALPHA_HYPERMARKET 2026-05-16 (operator D1/Phase 1).
# carrefour-hypermarket-fr: Phase 5 DONE 2026-05-16 (493 stores, Q217599 tag, hypermarket-only variant).
# Phase 12 — whole-foods-us, chedraui-mx promoted from Food/generic to ALPHA_HYPERMARKET 2026-05-17.

# ── REGION CONFIGURATION ─────────────────────────────────────────────────────
REGION_CONFIG = {
    # Dual membership: Fortune-scale chains appear in BOTH anchor (can initiate clusters)
    # AND hardware/warehouse (are found as secondaries by other anchors, contributing scores).
    "US": {
        "anchor":    ["walmart-us", "target-us", "fred-meyer-us", "whole-foods-us", "heb-us", "wegmans-us", "winco-foods-us", "sprouts-us", "ikea-us", "home-depot-us", "costco-us"],
        "hardware":  ["home-depot-us", "alaska-industrial-hardware-us", "lowes-us", "menards-us"],
        "warehouse": ["costco-us", "sams-club-us", "bjs-wholesale-us"]
    },
    "CA": {
        "anchor":    ["walmart-ca", "ikea-ca", "real-canadian-superstore-ca", "home-depot-ca", "costco-ca"],
        "hardware":  ["home-depot-ca", "canadian-tire-ca"],  # lowes-ca removed 2026-05-22
        "warehouse": ["costco-ca"]
    },
    "MX": {
        "anchor":    ["walmart-mx", "soriana-mx", "chedraui-mx", "ikea-mx", "home-depot-mx", "costco-mx"],
        "hardware":  ["home-depot-mx"],
        "warehouse": ["costco-mx", "sams-club-mx"]
    },
    "ES": {
        "anchor":    ["mercadona-es", "ikea-es", "costco-es", "makro-es"],
        "hardware":  ["leroy-merlin-es", "brico-depot-es", "bauhaus-es"],
        "warehouse": ["costco-es", "makro-es"]
    },
    "IT": {
        "anchor":    ["esselunga-it", "ikea-it"],
        "hardware":  ["leroy-merlin-it", "obi-it", "bricocenter-it"],
        "warehouse": ["metro-it"]
    },
    "GR": {
        "anchor":    ["sklavenitis-gr", "ikea-gr"],
        "hardware":  ["leroy-merlin-gr", "praktiker-gr"],
        "warehouse": ["the-mart-gr"]
    },
    "PL": {
        "anchor":    ["ikea-pl", "makro-pl"],
        "hardware":  ["leroy-merlin-pl", "obi-pl", "castorama-pl"],
        "warehouse": ["makro-pl", "selgros-pl"]
    },
    # NORDICS region kept for legacy build-clusters.py; new build-clusters-v2.py uses taxonomy.py per-ISO
    "NORDICS": {
        "anchor":    ["ikea-se", "ikea-dk", "ikea-no", "ikea-fi",
                      "bilka-dk", "prisma-fi", "k-citymarket-fi", "obs-coop-no", "hagkaup-is",
                      "costco-se", "costco-is", "coop-forum-se"],
        "hardware":  ["clas-ohlson-se", "k-rauta-fi", "imerco-dk", "obs-bygg-no",
                      "husasmidjan-is", "bauhaus-se", "silvan-dk", "byko-is"],
        "warehouse": ["costco-se", "costco-is"]
    },
    # Phase B: France, Germany, UK (data ingested 2026-05-05)
    # Phase 6 (2026-05-16): auchan-fr, leclerc-fr, carrefour-hypermarket-fr added to FR anchor;
    #   ecenter-de, marktkauf-de added to DE anchor (enables hyper_list → T1 composition predicate).
    "FR": {
        "anchor":    ["ikea-fr", "costco-fr", "carrefour-hypermarket-fr", "auchan-fr", "leclerc-fr"],
        "hardware":  ["leroy-merlin-fr", "castorama-fr"],
        "warehouse": ["costco-fr"]
    },
    "DE": {
        "anchor":    ["ikea-de", "ecenter-de", "marktkauf-de", "kaufland-de"],
        "hardware":  ["hornbach-de", "obi-de", "bauhaus-de", "toom-baumarkt-de", "hagebaumarkt-de"],
        "warehouse": ["metro-de", "selgros-de"]
    },
    "GB": {
        "anchor":    ["tesco-uk", "sainsburys-uk", "asda-uk", "morrisons-uk", "ikea-uk", "costco-uk"],
        "hardware":  ["bq-uk"],
        "warehouse": ["costco-uk"]
    },
    # Phase C: Austria, Netherlands, Portugal
    # Phase 16 (2026-05-19): billa-plus-at, albert-heijn-xl-nl, continente-pt added as anchors.
    "AT": {
        "anchor":    ["billa-plus-at", "ikea-at"],
        "hardware":  ["hornbach-at"],
        "warehouse": []
    },
    "NL": {
        "anchor":    ["albert-heijn-xl-nl", "ikea-nl", "makro-nl"],
        "hardware":  ["praxis-nl", "gamma-nl", "karwei-nl"],
        "warehouse": ["makro-nl"]
    },
    "PT": {
        "anchor":    ["continente-pt", "ikea-pt"],
        "hardware":  ["leroy-merlin-pt"],
        "warehouse": []
    },
}

ISO_TO_REGION = {
    "US": "US", "CA": "CA", "MX": "MX",
    "ES": "ES", "IT": "IT", "GR": "GR", "PL": "PL",
    "SE": "NORDICS", "NO": "NORDICS", "DK": "NORDICS", "FI": "NORDICS", "IS": "NORDICS",
    "FR": "FR", "DE": "DE", "GB": "GB",
    "AT": "AT", "NL": "NL", "PT": "PT",
}

# ── ALGORITHM TUNING ─────────────────────────────────────────────────────────
SECONDARY_RADIUS_KM   = 3.0   # Anchor → Hardware / Warehouse
TERTIARY_RADIUS_KM    = 5.0   # Anchor → Healthcare / Higher Ed / Airport
CALIBRATION_THRESHOLD = 0.10  # Tighten radius if Rank-1 > 10%
DEFAULT_CATCHMENT_KM  = 150.0  # Standard regional pull
DENSE_CATCHMENT_KM    = 27.0  # Urban corridor pull

GENERIC_WAREHOUSE = {
    # B2B cash-and-carry chains (professional buyers; not consumer membership format).
    # Costco / Makro / Sam's / BJ's promoted to ALPHA_WAREHOUSE (operator D5 2026-05-16).
    "NA": set(),
    "EU": {"selgros-pl", "selgros-de", "the-mart-gr", "metro-it", "metro-de"},
}

# ── PATHS FOR BUILD-TILES.PY ─────────────────────────────────────────────────
SERVICE_BUSINESS = TOTEBOX_DATA_PATH / "service-business"
SERVICE_PLACES   = TOTEBOX_DATA_PATH / "service-places"
TILES_DIR = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles")
WWW_DIR   = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www")

def all_algorithm_chain_ids() -> set:
    ids = set()
    for roles in REGION_CONFIG.values():
        for lst in roles.values():
            ids.update(lst)
    return ids


# ── SUB-ENTITY DEDUPLICATION ──────────────────────────────────────────────────
# Maps sub-entity chain_id → canonical parent. Sub-entities are never in
# REGION_CONFIG and never scored. They appear only on info cards.
CHAIN_FAMILIES: dict = {
    "walmart-pharmacy-us": "walmart-us",
    "walmart-gas-us": "walmart-us",
    "walmart-tire-center-us": "walmart-us",
    "walmart-vision-center-us": "walmart-us",
    "walmart-pharmacy-ca": "walmart-ca",
    "walmart-gas-ca": "walmart-ca",
    "walmart-pharmacy-mx": "walmart-mx",
    "walmart-gas-mx": "walmart-mx",
    "costco-tire-us": "costco-us",
    "costco-gas-us": "costco-us",
    "costco-pharmacy-us": "costco-us",
    "costco-tire-ca": "costco-ca",
    "costco-gas-ca": "costco-ca",
    "costco-tire-es": "costco-es",
    "costco-gas-es": "costco-es",
    "home-depot-pro-us": "home-depot-us",
    "home-depot-garden-us": "home-depot-us",
    "home-depot-pro-ca": "home-depot-ca",
    "home-depot-pro-mx": "home-depot-mx",
    "ikea-planning-studio-us": "ikea-us",
    "ikea-restaurant-us": "ikea-us",
    "ikea-planning-studio-es": "ikea-es",
    "ikea-restaurant-es": "ikea-es",
    "ikea-planning-studio-it": "ikea-it",
    "ikea-planning-studio-pl": "ikea-pl",
    "fred-meyer-fuel-us": "fred-meyer-us",
    "fred-meyer-pharmacy-us": "fred-meyer-us",
    "leroy-merlin-pro-es": "leroy-merlin-es",
    "leroy-merlin-pro-it": "leroy-merlin-it",
    "leroy-merlin-pro-pl": "leroy-merlin-pl",
    "makro-petrol-es": "makro-es",
    "makro-petrol-pl": "makro-pl",
}

CHAIN_SUB_LABELS: dict = {
    "walmart-us": {
        "walmart-pharmacy-us": "Pharmacy",
        "walmart-gas-us": "Gas Station",
        "walmart-tire-center-us": "Tire & Lube",
        "walmart-vision-center-us": "Vision Center",
    },
    "walmart-ca": {"walmart-pharmacy-ca": "Pharmacy", "walmart-gas-ca": "Gas Station"},
    "walmart-mx": {"walmart-pharmacy-mx": "Farmacia", "walmart-gas-mx": "Gasolinera"},
    "costco-us": {
        "costco-tire-us": "Tire Center",
        "costco-gas-us": "Gas Station",
        "costco-pharmacy-us": "Pharmacy",
    },
    "costco-ca": {"costco-tire-ca": "Tire Centre", "costco-gas-ca": "Gas Station"},
    "costco-es": {"costco-tire-es": "Centro neumáticos", "costco-gas-es": "Gasolinera"},
    "home-depot-us": {"home-depot-pro-us": "Pro Desk", "home-depot-garden-us": "Garden Center"},
    "ikea-us": {"ikea-planning-studio-us": "Planning Studio", "ikea-restaurant-us": "Restaurant"},
    "ikea-es": {"ikea-planning-studio-es": "Estudio planificación", "ikea-restaurant-es": "Restaurante"},
    "ikea-it": {"ikea-planning-studio-it": "Studio pianificazione"},
    "ikea-pl": {"ikea-planning-studio-pl": "Studio planowania"},
    "leroy-merlin-es": {"leroy-merlin-pro-es": "Pro (B2B)"},
    "leroy-merlin-it": {"leroy-merlin-pro-it": "Pro (B2B)"},
    "leroy-merlin-pl": {"leroy-merlin-pro-pl": "Pro (B2B)"},
}

ANCHOR_DISPLAY_NAMES: dict = {
    # Hypermarket anchors
    "walmart-us": "Walmart", "walmart-ca": "Walmart", "walmart-mx": "Walmart",
    "bodega-aurrera-mx": "Bodega Aurrera",
    "soriana-mx": "Soriana",
    "mercadona-es": "Mercadona",
    "tesco-uk": "Tesco", "sainsburys-uk": "Sainsbury's",
    "asda-uk": "ASDA", "morrisons-uk": "Morrisons",
    "fred-meyer-us": "Fred Meyer",
    "whole-foods-us": "Whole Foods Market",
    "heb-us": "H-E-B",
    "wegmans-us": "Wegmans",
    "winco-foods-us": "WinCo Foods",
    "sprouts-us": "Sprouts Farmers Market",
    "chedraui-mx": "Chedraui",
    "target-us": "Target",
    "real-canadian-superstore-ca": "Real Canadian Superstore",
    "carrefour-hypermarket-es": "Carrefour", "carrefour-hypermarket-it": "Carrefour",
    "carrefour-hypermarket-pl": "Carrefour", "carrefour-hypermarket-fr": "Carrefour",
    "alcampo-es": "Alcampo", "leclerc-es": "E.Leclerc", "leclerc-pl": "E.Leclerc",
    "auchan-pl": "Auchan", "auchan-fr": "Auchan",
    "leclerc-fr": "E.Leclerc",
    "ecenter-de": "E center", "marktkauf-de": "Marktkauf", "kaufland-de": "Kaufland",
    "esselunga-it": "Esselunga",
    "sklavenitis-gr": "Sklavenitis",
    "billa-plus-at": "Billa Plus",
    "continente-pt": "Continente",
    "albert-heijn-xl-nl": "Albert Heijn XL",
    "ipercoop-it": "Ipercoop", "iper-it": "Iper", "bennet-it": "Bennet",
    "bilka-dk": "Bilka", "obs-coop-no": "OBS Coop",
    "k-citymarket-fi": "K-Citymarket", "hagkaup-is": "Hagkaup",
    "prisma-fi": "Prisma", "coop-forum-se": "Coop Forum", "maxi-ica-se": "Maxi ICA",
    # IKEA
    "ikea-us": "IKEA", "ikea-ca": "IKEA", "ikea-mx": "IKEA",
    "ikea-es": "IKEA", "ikea-it": "IKEA", "ikea-gr": "IKEA",
    "ikea-pl": "IKEA",
    "ikea-se": "IKEA", "ikea-dk": "IKEA", "ikea-no": "IKEA", "ikea-fi": "IKEA",
    "ikea-fr": "IKEA", "ikea-de": "IKEA", "ikea-uk": "IKEA",
    "ikea-at": "IKEA", "ikea-nl": "IKEA", "ikea-pt": "IKEA",
    # Hardware / Home improvement anchors
    "home-depot-us": "The Home Depot", "home-depot-ca": "The Home Depot",
    "home-depot-mx": "The Home Depot",
    "lowes-us": "Lowe's", "lowes-ca": "Lowe's",
    "leroy-merlin-es": "Leroy Merlin", "leroy-merlin-it": "Leroy Merlin",
    "leroy-merlin-gr": "Leroy Merlin", "leroy-merlin-pl": "Leroy Merlin",
    "leroy-merlin-pt": "Leroy Merlin",
    "canadian-tire-ca": "Canadian Tire", "peavey-mart-ca": "Peavey Mart",
    "clas-ohlson-se": "Clas Ohlson",
    "imerco-dk": "Imerco", "k-rauta-fi": "K-Rauta", "obs-bygg-no": "OBS Bygg",
    "gamma-nl": "Gamma", "karwei-nl": "Karwei",
    "castorama-fr": "Castorama", "husasmidjan-is": "Húsasmiðjan",
    "brico-depot-es": "Brico Dépôt",
    "toom-baumarkt-de": "toom Baumarkt", "hagebaumarkt-de": "Hagebaumarkt",
    "bricocenter-it": "Bricocenter", "silvan-dk": "Silvan",
    "praktiker-gr": "Praktiker", "byko-is": "Byko",
    "hornbach-de": "Hornbach", "hornbach-at": "Hornbach",
    "praxis-nl": "Praxis",
    # Warehouse / Cash & Carry anchors
    "costco-us": "Costco", "costco-ca": "Costco", "costco-mx": "Costco",
    "costco-es": "Costco", "costco-se": "Costco", "costco-is": "Costco",
    "costco-uk": "Costco", "costco-fr": "Costco",
    "sams-club-us": "Sam's Club", "sams-club-mx": "Sam's Club",
    "bjs-wholesale-us": "BJ's Wholesale Club",
    "makro-es": "Makro", "makro-nl": "Makro", "makro-pl": "Makro",
    "metro-it": "Metro", "metro-de": "Metro",
    "selgros-de": "Selgros", "selgros-pl": "Selgros",
    "the-mart-gr": "The Mart",
}
