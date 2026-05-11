# ARTICLE-FRAMEWORK-SPEC.md

> **Purpose:** The definitive design manual for PointSav content.
> **Scope:** Defines the tokenised architecture for reconstructing Wikipedia-style articles.
> **Goal:** Standardise all content as assemblies of reusable design tokens.

---

## 1. Article Archetypes
To ensure comprehensive parity, all content must map to one of these core archetypes:

### Archetype A: Biographies (People)
- **Primary Tokens:** `{{Infobox-Person}}`, `{{Timeline-Row}}`, `{{Citation-Cluster}}`.
- **Framework Requirements:** Birth/Death/Life data, professional milestones, influence networks.

### Archetype B: Geospatial (Places)
- **Primary Tokens:** `{{Infobox-Settlement}}`, `{{Map-Embed}}`, `{{Demographics-Table}}`.
- **Framework Requirements:** Geolocation, governance, population density, historical markers.

### Archetype C: Science & Concepts (General)
- **Primary Tokens:** `{{Chemical-Formulae}}`, `{{Math-Formulae}}`, `{{Diagram-Row}}`.
- **Framework Requirements:** Abstract reasoning, complex structure breakdown.

### Archetype D: Media/Corporate (Entities)
- **Primary Tokens:** `{{Infobox-Company}}`, `{{Logo-Embed}}`, `{{Financial-Table}}`.
- **Framework Requirements:** Founding date, market presence, core governance logic.

---

## 2. Tokenisation Schema (Design-Level)

| Macro-Frame | Token | MediaWiki Parity Target |
|---|---|---|
| Page Chrome | `.mw-body` | 100% |
| Infobox | `{{Infobox-*}}` | `.infobox` (floating right) |
| Navigation | `{{Navbox-*}}` | `.navbox` (collapsible) |
| Citations | `{{Citation-Cluster}}` | `.reflist` |
| Media | `{{Embed-Media}}` | `.mw-parser-output` figure logic |

---

## 3. Editorial Strategy: The "Suture" Report
The `project-editorial` team must ensure that every article is "sutured" correctly:
1.  **Lede Stitching:** First paragraph must be a self-contained summary.
2.  **Wikilink Anchoring:** Every internal link must resolve to a valid `[[slug]]` or be explicitly flagged as a `redlink`.
3.  **Reference Integrity:** No statement of fact without a corresponding `{{Citation}}`.
