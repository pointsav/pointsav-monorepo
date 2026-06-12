---
artifact: brief
status: active
title: "Artifact Style Guide — Internal/External Voice Discipline"
created: 2026-06-11
updated: 2026-06-11
owner: project-editorial
covers: TOPIC, GUIDE, JOURNAL, COMMS, TEXT
---

# Artifact Style Guide — Internal/External Voice Discipline

## Purpose of this BRIEF

This BRIEF is the dialogue space. The artifacts it governs are the products.

Nothing in this BRIEF appears in a committed artifact. Everything in the artifacts
could appear on a publisher's website without embarrassment or context. That is the
test.

This BRIEF is a living reference. Every artifact that passes through project-editorial
is checked against it. When a new leakage pattern is found, it is added here. When a
known violation is fixed, §7 is updated.

---

## §1 — The core principle

**BRIEFs = internal dialogue.** Open analysis, decisions, operator instructions, AI
reasoning, options considered and rejected, process notes, sprint planning, phase
numbering, cluster names, inbox discussions — all of this belongs inside a BRIEF or
inside the session's conversation. It does not leave.

**ARTIFACTs = external product.** A TOPIC is a wiki article a customer reads. A GUIDE
is an operational manual the operator follows. A JOURNAL is a peer-reviewed paper a
scientist evaluates. None of these readers knows how the artifact was made, who
discussed it, what the internal phase number was, or what the AI session looked like.
They should not be able to tell.

The artifact is the clean output of the process. The process is invisible.

A reader who has never seen a BRIEF, never heard of Foundry, and never seen this
session's conversation should be able to read any committed artifact and find it
completely self-contained and professionally produced.

---

## §2 — Universal forbidden patterns (all artifact types)

These patterns must not appear in any committed artifact — TOPIC, GUIDE, JOURNAL,
COMMS, or TEXT — regardless of `bcsc_class` unless an exception is noted.

### 2.1 Internal phase and build identifiers

| Pattern | What it is | Replace with |
|---|---|---|
| `Phase N` | Internal build/sprint counter | Dataset description, date, or version |
| `Phase N+Change B` | Specific internal build designator | Dataset description (e.g. "May 2026 dataset") |
| `Change B` build designators | Internal sub-phase | Remove; describe the actual change |
| `Sprint N` | Internal sprint counter | Remove entirely |

**Why:** These numbers are meaningful only inside the workspace. A reader sees
"Phase 23+Change B" and has no idea what it means. Worse, it implies internal
process that a clean product would not expose.

**Example — leak:**
> "The Phase 23+Change B dataset (actual counts: T1=1,746, T2=2,726) confirms..."

**Example — fixed:**
> "The May 2026 dataset (actual counts: T1=1,746, T2=2,726) confirms..."

### 2.2 Project archive names

| Pattern | Replace with |
|---|---|
| `project-editorial` | Remove entirely or say "editorial review" |
| `project-gis`, `project-intelligence`, `project-data` | Remove; describe the function |
| `project-*` in any metadata field | Use role noun: `editorial`, `engineering`, `data` |

**In frontmatter:** `editor: project-editorial` → `editor: editorial`

**In revision history:** `"Language pass (project-editorial)"` → `"Language pass"`

**Why:** Archive names are workspace-internal routing labels. A peer reviewer or
customer does not need to know that project-editorial conducted a language pass.

### 2.3 `.agent/` system paths and coordination vocabulary

| Pattern | Replace with |
|---|---|
| `.agent/outbox.md`, `.agent/inbox.md` | "coordination file" or remove |
| `.agent/drafts-outbound/` | "staging directory" or remove |
| `.agent/rules/`, `.agent/engines/`, `.agent/manifest.md` | Remove |
| `outbox`, `inbox` (as mailbox terms) | "coordination channel" or "notification" |
| `NOTAM` | Remove; describe the actual advisory content |
| `apprenticeship queue`, `apprenticeship corpus` | Remove; describe the function |

**Exception:** `customer-internal` GUIDEs for workspace administration (foundry-workspace/,
vault-privategit-source/) may reference internal infrastructure by *function name* (e.g.
"the workspace configuration directory") but not by internal path (`.agent/engines/`).

### 2.4 Internal role and tier names

| Pattern | Replace with |
|---|---|
| `P1 administrator`, `P2 contributor`, `P3 viewer` | "workspace administrator", "contributor" |
| `Tier A`, `Tier B`, `Tier C` (Doorman tiers) | "primary inference", "secondary inference" |
| `Root Claude`, `Task Claude`, `Master Claude` | Remove; describe the action |
| `Command Session`, `Totebox Session`, `Totebox Archive` | Remove; describe the scope |
| `jwoodfine`, `pwoodfine`, `ps-administrator`, `mcorp-administrator` | Remove entirely |

### 2.5 Implicit brief references

| Pattern | Replace with |
|---|---|
| "the research brief specified..." | "The methodology was designed to..." or cite the paper's own §N |
| "per our earlier discussion..." | Remove; state the decision directly |
| "as noted in the BRIEF..." | Remove; restate the point |
| "per the operator's request..." | Remove; state what was done |
| "the specification called for..." | Remove or cite a public standard |

**Why:** These phrases reveal that the artifact was produced in response to an internal
specification. A published product does not carry its production history in the text.

### 2.6 Meta-commentary about the artifact itself

| Pattern | Replace with |
|---|---|
| "This article covers..." | Begin with the content directly |
| "This guide was prepared to..." | Begin with the operational procedure |
| "This section provides an overview of..." | Begin with the overview |
| "The following table shows..." | Write the sentence that the table supports |
| "This document was drafted as part of..." | Remove entirely |

**Why:** A published book does not begin chapters with "This chapter covers...". An
institutional article does not open with "This article was prepared following...".
The artifact speaks for itself.

### 2.7 "We" voice — the internal authorship tell

Academic "we" is acceptable in JOURNALs: "We evaluated three approaches..." (standard
academic collaborative voice; the authors are the subject).

**Forbidden** in all artifact types: "we" meaning the AI session and operator together:
- "We discussed this approach and decided..."
- "As we noted earlier in our conversation..."
- "We found through iterative testing that..." (if "we" = the session, not the authors)
- "We use on-demand inference here" (if "we" = the Foundry workspace)

**Test:** Substitute "The authors" for "we". If the sentence still makes sense as published
research or documentation, it is legitimate. If it sounds like a description of a private
conversation, it is internal dialogue.

### 2.8 Draft-state placeholder text

| Pattern | Replace with |
|---|---|
| `TBD` | Write the content, or remove the sentence |
| `Pending data from project-X` | Remove; flag in the BRIEF, not in the article |
| `[to be completed]`, `[placeholder]` | Write the content, or remove the section |
| `See BRIEF for details` | Remove; surface the detail in the article |

**Exception:** A `pre-build` status article may carry forward-looking language ("planned",
"intended") for content not yet built. This is BCSC-posture language, not a placeholder.
The distinction: "planned" describes a real future state; "[TBD]" describes an unfilled
slot in the draft.

---

## §3 — TOPIC-specific rules

TOPICs are public wiki articles served at `documentation.pointsav.com` or the woodfine
knowledge platform. `bcsc_class: public-disclosure-safe` is the strictest standard.

**Frontmatter hygiene:**
- `editor:` — role noun only: `editorial`, `engineering`, `data`. Never an archive name.
- `status: pre-build` is acceptable for content describing planned features; all such
  sentences carry "planned", "intended", or "may" language (BCSC posture).
- `research_provenance:` — if this field references an internal BRIEF, replace with
  "internal specification" or the relevant external standard.

**Body text:**
- All §2 patterns apply without exception.
- External dataset references: use date and count, not build phase identifier.
  "The June 2026 co-location dataset" not "the Phase 23+Change B build".
- Bilingual pairs (`.es.md`) must receive the same leakage pass — translations of
  internal phrases are still internal.

---

## §4 — GUIDE-specific rules

GUIDEs are operational manuals in woodfine-fleet-deployment. Two tiers apply:

**`bcsc_class: public-disclosure-safe` and `customer-woodfine` guides:**
Same standard as TOPICs. No internal vocabulary. Operational steps describe
what the operator does, not how the system was built.

**`bcsc_class: customer-internal` guides (foundry-workspace/, vault-privategit-source/):**
These are workspace-administration guides for the operator themselves. They may:
- Reference internal infrastructure by *function* ("the workspace configuration", "the
  staging directory", "the binary ledger")
- But not by *internal path* (`.agent/engines/claude-code/`, `.agent/outbox.md`)
- And not by *internal phase name* ("Stage 6 promotion" → "canonical promotion")

**In all guides regardless of `bcsc_class`:**
- Internal workflow phase numbers ("Stage 6", "Phase N") must not appear as section
  headers or step labels. Describe the operation: "Canonical promotion" not "Stage 6".
- Internal role tiers ("P1 administrator") must be replaced with functional descriptions
  ("workspace administrator").
- `.agent/` system paths must not appear. Use functional descriptions.

---

## §5 — JOURNAL-specific rules (extends journal-artifact-discipline.md)

`journal-artifact-discipline.md` is the canonical source for the JOURNAL forbidden
vocabulary list. Read it first. This section adds rules discovered in the 2026-06-11 audit
that are not yet in that file.

**Additions to the forbidden vocabulary list:**

*Internal project archive names:*
- `project-editorial`, `project-gis`, `project-intelligence`, `project-data`,
  `project-system`, `project-software`, `project-bim`, `project-design`,
  `project-private-network`, `project-orgcharts`, `project-documents`, `project-proforma`,
  `project-console`, `project-knowledge`, `project-proofreader`, `project-language`,
  `project-marketing`, `project-infrastructure` — and any `project-*` pattern

*Internal phase/build identifiers:*
- `Phase N` (any numbered phase), `Phase N+Change B`, `Change B`, `Sprint N`
- These must not appear in body text, table headers, appendix titles, figure captions,
  or any frontmatter field

*Internal coordination vocabulary:*
- `.agent/outbox`, `.agent/inbox`, `.agent/drafts-outbound`, NOTAM, apprenticeship corpus

*Internal role tiers:*
- `P1 administrator`, `P2 contributor`, `Tier A`, `Tier B`, `Tier C` (Doorman circuit tiers)

**`revision_history:` discipline:**
The `revision_history:` frontmatter field is part of the published manuscript (it appears
in the preprint block). It must be clean of all internal vocabulary.

- Acceptable: `"Language pass: §4–§5 forbidden-vocabulary scan complete."`
- Forbidden: `"Language pass §4–§5 (project-editorial): forbidden-vocabulary scan complete."`
- Acceptable: `"§6 restructured as 'Preliminary Coverage Assessment'; §7.5 added."`
- Forbidden: `"Phase 23+Change B actual counts incorporated into §6 table."`

**`notes_for_editor:` discipline:**
See `journal-artifact-discipline.md` §`notes_for_editor discipline` for the full rule.
Summary: anonymisation-safe language only. No phase numbers, archive names, sprint labels,
BRIEF references, or internal process terms.

---

## §6 — The voice test

Before committing any artifact, ask these three questions of every paragraph:

**1. The stranger test:** Could a reader who has never seen a BRIEF, never heard of
Foundry, and has no context from this session understand every sentence? If a sentence
requires prior context from a BRIEF or conversation, it is internal dialogue wearing
artifact clothing.

**2. The product test:** Would this sentence appear in a published academic paper, a
commercial software manual, or a professional wiki without seeming out of place? If
it would read as odd or internal in that context, revise.

**3. The process/product test:** Does this sentence describe *what was decided* (product)
or *how we came to decide it* (process)? Process sentences belong in the BRIEF.
Product sentences belong in the artifact.

---

## §7 — Known violations (as of 2026-06-11)

Living table. Update when a violation is fixed (Status: FIXED + commit SHA).
Additions: append new rows when found during editorial passes.

| Artifact | File | Location | Violation | Priority | Status |
|---|---|---|---|---|---|
| JOURNAL J5 | `JOURNAL-retail-colocation-v0.5.draft.md` | Lines 78, 347, 349, 362, 390, 753, 757, 785 | `Phase 23+Change B` in body text, table headers, and Appendix B title (7 instances) | EGREGIOUS | OPEN |
| JOURNAL J5 | `JOURNAL-retail-colocation-v0.5.draft.md` | Line 362 | "the research brief specified parameter calibration" | EGREGIOUS | OPEN |
| JOURNAL J5 | `JOURNAL-retail-colocation-v0.5.draft.md` | Lines 84, 87 | `project-editorial` in `revision_history: changes:` | MEDIUM | OPEN |
| JOURNAL J4 | `JOURNAL-private-network-v0.4.draft.md` | Line 81 | `project-editorial` in `revision_history: changes:` | MEDIUM | OPEN |
| TOPIC | `research/geometric-site-selection-national-tenancy.md` | Frontmatter | `editor: project-editorial` | MEDIUM | OPEN |
| TOPIC | `research/geometric-site-selection-national-tenancy.md` | Body (~6 instances) | `Phase 21`, `Phase 22`, `Phase 22 rebuild` as internal build identifiers | HIGH | OPEN |
| GUIDE | `media-knowledge-documentation/guide-editorial-content-sweep.md` | Lines 15–17, 101–110 | `project-editorial`, `project-intelligence`, `project-data`, `.agent/artifacts/` path, "DataGraph" internal tool | EGREGIOUS | OPEN |
| GUIDE | `foundry-workspace/guide-onboarding-new-archive.md` | Multiple | `.agent/` directory structure, `cleanup-log`, `handoffs-outbound`, "tetrad leg-pending" | EGREGIOUS | OPEN |
| GUIDE | `vault-privategit-source/guide-command-session.md` | Lines 62, 69 | "Stage 6 promotion" in operation table; "outbox", "identity store" | EGREGIOUS | OPEN |
| GUIDE | `gateway-orchestration-gis/guide-gis-pipeline-rebuild.md` | Line 147 | `## Stage 6 — O-D Study and Catchment Layers` as section header | MODERATE | OPEN |
| GUIDE | `foundry-workspace/guide-claude-code-hooks-installed.md` | Line 49–50 | "Stage 6 pending flag", "apprenticeship corpus" | HIGH | OPEN |
| GUIDE | `foundry-workspace/guide-foundry-vm-resource-recovery.md` | Line 101 | "workspace outbox", "P1 administrator" | HIGH | OPEN |
| GUIDE | `foundry-workspace/guide-pre-commit-gate-operator-flow.md` | Lines 34, 58, 92 | `.agent/inbox.md`, "NOTAM", "P1 administrator" | HIGH | OPEN |
| GUIDE | `vault-privategit-source/guide-workbench-setup.md` | Line 29 | "Stage 6 and binary-ledger process" | HIGH | OPEN |
| GUIDE | `node-console-operator/guide-os-console-operator.md` | Line 132 | `.agent/drafts-outbound/` | HIGH | OPEN |
| GUIDE | `cluster-intelligence/guide-tier-b-batch-gcp-deploy.md` | Multiple | "Tier A", "Tier B" Doorman tier naming | MODERATE | OPEN |

### Remediation protocol

Violations are fixed when the relevant artifact next passes through project-editorial for
any editorial reason. They are not fixed in isolation — a patch commit for a vocabulary
fix is a low-value commit; the fix travels with the next substantive edit.

**Exception:** EGREGIOUS violations in JOURNAL manuscripts (J5 Phase 23+Change B) should
be fixed in a dedicated language-fix pass before any public posting of that paper. These
7 body-text instances compromise double-blind anonymity and factual clarity.

---

## §8 — Relationship to journal-artifact-discipline.md

`journal-artifact-discipline.md` at `.agent/rules/journal-artifact-discipline.md` is the
authoritative rule file for JOURNALs — it governs mandatory 22-section structure, author
rules, submission workflow, and the JOURNAL-specific forbidden vocabulary list.

This BRIEF extends that list (§5 above) with patterns found in the 2026-06-11 audit that
are not yet in that file. When `journal-artifact-discipline.md` is next revised, the
additions in §5 of this BRIEF should be folded in.

This BRIEF also governs TOPICs and GUIDEs, which `journal-artifact-discipline.md` does not
address. The two documents are complementary: read both when processing a JOURNAL; read
this BRIEF alone when processing a TOPIC or GUIDE.

---

## §9 — Layout conventions per artifact type

Based on three-agent survey of 25+ current artifacts + professional standards research
(Google SRE runbooks, ITIL, Wikipedia featured articles, ACM/Elsevier/Wiley submission
guidelines, Material Design, Apple HIG, buildingSMART/ISO 19650 documentation).

### 9.1 GUIDE — required skeleton

```
# Guide — <Title>                    ← always present; "Guide — " prefix mandatory
<scope paragraph: what, where commands run, expected end state. ~3 sentences.>

## Prerequisites    ← three sub-groups: access / tools+versions / system state
## Purpose          ← optional if scope paragraph is sufficient
## Procedure        ← task-subsections (### Task) or Step N — form;
                      one command block per action;
                      expected output as inline comment;
                      failure branches inline at point of failure
## Expected Outcome ← bullet list of observable end-state facts
## Verification     ← copy-paste checks with expected output
## Rollback         ← exact undo sequence, or "no rollback; see X"
## Appendix         ← migrations, edge cases, decision tables (optional)
```

**New mandatory frontmatter field:** `last_verified:` — records when commands were last
tested against the live system (distinct from `last_edited:` which records prose edits).
GUIDEs that describe fast-moving scripts or deployments silently rot when only `last_edited`
is tracked.

**Blast-radius statement:** one sentence at the top of the Procedure section stating what
this procedure can break if it goes wrong.

**Reader:** a competent operator who has never run this procedure. Second-person imperative
("Run…", "Confirm…"), present tense, zero marketing vocabulary.

### 9.2 TOPIC — required skeleton

No body H1 (renderer supplies from `title:`).

```
<Lede block, 3 paragraphs, no heading:
 1. Consequence-first hook (1–2 sentences)
 2. Definition + claim-wrapped core assertion
 3. "For <primary audience>... This article covers A, B, C.">

## <Why / The problem>
## <Body sections — 2–6 H2s>     ← one concept each; diagram or table for any
                                    3+-way comparison; assertion headings, not labels
## See also                       ← 3–5 wikilinks, each with one-line rationale
```

**Figures rule:** every architecture TOPIC requires ≥1 diagram (Mermaid or ASCII
acceptable; no excuses for prose-only topology descriptions).

**Claim discipline:** `cites=[]` on a load-bearing claim wrapper is a defect — either
cite or remove the wrapper. An empty wrapper is indistinguishable from a verified one.

**`type:` field:** must come from the closed enum in `naming-convention.md §6`. Drift
from the enum (e.g., `concept`, `research`, `reference`) breaks planned JSON-LD/search
facets. Enforce on every article touched.

**Reader:** dual — engineer and institutional/financial reader; secondary audience must
survive the first paragraph of every section.

### 9.3 JOURNAL — additions to existing 22-section skeleton

Keep the full 22-section mandatory structure from `journal-artifact-discipline.md`. Add:

**≥3 figures required before `under-review` promotion:**
1. Study-area / cluster map (spatial papers)
2. Method / pipeline diagram
3. Headline-result chart or visualization

Format: `**Figure N.** Caption.` on the line immediately following the figure block.
A spatial clustering paper submitted to Economic Geography without a single map, or a
coverage-assessment paper submitted to Automation in Construction without a pipeline
diagram, will be desk-rejected by the editor.

**DOI on every reference:** required by Elsevier, Wiley, IEEE, and ACM formatting guides.
Append ` doi:10.xxxx/...` at the end of each reference entry that has one.

**Anonymized build:** A separate build exists with authors, affiliation, cite_as, funding
acknowledgements, and internal filenames stripped or replaced with "replication archive
(DOI)". Must be created before any journal submission. The current double-blind venues
(Economic Geography, ASPLOS, IEEE TIFS, Automation in Construction) require it.

**Run-before-submit rule:** every test marked "executable" in the Falsification Programme
section must be run and the actual results must appear in the Results section before
promotion to `state: under-review`. Promised tests that are not run are a reviewer red flag.

**Abstract density:** must state the headline finding AND its quantified magnitude in the
abstract (not just the methodology). The existing rule-of-thumb check: a reader who only
reads the abstract must know what the paper found, not merely how it studied something.

### 9.4 TEXT — required skeleton

```
# <Product> — <Month YYYY> <Release|Coverage|Maintenance> Update
<Lede: 1–2 sentences — what changed, where it is live, action required?>

## <Theme 1..N>       ← per theme: change → user meaning → numbers → mechanism
                        Tagged: **Added** / **Changed** / **Fixed** / **Deprecated**
## Key figures         ← bullet list, every figure with as-of date (REQUIRED)
## What is next        ← hedged ("planned", "anticipated… subject to")
<Italic methodology footnote: data sources, as-of date, definitions>
<Cross-references: full canonical URLs — never relative .md paths>
```

**New required frontmatter fields (add to schema):**
```yaml
as_of: YYYY-MM-DD       # figures accurate as of this date
action_required: yes|no # first thing the reader wants to know
```

**PROSE-TEXT sub-protocol** (additional schema fields):
```yaml
text_kind: release-note | ui-string | data-label | corpus
route: editorial | intelligence
pii_cleared: true | false   # required when route: intelligence
```
Corpus gate: `pii_cleared: true` AND no internal vocabulary (Do-Not-Use list + §2 patterns)
before any text enters `data/training-corpus/`.

**Length:** 50–150 words for standalone notes; labels ≤4 words; descriptions ≤140 chars.

### 9.5 DESIGN copy — additions to existing shape

**DESIGN-COMPONENT** — insert before existing Recipe sections:
```
## When to use / When not to use   ← 2 short bullet lists
## Anatomy                          ← labeled parts list (diagram strongly preferred)
## States and variants              ← table: state × treatment × token
## Content guidance                 ← label casing, length budget, voice
```

**DESIGN-RESEARCH** — canonical sequence (standardize across all existing drafts):
Problem → Decision → Evidence (empirical, measured from a named commit or tool) →
Reusable Pattern (3–5 generalized rules) → Research Trail → Implementation Reference
(file + line-number table).

**Research files land in `dtcg-vault/research/`** — not `docs/`. They are the AI
consumption surface for codegen agents; they have a different reader than component guides.

### 9.6 ASSET copy — per-asset metadata record required

Every committed asset must have a metadata record — either a sidecar `.meta.yaml` or an
entry in `assets-manifest.yaml` in the same repo. Minimum fields:

```yaml
file: ASSET-<name>.<ext>
title: "<canonical title>"
alt: "<canonical alt text — ≤125 chars; no 'image of' / 'screenshot of' prefix>"
description: "<1–2 sentences: what it depicts, when to use>"
dimensions:              # or viewBox for SVG
format: svg | png | jpg | webp
tokens: [<token-id>]     # design token correspondence (for update-coupling traceability)
license_attribution: >   # mandatory for ODbL/OSM-derived screenshots
  OpenStreetMap contributors (ODbL 1.0); Overture Maps (CDLA Permissive 2.0)
source: "<capture brief ref or design file>"
superseded_by: ~
```

Capture briefs: add `caption:` and `alt:` field per required capture — the editorial
half of the asset is missing if only capture steps are specified.

No asset may be committed without its metadata record. No embedded asset may be published
in a TOPIC or TEXT without an `alt:` value sourced from the record.

### 9.7 BIM copy — additions

**Worked example required** in every conceptual BIM TOPIC (the single most effective
explanatory device in technical AEC documentation): one element, end to end — its IFC
snippet + its sidecar YAML + its BCF topic (3 annotated code blocks, ~30 lines total).

**Property values must carry units** at every occurrence. Example:
`ThermalTransmittance: 0.300 W/(m²·K)` not `ThermalTransmittance: 0.300`.

**GUID notation:** state the 22-char base64-encoded IFC GloballyUniqueId form at first
use in any TOPIC that uses GUIDs — operators copy the wrong form without this.

**Standards-version registry:** create `reference/bim-standards-register.md` — a single
table of all pinned standard versions (IFC4X3, BCF 3.0, IDS 1.0, ifcopenshell ≥0.8.5,
web-ifc 0.77) so when a standard version changes, there is one canonical place to update.

---

## §10 — Language Protocol standards

### Canonical `language_protocol:` enum

From `artifact-classification.yaml`. All other values are normalization targets —
normalize on the next substantive edit to that file.

```
PROSE-TOPIC        PROSE-GUIDE        PROSE-README       PROSE-RESEARCH
PROSE-MEMO         PROSE-ARCHITECTURE PROSE-INVENTORY    PROSE-DIRECTIVE
PROSE-TEXT
COMMS-ANNOUNCEMENT COMMS-PRESS        COMMS-CORPORATE    COMMS-EMAIL     COMMS-NOTES
LEGAL-MANIFEST     LEGAL-DISCLAIMER   LEGAL-CORRECTIONS
TRANSLATE-ES
DESIGN-COMPONENT   DESIGN-TOKEN-CHANGE DESIGN-RESEARCH   DESIGN-WIREFRAME
ASSET
JOURNAL
```

**Known drift:** 44+ drafts use bare `TOPIC` / `GUIDE` / `TOPIC-*` as the protocol value.
One file has the invalid value `language_protocol: project-editorial`. Normalize on first
substantive edit — no dedicated cleanup commit.

### TRANSLATE-ES standard

**Register:** neutral Latin American Spanish (español neutro). `ustedes` not `vosotros`.
No regional idiom. `computadora/equipo/sistema/nodo` over `ordenador`. Formal register;
impersonal *se* constructions for procedures.

**Two named modes** (operator ratification pending — use `parity` as default until ratified):

| Mode | When | Body structure |
|---|---|---|
| `parity` | Wiki articles (default) | Same H2 skeleton as EN pair; sections may compress but not be dropped; ~30% length expansion is normal |
| `overview` | Long governance documents (DOCTRINE.es.md pattern) | 1–2 page strategic summary; does not mirror EN section structure |

**Never translate:** product names, CLI commands, F-keys, file paths, `service-*` names,
YAML/JSON keys, code identifiers. Keep verbatim.

**Industry borrowings:** italicized with article on first use: *el kernel*, *el commit*,
*el firmware*. Once introduced, use consistently.

**BCSC hedge vocabulary (controlled list):** *previsto/a*, *se prevé que*, *tiene como
objetivo*, *podría*, *está planificado/a*, *se anticipa*. Never bare future tense
(*será*, *estará*) for forward-looking claims — it removes the hedge.

**Staleness check:** `paired_with:` field + matching `last_edited` date. When the EN
article updates, the ES pair must update in the same session.

**Tooling gap — `GLOSSARY.es.md` does not exist.** This is the enforcement mechanism for
terminology consistency across ES articles. Per DOCTRINE §XII roadmap item. Create the stub;
populate as terminology is settled. Without it, term drift (three different renderings of
"ledger") is guaranteed.

### COMMS standard

Bare `COMMS` as a `language_protocol:` value is **deprecated**. Use the closed sub-type.

| Sub-type | Structure | When |
|---|---|---|
| `COMMS-ANNOUNCEMENT` | Inverted pyramid + FLS paragraph | Coverage releases, product updates |
| `COMMS-PRESS` | Inverted pyramid + dateline + boilerplate + media contact | Press distribution |
| `COMMS-CORPORATE` | Document-header block + numbered statutory sections | Disclosure memos, regulatory filings |
| `COMMS-EMAIL` | BLUF: ask/decision line 1; context after; explicit action items | Correspondence |
| `COMMS-NOTES` | Decisions-made + action items + attendees | Meeting records |

**Inverted pyramid** applies to ANNOUNCEMENT and PRESS: most newsworthy fact first
(who/what/when/where/why), supporting detail second, background last. The piece survives
truncation at any paragraph.

**Forward-looking-statements paragraph:** mandatory in every COMMS-ANNOUNCEMENT and
COMMS-PRESS when `bcsc_class: forward-looking`. The FLS paragraph text must contain none
of the §2 forbidden vocabulary.

**Bloomberg standard applies with no exceptions** to COMMS artifacts. Zero superlatives;
every quantified claim carries the number; attribution for every claim not the issuer's
own act.

### PROSE-RESEARCH standard

Declare `research_type:` in frontmatter before drafting:

- `position` — advocacy / white-paper structure: answer first, assertion headings,
  Pyramid Principle grouping (MECE sets of ~3), evidence in exhibits referenced from body
- `findings` — study structure: question → method → results → limitations; neutral on the
  thesis until the data speak

Mixing the two in one artifact is the primary PROSE-RESEARCH failure mode. A white paper
that buries its position inside a findings structure persuades no one. A findings paper
that leads with the conclusion is an advocacy document masquerading as research.

**Falsifiable-claim-first abstract** is required when JOURNAL promotion is in view (the
PROSE-RESEARCH becomes a JOURNAL stub). This is the transition point: write the abstract
in the JOURNAL format (falsifiable claim sentence 1; method sentences 2–3; quantified
result) before the paper is fully drafted.

### PROSE-TEXT / TEXT standard

Zero TEXT files on disk today. Registry rows B5/B11/B12 in `artifact-registry.md` are
stale — the files they claim are staged do not exist. This protocol is unrealized.

First real TEXT artifact should be produced against the skeleton in §9.4 to validate the
standard before it is applied in bulk. Required frontmatter additions (§9.4 above) must be
added to the schema.

---

## §11 — DataGraph ↔ Artifact feedback loop

### The virtuous cycle

```
DataGraph ─── pre-write query ──►  project-editorial
     ▲                                     │
     │                             artifact committed
     │                                     │
     └──── submit_extraction ◄─────────────┘

Each session: entity graph enriches → richer pre-write context next session.
```

### Current state (as of 2026-06-11)

The `query_datagraph()` MCP tool returns `[]` for all queries because it forwards to
Doorman with `X-Foundry-Module-ID: mcp-foundry` — an empty module namespace. The live
graph at `127.0.0.1:9081` holds **10,125 entities** across three useful namespaces:

| Namespace | Content | Editorial value |
|---|---|---|
| `__taxonomy__` | Bilingual glossary (EN term + ES pair + definition), themes, topic/guide entities with `wiki_path` | **Primary pre-write source** — canonical term lookup + existing-coverage check |
| `woodfine` | Person/Company/Project entities (dedup issues: fabricated "PointSav Digital Systems AG" exists alongside real entity) | Entity name canonicalization |
| `foundry-workspace` | Workspace project names | Project name check |

**P0 fix (config, not code rewrite — project-intelligence to action):**
Add optional `module_id` to `QueryDatagraphInput` in
`slm-mcp-server/src/main.rs:91-111`, defaulting to `__taxonomy__`.
Interim workaround: set `SLM_MODULE_ID=__taxonomy__` env var in the archive's
`.agent/engines/claude-code/settings.json`.

### Pre-write protocol (effective immediately with Doorman direct query as workaround)

Before drafting any TOPIC, GUIDE, or JOURNAL:

1. **Taxonomy query** — `query_datagraph(__taxonomy__, <title terms>)`:
   returns canonical term, ES pair, definition, existing `wiki_path` (coverage check)
2. **Entity query** — `query_datagraph(woodfine, <company/project/person names>)`:
   returns entity names; flag any variant that differs from the canonical form
3. **Nomenclature check** — unfound terms checked against:
   - `~/Foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md`
   - `POINTSAV-Project-Instructions.md §5` (Do-Not-Use list)
4. **Terminology map** — before writing, resolve: `{term → canonical_form, es_pair, definition}`

Result: every entity name in the draft uses the canonical form; no terminology drift;
no duplicate-coverage articles on topics already published.

### Post-commit protocol

After every committed TOPIC or GUIDE: call `submit_extraction(text, schema)` with the
article body text. This queues the article for entity extraction, which enriches:
- New entity mentions → added to entity graph
- Article lede definitions → improved entity descriptions
- The article's `wiki_path` → available to future pre-write coverage checks

The feedback loop compounds: each committed article makes the next article more accurate.

### P1 API proposal (project-data / project-intelligence to action)

New service-content endpoint: `GET /v1/editorial-context?topic=<slug>`

```json
{
  "canonical_entities": [{
    "term": "co-location",
    "canonical": "co-location",
    "es_pair": "co-ubicación",
    "definition": "...(≤200 chars)",
    "existing_wiki_path": "architecture/co-location-methodology"
  }],
  "existing_coverage": ["architecture/doorman-protocol", "substrate/compounding-substrate"],
  "themes": [{"theme": "...", "thesis": "...", "keywords": []}],
  "unresolved_terms": ["term-not-in-graph"]
}
```

Wire this at session startup step 3b, after exporting `FOUNDRY_CURRENT_TASK`.

### Entity enrichment needed (project-intelligence to action)

- **Dedup** `woodfine` namespace — remove fabricated entities; merge name variants
- **Populate `description`** on the ~25 most-cited entities (currently null)
- **Lift the 200-char truncation** in `taxonomy.rs:396` — definitions are cut off mid-sentence
- **Ingest project-registry state** — enables correct shipped-vs-planned BCSC hedging
  from graph data (what is `Active` vs `Scaffold-coded` vs `Reserved-folder`)
- **Load Do-Not-Use / rename tables** as a `__nomenclature__` class

### Known infrastructure faults (flag to project-intelligence)

- Tier A `ask_local` transport failure: `tier_a_tok_per_s: 0.0` — OLMo inference down
- Contradictory health states: `doorman_health()` and `get_doorman_status()` return
  inconsistent queue/readyz values

---

## §12 — Better writing framework

**Priority order for systemic improvement:**

1. **Figures.** Zero diagrams across TOPICs, JOURNALs, and DESIGN docs — the single
   largest systemic gap found in the 2026-06-11 survey. Rule: if you describe a topology,
   pipeline, or flow, there is a diagram. No architecture TOPIC ships without ≥1 diagram.
   No JOURNAL moves to `under-review` without ≥3 figures. No DESIGN-COMPONENT is committed
   without an anatomy diagram.

2. **DataGraph pre-write (§11).** The highest-leverage process change available. When every
   entity name is looked up before it appears in prose, terminology drift stops. The §11
   feedback loop makes each session materially better than the last.

3. **Claim citation discipline.** `cites=[]` on a structural claim wrapper is a defect.
   The claim wrapper apparatus is built and not being exercised — every wrapper with a
   load-bearing assertion must cite or be removed.

4. **Freshness semantics.** Three fields serve three distinct purposes:
   - `last_edited:` — prose was touched (current usage, continue)
   - `last_verified:` — commands in a GUIDE were tested against the live system (new)
   - `as_of:` — figures/statistics accurate as of this date (new, TEXT + TOPIC)
   Using `last_edited` for all three is why GUIDEs appear current when command output
   has silently changed.

5. **Language protocol normalization.** 44+ files with wrong `language_protocol:` values
   misdirect the editorial pipeline. Normalize on first substantive edit per file; no
   dedicated cleanup commit (same remediation protocol as §7 violations).

6. **COMMS sub-type enforcement.** Bare `COMMS` deprecated. Every existing COMMS file
   gets a sub-type on first edit.

7. **ES pair tooling.** Build `GLOSSARY.es.md` — the enforcement mechanism for terminology
   consistency across Spanish articles. Create the stub now; populate as terminology
   is settled. Without a termbase, three sessions will produce three renderings of the
   same term.

8. **JOURNAL anonymized builds.** Every active JOURNAL paper needs a corresponding
   anonymized variant before its target journal submission. This is a mechanical operation
   (strip/replace author block + funding + internal filenames) that has never been done.
   Add an "anonymized build created" checkbox to each paper's submission workflow.

---

## §13 — Pending work (absorbed from BRIEF-phase-fg-institutional-redesign.md)

### Phase F — DESIGN-wiki-institutional-redesign

**Artifact type:** DESIGN-TOKEN-CHANGE (and possibly DESIGN-COMPONENT) targeting
`woodfine-media-assets` and/or `pointsav-design-system`.

**Scope (interpretation pending operator confirmation):**
- CSS custom-property tokens: `--wf-claret`, `--wf-slate`, `--ds-*` variables used
  across the three wiki sites
- Typography scale: Oswald + Roboto Slab + Nunito Sans stack
- Header recipe (utility bar | brand bar | nav bar) and footer recipe

**Gate:** `master_cosign:` field in frontmatter is mandatory — not committed without it.
Phase F artifacts route to project-design, not committed directly by project-editorial.

### Phase G — 6 GUIDEs (2 per wiki site × 3 sites)

Each pair covers: deployment procedure, content operations, service management, binary
rebuild, nginx + certbot renewal, logging, rollback. Six total GUIDE artifacts, one pair
per site.

### Open questions — BLOCKED until operator responds

1. Is Phase E a gate for Phase F, or can F start independently of Phase E completion?
2. Confirm subdirectory names for the three deployment targets:
   `documentation.pointsav.com`, `projects.woodfinegroup.com`, `corporate.woodfinegroup.com`
3. Is the current design-system foundation (tokens + components) sufficient to author the
   Phase G GUIDEs, or must Phase F land first?
4. Who is the `master_cosign` for the DESIGN-TOKEN-CHANGE artifact?
5. Are the three woodfine-fleet-deployment staging mirrors current with Stage 6? Or is a
   pending push required before new GUIDEs can be added?
6. Which of the three wiki sites has a live deployment instance ready to receive new guides?

**Status: no content work begins until all 6 questions are answered.**

---

## §14 — BRIEF consolidation log

| Brief | Action | Date |
|---|---|---|
| `BRIEF-artifact-style-guide.md` | Master BRIEF — expanded with §9–§13 above | 2026-06-11 |
| `BRIEF-phase-fg-institutional-redesign.md` | Content absorbed into §13; `status: archived` | 2026-06-11 |
| `BRIEF-regional-markets-system.md` | Retained as domain reference artifact — not editorial workflow | 2026-06-11 |
| `audit-foundry-wide-2026-05-16.md` | Retained as audit log artifact — not a BRIEF | 2026-06-11 |
| `archive/BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` | `status: archived` — work complete per 2026-05-22 close-out | 2026-06-11 |

---

## Changelog

| Date | What |
|---|---|
| 2026-06-11 | §9–§14 added — layout conventions, language protocols, DataGraph feedback loop, better-writing framework, Phase F+G pending work, BRIEF consolidation (three Fable agents + consolidation of BRIEF-phase-fg-institutional-redesign.md) |
| 2026-06-11 | Created — populated from three-agent corpus audit across 717 articles + 8 JOURNALs |
