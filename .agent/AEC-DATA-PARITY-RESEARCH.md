---
plan: AEC-DATA-PARITY-RESEARCH
created: 2026-05-23
author: Opus research agent + Jennifer Woodfine synthesis
status: COMPLETE — findings ready to incorporate into nightly build plan
companion: AEC-NIGHTLY-BUILD-PLAN.md, AEC-LAYERS-RESEARCH.md
---

# AEC Data Parity — Research Findings

Research question: what open-license data can close the gap between US regulatory-grade
AEC layers and the Köppen/AQUEDUCT proxies currently planned for all other countries?

**Key finding:** The gap is narrower than the existing plans assumed. EU flood and EU
climate-zone data are both regulatory-grade and mostly open. The CONABIO Mexico blocker
and Canada FHIMP absence are the substantive gaps.

---

## 1 — EU national building-code climate zones

**Pattern:** No pan-EU harmonised shapefile. Each member state defines zones keyed to
municipal (LAU2) boundaries. Pipeline: download national code zone lookup table → join
to GISCO LAU2 polygons → produce harmonised `iso/lau2_id/zone_code/source/vintage` layer.

| ISO | Code | Zones | Vector approach | License |
|---|---|---|---|---|
| DE | GEG 2023 / TRY 2017 | 15 TRY regions | BBSR TRY 2017 portal + DWD CDC 1 km² raster (account required) | DWD free + attribution |
| FR | RE2020 | 8 (H1a–H3) | Arrêté 4 août 2021 département→zone lookup + IGN/Eurostat dept boundaries | Etalab 2.0 |
| ES | CTE DB-HE | 12 (A3–E1) | Annex B municipal lookup (in PDF) + INE/GISCO municipios | Free, Ministerio de Vivienda attribution |
| IT | DPR 412/1993 | 6 (A–F) | ENEA Solaritaly GradiGiorni + ISTAT comuni shp | CC BY 4.0 (ENEA) + CC BY 3.0 IT (ISTAT) |
| GR | KENAK | 4 (A–D) | KENAK regulation text lookup + ELSTAT municipality boundaries | ELSTAT free |
| PT | SCE / REH-RECS | 3 winter + 3 summer | DGEG climate data + SNIG/DGT boundaries | Free, attribution |
| FI | SFS-EN ISO 15927-4 | 4 | Municipality lookup + Maanmittauslaitos (CC BY 4.0) | CC BY 4.0 |
| PL | WT 2021 / EN 12831 | 5 (I–V) | MDPI 2024 geospatial model (CC BY) | CC BY |
| SE | BBR | 4 | Boverket Klimatdatabas API (JSON/XML) + Lantmäteriet boundaries | Open data, CC0/CC BY |
| NL | NTA 8800 | Single reference | Not applicable — single-zone country | Open data |
| DK | BR18 | Single reference | Not applicable | Open data |
| NO | TEK17 | Single reference | Not applicable | Open data |
| GB | SAP / HEM | No zone polygon | No analogous zone system; use Met Office HadUK-Grid | OGL v3 |

**Eight EU ISOs can get regulatory-grade climate zones via build-by-join.** Four single-reference
countries (NL/DK/NO/GB) don't need a zone polygon.

Key URLs:
- FR zone lookup: https://www.ecologie.gouv.fr/sites/default/files/documents/La%20r%C3%A9partition%20des%20d%C3%A9partements%20par%20zone%20climatique.pdf
- ES CTE DB-HE Annex B: https://www.codigotecnico.org/pdf/Documentos/HE/DccHE.pdf
- IT ENEA GradiGiorni: http://solaritaly.enea.it/clisun/Pagine/GradiGiorni.htm
- ISTAT comuni: https://www.istat.it/notizia/basi-territoriali-e-variabili-censuarie/
- DGEG PT climate data: https://www.dgeg.gov.pt/pt/areas-transversais/investigacao-e-inovacao/publicacoes-relatorios-estudos/portuguese-climate-data-for-building-simulation/
- PL MDPI 2024: https://www.mdpi.com/1996-1073/17/16/3905
- Boverket SE: https://www.boverket.se/sv/om-boverket/oppna-data/boverkets-klimatdatabas/
- GISCO LAU2 (EU municipal boundaries): https://gisco-services.ec.europa.eu/distribution/v2/nuts/download/

---

## 2 — EU Floods Directive data (regulatory-grade, not AQUEDUCT proxy)

**Key insight:** EU Floods Directive 2007/60/EC requires all member states to publish flood
hazard maps. Regulatory-grade shapefiles exist for most tracked EU ISOs — this replaces
the AQUEDUCT-only proxy approach for EU.

**Pan-EU fallback:** JRC Global River Flood Hazard Maps v2.1 — 3 arc-second (~30-75m) GeoTIFFs
for 7 return periods (10/20/50/100/200/500 yr). Free, no restrictions.
- Earth Engine: https://developers.google.com/earth-engine/datasets/catalog/JRC_CEMS_GLOFAS_FloodHazard_v2_1
- JRC data: https://data.jrc.ec.europa.eu/collection/id-0054

**Per-country regulatory downloads:**

| ISO | Dataset | URL | License | Format | Notes |
|---|---|---|---|---|---|
| GB | EA Flood Map for Planning (Flood Zones 2 & 3) | https://environment.data.gov.uk/dataset/04532375-a198-476e-985e-0579a0a11b47 | OGL v3 | GeoPackage/GeoJSON/GDB | Direct download. Most comparable to FEMA NFHL. |
| FR | Géorisques Zonages Inondation 2020 (TRI) | https://www.georisques.gouv.fr/donnees/bases-de-donnees/zonages-inondation-rapportage-2020 | Etalab 2.0 | Shapefile (COVADIS DI) | Commercial OK |
| ES | SNCZI Zonas Inundables T10/T50/T100/T500 | https://www.miteco.gob.es/en/cartografia-y-sig/ide/descargas/agua/descargas_agua_snczi.html | Free, MITECO attribution | Shapefile | T100: 1.02 GB; T10: 838 MB. Large but free. |
| IT | IdroGEO PAI + Floods Directive maps | https://idrogeo.isprambiente.it/app/page/open-data | IODL 2.0 / CC BY | Open data download | River Basin Districts; per-district |
| DE | LAWA HWGK via Bundesland portals + BfG aggregator | https://www.bfg.de/ | Per-Bundesland, generally free | WMS/WFS per state | No single national bulk download; ingest via WFS |
| PT | Áreas de Risco de Inundação | https://sniamb.apambiente.pt/; https://snig.dgterritorio.gov.pt/ | Free, attribution | INSPIRE ATOM / shapefile | |
| PL / GR / NL / SE / DK / NO / FI | All publish Floods Directive PFRA + hazard/risk maps via INSPIRE | https://inspire-geoportal.ec.europa.eu/ — search "FloodRiskZones-dir-2007-60" + ISO | Per national INSPIRE policy; usually open | WMS/WFS → GeoPackage | Ingest via INSPIRE WFS into PostGIS |

**Approach:** Per-country regulatory shapefiles for GB/FR/ES/IT; DE via LAWA WFS;
smaller ISOs via INSPIRE WFS. JRC v2.1 GeoTIFF as global gap-filler for any
cluster not covered by the regulatory layer.

---

## 3 — Mexico AEC data gaps

**Critical blocker: CONABIO is CC BY-NC** — non-commercial restriction blocks use on
gis.woodfinegroup.com without written exception. Two options:
1. Request commercial-use exception from CONABIO in writing.
2. Use INEGI climate raster (free for commercial reuse) for NMX-C-460 zone join.

| Layer | Status | URL | License | Notes |
|---|---|---|---|---|
| Seismic (CENAPRED CFE-2015) | Available but intermittent | http://www.atlasnacionalderiesgos.gob.mx/ | Government open data | Contact: anr.administracion@cenapred.unam.mx for shapefile bundles |
| Seismotectonic zones (SGM) | Available | https://www.sgm.gob.mx/Sismotectonica/ | Free, SGM attribution | |
| Flood (Atlas Nacional) | Patchy; per-state atlases more reliable | http://www.atlasnacionalderiesgos.gob.mx/ | Government open data | |
| NMX-C-460 climate zones | No published vector — build from INEGI climate raster | https://en.www.inegi.org.mx/temas/climatologia/ | Free, commercial OK | CONABIO Köppen-García is CC BY-NC; use INEGI raster instead |
| CONABIO eco-regions 2008 | CC BY-NC — blocked for commercial | http://geoportal.conabio.gob.mx/metadatos/doc/html/ecort08gw.html | **CC BY-NC 2.5 MX** | Use Resolve 2017 CC BY instead |
| INEGI base layers | Good coverage | https://en.www.inegi.org.mx/temas/mg/ | Free, attribution | DEM, land use, admin boundaries |

---

## 4 — Eco-regions: global parity

**RESOLVE Ecoregions 2017 — confirmed:**
- Direct download: `https://storage.googleapis.com/teow2016/Ecoregions2017.zip` (150 MB)
- License: **CC BY 4.0** (commercial use OK with attribution)
- Scope: 846 ecoregions globally; 14 biomes × 8 realms
- Earth Engine asset: `RESOLVE/ECOREGIONS/2017`
- Viewer: https://ecoregions.appspot.com/

**Ecoregion counts per ISO (approximate):** US ~35; CA ~16; MX ~21; GB 3; FR ~6;
DE ~4; ES ~7; IT ~6; PL ~3; NL 1; PT ~3; SE/FI ~3 each; DK 1; NO ~3; GR ~3.

**Resolution note:** Resolve polygons are typically thousands of km² — each retail site
(50–200 ha) falls in one ecoregion. Appropriate for LEED SS and broad biome framing;
not site-scale planting design.

**Recommended dual-layer approach:**
- **Global baseline:** Resolve Ecoregions 2017 (CC BY 4.0) — all 16 ISOs
- **US precision:** EPA Level III (public domain, 85 zones) — override for US clusters
- **EU regulatory reference:** EEA Biogeographical Regions 2016 (11 EU regions per
  Habitats Directive 92/43/EEC) — the reference frame EU landscape architects use
  - Download: https://www.eea.europa.eu/data-and-maps/data/biogeographical-regions-europe-3/zipped-shapefile-format-vector-polygon
  - License: EEA standard reuse policy (free with attribution)
- **Optional EU complement:** EEA Environmental Zones 2018 (Metzger EnS, 13 zones/84 strata)
  - https://sdi.eea.europa.eu/catalogue/idp/api/records/6ef007ab-1fcd-4c4f-bc96-14e8afbcb688
- **MX precision:** Deferred pending CONABIO licensing decision; use Resolve as fallback

---

## 5 — Canada parity check

| Layer | URL | License | Status |
|---|---|---|---|
| NECB climate zones (HOT2000 Climate Map) | https://open.canada.ca/data/en/dataset/4672733b-bbb6-4299-a57f-f19ab475ac11 | OGL-Canada | ✓ ESRI MapServer extractable to GeoJSON |
| NECB MapServer (EN) | https://maps-cartes.services.geo.ca/server_serveur/rest/services/NRCan/Carte_climatique_HOT2000_Climate_Map_EN/MapServer | OGL-Canada | Direct REST extraction |
| NSHM 2015 seismic (point tool) | https://www.earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/index-en.php | OGL-Canada | Point query; raster input files at OSDP |
| NSHM 2015 input files | https://ostrnrcan-dostrncan.canada.ca/handle/1845/155327 | OGL-Canada | Regenerate via OpenQuake |
| Flood hazard (FHIMP) | https://natural-resources.canada.ca/.../flood-hazard-identification-mapping-program | OGL-Canada | **No national layer** — 2024–2028 program |
| Future Flood Susceptibility 2024 | https://open.canada.ca/data/en/dataset/c00f95a3-7bab-4d28-b9cc-b30f06b5afd2 | OGL-Canada | XGBoost-modelled national proxy |
| IDF curves (ECCC) | https://climate.weather.gc.ca/prods_servs/engineering_e.html | OGL-Canada | ~896 stations; Atlas 14 equivalent |

**Canada flood:** No FEMA-equivalent national layer exists yet. Use Future Flood
Susceptibility (2024) as modelled proxy + WRI AQUEDUCT as background.

---

## 6 — Global equivalents for US-only regulatory layers

### Wetlands

| Source | URL | License | Notes |
|---|---|---|---|
| **GWL_FCS30 (30m global, 2020)** | https://zenodo.org/records/7340516 | **CC BY 4.0** | 8 wetland classes; 30m global; best open NWI equivalent |
| GWL_FCS30D (annual 2000–2022) | per Nature Scientific Data 2024 | CC BY 4.0 | Change detection version |
| Ramsar Sites | https://rsis.ramsar.org/ | Free, attribution | Protected sites only (~2,500 global) |

**Recommendation:** GWL_FCS30 globally + EPA NWI (higher res) for US.

### Historic preservation

| Source | URL | License | Notes |
|---|---|---|---|
| **UNESCO World Heritage Sites + buffers** | https://data.europa.eu/data/datasets/world-heritage-sites-and-buffer-zones | CC BY-SA 3.0 IGO | ~1,200 sites; buffer zones included |
| UK Historic England NHLE | https://historicengland.org.uk/listing/the-list/ | OGL | ~400K records; best per-country dataset |
| FR Mérimée database | data.gouv.fr | Etalab 2.0 | Points + polygons |
| IT VIR — Vincoli in Rete (MiC) | http://vincoliinrete.beniculturali.it/ | Free | Building heritage constraints |

**No INSPIRE Cultural Heritage harmonised pan-EU shapefile exists in practice.**

### Wildfire

| Source | URL | License | Notes |
|---|---|---|---|
| **JRC EFFIS wildfire risk** | https://forest-fire.emergency.copernicus.eu/applications/data-and-services | **Requires formal data request** | Best EU layer; not direct download |
| GWIS FWI (global raster) | https://gwis.jrc.ec.europa.eu/ | Free, attribution | Modelled; use as fallback |
| CWFIS (Canada) | NRCan | OGL-Canada | Fire-weather index; not WHP-equivalent |
| CONAFOR SNIF (Mexico) | https://snigf.cnf.gob.mx/ | MX gov open data | Fire history; not risk-zone polygon |

**Action required:** Submit EFFIS data request before Night 5. Without it, use GWIS FWI raster.
URL: https://forest-fire.emergency.copernicus.eu/applications/data-and-services

### Environmental Justice / Social Vulnerability

| Source | URL | License | Notes |
|---|---|---|---|
| GISCO NUTS-3 polygons | https://gisco-services.ec.europa.eu/distribution/v2/nuts/download/ | Free, attribution | Geometry for EU vulnerability layer |
| Eurostat regional indicators | Eurostat API | Free, attribution | Join to NUTS-3; no pre-built EJSCREEN equivalent |
| UK IMD 2019 (LSOA) | https://www.gov.uk/government/statistics/english-indices-of-deprivation-2019 | OGL | Best EU per-country EJ proxy |

**No EU EJSCREEN-equivalent index exists.** Build by joining Eurostat indicators to NUTS-3.
Structurally coarser than US EJSCREEN/CDC SVI block-group data — unavoidable.

---

## 7 — Revised parity scorecard (all 16 ISOs)

✓ = regulatory-grade open vector/raster | ~ = proxy or partial | ✗ = no open data

| ISO | Climate zone | Flood | Seismic | Solar | Eco-region | Historic | Wetlands | Wildfire | Weather |
|---|---|---|---|---|---|---|---|---|---|
| US | ✓ ASHRAE 169 | ✓ FEMA NFHL | ✓ USGS | ✓ NSRDB | ✓ EPA L3 | ✓ NRHP | ✓ NWI | ✓ USFS WHP | ✓ Atlas 14 |
| CA | ✓ NECB HOT2000 | ~ Future Susceptibility 2024 | ✓ NRCan 2015 | ✓ NSRDB | ~ Resolve | ~ Parks Canada | ~ Ramsar+GWL | ~ CWFIS | ✓ ECCC IDF |
| MX | ~ NMX via INEGI | ~ CENAPRED Atlas | ~ CENAPRED/SGM (intermittent) | ✓ NSRDB | ~ Resolve | ✗ | ~ Ramsar+GWL | ~ CONAFOR | ~ INEGI raster |
| GB | ✗ no zone polygon | ✓ EA Flood Map (OGL) | ~ ESHM20 (low) | ✓ PVGIS | ~ Resolve/EEA | ✓ Historic England | ~ Ramsar+GWL | ~ GWIS FWI | ✓ HadUK |
| FR | ✓ RE2020 dept join | ✓ Géorisques TRI | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✓ Mérimée | ~ INPN+Ramsar | ~ EFFIS (request) | ✓ Météo France |
| DE | ~ TRY 2017 raster | ✓ LAWA via Bundesland WFS | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✗ per-Bundesland | ~ Ramsar+GWL | ~ GWIS FWI | ✓ DWD CDC |
| ES | ✓ CTE DB-HE join | ✓ SNCZI T10/T100 (free) | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ BIC per-Comunidad | ~ Ramsar+GWL | ✓ EFFIS (request) | ✓ AEMET |
| IT | ✓ DPR 412 join | ✓ IdroGEO PAI | ✓ ESHM20+INGV | ✓ PVGIS | ~ Resolve/EEA | ✓ VIR MiC | ~ Ramsar+GWL | ✓ EFFIS (request) | ✓ ENEA/Aeronautica |
| PL | ~ WT 2021 MDPI join | ~ KZGW INSPIRE | ✓ ESHM20 (low) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ GWIS FWI | ✓ IMGW |
| NL | n/a single climate | ~ Risicokaart | ✓ ESHM20 (induced) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ GWIS FWI | ✓ KNMI |
| PT | ✓ SCE municipality join | ✓ SNIAmb INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ✓ EFFIS (request) | ✓ IPMA |
| SE | ~ BBR 4 zones (API) | ~ MSB INSPIRE | ✓ ESHM20 (low) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ very low | ✓ SMHI |
| DK | n/a single climate | ~ INSPIRE WFS | ✓ ESHM20 (low) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ very low | ✓ DMI |
| NO | n/a single climate | ~ NVE INSPIRE | ✓ ESHM20 (W. NO) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ very low | ✓ Met Norway |
| FI | ~ SFS 4 zone join | ~ SYKE INSPIRE | ✓ ESHM20 (very low) | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ~ very low | ✓ FMI |
| GR | ~ KENAK 4 zone join | ~ YPEN INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✗ | ~ Ramsar+GWL | ✓ EFFIS (request) | ✓ HNMS |

---

## Caveats for DATA-MANIFEST / methodology dialog

1. **EU climate-zone polygons are derived layers** (national code lookup + LAU2 polygons).
   Disclose provenance in DATA-MANIFEST. Not native national publications.
2. **JRC Global River Flood Hazard is modelled** — label as "Modelled (JRC LISFLOOD-FP)
   — for comparative use only, not authoritative" wherever displayed.
3. **CONABIO CC BY-NC** blocks Mexico precision eco-regions/climate without written
   exception. Using INEGI raster + Resolve as substitutes.
4. **EFFIS wildfire requires formal data request** — submit before Night 5 build.
   URL: https://forest-fire.emergency.copernicus.eu/applications/data-and-services
5. **Canada has no national flood hazard layer** (FHIMP 2024–2028). Using Future
   Flood Susceptibility (XGBoost, 2024) as documented modelled proxy.
6. **No EU EJSCREEN equivalent** — NUTS-3 Eurostat layer is the structural maximum;
   unavoidably coarser than US CDC SVI/EJSCREEN.
7. **GB has no building-code climate zone polygon** — use HadUK-Grid or skip zone layer.
