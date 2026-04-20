# DataGraph / Derivative Knowledge Architecture
## Project Index and Quick Reference
**PointSav Digital Systems — Internal**
**Last updated:** April 19, 2026
**Status:** Pre-trial. All documentation written. No code written yet.

---

## Context

PointSav Digital Systems is building a sovereign, fully open-source enterprise knowledge graph
platform called the **Derivative Knowledge Architecture (DKA)**. The first institutional deployment
is **Woodfine Management Corp.**, a real estate holding company that is both the customer and
public showcase for the platform.

This repository tracks the architecture, design decisions, research, and operational documentation
for the **Phase 1 DataGraph test build** — a pipeline that reads existing structured data from
Laptop-A, builds a self-healing knowledge graph via a yo-yo GCP compute node, and generates
output via the Claude API.

**100% open source. We own it. Everything must be forkable and developable as a standalone product.**

---

## Corporate Structure

| Entity | Role |
|---|---|
| Woodfine Capital Projects Inc. | Parent holding company. Owns 100% of PointSav and Woodfine. No public GitHub presence. |
| PointSav Digital Systems | Subsidiary. The vendor. Builds the software. Cost-plus model. |
| Woodfine Management Corp. | Subsidiary. The customer. First institutional deployment. Public showcase. |
| Sovereign Data Foundation (Denmark) | Planned intended equity holder in PointSav. **Not yet formally executed.** All language must be planned/intended only — never described as a current equity holder or active governance body. |

GitHub: `github.com/pointsav` (vendor source) · `github.com/woodfine` (customer showcase)

---

## Document Index

| File | Contents |
|---|---|
| `README.md` | This file |
| `docs/ARCHITECTURE.md` | Full architecture — seven layers, pipeline, design principles |
| `docs/LAYERS.md` | Each layer explained in detail |
| `docs/SERVICES.md` | Service definitions, responsibilities, file tree |
| `docs/SCHEMA.md` | Full LadybugDB graph schema — node and relationship tables |
| `docs/STANDARDS.md` | Compliance standards reference: GARP, DARP, SOC3, IAB, DCAT, ODRL |
| `docs/VERIFICATION.md` | Distributed verification workforce specification |
| `docs/MARKETPLACE.md` | Data Marketplace and Ad Exchange specification |
| `docs/STACK.md` | Complete technical stack — all tools, versions, licenses |
| `docs/GCP-NODE.md` | GCP Cloud Run provisioning outline |
| `docs/TRIAL.md` | Trial run scope, checklist, pass criteria |
| `docs/DECISIONS.md` | All confirmed decisions and open items |
| `docs/RESEARCH.md` | Research synthesis — market, architecture, competitive landscape |

---

## The Seven-Layer Stack

```
V   Optional    Verification Layer (distributed workforce — switched on separately as a paid feature)
L5  Output      Content Generation · Wiki · Data Marketplace · Ad Exchange
L4  Themes      Temporal trend analysis — where data is moving over time
L3  Domains     Each domain has its own Glossaries + Topics (these are index nodes, not content)
L2  Structure   Chart of Accounts (frozen spine + self-healing periphery) · Archetypes (five-layer dual-labeled)
L1  Graph       Knowledge Graph — Entities · Chunks · Metrics · Documents + relationships
L0  Base        /source /ledger /assets — WORM · SHA-256 sealed · immutable · legal record
```

**L3 domains for Woodfine (fixed):** Corporate · Projects · Documentation
**L3 domains for other deployments:** unlimited — user configures in `seeds/domains.csv`

---

## Phase 1 Hardware

| Node | Hardware | Role | Status |
|---|---|---|---|
| Laptop-A | MacBook Air 2013, 4 GB RAM, Linux Mint | Substitute Totebox Archive | Active |
| MacPro | OSX 10.13.6, Python 3.10.11 | Development machine | Active |
| GCP Cloud Run | GPU node (transient) | Yo-yo compute for graph build | To provision |

Laptop-A is a substitute Totebox Archive — os-totebox is not yet stable enough for this test.
This is the accepted and stated condition for Phase 1.

---

## Phase 1 Scope

**In scope:** Load existing 2.5 GB corpus (/ledger + /assets already extracted) into the graph.
Build derivative layers L1–L4. Validate full pipeline with 3-file trial first.

**Out of scope:** Extraction from source documents (already done by Gemini API). Marketplace.
Ad Exchange. Verification workforce. Wiki publication.

---

## Five Blocking Items — Must Resolve Before Trial

All five of these must be answered before the GCP node is provisioned.

1. GCP region confirmed + GPU tier available (L4 for trial · A100 80GB for full run)
2. MacPro venv — run `pip list` and share output
3. Laptop-A — run `python3 --version` and share
4. SSH key pair (Laptop-A ↔ GCP) — exists or needs generation?
5. Top two levels of the 2.5 GB file tree on Laptop-A

---

## Core Principles

- Nothing is seeded manually except CoA structure and Domain names (plus a standard-library pack)
- No single source of truth — duplicates self-heal over time or remain as duplicates
- DARP compliance is baked into graph geometry as four structural invariants (I1–I4) — not a policy layer
- SOC3 and DARP are the governing regulatory frameworks
- Ad Exchange is against the total data graph — NOT against wiki content
- IAB Content Taxonomy v3.0 → wiki pages · IAB Audience Taxonomy v1.1 → Ad Exchange segments
- All outputs sell derivatives (L2–L5) — base assets (L0) are never sold
- Verification is an optional feature that is switched on separately
- No wiki page publishes without operator approval (SYS-ADR-19)
- Structured data never routes through AI (SYS-ADR-07)
- 100% open source — forkable — "We own it"
