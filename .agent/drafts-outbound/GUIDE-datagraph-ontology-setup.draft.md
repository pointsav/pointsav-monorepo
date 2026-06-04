---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Configure the organizational knowledge graph ontology for your business domain"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-datagraph-ontology-setup.md
target_filename: guide-datagraph-ontology-setup.md
bcsc_class: no-disclosure-implication
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — service-content ontology CSV system, taxonomy loading code"
research_inline: true
---

# Configure the organizational knowledge graph ontology for your business domain

service-content loads its entity taxonomy at startup from CSV files in a configurable
directory. This guide covers how to define entity types, chart of accounts, domains,
and glossary terms specific to your organization, so the knowledge graph reflects
your actual business vocabulary.

## Where the ontology files live

The directory is configured by the `SERVICE_CONTENT_ONTOLOGY_DIR` environment variable,
defaulting to `./ontology` relative to the service-content binary. The Woodfine
deployment uses `service-content/ontology/` in the monorepo clone.

```
ontology/
├── archetypes.csv           entity role archetypes
├── chart_of_accounts.csv   account type taxonomy
├── entity_classes.csv      entity classification types
├── themes.csv              organizational themes
├── domains/
│   ├── domain_corporate.csv
│   ├── domain_documentation.csv
│   └── domain_projects.csv
├── glossary/
│   ├── glossary_corporate.csv
│   ├── glossary_documentation.csv
│   └── glossary_projects.csv
├── guides/
│   └── guides_documentation.csv
└── topics/
    ├── topics_corporate.csv
    ├── topics_documentation.csv
    └── topics_projects.csv
```

## Editing archetypes

`archetypes.csv` defines the role profiles used during extraction. Each row becomes
an entity of classification `archetype` in the graph.

Column headers: `id, label, description, domain`

Example:
```csv
id,label,description,domain
exec-director,The Executive Director,Strategic oversight and accountability,corporate
legal-counsel,The Legal Counsel,Regulatory compliance and contract management,legal
property-manager,The Property Manager,Site operations and tenant relations,real-estate
```

After editing, reload without restart:
```bash
curl -X POST http://127.0.0.1:9081/v1/config/archetypes \
  --data-binary @ontology/archetypes.csv
```

## Editing the chart of accounts

`chart_of_accounts.csv` defines the account type taxonomy. Rows appear in graph queries
as `coa-profile` entities, giving the inference model context about financial structure.

Column headers: `id, label, category, description`

Example for a real estate operation:
```csv
id,label,category,description
coa-rent,Rental Income,revenue,Gross rent collected from tenants
coa-maintenance,Maintenance Expense,expense,Repairs and upkeep of managed properties
coa-mgmt-fee,Management Fee,expense,Fees paid to property management firm
```

## Editing domains

Each domain CSV defines the high-level topic areas for a business segment. These
appear as `domain` entities and drive topic and guide lookup during context injection.

Column headers: `id, label, description, active`

Keep domains to 5–10 entries. More than that dilutes context injection quality.

## Editing glossary terms

Glossary CSVs provide bilingual term definitions used in extraction and context
injection. Each domain has its own glossary file.

Column headers: `term_en, term_es, definition, domain`

Example for a legal practice:
```csv
term_en,term_es,definition,domain
encumbrance,gravamen,A claim or lien attached to a property that may affect its transfer,legal
beneficial owner,propietario beneficiario,The individual who ultimately owns or controls an asset,legal
```

## Reloading without restart

Most taxonomy files can be reloaded live:

```bash
# Reload all taxonomy from disk (runs graph-cleanup.sh internally)
curl -X POST http://127.0.0.1:9081/v1/config/guides/reload

# Reload a specific section
curl -X POST http://127.0.0.1:9081/v1/config/archetypes --data-binary @ontology/archetypes.csv
curl -X POST http://127.0.0.1:9081/v1/config/coa --data-binary @ontology/chart_of_accounts.csv
curl -X POST http://127.0.0.1:9081/v1/config/domains --data-binary @ontology/domains/domain_corporate.csv
```

## Verifying the updated ontology is active

After reloading, query the graph for taxonomy entities:

```bash
curl -s "http://127.0.0.1:9081/v1/graph/context?q=property+manager&module_id=woodfine&limit=5"
# Expect: response includes the archetype entity if you defined it
```

Check entity counts to confirm taxonomy loaded:
```bash
curl -s http://127.0.0.1:9081/healthz
# Expect: "entity_count" increasing after reload
```

## Adding a new entity classification via extraction

The base extraction schema recognizes: Person, Company, Project, Account, Location.
To add a new classification, add it to the extraction schema in
`service-content/src/` (requires a code change) OR teach the extractor to map
incoming text to existing classifications creatively using role_vector.

For example, a `Regulation` can be represented as a `Policy` entity with
`role_vector: "regulation|legal"`. This avoids a code change while still making
the regulatory reference available for graph traversal.

For significant domain expansion requiring new first-class entity types, open a
development session to extend the extraction schema enum.
