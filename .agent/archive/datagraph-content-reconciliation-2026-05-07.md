---
title: DataGraph Content Reconciliation Report
date: 2026-05-07
produced_by: task@claude-code (project-editorial)
scope: read-only analysis; no wiki edits performed
---

# DataGraph Content Reconciliation — 2026-05-07

---

## 1. Executive Summary

| Metric | Count |
|---|---|
| CSV seed entries (all three files) | 15 |
| CSV entries matched to a wiki article | 1 |
| CSV entries with no matching wiki article (wanted) | 14 |
| Documentation wiki English articles | 194 (excl. index.md) |
| Projects wiki English articles | 17 (excl. index.md; 1 comms item) |
| Corporate wiki English articles | 5 (excl. index.md) |
| Total English articles across all three wikis | 216 |
| Wiki articles matched to CSV | 1 |
| Wiki articles unclassified (no CSV entry) | 215 |
| Woodfine fleet projects (filesystem) | 15 |
| Woodfine fleet projects covered | 11 |
| Woodfine fleet projects partial | 3 |
| Woodfine fleet projects empty | 0 |
| Woodfine fleet registry drift entries | 2 |
| Pointsav fleet projects (filesystem) | 6 |
| Pointsav fleet projects covered | 5 |
| Pointsav fleet projects partial | 1 |
| Pointsav fleet registry drift entries | 1 |
| guides_documentation.csv present | No — gap confirmed |

**Key finding:** The three topic CSVs represent only 15 seed entries against a real wiki corpus of 216 articles. The CSVs were authored as a scaffold and have not been updated to reflect the actual wiki. Every CSV topic still carries `active_state: pending`. The wiki has grown significantly beyond the CSV seed — the CSVs are effectively a stub placeholder, not an authoritative index. The correct action is to build a new ontology sweep (see §9).

---

## 2. TOPIC Reconciliation Table

All 15 CSV entries carry `active_state: pending`. The CSV `wiki_path` column uses a stale path format (`topics/topic-*.md`) — the actual format is `<category>/<slug>.md`.

Additional structural bug: all 15 entries specify `wiki_repo: content-wiki-documentation`. The corporate and projects topics should point to `content-wiki-corporate` and `content-wiki-projects` respectively.

### 2a. Documentation domain (topics_documentation.csv)

| topic_id | title | Status | Notes |
|---|---|---|---|
| `topic-doorman-protocol` | Doorman Protocol | **active** | Article exists at `architecture/doorman-protocol.md` in content-wiki-documentation. CSV `wiki_path` is stale. |
| `topic-ontological-data-graph` | Ontological Data Graph | **wanted** | No matching article found. `governance/ontological-governance.md` covers governance but not the DataGraph itself. |
| `topic-knowledge-graph-architecture` | Knowledge Graph Architecture | **wanted** | `architecture/knowledge-graph-grounded-apprenticeship.md` is related but not the same concept. No exact match. |
| `topic-semantic-entity-extraction` | Semantic Entity Extraction | **wanted** | No matching article in any wiki. |
| `topic-taxonomy-bootstrap` | Taxonomy Bootstrap | **wanted** | `architecture/seed-taxonomy-as-smb-bootstrap.md` is semantically related but covers SMB seeding, not the bootstrap concept directly. No exact slug match. |

### 2b. Corporate domain (topics_corporate.csv)

**All five have wrong `wiki_repo`** — they are committed to `content-wiki-documentation` but belong in `content-wiki-corporate`. The corporate wiki has five articles, but none match the CSV slugs.

| topic_id | title | Status | Closest existing article | Notes |
|---|---|---|---|---|
| `topic-direct-hold-structures` | Direct-Hold Tax Structures | **wanted** | `topic-direct-hold-framework.md` (corporate wiki) | Similar subject, different slug. CSV expects "structures"; wiki has "framework". |
| `topic-flow-through-taxation` | Flow-Through Taxation | **wanted** | None | No match anywhere. |
| `topic-perpetual-equity` | Perpetual Equity Model | **wanted** | `topic-equity-transfer-model.md` (corporate wiki) | Different concept. |
| `topic-accredited-investors` | Accredited Investor Framework | **wanted** | None | No match anywhere. |
| `topic-corporate-governance` | Corporate Governance Standards | **wanted** | None | No match anywhere. |

### 2c. Projects domain (topics_projects.csv)

**All five have wrong `wiki_repo`** — committed to `content-wiki-documentation` but belong in `content-wiki-projects`. The projects wiki has 17 English articles, mostly co-location intelligence coverage; none match the CSV slugs.

| topic_id | title | Status | Closest existing article | Notes |
|---|---|---|---|---|
| `topic-co-location-mandate` | Co-Location Mandate | **wanted** | `topic-co-location-intelligence-overview.md` (projects wiki) | Different concept; overview covers the programme, not the mandate. |
| `topic-woodfine-buildings` | Woodfine Buildings Standard | **wanted** | None | No match anywhere. |
| `topic-self-similar-governance` | Self-Similar Governance | **wanted** | None | Governance wiki exists in doc wiki but covers ontological governance, not self-similar governance. |
| `topic-mix-of-use` | Mix-of-Use Design Model | **wanted** | None | No match anywhere. |
| `topic-capital-framework` | Capital Framework | **wanted** | None | No match anywhere. |

---

## 3. Unclassified Wiki Articles — content-wiki-documentation

194 English articles in content-wiki-documentation have no corresponding CSV entry. Grouped by category:

| Category | Count | Notable articles |
|---|---|---|
| `architecture/` | 71 | leapfrog-2030-architecture, compounding-substrate, doorman-protocol (the 1 matched), three-ring-architecture, worm-ledger-design, sovereign-ai-commons, yoyo-compute-substrate, slm-stack-architecture, 60+ others |
| `reference/` | 37 | All style-guide-* (15 articles), model-tier-discipline, bcsc-disclosure-posture, draft-research-trail-discipline, project-tetrad-discipline, wikipedia-leapfrog-pattern, 30+ others |
| `design-system/` | 25 | design-color, design-typography, design-spacing, wiki-component-library, 16 guide-component-* articles, guide-component-* set is design-system-specific and unlikely to ever be CSV topics |
| `reference/` (style guides) | 15 | style-guide-architecture, style-guide-guide, style-guide-topic, etc. — editorial reference, not DataGraph TOPIC entities |
| `services/` | 15 | service-slm, service-extraction, service-people, service-search, service-email, service-fs-architecture, pointsav-gis-engine, 8 others |
| `applications/` | 9 | app-mediakit-knowledge, app-mediakit-marketing, app-orchestration-gis, location-intelligence-platform, wikipedia-leapfrog-design, 4 others |
| `governance/` | 7 | ontological-governance, contributor-model, sovereign-airlock-doctrine, moonshot-initiatives, 3 others |
| `infrastructure/` | 4 | edge-deployment, storage, telemetry-architecture, and 2 guide-* (guide-telemetry, guide-totebox-orchestration-gis — these are fleet GUIDEs misplaced in the wiki) |
| `systems/` | 6 | topic-console-os, topic-infrastructure-os, topic-mediakit-os, topic-totebox-os, topic-totebox-orchestration, topic-totebox-archive |

**Anomalies noted:**
- `applications/user-guide-2026-03-30-v2.md` — violates file naming convention (date in slug, `_V2` suffix). English-only, no bilingual pair. Likely a draft artifact.
- `infrastructure/guide-telemetry.md` and `infrastructure/guide-totebox-orchestration-gis.md` — these are fleet GUIDE files (how-to documents) placed in the wiki rather than in `woodfine-fleet-deployment`. Per Doctrine §14 ("How to operate it → GUIDE"), they belong in a fleet-deployment project directory, not in content-wiki-documentation.
- `design-system/guide-component-*.md` (16 files, English-only, no bilingual pair) — design-system component usage guides. These are wiki-native component documentation, not DataGraph TOPIC entities. Fine as-is; just not candidates for CSV registration.

---

## 4. Wiki-Specific Article Lists

### content-wiki-projects (17 English articles + 1 comms)

| Slug | Type |
|---|---|
| `topic-asset-architecture-standard` | topic |
| `topic-co-location-anchors` | topic |
| `topic-co-location-index-canada` | topic |
| `topic-co-location-index-italy` | topic |
| `topic-co-location-index-mexico` | topic |
| `topic-co-location-index-nordics` | topic |
| `topic-co-location-index-poland` | topic |
| `topic-co-location-index-spain` | topic |
| `topic-co-location-index-us` | topic |
| `topic-co-location-intelligence-overview` | topic |
| `topic-co-location-methodology` | topic |
| `topic-co-location-ranking-system` | topic |
| `topic-regional-market-matrix` | topic |
| `topic-site-ledger-integration` | topic |
| `topic-tier-index-europe` | topic |
| `topic-tier-index-north-america` | topic |
| `topic-zoning-acquisition-rules` | topic |
| `comms/text-gis-nordic-coverage-release` | comms (news release) |

### content-wiki-corporate (5 English articles)

| Slug | Type |
|---|---|
| `topic-direct-hold-framework` | topic |
| `topic-equity-transfer-model` | topic |
| `topic-fiduciary-data-mandate` | topic |
| `topic-interest-coverage-ratio` | topic |
| `topic-redemption-elimination` | topic |

**Note:** None of the five corporate wiki articles match any of the five CSV entries for the corporate domain. The CSV uses different concept names than what was actually written.

---

## 5. GUIDE Gap Table

### 5a. Woodfine Fleet Deployment

The skeleton minimum is: `guide-deployment.md` + `guide-provision-node.md`. Projects using project-specific naming (e.g., `guide-deployment-marketing-site.md`) are classified partial unless a standard skeleton supplement is confirmed. Registry state is from `/srv/foundry/customer/woodfine-fleet-deployment/.agent/rules/project-registry.md` last updated 2026-05-07.

| Project | Registry State | GUIDE files | Classification |
|---|---|---|---|
| `cluster-totebox-corporate` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `cluster-totebox-personnel` | Scaffold-coded | guide-deployment, guide-provision-node, guide-cold-storage-sync, guide-ingress-operations, guide-linkedin-adapter, guide-msft-entra-id, guide-personnel-ledger, guide-slm-execution, guide-sovereign-search, guide-totebox-orchestration | **covered** |
| `cluster-totebox-property` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `fleet-infrastructure-cloud` | Scaffold-coded | guide-deployment, guide-provision-node, guide-provision-relay | **covered** |
| `fleet-infrastructure-leased` | Scaffold-coded | guide-deploy-vpn, guide-provision-standalone | **partial** — has operational guides but not standard skeleton names |
| `fleet-infrastructure-onprem` | Scaffold-coded | guide-deployment, guide-provision-node, guide-lxc-network-admin, guide-provision-onprem | **covered** |
| `gateway-interface-command` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `gateway-orchestration-bim` | Active | guide-deployment, guide-provision-node | **covered** |
| `gateway-orchestration-gis` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `media-knowledge-corporate` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `media-knowledge-projects` | Scaffold-coded | guide-deployment, guide-provision-node | **covered** |
| `media-marketing-landing` | Scaffold-coded | guide-deployment-marketing-site, guide-provision-marketing-site, guide-telemetry-governance, guide-telemetry-operations | **partial** — custom skeleton names, no standard guide-deployment.md/guide-provision-node.md |
| `node-console-operator` | Reserved-folder | guide-console-operations, guide-command-ledger | **partial** — operational guides exist but no deployment skeleton; Reserved state appropriate for now |
| `route-network-admin` | Scaffold-coded | guide-deployment, guide-provision-node, guide-mesh-orchestration | **covered** |
| `vault-privategit-source` | Scaffold-coded | guide-deployment, guide-provision-node, guide-doorman, guide-doorman-deployment, guide-operating-yoyo, guide-tier-a-sysadmin-tui | **covered** |

**Registry drift (woodfine):**
- `gateway-knowledge-documentation-1` — in registry as Scaffold-coded; **directory does not exist** in filesystem.
- `gateway-orchestration-gis-1` — in registry as Scaffold-coded; **directory does not exist** in filesystem.
- `media-knowledge-documentation` — in registry as Archived (removed at 6d5cda2); **directory does not exist** in filesystem (correct, but registry entry should mark state=Archived with date).
- `cluster-totebox-property` registry notes `guide-bim-archive-operations.md` added 2026-05-07, but this file **does not exist** in the filesystem directory.

**Root-level guide files in woodfine-fleet-deployment (not in any project):**
- `guide-mesh-execution.md`, `guide-physical-egress.md`, `guide-telemetry-operations.md`
These three files are at the repo root, not inside any project directory. They should be moved to the appropriate project folder.

### 5b. Pointsav Fleet Deployment

Registry last updated 2026-04-22 (stale relative to filesystem).

| Project | Registry State | GUIDE files | Classification |
|---|---|---|---|
| `gateway-orchestration-proofreader` | **NOT IN REGISTRY** | guide-deployment, guide-provision-node | **covered** (registry gap) |
| `media-knowledge-distribution` | Reserved-folder (stale) | guide-deployment, guide-provision-node | **covered** (registry state stale — should be Scaffold-coded) |
| `media-knowledge-documentation` | Reserved-folder (stale) | guide-deployment, guide-provision-node, guide-keep-the-home-page-the-gold-standard, guide-operate-knowledge-wiki | **covered** (registry state stale — should be Scaffold-coded or Active) |
| `media-marketing-landing` | Scaffold-coded | guide-deployment-marketing-site, guide-provision-marketing-site, guide-telemetry-integration, guide-telemetry-operations | **partial** — same custom-naming issue as woodfine counterpart |
| `vault-privategit-design-system` | Reserved-folder (stale) | guide-deployment, guide-provision-node | **covered** (registry state stale) |
| `vault-privategit-source` | Reserved-folder (stale) | guide-deployment, guide-provision-node | **covered** (registry state stale) |

---

## 6. Vendor-Customer GUIDE Sync Issues

Two projects appear in both `woodfine-fleet-deployment` and `pointsav-fleet-deployment`: `media-marketing-landing` and `vault-privategit-source`.

### media-marketing-landing

| File | Woodfine | Pointsav |
|---|---|---|
| guide-deployment-marketing-site.md | present | present |
| guide-provision-marketing-site.md | present | present |
| guide-telemetry-operations.md | present | present |
| guide-telemetry-governance.md | **present** | absent |
| guide-telemetry-integration.md | absent | **present** |

**Issue:** `guide-telemetry-governance.md` (woodfine) and `guide-telemetry-integration.md` (pointsav) appear to be related files that diverged — one in each repo. Per operator rule, identical GUIDEs in both repos when a GUIDE exists in both. These two files need reconciliation: determine which is canonical, propagate to the other repo, or confirm they cover different topics with different names.

### vault-privategit-source

| File | Woodfine | Pointsav |
|---|---|---|
| guide-deployment.md | present | present |
| guide-provision-node.md | present | present |
| guide-doorman.md | **present** | absent |
| guide-doorman-deployment.md | **present** | absent |
| guide-operating-yoyo.md | **present** | absent |
| guide-tier-a-sysadmin-tui.md | **present** | absent |

**Issue:** Four operational GUIDEs exist in woodfine but have no counterpart in pointsav. If these are customer-specific GUIDEs (woodfine-only procedures), the asymmetry is intentional and no action is needed. If they are vendor-neutral operational runbooks that should travel with the software, they need to be propagated to pointsav/vault-privategit-source. This requires operator decision.

---

## 7. GUIDE Entity CSV Gap

**Convention:** `/srv/foundry/conventions/datagraph-guide-entity-class.md` (ratified 2026-05-07) requires a `guides_documentation.csv` in the ontology topics directory.

**Finding:** The file does not exist.

Path checked: `/srv/foundry/vendor/pointsav-monorepo/service-content/ontology/topics/guides_documentation.csv`

Current ontology topics directory contains only: `topics_corporate.csv`, `topics_documentation.csv`, `topics_projects.csv`.

The convention also requires:
- A `documentation` domain row in the domains CSV
- A `GUIDE` entity class row in the entity-classes CSV
- A `guide-*` prefix assignment in the canonical-prefix table

None of these have been confirmed present. The convention assigns this work to project-intelligence or project-data Task scope. It is currently unexecuted.

---

## 8. Archetype / COA / Theme / Entity-Class TOPIC Coverage

### 8a. Archetypes

Eleven archetypes defined in `archetypes.csv`. No dedicated archetype-explanation TOPIC article exists in any wiki.

| Archetype | Dedicated TOPIC? |
|---|---|
| The Executive | No |
| The Guardian | No |
| The Fiduciary | No (topic-fiduciary-data-mandate.md covers a mandate concept, not the archetype) |
| The Architect | No (asset-architecture-standard.md is not about the archetype) |
| The Engineer | No |
| The Artisan | No |
| The Constructor | No |
| The Catalyst | No |
| The Envoy | No |
| The Steward | No |
| The Sage | No |

**Coverage: 0 of 11.** Whether archetypes warrant individual TOPIC articles is a design question — they may be better covered as a single `topic-archetype-system.md` overview. None currently exist.

### 8b. Chart of Accounts Profile Types

Six COA categories defined (`chart_of_accounts.csv`): Personal, Compliance, Real Estate, Collaborators, Finance, IT Support. No TOPIC article exists explaining any COA category or the COA profile system.

**Coverage: 0 of 6 categories.**

### 8c. Themes

Eleven themes defined in `themes.csv` (4 tactical, 7 strategic). Only partial indirect coverage exists.

| Theme ID | Name | TOPIC exists? |
|---|---|---|
| THM-01 | Co-Location Mandate Expansion | Partial — co-location-intelligence-overview.md touches the co-location programme but is not a mandate/theme article |
| THM-02 | Flow-Through Taxation Structuring | No |
| THM-03 | Broadcom Driver Migration | No |
| THM-04 | Q3 Capital Procurement | No |
| THM-A | Growth Strategy | No |
| THM-B | ESG Direct-Hold Solutions | No |
| THM-C | ESG Woodfine Buildings | No |
| THM-D | Mix-of-Use | No |
| THM-E | Income Solution | No |
| THM-F | Self-Similar Governance | No |
| THM-G | Capital Framework | No |

**Coverage: 0 of 11 (partial touch on THM-01).**

### 8d. Entity Classifications (9 types)

The DataGraph convention identifies nine entity classification types: person, company, organization, domain-term, research-document, corporate-document, regulatory-document, architecture-reference, technical-reference.

**No TOPIC article exists for any of these nine entity types in any wiki.** There is no `topic-entity-class-overview.md` or per-type article explaining what each entity class is, how to classify a new entity, or how to use these in the graph.

**Coverage: 0 of 9.**

---

## 9. Recommended Next Actions (prioritized)

### Priority 1 — Fix the CSV structural bugs (project-intelligence or project-data Task)

1. **Correct `wiki_repo` in topics_corporate.csv and topics_projects.csv.** All 10 rows currently point to `content-wiki-documentation`; they should point to `content-wiki-corporate` and `content-wiki-projects` respectively.
2. **Update `wiki_path` format.** Change from `topics/topic-*.md` to `<category>/<slug>.md` for the one active entry (`topic-doorman-protocol` → `architecture/doorman-protocol.md`). All others remain pending so their paths are aspirational only.
3. **Create `guides_documentation.csv`** in the ontology topics directory per the datagraph-guide-entity-class.md convention. Register all existing fleet GUIDEs as DataGraph entities (Documentation domain). Also add the `documentation` domain and `GUIDE` entity class to the domains and entity-class CSVs.

### Priority 2 — Fix wiki GUIDE misplacements (project-editorial Root)

4. **Move `infrastructure/guide-telemetry.md` and `infrastructure/guide-totebox-orchestration-gis.md`** from content-wiki-documentation to the appropriate fleet-deployment project directories. These are operational runbooks, not wiki TOPIC articles.
5. **Move or delete `applications/user-guide-2026-03-30-v2.md`** — violates file naming convention (date in slug, `_V2` suffix). Determine if this content should be migrated into a properly named article.

### Priority 3 — Resolve vault-privategit-source sync ambiguity (operator decision)

6. **Determine whether** `guide-doorman.md`, `guide-doorman-deployment.md`, `guide-operating-yoyo.md`, and `guide-tier-a-sysadmin-tui.md` in woodfine/vault-privategit-source should propagate to pointsav/vault-privategit-source. Document the decision (customer-specific vs. vendor-neutral).

### Priority 4 — Reconcile media-marketing-landing telemetry guides

7. **Reconcile** `guide-telemetry-governance.md` (woodfine) vs. `guide-telemetry-integration.md` (pointsav). Determine if these cover the same content with different names, or different topics. If same: rename to a single canonical name and sync. If different: add cross-reference and confirm intentional divergence.

### Priority 5 — Fix woodfine registry drift (project-editorial Root at woodfine-fleet-deployment)

8. **Remove or re-create** `gateway-knowledge-documentation-1` and `gateway-orchestration-gis-1` from the registry — both have no corresponding filesystem directory.
9. **Remove the note** from `cluster-totebox-property` registry entry claiming `guide-bim-archive-operations.md` exists — this file is not present in the filesystem.
10. **Update pointsav fleet registry** — last updated 2026-04-22; five of six projects have stale state (`Reserved-folder` when they are `Scaffold-coded`); `gateway-orchestration-proofreader` is completely absent from the registry.

### Priority 6 — Sweep CSV forward to reflect actual wiki corpus (project-intelligence or project-data Task)

11. **Conduct a full CSV population pass** — add CSV entries for the 215 unclassified wiki articles. The architecture category alone (71 articles) represents the largest documentation investment and is not indexed in any CSV. This is the largest single work item. A sub-agent sweep using the existing wiki file listing above would be the most efficient approach.

### Priority 7 — Evaluate theme / archetype / entity-class TOPIC creation

12. **Assess whether themes warrant TOPIC articles.** THM-A through THM-G are strategic themes that drive editorial framing across corporate and projects wikis. At minimum, a `topic-capital-framework.md` (THM-G), `topic-self-similar-governance.md` (THM-F), and `topic-mix-of-use.md` (THM-D) are likely candidates — these align with existing CSV seed entries that are `wanted` but have no article yet.
13. **Consider a single `topic-archetype-system.md`** article explaining the eleven archetypes and how the system works. Individual archetype articles are probably unnecessary at current wiki scale.
14. **Consider a single `topic-datagraph-entity-classes.md`** article explaining the nine entity classification types. Required for DataGraph onboarding documentation completeness.

---

## 10. Language Register Framework — Do This Before New Content

**Operator decision (2026-05-08):** Build language tokens for the Design System before any further TOPIC or GUIDE authorship. The current article corpus has systematic language problems that would be propagated and amplified if more articles are written in the same register. Fixing the token framework first means every subsequent article, and every service-slm training tuple derived from it, is correct from the start.

---

### 10a. The core problem

The content audit (Part 2 above) found a stark split:

- **Architecture and governance articles** are written for a Foundry internal engineering audience. They assume readers know terms like "Doorman," "substrate," "compounding," "doctrine," "Ring 3," and "Totebox." They do not explain why any of this matters to the reader.
- **Corporate and projects wiki articles** are written for external stakeholders (investors, construction professionals) and are substantially more readable. They lead with concrete consequences, use precise numbers, and avoid internal jargon.

The gap is not random — it reflects the fact that architecture articles were written by and for engineers, while the corporate and projects articles were written with an actual external reader in mind. The fix is to make the external-reader standard universal and systematic.

**Why this compounds the SLM problem:**

service-slm's training corpus ingests editorial interactions. If the corpus contains articles written in the internal-engineering register (architecture, governance), the SLM learns to produce that register. Language tokens constrain what the SLM generates and what the editorial pipeline accepts, so every training tuple is graded against the correct standard from the start.

---

### 10b. Where the RESEARCH folder is

**Location confirmed:** `/srv/foundry/deployments/vault-privategit-design-1/research/`

Contains 8 documents: `brand-voice.md`, `design-philosophy.md`, `primitive-vocabulary-rationale.md`, `wikipedia-leapfrog-2030.md`, and four UX/GIS research briefs. These are written in Bloomberg/FT style — short declarative paragraphs, no Foundry jargon, consequence-first structure. They are the closest thing the system currently has to a "positive examples" corpus for language style.

**Gap:** This folder is not yet registered in the service-content ontology and is not referenced by any wiki style guide. It should become the seed corpus for the language register training data.

A secondary research folder at `/srv/foundry/deployments/gateway-orchestration-bim-1/research/` contains BIM-specific research documents relevant to the Projects wiki register.

---

### 10c. The three audience registers

Three distinct wikis serve three distinct audiences. Each requires a different linguistic register. The same technical fact must be written three different ways — not dumbed down, but translated into the vocabulary and decision-making context of each reader.

#### Register 1 — Bloomberg/FT/Economist (Corporate wiki)
**Audience:** Bankers, family offices, institutional investors.
**Their literacy:** Financial structures, regulatory frameworks, capital markets. Not software architecture.
**What they need:** Institutional facts, not system descriptions. Consequence first. Numbers always specific.

| Dimension | Rule |
|---|---|
| Sentence length | 14–18 words target, 25 max |
| Lead structure | Most important fact first (inverted pyramid) |
| Voice | Active. Passive = red flag. |
| Numbers | Always specific. "$7/month" not "low-cost." |
| Jargon | Translate every platform term. No "substrate," "Doorman," "compounding." |
| Code blocks | Never |
| Citations | Financial research, regulatory filings, industry reports |
| What to avoid | Academic hedging ("may leverage," "potentially"), abstract nouns, dramatic marketing claims |

**Pattern example — same fact, wrong register vs. right register:**

❌ "PointSav orchestrates a Compounding Substrate where every operational interaction generates training signal that compounds across all tenant deployments."

✅ "PointSav's platform generates a shared research signal from all customer deployments. This trains improved models that flow back to every customer — a cost structure no hyperscaler can match because it would undermine the lock-in model that justifies their margins."

#### Register 2 — Stripe/Cloudflare/Linear (Documentation wiki)
**Audience:** Software engineers, graphic designers, developers.
**Their literacy:** Full technical stack. Assumes knowledge of git, SSH, JSON, databases, deployment pipelines.
**What they need:** Concept → why it matters → how it works → real code → edge cases. Peer-to-peer tone. No patronizing basics.

| Dimension | Rule |
|---|---|
| Structure | Concept → Why → How → Code → Edge cases |
| Code blocks | Real and runnable. If abbreviated, mark with `// ...` |
| Code placement | Before explanation when structure clarifies; after when context is required first |
| Jargon | Use field terms freely (async, gRPC, systemd). Define PointSav-specific terms once on first use. |
| Tone | Confident, direct, peer-level. "This is how it works" not "we believe" |
| What to avoid | Explaining SSH or JSON. Vague architecture descriptions. Incomplete code examples. |

**Structure example — Stripe register:**

"**service-slm** is the AI request router. It transparently selects among three compute tiers based on request deadline and budget caps: local inference (5s timeout), cloud burst (10s timeout), external API (30s timeout). When local inference completes, the router returns the response. If local times out, it escalates to cloud. If cloud times out, it tries the external API. All three tiers are configured in the deployment policy; see [[service-slm-architecture]] for examples."

#### Register 3 — RIBA/IFC/specification (Projects wiki)
**Audience:** Architects, engineers, construction managers, building code officials.
**Their literacy:** Technical standards, building codes, IFC, contractual specifications.
**What they need:** Prescriptive language for requirements, informative for guidance. Every measurement has units. Every claim is traceable to a standard.

| Dimension | Rule |
|---|---|
| Requirements | Use "shall" / "shall not." These are mandatory, testable, auditable. |
| Guidance | Use "may" / "should." These are best practices, not requirements. |
| Numbers | Always with units and tolerances. "±0.01 meters" not "precise." |
| Citations | Standards bodies (IFC, ASHRAE, NBC, RIBA), legal instruments, formal verification proofs |
| Lead structure | Scope → normative requirements → informative guidance → cross-references |
| Jargon | Use standards terminology freely. Cross-reference formal specifications. |
| What to avoid | Unmeasured claims, ambiguous shall/should usage, undocumented constraints |

---

### 10d. Vocabulary to retire system-wide

The following terms appear throughout the current wiki in internal-engineering usage. They are meaningful within the PointSav engineering team and worthless — or actively confusing — to any external audience. They must be replaced on first use in any externally-facing article with plain language plus a wikilink to the technical article where appropriate.

| Term to retire | Plain replacement (context-dependent) |
|---|---|
| Substrate | the data layer / the security foundation / the platform code |
| Doctrine | architectural principle / engineering policy / design decision |
| Compounding | training signal aggregation / learning over time / model improvement |
| Leapfrog | designed to exceed / planned to replace / targeting [specific capability] by [date] |
| Doorman | access-control gateway / AI request router / external AI mediator |
| Ring 1 / Ring 2 / Ring 3 | archive tier / data tier / inference gateway (or describe the function) |
| Totebox | property archive / data vault / the archive storage system |
| Yo-Yo / Yo-Yo pool | on-demand GPU instances / ephemeral inference nodes |
| Sovereign (adjective) | independently verifiable / formally verified / operator-controlled |
| The Compounding Substrate | the platform (on first use); link to architecture article |
| Apprenticeship Substrate | the learning pipeline / the training system |
| Mooncake / LadybugDB | the key-value cache / the graph database (on first use) |

**Rule:** These terms may appear in architecture and services TOPIC articles where they are being *defined*. They must not appear in corporate or projects TOPIC articles, or in GUIDEs, without a plain-language translation preceding them.

---

### 10e. What language tokens are and where they live

Language tokens are style constraints expressed in the same structured format as design tokens — they govern the verbal layer of the design system rather than the visual layer. They belong in the Design System alongside typography, color, and spacing tokens.

**Proposed location:** `pointsav-design-system/tokens/language/`

**Token types needed:**

| Token type | What it specifies | Scope |
|---|---|---|
| `register-*` | Full register specification per audience (sentence length, voice, lead structure, code presence, citation style) | Per wiki |
| `vocabulary-banned-*` | Terms forbidden in each register, with replacements | Per register |
| `vocabulary-preferred-*` | Preferred terms and constructions per audience | Per register |
| `template-topic-lead` | Sentence patterns for the opening of a TOPIC article | Per register |
| `template-guide-lead` | Sentence patterns for the opening of a GUIDE article | All GUIDEs |
| `template-section-*` | Structural templates for recurring section types (economics, validation, edge cases, references) | Per article type |

**How they integrate with service-content and service-slm:**

```
language tokens (design-system/tokens/language/)
    ↓ consumed by
service-content/schemas/
    ├── banned-vocab.lark      ← vocabulary-banned-* tokens → grammar rules
    ├── genre-templates/       ← template-* tokens → article scaffolds
    └── register-specs/        ← register-* tokens → scoring criteria
    ↓ consumed by
service-slm (Doorman editorial pipeline)
    ├── pre-generation: inject register spec into system prompt
    ├── post-generation: score against register-spec tokens
    └── training tuple: (draft, verdict, register_score) → DPO corpus
```

Every editorial interaction that passes through the Doorman becomes a training tuple tagged with the register it was scored against. Over time, the SLM learns to produce register-correct output without being prompted explicitly — the compounding effect the user is targeting.

---

### 10f. Sequencing — do this before new TOPIC and GUIDE authorship

The correct order for the content work queue is:

```
1. Language token framework (BEFORE any new articles)
   → project-design Task: author register-* + vocabulary-* tokens
   → project-intelligence Task: extend banned-vocab.lark with per-register rules
   → project-editorial Root: update style-guide-topic.md + style-guide-guide.md
      to reference the ratified registers

2. RESEARCH corpus registration (same sprint)
   → project-data or project-intelligence Task:
     register vault-privategit-design-1/research/ documents in service-content
     ontology as research-document entities (mark active_state: active)
     These become the positive-examples corpus for SLM training.

3. Existing article sweep (AFTER language tokens are ratified)
   → Large batch: flag architecture and governance articles with register violations
   → Prioritize articles most likely to be read by external audiences first
   → Rewrite leads and retire internal vocabulary; do not rewrite entire articles

4. CSV population pass (can run in parallel with step 3)
   → project-intelligence or project-data Task
   → Register 215 unclassified wiki articles in the service-content ontology CSVs
   → Tag each with the correct register (corporate-bloomberg, documentation-stripe, projects-spec)

5. New TOPIC and GUIDE authorship (AFTER steps 1–2, using ratified tokens)
   → project-editorial Task
   → Every new article is written against the register-* token for its target wiki
   → Every new article generates a training tuple tagged with the register score
```

**The articles that most urgently need a register rewrite** (highest external exposure, worst current language):
- `architecture/compounding-substrate.md` — defines the core value proposition for investors; currently written for engineers
- `architecture/doorman-protocol.md` — describes the AI boundary; currently full of internal service names
- `governance/ontological-governance.md` — covers the data governance system; impenetrable abstract jargon
- `architecture/leapfrog-2030-architecture.md` — the platform vision; needs three register variants (or a summary for each wiki)

**The articles that are already good models** (can be used as positive training examples immediately):
- All five `content-wiki-corporate/` articles — correct Bloomberg register
- `content-wiki-projects/topic-co-location-methodology.md` — correct specification register
- `content-wiki-projects/topic-zoning-acquisition-rules.md` — correct specification register

---

*End of report. This artifact is read-only reference material. All action items require a new session at the appropriate layer (Root or Task).*
