---
artifact: brief
status: archived
---

# Master Strategy — Award-Winning Documentation Wiki (2026)
# Version: 3.0.0-CONSOLIDATED
# Status: Production-Ready / Engineering Directive
# Target: Full-Stack Engineers, Editorial Leads, AI Agents

## 0. Executive Summary: The Institutional Convergence
Our goal is to elevate the PointSav/Woodfine wiki surfaces from a "C-" technical archive to an "A-" award-winning knowledge environment. This system must serve a high-stakes, cross-functional audience:
*   **Institutional Finance (Goldman Sachs, Bloomberg):** Demands clinical clarity, fiduciary-grade assertions, and regulatory transparency (WORM Ledger/Direct-Hold).
*   **Systems Engineering (Google/Platform Developers):** Demands "Agent-legible" depth, MCP/JSON-LD integration, and API-first information architecture.

This blueprint aligns these audiences by treating the three wiki surfaces (`Corporate`, `Projects`, `Engineering`) as distinct **Data Interfaces** on the same **Knowledge Substrate**.

---

## 1. Editorial Physics: "The Pulitzer Lucidity Protocol"

### 1.1 Syntactic Rhythm (The Accordion Rule)
*   **Expansion:** Complex technical explanations must use long, lush sentences (30–60 words) to build immersion.
*   **Contraction:** Follow every expansion with a "short punch" sentence (5–10 words) for definitive clarity.
*   **Goal:** High standard deviation in sentence length.

### 1.2 Muscular Verbs (Agency Mandate)
*   **Forbidden:** Passive "to be" states (*is, are, was*).
*   **Mandatory:** Sensory, active verbs that assign agency to the technology.
    *   *Examples:* The microkernel **branches**, the protocol **enforces**, the data **betrays**, the architecture **insulates**.

### 1.3 The Franklin Narrative Arc
Every core TOPIC must follow this 3-step arc:
1.  **Complication (Crisis):** The systemic failure or data-sovereignty gap (The "Why").
2.  **Quest (Challenge):** The engineering struggle to solve it.
3.  **Resolution (Breakthrough):** The specific PointSav mechanism that resolves the crisis.

### 1.4 UX Writing (Microcopy Standards)
*   **Intent-Based Labels:** Buttons must describe the *outcome* (e.g., "Deploy Mirror" instead of "Submit").
*   **Decision-First Headlines:** Headlines provide insights, not labels (e.g., "Sovereignty is structurally enforced" vs "Sovereignty features").
*   **In-line Analogies:** One major physical analogy every 300 words.

---

## 2. Visual Physics: "Liquid Glass & Typographic Authority"

### 2.1 The Glass Engine (CSS/SVG Physics)
Production implementation uses a GPU-optimized pipeline for 60 FPS performance:
*   **Refraction:** SVG `feDisplacementMap` + CSS `backdrop-filter: blur(12px) saturate(180%)`.
*   **Specular Highlights:** CSS `@property` to sweep light across card surfaces.
*   **Optics:** `oklch` color spaces for perceptually uniform transparency.

### 2.2 Typographic Authority (Linear + Encyclopedia Standard)
*   **Headings (Authority):** `Georgia` | `300 weight` | `-0.02em tracking`.
*   **Body (Encyclopedia):** `Linux Libertine` | `1.65 line-height`.
*   **Code (Precision):** `IBM Plex Mono` | `-10% font-size` vs body.

---

## 3. Design Token Architecture (DTCG 2026)

### 3.1 Primitive Sync (Single Source of Truth)
Extracted from `pointsav-design-system`, `woodfine-media-assets`, and `pointsav-media-assets`.

| Variable | Production Value (PointSav) | Production Value (Woodfine) |
| :--- | :--- | :--- |
| `--sys-canvas` | `#09090B` (Brutalist Dark) | `#F7F9FA` (Institutional Light) |
| `--sys-card` | `#111827` (Slate Card) | `#FFFFFF` (Pure White) |
| `--sys-accent` | `#869FB9` (Steel) | `#164679` (Woodfine Blue) |

### 3.2 Layout Blueprints (4-Tier Semantic Mapping)
The engine selects the blueprint based on the `type:` frontmatter field.

| Blueprint | Semantic Intent | Goal |
| :--- | :--- | :--- |
| **Strategic** | Narrative flow, depth | Narrative Authority |
| **Operational** | Density, precision | Technical Precision |
| **Definitional** | Fast Scanning | Scanning Efficiency |
| **Procedural** | Steps, progress | Step-by-Step Guidance |

---

## 4. Intelligence Layer: "Agent-Legibility" (Stripe v2026)

### 4.1 MCP JSON-RPC 2.0 Tools
The engine exposes the following Model Context Protocol tools:
*   `fetch_architectural_intent(slug)`: Returns full text + research trail.
*   `resolve_link_graph(slug)`: Returns inbound/outbound links.
*   `validate_editorial_standards(content)`: Checks against Pulitzer rules.

### 4.2 JSON-LD (TechArticle)
Every article in `<head>` emits structured `TechArticle` schema for crawlers and agents.

---

## 5. Engineering Roadmap (Non-Destructive Upgrade)

1.  **Phase 1 (Inject):** Modify `server.rs` to parse Design System tokens at startup and inject as CSS variables.
2.  **Phase 2 (Pivot):** Update `home_chrome()` to follow the new slot order (Banner -> Lede -> Featured -> DYK -> Grid).
3.  **Phase 3 (Refract):** Add `refraction.svg` and apply `.wiki-card-glass` to UI components.
4.  **Phase 4 (Agent):** Mount the `POST /mcp` endpoint and `jsonld_for_topic()` helper.
