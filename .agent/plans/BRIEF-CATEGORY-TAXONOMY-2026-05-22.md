# BRIEF — Co-location Category Taxonomy

> Small research brief · 2026-05-22 · feeds a 3-agent study (GIS · CBRE · coding).
> Companion: `BRIEF-VARIABLE-DISTANCE-2026-05-21.md` — that brief sets
> *tier = retailer-category composition*; **this brief defines the composition.**

---

## 1. Purpose

Tiers are built from retailer-**category composition**. This brief defines the
**categories themselves** — the building blocks. Category membership is purely
**geometric co-location** of the category's anchors; **no ranking, no demand**
in the category definition.

## 2. The premise

Investors and retailers want to **co-locate with the international Fortune-500
anchor retailers** — Walmart, IKEA, Home Depot, Costco. A co-location's tier is
set by which category anchors are present. Europe is nationally fragmented —
the same categories must be filled by **national-champion brands** per market.

## 3. The category set (operator draft — to validate / refine)

**Retail anchor categories:**
- **Hypermarket** — big-format general-merchandise + grocery (Walmart, Real
  Canadian Superstore; EU: Carrefour, Tesco, …). **Not department stores.**
- **Hardware / home improvement** — Home Depot, Lowe's; EU: Leroy Merlin, B&Q,
  Bauhaus, …
- **Price club / warehouse club** — Costco; EU equivalents are limited.
- **Furniture / lifestyle** — must **include IKEA** but **exclude department
  stores**. Open: its own category, or folded into a broadly-named anchor
  category? Operator wants "a name that includes IKEA — just IKEA, not
  department stores."

**Civic anchor categories:**
- **Medical** — regional **hospitals**, not clinics.
- **Education** — real colleges / **universities**, not schools or training centres.

## 4. Requirements

- **One unified category system that works identically for North America and
  Europe** — same categories, different brand fills.
- For each category, in each region, pick the **one or two flagship brands**
  that "elevate" / define the category — the anchors investors want to be near.
- Civic categories need a **size / scale filter**: regional hospital not clinic;
  university not school.
- Categories are **composition only — geometric. No ranking.**

## 5. Research questions

1. Is the category set right? Validate hypermarket / hardware / price club /
   furniture-lifestyle / medical / education — right cuts, right names?
2. Where does **IKEA** sit — its own category vs a broad anchor category — and
   how to **exclude department stores** cleanly?
3. The international Fortune-500 anchor set **+ the European national-champion
   equivalents**, per category, per major EU market.
4. **Medical** — criteria distinguishing a regional hospital from a clinic.
5. **Education** — criteria distinguishing a university/college from a
   school / training centre.
6. How to **encode** the taxonomy (category definitions + per-region brand
   lists + civic size-filter) and how it cross-checks against the chains the
   platform has already ingested.

---

## 6. Findings & recommended taxonomy — 3-agent study, 2026-05-22

### The category set — **6 categories, 4 retail + 2 civic** (unanimous)

| Key | Category | NA flagship(s) | EU national champions |
|---|---|---|---|
| `hypermarket` | Hypermarket (general-merch + grocery) | Walmart | Tesco · Carrefour · E.Leclerc · Mercadona · Kaufland · Esselunga · … |
| `hardware` | Hardware / Home Improvement | Home Depot · Lowe's | B&Q · Leroy Merlin · Hornbach · Bauhaus · … |
| `price_club` | Price Club / Warehouse Club | Costco | Costco (UK/ES/FR/SE/IS only — EU-thin; treat absence as normal) |
| `lifestyle` | Lifestyle Anchor (IKEA) | IKEA | IKEA — identical brand, all markets |
| `medical` | Medical — regional hospital | OSM, scale-filtered | OSM / national registers |
| `education` | Education — university | OSM / IPEDS | OSM / ETER / HESA |

No more categories — apparel/electronics/pharmacy are co-tenants, not anchors,
and would dilute the tier signal. Tier rule (composition only, geometric):
**T1 = hypermarket ∧ hardware ∧ (price_club ∨ lifestyle); T2 = hypermarket ∧
one other; T3 = ≥2 retail categories** — matches the sim's `evaluate_tier_new`.

### IKEA — its own category, allowlist-defined
`lifestyle` is a category of one. Department-store exclusion is **structural**:
a department store is simply never added to the brand list. Self-maintaining.

### The core structural fix — one declarative taxonomy
The taxonomy currently lives in **three disagreeing places**: `config.py`
(`ALPHA_*` sets), `build-tiles.py` (`CHAIN_FAMILY`, 7 families), and
`simulate-dbscan-ab.py` (`CHAIN_META`). Live drift bugs: `sklavenitis-gr` is an
anchor in config but tiled as "Food"; `real-canadian-superstore-ca` is in
`REGION_CONFIG` but absent from every `ALPHA_*` set.
→ Replace with one **`taxonomy.py`**: `CATEGORIES` (6, region-agnostic) +
`BRAND_FILL` (region-keyed brand lists, flagship = first 1–2 entries) +
`category_of()` + a shared `tier_of()`. The sim imports it; `config.py` keeps
only paths/constants. Category enum is region-agnostic — region rides on the
brand record, so the tier predicate is brand-blind: one taxonomy, two regions.

### Civic scale filter
- **Regional hospital** = acute in-patient + 24/7 ED + bed-count gate + campus.
  The current `ingest-osm-civic.py` checks the *name* before bed count
  (inverted) — fix to **attribute-first**: bed count is the primary gate.
- **University** = degree-granting + enrolment gate + campus. EU has no
  enrolment source — wire **ETER** (pan-EU register) + **HESA** (UK); US has IPEDS.
- Keep the filter at ingest (fail-closed); `taxonomy.py` declares the thresholds.

### Open operator decisions
1. **Hypermarket purity** — Target / Whole Foods / Wegmans / Sprouts / WinCo are
   currently classed as hypermarket anchors but are department-store / premium-
   grocery formats, not Walmart-class general-merchandise hypermarkets. Keep /
   demote to non-flagship / drop?
2. **Civic in the tier gate, or descriptor only?** Recommended: civic in the
   descriptor + present-set, but `tier_of()` gates on **retail categories only**
   (tier = retail composition; civic feeds the Stage-2 demand rank).
3. **Bed-count threshold** for "regional hospital" — 100 (current code) vs 150 (CBRE).
4. Generic grocery (Lidl/Aldi) has no category → invisible to the tier model — confirm OK.
5. B2B cash-and-carry (Metro/Selgros) excluded from `price_club` — recommended.
6. Category display names — CBRE suggests "General Merchandise & Grocery Anchor"
   / "Big-Box Lifestyle Anchor"; operator's call.

### Implementation
- **S0:** create `taxonomy.py`; retarget `simulate-dbscan-ab.py` to import it;
  `cross-check-taxonomy.py` audit script. No overnight build.
- **S2:** strip `ALPHA_*` / `REGION_CONFIG` from `config.py`; rewire
  `build-clusters.py` + `build-geometric-ranking.py` (also executes the
  variable-distance brief §8.1 demand-gate strip); fix the civic filter;
  overnight rebuild.

---

## 7. Round-2 — per-country modeling & data-driven cleanup (operator, 2026-05-22)

The old worry — too few Regional/T1 hits — is considered **solved** by the
geometric variable-distance model. The taxonomy work now shifts to **empirical,
data-driven cleanup**: there is finally enough ingested data to model it.

**Guiding principle:** find the **countries that genuinely have** these
categories — do **not** dilute a category's brand list to make a country
qualify. Categories are fixed; country coverage is *discovered*.

- **7a — Brand fills by COUNTRY, not by NA/EU continent.** `BRAND_FILL[category][country]`.
- **7b — Explicit country display list.** Define which countries the platform
  displays — North America, UK, the Nordics, and the rest of Europe called out
  distinctly.
- **7c — Per-country slot modeling.** For each (country × category) pick the
  brand(s). Where the "pure" international flagship is absent, a credible
  substitute is **promoted into the slot**. Model **two options per country per
  category** (#1 and #2). Do it empirically — store counts, co-location
  representation.
- **7d — Civic out of the geometry.** Civic-in-tier "did not work before" —
  keep Medical/Education out of the geometric tier (informational only). Confirm
  with analysis.
- **7e — Scale thresholds follow an external standard.** The regional-hospital
  bed threshold (and scale filters generally) should follow a CBRE call, an
  international standard, **or the SafeGraph data schema** — and the platform
  should align to the **SafeGraph data schema** generally.
- **7f — Non-anchor retailers (Lidl/Aldi etc.) — display or not?** The EU map is
  now over-dotted. Decide whether to show retailers that fill no anchor category.
- **7g — Deliverable:** **simulations on the real ingested data** (a
  data-science exercise) → the country display list, the per-country×category
  brand model (with #2 substitutes), the civic analysis, and the display-cleanup
  recommendation.

---

## 8. Round-2 findings — census + per-country model, 2026-05-22

*(The data-science agent was environment-blocked from Bash; the Totebox ran the
census directly. CBRE + coding agents completed normally.)*

### Country × category coverage — empirical (150 chains, 72,239 stores)

Anchor categories genuinely present — hypermarket / hardware / price_club / lifestyle:

| Coverage | Countries |
|---|---|
| **4 / 4** | US · CA · MX · FR · GB · ES |
| **3 / 4** (no price club) | DE · PL · IT · NL · PT · GR · AT · SE |
| **2 / 4** | FI · DK · NO (hyper+hardware) · IS (hardware+price-club) |
| token only | the `ikea-nordics` blob · UY |

**Price club is the scarce category** — genuine presence only US/CA/MX/GB; FR(7)
ES(6) SE(2) IS(1) are token. Confirms: EU price-club absence is the *normal* state.

### The data is already SafeGraph-schema
Records carry `placekey`, `naics_code`, `top_category`, `sub_category`, `brands`
— ~85 % SafeGraph-aligned. The 6 categories map to NAICS leaf codes
(445110 / 452311 / 444110 / 442110 / 622110 / 611310). Make `naics_code` the
category key (sourced from `taxonomy.py`, not the chain YAML); add
`parent_placekey`, `opened_on`, `closed_on`. Caveat — Walmart and Costco are
both NAICS 452311; the platform's hypermarket-vs-price-club cut is intentionally
*finer* than NAICS, so the 6-value enum stays the tier discriminator and NAICS
is carried for export/audit.

### Per-country BRAND_FILL — #1/#2 slot model
`BRAND_FILL[category][ISO-country]` → ordered slots: slot 1 = flagship, slot 2 =
substitute. A `slots_for()` resolver yields *flagship / substitute-promoted /
category-absent*. An empty list = category genuinely absent — never diluted.
CBRE delivered the full per-country #1/#2 table (US/CA/MX, UK, Nordics, and
FR/DE/ES/IT/NL/PT/AT/GR/PL individually) — retained in the session transcript.

### Decided / recommended
- **Civic stays out of the geometric tier** — confirmed (CBRE + operator); it
  destabilised tiers before. Descriptor / demand-rank only.
- **Non-anchor retailers off the default map** — Lidl/Aldi/discount grocery fill
  no anchor category; the census shows ~25,000+ such dots burying ~350 anchor
  dots. Default layer = anchors only + an optional co-tenant toggle.
- **Regional-hospital bed threshold = 150** — CBRE and coding converge
  (WHO/OECD regional-referral-hospital convention); a named constant in `taxonomy.py`.
- **Country display list** — Tier A (full multi-category story, 12): US · CA ·
  MX · GB · FR · DE · ES · IT · PL · NL · FI · SE. Tier B (single-category, 5):
  PT · AT · GR · DK · NO. IS marginal. UI groups: NA / UK / Nordics / Continental EU.

### Open operator decision — hypermarket purity
Whole Foods / Wegmans / WinCo / Sprouts / Fred Meyer are premium/regional
grocery (NAICS 445110), not Walmart-class general-merchandise hypermarket
anchors. CBRE: demote them out of the flagship slot — US hypermarket flagship =
Walmart, with Target as #2. Recommended; operator to confirm.

### Implementation — all S0
Create `taxonomy.py` (`CATEGORIES` + per-country `BRAND_FILL` + `THRESHOLDS` +
`DISPLAY_COUNTRIES`); retarget `simulate-dbscan-ab.py` to import it; add
`cross-check-taxonomy.py`. S2 = production rewire + civic-filter fix + re-ingest.

---

## 9. Round-2b — operator decisions, 2026-05-22

### Hypermarket purity — DECIDED
Whole Foods / Wegmans / WinCo / Sprouts / Fred Meyer are **demoted** out of the
hypermarket anchor category (premium/regional grocery, not Walmart-class).
**US hypermarket flagship = Walmart (slot 1), Target = slot 2.** The demoted
grocers move to the non-anchor co-tenant layer.

### Price-club data check (Totebox census, 2026-05-22)
True **consumer warehouse club** (NAICS 452910): US (Costco 739 / Sam's 640 /
BJ's 232), MX (Sam's 254 / Costco 94), CA (Costco 109) — strong; GB (Costco 32)
modest; FR 7 · ES 6 · SE 2 · IS 1 — token. **The only warehouse-format
alternative elsewhere is B2B cash-and-carry** (NAICS 424410): DE (Metro 67 /
Selgros 32), IT (Metro 53), ES (Makro 40), PL (Makro 33 / Selgros 13), NL
(Makro 17), GR (The-Mart 11). **AT · PT · FI · DK · NO have neither.**
→ Open question for the data-science agent: use cash-and-carry as the *internal*
price-club #2 substitute where no consumer club exists, or leave the slot empty?

### Internal region grouping (operator) — NOT a map construct
Internally the taxonomy groups countries into **four regions** (used for config
organisation and ranking pools, not map display):
- **North America** — US · CA · MX
- **United Kingdom** — GB
- **Nordics** — SE · DK · NO · FI · IS
- **Continental Europe** — FR · DE · ES · IT · NL · PT · AT · GR · PL

`BRAND_FILL` keys stay per-country; the 4-region grouping is the parent layer.

---

## 10. Round-2b findings — data-science per-country model, 2026-05-22

Census-driven (`work/taxonomy-census-2026-05-22.md`). #1 = flagship, #2 = substitute; "—" = no credible anchor (explicitly empty, never diluted).

### Per-country BRAND_FILL slot model

| Country | hypermarket #1 / #2 | hardware #1 / #2 | price_club | lifestyle |
|---|---|---|---|---|
| **US** | Walmart / Target | Home Depot / Lowe's | Costco / Sam's Club | IKEA / — |
| **CA** | Walmart / Real Canadian Superstore | Canadian Tire / Home Depot | Costco / — | IKEA / — |
| **MX** | Bodega Aurrera / Soriana *(Walmart MX #3)* | Home Depot / — | Sam's Club / Costco | IKEA / — |
| **GB** | Asda / Morrisons | B&Q / — | Costco / — | IKEA / — |
| **FR** | E.Leclerc / Carrefour Hypermarket | Leroy Merlin / Castorama | — | IKEA / — |
| **DE** | Kaufland / Marktkauf | OBI / Hagebaumarkt | — | IKEA / — |
| **ES** | Carrefour Hypermarket / Alcampo | Leroy Merlin / Brico Dépôt | — | IKEA / — |
| **IT** | Esselunga / Carrefour Hypermarket | OBI / Leroy Merlin | — | IKEA / — |
| **PL** | Auchan / Carrefour Hypermarket | OBI / Castorama | — | IKEA / — |
| **NL** | Albert Heijn XL / — | Gamma / Praxis | — | IKEA / — |
| **AT** | Billa Plus / — | Hornbach / — | — | IKEA / — |
| **GR** | Sklavenitis / — | Praktiker / Leroy Merlin | — | IKEA / — |
| **PT** | Continente / — | Leroy Merlin / — | — | IKEA / — |
| **SE** | Maxi ICA / — | Clas Ohlson / Bauhaus | — | IKEA *(blob)* |
| **FI** | K-Citymarket / Prisma | K-Rauta / — | — | IKEA *(blob)* |
| **DK** | Bilka / — | Imerco / Silvan | — | IKEA *(blob)* |
| **NO** | Obs (Coop) / — | Obs Bygg / — | — | IKEA *(blob)* |

Standard supermarkets (Tesco, Carrefour-FR, Mercadona, Biedronka) and discount
grocery (Lidl, Aldi) are **not** promoted into hypermarket slots — different format.

### Price club — DECIDED: empty outside US / CA / MX / GB
B2B cash-and-carry (Metro / Makro / Selgros / The-Mart) is **not** promoted into
the `price_club` anchor slot — NAICS 424410 merchant wholesaler, business-
membership customer; promoting it would dilute the geometric tier. Genuine
`price_club`: US, CA, MX, GB only. Cash-and-carry surfaces as a *flagged
co-tenant* on the optional toggle, never as an anchor.

### Country display list — 17 countries
NA: US·CA·MX. UK: GB. Continental Europe: FR·ES (4/4), DE·IT·PL·NL·AT·GR·PT
(3/4). Nordics: SE (3/4), FI·DK·NO (2/4, marginal). **IS flagged marginal**
(hardware only credible). **UY excluded** (0/4).

### Map clutter — non-anchors ≈ 65 % of dots
~46,000 of 72,239 stores are non-anchor (supermarket / discount / cash-carry /
department). Default map = anchor categories only (~26,000 dots); optional
co-tenant toggle for the rest.

### Pre-freeze data-quality fixes
1. **`ikea-nordics` blob (42 stores)** — split per-country (SE/DK/NO/FI); it is
   why all four Nordic lifestyle slots read empty. A data artifact, not a country.
2. **`lowes-ca` = 1** — stale (Lowe's exited Canada → Rona); drop.
3. **`coop-forum-se` = 1** — severe under-ingest; re-ingest before trusting the
   SE hypermarket #2 slot.

---

## 11. Tier projection — 2026-05-22

Global two-pass tight-first DBSCAN over **28,048 anchor stores** (post-demotion,
new taxonomy), composition tier rule `T1 = hypermarket ∧ hardware ∧ (price_club
∨ lifestyle)`. Script: `work/tier-projection.py`.

| Tier | Count |
|---|---|
| **T1 — Regional** | **1,044** |
| T2 — District | 3,609 |
| T3 — Local | 1,421 |
| **Total co-locations** | **6,074** |

- **T1 = 1,044 — squarely inside the operator's 800–1,200 expectation.** The old
  "too few Regional hits" worry is resolved by the geometric model + corrected
  taxonomy. US alone = 814 T1 / 2,985 total — the old "500 US T1" target is
  comfortably exceeded.
- The rule yields **three tiers** (T1/T2/T3); singletons are dropped as
  non-co-locations. There is **no T4** unless a Fringe tier is explicitly defined.
- Caveat: the `ikea-nordics` blob is unsplit, so Nordic T1 is slightly understated.

| Country | T1 | T2 | T3 | total |
|---|---|---|---|---|
| US | 814 | 1,777 | 394 | 2,985 |
| DE | 19 | 519 | 85 | 623 |
| MX | 65 | 127 | 338 | 530 |
| CA | 64 | 286 | 56 | 406 |
| GB | 11 | 243 | 109 | 363 |
| FR | 21 | 158 | 102 | 281 |
| ES | 17 | 106 | 99 | 222 |
| PL · NL · IT | 10 / 1 / 8 | 108 / 31 / 62 | 13 / 96 / 51 | 131 / 128 / 121 |
| FI · NO · SE · DK | 1 / 3 / 1 / 1 | 66 / 47 / 18 / 11 | 13 / 0 / 22 / 28 | 80 / 50 / 41 / 40 |
| PT · GR · AT · IS | 2 / 2 / 4 / 0 | 20 / 17 / 12 / 0 | 6 / 1 / 1 / 5 | 28 / 20 / 17 / 5 |

---

## 12. Open items & implementation roadmap

The category-taxonomy research is **complete** — three multi-agent rounds
(GIS · CBRE · coding · data-science) plus the Totebox census and the tier
projection (§11). Remaining before `taxonomy.py` is frozen:

### Open operator decision
- **T4 / Fringe tier** — the composition rule (§11) yields **three tiers**
  (T1/T2/T3); singletons drop out as non-co-locations. Either **define a T4
  Fringe** (e.g. a single-anchor-category cluster, or a co-location that just
  misses T3) **or confirm the platform is three-tier.**

### Pre-freeze data-quality fixes (from §10)
1. Split the `ikea-nordics` blob (42 stores) into SE/DK/NO/FI by coordinate.
2. Drop `lowes-ca` (1 stale store — Lowe's exited Canada).
3. Re-ingest `coop-forum-se` (1 store — severe under-ingest).

### Implementation roadmap
- **S0 (build now, no overnight job):** create `taxonomy.py` — `CATEGORIES` (6,
  NAICS-keyed) + per-country `BRAND_FILL` (§10 slot table) + `THRESHOLDS`
  (hospital beds = 150) + `DISPLAY_COUNTRIES` (17, 4-region grouping §9);
  retarget `simulate-dbscan-ab.py` to import it; add `cross-check-taxonomy.py`.
- **S2 (overnight rebuild):** strip `ALPHA_*` / `REGION_CONFIG` from `config.py`;
  rewire `build-clusters.py` + `build-geometric-ranking.py` (also executes the
  variable-distance brief §8.1 demand-gate strip); fix the civic filter
  (attribute-first, NAICS-gated); add SafeGraph fields (`parent_placekey`,
  `opened_on`, `closed_on`); full multi-country rebuild.

### Artifacts
- Census: `pointsav-monorepo/app-orchestration-gis/work/taxonomy-census-2026-05-22.md`
- Tier projection script: `pointsav-monorepo/app-orchestration-gis/work/tier-projection.py`
