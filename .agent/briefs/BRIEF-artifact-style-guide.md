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
| JOURNAL J5 | `JOURNAL-retail-colocation-v0.5.draft.md` | Lines 78, 347, 349, 362, 390, 753, 757, 785 | `Phase 23+Change B` in body text, table headers, and Appendix B title (7 instances) | HIGH | OPEN |
| JOURNAL J5 | `JOURNAL-retail-colocation-v0.5.draft.md` | Line 362 | "the research brief specified parameter calibration" | HIGH | OPEN |
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

## Changelog

| Date | What |
|---|---|
| 2026-06-11 | Created — populated from three-agent corpus audit across 717 articles + 8 JOURNALs |
