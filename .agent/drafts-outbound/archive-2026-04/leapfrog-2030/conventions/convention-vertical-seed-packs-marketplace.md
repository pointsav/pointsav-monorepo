---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/vertical-seed-packs-marketplace.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 50
research_done_count: 5
research_suggested_count: 4
open_questions_count: 2
research_provenance: master-cross-industry-vertical-saas-research
research_inline: true
---

# Vertical Seed Packs Marketplace

Foundry intends to distribute industry-specific seed packs as starter taxonomies
for Tier 0 customer deployments.

This convention codifies Doctrine claim #50 (ratified v0.1.0).

## §1 — Pack content

Each pack is a curated bundle:

- `archetypes.json` — 5–7 industry-specific role identities
- `chart-of-accounts.json` — 5–10 industry-specific business profiles
- `domains.json` — 3–5 macro categories
- `themes.json` — 4–10 starter time-bound initiatives (often empty; the
  customer adds their own)
- `glossary.csv` — terms used in the vertical
- `mcp-server-extensions.toml` — vertical-specific MCP tools (POS integration
  for restaurants, EHR integration for hospitals, etc.)
- `pack-manifest.toml` — metadata, version, license, contributors

## §2 — Reference packs (Phase 5 launch, planned)

| Pack | Vertical | Reference customer |
|---|---|---|
| `pack-restaurant-smb` | 5–50 employee restaurants | Dogfood: synthetic restaurant test data |
| `pack-law-firm-mid` | 30–300 lawyer firms | Dogfood: synthetic mid-firm test data |
| `pack-hospital-regional` | Small / rural hospitals | Dogfood: synthetic regional-hospital test data |
| `pack-real-estate-mid` | Mid-size property / development firms | Live: Woodfine |
| `pack-default` | Universal floor (fallback) | Foundry default |

## §3 — Pack distribution mechanism

Packs are intended to be distributed via the marketplace gateway (composition
with claim #52):

- Customers browse packs in slm-cli (`/marketplace browse-packs`)
- Pack installation: `slm-cli /seed install pack-restaurant-smb`
- The pack imports into the per-tenant graph in service-content
- The customer customizes (adds, edits, or removes entities) post-install
- The operator may run `slm-cli /seed update pack-restaurant-smb v0.0.2` to
  receive pack improvements (with a diff preview before apply)

## §4 — Customer contribution flow

When a customer extends their pack with additions that may be useful to others
(e.g., a restaurant adds a "Catering" sub-domain), they may contribute back:

- `slm-cli /seed contribute --to pack-restaurant-smb`
- The TUI guides the operator through review, sanitization, and consent
- The contribution becomes a pull-request equivalent to the marketplace
- Foundry curators review (per Phase 5 governance; pending operator decision)
- Accepted contributions land in the next pack version
- Other customers in the vertical receive the improvement

This is the intended structural opening for community-driven vertical evolution
without Foundry authoring all pack content.

## §5 — Pack licensing

Reference packs are intended to ship under a permissive license (CC-BY-SA or
equivalent). Customer-contributed additions retain customer copyright but grant
Foundry distribution rights for marketplace inclusion.

The customer-owned-graph-IP claim (#48) does not conflict with this: the
*customer's instance* of a pack (their customized seed graph) is their IP; the
*pack itself* is permissively licensed material.

## §6 — Composition with other claims

- Claim #47 (Seed Taxonomy as SMB Bootstrap): packs are the distribution
  mechanism for the seed pattern
- Claim #52 (Reverse-Flow Substrate): packs are intended first-class marketplace
  inventory (alongside data listings and adapters)
- Claim #48 (Customer-Owned Graph IP): the customer's customized pack instance
  is their IP; the pack itself is licensed material

## Provenance

Research reviewed: Salesforce Industry Cloud (what Foundry's approach diverges
from); ServiceNow Industry Workflows (similar comparison); WordPress Plugin
marketplace governance (community-pack precedent); Homebrew tap pattern
(decentralized package distribution); existing Woodfine seed (reference for
pack-real-estate-mid v0.0.1).

Suggested next research: (1) pack-authoring tooling (`slm-cli /seed author-pack`
flow); (2) pack governance model (Foundry curators vs community moderators);
(3) pack versioning and migration mechanics; (4) cross-pack composition (a
customer that is both a restaurant and a real-estate leaseholder).

**OQ #1 — Foundry curation criteria.** What is the bar for accepting a
community-contributed pack into the marketplace? Operator decision.

**OQ #2 — Vertical scope boundaries.** How granular are packs? "Restaurant" or
"pizza-joint" or "QSR (quick-service restaurant)"? Affects pack count and
customer-onboarding choice friction. Pending product decision.

## References

- `DOCTRINE.md` claim #50
- Companion: `conventions/seed-taxonomy-as-smb-bootstrap.md` (claim #47)
- Companion: `conventions/reverse-flow-substrate.md` (claim #52)
