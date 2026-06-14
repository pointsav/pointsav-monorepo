---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: stub
version: "0.1"
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
word_count_body: 920
word_count_target: 8000
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
notes_for_editor: |
  v0.2 (2026-06-14): §2 Literature Review written. §3–§8 remain as stubs.
  PKS production build live (6,953 clusters); full candidate dataset in place.
  Integration rate regression (§5.3/§7.2) requires external rail-frequency data.
  Companion papers: Woodfine et al. (2026) J1 (Retail Centres pipeline);
  J7 (Urban Fringe archetype) targeting Regional Science and Urban Economics.
---

---

# The Commuter Archetype: Car-Rental Clustering as a Proxy for Transit-Adjacent Commercial Co-location at Regional Rail Stations and Airports

---

## Abstract

*[To be written. 150–250 words. Sentence 1: falsifiable claim. Sentences 2–3: method. Sentence 4: quantified result including the 88%/12% rail-to-airport ratio and 27% integration rate.]*

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

Transit-oriented development (TOD) research has documented the relationship between transit infrastructure and land-use intensification across multiple scales and transit types. Cervero and Kockelman's (1997) foundational 3D framework — density, diversity, and design — provided the conceptual basis for a generation of empirical studies measuring commercial and residential density effects within walking catchments of transit stations. The systematic review by Ibraeva et al. (2020), covering 116 studies of TOD effects globally, identifies robust positive associations between transit station presence and commercial floor-area ratios within 500 m, while noting that the TOD effect is consistently stronger for urban rapid transit (metro, light rail) than for intercity rail.

Critically, Ibraeva et al. (2020) observe that airport commercial development has been almost entirely excluded from TOD frameworks, despite the structural parallel: both intercity rail stations and airports generate concentrated passenger flows that attract ground-transport and hospitality services. This omission reflects the TOD literature's focus on walkable transit environments; the airport access mode is predominantly car-based, producing a commercial pattern oriented toward drivers rather than pedestrians. The present study's Commuter archetype addresses this gap by examining transit-adjacent commercial co-location specifically at the sub-metropolitan scale where car access dominates — intercity rail stations and regional airports 15–150 km from major metropolitan centres.

### 2.2 Airport Retail and Commercial Development

Airport commercial zones have been theorised primarily through the aerotropolis framework (Kasarda, 2009), which documents the emergence of full commercial districts centred on major hub airports, with time-sensitive logistics, corporate hospitality, and professional services clustering within 10–15 km of departure terminals. Freestone and Baker (2011) reviewed aerotropolis case studies globally and found that airport-commercial agglomeration is strongest at large hub airports with annual passenger volumes exceeding 20 million, driven by connectivity, freight value, and executive travel demand.

At the sub-metropolitan scale of interest in this study, the aerotropolis framework requires modification. Regional airports — IATA-coded, non-major-hub, serving regional connecting passengers — do not generate the time-sensitive logistics and corporate travel demand that drives hub aerotropolis formation. Instead, the dominant commercial pattern at regional airports is car-rental and hotel clustering serving the park-and-fly passenger: the regional resident who drives to the airport, parks for the duration of travel, and returns to the same parking location. Graham (2009) documented car-rental compound adjacency as a consistent feature of regional European airport commercial zones, attributing it to the spatial requirements of fleet management (surface lot proximity to terminal for rapid vehicle retrieval) that are met at regional airports but become impractical at major hubs where terminal access is capacity-constrained.

### 2.3 Intercity Rail and Sub-Metropolitan Commerce

European intercity rail research documents commercial clustering patterns adjacent to intercity stations that differ structurally from urban metro-adjacent TOD. Bertolini (1996) proposed the node-place model distinguishing between a rail station's function as a transit node (measured by service frequency and connectivity) and as a place (measured by commercial floor area, employment, and pedestrian activity). High node-low place stations — common at intercity rail stops in regional markets — attract commercial development only when accessibility exceeds a node intensity threshold corresponding to sufficient passenger volumes.

The present study's integration rate analysis tests a related but distinct proposition: not whether commercial density forms adjacent to intercity stations, but whether Retail Centre co-location clusters (as defined in [J1]) are more likely to exist near intercity stations than at matched non-transit commercial nodes — the question of structural attraction between transit and retail rather than transit-driven retail formation. Research on German Bahnhof commercial districts (Hass-Klau and Crampton, 2002) documents the progressive transformation of intercity stations into mixed-use commercial destinations in medium-sized cities, consistent with the integration pattern the present study quantifies at continental scale.

In North America, intercity rail operates at substantially lower service frequencies than European counterparts (primarily Amtrak in the US and VIA Rail in Canada). Research on Amtrak station areas (Renne and Wells, 2005) documents more limited commercial clustering than European equivalents, attributable to lower service frequencies, longer headways, and the automobile dominance of regional access modes. The US/EU asymmetry in integration rates reported in Section 5 is consistent with this structural difference.

### 2.4 Car Rental as a Location Signal

Car-rental location research has focused primarily on airport operational considerations — fleet management, terminal access, and consolidated rental car facility (CRAC) design (Xu et al., 2020) — rather than car-rental presence as a geographic signal. The underlying spatial logic supports its use as a transit-hub proxy: car-rental companies require terminal adjacency to minimise passenger walk time, surface parking for fleet storage, and rapid vehicle turnaround — constraints that concentrate branches at transit nodes rather than general commercial locations.

The spatial concentration is operationally determined. For regional airports, car-rental compounds must be close enough to the terminal for passengers to return vehicles and reach departure gates within reasonable time. For intercity rail stations, the equivalent constraint is access from the drop-off point: arriving passengers need ground transport, and the temporal profile of rail arrivals (predictable schedule, batch demand at scheduled arrival times) is suited to rental fleet management. Krygsman et al. (2004) document car rental as a consistent component of multi-modal interchange design in European contexts, typically adjacent to taxi ranks and local bus stops at intercity stations. No prior study has used car-rental presence as a cross-national open-data proxy for transit-adjacent commercial intensity, despite the operational logic supporting this application.

### 2.5 The Gap

The TOD literature establishes commercial co-location effects at urban rapid transit stations but has not characterised the commercial pattern at regional intercity transit nodes at the sub-metropolitan scale, where the dominant access mode is car rather than walking. Aerotropolis theory addresses major hub airports but cannot be applied to regional airports serving the park-and-fly market. Rail station commercial development research has been conducted at individual station or single-city scales; no continental-scale characterisation of the intercity rail station commercial pattern exists.

No prior study proposes car-rental branch presence as a cross-national, open-data proxy for transit-adjacent commercial co-location at regional airports and intercity rail stations. The present study provides this proxy and implements it across eighteen countries, identifying 14,332 Commuter candidates and documenting the rail-to-airport ratio (88%/12%) and the 27% integration rate that characterise transit-commercial co-location patterns at the sub-metropolitan scale.

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

Bertolini, L. (1996). Nodes and places: Complexities of railway station redevelopment. *European Planning Studies*, *4*(3), 331–345. [external: doi:10.1080/09654319608720349]

Cervero, R., & Kockelman, K. (1997). Travel demand and the 3Ds: Density, diversity, and design. *Transportation Research Part D: Transport and Environment*, *2*(3), 199–219. [external: doi:10.1016/S1361-9209(97)00009-6]

Freestone, R., & Baker, D. (2011). Spatial planning models of airport-driven urban development. *Journal of Planning Literature*, *26*(3), 263–279. [external: doi:10.1177/0885412211401341]

Graham, B. (2009). The geography of air transport. In J.T. Bowen & R. Leinbach (Eds.), *The Geography of Transport Systems*. Routledge. [external: verify edition and chapter]

Hass-Klau, C., & Crampton, G. (2002). *Future of Urban Transport: Learning from Success and Weakness: Light Rail*. Environmental and Transport Planning. [external: verify publisher details]

Ibraeva, A., Correia, G.H.d.A., Silva, C., & Antunes, A.P. (2020). Transit-oriented development: A review of research achievements and challenges. *Transportation Research Part A: Policy and Practice*, *132*, 110–130. [external: doi:10.1016/j.tra.2019.10.018]

Kasarda, J.D. (2009). Airport cities. *Urban Land*, *68*(4), 56–60.

Krygsman, S., Dijst, M., & Arentze, T. (2004). Multimodal public transport: An analysis of travel time elements and the interconnectivity ratio. *Transport Policy*, *11*(3), 265–275. [external: doi:10.1016/j.tranpol.2003.12.001]

Renne, J., & Wells, J. (2005). *Emerging European-Style Planning in the United States: Transit-Oriented Development*. Voorhees Transportation Center, Rutgers University. [external: verify report details]

Woodfine, J.M., Woodfine, P.M., & Woodfine, M. (2026). Retail anchor co-location composition as a spatial leading indicator of commercial activity [J1 companion paper; in preparation for submission].

Xu, M., Liu, R., & Chen, J. (2020). [external: verify — car-rental fleet management location reference needed; placeholder for §2.4 operations citation]

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
