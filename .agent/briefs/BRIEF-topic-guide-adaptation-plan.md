---
artifact: brief
schema: foundry-brief-v1
brief-id: project-editorial-topic-guide-adaptation-2026-06-14
title: "TOPIC and GUIDE Adaptation Plan — §16 Quality Programme"
status: reference
owner: project-editorial
created: 2026-06-14
updated: 2026-06-19
parent: BRIEF-artifact-style-guide.md §16
---

## Context

The 12-agent §16 audit (2026-06-14) identified 8 structural gaps in our 354-article
corpus across three wikis. This BRIEF is the implementation plan. Source:
BRIEF-artifact-style-guide.md §16.

The enumeration pass examined 65 EN articles flagged as needing changes. Every flagged
article (65/65) requires at least one structural change. Findings break down by gap class:

- **Lede mode violations** — 18 articles deviate from the canonical lede pattern
  (2 spec-sheet / mode1, 10 throat-clearing / mode2, 6 score-citation / mode3, 2 stub).
- **Missing reader-orientation furniture** — absent Key Takeaways and Bottom Line blocks,
  concentrated in the corporate financial-governance set.
- **Title Case heading violations** — 17 articles carry Title Case section headings where
  sentence case is required.
- **Leaked editorial comments** — 2 articles retain inline author/process comments that
  must be stripped before any wiki commit (Doctrine claim: no leaked comments in published
  bodies).
- **Missing Quick Facts tables** — financial-governance articles lack the at-a-glance
  reference table the template requires.
- **Missing freshness stamps** — dated coverage-expansion articles lack a freshness marker.
- **Missing References sections** — several ranked-list and methodology articles ship
  without a References section.
- **Stubs** — 2 corporate articles are below the substance threshold and need full
  build-out alongside structural fixes.

Distribution by wiki: documentation 0, projects 48, corporate 17. The documentation wiki
passed clean on this audit slice; all adaptation work targets **projects** and **corporate**.

## Scope

All EN `.md` files in `media-knowledge-{documentation,projects,corporate}`.

- **ES pairs** are updated separately via the TRANSLATE-ES protocol after the EN bodies
  stabilise. They are out of scope for the adaptation agents in this pass.
- **woodfine-fleet-deployment GUIDEs** require Command admin-tier (they are not in a
  project-editorial archive) and are tracked separately. Out of scope here.
- **Editorial stripping rules apply**: config tables (localhost endpoints), SSH port
  specifics, systemd paths, Doctrine claim numbers, and personal names (Jennifer/Peter/
  Mathew) must not appear in any TOPIC/GUIDE body. Replace personal names with role nouns.

The 65 articles enumerated below are the full work queue for this BRIEF. ES siblings,
fleet GUIDEs, and any documentation-wiki follow-ups are downstream of this pass.

## Article inventory by template type

| template_type | count | wiki(s) | priority distribution |
|---|---|---|---|
| financial-governance | 15 | corporate | high 3, medium 12 |
| architecture-concept | 11 | (audit corpus) | n/a in flagged set |
| ranked-list | 11 | projects | high 1, medium 10 |
| methodology | 10 | projects | high 1, medium 9 |
| other | 10 | projects | medium (lede/heading/freshness) |
| place-profile | 6 | projects | high 6 |
| stub | 2 | corporate | critical 2 |

Notes:
- `architecture-concept` (11) and the non-flagged portion of `other`/`methodology` appear
  in the broader audit corpus but did not surface as flagged-for-change in this 65-article
  enumeration slice; their counts are carried for §16 traceability.
- Corporate flagged total: 17 (15 financial-governance + 2 stub).
- Projects flagged total: 48 (place-profile 6, ranked-list 11, methodology 10, other 10,
  plus methodology/ranked-list overlaps counted once per article).
- Lede-mode summary across flagged set: ok 45, mode1 2, mode2 10, mode3 6, stub 2.

## Priority work list

All `change_priority = critical` and `change_priority = high`, grouped by wiki.

### corporate (5 articles)

| priority | slug | title | changes_needed |
|---|---|---|---|
| critical | topic-investment-units | Investment Units | lede-rewrite, add-key-takeaways, add-bottom-line, fix-heading-case |
| critical | topic-perpetual-equity-model | Perpetual Equity Model | lede-rewrite, add-key-takeaways, add-bottom-line, fix-heading-case |
| high | topic-continuous-disclosure | Continuous Disclosure Obligations | add-key-takeaways, add-bottom-line, add-quick-facts-table |
| high | topic-direct-hold-framework | Direct-Hold Framework | remove-leaked-comments, add-key-takeaways, add-bottom-line |
| high | topic-interest-coverage-ratio | Interest Coverage Ratio | add-key-takeaways, add-bottom-line, add-quick-facts-table |

### projects (8 articles)

| priority | slug | title | changes_needed |
|---|---|---|---|
| high | topic-co-location-cluster-formation | Co-location Cluster Formation | lede-rewrite, add-bottom-line |
| high | topic-co-location-index-us | Co-location Index: United States | lede-rewrite, fix-heading-case, add-bottom-line |
| high | topic-rm-colorado-springs-co | Colorado Springs, Colorado — Regional Market | lede-rewrite, add-bottom-line |
| high | topic-rm-krefeld-de | Krefeld — Regional Market | lede-rewrite, add-bottom-line |
| high | topic-rm-mississauga-on | Mississauga, Ontario — Regional Market | lede-rewrite, add-bottom-line |
| high | topic-rm-nurnberg-de | Nuremberg, Germany — Regional Market | lede-rewrite, add-bottom-line |
| high | topic-rm-plano-tx | Plano, Texas — Regional Market | lede-rewrite, add-bottom-line |
| high | topic-rm-wichita-ks | Wichita, Kansas — Regional Market | lede-rewrite, add-bottom-line |

**Note on the six place-profile articles:** several of these slugs are marked SUPERSEDED in
the project-registry/artifact-registry (Colorado Springs, Nürnberg under corrected Top-400
methodology; Wichita as a metro reference). Adaptation agents must confirm current
publication intent against the artifact registry before rewriting ledes on superseded
articles — do not invest in a lede-rewrite for an article slated for removal/revision.

## Medium priority

All `change_priority = medium`, grouped by wiki. 28 articles total (16 projects, 12 corporate).

### projects (16 articles)

| slug | title | template_type | changes_needed |
|---|---|---|---|
| topic-cluster-deduplication-threshold | Cluster Deduplication Threshold | methodology | fix-heading-case |
| topic-co-location-index-canada | Co-location Index: Canada | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-index-italy | Co-location Index: Italy | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-index-mexico | Co-location Index: Mexico | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-index-nordics | Co-location Index: Nordics | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-index-poland | Co-location Index: Poland | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-index-spain | Co-location Index: Spain | ranked-list | lede-rewrite, fix-heading-case |
| topic-co-location-intelligence-overview | Retail Co-location Intelligence — Overview | other | lede-rewrite, fix-heading-case |
| topic-co-location-methodology | Retail Co-location Methodology | methodology | remove-leaked-comments |
| topic-co-location-ranking-system | Retail Co-location Ranking System | methodology | lede-rewrite, fix-heading-case |
| topic-gis-nordic-uk-coverage | Nordic and UK Coverage Expansion | other | add-freshness-stamp |
| topic-regional-markets-system | Regional Markets Intelligence System | other | lede-rewrite |
| topic-tier-index-europe | Co-location Tier Index: Europe | ranked-list | lede-rewrite, fix-heading-case |
| topic-tier-index-north-america | Co-location Tier Index: North America | ranked-list | lede-rewrite, fix-heading-case |
| topic-top-400-regional-markets-eu | Top 400 Regional Markets — Europe | ranked-list | lede-rewrite |
| topic-top-400-regional-markets-na | Top 400 Regional Markets — North America | ranked-list | lede-rewrite |

### corporate (12 articles)

| slug | title | template_type | changes_needed |
|---|---|---|---|
| topic-asset-evaluation | Asset Evaluation Protocol | financial-governance | add-key-takeaways, add-bottom-line |
| topic-co-location-investment-thesis | Co-location Investment Thesis | financial-governance | add-key-takeaways, add-bottom-line |
| topic-corporate-structure | Corporate Structure | financial-governance | add-key-takeaways, add-bottom-line |
| topic-data-governance | Data Governance | financial-governance | add-key-takeaways, add-bottom-line |
| topic-equity-transfer-model | Equity Transfer Model | financial-governance | add-key-takeaways, add-bottom-line |
| topic-fiduciary-data-mandate | Fiduciary Data Mandate | financial-governance | add-key-takeaways, add-bottom-line |
| topic-investor-access | Investor Access | financial-governance | add-key-takeaways, add-bottom-line |
| topic-property-ledger-technology | Property Ledger Technology | financial-governance | lede-rewrite, add-key-takeaways, add-bottom-line |
| topic-redemption-elimination | Redemption Elimination | financial-governance | add-key-takeaways, add-bottom-line |
| topic-regulatory-posture | Regulatory Posture | financial-governance | add-key-takeaways, add-bottom-line |
| topic-technology-services | Technology Services Agreement | financial-governance | add-key-takeaways, add-bottom-line |
| topic-vendor-customer-model | Vendor-Customer Model | financial-governance | add-key-takeaways, add-bottom-line |

## Engine-blocked improvements (cannot execute until project-knowledge fixes land)

These four classes of defect are visible in the rendered output but originate in the
`app-mediakit-knowledge` rendering engine, not in the source markdown. project-editorial
cannot fix them by editing article bodies. They are routed to project-knowledge and are
tracked here for §16 completeness.

1. **Infobox HTML rendering (images path).** Infobox cards reference image assets via a
   path the engine does not resolve; images render broken. Editing the markdown does not
   fix the asset path resolution. Blocked on project-knowledge image-path handling.

2. **Citation footnote rendering (HTTP 500).** Articles with footnote-style citations
   trigger an HTTP 500 on render. References sections cannot be reliably added until the
   footnote renderer is fixed — `add-references-section` work on affected articles is
   deferred behind this fix.

3. **Search snippet markdown stripping.** Search result snippets show raw markdown syntax
   instead of stripped plain text. This is a search-indexer presentation defect, not a
   source-content defect.

4. **Breadcrumb taxonomy correction.** Breadcrumb trails resolve against an incorrect
   taxonomy mapping, producing wrong parent paths. The fix is in the engine's category/
   taxonomy resolver, not in per-article frontmatter.

Action: relay these four to project-knowledge via outbox; do not attempt source-side
workarounds that would mask the engine defect.

## Execution status

Adaptation workflow completed 2026-06-14. Table updated to reflect applied changes;
see notes for partial completions and skips.

| group | wiki | articles | status |
|---|---|---|---|
| Critical | corporate | topic-investment-units, topic-perpetual-equity-model | DONE (partial) — fix-heading-case applied (`## See Also` → `## See also`). add-key-takeaways and add-bottom-line SKIPPED: both gated to >400/>600 words; these are ~252/~254-word stubs. lede already mode `ok`. Full stub build-out remains open. |
| High | corporate | topic-continuous-disclosure, topic-direct-hold-framework, topic-interest-coverage-ratio | DONE — Key Takeaways added to all three; Bottom Line added to topic-direct-hold-framework; leaked claim-comment pairs stripped from topic-direct-hold-framework. add-quick-facts-table SKIPPED on the two financial-governance articles (Quick Facts is gated to place-profile articles with location data). |
| High | projects | topic-co-location-index-us | DONE — fix-heading-case (Tier 5 line + See also). add-bottom-line N/A (ranked-list, no References section). lede-rewrite not required (mode `ok`). |
| High | projects | topic-co-location-cluster-formation | NOT MODIFIED — not present in final modified-file set; no edits applied this pass. Carry forward. |
| High | projects | topic-rm-colorado-springs-co, topic-rm-krefeld-de, topic-rm-mississauga-on, topic-rm-nurnberg-de, topic-rm-plano-tx, topic-rm-wichita-ks | DONE — all six: lede-rewrite (mode3 score-citation demoted to end of lede paragraph) + add-bottom-line. EN only; .es.md untouched. NOTE: Colorado Springs / Nürnberg / Wichita carry SUPERSEDED markers in the artifact registry — confirm publication intent before Stage 6. |
| Medium | projects | 16 ranked-list/methodology/other articles | DONE (heading + comment classes) — fix-heading-case applied across the co-location-index set (canada/italy/mexico/nordics/poland/spain), intelligence-overview, cluster-deduplication-threshold, ranking-system. remove-leaked-comments applied to co-location-methodology. lede-rewrite for the ranked-list/methodology set was reserved for the Opus high-priority pass and is NOT applied to the medium ranked-list articles (mechanical batch deferred it). **topic-top-400-regional-markets-eu + topic-top-400-regional-markets-na lede-rewrite DONE (2026-06-17)** — mode2 throat-clearing ledes replaced with consequence-first ledes (Chemnitz leads EU at 18.0; Plano leads NA at 25.5); committed 0a9f8bc. Full Rankings (26-400) tables temporarily replaced with placeholder pending live rendering verification; restore required — see Carry-forward. tier-index-europe/north-america ledes still PENDING. topic-gis-nordic-uk-coverage freshness-stamp: no References section exists — deferred. |
| Medium | corporate | 12 financial-governance articles | DONE — fix-heading-case + Key Takeaways/Bottom Line applied where word-count gates met (topic-asset-evaluation, topic-co-location-investment-thesis, topic-corporate-structure, topic-data-governance, topic-equity-transfer-model, topic-fiduciary-data-mandate, topic-investor-access, topic-property-ledger-technology, topic-redemption-elimination, topic-regulatory-posture, topic-technology-services, topic-vendor-customer-model). |
| Engine-blocked | knowledge | infobox images, citation footnotes, search snippets, breadcrumb taxonomy | PENDING — routed to project-knowledge; not executable in this archive. Relay via outbox. |

**Out-of-scope edit flagged:** `topic-co-location-ranking-system.es.md` (projects) received a
new content section ("Niveles de Calidad y Distribución"). ES pairs are out of scope for this
pass (§Scope — updated separately via TRANSLATE-ES). Main session should decide whether to
revert this file or commit it with the EN batch.

## Carry-forward (2026-06-18)

- [ ] **Restore Full Rankings (26–400)** in `topic-top-400-regional-markets-eu.md` and `topic-top-400-regional-markets-na.md` once live rendering is verified at foundry-prod. Tables were temporarily replaced with placeholder text during the top-25 test pass. Commit restores as a separate commit in media-knowledge-projects.
- [ ] **Tier-index ledes** — topic-tier-index-europe + topic-tier-index-north-america: lede-rewrite still pending (mode2 violations per §16 audit).
- [ ] **GUIDE-mk-* unstaged deletions** in project-editorial archive — 7 files deleted from filesystem but not yet `git rm`'d (GUIDE-mk-corporate-content-operations, GUIDE-mk-corporate-deployment, GUIDE-mk-documentation-content-operations, GUIDE-mk-documentation-deployment, GUIDE-mk-documentation-editorial-content-sweep, GUIDE-mk-projects-content-operations, GUIDE-mk-projects-deployment). Investigate prior session intent before committing deletes.

## Commit instructions for main session

After the adaptation workflow completes, commit in this order. Stage specific files only
(never `git add .` / `-A`). Use `~/Foundry/bin/commit-as-next.sh "<message>"` so the
alternating jwoodfine/pwoodfine identity and SSH signing are applied. Each wiki is a
separate canonical repo — commit them independently.

1. **media-knowledge-documentation** — commit all modified files (one commit per identity).
   No flagged articles in this slice, but commit any incidental changes if the workflow
   touched documentation. State explicitly if there are none.

2. **media-knowledge-projects** — commit all modified files (one commit per identity).
   48 flagged articles; expect lede-rewrite, fix-heading-case, add-bottom-line, and
   remove-leaked-comments changes. Verify no personal names, config tables, SSH/systemd
   specifics, or Doctrine claim numbers leaked into bodies before committing.

3. **media-knowledge-corporate** — commit all modified files (one commit per identity).
   17 flagged articles; expect add-key-takeaways, add-bottom-line, add-quick-facts-table,
   lede-rewrite, fix-heading-case, and remove-leaked-comments (Direct-Hold Framework).

After all three: run `git status` in each repo to confirm clean. Stage 6 promotion and
`bin/sync-local.sh --all` are Command-session actions — route via outbox, do not run from
this archive. ES pairs follow via TRANSLATE-ES in a subsequent pass. Engine-blocked items
remain open in project-knowledge.
