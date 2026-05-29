---
name: journal-artifact-discipline
description: Rules and schema for JOURNAL artifacts — peer-reviewed academic papers. Distinct from TOPIC-*, GUIDE-*, and PROSE-RESEARCH.
metadata:
  type: project
---

# JOURNAL Artifact Discipline

Peer-reviewed academic papers produced by the Foundry workspace. Applies to all
files matching `JOURNAL-*.draft.md` or `JOURNAL-*.stub.md` in any `drafts-outbound/`.

---

## Purpose and distinction

| Artifact type | Audience | Branding | Goal |
|---|---|---|---|
| TOPIC-* | Practitioners (internal wiki) | PointSav-branded | Explain existing knowledge |
| GUIDE-* | Operators | PointSav-branded | Enable a specific procedure |
| PROSE-RESEARCH | Project researchers | Internal | Scaffold academic thinking |
| **JOURNAL** | Peer reviewers + academic community | **No branding** | Test a falsifiable claim |

JOURNAL is the promotion target of PROSE-RESEARCH when the falsification programme
is stable. The two artifact types are not interchangeable — a JOURNAL manuscript
must be publishable without modification (after language-pass) to an external journal.

---

## Frontmatter schema (`foundry-journal-v1`)

```yaml
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft           # draft | under-review | accepted | published | archived
version: "0.1"
title: "..."
target_journal: "..."
target_publisher: "..."
impact_factor: ""      # fill before submission
alternate_venue: ""    # optional second-choice venue
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, USA"
    email: corporate.secretary@woodfinegroup.com
    orcid: ""           # required before submission
    credit_roles:
      - Conceptualization
      - Methodology
      - Writing – Original Draft
      - Writing – Review & Editing
subject_codes:          # ACM CCS / JEL / domain-specific
  - "..."
keywords:
  - "..."
bcsc_class: no-disclosure-implication
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: corporate.secretary@woodfinegroup.com
word_count_body: 0      # fill at each revision
word_count_target: 8500
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false  # set true only after full language pass
notes_for_editor: |    # optional; must use anonymisation-safe language only (see § below)
  ...
```

---

## notes_for_editor discipline

The `notes_for_editor:` frontmatter field must contain only anonymisation-safe language
that could appear in a cover letter or submission portal without compromising double-blind
review or disclosing internal process details.

**Permitted:** venue rationale in public terms; word-count status; data-collection
blockers described in general terms; citation and figure status in plain language.

**Forbidden:** all terms from the Forbidden vocabulary list below; internal phase
numbers (Phase 22, Phase 24B, etc.); archive names (project-gis, project-system, etc.);
internal process terms (Stage 6, F12, BRIEF, NEXT.md, spawned agents, Opus pipeline,
routing pass, etc.); internal file paths; scoresheet or convention names.

---

## Author rules

- Named natural persons only: **Peter M. Woodfine**, **Jennifer M. Woodfine**, **Mathew Woodfine**
- AI is a tool, not an author; disclosed per COPE 2024 in §18 of the manuscript
- ORCID IDs required before any journal submission (leave blank until obtained)
- Affiliation primary: `Woodfine Management Corp., New York, NY, USA`
- Affiliation alternative: `Independent Researcher, New York, NY, USA`
- Corresponding author: corporate.secretary@woodfinegroup.com

---

## Mandatory 22 structural sections

Every JOURNAL manuscript must contain all 22 sections. Stubs may have empty
section bodies; promotion to `under-review` requires all sections populated.

1. **Title** — no internal product names; no Do-Not-Use vocabulary
2. **Authors** — Peter M. Woodfine / Jennifer M. Woodfine / Mathew Woodfine
3. **Affiliations** — Woodfine Management Corp. or Independent Researcher
4. **Corresponding author email**
5. **ORCID IDs** (leave blank in `draft` state; mandatory for `under-review`)
6. **Keywords** — domain-appropriate; no internal terms
7. **Subject codes** — ACM CCS, JEL, CCS, PACS (domain-specific)
8. **Abstract** — 150–250 words; falsifiable claim sentence 1; method sentences 2–3; result quantified; no unhedged forward-looking claims
9. **Introduction** — research gap + three contributions
10. **Literature Review** — establishes gap; no structural positioning by competitor name
11. **Methodology**
12. **Results**
13. **Discussion** — composition-as-contribution framing
14. **Limitations** — explicit; hedged forward-looking claims match BCSC posture
15. **Conclusion**
16. **Formal Hypotheses** — H₁ (primary) + H₀ (null) minimum; H₂+ optional
17. **Falsification Programme** — test specifications; conditions for falsification
18. **AI Use Disclosure** — per COPE 2024; model identified; human editorial direction stated
19. **CRediT Contributor Roles** — conceptualization, methodology, software, writing at minimum
20. **Conflict of Interest Declaration**
21. **Funding Statement** — or "No external funding received"
22. **Data Availability Statement**

---

## Prose and typography standards

The target register is a traditional academic PhD dissertation updated for open-access
preprints: flowing proposition-based prose, numbered sections, formal captions on all
tables and listings, run-in italic for first-use terminology, and no bold structural
labels inside argument sections. The goal is accessibility — plain language wherever
precision permits, technical vocabulary only where precision requires it.

1. **Prose-first.** Numbered or bulleted lists are prohibited inside argument sections
   (Introduction, Background, Methodology, Results, Discussion, Limitations, Conclusion).
   Permitted in: formal Algorithm environments, Appendix reference tables, and
   measurement protocol steps where sequence is operationally mandatory.

2. **Bold usage.** Bold is reserved for hypothesis labels (H₁, H₀, H₂) and author names
   in the CRediT block. Never use bold as a paragraph-opening label for a concept. Use
   run-in italic instead: `*Term.* Prose continues in the same paragraph.`

3. **Section depth.** Maximum three decimal levels (e.g. 3.1.2). No H4/H5 subheadings.
   Further structural distinctions within a subsection use run-in italic paragraph leads.

4. **Tables.** Every table requires: (a) a formal caption `**Table N.** Caption.` on the
   line immediately preceding the table; (b) a parenthetical reference `(Table N)` in the
   body text that introduces the table. Tables containing no comparative or quantitative
   data should be converted to prose.

5. **Code listings.** Every code block requires a formal caption `**Listing N.** Caption.`
   on the line immediately preceding the fence. Surrounding prose narrates at the level of
   mechanism — not step-by-step ("First... Then..."). One explanatory sentence suffices.

6. **Symbols.** ✓/❌ in table cells → replace with "Yes"/"No" or convert the comparison
   to prose. Three-level quality scales (✓/~/ ✗) may be retained with a key.

7. **Results sub-labels.** Never `**Results — label.**` inside a running paragraph.
   Results flow as prose with parenthetical benchmark identifiers: `(B1)`, `(B3)`.

8. **Figures.** `**Figure N.** Caption.` on the line immediately following the figure
   reference or the figure block.

---

## Forbidden vocabulary

The following terms must never appear in any JOURNAL manuscript body text.
`forbidden_terms_cleared: false` until a dedicated language-pass session
has verified their absence.

**Internal product and system names:**
PointSav, Foundry, Totebox, Doorman, service-slm, app-console-*, app-workplace-*,
app-network-*, app-mediakit-*, moonshot-*, os-console, os-network-admin, os-privategit,
system-ledger (use the crate's public name instead), service-extraction, service-fs
(use the function/interface name instead), cluster-totebox-*, vault-privategit-*

**Internal process and governance terms:**
BCSC, Bloomberg standard, F12 (commit action), Task Claude, Root Claude, Master Claude,
Command Session, Totebox Session, Totebox Archive, drafts-outbound, Stage 6, Doctrine
claim #N, NEXT.md, cleanup-log, Scaffold-coded, Reserved-folder, sprint, foundry-draft-v1,
foundry-journal-v1, commit SHA, jwoodfine, pwoodfine, ps-administrator, mcorp-administrator,
Linguistic Air-Lock, Cognitive Forge, Data Vault

**Descriptive sovereign language (BCSC posture):**
Sovereign Data Foundation (descriptive use) — use "planned/intended" framing only
"sovereign" (as a descriptive adjective for infrastructure) — use "customer-controlled",
"customer-rooted", or "trustworthy" instead

**Generic banned marketing vocabulary:**
seamless, robust, cutting-edge, leverage, next-generation, groundbreaking, revolutionary,
transformative, game-changing, state-of-the-art (use specific claim + citation instead)

---

## Forward-looking language (BCSC posture)

Any deployment, production use, or roadmap item not yet delivered must carry
"planned", "intended", "may", or "target". Inherited from bcsc-disclosure-posture.md.
Applied identically in JOURNAL manuscripts as in all other Foundry artifacts.

This rule applies in both directions:
- Over-hedging already-delivered items (committed code, passing tests, benchmarks)
  with "planned/intended" is also an error — state facts as facts.
- Under-hedging forward-looking items (future deployments, roadmap features) is
  a disclosure violation — hedge them.

---

## PROSE-RESEARCH → JOURNAL promotion criteria

All six criteria must be met before changing `state` from `draft` (PROSE-RESEARCH)
to `draft` (JOURNAL):

1. Falsification programme stable: test specifications written; conditions for
   falsification explicit
2. Literature review establishes a gap: prior work surveyed; the gap is
   compositional or empirical; no citation more than 3 years old as the primary claim
3. Forbidden vocabulary scrubbed: language pass complete; `forbidden_terms_cleared: true`
4. Target journal named with rationale: impact factor confirmed; recent article
   alignment documented in notes_for_editor
5. Named-author byline confirmed: operator has approved author list and credit roles
6. Abstract conforms: ≤250 words; falsifiable claim sentence 1; method; quantified result

---

## Public posting requirements

When a JOURNAL manuscript with `state: draft` or `state: under-review` is posted to
a public URL (operator website, research portal, preprint server), two blocks are
mandatory in the paper body and two fields are mandatory in the frontmatter.

### Mandatory body blocks (insert after frontmatter `---`, before the title heading `# ...`)

**Block 1 — Preprint / WIP notice**
```markdown
> **Working Paper · Version X.X · YYYY-MM-DD · CC BY 4.0**
> This manuscript is a working draft. It has not been peer reviewed. Findings are
> preliminary and subject to revision without notice. Correspondence: corporate.secretary@woodfinegroup.com.
>
> *Cite as:* [full cite_as string from frontmatter]
```
Version from `version:` field; date from `language_pass_date:` or today if absent;
cite_as string from the `cite_as:` frontmatter field.

**Block 2 — Forward-Looking Statements advisory** (place immediately after Block 1)
```markdown
> **Forward-Looking Statements**
> Certain statements in this paper describe intended research directions, planned
> system capabilities, and anticipated outcomes. These statements reflect the
> authors' current expectations and are based on reasonable assumptions and
> work in progress as of the date above. Actual results, measurements, and
> findings may differ materially. Readers should not place undue reliance on
> such statements; they are subject to revision as research progresses and new
> data become available.
```

Neither block may contain: BCSC, securities, regulation, compliance, PointSav,
Foundry, or any term from the Forbidden vocabulary list above.

### Mandatory frontmatter fields (add at first public posting)

```yaml
preprint_posted: true
preprint_posted_date: YYYY-MM-DD
doi: ""                # fill when registered on Zenodo (ISO 26324); leave blank until then
license: "CC BY 4.0"  # Creative Commons Attribution 4.0 — de facto open-access preprint standard
cite_as: "Author, Given M. et al. (YYYY). Title. Working Paper vX.X, DD Month YYYY. Woodfine Management Corp., New York, NY."
revision_history:
  - version: "X.X"
    date: "YYYY-MM-DD"
    changes: "Description of changes this version"
```

**Versioning standards applied:**
- `version:` field follows Semantic Versioning (SemVer): MAJOR.MINOR — increment MINOR per writing pass,
  MAJOR per accepted publication
- `doi:` follows ISO 26324; register on Zenodo (free, CERN-operated) to mint a DOI per version plus
  a stable concept DOI across all versions; operator action required (Zenodo account)
- `license:` CC BY 4.0 is the standard for open-access academic preprints
- `cite_as:` provides a formatted citation string readers can copy; version and date must match `version:` and
  `language_pass_date:` / `preprint_posted_date:`
- `revision_history:` tracks version changes chronologically; one entry per version; newest last

**Git tagging (per public posting):**
Create an annotated tag immediately before the commit that records the public posting:
```bash
git tag -a "J<N>-v<X.X>-<YYYY-MM-DD>" -m "JOURNAL-<slug> v<X.X> public posting"
```
Tags are not pushed automatically — pushed separately when operator confirms external URL is live.

### Standard basis

Block 1 follows the arXiv/bioRxiv/SSRN working-paper convention. Block 2 follows
standard forward-looking-statements language used in economics, systems, and applied
research preprints. Together they satisfy the silent disclosure posture required for
publicly-posted research documents without referencing any regulatory framework by name.

---

## Submission workflow

1. `draft` → language pass → `forbidden_terms_cleared: true`
2. Operator reviews manuscript + byline
3. ORCID IDs obtained for all authors
4. Word count confirmed on target
5. Operator submits to journal; update `submission_status: submitted`
6. On decision: update `state` to `under-review` / `accepted` / `archived`

---

## Active JOURNAL manuscripts (updated 2026-05-28)

All papers target the top 3 venues in their domain. Primary venue listed; alternates in each
paper's frontmatter. All venues require double-blind peer review — `forbidden_terms_cleared`
pass is simultaneously the double-blind anonymization pass.

Canonical current versions are in `JOURNAL/` at the archive root.
Working drafts (may differ from canonical) are in `.agent/drafts-outbound/`.

| File | Domain | Primary target (top-3 strategy) | Ver | State |
|---|---|---|---|---|
| JOURNAL-retail-colocation-v0.1.draft.md | Economic geography | *Economic Geography* (Wiley, IF 7.2) | 0.3 | language-cleared; §7.2 pending Phase 24B |
| JOURNAL-trustworthy-systems-v0.1.draft.md | Systems CS | ASPLOS (ACM, 19.4% AR) | 0.1 | language-cleared; Bench #9 re-run pending |
| JOURNAL-aec-data-layers-v0.1.draft.md | AEC/construction | *Automation in Construction* (Elsevier, IF 12.0) | 0.2 | language-cleared; §6 Results pending coverage metrics |
| JOURNAL-private-network-v0.1.stub.md | Networking/ZTA | *IEEE TIFS* (IEEE, IF 9.65) | 0.2 | language-cleared; §4–§5 pending benchmarks |
| JOURNAL-totebox-orchestration-v0.1.stub.md | Systems/ML | MLSys (ACM, 22% AR) | 0.1 | stub — HOLD until J2 submitted |
| JOURNAL-desktop-environment-v0.1.stub.md | HCI/BIM | *ACM TOCHI* (ACM, Q1 HCI) | 0.2 | language-cleared; §6 Results pending user study |
