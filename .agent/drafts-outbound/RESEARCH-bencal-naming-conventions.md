---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-orgcharts
language_protocol: DESIGN-RESEARCH
authored: 2026-05-29T00:00:00Z
authored_by: totebox-project-orgcharts / claude-sonnet-4-6
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: sub-agent
research_inline: true
---

# RESEARCH — Bencal Corporate Naming Convention

Research basis: 27 firms benchmarked across North America and Europe. Source documents read:
org chart `INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_Organization.html` and tear sheet
`INVESTOR RELATIONS_Woodfine LPs_2026_01_06_Tear Sheet_Alternatives Investments_Woodfine_FIN.docx`.

---

## 1. Current Structure

| Node | Current name | Code | Role |
|------|-------------|------|------|
| 100 | Kiel Capital Inc. | BCL-CA-01-HLD | Personal holding company |
| 101 | Elzen Holdings Inc. | — | Personal holding company |
| 95 | Bencal Asset Management Inc. | BCL-CA-01-OPR | Manager / Promoter |
| 95b | Bencal Asset Management Inc. *(duplicate)* | BCL-CA-01-OPR | Manager — second instance |
| 96 | Bencal Special Purpose 1 Inc. | BCL-CA-01-AST | SPV-GP (SPV 1) |
| 97 | Bencal Special Purpose 2 Inc. | BCL-CA-02-ADM | SPV-GP (SPV 2) |
| 98 | Bencal Special Purpose 2 Limited Partnership | BCL-CA-02-AST | SPV-LP (SPV 2) |
| 102 | WCP Allocation and Sales Agreement | — | Common Shares agreement |
| 103 | Bencal Special Purpose • Inc. | BCL-CA-0•-AST | Future SPV placeholder |

**Code collision:** nodes 95 and 103-upper both carry `BCL-CA-01-OPR`. Addressed in §5.

---

## 2. Four Naming Deliverables

### A. Parent Company Name

The dominant 2026 pattern among institutional alternatives firms is **coined brand + minimal legal suffix, no descriptor**. Brookfield dropped "Asset Management" from its parent in 2022. Blackstone, KKR, TPG, and Apollo converted to plain `Inc.` between 2018–2022. ICG formally shed "Intermediate Capital Group" in favour of `ICG plc` at its July 2025 AGM.

| Rank | Candidate | Rationale | 5-year durability |
|------|-----------|-----------|------------------|
| 1 | **Bencal Corporation** | Mirrors Brookfield Corporation post-2022; signals balance-sheet substance, not "fund manager only"; cleanest institutional signal in 2026; works in BC as `Ltd.` | Excellent — "Corporation" has no semantic content to age |
| 2 | **Bencal Inc.** | Mirrors Blackstone Inc., TPG Inc.; maximally stripped-down; strongest if Bencal is a pure investment manager rather than a diversified HoldCo | Excellent — no descriptor to retire |
| 3 | **Bencal Group Inc.** | Mirrors Carlyle Group; traditional institutional register; "Group" is holding but durable | Good — "Group" is slightly legacy but not stale |

**Not recommended at parent tier:** Bencal Asset Management Inc., Bencal Capital Inc., Bencal Partners Inc., Bencal Holdings Inc. (acceptable only at intermediate tier), Bencal Alternatives.

---

### B. Operating / Management Subsidiary

This entity will hold the exempt-market dealer or portfolio-manager licence. The name must register as a fiduciary investment manager, not a brokerage or bank.

| Rank | Candidate | Rationale |
|------|-----------|-----------|
| 1 | **Bencal Investment Counsel Inc.** | OSC-recognized category; distinctively Canadian (Hamblin Watsa, Mawer Investment Management, Leith Wheeler Investment Counsel); strongest fiduciary signal; differentiates from US-pattern "Management" |
| 2 | **Bencal Investment Management Inc.** | Most institutional, full-spectrum; better if cross-border AUM is material and US LPs are priority; mirrors Hamilton Lane, Neuberger Berman Advisers |

**Note on the two-tier model:** Following the Brookfield pattern, the parent (`Bencal Corporation`) holds the operating manager (`Bencal Investment Counsel Inc.`), which in turn acts as GP and promoter for all vehicle series. "Asset Management" remains acceptable at the Tier 2 level even after being retired at Tier 1.

---

### C. SPV + Club Deal Naming Convention

#### Structural distinction

| Type | Investor count | Characteristics |
|------|---------------|-----------------|
| Private SPV | 1–3 investors | Bespoke, single-asset or single-financing; typically one lead investor |
| Club Deal | 10–300 investors | Subscription-agreement based; Bencal acts as promoter; similar mechanics to a private fund without full fund-registration |

#### Recommendation: Category-split, Arabic numerals

All 27 benchmarked firms organize vehicles into **named strategy families**, each with its own sequential numbering. A single sequential stream (Bencal Fund 1, 2, 3…) is absent from institutional practice and signals an early-stage single-strategy manager.

**Strategy categories for Bencal:**

| Category | Scope |
|----------|-------|
| Credit | Debt, mezzanine, income-generating structures |
| Real Assets | Real estate, infrastructure, tangible-asset direct deals |
| Equity | Growth equity, co-investment, direct equity |

**Arabic vs. Roman numerals:** Arabic numerals are preferred for Club Deals (Warburg Pincus Global Growth 14 model). Roman numerals are acceptable for named-strategy series but feel increasingly legacy. Recommendation: Arabic.

#### Club Deal naming pattern

**GP corp (one per strategy, standing entity):**
```
Bencal Credit Partners Inc.
Bencal Real Assets Partners Inc.
Bencal Equity Partners Inc.
```

**Club Deal LP (per vehicle):**
```
Bencal Credit Club 1 LP
Bencal Real Assets Club 1 LP
Bencal Equity Club 1 LP
Bencal Real Assets Club 2 LP
```

#### Private SPV naming pattern — PSP place-name approach

PSP Investments names private SPVs after Canadian geographical names — the strategy/vintage/sector is completely absent from the LP name. The GP corp carries the Bencal brand; the LP carries a place name.

**GP corp (standing, one for all private SPVs):**
```
Bencal Private Investments Inc.
```

**Private SPV LP (per deal, no Bencal mark):**
```
Sturgeon Bay Investments LP
Bella Coola Investments LP
Kicking Horse Investments LP
Sooke Investments LP
Clearwater Investments LP
```

This pattern is more durable than extending the fund-series name because:
- Discloses nothing about strategy, vintage, or sector
- Never requires a version number
- Is impossible for a competitor to predict or replicate
- Works equally well for credit, real estate, or equity

Maintain a curated register of 20–30 BC/Canadian geographical names for ongoing use.

#### Access / perpetual vehicle (if Bencal distributes to private wealth channel)
```
Bencal Private Credit Fund
Bencal Real Assets Fund
```
No number; designed for ongoing subscription like BREIT / BCRED model.

---

### D. Securities Distribution Entity

This entity registers as an exempt-market dealer (EMD) under NI 31-103 or a restricted dealer, and distributes Bencal's club deal subscriptions.

| Rank | Candidate | Rationale |
|------|-----------|-----------|
| 1 | **Bencal Securities Inc.** | Standard Canadian EMD naming pattern; clean, mirrors Canaccord Genuity Securities, Raymond James Ltd.; regulators recognize it immediately |
| 2 | **Bencal Capital Markets Inc.** | Broader scope; works pre- and post-dealer registration; suitable if Bencal anticipates moving beyond EMD to full investment dealer over time |

**Not recommended:** Bencal Private Wealth Inc. (retail-brokerage signal), Bencal Distribution Inc. (product-company register), Bencal Placement Inc. (European idiom, unfamiliar to OSC).

---

## 3. Words to Avoid

| Word / Phrase | Why to avoid | Preferred alternative |
|---|---|---|
| **Alternative / Alternatives** | Zero benchmarked firms use it in entity names; magazine-section vocabulary; Apollo uses "Aligned Alternatives" only at fund level | Omit; use strategy descriptors (Credit, Real Assets) |
| **Capital** (parent tier) | Saturated; elite firms (Partners Group, Ardian, Onex, Fairfax) deliberately avoid at topco | Acceptable at Tier 2 operating entity or fund name only |
| **Private Wealth / Wealth Management** | Retail-brokerage signal; wrong register for institutional GP/PM; pigeonholes the firm | Use "Securities" or "Capital Markets" for distribution entity |
| **Asset Management** (parent tier) | Brookfield deleted it from parent in 2022; ICG shed "Intermediate Capital Group" in 2025; trend is accelerating | Retain only at Tier 2 operating manager if needed |
| **Holdings** (parent tier) | Survives at intermediate tier (Blackstone Holdings, Brookfield Oaktree Holdings); reads as internal entity, not brand-forward parent | Use "Corporation" or "Inc." at parent; reserve "Holdings" for intermediate holding vehicles |
| **Global** | Ubiquitous to the point of invisibility; obligatory rather than differentiating | Omit; geography is implied by investor base |
| **Advisors / Advisory** | Mutual-fund / RIA / family-office register; signals smaller AUM | Use "Counsel" (Canadian fiduciary) or "Management" |
| **Ventures** | Venture capital flavor; wrong register for real-assets and credit strategies | Omit entirely |
| **Trust** | Canadian regulatory protection; cannot be used without trust-company licence | Omit |
| **Fund** (in entity name) | Implies regulatory registration; use only for registered vehicles or access funds | Use "Club", "Partners", or place-name for unregistered SPVs |

---

## 4. Company Code Convention

**Current format:** `BCL-CA-NN-TTT`
- `BCL` = Bencal brand abbreviation
- `CA` = country code (ISO 2-letter)
- `NN` = two-digit entity sequence number
- `TTT` = role type (HLD / OPR / ADM / AST / AGT / SEC)

**Current collision:** both `BCL-CA-01-OPR` codes are assigned to Node 95 (Asset Management Inc.) and to the future operating company (Node 103-upper / Bencal Operating Corp.). Recommend:

| Entity | Proposed code | Notes |
|--------|--------------|-------|
| Kiel Capital Inc. | `BCL-CA-01-HLD` | Unchanged |
| Elzen Holdings Inc. | `BCL-CA-02-HLD` | Assign code |
| Bencal Corporation (new parent) | `BCL-CA-01-PAR` | New type: PAR = parent |
| Bencal Investment Counsel Inc. | `BCL-CA-01-OPR` | Manager / Promoter |
| Future operating company | `BCL-CA-02-OPR` | Separate sequence slot |
| Bencal Private Investments Inc. | `BCL-CA-01-GPR` | GPR = GP (private SPVs) |
| Bencal Credit Partners Inc. | `BCL-CA-02-GPR` | GP (Credit club deals) |
| Bencal Real Assets Partners Inc. | `BCL-CA-03-GPR` | GP (Real Assets club deals) |
| Bencal Securities Inc. | `BCL-CA-01-SEC` | EMD / distribution |
| Club Deal LP (per vehicle) | `BCL-CA-NN-AST` | Sequential per LP |

**On the BPC abbreviation:** if the parent renames to `Bencal Partners Corporation` or similar, the prefix migrates to `BPC`. All existing `BCL` codes remain valid for existing entities; new entities use `BPC`. A one-time code migration is not required.

---

## 5. Recommended Full Corporate Stack

```
Kiel Capital Inc. (BCL-CA-01-HLD)          ← Personal holding — no change
Elzen Holdings Inc. (BCL-CA-02-HLD)         ← Personal holding — no change
    │
    ▼
Bencal Corporation (BCL-CA-01-PAR)          ← Tier 1: Parent HoldCo (NEW NAME)
    │
    ├── Bencal Investment Counsel Inc.       ← Tier 2: Licensed Manager / Promoter (NEW NAME)
    │   (BCL-CA-01-OPR)
    │       │
    │       ├── Bencal Credit Partners Inc.  ← Tier 3: GP — Credit club deals
    │       │   (BCL-CA-02-GPR)
    │       │       └── Bencal Credit Club 1 LP (BCL-CA-01-AST)
    │       │       └── Bencal Credit Club 2 LP (BCL-CA-02-AST)
    │       │
    │       ├── Bencal Real Assets Partners Inc.  ← Tier 3: GP — Real Assets club deals
    │       │   (BCL-CA-03-GPR)
    │       │       └── Bencal Real Assets Club 1 LP (BCL-CA-03-AST)
    │       │
    │       ├── Bencal Private Investments Inc.   ← Tier 3: GP — Private SPVs
    │       │   (BCL-CA-01-GPR)
    │       │       └── [Place Name] Investments LP (BCL-CA-NN-AST)
    │       │           e.g., Sturgeon Bay Investments LP
    │       │
    │       └── Bencal Securities Inc.       ← Tier 3: EMD / Distribution (NEW)
    │           (BCL-CA-01-SEC)
    │
    └── [Future operating company]           ← Tier 2: Operating Co. / Employees
        (BCL-CA-02-OPR)
```

---

## Research trail

### Done

1. **North American hyperscaler naming research** — 13 firms benchmarked (Blackstone, KKR, Apollo,
   Carlyle, Ares, Brookfield, TPG, Warburg Pincus, General Atlantic, Blue Owl, Hamilton Lane,
   Neuberger Berman, Oaktree). SEC filings, Wikipedia, Bloomberg, firm websites verified.
   Key finding: "Bencal Corporation" mirrors the 2022 Brookfield Corporation rename — the
   single most transferable structural precedent.

2. **European and Canadian institutional naming research** — 14 firms benchmarked (Partners Group,
   Ardian, Tikehau, Eurazeo, ICG, Bridgepoint, CVC, Antin, Sagard, CDPQ/La Caisse, PSP, Fiera,
   Onex, Fairfax). Key finding: ICG shed "Intermediate Capital Group" at July 2025 AGM — the
   clearest current signal that descriptor-shedding is accelerating. PSP's place-name SPV pattern
   is the most transferable innovation for Bencal's private SPV series.

3. **Source document review** — org chart HTML and Woodfine LPs tear sheet read. Confirmed entity
   structure, code collision, and current brand language. Tear sheet confirms "Alternatives" is
   already recognized as a fading descriptor internally.
