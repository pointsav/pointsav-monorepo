# Chain Coverage Audit — 2026-05-17

Audit of `config.py` taxonomy (ALPHA_HYPERMARKET, ALPHA_LIFESTYLE,
ALPHA_HARDWARE, ALPHA_WAREHOUSE, REGION_CONFIG) against major
national / sub-national retail chains across the 13-country footprint
(US, CA, MX, ES, FR, DE, GB, IT, NL, AT, PL, GR, PT, plus Nordics
SE/NO/DK/FI/IS).

Format per gap entry:

- **Chain** — display name
- **ISO** — country
- **QID** — Wikidata identifier
- **Stores** — order-of-magnitude
- **Placement** — recommended bucket
- **Reasoning** — qualification basis

---

## Section 1 — Current taxonomy snapshot

### ALPHA_HYPERMARKET
- **NA:** walmart-{us,ca,mx}, target-us, soriana-mx, fred-meyer-us
- **EU:** mercadona-es, tesco-uk, sainsburys-uk, bilka-dk, obs-coop-no,
  hagkaup-is, k-citymarket-fi, prisma-fi, carrefour-hypermarket-fr,
  auchan-fr, leclerc-fr, ecenter-de, marktkauf-de, kaufland-de

### ALPHA_LIFESTYLE
- **NA + EU:** IKEA-only across all 13 countries + Nordics.

### ALPHA_HARDWARE
- **NA:** home-depot-{us,ca,mx}, alaska-industrial-hardware-us, menards-us
- **EU:** leroy-merlin-{es,it,gr,pl,fr,pt}, castorama-{pl,fr},
  k-rauta-fi, hornbach-{de,at}, praxis-nl, gamma-nl, karwei-nl,
  bauhaus-se, bq-uk, obs-bygg-no

### ALPHA_WAREHOUSE
- **NA:** costco-{us,ca,mx}, sams-club-{us,mx}, bjs-wholesale-us
- **EU:** costco-{es,se,is,uk,fr}, makro-{es,nl,pl}

### REGION_CONFIG — `anchor` slot composition (per country)
| Country | Anchors |
|---|---|
| US | walmart, target, fred-meyer, ikea, home-depot, costco |
| CA | walmart, ikea, real-canadian-superstore, home-depot, costco |
| MX | walmart, soriana, ikea, home-depot, costco |
| ES | mercadona, ikea, costco, makro |
| IT | ikea (sole anchor) |
| GR | ikea (sole anchor) |
| PL | ikea, makro |
| FR | ikea, costco, carrefour-hyper, auchan, leclerc |
| DE | ikea, ecenter, marktkauf, kaufland |
| GB | tesco, sainsburys, ikea, costco |
| AT | ikea (sole anchor) |
| NL | ikea, makro |
| PT | ikea (sole anchor) |
| NORDICS | ikea, bilka, prisma, k-citymarket, obs-coop, hagkaup, costco-{se,is} |

**Observation:** IT, GR, AT, PT are IKEA-only anchor regions. PL has
only ikea+makro. NL has only ikea+makro. This is the highest-priority
gap area — these countries cannot produce T1 clusters without a
hypermarket-class anchor.

---

## Section 2 — GAP CANDIDATES (recommended additions)

### 2.A — UNITED STATES (US)

#### Whole Foods Market
- **ISO:** US
- **QID:** Q1758180
- **Stores:** ~530
- **Placement:** ALPHA_HYPERMARKET (premium grocery anchor) — currently
  in `GENERIC_FOOD` only (`whole-foods-us`)
- **Reasoning:** Amazon-owned, national footprint, strong brand
  identity, anchor-tenant role in premium retail centers. Format is
  smaller than Walmart but cluster-initiator-worthy in dense
  metros. Recommend HYPERMARKET; alternatively retain as Food and add
  as REGION_CONFIG secondary.

#### H-E-B
- **ISO:** US
- **QID:** Q830621
- **Stores:** ~435 (TX + emerging MX presence as Mi Tienda / Joe V's)
- **Placement:** ALPHA_HYPERMARKET (sub-national but dominant)
- **Reasoning:** Dominant TX grocer; H-E-B Plus! locations are
  hypermarket format with general merchandise. Sub-national but a T1
  anchor in any TX market — leaving it out under-counts ~60% of TX
  clusters.

#### Publix
- **ISO:** US
- **QID:** Q1639305
- **Stores:** ~1,400 (SE US)
- **Placement:** ALPHA_HYPERMARKET (sub-national, SE-dominant)
- **Reasoning:** FL/GA/AL/SC/NC/TN/VA flagship grocer; strong brand;
  anchor-tenant role. Larger store count than Fred Meyer (which IS in
  the taxonomy). Under-counting SE US T1 clusters.

#### Meijer
- **ISO:** US
- **QID:** Q1639930
- **Stores:** ~260 (Midwest)
- **Placement:** ALPHA_HYPERMARKET (Midwest-regional supercenter)
- **Reasoning:** Invented the supercenter format; MI/OH/IN/IL/KY/WI
  general-merchandise+grocery hypermarket. Direct functional analog to
  Walmart Supercenter. Strong T1-anchor candidate for Midwest.

#### Wegmans
- **ISO:** US
- **QID:** Q1377617
- **Stores:** ~110 (Mid-Atlantic / Northeast)
- **Placement:** ALPHA_HYPERMARKET (premium-format, regional)
- **Reasoning:** Cult brand; large-format premium grocery; consistent
  anchor-tenant role. Lower store count than peers but extremely high
  per-store throughput and brand pull.

#### Sprouts Farmers Market
- **ISO:** US
- **QID:** Q7581369
- **Stores:** ~415
- **Placement:** REGION_CONFIG secondary only (or GENERIC_FOOD)
- **Reasoning:** Specialty natural grocer, format smaller than
  hypermarket. Useful as secondary co-tenant signal but not a T1
  cluster initiator.

#### Trader Joe's
- **ISO:** US
- **QID:** Q2597711
- **Stores:** ~580
- **Placement:** REGION_CONFIG secondary only (or GENERIC_FOOD)
- **Reasoning:** Small-format specialty; high brand pull but does not
  anchor large retail centers. Useful co-tenant indicator.

#### Kroger (parent brand)
- **ISO:** US
- **QID:** Q153417
- **Stores:** ~2,700 incl. all banners
- **Placement:** ALPHA_HYPERMARKET (consider Kroger Marketplace
  sub-format specifically, ~140 supercenter-class)
- **Reasoning:** Fred Meyer already covered. Kroger Marketplace is the
  Walmart-Supercenter analog in OH/MI/TX/AZ etc. — likely missing T1
  clusters in markets where Fred Meyer is absent. Recommend ingesting
  as `kroger-marketplace-us` (sub-format), not full Kroger fleet
  (which includes small-format banners).

---

### 2.B — CANADA (CA)

#### Real Canadian Superstore — already added (REGION_CONFIG)
- **Status:** present in `REGION_CONFIG["CA"]["anchor"]` but NOT in
  `ALPHA_HYPERMARKET["NA"]`. **DEFECT:** dual-membership is required
  for cluster-initiator + secondary-scoring. Likely needs to be added
  to `ALPHA_HYPERMARKET["NA"]` set.
- **QID:** Q7300856
- **Stores:** ~130
- **Action:** add `real-canadian-superstore-ca` to
  ALPHA_HYPERMARKET["NA"].

#### Loblaws (parent)
- **ISO:** CA
- **QID:** Q3257626
- **Stores:** ~570 across banners; Loblaws-branded ~135 in ON/QC
- **Placement:** REGION_CONFIG secondary (or ALPHA_HYPERMARKET for
  the large Loblaws urban-format)
- **Reasoning:** Real Canadian Superstore is Loblaw Companies' western
  banner; Loblaws (the storefront name) is the eastern equivalent.
  Without it, ON/QC under-counts T1 clusters.

#### Sobeys
- **ISO:** CA
- **QID:** Q1758199
- **Stores:** ~1,500 across banners (Sobeys, Safeway-CA, IGA, FreshCo)
- **Placement:** ALPHA_HYPERMARKET (Sobeys + Safeway-CA as banners)
- **Reasoning:** #2 Canadian grocer; safeway-ca already in
  GENERIC_FOOD. Sobeys main banner not ingested. Strong anchor-tenant
  role.

#### Metro Inc.
- **ISO:** CA
- **QID:** Q1944628
- **Stores:** ~950 (QC/ON; banners: Metro, Super C, Food Basics)
- **Placement:** REGION_CONFIG secondary (QC/ON)
- **Reasoning:** #3 Canadian grocer; QC-dominant. Smaller format
  than Superstore but anchor-tenant in QC plazas.

#### Canadian Tire — already added
- **Status:** in REGION_CONFIG CA hardware + GENERIC_HARDWARE.
  **Note:** Canadian Tire is hardware-adjacent (automotive + sports +
  home) — current "secondary only" placement is correct. Confirmed
  not a T1 hardware anchor (smaller stores than Home Depot/Lowe's
  Canada).

#### Rona / RONA+
- **ISO:** CA
- **QID:** Q940519
- **Stores:** ~425
- **Placement:** ALPHA_HARDWARE NA
- **Reasoning:** Canada's #2 home-improvement chain after Home Depot;
  larger national footprint than Lowe's Canada (Lowe's sold its
  Canadian operations to Sycamore Partners in 2023, rebranded under
  RONA). **High-priority gap.**

---

### 2.C — MEXICO (MX)

#### Chedraui
- **ISO:** MX
- **QID:** Q5088737
- **Stores:** ~280 (Selecto + Súper Chedraui banners)
- **Placement:** ALPHA_HYPERMARKET — currently in GENERIC_FOOD per
  comment
- **Reasoning:** #3 MX hypermarket after Walmart/Soriana; clearly
  anchor-tenant scale. Comment in config.py says "remain Food (format
  mismatch)" — recommend revisit; Chedraui Selecto is full hypermarket.

#### Bodega Aurrerá
- **ISO:** MX
- **QID:** Q4116302
- **Stores:** ~2,200
- **Placement:** Per CHAIN_FAMILIES doctrine, this is a Walmart
  sub-brand — but Walmart de México operates Bodega Aurrerá as a
  distinct retail format, not a sub-entity of a Walmart store. Two
  options:
  (a) Add as standalone ALPHA_HYPERMARKET (recommended) — different
      format (hard-discount), distinct store locations, would dilute
      Walmart-MX if conflated.
  (b) Skip — already noted in ANCHOR_DISPLAY_NAMES as
      `bodega-aurrera-mx` but not in any ALPHA_* set.
- **Reasoning:** ANCHOR_DISPLAY_NAMES has the entry but no ingest
  configured. Largest hard-discount chain in MX. Anchor-tenant in
  smaller cities where full Walmart is absent.

#### La Comer / City Market / Fresko / Sumesa
- **ISO:** MX
- **QID:** Q103862594 (La Comer)
- **Stores:** ~80 (premium banner)
- **Placement:** REGION_CONFIG secondary
- **Reasoning:** Premium-tier, low store count, sub-national (mostly
  CDMX/Bajío). Not T1 anchor scale.

#### HEB México
- **ISO:** MX
- **QID:** part of Q830621
- **Stores:** ~70 (NL/Coahuila/Tamaulipas)
- **Placement:** REGION_CONFIG secondary (or extension of
  h-e-b-us ingest if BBOX permits cross-border)
- **Reasoning:** Sub-national MX presence; anchor-tenant in Monterrey
  metro and along TX border.

---

### 2.D — SPAIN (ES)

#### Carrefour Hypermarket — Spain variant
- **ISO:** ES
- **QID:** Q217599
- **Stores:** ~205 (Carrefour ES large-format)
- **Placement:** ALPHA_HYPERMARKET EU — **defect:** France variant
  added in Phase 5, ES variant not present despite ANCHOR_DISPLAY_NAMES
  entry for `carrefour-hypermarket-es`.
- **Reasoning:** Mercadona is currently the sole ES anchor — adding
  Carrefour-Hyper-ES would unlock many additional T1 clusters in
  Madrid/Barcelona/Valencia metros.

#### Alcampo (Auchan Spain)
- **ISO:** ES
- **QID:** Q2832081
- **Stores:** ~65 hypermarkets + ~250 supermarkets
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Major Spanish hypermarket; entry exists in
  ANCHOR_DISPLAY_NAMES (`alcampo-es`) but not ingested. Direct
  Walmart-class analog. **High-priority gap.**

#### Hipercor (El Corte Inglés hypermarket format)
- **ISO:** ES
- **QID:** Q5765404
- **Stores:** ~40
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Hypermarket arm of El Corte Inglés group;
  cluster-initiator in major metros.

#### El Corte Inglés (department-store format)
- **ISO:** ES
- **QID:** Q623634
- **Stores:** ~90
- **Placement:** ALPHA_LIFESTYLE EU (premium department store,
  IKEA-class footprint though different format) OR ALPHA_HYPERMARKET
  (the Hipercor floors qualify it).
- **Reasoning:** Spain's flagship department-store chain; flagship
  stores are anchor-tenant cathedrals in every major city. Best fit
  may be ALPHA_LIFESTYLE alongside IKEA given format (multi-floor
  destination retail).

#### E.Leclerc Spain
- **ISO:** ES
- **QID:** Q1273376
- **Stores:** ~17
- **Placement:** REGION_CONFIG secondary (low count)
- **Reasoning:** Display name exists (`leclerc-es`) but small Spanish
  footprint. Not T1 scale in ES.

#### Eroski
- **ISO:** ES
- **QID:** Q1356126
- **Stores:** ~1,600 across banners; ~50 hypermarkets
- **Placement:** ALPHA_HYPERMARKET (hypermarket banner only) or
  REGION_CONFIG secondary
- **Reasoning:** Basque cooperative; dominant in N. Spain (País
  Vasco, Navarra, La Rioja). Eroski hypermarket format is T1-scale
  regionally.

#### DIA
- **ISO:** ES
- **QID:** Q925417
- **Stores:** ~2,300
- **Placement:** REGION_CONFIG secondary (or GENERIC_FOOD)
- **Reasoning:** Hard-discount small format; not anchor-tenant
  format. Useful co-tenant signal.

---

### 2.E — FRANCE (FR)

#### Intermarché
- **ISO:** FR
- **QID:** Q3153200
- **Stores:** ~1,800
- **Placement:** ALPHA_HYPERMARKET EU (Hyper version) or REGION_CONFIG
  secondary
- **Reasoning:** Les Mousquetaires cooperative; second-largest French
  grocer after Leclerc. Intermarché Hyper sub-format is T1-anchor scale.

#### Casino / Géant Casino
- **ISO:** FR
- **QID:** Q341741 (Casino) / Q1495785 (Géant)
- **Stores:** ~120 Géant Casino hypermarkets
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Major French hypermarket banner; though group is in
  financial restructuring, locations remain anchor-tenants.

#### Super U / Hyper U (Système U)
- **ISO:** FR
- **QID:** Q2378230
- **Stores:** ~1,600 across banners; ~60 Hyper U
- **Placement:** ALPHA_HYPERMARKET (Hyper U only) + REGION_CONFIG
  secondary for Super U
- **Reasoning:** Hyper U is T1-anchor; Super U is co-tenant scale.

#### Cora
- **ISO:** FR
- **QID:** Q1129778
- **Stores:** ~60 (acquired by Carrefour 2024 — being rebranded)
- **Placement:** Skip (in transition)
- **Reasoning:** Carrefour acquired Cora 2024; locations being
  converted to Carrefour format. Will be captured under
  carrefour-hypermarket-fr re-ingest.

#### Galeries Lafayette / Printemps
- **ISO:** FR
- **QID:** Q574569 / Q913498
- **Stores:** ~65 / ~21
- **Placement:** ALPHA_LIFESTYLE EU (premium department store)
- **Reasoning:** Multi-floor destination retail; anchor-tenant in
  every major French city. Format parallel to El Corte Inglés in ES.

---

### 2.F — GERMANY (DE)

#### Real
- **ISO:** DE
- **QID:** Q2045759
- **Stores:** Declining — ~30 remaining after 2020 Metro divestiture;
  many converted to Kaufland / Edeka.
- **Placement:** Skip (in liquidation; functionally replaced by
  Kaufland which is already covered).
- **Reasoning:** Brand effectively wound down.

#### Globus
- **ISO:** DE
- **QID:** Q1522864
- **Stores:** ~50 hypermarkets (DE) + AT/CZ/RU
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Independent SHK Group hypermarket; T1-anchor scale,
  particularly in Saarland/Rheinland-Pfalz.

#### Rewe Center / Rewe XL
- **ISO:** DE
- **QID:** Q689380 (Rewe)
- **Stores:** ~3,300 Rewe total; ~70 Rewe Center large-format
- **Placement:** Rewe Center / Rewe XL as ALPHA_HYPERMARKET; standard
  Rewe as REGION_CONFIG secondary.
- **Reasoning:** Rewe Center is the large-format hypermarket arm.

#### Edeka (full brand, not just E center/Marktkauf already covered)
- **ISO:** DE
- **QID:** Q701755
- **Stores:** ~11,200 Edeka full network
- **Placement:** Already covered via subsidiaries ecenter-de +
  marktkauf-de (the hypermarket sub-formats). Standard Edeka =
  REGION_CONFIG secondary if added. Likely no action needed.

#### Karstadt / Galeria Kaufhof (now Galeria)
- **ISO:** DE
- **QID:** Q695934
- **Stores:** ~83 remaining (2024 restructuring; further closures
  through 2026)
- **Placement:** ALPHA_LIFESTYLE EU (department store)
- **Reasoning:** Germany's flagship department store; multi-floor
  anchor-tenant. Footprint shrinking; verify current count before
  ingest.

---

### 2.G — UNITED KINGDOM (GB)

#### ASDA
- **ISO:** GB
- **QID:** Q297410
- **Stores:** ~630
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** UK #3 grocer after Tesco/Sainsbury's; Walmart-owned
  until 2021. Direct hypermarket analog. **High-priority gap.**

#### Morrisons
- **ISO:** GB
- **QID:** Q922344
- **Stores:** ~500
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** UK #4 grocer; full-format supermarket/hypermarket.
  **High-priority gap.**

#### Waitrose
- **ISO:** GB
- **QID:** Q771734
- **Stores:** ~330
- **Placement:** REGION_CONFIG secondary (premium-small format)
- **Reasoning:** Premium grocer; smaller-format; high brand pull but
  not T1 anchor scale.

#### Marks & Spencer (Food + Clothing)
- **ISO:** GB
- **QID:** Q714491
- **Stores:** ~1,000 (incl. ~300 full stores; rest are M&S Food only)
- **Placement:** ALPHA_LIFESTYLE EU (full department stores) +
  REGION_CONFIG secondary
- **Reasoning:** Full-line M&S stores are department-store anchors;
  M&S Food are convenience-format secondaries. Recommend ingest as
  `marks-and-spencer-uk` for full-line + skip food-only.

#### John Lewis
- **ISO:** GB
- **QID:** Q1918981
- **Stores:** ~34
- **Placement:** ALPHA_LIFESTYLE EU (premium department store)
- **Reasoning:** Low count but extreme anchor-tenant role in
  John-Lewis-anchored shopping centres. Worth ingesting alongside IKEA.

#### Selfridges / Harrods / Liberty
- **ISO:** GB
- **QID:** Q920860 / Q188822
- **Stores:** ~4 / 1
- **Placement:** Skip (too few; flagship-only)
- **Reasoning:** Iconic but not a fleet — not useful for cluster
  detection.

---

### 2.H — ITALY (IT)

**Note:** IT currently has IKEA as sole anchor — critical gap.

#### Esselunga
- **ISO:** IT
- **QID:** Q1377048
- **Stores:** ~190
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Dominant N. Italian (Lombardy/Toscana/Lazio)
  grocer; large-format. **HIGHEST-PRIORITY IT GAP.**

#### Conad / Conad Superstore
- **ISO:** IT
- **QID:** Q1129178
- **Stores:** ~3,300 across banners; ~200 Conad Superstore (hyper format)
- **Placement:** Conad Superstore = ALPHA_HYPERMARKET EU
- **Reasoning:** Largest IT grocery cooperative. Conad Superstore is
  T1-anchor format.

#### Coop Italia / Ipercoop
- **ISO:** IT
- **QID:** Q1129233 (Coop Italia) / Q3801670 (Ipercoop)
- **Stores:** ~1,100 Coop + ~90 Ipercoop
- **Placement:** Ipercoop = ALPHA_HYPERMARKET EU. Standard Coop =
  REGION_CONFIG secondary.
- **Reasoning:** Ipercoop entry exists in ANCHOR_DISPLAY_NAMES but
  not in any ALPHA_* set.

#### Iper / Iper La Grande i
- **ISO:** IT
- **QID:** Q3801536
- **Stores:** ~27
- **Placement:** ALPHA_HYPERMARKET EU (low count but pure hypermarket)
- **Reasoning:** Iper La Grande i is pure-play hypermarket; entry
  exists in ANCHOR_DISPLAY_NAMES (`iper-it`); not ingested.

#### Bennet
- **ISO:** IT
- **QID:** Q3637710
- **Stores:** ~70
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Lombardy-dominant hypermarket chain; entry exists
  in ANCHOR_DISPLAY_NAMES; not ingested.

#### Auchan Italia
- **ISO:** IT
- **QID:** Q758603 / sold to Conad 2020
- **Stores:** 0 remaining (all rebranded Conad/Margherita)
- **Placement:** Skip (defunct in IT).
- **Reasoning:** Conad acquired and converted all Auchan IT 2020.

#### La Rinascente (department store)
- **ISO:** IT
- **QID:** Q1772341
- **Stores:** ~9
- **Placement:** Skip or ALPHA_LIFESTYLE (low count)
- **Reasoning:** Flagship-only; not fleet scale.

---

### 2.I — NETHERLANDS (NL)

#### Albert Heijn / Albert Heijn XL
- **ISO:** NL
- **QID:** Q1653985
- **Stores:** ~1,250 AH total; ~85 AH XL (hypermarket format)
- **Placement:** AH XL = ALPHA_HYPERMARKET EU. Standard AH =
  REGION_CONFIG secondary.
- **Reasoning:** NL #1 grocer (Ahold Delhaize). AH XL is direct
  Walmart-Supercenter analog. **HIGHEST-PRIORITY NL GAP.**

#### Jumbo
- **ISO:** NL
- **QID:** Q14716185
- **Stores:** ~700
- **Placement:** ALPHA_HYPERMARKET EU (Jumbo Foodmarkt sub-format) or
  REGION_CONFIG secondary
- **Reasoning:** NL #2 grocer; Jumbo Foodmarkt large-format is
  T1-anchor scale.

#### Hoogvliet / Plus / Vomar
- **ISO:** NL
- **QID:** Q1639538 / Q2103392
- **Stores:** ~70 / ~270
- **Placement:** REGION_CONFIG secondary
- **Reasoning:** Smaller-format chains; co-tenant scale.

#### HEMA (department-store / variety)
- **ISO:** NL
- **QID:** Q1130088
- **Stores:** ~750
- **Placement:** REGION_CONFIG secondary (small-format variety, not
  destination-retail anchor)
- **Reasoning:** Ubiquitous NL brand but small-format; not
  cluster-anchor.

#### Bijenkorf (department store)
- **ISO:** NL
- **QID:** Q920860
- **Stores:** 7
- **Placement:** Skip (flagship-only)

---

### 2.J — AUSTRIA (AT)

**Note:** AT currently has IKEA as sole anchor.

#### Billa Plus (Rewe Group AT, formerly Merkur)
- **ISO:** AT
- **QID:** Q806085
- **Stores:** ~135
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** AT's flagship hypermarket; rebranded from Merkur 2021.
  **HIGHEST-PRIORITY AT GAP.**

#### Interspar
- **ISO:** AT
- **QID:** Q1364056
- **Stores:** ~75
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Spar Austria's hypermarket format; T1-anchor.

#### Hofer (Aldi Süd AT)
- **ISO:** AT
- **QID:** Q1206257
- **Stores:** ~530
- **Placement:** REGION_CONFIG secondary
- **Reasoning:** Hard-discount small-format. Aldi-equivalent.

---

### 2.K — POLAND (PL)

**Note:** PL currently has only IKEA + Makro as anchors.

#### Auchan Polska
- **ISO:** PL
- **QID:** Q758603
- **Stores:** ~80 hypermarkets + ~25 supermarkets
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Display name exists (`auchan-pl`); not ingested.
  T1-anchor scale.

#### Carrefour Hypermarket Polska
- **ISO:** PL
- **QID:** Q217599
- **Stores:** ~85 hypermarkets
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** Display name exists (`carrefour-hypermarket-pl`);
  not ingested. T1-anchor scale.

#### E.Leclerc Polska
- **ISO:** PL
- **QID:** Q1273376
- **Stores:** ~50
- **Placement:** ALPHA_HYPERMARKET EU (borderline)
- **Reasoning:** Display name exists (`leclerc-pl`); not ingested.

#### Biedronka
- **ISO:** PL
- **QID:** Q857182
- **Stores:** ~3,700
- **Placement:** REGION_CONFIG secondary (in GENERIC_FOOD already)
- **Reasoning:** PL #1 by store count; hard-discount small-format.
  Already in GENERIC_FOOD. Co-tenant scale, not anchor.

---

### 2.L — GREECE (GR)

**Note:** GR currently has IKEA as sole anchor.

#### Sklavenitis
- **ISO:** GR
- **QID:** Q7536996
- **Stores:** ~600 (acquired Marinopoulos 2017)
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** GR #1 grocer; acquired Carrefour Marinopoulos
  hypermarket fleet. **HIGHEST-PRIORITY GR GAP.**

#### AB Vasilopoulos (Ahold Delhaize)
- **ISO:** GR
- **QID:** Q1631912
- **Stores:** ~520
- **Placement:** REGION_CONFIG secondary
- **Reasoning:** GR #2; smaller-format. Co-tenant scale.

#### Masoutis
- **ISO:** GR
- **QID:** Q6783499
- **Stores:** ~330
- **Placement:** REGION_CONFIG secondary (regional, N. Greece)
- **Reasoning:** N. Greece dominant; smaller format.

#### My Market (Metro AEBE GR)
- **ISO:** GR
- **QID:** Q15211566
- **Stores:** ~220
- **Placement:** REGION_CONFIG secondary

---

### 2.M — PORTUGAL (PT)

**Note:** PT currently has IKEA as sole anchor.

#### Continente (Sonae)
- **ISO:** PT
- **QID:** Q5164541
- **Stores:** ~315 across banners; ~40 Continente hypermarkets
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** PT #1 hypermarket chain (Sonae MC). **HIGHEST-PRIORITY
  PT GAP.**

#### Pingo Doce (Jerónimo Martins)
- **ISO:** PT
- **QID:** Q3905731
- **Stores:** ~480
- **Placement:** ALPHA_HYPERMARKET EU (large-format Pingo Doce stores)
  or REGION_CONFIG secondary
- **Reasoning:** PT #2 grocer; mixed format.

#### Auchan Portugal (formerly Jumbo)
- **ISO:** PT
- **QID:** Q758603
- **Stores:** ~35 hypermarkets + ~150 Mini Preço
- **Placement:** ALPHA_HYPERMARKET EU (Auchan-branded only)
- **Reasoning:** Rebranded from Jumbo 2019; PT #3 hypermarket.

#### El Corte Inglés Portugal
- **ISO:** PT
- **QID:** Q623634
- **Stores:** 2 (Lisbon + Vilamoura)
- **Placement:** Skip (too few)

---

### 2.N — NORDICS (SE/NO/DK/FI/IS)

#### ICA Maxi Stormarknad — already added (display name only)
- **Status:** `maxi-ica-se` in ANCHOR_DISPLAY_NAMES; not in any
  ALPHA_* set.
- **ISO:** SE
- **QID:** Q1645858
- **Stores:** ~85
- **Placement:** ALPHA_HYPERMARKET EU (NORDICS region)
- **Reasoning:** SE #1 hypermarket format. **GAP.**

#### Coop Forum SE
- **Status:** `coop-forum-se` in ANCHOR_DISPLAY_NAMES; not in any
  ALPHA_* set.
- **ISO:** SE
- **QID:** Q1135295
- **Stores:** ~36
- **Placement:** ALPHA_HYPERMARKET EU
- **Reasoning:** SE Coop hypermarket; T1-anchor.

#### Willys (SE hard-discount)
- **ISO:** SE
- **QID:** Q10720922
- **Stores:** ~225
- **Placement:** REGION_CONFIG secondary

#### Meny (NO/DK)
- **ISO:** NO/DK
- **QID:** Q11971894
- **Stores:** ~190
- **Placement:** REGION_CONFIG secondary

#### Rema 1000 (NO/DK)
- **ISO:** NO/DK
- **QID:** Q2071741
- **Stores:** ~1,100
- **Placement:** REGION_CONFIG secondary (hard-discount)

#### Kvickly (Coop DK)
- **ISO:** DK
- **QID:** Q12317551
- **Stores:** ~80
- **Placement:** REGION_CONFIG secondary or ALPHA_HYPERMARKET (mixed
  format)

#### Føtex (Salling Group DK)
- **ISO:** DK
- **QID:** Q3084416
- **Stores:** ~100
- **Placement:** REGION_CONFIG secondary (Bilka covers DK hyper slot)

#### Stockmann
- **ISO:** FI
- **QID:** Q1789814
- **Stores:** 5
- **Placement:** Skip (flagship-only)

#### Magasin du Nord (DK)
- **ISO:** DK
- **QID:** Q1855258
- **Stores:** 6
- **Placement:** Skip (flagship-only; though anchor-tenant role)

#### S Group / S-Market FI
- **ISO:** FI
- **QID:** Q1063279 (S Group); Q11900236 (S-market)
- **Stores:** ~440 S-market
- **Placement:** REGION_CONFIG secondary (K-Citymarket + Prisma
  already cover FI hyper slot)

---

## Section 3 — HARDWARE / DIY GAPS

### 3.A — OBI Germany (and other OBI markets)
- **ISO:** DE / IT / PL / AT / CH
- **QID:** Q310208
- **Stores:** ~350 DE, ~50 IT, ~60 PL, ~75 AT
- **Status:** `obi-de`, `obi-it`, `obi-pl` in GENERIC_HARDWARE but
  flagged in TODO comment as "0 records — need re-ingest with name=
  query".
- **Placement (after re-ingest):** ALPHA_HARDWARE EU for DE, AT
  primarily. **HIGHEST-PRIORITY DE HARDWARE GAP** (Hornbach alone is
  insufficient for DE T1 hardware scoring).
- **Action:** re-ingest using name= tag; promote on confirmation.

### 3.B — Bauhaus DE/ES
- **ISO:** DE / ES / AT / Nordics (covered as bauhaus-se)
- **QID:** Q672660
- **Stores:** ~155 DE, ~6 ES
- **Status:** `bauhaus-de`, `bauhaus-es` in GENERIC_HARDWARE; flagged
  0-records.
- **Placement (after re-ingest):** ALPHA_HARDWARE EU for DE.
- **Action:** re-ingest using name= tag; promote bauhaus-de.

### 3.C — Bricomarché FR
- **ISO:** FR
- **QID:** Q2896882
- **Stores:** ~600
- **Placement:** ALPHA_HARDWARE EU (FR)
- **Reasoning:** Les Mousquetaires DIY format; FR #3 hardware after
  Leroy Merlin / Castorama. Not currently in taxonomy. **HIGH-PRIORITY
  FR GAP.**

### 3.D — Brico Dépôt (FR variant)
- **ISO:** FR (also ES already covered)
- **QID:** Q3007003
- **Stores:** ~125 FR
- **Placement:** ALPHA_HARDWARE EU (FR)
- **Reasoning:** Kingfisher Group warehouse-format DIY. Add as
  `brico-depot-fr`.

### 3.E — Mr. Bricolage (FR)
- **ISO:** FR
- **QID:** Q3320536
- **Stores:** ~660
- **Placement:** REGION_CONFIG secondary (small-format)

### 3.F — Wickes / Screwfix / Toolstation (GB)
- **ISO:** GB
- **QID:** Q7997220 / Q7434515 / Q12063253
- **Stores:** ~230 / ~830 / ~620
- **Placement:** Wickes = ALPHA_HARDWARE EU (GB) — direct B&Q analog.
  Screwfix/Toolstation = REGION_CONFIG secondary (trade-only counter
  format).
- **Reasoning:** Wickes is the GB #2 home-improvement chain after B&Q.
  GB currently has only bq-uk in hardware. **HIGH-PRIORITY GB GAP.**

### 3.G — Bricoman / BricoCenter IT additions
- **ISO:** IT
- **QID:** Q3643470 (Bricoman) / Q3645032 (BricoCenter)
- **Stores:** ~50 Bricoman + ~50 BricoCenter (covered)
- **Placement:** Bricoman = ALPHA_HARDWARE EU. BricoCenter already in
  GENERIC_HARDWARE.

### 3.H — Leroy Merlin DE / GB / NL
- **ISO:** DE / GB / NL
- **Status:** Leroy Merlin exited GB market 2003; not present DE/NL.
  No action.

### 3.I — Hubo / Brico (BE-NL)
- **ISO:** NL
- **QID:** Q2376569 (Hubo)
- **Stores:** ~150 NL
- **Placement:** REGION_CONFIG secondary (NL primary already covered:
  praxis/gamma/karwei)

### 3.J — Bauhaus AT
- **ISO:** AT
- **QID:** Q672660
- **Stores:** ~19
- **Placement:** REGION_CONFIG secondary (Hornbach-AT covers primary)

### 3.K — Leroy Merlin NL
- **ISO:** NL — no presence. Skip.

### 3.L — Praktiker DE
- **ISO:** DE — defunct (2013). Skip.

### 3.M — Bricoking / Würth (specialty/trade)
- **Placement:** Skip (trade-only, not consumer anchor).

---

## Section 4 — WAREHOUSE CLUB GAPS

### 4.A — BC Liquor / SAQ (sub-national, not warehouse)
- N/A — not warehouse club format.

### 4.B — Selgros DE
- **Status:** `selgros-de` in GENERIC_WAREHOUSE.
- **Placement (consider promotion):** ALPHA_WAREHOUSE EU
- **Reasoning:** Already in REGION_CONFIG["DE"]["warehouse"]; promote
  to ALPHA_WAREHOUSE for dual-membership.

### 4.C — Metro Cash & Carry DE / IT
- **Status:** `metro-de`, `metro-it` in GENERIC_WAREHOUSE.
- **Placement (consider promotion):** ALPHA_WAREHOUSE EU
- **Reasoning:** B2B but warehouse-format; same justification as Makro.
  Already in REGION_CONFIG warehouse slots.

### 4.D — Booker / Bestway / Costco Business (GB)
- **Placement:** Trade-only. Skip.

### 4.E — Smart & Final (US)
- **ISO:** US
- **QID:** Q7544454
- **Stores:** ~250 (West Coast)
- **Placement:** REGION_CONFIG secondary (warehouse-grocery hybrid;
  not membership format)

---

## Section 5 — LOW-OSM-COVERAGE FLAGS

Chains already in config.py that the comments flag as having weak OSM
data — these will UNDER-count on current ingest:

| Chain | Current bucket | Flag |
|---|---|---|
| `obi-de` | GENERIC_HARDWARE | "0 records — need re-ingest with name= query" |
| `obi-it` | GENERIC_HARDWARE | "0 records" |
| `obi-pl` | GENERIC_HARDWARE | "0 records" |
| `bauhaus-de` | GENERIC_HARDWARE | "0 records" |
| `bauhaus-es` | GENERIC_HARDWARE | "0 records" |
| `brico-depot-es` | GENERIC_HARDWARE | check wikidata Q3007003 ingest |
| `imerco-dk` | GENERIC_HARDWARE | Nordic specialty; verify |
| `husasmidjan-is` | GENERIC_HARDWARE | Nordic specialty; verify |
| `gamma-nl` | ALPHA_HARDWARE | "pending first ingest" |
| `karwei-nl` | ALPHA_HARDWARE | "pending first ingest" |
| `toom-baumarkt-de` | GENERIC_HARDWARE | "pending first ingest" |
| `hagebaumarkt-de` | GENERIC_HARDWARE | "pending first ingest" |
| `bricocenter-it` | GENERIC_HARDWARE | "pending first ingest" |
| `silvan-dk` | GENERIC_HARDWARE | "pending first ingest" |
| `praktiker-gr` | GENERIC_HARDWARE | "pending first ingest" |
| `byko-is` | GENERIC_HARDWARE | "pending first ingest" |

Additionally — chains with display names but no ingest configured:
- `bodega-aurrera-mx`, `alcampo-es`, `leclerc-es`,
  `carrefour-hypermarket-es`, `carrefour-hypermarket-it`,
  `carrefour-hypermarket-pl`, `auchan-pl`, `leclerc-pl`,
  `ipercoop-it`, `iper-it`, `bennet-it`, `coop-forum-se`,
  `maxi-ica-se`

These are partially configured (display labels declared) but absent
from any algorithm set — easy follow-on ingest wins.

---

## Section 6 — PRIORITY RECOMMENDATIONS

**Tier-1 (largest cluster-count uplift expected):**
1. Esselunga IT (sole-IKEA-anchor country)
2. Albert Heijn XL NL (sole-IKEA+makro country)
3. Continente PT (sole-IKEA-anchor country)
4. Sklavenitis GR (sole-IKEA-anchor country)
5. Billa Plus AT (sole-IKEA-anchor country)
6. ASDA UK + Morrisons UK (large fleet, anchor-class)
7. Alcampo ES + Carrefour-Hyper ES (multiplies ES T1 beyond Mercadona)
8. Auchan-PL + Carrefour-Hyper-PL (multiplies PL T1)
9. Rona CA (replaces defunct Lowe's-CA in hardware)
10. Real Canadian Superstore — add to ALPHA_HYPERMARKET set (currently
    only in REGION_CONFIG — config defect)

**Tier-2 (regional uplift):**
- H-E-B US (TX), Publix US (SE), Meijer US (Midwest), Wegmans US (NE)
- Chedraui MX promotion from Food → Hypermarket
- Globus DE, Rewe Center DE
- Wickes UK (hardware second anchor)
- Bricomarché FR, Brico Dépôt FR (FR hardware breadth)
- OBI DE + Bauhaus DE re-ingest (DE hardware second anchor)
- Conad Superstore IT, Ipercoop IT, Iper IT, Bennet IT (IT breadth)
- ICA Maxi SE, Coop Forum SE (SE hyper breadth)

**Tier-3 (premium / lifestyle additions):**
- El Corte Inglés ES → ALPHA_LIFESTYLE (alongside IKEA)
- Galeries Lafayette FR → ALPHA_LIFESTYLE
- John Lewis GB + Marks & Spencer (full-line) → ALPHA_LIFESTYLE
- Karstadt-Galeria DE → ALPHA_LIFESTYLE (verify current store count)

**Configuration defects (low effort):**
- `real-canadian-superstore-ca` missing from ALPHA_HYPERMARKET set
- `selgros-de`, `metro-de`, `metro-it` candidates for ALPHA_WAREHOUSE
  promotion (already in REGION_CONFIG warehouse slot)
