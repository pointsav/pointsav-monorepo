---
title: Content Briefs — Corporate Wiki (Step 3 output)
date: 2026-05-08
scope: 5 corporate wiki articles — what DataGraph knows that articles don't express
produced_by: task@claude-code (project-editorial)
---

# Content Briefs — Corporate Wiki

Step 3 output per the editorial reference plan. These briefs drive Step 5 rewrites.
One entry per article: domain, DataGraph connections, themes, what's missing.

---

## Domain context — content-wiki-corporate

DataGraph domain terms (domain_corporate.csv):
- Qualified Investment — contiguous parcels for a Woodfine Professional Centre
- Direct-Hold Solutions — the foundational operating entity structure
- Perpetual Equity — long-term value creation over short-term liquidity
- Flow-Through Taxation — income passed directly to investors, avoiding corporate-level tax

Active themes touching this domain:
- THM-02: Flow-Through Taxation Structuring — not currently represented in any corporate article
- THM-B: ESG Direct-Hold Solutions (strategic) — not mentioned in articles

Archetypes most relevant to corporate wiki audience:
- The Fiduciary (Resource Integrity / healing_trigger: Leakage)
- The Steward (Asset Preservation / healing_trigger: Degradation)
- The Guardian (Risk & Compliance / healing_trigger: Breach)

---

## 1. topic-direct-hold-framework.md

**Domain:** corporate — Direct-Hold Solutions

**What DataGraph knows that the article doesn't express:**
- The connection to Flow-Through Taxation (THM-02) — the Direct-Hold structure is the precondition for flow-through tax treatment; no article covers this link
- The Perpetual Equity framing — the framework is designed for long-term capital preservation, not short-term liquidity events; implicit in the article but not named
- Cross-article relationships: this is the hub article that all other 4 corporate articles depend on

**Register issues to fix:**
- Lead opens with definition: "The Direct-Hold framework is Woodfine Management Corp.'s structural approach..." → must be consequence-first
- Correct lead: "The Direct-Hold framework issues property-specific equity that corresponds to a single physical asset." (14 words, matches the token example exactly)

**Content to add:**
- "See also" section linking to all 4 sibling articles
- One-sentence reference to flow-through taxation as the tax consequence of the structure

**Word/number check:** No vague quantifiers found. Article body is specific and correct.

---

## 2. topic-equity-transfer-model.md

**Domain:** corporate — Direct-Hold Solutions (transfer mechanics)

**What DataGraph knows that the article doesn't express:**
- The "freely transferable" characteristic is named in domain_corporate.csv as a defining property of Direct-Hold Solutions — the article covers it but doesn't connect it to the domain definition
- The Equity Transfer Model is the mechanism that makes Perpetual Equity workable: because equity transfers freely rather than through a redemption queue, long-term holders don't need a corporate buyout to exit

**Register issues to fix:**
- Lead opens with definition: "The Equity Transfer Model governs how ownership interests..." → consequence-first
- Correct lead: "Woodfine Direct-Hold equity transfers freely between private parties; the corporate entity records the change but does not approve or intermediate it."

**Content to add:**
- "See also" section (direct-hold-framework, fiduciary-data-mandate, redemption-elimination)
- The existing `[[topic-fiduciary-data-mandate]]` link in the body is good — keep it

**Word/number check:** No vague quantifiers. No code blocks. Good article.

---

## 3. topic-fiduciary-data-mandate.md

**Domain:** corporate — Direct-Hold Solutions (data governance arm)

**Vocabulary violation (must fix):**
- "PointSav substrate" appears once → replace with "PointSav platform"
- Full sentence: "The corporate entity's compliance with the mandate is supported by the PointSav substrate." → "...by the PointSav platform."

**What DataGraph knows that the article doesn't express:**
- The Fiduciary Data Mandate is what makes Perpetual Equity legally defensible: an investor cannot exercise governance rights over an asset whose ledger they don't control; the mandate closes this gap
- The domain term "Sovereign Telemetry" connects here: the platform's zero-cookie, mathematically absolute ledger is the technical implementation of the mandate (crossref to documentation wiki, not inline)
- The Fiduciary archetype's healing trigger is "Leakage" — the mandate directly addresses data leakage risk

**Register issues to fix:**
- Lead opens with definition: "The Fiduciary Data Mandate establishes that..." → consequence-first
- Correct lead: "Outsourcing investor ledger data to third-party cloud infrastructure is a fiduciary breach under Woodfine Management Corp.'s operating mandate — not a vendor relationship."

**Content to add:**
- "See also" section (direct-hold-framework, equity-transfer-model)
- One sentence connecting ledger sovereignty to investor governance rights (the investor cannot exercise rights over an asset whose ledger they don't control)

---

## 4. topic-interest-coverage-ratio.md

**Domain:** corporate — Direct-Hold Solutions (debt constraint)

**What DataGraph knows that the article doesn't express:**
- The ICR floor is the primary mechanism that protects Perpetual Equity: a portfolio that maintains 1.2x ICR indefinitely can compound equity value without forced asset sales
- The per-vehicle isolation (ICR cannot be satisfied by cross-subsidizing one asset with another) is mentioned but not connected to the legal isolation described in direct-hold-framework

**Register issues to fix:**
- Lead opens with definition: "The Interest Coverage Ratio (ICR) is Woodfine Management Corp.'s primary debt management constraint." → consequence-first
- Correct lead: "A minimum 1.2x interest coverage ratio constrains every Woodfine Direct-Hold asset — no new debt may be issued that would reduce operating-income-to-interest coverage below this floor."

**Content to add:**
- "See also" section (direct-hold-framework, redemption-elimination)
- One sentence: the 1.2x floor enables indefinite equity compounding by preventing forced sales triggered by debt-service shortfalls

**Word/number check:** 1.2x is present throughout. Good specific number discipline.

---

## 5. topic-redemption-elimination.md

**Domain:** corporate — Direct-Hold Solutions (consequence of no pool)

**What DataGraph knows that the article doesn't express:**
- Redemption elimination is what converts the Direct-Hold structure from a locked-up illiquid vehicle into a Perpetual Equity model: no queue means no coordinated exit pressure, which means long-term capital is structurally stable
- The "artificial bank runs" section describes the Guardian archetype's healing event (Breach) — the coordination failure that pooled funds are vulnerable to; Direct-Hold is immune by design

**Register issues to fix:**
- Lead opens with definition: "The Redemption Elimination principle is the structural consequence of..." → consequence-first
- Correct lead: "Woodfine Direct-Hold assets carry no redemption queue — each investor's equity corresponds to a specific property, and liquidity depends on the private market for that asset."

**Content to add:**
- "See also" section (direct-hold-framework, equity-transfer-model)
- One sentence explicitly connecting to Perpetual Equity: no queue means capital is structurally stable for long-term compounding without coordinated exit pressure

---

## What NOT to add to corporate articles

- No platform architecture detail (Doorman, service-slm, Ring 1/2/3)
- No code blocks or file paths
- No RIBA/IFC specification language
- No mention of internal project names or cluster vocabulary
- No Flow-Through Taxation detail (warrant its own article — THM-02)
