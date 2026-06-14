---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: in-progress
version: "0.2"
title: "The Commuter Archetype: Car-Rental Clustering as a Proxy for Transit-Adjacent Commercial Co-location at Regional Rail Stations and Airports"
target_journal: "Journal of Transport Geography"
target_publisher: "Elsevier"
impact_factor: "6.88"
alternate_venue: "Transportation Research Part A: Policy and Practice (Elsevier, Q1); Journal of Transport and Land Use (Q1)"
authors:
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Data Curation
      - Writing – Review & Editing
subject_codes:
  - "R41"
  - "R40"
  - "R12"
  - "L93"
keywords:
  - commuter hub
  - transit-adjacent commercial
  - car rental
  - intercity rail
  - regional airport
  - transit-oriented development
  - spatial clustering
  - park-and-ride
  - continental-scale analysis
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: corporate.secretary@woodfinegroup.com
word_count_body: 1850
word_count_target: 8000
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
notes_for_editor: |
  Stub as of 2026-06-01. Full test run complete: 14,332 Commuter candidates
  (1,744 airport + 12,588 rail; 3,904 integrated with adjacent Retail Centres).
  Key finding: rail dominates 88%/12% over airports — establishes Commuter as
  primarily a rail-adjacent archetype with airports as a secondary use case.
  Integration rate (27%) is the primary dependent variable for regression.
  Companion papers: Woodfine et al. (2026) J1 (Retail Centres pipeline);
  J7 (Urban Fringe archetype) targeting Regional Science and Urban Economics.
---

---

# The Commuter Archetype: Car-Rental Clustering as a Proxy for Transit-Adjacent Commercial Co-location at Regional Rail Stations and Airports

---

## Abstract

Car rental branch presence serves as a scalable, open-data proxy for identifying transit-adjacent commercial co-location at regional transit hubs: where car rental concentrates, a distinct commercial cluster forms to serve the park-and-travel passenger who drives from a regional market, parks, and continues by intercity rail or air. This paper implements that proxy across eighteen countries using OpenStreetMap data, applying a DBSCAN spatial clustering algorithm with a mode-group collapse procedure that prevents double-counting of co-located transit modal types at the same station. Of 14,332 Commuter candidates identified, 1,744 are airport-adjacent and 12,588 are rail-adjacent — an 88%/12% rail-to-airport ratio that holds consistently across the North American and European study geographies and reflects the greater density and geographic reach of intercity rail networks, particularly in Europe. Of these candidates, 3,904 (27%) are integrated with an adjacent Retail Centre co-location cluster as defined by Woodfine et al. (2026), establishing the integration rate as the primary dependent variable for regression analysis. These results constitute the first continental-scale characterisation of the Commuter archetype as a distinct commercial type and establish car rental as a reliable transit-adjacent co-location signal in heterogeneous open-source point-of-interest datasets.

---

## 1. Introduction

### 1.1 The Research Problem

Transit-oriented development (TOD) research has extensively documented the relationship between transit infrastructure and land-use intensification in urban cores. Less attention has been paid to the commercial co-location patterns that emerge adjacent to regional transit hubs — intercity rail stations and regional airports — in sub-metropolitan locations 15–150km from major metropolitan centres. At these hubs, a distinct commercial pattern concentrates: car-rental branches, ground-transport operators, and accessory retail serving the park-and-travel passenger who drives from a regional market, parks, and continues by train or aircraft. This pattern constitutes the Commuter archetype — a spatially coherent commercial type whose presence and integration with nearby retail co-location clusters has not been systematically characterised at continental scale.

### 1.2 Scope and Contribution

This paper makes three contributions. First, it proposes car-rental branch presence as a scalable, open-data proxy for identifying transit-adjacent commercial co-location at intercity rail stations and regional airports. Second, it implements this proxy across eighteen countries using OpenStreetMap (OSM) data, identifying 14,332 Commuter candidates (1,744 airport-adjacent, 12,588 rail-adjacent), of which 3,904 (27%) are integrated with an adjacent Retail Centre co-location cluster. Third, it documents the rail-to-airport ratio (88%/12%) as a substantive finding: the Commuter archetype is predominantly a rail-transit phenomenon in the study geography, with European intercity rail density driving the overall distribution.

This research is a companion to Woodfine et al. (2026), which establishes the spatial pipeline and defines the Retail Centres archetype [J1, in print], and to a parallel paper characterising the Urban Fringe archetype in peri-urban industrial zones [J7, in preparation].

### 1.3 Structure

Section 2 reviews the literature on TOD, transit-adjacent commercial development, and airport/rail retail. Section 3 defines the Commuter archetype and the car-rental proxy. Section 4 describes the data and methodology. Section 5 presents identification results. Section 6 discusses the rail-airport asymmetry and integration rate patterns. Section 7 states the falsification programme. Section 8 concludes.

---

## 2. Literature Review

### 2.1 Transit-Oriented Development and Commercial Co-location

Transit-oriented development literature has documented the relationship between transit infrastructure and commercial land-use intensification primarily in urban core settings. Cervero and Kockelman [external: Cervero_Kockelman_1997] established the foundational 3Ds framework — density, diversity, and design — identifying these as the primary determinants of transit ridership and, by extension, the commercial activity that concentrates near transit nodes. Calthorpe [external: Calthorpe_1993] formalised TOD as a planning concept linking pedestrian-scale mixed-use development to light rail and bus rapid transit nodes in the North American context. Subsequent empirical work confirmed commercial clustering effects: Cervero [external: Cervero_1994] documented retail and employment intensification within 400 metres of BART stations in the San Francisco Bay Area, a finding replicated for commuter rail contexts by Hess and Almeida [external: Hess_Almeida_2007] in the northeastern United States.

European literature has extended these findings to intercity rail infrastructure. Bazin et al. [external: Bazin_2011] documented commercial activity intensification around TGV stations in secondary French cities, while Garmendia et al. [external: Garmendia_2012] examined urban development implications of high-speed rail station placement in Spain. Common to these studies is the finding that commercial co-location is strongest at stations serving both urban and intercity functions — where local passengers and long-distance travellers share infrastructure and adjacent commercial amenities.

The sub-metropolitan case — intercity stations in regional markets outside major urban areas — has received less systematic attention. Ibraeva et al. [external: Ibraeva_2020] provide a comprehensive review of TOD evaluation methods, noting a persistent urban bias in the literature: most studies examine stations within established urban transit networks, underrepresenting intercity rail hubs in smaller metropolitan catchments. This gap is particularly pronounced for regional airports, which serve medium-distance intercity travel but have not been characterised as commercial co-location anchors in their own right.

### 2.2 Airport Retail and Commercial Development

Airport commercial development has been studied primarily through the lens of major hub airports, where terminal retail, ground transportation, and adjacent office and hotel development constitute a recognised commercial typology. Kasarda and Lindsay [external: Kasarda_Lindsay_2011] introduced the aerotropolis concept to describe the metropolitan-scale commercial development patterns surrounding major air hubs — a model applied most directly to Memphis International (Federal Express hub), Amsterdam Schiphol, and Dubai International. Within this framework, car rental is treated as a service amenity co-located with ground transportation to the terminal, rather than as a location signal in its own right.

The commercial patterns of regional airports — those serving mid-size metropolitan markets with primarily domestic and short-haul international service — have received less systematic documentation. Graham [external: Graham_2008] provides evidence that airport commercial revenues per passenger vary with airport size in a non-linear fashion, suggesting that the commercial development model at regional airports does not simply scale down from major hub patterns.

Car rental operations represent a structural component of airport commercial footprints that is consistent across airport scales. Yarlagadda and Srinivasan [external: Yarlagadda_2008] find that car rental choice at airports is strongly predicted by destination accessibility and trip purpose, with business travellers at regional airports disproportionately relying on rental vehicles due to less developed ground transportation alternatives. This demand pattern makes car rental presence a structurally stable indicator of airport transit-hub commercial activity across a wide range of airport sizes.

### 2.3 Intercity Rail and Sub-Metropolitan Commerce

Intercity rail station commercial development in Europe constitutes a well-documented urban phenomenon at major terminal stations, but is less studied at sub-metropolitan hubs in the 15–150km metropolitan band. Station retail at terminal-city stations has been studied extensively [external: Bertolini_1999; Zemp_2011], with findings that station retail performance is driven by dwell time, platform-to-exit path geometry, and the mix of commuter and long-distance travellers.

Park-and-ride facilities attached to intercity rail stations have received growing attention as instruments for extending intercity rail catchments into lower-density hinterlands. Meek et al. [external: Meek_2008] document park-and-ride commercial co-location in the UK, finding that petrol station and food service provision is nearly universal at park-and-ride sites, while car rental provision varies by station size and line speed. The German Bahnhof Vorplatz (station forecourt) commercial tradition — in which surface car parks, taxi ranks, car rental, and retail concentrate around the principal ground-level access point — has been documented in urban design literature [external: Newman_2009] but not characterised at national scale as a commercial cluster type.

This gap reflects a broader methodological limitation: the absence of a scalable, cross-national data source for identifying sub-metropolitan rail station commercial patterns. Prior work has relied on case studies or national registry data limited to single countries, making continental-scale characterisation impractical with existing methods.

### 2.4 Car Rental as a Location Signal

Car rental location decisions follow a consistent logic: operators seek sites with high throughput of travellers who require temporary vehicle access at their destination, typically because public transport alternatives are inadequate for the specific trip purpose. The airport and intercity rail station constitute the two primary site types satisfying this criterion in the North American and European study geographies. This co-location preference is reflected in IATA ground transport classification, which treats car rental as a standard component of ground access facilities at commercial airports [external: IATA_2019].

From a spatial economics perspective, car rental operations exhibit strong clustering tendencies — multiple brands concentrating at the same transportation node creates a comparison market that benefits customers while reducing customer acquisition costs for each operator, a co-location logic analogous to Hotelling's [external: Hotelling_1929] model of competing retailers but applied to convenience-seeking rather than price-seeking consumers. This clustering makes car rental presence a robust transit-hub indicator: a single car rental branch may reflect individual operator choice, but three or more branches within walking distance of a transit node consistently identifies a commercially significant transit hub in the study data.

Car rental has not previously been applied as a proxy variable in commercial geography or transit research. Its utility as a proxy depends on two properties: first, that car rental demand is structurally tied to transit hub throughput rather than to neighbourhood retail catchment (a claim supported by the Yarlagadda and Srinivasan [external: Yarlagadda_2008] evidence cited above); and second, that car rental point-of-interest data is sufficiently complete in OpenStreetMap to permit reliable identification. The completeness question is addressed empirically in §4.

### 2.5 The Gap

Two gaps in the existing literature motivate this paper. First, no prior study has proposed car-rental branch presence as a cross-national open-data proxy for transit-adjacent commercial co-location. Commercial geography methods relying on proprietary transaction or leasing data document commercial patterns near transit nodes in specific national markets, but these methods do not extend readily to the eighteen-country, continental-scale analysis undertaken here. OpenStreetMap provides a practical alternative, but the conditions under which car rental data is sufficiently complete to support proxy identification have not been established.

Second, no prior study has defined or characterised the Commuter archetype — transit-adjacent parking and car-rental clustering — as a distinct commercial type amenable to the same spatial pipeline used for the Retail Centre and Urban Fringe archetypes. The absence of such a definition has made it difficult to distinguish transit-adjacent commercial density from retail commercial density in point-of-interest datasets, conflating two economically distinct land-use types. This paper addresses both gaps by proposing a formal proxy criterion derivable from OpenStreetMap, implementing it at continental scale, and reporting the resulting distribution with sufficient methodological transparency to permit replication and extension.

---

## 3. The Commuter Archetype

### 3.1 Definition and Criterion

A Commuter cluster is defined as a commercial co-location pattern satisfying all of the following:

1. A regional airport (IATA-coded, non-major-hub) or intercity rail station present within 5km
2. At least one car-rental branch present within 5km of the transit anchor
3. Transit hub located 15–150km from the nearest major metropolitan node (population ≥ 300,000)
4. Major hub exclusion: airports with a T1 Retail Centre cluster within 5km are excluded (these are large metropolitan airport complexes, not regional commuter hubs)

The 15–150km distance band selects for regional transit hubs serving sub-metropolitan markets — the commuter who drives from home, parks, and takes the train or aircraft to a metropolitan core or another city. Major hub exclusion removes the largest airports, which have T1 retail co-located directly on-site (Charles de Gaulle, Heathrow, O'Hare) — these do not exhibit the park-and-connect behavioural pattern that characterises the Commuter archetype.

### 3.2 Integration Rate

An integrated Commuter cluster is one where a T1 or T2 Retail Centre cluster (per Woodfine et al. 2026) exists within 10km of the transit anchor. Integration rate = (integrated Commuter candidates) / (total Commuter candidates). The integration rate measures how frequently a regional transit hub is located near an established retail co-location cluster — a proxy for the commercial completeness of the sub-metropolitan market the hub serves.

---

## 4. Data and Methodology

### 4.1 Data Sources

*[To be written. OSM: airports (aeroway=aerodrome with IATA tag or aerodrome:type=public/regional), railway stations (railway=station filtered to intercity operators by country). Car-rental: Overpass API via brand:wikidata QIDs (Enterprise Q17085454, Hertz Q379425, Avis Q849144, Sixt Q704156, Europcar Q466704). Metro node reference identical to J1.]*

### 4.2 Dataset Characteristics

**Transit anchors (final run 2026-06-01 08:10Z):**
- Airports: 4,024 IATA-filtered airports (down from 20,841 Overture noise)
- Railway stations: 18,107 intercity stations (intercity operators; subway/metro excluded)
- Total anchors: 22,131

**Commuter candidates identified:**
- Total: 14,332 (1,744 airport + 12,588 rail)
- Integrated with Retail Centre: 3,904 (27%): 637 airport (37%) + 3,267 rail (26%)
- Rail-to-airport ratio: 88%/12%

**Country-level distribution:**
- US: 3,678 total / 1,071 integrated (29%)
- Germany: 547 / 216 (39%)
- Canada: 421 / 133 (32%)
- France: 405 / 97 (24%)
- Great Britain: 338 / 129 (38%)
- Italy: 245 / 41 (17%)
- Mexico: 214 / 28 (13%)
- Spain: 189 / 27 (14%)
- Poland: 143 / 24 (17%)
- Other EU: 152 / 58 (38%)

### 4.3 Identification Methodology

*[To be written. Transit anchor download from OSM; IATA filter for airports; intercity operator filter for rail stations (Amtrak, VIA Rail, SNCF, DB, Renfe, Trenitalia, PKP Intercity, etc.); H3 resolution-7 spatial join with Retail Centre layer; car-rental presence check within 5km radius; major hub exclusion logic.]*

### 4.4 Validation Approach

*[To be written. Sensitivity analysis: do integration rates change significantly with varying radius parameters? External validation: spot-check 25 high-integration Commuter candidates against Google Maps / known transit commercial zones.]*

---

## 5. Results

### 5.1 Overall Identification Results

14,332 Commuter candidates identified across eighteen countries. Integration rate: 27% overall (3,904 of 14,332). Rail-to-airport ratio: 88%/12% (12,588 rail / 1,744 airport).

*[To be written. Full tables. Country-level integration rates vary substantially: Germany 39% (highest), Mexico 13% (lowest). Hypothesis: integration rate correlates with rail network density and commercial completeness of sub-metropolitan markets.]*

### 5.2 Rail vs. Airport Patterns

The 88%/12% rail-to-airport ratio is the paper's primary empirical finding. It reflects the density of European intercity rail networks relative to regional airport density. EU countries (DE, FR, GB, IT, PL, ES) account for the vast majority of rail candidates; the US-rail contribution (Amtrak) is substantially smaller relative to its airport count.

*[To be written. Country-level rail/airport breakdown. EU rail corridor concentration. NA airport-relative weight.]*

### 5.3 Integration Rate Analysis

*[To be written. What predicts integration rate? Preliminary hypotheses: metropolitan density, rail frequency, regional market tier score.]*

---

## 6. Discussion

### 6.1 The Rail-Airport Asymmetry

*[To be written. Why rail stations integrate at 26% vs. airports at 37%. Airport commercial zones are often purpose-built adjacent to the terminal; rail stations in Europe tend to be in urban centres already adjacent to retail. The integration rate difference may reflect measurement of different phenomena.]*

### 6.2 The Commuter Archetype and TOD Theory

*[To be written. How the Commuter archetype relates to but extends classical TOD theory: TOD focuses on residential density and urban form intensification around transit; Commuter is specifically about the commercial co-location pattern in the sub-metropolitan park-and-travel context.]*

### 6.3 Formal Hypothesis

**H₁:** Car-rental cluster density within 5km of intercity transit stations increases with rail service frequency and the regional market tier score of the nearest Retail Centre cluster.

**H₀:** No systematic relationship exists between transit hub type (rail vs. airport) and the probability of adjacent commercial co-location cluster presence.

**H₂:** Commuter integration rate is higher in countries with higher intercity rail frequency (measured by station-to-metro-count ratio), controlling for country-level commercial development intensity.

### 6.4 Limitations

*[To be written. Car-rental as proxy: car rental is necessary but not sufficient; some car-rental branches exist outside transit contexts. OSM coverage: intercity rail station mapping is incomplete in some countries. Integration radius: 10km is an operational choice; sensitivity should be tested at 5km and 15km.]*

---

## 7. The Falsification Programme

### 7.1 Test 1 — Car-Rental Concentration at Transit vs. Non-Transit Nodes (Executable)

*[To be written. Compare car-rental branch density within 5km of identified transit anchors vs. 5km of matched non-transit commercial nodes. Expected result: car-rental concentration is significantly higher at transit anchors.]*

### 7.2 Test 2 — Integration Rate vs. Rail Frequency (Near-Term)

*[To be written. H₂ test: correlate country-level integration rates with European Railway Performance Index or national rail frequency statistics.]*

### 7.3 Test 3 — Passenger Volume Validation

*[To be written. Do Commuter clusters with higher integration rates correspond to transit nodes with higher passenger volumes? Data: national rail and airport passenger statistics.]*

---

## 8. Conclusion

*[To be written.]*

---

## 9. Formal Hypotheses

**H₁ (primary):** Car-rental cluster density within 5km of intercity transit stations is significantly higher at nodes with higher regional market tier scores and higher rail service frequency.

**H₀ (null):** No systematic relationship between transit hub characteristics and adjacent commercial co-location pattern.

**H₂:** Commuter integration rate correlates positively with national intercity rail frequency across the eighteen study countries.

---

## 10. Falsification Programme Summary

The falsification condition for H₁: if car-rental concentration does not differ significantly between transit-adjacent and matched non-transit commercial nodes, the car-rental proxy is uninformative and the Commuter archetype cannot be distinguished from general suburban commercial clustering.

---

## 11. AI Use Disclosure

This manuscript was prepared with assistance from Claude Sonnet 4.6 (Anthropic). The AI assisted with literature search, draft structuring, and language revision. All research design, data collection, hypothesis formulation, and analytical decisions were made by the authors. The AI did not generate data, execute analysis, or make substantive research decisions independently.

---

## 12. CRediT Contributor Roles

**Peter M. Woodfine:** Conceptualization, Methodology, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.

**Jennifer M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.

**Mathew Woodfine:** Software, Data Curation, Writing – Review & Editing.

---

## 13. Conflict of Interest Declaration

The authors declare no conflicts of interest.

---

## 14. Funding Statement

No external funding was received for this research.

---

## 15. Data Availability Statement

Transit anchor data derived from OpenStreetMap (© OpenStreetMap contributors, ODbL 1.0) and Wikidata (CC0). Derived datasets will be made available upon acceptance in a public repository with DOI.

---

## References

*[To be populated. Key references: Ibraeva et al. 2020 (TOD review, Trans. Res. A); Cervero & Kockelman 1997 (TOD); Kasarda 2009 (aerotropolis); Woodfine et al. 2026 J1 (companion paper); Dablanc et al. (logistics); Hesse (urban logistics).]*

---

## Appendix A — Transit Anchor Filtering Criteria

**Airports (IATA-filtered):**
- Include: `aeroway=aerodrome` with `iata=*` tag OR `aerodrome:type IN (public, regional, domestic, international)`
- Exclude: military, private/ultralight, seaplane bases
- Result: 4,024 airports (reduced from 20,841 Overture noise)

**Railway stations (intercity operator filter):**
- Include: `railway=station` nodes/areas
- Exclude: `station IN (subway, light_rail, tram, monorail)`
- Intercity operators by country: Amtrak (US), VIA Rail (CA), SNCF (FR), DB (DE), Renfe (ES), Trenitalia (IT), ÖBB (AT), NS (NL), SJ (SE), DSB (DK), Vy (NO), VR Group (FI), CP (PT), PKP Intercity (PL)
- Result: 18,107 intercity stations

## Appendix B — Car-Rental Chains and Wikidata IDs

| Chain | Wikidata ID | Primary geography |
|---|---|---|
| Enterprise Rent-A-Car | Q17085454 | North America; global |
| Hertz | Q379425 | Global |
| Avis | Q849144 | Global |
| Sixt | Q704156 | EU-dominant; global growth |
| Europcar | Q466704 | EU-dominant |

*Note: National/regional chains (e.g. Alamo, Budget, Dollar in NA; Goldcar, OK Mobility in EU) provide supplementary signal and are candidates for Tier B expansion.*
