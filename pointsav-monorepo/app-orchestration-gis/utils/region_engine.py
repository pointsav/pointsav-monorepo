"""
utils/region_engine.py — Offline reverse geocoding via pre-downloaded boundary GeoJSONs.

Uses Shapely STRtree point-in-polygon (O(log N)) to assign human-readable
metropolitan/regional names to lat/lon coordinates. All lookups are offline
— no runtime API calls.

Boundary files (in BOUNDARIES_DIR):
    us_cbsa.geojson       — US Census TIGER GENZ2023 CBSA/MSA boundaries
    ca_cma.geojson        — Statistics Canada 2021 Census Metropolitan Areas
    mx_metro.geojson      — INEGI 2018 Zonas Metropolitanas
    eu_nuts3.geojson      — Eurostat GISCO NUTS-3 2021 level-3 (1:3M)
    fallback_ne_admin1.geojson — Natural Earth Admin-1 10m (global fallback)

Usage:
    from utils.region_engine import RegionEngine
    engine = RegionEngine(BOUNDARIES_DIR)
    name = engine.resolve(lat=43.6532, lon=-79.3832, iso="CA")
    # → "Toronto CMA"
"""

import json
import re
from pathlib import Path
from typing import Optional

try:
    from shapely.geometry import Point, shape
    from shapely.strtree import STRtree
    _SHAPELY_AVAILABLE = True
except ImportError:
    _SHAPELY_AVAILABLE = False

# EU country codes that route to NUTS-3
_EU_ISO = {
    "AT", "BE", "BG", "CY", "CZ", "DE", "DK", "EE", "ES", "FI",
    "FR", "GR", "HR", "HU", "IE", "IT", "LT", "LU", "LV", "MT",
    "NL", "PL", "PT", "RO", "SE", "SI", "SK", "GB", "NO", "IS",
    "CH", "AL", "BA", "ME", "MK", "RS", "TR",
}

# Mexican metro name overrides (INEGI names → Spanish display)
_MX_DISPLAY = {
    "Zona Metropolitana del Valle de México": "Ciudad de México",
    "Zona Metropolitana de Guadalajara": "Guadalajara",
    "Zona Metropolitana de Monterrey": "Monterrey",
    "Zona Metropolitana del Valle de Toluca": "Toluca",
    "Zona Metropolitana de Puebla-Tlaxcala": "Puebla",
    "Zona Metropolitana de Tijuana": "Tijuana",
    "Zona Metropolitana de La Laguna": "La Laguna",
    "Zona Metropolitana de León": "León",
    "Zona Metropolitana de Juárez": "Juárez",
    "Zona Metropolitana de San Luis Potosí-Soledad": "San Luis Potosí",
    "Zona Metropolitana de Querétaro": "Querétaro",
    "Zona Metropolitana de Mérida": "Mérida",
    "Zona Metropolitana de Cancún": "Cancún",
    "Zona Metropolitana de Veracruz": "Veracruz",
    "Zona Metropolitana de Aguascalientes": "Aguascalientes",
}

# Explicit overrides: raw NUTS-3 / NE Admin-1 name → clean display name.
# Covers non-English scripts, local-language names, and jargon suffixes.
_REGION_CLEAN: dict[str, str] = {
    # Greece — local script → transliterated English
    "Ανατολική Αττική":         "East Attica",
    "Δυτικός Τομέας Αθηνών":    "West Athens",
    "Ιωάννινα":                 "Ioannina",
    "Λακωνία, Μεσσηνία":        "Laconia — Messenia",
    "Πειραιάς, Νήσοι":          "Piraeus",
    "Κεντρικός Τομέας Αθηνών":  "Central Athens",
    "Βόρειος Τομέας Αθηνών":    "North Athens",
    "Νότιος Τομέας Αθηνών":     "South Athens",
    "Θεσσαλονίκη":              "Thessaloniki",
    "Ηράκλειο":                 "Heraklion",
    "Δωδεκάνησος":              "Dodecanese",
    "Κυκλάδες":                 "Cyclades",
    "Αχαΐα":                    "Achaea",
    "Ηλεία":                    "Elis",
    "Αρκαδία":                  "Arcadia",
    "Κορινθία":                 "Corinthia",
    "Αργολίδα":                 "Argolis",
    "Εύβοια":                   "Euboea",
    "Βοιωτία":                  "Boeotia",
    "Φωκίδα":                   "Phocis",
    "Φθιώτιδα":                 "Phthiotis",
    "Μαγνησία":                 "Magnesia",
    "Λάρισα":                   "Larissa",
    "Κοζάνη":                   "Kozani",
    "Γρεβενά":                  "Grevena",
    "Κιλκίς":                   "Kilkis",
    "Πέλλα":                    "Pella",
    "Ημαθία":                   "Imathia",
    "Πιερία":                   "Pieria",
    "Χαλκιδική":                "Chalkidiki",
    "Σέρρες":                   "Serres",
    "Δράμα":                    "Drama",
    "Καβάλα":                   "Kavala",
    "Ξάνθη":                    "Xanthi",
    "Ροδόπη":                   "Rhodope",
    "Έβρος":                    "Evros",
    # Iceland
    "Höfuðborgarsvæði":         "Reykjavík",
    # Denmark — local → English
    "Byen København":           "Copenhagen",
    "Københavns omegn":         "Copenhagen Suburbs",
    "Nordsjælland":             "North Zealand",
    "Østsjælland":              "East Zealand",
    "Vest- og Sydsjælland":     "West and South Zealand",
    "Østjylland":               "East Jutland",
    "Vestjylland":              "West Jutland",
    "Sydjylland":               "South Jutland",
    "Midtjylland":              "Central Jutland",
    "Nordjylland":              "North Jutland",
    "Syddanmark":               "South Denmark",
    "Fyn":                      "Funen",
    "Hovedstaden":              "Copenhagen Region",
    "Sjælland":                 "Zealand",
    # Norway — local → English
    "Sør-Trøndelag":            "South Trøndelag",
    "Møre og Romsdal":          "Møre og Romsdal",
    "Vestfold og Telemark":     "Vestfold and Telemark",
    "Vest-Agder":               "West Agder",
    # Finland — local → English
    "Helsinki-Uusimaa":         "Helsinki",
    "Pirkanmaa":                "Tampere Region",
    "Varsinais-Suomi":          "Southwest Finland",
    "Finland Proper":           "Southwest Finland",
    "Lappi":                    "Lapland",
    "Etelä-Karjala":            "South Karelia",
    "Etelä-Pohjanmaa":          "South Ostrobothnia",
    "Etelä-Savo":               "South Savo",
    "Kanta-Häme":               "Tavastia Proper",
    "Keski-Pohjanmaa":          "Central Ostrobothnia",
    "Keski-Suomi":              "Central Finland",
    "Kymenlaakso":              "Kymenlaakso",
    "Pohjois-Karjala":          "North Karelia",
    "Pohjois-Pohjanmaa":        "North Ostrobothnia",
    "Pohjois-Savo":             "North Savo",
    "Pohjanmaa":                "Ostrobothnia",
    "Satakunta":                "Satakunta",
    "Päijät-Häme":              "Lahti Region",
    "Uusimaa":                  "Uusimaa",
    "Northern Ostrobothnia":    "North Ostrobothnia",
    "Northern Savonia":         "North Savo",
    # Netherlands — local → English
    "Groot-Amsterdam":          "Greater Amsterdam",
    "Groot-Rijnmond":           "Rotterdam",
    "Agglomeratie Haarlem":     "Haarlem",
    "Agglomeratie Leiden en Bollenstreek": "Leiden",
    "Agglomeratie 's-Gravenhage": "The Hague",
    "Delft en Westland":        "Delft",
    "West-Noord-Brabant":       "West North Brabant",
    "Zuidoost-Noord-Brabant":   "Southeast North Brabant",
    "Noord-Friesland":          "North Frisia",
    "Overig Groningen":         "Groningen",
    "Twente":                   "Twente",
    "Arnhem/Nijmegen":          "Arnhem — Nijmegen",
    # Belgium
    "Arr. de Bruxelles-Capitale/Arr. Brussel-Hoofdstad": "Brussels",
    "Prov. Antwerpen":          "Antwerp",
    "Prov. Oost-Vlaanderen":    "East Flanders",
    "Prov. West-Vlaanderen":    "West Flanders",
    "Prov. Vlaams-Brabant":     "Flemish Brabant",
    "Prov. Hainaut":            "Hainaut",
    "Prov. Liège":              "Liège",
    "Prov. Namur":              "Namur",
    "Prov. Luxembourg (B)":     "Luxembourg Province",
    # Sweden — local → English for less-obvious ones
    "Dalarnas län":             "Dalarna",
    "Gävleborgs län":           "Gävleborg",
    "Kalmar län":               "Kalmar",
    "Jönköping":                "Jönköping",
    "Orebro":                   "Örebro",
    "Skåne":                    "Scania",
    "Skåne län":                "Scania",
    "Värmlands län":            "Värmland",
    "Västerbotten":             "Västerbotten",
    "Västernorrland":           "Västernorrland",
    "Västmanland":              "Västmanland",
    "Västra Götaland":          "West Gothenburg Region",
    "Västra Götalands län":     "West Gothenburg Region",
    "Östergötland":             "Östergötland",
    "Östergötlands län":        "Östergötland",
    # Czech Republic
    "Hlavní město Praha":       "Prague",
    "Jihomoravský kraj":        "South Moravia",
    "Moravskoslezský kraj":     "Moravian-Silesian Region",
    "Olomoucký kraj":           "Olomouc Region",
    "Kraj Vysočina":            "Vysočina Region",
    "Královéhradecký kraj":     "Hradec Králové Region",
    # Austria
    "Wiener Umland/Südteil":    "Vienna Suburbs South",
    "Niederösterreich-Süd":     "Lower Austria South",
    "Linz-Wels":                "Linz",
    "Klagenfurt-Villach":       "Klagenfurt",
    "Innsbruck":                "Innsbruck",
    "Klagenfurt-Villach":       "Klagenfurt",
    # Switzerland
    "St. Gallen":               "St. Gallen",
    # Poland — transliterated names are fine; fix encoding issues
    "Bydgosko-toruński":        "Bydgoszcz — Toruń",
    "Trójmiejski":              "Tricity (Gdańsk)",
    "Wrocławski":               "Wrocław",
    "Miasto Kraków":            "Kraków",
    "Miasto Poznań":            "Poznań",
    "Miasto Szczecin":          "Szczecin",
    "Miasto Warszawa":          "Warsaw",
    "Miasto Łódź":              "Łódź",
    "Lubelski":                 "Lublin",
    "Radomski":                 "Radom",
    "Rzeszowski":               "Rzeszów",
    "Gorzowski":                "Gorzów Wielkopolski",
    "Zielonogórski":            "Zielona Góra",
    "Katowicki":                "Katowice",
    "Warszawski zachodni":      "Warsaw West",
    "Olsztyński":               "Olsztyn",
    "Białostocki":              "Białystok",
    # Slovakia
    "Bratislavský kraj":        "Bratislava",
    "Žilinský kraj":            "Žilina",
    # Croatia
    "Primorsko-goranska županija": "Kvarner Bay",
    # Portugal
    "Área Metropolitana de Lisboa": "Lisbon",
    "Área Metropolitana do Porto":  "Porto",
    "Região de Aveiro":         "Aveiro",
    # UK — NUTS names that may be confusing
    "Aberdeen City and Aberdeenshire": "Aberdeen",
    "Angus and Dundee City":    "Dundee",
    "East Lothian and Midlothian": "Edinburgh Suburbs",
    "Bexley and Greenwich":     "Southeast London",
    "Cardiff and Vale of Glamorgan": "Cardiff",
    "Aberdeen City and Aberdeenshire": "Aberdeen",
    "South Nottinghamshire":    "Nottingham",
    "Greater Manchester South East": "Manchester",
    "West Essex":               "West Essex",
    "Devon CC":                 "Devon",
}

# German admin-tier suffixes to strip from NUTS-3 names
_DE_SUFFIXES = (
    ", Kreisfreie Stadt",
    ", Landkreis",
    ", Stadtkreis",
    ", Stadtbezirk",
    ", Kreisfreie Städte",
)


def _clean_region_name(name: str) -> str:
    """Apply override map and suffix cleanup to produce a display-ready region name."""
    if not name:
        return name
    # Explicit override takes priority
    if name in _REGION_CLEAN:
        return _REGION_CLEAN[name]
    # Strip German administrative suffixes
    for suffix in _DE_SUFFIXES:
        if name.endswith(suffix):
            return name[: -len(suffix)]
    # Clean double hyphens in US CBSA names (Census uses -- for multi-city)
    if "--" in name:
        # Keep first primary city, drop the rest before "Metro Area"
        primary = name.split("--")[0].strip()
        if "Metro Area" in name:
            primary = primary.split("-")[0].strip()  # first city only
            return f"{primary} Metro Area"
    return name


def _load_geojson(path: Path) -> list[tuple]:
    """Load a GeoJSON file and return list of (shapely_geom, properties_dict)."""
    if not path.exists():
        return []
    try:
        with open(path) as f:
            fc = json.load(f)
        result = []
        for feat in fc.get("features", []):
            try:
                geom = shape(feat["geometry"])
                result.append((geom, feat.get("properties", {})))
            except Exception:
                continue
        return result
    except Exception:
        return []


def _strtree_lookup(point: "Point", features: list[tuple]) -> Optional[dict]:
    """Return properties of the first polygon containing point, or None."""
    if not features:
        return None
    geoms = [f[0] for f in features]
    props = [f[1] for f in features]
    tree = STRtree(geoms)
    hits = tree.query(point, predicate="within")
    if len(hits) == 0:
        return None
    return props[hits[0]]


def _format_us_cbsa(props: dict) -> str:
    name = props.get("NAME") or props.get("NAMELSAD") or ""
    # Strip state suffixes like ", IL-IN-WI" from CBSA name
    if "," in name:
        name = name.split(",")[0].strip()
    if name and "Metro" not in name and "Micro" not in name:
        name = name + " Metro Area"
    return _clean_region_name(name) if name else "Metro Area"


def _format_ca_cma(props: dict) -> str:
    name = props.get("CMANAME") or props.get("CMANAMEE") or ""
    return name or "Metro Area"


def _format_mx_metro(props: dict) -> str:
    raw = props.get("NOM_ZM") or props.get("NOMZM") or props.get("nom_zm") or ""
    return _MX_DISPLAY.get(raw, raw) or "Zona Metropolitana"


def _format_eu_nuts3(props: dict) -> str:
    name = props.get("NUTS_NAME") or props.get("NAME_LATN") or "Region"
    return _clean_region_name(name)


def _format_ne_admin1(props: dict) -> str:
    name = props.get("name") or props.get("NAME") or ""
    return _clean_region_name(name) if name else "Region"


# GADM CamelCase splitter: "StrathconaCounty" → "Strathcona County"
_CAMELCASE_RE = re.compile(r"(?<=[a-z])(?=[A-Z])")
# Spanish prepositions glued to preceding word in GADM ES data (Acapulcode Juárez,
# Bocadel Río). Capturing the preposition consumes the match — avoids overlapping
# substitutions that would split "del" → "d el". Order matters: longest first.
_GADM_SPANISH_PREPS_RE = re.compile(
    r"(?<=[a-záéíóúñü])(del|las|los|de|la|el)(?=\s)",
    flags=re.IGNORECASE,
)
# Period without trailing space ("A.Madero" → "A. Madero")
_GADM_PERIOD_RE = re.compile(r"\.(?=[A-Za-zÁÉÍÓÚÑÜ])")


def _gadm_camelcase_split(name: str) -> str:
    if not name:
        return name
    s = _CAMELCASE_RE.sub(" ", name)
    s = _GADM_SPANISH_PREPS_RE.sub(r" \1", s)
    s = _GADM_PERIOD_RE.sub(". ", s)
    return s.strip()


def _format_ca_csd(props: dict) -> str:
    """GADM CA admin-3 → human-readable Census Subdivision proxy.
    NAME_3 carries municipality names like 'Edmonton' or 'StrathconaCounty'."""
    name = props.get("NAME_3") or ""
    return _gadm_camelcase_split(name) if name else ""


def _format_mx_municipio(props: dict) -> str:
    """GADM MX admin-2 → municipio name (Cuauhtémoc, Coyoacán, etc.)."""
    name = props.get("NAME_2") or ""
    return _gadm_camelcase_split(name) if name else ""


class RegionEngine:
    """
    Offline point-in-polygon reverse geocoding.
    Instantiate once; call resolve() for each coordinate.
    """

    def __init__(self, boundaries_dir: Path) -> None:
        self._available = _SHAPELY_AVAILABLE
        if not self._available:
            print("WARNING: shapely not installed — RegionEngine will return None for all lookups.")
            return

        d = Path(boundaries_dir)
        self._us_feats = _load_geojson(d / "us_cbsa.geojson")
        self._ca_feats = _load_geojson(d / "ca_cma.geojson")
        self._mx_feats = _load_geojson(d / "mx_metro.geojson")
        self._eu_feats = _load_geojson(d / "eu_nuts3.geojson")
        self._fb_feats = _load_geojson(d / "fallback_ne_admin1.geojson")
        # Sprint 10 — admin-3 CSD (CA) + admin-2 Municipio (MX) via GADM 4.1
        self._ca_csd_feats = _load_geojson(d / "ca_csd.geojson")
        self._mx_mun_feats = _load_geojson(d / "mx_municipio.geojson")

        # Build STRtrees upfront for repeated O(log N) queries
        if self._us_feats:
            self._us_tree = STRtree([f[0] for f in self._us_feats])
        if self._ca_feats:
            self._ca_tree = STRtree([f[0] for f in self._ca_feats])
        if self._mx_feats:
            self._mx_tree = STRtree([f[0] for f in self._mx_feats])
        if self._eu_feats:
            self._eu_tree = STRtree([f[0] for f in self._eu_feats])
        if self._fb_feats:
            self._fb_tree = STRtree([f[0] for f in self._fb_feats])
        if self._ca_csd_feats:
            self._ca_csd_tree = STRtree([f[0] for f in self._ca_csd_feats])
        if self._mx_mun_feats:
            self._mx_mun_tree = STRtree([f[0] for f in self._mx_mun_feats])

        counts = {
            "US": len(self._us_feats), "CA": len(self._ca_feats),
            "MX": len(self._mx_feats), "EU": len(self._eu_feats),
            "FB": len(self._fb_feats),
            "CA_CSD": len(self._ca_csd_feats), "MX_MUN": len(self._mx_mun_feats),
        }
        loaded = sum(counts.values())
        print(f"RegionEngine: {loaded} boundary polygons loaded "
              f"(US={counts['US']} CA={counts['CA']} MX={counts['MX']} "
              f"EU={counts['EU']} FB={counts['FB']} "
              f"CA_CSD={counts['CA_CSD']} MX_MUN={counts['MX_MUN']})")

    def _tree_query(self, tree: "STRtree", features: list[tuple], point: "Point") -> Optional[dict]:
        hits = tree.query(point, predicate="within")
        if len(hits) > 0:
            return features[hits[0]][1]
        # Coastal/fjord edge case: assign to nearest polygon if within ~15 km (0.15°)
        nearest = tree.nearest(point)
        if nearest is not None and features[nearest][0].distance(point) < 0.15:
            return features[nearest][1]
        return None

    # Composite country codes (multi-country bboxes) → route to EU NUTS-3
    _COMPOSITE_EU = {"Nordics"}

    def resolve(self, lat: float, lon: float, iso: str) -> Optional[str]:
        """
        Return a human-readable metropolitan region name for the given coordinate.
        Returns None if no boundary file is available or no polygon contains the point.
        """
        if not self._available:
            return None

        point = Point(lon, lat)  # Shapely uses (lon, lat)

        try:
            if iso == "US" and self._us_feats:
                props = self._tree_query(self._us_tree, self._us_feats, point)
                if props:
                    return _format_us_cbsa(props)

            elif iso == "CA":
                # CSD (admin-3) primary — distinguishes Edmonton from Strathcona County
                csd_name = ""
                if self._ca_csd_feats:
                    csd_props = self._tree_query(self._ca_csd_tree, self._ca_csd_feats, point)
                    if csd_props:
                        csd_name = _format_ca_csd(csd_props)
                # CMA (admin-2 metro) for outer context
                cma_name = ""
                if self._ca_feats:
                    cma_props = self._tree_query(self._ca_tree, self._ca_feats, point)
                    if cma_props:
                        cma_name = _format_ca_cma(cma_props)
                if csd_name and cma_name and cma_name not in csd_name and csd_name not in cma_name:
                    return f"{csd_name}, {cma_name}"
                if csd_name:
                    return csd_name
                if cma_name:
                    return cma_name

            elif iso == "MX":
                # Municipio (admin-2) primary — Cuauhtémoc, Coyoacán, etc.
                if self._mx_mun_feats:
                    mun_props = self._tree_query(self._mx_mun_tree, self._mx_mun_feats, point)
                    if mun_props:
                        mun_name = _format_mx_municipio(mun_props)
                        if mun_name:
                            return mun_name
                # Fallback: legacy INEGI ZM (mx_metro.geojson) if file present
                if self._mx_feats:
                    props = self._tree_query(self._mx_tree, self._mx_feats, point)
                    if props:
                        return _format_mx_metro(props)

            elif (iso in _EU_ISO or iso in self._COMPOSITE_EU) and self._eu_feats:
                props = self._tree_query(self._eu_tree, self._eu_feats, point)
                if props:
                    return _format_eu_nuts3(props)

            # Global fallback — Natural Earth Admin-1
            if self._fb_feats:
                props = self._tree_query(self._fb_tree, self._fb_feats, point)
                if props:
                    return _format_ne_admin1(props)

        except Exception:
            pass

        return None
