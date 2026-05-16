---
schema: overhaul-progress-v1
plan: overhaul-documentation-pointsav-com.md
phase: 2
sub_phase: done
status: complete
safe_to_resume: false
unsafe_reason: ""
owner_engine: ""
last_updated: 2026-05-16T18:45:00Z
last_session_id: c5d87d6f-be9a-48df-bfe9-a48f1ea3e1d0-441063
---

## Phase 2 COMPLETE — 2026-05-16

§9.4 quality verification results:
- Broken wikilinks: 0 (was 42 genuine after false-positive triage; 46 files fixed, committed f092f94)
- bcsc_class: internal: 0
- Personal names in wiki articles: 0 (4 hits in AGENT.md/CLAUDE.md operational files only)
- Governance vocabulary in wiki articles: 0 (2 hits in operational files / anti-pattern teaching context)
- Glossary stubs: 0
- OrchestrationOS duplicate heading: removed (1 plain heading remains)
- Design-system split: 48 files removed (committed 9bbee55); _index MOC + redirects.yaml + naming-convention §7 complete
- Plan archived: .agent/plans/archive/overhaul-documentation-pointsav-com.md

## Last completed sub-task
- task: Design-system source cleanup + P1b/P2 README fixes
- commit_sha: 9bbee55 (content-wiki-documentation source cleanup), 7ece788f (pointsav-monorepo P1b), b79ff02 (content-wiki-documentation P2), d3bfd6c (woodfine-fleet-deployment P2), 7d97b51 (pointsav-fleet-deployment P2), 1c8d9bf (pointsav-design-system P2)
- committed_at: 2026-05-16
- detail: 48 files git rm'd from design-system/; _index.md rewritten (EN+ES); redirects.yaml created
  (48 entries); naming-convention.md §7 added; three handoff entries closed. P1b (pointsav-monorepo
  README) fixed on editorial-readme-fix branch — stale Apache 2.0 copyright line removed, canonical
  Spanish footer added. P2 READMEs fixed in 4 staging-tier sub-clones (Copyright (c) → Copyright ©,
  Inc.. → Inc.). Two admin-only READMEs (pointsav-media-assets, woodfine-media-assets) deferred to
  Command Session.

## Three-wiki relaunch — COMPLETE (2026-05-16)
- task: Stage 6 promotion of all three wikis + structural fixes
- committed_at: 2026-05-16
- detail:
  - content-wiki-documentation: 86 commits promoted via bin/promote.sh (commits 1c9cee3→769980b)
  - content-wiki-corporate: 3 commits promoted (commits c65be14→53a3169); structural fixes:
    index.es.md added, bcsc_class added, 3 topics brought to ≥3 wikilinks
  - content-wiki-projects: 4 commits promoted (commits 1c1e48b→420e8cf); structural fixes:
    5 slug inconsistencies fixed, broken wikilinks repaired across 24 files, index.es.md added,
    bcsc_class added, 12 articles given ≥3 wikilinks
  - service-content CSVs: rebuilt in project-intelligence (commit 6d73126b) — topics_corporate
    (5 rows), topics_projects (17 rows), topics_documentation (1 confirmed row)

## Last completed sub-task (prior session)
- task: Sub-phase 2j — Bloomberg vocabulary sweep across all remaining categories (complete)
- commit_sha: 96a6379 (content-wiki-documentation, final substrate residual batch 7b)
- committed_at: 2026-05-15
- detail: Full Foundry/Doctrine/X-Foundry-Module/~/Foundry/ vocabulary sweep across 7 batches:
  - Batch 1 (Peter): services/ EN+ES
  - Batch 2 (Jennifer): systems/ EN+ES
  - Batch 3 (Peter): infrastructure/ + patterns/ EN+ES
  - Batch 4 (Jennifer): design-system/ EN+ES
  - Batch 5 (Peter): reference/ EN+ES
  - Batch 6 (Peter, 9e891c8): substrate/ EN (13 files) + patterns/pairing-as-permission + applications/app-mediakit-knowledge.es.md
  - Batch 7 (Jennifer, e899768): substrate/ ES (18 files)
  - Batch 7b (Peter, 96a6379): 4 residual files (single-boundary-compute-discipline.md,
    reverse-flow-substrate.md, reverse-flow-substrate.es.md, design-system-substrate.es.md)
  Substitution rules: "Foundry" → "the platform" / "PointSav"; "Doctrine claim #N" → removed;
  "X-Foundry-Module-ID" → "X-Module-ID"; "Foundry artifact" → "Platform artifact";
  "~/Foundry/" paths → removed; reclamación/afirmación doctrinal #N (ES) → removed.
  Final grep across all non-architecture categories returned zero hits.
  Sub-phase 2i (architecture/ + substrate/ Bloomberg lede scrub, d5a542b) also completed this session.

## Sub-phase 2e summary (cross-reference gap fill — COMPLETE)
Audit (enhanced to skip inline code spans and .agent/ rules files) found 44 flagged links;
22 genuine broken links in published wiki content, 22 false positives (inline backtick code
examples, internal rules files, woodfine-fleet-deployment cross-references which resolve
within their own cluster). All 22 genuine broken links fixed across 6 commits:
- 25e3100 docs(systems): fix topic- prefix and os- slug renames in wikilinks
- 0d6b395 docs(patterns): fix os-totebox/os-mediakit slug renames, delink service-minutebook, fix category links
- 61a808b docs(substrate): fix category links to path-style, delink missing capability-ledger-substrate
- a23b23b docs(architecture): fix broken service-slm-architecture wikilink in doorman-protocol
- beb86c7 docs(applications): delink missing service-minutebook, service-bookkeeper, app-orchestration-bim
- 2a693af docs(services): delink missing service-minutebook in archetypes-and-chart-of-accounts

Wikilink density audit (minimum 3 per TOPIC body) deferred to sub-phase 2g (readability pass),
which applies the full quality-metric suite per-article. Orphan TOPIC linking likewise deferred
to 2g where §4 stub analysis will be populated article-by-article.

## Sub-phase 2f summary (fenced code-block normalisation — COMPLETE)
Classifier detected and tagged 87 unlabelled fenced blocks across 17 guide files in two clusters.
- Language tag distribution: bash (~65), text (~15), ini (~7)
- Classification logic: shell prefixes > bare paths (→text) > shell commands > env vars > systemd keywords > json > text
- Key edge cases handled: sudo tee heredoc classified as bash (not ini), bare /path references as text, $VAR/path as text, Environment="..." drop-ins as ini
- Classifier bug fixed: initial version failed to track labeled blocks, causing their closing fences to be treated as new unlabelled openers (all 165 false tags reverted before fix)
- 2 commits: 1ffb3bc (woodfine, 14 files, 51 changes), 1b2d50a (pointsav, 3 files, 36 changes)

## Sub-phase 2g status — readability pass (in progress)

### Completed corpus-wide mechanical sweeps (commits e8a740f, fd0d647, 7bb084e)
- quality field: `published`→`complete` (54 files) + `core`→`complete` (49 files) = 103 files
- `## See Also` → `## See also` heading normalization: 151 files
- Internal-only `## References` sections removed: 54 files
  (kept 7 articles with external URL citations; frontmatter references retained)
- `Doctrine §` body-text references replaced with plain prose: 5 files
- Doctrine claim vocabulary removed corpus-wide: 227 files (c0f7adb)
- 6 category _index.md landings rebuilt as full MOC pages (EN+ES, 12 files, ed06bec)
- Body-level 'Convention' vocabulary audit complete — 4 remaining uses are legitimate English

### Featured pool articles — individual lede/quality pass (commits e8a740f, fd0d647)
- COMPLETE: three-ring-architecture (Bloomberg lede rewrite; sys-adr-07 wikilinks fixed)
- COMPLETE: economic-model (quality + duplicate ES See also merged)
- COMPLETE: worm-ledger-design (quality + duplicate ES See also merged)
- COMPLETE: sovereign-ai-commons (quality + duplicate ES See also merged)
- COMPLETE: llm-substrate-decision (quality + duplicate ES See also merged)
- COMPLETE: customer-hostability (quality core→complete; Doctrine refs removed)
- COMPLETE: knowledge-commons (internal vocab 'conventions' replaced)
- COMPLETE: substrate-native-compatibility (already clean — no changes needed)
Note: compounding-substrate, doorman-protocol, disclosure-substrate, leapfrog-2030-architecture
  rewritten in step 5 (commits 96e221d/91b8910); not revisited in 2g.

### Sub-phase 2h — COMPLETE (commit 6050420)
Style guides updated with OPUS editorial rules.

### Sub-phase 2i — COMPLETE (architecture/ + substrate/)
- governance/procurement-overview.md + security-overview.md (EN+ES) — new articles, committed f1383b4
- governance/_index.md + .es.md — updated with institutional due-diligence group
- substrate/ category — Bloomberg lede + vocab scrub — 9 articles EN+ES (2ab8fee + eac2449)
- architecture/ category — Foundry/Doctrine/~/Foundry/Task Claude vocab scrub across all EN+ES pairs:
  - Batch 1 (7c3ca97): 5 articles from crashed session (city-code-as-composable-geometry,
    foundry-doctrine-architecture, pointsav-llm, totebox-orchestration-development, totebox-session)
  - Batch 2 (ea347f8): 12 EN + 12 ES (24 files) — _index, collab-via-passthrough-relay,
    cryptographic-ledgers, data-sovereignty-telemetry, decode-time-constraints,
    direct-payment-settlement, flat-file-bim-leapfrog, foundry-doctrine-architecture,
    foundry-doctrine-overview, identity-ledger-schema-design, location-intelligence-strategy,
    totebox-orchestration-development, vertical-seed-packs-marketplace, zero-container-inference
  - Batch 3 (8fb0eb7): 8 files — customer-hostability, foundry-doctrine-overview,
    identity-ledger-schema-design, pointsav-llm, zero-container-inference (EN+ES)
  - Batch 4 (d5a542b): totebox-session.es.md (~/Foundry/ paths + charter vocabulary)

### Sub-phase 2j — COMPLETE (commits 9e891c8, e899768, 96a6379 + batches 1–5 earlier this session)
Bloomberg vocabulary sweep across ALL non-architecture categories. All Foundry/Doctrine/internal
path vocabulary removed from services/, systems/, patterns/, reference/, governance/,
infrastructure/, design-system/, applications/, and substrate/ (EN+ES). Final grep clean.

### Phase 2 final cleanup — COMPLETE (commit 769980b, Peter)
- 11 `conventions/` internal path refs removed from published articles (agent + session)
- 2 `X-Foundry-Module-ID` → `X-Module-ID` residuals fixed (totebox-orchestration-development.md/.es.md)
- `comisión de Foundry` → `comisión de la plataforma` (architecture/_index.es.md)
- `carta constitucional de Foundry` → `carta constitucional de la plataforma` (personnel-permissions.es.md)
- Phase 2 §9.4 verification: bcsc_class CLEAN · personal names CLEAN · Foundry/Doctrine CLEAN
- Wikilink density sample: 9/10 articles ≥3 wikilinks; sys-adr-07.md has 0 (ADR format — acceptable)

### OPUS audit of content-wiki-corporate + content-wiki-projects (2026-05-15)
Both wikis are 100% clean on Bloomberg vocabulary. Structural issues found:

**content-wiki-corporate:**
- 3 of 5 TOPICs below 3-wikilink minimum (fiduciary-data-mandate, interest-coverage-ratio, redemption-elimination)
- index.es.md missing (bilingual obligation)
- bcsc_class missing on index.md

**content-wiki-projects:**
- 4 broken [[wikilinks]] in tier-index-north-america.md/.es.md (co-location-index-us, co-location-index-mexico)
- 5 broken markdown links to `(co-location-methodology.md)` (missing `topic-` prefix)
- 5 of 17 TOPIC slugs use bare form (no `topic-` prefix) — inconsistent with 12 others
- 12 of 19 published articles have 0 wikilinks
- index.es.md missing; bcsc_class missing on index.md
- Provenance blocks in country-index articles expose internal project names

**service-content seeds:** Not available in this cluster's monorepo sub-clone.
Must be audited in a project-data/project-slm session. Known issues: wiki_repo/wiki_path
fields in topics_corporate.csv + topics_projects.csv are stale; v9 glossary terms not
applied to service-content CSVs.

### Next — operator-designated next phase
- Stage 6 promotion of content-wiki-documentation (86 commits) → Command Session
- Stage 6 promotion of woodfine-fleet-deployment → Command Session
- content-wiki-projects structural fixes (broken links, slug unification, wikilinks, index.es.md) → new session
- content-wiki-corporate structural fixes (wikilink density, index.es.md, bcsc_class) → new session
- service-content vocabulary/CSV audit → project-data or project-slm session

## Blockers
- (none)

## Gate check results (§14.2) — 2026-05-15
1. `grep -c "^## " overhaul-gemini-analysis.md` → **8** ✓
2. `test -f vocabulary-baseline.tsv` → **PASS** (307 terms; 9-column schema) ✓
3. `git status --porcelain` cluster root → **clean** (untracked = ZIP drafts + sub-clone dirs, both expected) ✓
4. `git status --porcelain` content-wiki-documentation → **clean** ✓
5. `git status --porcelain` woodfine-fleet-deployment → **clean** ✓
6. `grep -ri "jennifer" content-wiki-documentation/*/*.md` filenames → **0 hits** ✓
7. topic-prefixed systems files → **0** (all 12 renamed; slugs updated) ✓
8. Duplicate applications/ files → **removed** (4 files git rm'd; patterns/ canonical) ✓
9. Broken-link baseline → **19 broken links** (under 20 threshold; audit/baseline-broken-links.tsv committed) ✓

## Per-item tracker — sub-phase 2a (27 ZIP drafts)
| id | slug                                  | state   | sha | notes |
|----|---------------------------------------|---------|-----|-------|
| 01 | zip-topic-app-console-input-f12       | committed | 3e384b4 | new: applications/app-console-input.md |
| 02 | zip-topic-archetypes-and-coa          | committed | 8839e16 | new: services/archetypes-and-chart-of-accounts.md |
| 03 | zip-topic-bim-product-family          | committed | df6b313 | new: applications/bim-and-real-property-surfaces.md |
| 04 | zip-topic-competitive-positioning     | committed | d64f3de | new: reference/structural-positioning.md |
| 05 | zip-topic-compliance-disclosure       | committed | 9cb0d0b | new: governance/compliance-and-continuous-disclosure.md |
| 06 | zip-topic-deployment-patterns         | committed | 0772534 | new: patterns/deployment-patterns.md |
| 07 | zip-topic-design-system               | committed | bce93ed | merge: design-system/design-typography.md |
| 08 | zip-topic-hardware-research           | committed | 84c2571 | new: reference/hardware-reference.md |
| 09 | zip-topic-leapfrog-2030               | committed | a4fa0f5 | merge: architecture/leapfrog-2030-architecture.md |
| 10 | zip-topic-legal-ip-structure          | committed | 4b41242 | new: governance/legal-and-ip-structure.md |
| 11 | zip-topic-machine-based-authorization | committed | 20057ac | merge: architecture/machine-based-auth.md |
| 12 | zip-topic-microkernel-substrate       | committed | 9712244 | new: substrate/sel4-microkernel-substrate.md |
| 13 | zip-topic-os-console                  | committed | 808a97e | merge: systems/console-os.md |
| 14 | zip-topic-os-family-overview          | committed | e2380b3 | new: systems/os-family-overview.md |
| 15 | zip-topic-os-infrastructure-network   | committed | 0156aa9 | merge: systems/infrastructure-os.md + os-network-admin.md |
| 16 | zip-topic-os-mediakit                 | committed | 8afeabb | merge: systems/mediakit-os.md |
| 17 | zip-topic-os-orchestration            | committed | 626563d | merge: systems/os-orchestration.md |
| 18 | zip-topic-os-totebox                  | committed | c003764 | merge: systems/totebox-os.md |
| 19 | zip-topic-os-workplace                | committed | ee335c1 | merge: systems/os-workplace.md |
| 20 | zip-topic-pointsav-overview           | committed | 7220c49 | new: architecture/pointsav-overview.md |
| 21 | zip-topic-service-content             | committed | 813efe3 | merge: services/service-content.md |
| 22 | zip-topic-service-email-people        | committed | d330b3c | merge: services/service-email.md + service-people.md |
| 23 | zip-topic-service-slm                 | committed | eb376d5 | merge: services/service-slm.md |
| 24 | zip-topic-six-tier-sovereignty-matrix | committed | 0316dcc | new: architecture/six-tier-sovereignty-matrix.md |
| 25 | zip-topic-supply-chain-governance     | committed | cf727ad | new: architecture/five-stage-supply-chain.md |
| 26 | zip-topic-the-diode-standard          | committed | 9e084ce | new: architecture/diode-standard.md |
| 27 | zip-topic-three-layer-architecture    | committed | cb7ef71 | new: architecture/three-layer-architecture.md |
