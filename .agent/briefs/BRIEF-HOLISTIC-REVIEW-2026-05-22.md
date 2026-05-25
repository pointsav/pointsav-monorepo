# BRIEF — Holistic Review: the Regional Market data gap as the mission

> Final research round · 2026-05-22 · feeds a 4-agent holistic cross-check
> (urban land · CRE/CBRE · GIS · economics).
> After this round the master `BRIEF-BUILD-SPEC-2026-05-22.md` is complete and
> the build begins.

---

## 1. The reframe — the gap *is* the mission

The Regional Markets study reached a striking conclusion: **there is no global,
public, sub-metro data repository for Regional Markets.** CBRE Econometric
Advisors and Oxford Economics cover **metros** — and even that is proprietary,
paid, US-centric, and metro-grained. Below the metro:
- CBRE EA submarkets are CoStar-licensed, US-only for retail, broker-drawn
  judgement geography — not a real, global boundary dataset.
- Oxford Economics has **no sub-metro geography at all**.
- No global sub-city administrative layer exists either (OSM wards are
  per-mapper, absent for most US/CA cities).

**That gap is exactly what Woodfine Capital Projects / Woodfine Management Corp.
is building gis.woodfinegroup.com + projects.woodfinegroup.com (the wiki) to
fill.** The mission: a **public repository of data and information about
Regional Markets** — so retailers can evaluate and expand into these markets,
and so the Regional Markets *themselves* have authoritative data about
themselves. The platform is the systematic, global, public answer to a gap the
incumbents leave open.

## 2. The ask — a holistic cross-check

Each agent takes a **holistic view** of the whole design and cross-checks it
against the mission above. Read `BRIEF-BUILD-SPEC-2026-05-22.md` (the master
spec — categories, tiers, distance/demand ranking, co-location, Regional Market,
Metro Market). The question for every agent:

**Does the design — categories · tiers · ranking · co-location · Regional Market
— actually serve the mission of being the authoritative public repository for
Regional Market data? What is missing, mis-framed, or over-built for that goal?**

This is a *cross-check*, not a re-litigation. The settled decisions stand unless
an agent finds a genuine mission-level flaw. Surface gaps, blind spots, and
anything the four prior briefs optimised in isolation that does not add up to a
coherent public data product.

## 3. The four lenses

- **Urban land** — land development / investment. Does the co-location +
  Regional Market frame inform real land and development decisions?
- **CRE / CBRE** — retail expansion. Does it serve a retailer choosing where to
  expand, the way CBRE/Oxford serve their clients — and better, at Regional grain?
- **GIS** — data integrity. Does the geospatial model support an *authoritative*
  public repository — coverage, reproducibility, the open-data posture?
- **Economics** — regional/urban economics. Are Regional Markets the right
  economic unit? Do the tiers and rankings reflect economic reality (retail
  gravity, agglomeration, market potential)? What would Oxford-Economics-grade
  rigour want from this dataset?

## 4. After this round

The master brief is complete and the build begins. **The build sequence starts
by completing the editorial and design artifacts** — the `TOPIC-*`, `GUIDE-*`,
and other artifacts routed to the other `project-*` repos (project-editorial,
project-design, …) — because the wiki / public repository is the product, not a
downstream afterthought.

---

## 5. Findings — 4-agent holistic cross-check (2026-05-22)

**Unanimous across urban-land · CRE · GIS · economics: the geometric build is
sound — freeze it, do not re-open it.** Every settled decision stands (6
categories, 3 tiers, two-pass DBSCAN, centroid geometry, `span_km`, municipal
Regional Market, override deletion, district self-clustering).

**The shared finding:** the design measures **geometry + supply** — where
big-box retail *already* agglomerated, a lagging indicator — while the mission
(the authoritative public Regional Market repository) needs **geometry +
supply + demand-side economics + provenance.** The fix is **additive**, not a
redesign, and does not block the geometric build.

- **CRE** — a discovery/shortlisting tool, not yet a siting tool: silent on
  saturation and cost. Demand with no supply denominator is half a metric.
  Tier ≠ market grade; co-location count = saturation as much as opportunity.
- **Urban-land** — co-location is a lagging indicator; the design will rank
  *closed* (built-out) markets above *open* (emergent) ones. The RM record needs
  population + CAGR + built-up-area share.
- **GIS** — the model is reproducible; the *inputs* are not. Needs an
  input-provenance layer + a published coverage statement. "Authoritative" =
  most transparent, not broadest.
- **Economics** — the municipality is the right *publishing* unit, wrong
  *analytical* unit. The design omits first-order economics — income, spend
  potential, growth, and **anchor-supply-per-capita saturation**.

**Actioned in `BRIEF-BUILD-SPEC-2026-05-22.md` §7** (economic-indicator block ·
framing discipline · input-provenance layer) **and §8** (artifact-first build
start). The master brief is now COMPLETE.
