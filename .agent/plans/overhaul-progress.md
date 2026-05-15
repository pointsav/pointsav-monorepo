---
schema: overhaul-progress-v1
plan: overhaul-documentation-pointsav-com.md
phase: 2
sub_phase: 2g-started
status: in-progress
safe_to_resume: true
unsafe_reason: ""
owner_engine: claude-code
last_updated: 2026-05-15T18:30:00Z
last_session_id: d64fccd3-6513-4802-a3dc-966cad0e754b-327278
---

## Last completed sub-task
- task: Style guide OPUS editorial rules update (sub-phase 2h)
- commit_sha: 6050420 (content-wiki-documentation)
- committed_at: 2026-05-15
- detail: style-guide-topic.md + .es.md + style-guide-guide.md + .es.md — Bloomberg four-paragraph
  lede structure, stand-alone PDF test, 75/25 register, CFO sentence test, named actors rule,
  "so what" discipline, internal governance vocabulary ban; broken style-guide-readme links removed;
  workspace path references replaced with plain prose

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

### Next — sub-phase 2i
- New articles for institutional readers: governance/procurement-overview.md and
  governance/security-overview.md (high-value landing pages identified by OPUS main page agent —
  not yet written; banker-relevant content gap)
- Broader article-by-article readability pass: substrate/ and architecture/ categories
  applying the Bloomberg + 75/25 + CFO sentence rules per article

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
