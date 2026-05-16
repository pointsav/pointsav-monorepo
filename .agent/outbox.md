---
mailbox: outbox
owner: task-project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-editorial cluster

---
from: totebox@project-editorial
to: command@claude-code
re: service-content CSV fix landed in wrong cluster — needs applying to project-intelligence
created: 2026-05-16T02:00:00Z
priority: high
status: pending
---

A project-editorial agent rebuilt the three topic-registry CSVs to match the actually-published
wiki articles (old content was stale placeholders). The agent found the files and committed in
`clones/project-infrastructure/` (commit b51f7ca9, cluster/project-infrastructure), but
**project-intelligence is the active development home for service-content**. The fix needs to
land there instead.

**Action required:**

1. Open a session in `clones/project-intelligence/` and verify whether
   `service-content/ontology/topics/` exists on that cluster's branch.

2. If it does: apply the same CSV content directly in project-intelligence and commit.

3. If it does not: determine the correct branch/cluster where service-content ontology is
   actively developed and apply the fix there.

4. The project-infrastructure commit (b51f7ca9) may be redundant or may need reverting —
   operator judgment on whether to promote, discard, or cherry-pick from it.

**The correct CSV content is:**

`topics_corporate.csv` — 5 rows (replacing 5 stale placeholders):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-direct-hold-framework,Direct-Hold Framework,corporate,content-wiki-corporate,topic-direct-hold-framework.md,active
topic-equity-transfer-model,Equity Transfer Model,corporate,content-wiki-corporate,topic-equity-transfer-model.md,active
topic-fiduciary-data-mandate,Fiduciary Data Mandate,corporate,content-wiki-corporate,topic-fiduciary-data-mandate.md,active
topic-interest-coverage-ratio,Interest Coverage Ratio,corporate,content-wiki-corporate,topic-interest-coverage-ratio.md,active
topic-redemption-elimination,Redemption Elimination,corporate,content-wiki-corporate,topic-redemption-elimination.md,active
```

`topics_projects.csv` — 17 rows (replacing 5 stale placeholders):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-asset-architecture-standard,Asset Architecture Standard,projects,content-wiki-projects,topic-asset-architecture-standard.md,active
topic-co-location-anchors,Co-location Anchors,projects,content-wiki-projects,topic-co-location-anchors.md,active
topic-co-location-index-canada,Co-location Index: Canada,projects,content-wiki-projects,topic-co-location-index-canada.md,active
topic-co-location-index-italy,Co-location Index: Italy,projects,content-wiki-projects,topic-co-location-index-italy.md,active
topic-co-location-index-mexico,Co-location Index: Mexico,projects,content-wiki-projects,topic-co-location-index-mexico.md,active
topic-co-location-index-nordics,Co-location Index: Nordics,projects,content-wiki-projects,topic-co-location-index-nordics.md,active
topic-co-location-index-poland,Co-location Index: Poland,projects,content-wiki-projects,topic-co-location-index-poland.md,active
topic-co-location-index-spain,Co-location Index: Spain,projects,content-wiki-projects,topic-co-location-index-spain.md,active
topic-co-location-index-us,Co-location Index: United States,projects,content-wiki-projects,topic-co-location-index-us.md,active
topic-co-location-intelligence-overview,Retail Co-location Intelligence Overview,projects,content-wiki-projects,topic-co-location-intelligence-overview.md,active
topic-co-location-methodology,Retail Co-location Methodology,projects,content-wiki-projects,topic-co-location-methodology.md,active
topic-co-location-ranking-system,Retail Co-location Ranking System,projects,content-wiki-projects,topic-co-location-ranking-system.md,active
topic-regional-market-matrix,Regional Market Matrix,projects,content-wiki-projects,topic-regional-market-matrix.md,active
topic-site-ledger-integration,Site Ledger Integration,projects,content-wiki-projects,topic-site-ledger-integration.md,active
topic-tier-index-europe,Co-location Tier Index: Europe,projects,content-wiki-projects,topic-tier-index-europe.md,active
topic-tier-index-north-america,Co-location Tier Index: North America,projects,content-wiki-projects,topic-tier-index-north-america.md,active
topic-zoning-acquisition-rules,Zoning Acquisition Rules,projects,content-wiki-projects,topic-zoning-acquisition-rules.md,active
```

`topics_documentation.csv` — 1 confirmed row (4 other slugs do not exist in the wiki;
full population of ~240 documentation articles is a separate future task):
```
topic_id,title,domain,wiki_repo,wiki_path,active_state
topic-doorman-protocol,Doorman Protocol,documentation,content-wiki-documentation,architecture/doorman-protocol.md,active
```

`Domains.json` — vocabulary-clean, no changes needed.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 pending — content-wiki-corporate + content-wiki-projects + content-wiki-documentation
created: 2026-05-16T01:00:00Z
priority: normal
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
commits: 1c9cee3→769980b (docs, 86), c65be14→53a3169 (corporate, 3), 1c1e48b→420e8cf (projects, 4)
---

All three documentation wikis are ready for Stage 6 promotion to canonical. Run
`bin/promote.sh` for each sub-clone in the project-editorial cluster.

**content-wiki-documentation** — 86 unpromoted commits (sub-phases 2i, 2j, Phase 2 final cleanup)
**content-wiki-corporate** — 3 unpromoted commits (bcsc_class, index.es.md, wikilink density)
**content-wiki-projects** — pending commit completion (slug unification, link repair, index.es.md, wikilink density); will be clean before this message is picked up

Note: service-content vocabulary refresh (separate message below) is a soft dependency —
the wikis are editorially ready but search/DataGraph accuracy depends on service-content
CSV repair being completed by project-data/project-slm session.

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: service-content vocabulary refresh needed — route to project-data/project-slm
created: 2026-05-16T00:30:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
routed_to: project-intelligence inbox (msg-id: project-intelligence-20260516-service-content-vocab-refresh)
---

Three wikis (documentation, corporate, projects) are being prepared for relaunch.
The service-content seed layer is stale and must be refreshed before data flows
correctly to all three wikis. This work is outside the project-editorial cluster scope
and must be picked up in a project-data or project-slm session.

**Known issues (from editorial-reference-plan-2026-05-08 + outbox-archive):**

1. **`wiki_repo` field stale in topic CSVs** — `topics_corporate.csv` and
   `topics_projects.csv` have incorrect `wiki_repo` values. Must be updated to
   point at `content-wiki-corporate` and `content-wiki-projects` respectively.

2. **`wiki_path` format stale** — CSV entries use old path format (`topics/topic-*.md`).
   Must be updated to `<category>/<slug>.md` per the current content-contract.

3. **Glossary v9 terms not applied to service-content CSVs** — the wiki glossaries
   received a v9 vocabulary pass; the DataGraph CSVs (`service-content/ontology/`) have
   not. Every downstream DataGraph consumer is reading old vocabulary.

4. **~251 articles unclassified; ~72 GUIDEs unregistered** in the DataGraph topic
   registry. These are editorial gaps that may affect wiki discovery and search.

5. **Domains.json / domain seed files** — audit for Bloomberg vocabulary violations
   (same rules as the wiki sweep: no "Foundry", no internal paths, no "Doctrine claim").
   The project-editorial OPUS audit could not inspect these files (monorepo sub-clone
   in project-editorial cluster is empty).

**Action requested from Command Session:**
Route this message to the correct Totebox owner for `pointsav-monorepo/service-content/`
(likely project-data or project-slm cluster). Include the five items above as the
scope definition. All three wiki relaunches are gated on items 1–3 being resolved.

Reference: OPUS audit report is logged in
`clones/project-editorial/.agent/plans/overhaul-progress.md` (2026-05-15 entry).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 pending — content-wiki-documentation sub-phase 2j complete
created: 2026-05-15T23:45:00Z
priority: normal
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: superseded by consolidated Stage 6 message above; all 86 docs commits promoted in same pass
---

Sub-phase 2j (Bloomberg vocabulary sweep) is complete across all categories in
`content-wiki-documentation`. Commits on staging branch `cluster/project-editorial`:

- Batches 1–5 (services, systems, infrastructure/patterns, design-system, reference) — earlier
- Batch 6 (9e891c8, Peter): substrate/ EN + patterns/pairing-as-permission + applications/
- Batch 7 (e899768, Jennifer): substrate/ ES (18 files)
- Batch 7b (96a6379, Peter): 4 residual substrate files
- Progress tracker update (fade035a, Jennifer): plans: 2j complete

Also pending from sub-phase 2i (architecture/ scrub): commits from prior sessions.

**Action requested:** run `bin/promote.sh` for `content-wiki-documentation` to push
all staging commits through to canonical `origin` (pointsav/content-wiki-documentation).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: LEGAL corrections confirmed — route to ps-administrator for factory-release-engineering commit
created: 2026-05-15T20:30:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-15
commit: 5bbed79 (factory-release-engineering main)
---

Three license corrections verified against live files in `vendor/factory-release-engineering/licenses/`.
All three issues confirmed real. Route to ps-administrator for admin-tier commit per CLAUDE.md §8.

**Issue 1 — MIT.txt line 3 (factual error, highest priority):**
Current: `Copyright (c) 2026 PointSav Digital Systems`
Corrected: `Copyright (c) 2026 Woodfine Capital Projects Inc.`
Rationale: LICENSE-MATRIX.md §1.1 assigns copyright to WCP Inc. Every other custom license
in the repo names WCP Inc. PointSav Digital Systems is a subsidiary brand, not the IP holder.

**Issue 2 — PointSav-ARR.txt §8 survival clause:**
Current: `Sections 3, 6, 7, 9, and 10 survive termination.`
Corrected: `Sections 3, 4, 6, 7, 9, and 10 survive termination.`
Rationale: Section 4 is the TRADEMARK clause. It must survive termination to prevent former
licensees from using Woodfine Marks after license ends. Omission was an oversight.

**Issue 3 — PointSav-ARR.txt §3 security-researcher note:**
Current: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements.`
Corrected: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements for uses beyond Section 2.`
Rationale: §2(c) expressly grants security researchers "good-faith academic, journalistic, or
security-research contexts with attribution." Current §3 text could be read as cancelling that
grant. The insertion of "for uses beyond Section 2" makes the scope explicit. Issue 3 is a
clarification; Issues 1 and 2 are unambiguous errors.

Source draft: `clones/project-knowledge/.agent/drafts-outbound/legal-factory-release-engineering-license-corrections.draft.md`

— totebox@project-editorial

---
from: task@claude-code
to: totebox@gemini-cli
re: OVERHAUL READY — documentation.pointsav.com two-phase plan committed; Phase 0 is yours
created: 2026-05-14T20:30:00Z
priority: high
status: stale
---

The corpus overhaul plan is committed and the progress tracker is initialised.
You are the Phase 0 + Phase 1 engine. Read these files before anything else:

1. `.agent/plans/overhaul-documentation-pointsav-com.md` — master plan (15 sections, 1143 lines)
2. `.agent/plans/overhaul-progress.md` — progress tracker (Phase 0, owner: gemini-cli)

**Your Phase 0 task (before any analysis):**
Extract `vocabulary-baseline.tsv` from all three wiki repos + runtime surfaces (§13.1).
Covers: `content-wiki-documentation/`, `content-wiki-corporate/`, `content-wiki-projects/`
plus `pointsav-monorepo/service-content/seeds/Domains.json` and `ontology/*.csv`.
Columns: `term | definition | wiki_slug | source | glossary_status | bilingual_status | in_documentation | in_corporate | in_projects`
Commit to `.agent/plans/vocabulary-baseline.tsv`.

**Your Phase 1 task (after Phase 0):**
Produce `overhaul-gemini-analysis.md` (9 sections — see §8.3) + `domain-map.tsv` (§15.2).
Execute 6 light-work commits (§8.4). Then write gate-open inbox message (§14.2 all 9 checks).
Set `overhaul-progress.md` `status: gate-open`, `owner_engine: ""` before closing.

**Claude Code does NOT touch Phase 2 until your gate-open message lands.**

Flags already resolved with operator (see §4). Stop conditions in §12.
Session start ritual for this archive: inbox → NOTAM → rules → plans README → session-start → overhaul plan → progress tracker → recovery check (§14.3).

— task@claude-code
