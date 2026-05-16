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
# Phase 5: carrefour-fr (re-ingest required — ~5,200 stores). Phase 6: auchan-fr, leclerc-fr, edeka-de.
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
    },
    "EU": {
        # Mercadona-ES: Spain's flagship hypermarket, 1,603 stores — operator D1 2026-05-16
        "mercadona-es",
        # Tesco-UK + Sainsbury's-UK: UK large-format hypermarkets — Phase 1 promotion 2026-05-16
        "tesco-uk", "sainsburys-uk",
        # Nordic large-format hypermarkets (migrated from ALPHA_ANCHORS EU 2026-05-16)
        "bilka-dk", "obs-coop-no", "hagkaup-is", "k-citymarket-fi", "prisma-fi",
        # Carrefour FR: PENDING re-ingest (Phase 5 — ~5,200 stores via name_query partial)
        # "carrefour-fr",
    }
}

# Destination furniture / home-goods (Lifestyle format).
ALPHA_LIFESTYLE = {
    "NA": {"ikea-us", "ikea-ca", "ikea-mx"},
    "EU": {
        "ikea-es", "ikea-it", "ikea-gr", "ikea-pl", "ikea-nordics",
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
    "NA": {"lowes-us", "lowes-ca", "canadian-tire-ca", "peavey-mart-ca"},
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
GENERIC_FOOD = {"lidl-es", "safeway-ca", "whole-foods-us", "biedronka-pl",
                "lidl-uk",
                "lidl-de", "lidl-fr", "lidl-nl", "lidl-at", "lidl-pt",
                "aldi-de", "aldi-uk", "aldi-nl", "aldi-pl"}
# Sprint 12 — soriana-mx promoted from Food to ALPHA_HYPERMARKET (operator decision A1).
# mercadona-es, tesco-uk, sainsburys-uk promoted to ALPHA_HYPERMARKET 2026-05-16 (operator D1/Phase 1).
# carrefour-fr: Phase 5 re-ingest pending — add to ALPHA_HYPERMARKET when ingested.
# Chedraui-mx, safeway-ca, whole-foods-us remain Food (operator decision, format mismatch).

# ── REGION CONFIGURATION ─────────────────────────────────────────────────────
REGION_CONFIG = {
    # Dual membership: Fortune-scale chains appear in BOTH anchor (can initiate clusters)
    # AND hardware/warehouse (are found as secondaries by other anchors, contributing scores).
    "US": {
        "anchor":    ["walmart-us", "target-us", "fred-meyer-us", "ikea-us", "home-depot-us", "costco-us"],
        "hardware":  ["home-depot-us", "alaska-industrial-hardware-us", "lowes-us", "menards-us"],
        "warehouse": ["costco-us", "sams-club-us", "bjs-wholesale-us"]
    },
    "CA": {
        "anchor":    ["walmart-ca", "ikea-ca", "real-canadian-superstore-ca", "home-depot-ca", "costco-ca"],
        "hardware":  ["home-depot-ca", "lowes-ca", "canadian-tire-ca"],
        "warehouse": ["costco-ca"]
    },
    "MX": {
        "anchor":    ["walmart-mx", "soriana-mx", "ikea-mx", "home-depot-mx", "costco-mx"],
        "hardware":  ["home-depot-mx"],
        "warehouse": ["costco-mx", "sams-club-mx"]
    },
    "ES": {
        "anchor":    ["mercadona-es", "ikea-es", "costco-es", "makro-es"],
        "hardware":  ["leroy-merlin-es", "brico-depot-es", "bauhaus-es"],
        "warehouse": ["costco-es", "makro-es"]
    },
    "IT": {
        "anchor":    ["ikea-it"],
        "hardware":  ["leroy-merlin-it", "obi-it", "bricocenter-it"],
        "warehouse": ["metro-it"]
    },
    "GR": {
        "anchor":    ["ikea-gr"],
        "hardware":  ["leroy-merlin-gr", "praktiker-gr"],
        "warehouse": ["the-mart-gr"]
    },
    "PL": {
        "anchor":    ["ikea-pl", "makro-pl"],
        "hardware":  ["leroy-merlin-pl", "obi-pl", "castorama-pl"],
        "warehouse": ["makro-pl", "selgros-pl"]
    },
    "NORDICS": {
        "anchor":    ["ikea-nordics", "bilka-dk", "prisma-fi", "k-citymarket-fi", "obs-coop-no", "hagkaup-is", "costco-se", "costco-is"],
        "hardware":  ["clas-ohlson-se", "k-rauta-fi", "imerco-dk", "obs-bygg-no", "husasmidjan-is", "bauhaus-se", "silvan-dk", "byko-is"],
        "warehouse": ["costco-se", "costco-is"]
    },
    # Phase B: France, Germany, UK (data ingested 2026-05-05)
    "FR": {
        "anchor":    ["ikea-fr", "costco-fr"],
        "hardware":  ["leroy-merlin-fr", "castorama-fr"],
        "warehouse": ["costco-fr"]
    },
    "DE": {
        "anchor":    ["ikea-de"],
        "hardware":  ["hornbach-de", "obi-de", "bauhaus-de", "toom-baumarkt-de", "hagebaumarkt-de"],
        "warehouse": ["metro-de", "selgros-de"]
    },
    "GB": {
        "anchor":    ["tesco-uk", "sainsburys-uk", "ikea-uk", "costco-uk"],
        "hardware":  ["bq-uk"],
        "warehouse": ["costco-uk"]
    },
    # Phase C: Austria, Netherlands, Portugal
    "AT": {
        "anchor":    ["ikea-at"],
        "hardware":  ["hornbach-at"],
        "warehouse": []
    },
    "NL": {
        "anchor":    ["ikea-nl", "makro-nl"],
        "hardware":  ["praxis-nl", "gamma-nl", "karwei-nl"],
        "warehouse": ["makro-nl"]
    },
    "PT": {
        "anchor":    ["ikea-pt"],
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
    "fred-meyer-us": "Fred Meyer",
    "target-us": "Target",
    "real-canadian-superstore-ca": "Real Canadian Superstore",
    "carrefour-hypermarket-es": "Carrefour", "carrefour-hypermarket-it": "Carrefour",
    "carrefour-hypermarket-pl": "Carrefour", "carrefour-hypermarket-fr": "Carrefour",
    "alcampo-es": "Alcampo", "leclerc-es": "E.Leclerc", "leclerc-pl": "E.Leclerc",
    "auchan-pl": "Auchan",
    "ipercoop-it": "Ipercoop", "iper-it": "Iper", "bennet-it": "Bennet",
    "bilka-dk": "Bilka", "obs-coop-no": "OBS Coop",
    "k-citymarket-fi": "K-Citymarket", "hagkaup-is": "Hagkaup",
    "prisma-fi": "Prisma", "coop-forum-se": "Coop Forum", "maxi-ica-se": "Maxi ICA",
    # IKEA
    "ikea-us": "IKEA", "ikea-ca": "IKEA", "ikea-mx": "IKEA",
    "ikea-es": "IKEA", "ikea-it": "IKEA", "ikea-gr": "IKEA",
    "ikea-pl": "IKEA", "ikea-nordics": "IKEA",
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
