---
artifact: brief
status: archived
---

# FINAL EXECUTION PLAN: Award-Winning Wiki Overhaul (v3.0)
# Status: Production-Ready Engineering & Editorial Directive
# Target: Full-Stack Engineers, Editorial Leads, AI Agents

## 0. Executive Summary & Objective
This document defines the definitive roadmap to elevate the PointSav/Woodfine wiki platforms from a "C-" technical archive to an "A-" award-winning knowledge environment. The overhaul aligns three surfaces (Corporate, Projects, Engineering) for high-stakes institutional audiences (Goldman Sachs, Bloomberg, Google Developers).

**Core Philosophy:** 
1.  **Editorial:** The "Lucidity Protocol" (Franklin Narrative Arc, Muscular Verbs).
2.  **Visual:** "Liquid Glass" Optics (GPU-accelerated refraction, Typographic Authority).
3.  **Functional:** "Agent-Legibility" (MCP tools, JSON-LD graph).

---

## 1.TODO: Phase 1 — Platform & Token Injection (Engineering)
- [ ] **Sync Primitives:** Map all tokens from `woodfine-media-assets` and `pointsav-media-assets` into `pointsav-design-system`.
- [ ] **CSS/SVG Implementation:** Inject `structural-physics.css` and the `liquid-refraction` SVG filter into the global `base.html` template in `app-mediakit-knowledge/src/server.rs`.
- [ ] **Token Injection:** Update `main.rs` and `server.rs` to load tokens at startup and inject them as CSS variables (`:root { ... }`) in the head of every page.
- [ ] **Verify Performance:** Profile "Liquid Glass" cards to ensure <16ms frame times (GPU locked, `translateZ(0)` applied).

## 2.TODO: Phase 2 — Blueprint & Template Pivot (Structural)
- [ ] **Implement 4-Blueprint Logic:** Update `wiki_page()` in `server.rs` to read the `type:` field from Markdown frontmatter.
- [ ] **Apply Blueprints:** Switch template structures based on the map:
    - `Strategic` (Concepts, Architecture)
    - `Operational` (Services, Systems)
    - `Definitional` (People, Places)
    - `Procedural` (Guides, Reference)
- [ ] **Standardize Home Page:** Reorder `home_chrome()` slots to Wikipedia-register standard: `Banner -> Lede -> Featured -> DYK -> Category Grid -> Recent`.

## 3.TODO: Phase 3 — Editorial & Lucidity Sweep (Content)
- [ ] **Rewrite `index.md`:** Apply the Franklin Narrative Arc (Crisis -> Quest -> Breakthrough). Start with the "Data Sovereignty Crisis."
- [ ] **Audit Top 12 TOPICs:** Rewrite to satisfy "Lucidity Rules":
    - **Sentence Rhythms:** Accordion variation (alternating 5–60 words).
    - **Active Agency:** Assign technology verbs (e.g., *the system orchestrates*, *the ledger validates*).
    - **Analogies:** Insert one sensory/kitchen-table analogy every 300 words.
- [ ] **Nut Graf Audit:** Ensure every core TOPIC has a "Nut Graf" defining the high-stakes impact within the first 10% of the text.

## 4.TODO: Phase 4 — Agentic Wiring (Intelligence)
- [ ] **MCP Mounting:** Implement the `POST /mcp` endpoint in `mcp.rs`. Expose tools:
    - `fetch_architectural_intent(slug)`
    - `resolve_link_graph(slug)`
    - `validate_editorial_standards(content)`
- [ ] **JSON-LD Schema:** Finalize `jsonld_for_topic()` to emit `TechArticle` schema, ensuring cross-platform searchability by LLM crawlers.

---

## 5. Verification Checklist (Definition of Done)
- [ ] **Visual:** Do the UI components warp background colors (Liquid Glass) without performance drops?
- [ ] **Editorial:** Does the prose rhythm pass the Pulitzer "Accordion" test?
- [ ] **Agentic:** Can an external agent (e.g., Claude) successfully invoke the MCP tools to summarize platform architecture?
- [ ] **Institutional:** Does the site pass high-trust checks (typography authority, clean citation ribbons)?
- [ ] **Bilingual:** Does auto-detection of `.es` siblings function perfectly without manual configuration?

---

## 6. Reference Links (Local Documentation)
- **Design Tokens:** `pointsav-design-system/tokens/`
- **Engineering Core:** `app-mediakit-knowledge/src/server.rs`
- **Editorial Standards:** "The Lucidity Protocol" in Section 1 of this document.
