---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/seed-taxonomy-as-smb-bootstrap.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 47
research_done_count: 9
research_suggested_count: 4
open_questions_count: 1
research_provenance: master-direct-read-of-existing-woodfine-seeds + cross-industry-validation
research_inline: true
---

# Seed Taxonomy as SMB Bootstrap

Every Foundry tenant deployment provisions a four-part seed taxonomy as the
bootstrap of its knowledge graph: Archetypes, Chart of Accounts, Domains,
Themes. Each entity carries `gravity_keywords` — explainable keyword anchors
for classification. The taxonomy is compact, hand-tunable, and auditable.

This convention codifies Doctrine claim #47 (ratified v0.1.0). It is the
operational form for service-content's tenant-onboarding flow.

## §1 — The four parts

Every per-tenant seed taxonomy has exactly four parts:

### Archetypes — who acts

5–7 role-by-cognitive-pattern identities. The Foundry default (derived from
Woodfine dogfood):

- **The Executive** — Strategic Direction
- **The Guardian** — Risk & Compliance
- **The Fiduciary** — Resource Integrity
- **The Architect** — System Design
- **The Constructor** — Physical Realization

Industry-specific Archetypes are added via Vertical Seed Packs (claim #50).
The Foundry default carries these five as the universal floor — every business
has versions of these roles regardless of industry. A restaurant owner may
occupy the Executive and Constructor roles simultaneously; a hospital
administrator occupies the Executive; a hospital's privacy officer occupies the
Guardian.

### Chart of Accounts — what business this is

5–10 industry-specific business profiles. Each profile has:
- `id` — sequential identifier
- `profile` — business profile name (e.g., "Real Estate", "Compliance",
  "Patient Services", "Practice Areas")
- `sub_domain` — finer-grained category within the profile
- `gravity_keywords` — terms that anchor a transaction or document into this
  profile

The COA profiles mirror the business's actual revenue and expense categories.
A restaurant's COA might be "Operations / Food Cost / Labor / Real Estate /
POS Fees / Marketing." A law firm's might be "Compliance / Practice Areas /
Trust Accounting / Real Estate / Tech / Marketing." The form is consistent;
the substance is industry-specific.

### Domains — macro categories of work

3–5 macro categories that group work units. The Foundry default:

- **Corporate** — strategic and governance work
- **Projects** — initiative-bound work products
- **Documentation** — knowledge artefacts and reference material

Each Domain holds a Glossary (key terms with definitions) and Topics (narrative
or reference articles). The Domain is the addressing scheme for the wiki
structure, composed via the Reverse-Funnel Editorial Pattern (claim #35).

### Themes — time-bound initiatives

4–10 active themes representing current strategic focus. Each Theme has:
- `id` — `THM-NN` format
- `name` — short identifying phrase
- `gravity_keywords` — terms that pull work products into this Theme

Themes age out as initiatives close; new ones are added as new initiatives
launch. The Theme set is the most volatile part of the taxonomy — it may
change quarterly or more often.

## §2 — The gravity_keywords mechanism

Classification of new content into the taxonomy uses keyword matching rather
than embedding similarity. When a document arrives:

1. service-extraction tokenizes the content
2. For each Archetype, COA profile, Domain, and Theme entity, count matches
   against `gravity_keywords`
3. The entity with the highest match count is the proposed classification
4. Ties are resolved by Doctrine claim #44 (graph-grounded apprenticeship) —
   the model picks based on existing graph relationships
5. The classification is written to the audit ledger; the operator can review
   and correct

Why keywords rather than embeddings:

- **Explainability**: "Document X classified as `COA_5: Real Estate` because
  keywords `Leasing`, `Office`, `Industrial` matched" is reviewable by the
  operator. An embedding cosine score is not.
- **Auditability**: per BCSC continuous-disclosure posture [ni-51-102], the
  classification path must be reproducible. Keywords are; embeddings drift
  across model versions.
- **Hand-tunability**: an operator who sees a misclassification edits the
  `gravity_keywords` list. Retraining an embedding model is not an
  SMB-scale operation.
- **Compactness**: 5–10 keywords per entity versus an N-dimensional vector
  per entity at every model version.

Embedding similarity is added at a separate layer (the vector store in
service-content, per the rebuild architecture). It augments keyword
classification when explicit keywords miss; it does not replace keywords.

## §3 — Provisioning a new tenant

When a new Foundry tenant is provisioned, service-content runs:

1. Operator chooses a Vertical Seed Pack (per claim #50): pack-restaurant-smb,
   pack-law-firm-mid, pack-hospital-regional, pack-real-estate-mid, or
   pack-default
2. service-content imports the pack's JSON files into the per-tenant graph:
   Archetypes, COA, Domains, Themes
3. Each entity becomes a graph node with `gravity_keywords` as properties
4. The graph is now seeded; service-extraction can classify new content
5. The operator customizes (adds, edits, or removes entities) via the TUI
   (`slm-cli /seed`)

Onboarding time: approximately 30 minutes for a typical SMB to review and
customize the pack defaults.

## §4 — Why this inverts the enterprise ontology pattern

Enterprise ontologies (Salesforce DMOs, ServiceNow CMDB classes, Microsoft
Dataverse tables) optimize for completeness across all possible customers. The
cost: any specific customer faces an overwhelming hierarchy and must hire
ontology-engineering staff to configure it.

Foundry's seed taxonomy optimizes for actionability for one specific customer.
The cost: the taxonomy does not transfer perfectly across customers (each pack
is industry-specific). The benefit: the customer can read the entire taxonomy
in 30 minutes and operate it themselves. Foundry does not require ontology
staff. This is the structural inversion for the SMB market.

## §5 — Composition with claims #44, #46, #47, #48, #50

- Claim #44 (Knowledge-Graph-Grounded Apprenticeship): the graph populated by
  the seed taxonomy is the grounding source for inference
- Claim #46 (MCP-as-Substrate): the seed taxonomy is exposed as MCP resources
  from service-content (`graph://module_id/Archetype/...`)
- Claim #48 (Customer-Owned Graph IP): the seeded and customized taxonomy is
  the customer's IP
- Claim #50 (Vertical Seed Packs Marketplace): packs are the distribution
  mechanism; customers may contribute pack refinements back

## §6 — Existing reference implementation

The Woodfine seed at `vendor/pointsav-monorepo/service-content/seeds/` is the
reference implementation:

- `Archetypes.json` — 5 entities
- `ChartOfAccounts.json` — 4 profiles (Compliance, Real Estate, Construction,
  Investor Relations)
- `Domains.json` — 3 categories (Corporate Strategy, Capital Projects,
  Digital Systems)
- `Themes.json` — 4 active themes (THM-01 Co-Location, THM-02 Flow-Through
  Taxation, THM-03 Broadcom Driver Migration, THM-04 Q3 Capital Procurement)

The Woodfine seed predates this convention (authored as part of the existing
service-content scaffold) but conforms to the convention exactly. This is
structural substantiation per CLAUDE.md §6 — the convention codifies a working
pattern.

## Provenance

Research reviewed: direct read of existing Woodfine seeds (5/4/3/4 entity
counts); cross-industry mapping across four verticals (restaurant, law firm,
hospital, real estate) — same form, different substance; Salesforce Data Cloud
DMO complexity; ServiceNow CMDB class hierarchy; Microsoft Dataverse table
model; iManage matter-classification effort (legal-ops headcount required);
Toast POS knowledge management gap (SMB-scale failure pattern); BCSC
continuous-disclosure auditability requirement; Reverse-Funnel Editorial
Pattern (claim #35) composition.

Suggested next research: (1) vertical seed pack starter content for four
reference verticals (~2 hours each); (2) onboarding wizard UX in slm-cli
(`/seed` slash command); (3) pack contribution flow (customer refinement →
marketplace listing → community pack); (4) seed taxonomy schema versioning
(additive vs breaking changes).

**OQ #1 — Cross-tenant pack derivation.** When a customer extends their pack
with new entities (e.g., a restaurant adds a "Catering" sub-domain), can other
same-vertical tenants benefit automatically? Options: opt-in "share-back" to
marketplace; opt-in private-pack sharing within an industry vertical; no
automatic sharing. Pending operational decision in Phase 5.

## References

- `DOCTRINE.md` claim #47
- Reference implementation: `vendor/pointsav-monorepo/service-content/seeds/*.json`
- Companion: `conventions/vertical-seed-packs-marketplace.md` (claim #50)
- Companion: `conventions/customer-owned-graph-ip.md` (claim #48)
- Companion: `conventions/knowledge-graph-grounded-apprenticeship.md` (claim #44)
