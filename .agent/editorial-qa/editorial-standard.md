---
artifact: editorial-qa-standard
title: The editorial standard
slug: editorial-standard
version: 1.0
status: active
ratified: 2026-05-21
created: 2026-05-21
owner: project-editorial
consumers:
  - editorial-lint.py (Track D / D1 — project-editorial)
  - validate_editorial_standards (app-mediakit-knowledge engine — project-knowledge)
---

# The editorial standard

This file is the **single canonical editorial standard** for the Foundry
content corpus. One ruleset, two consumers (KNOWLEDGE-PLATFORM-VISION §14):
the editorial linter `editorial-lint.py` and the wiki engine's
`validate_editorial_standards` both read from the files named here. There is no
second ruleset.

The standard has four parts, each with one canonical home:

| Part | Canonical file | Form |
|---|---|---|
| Gate-0 rules | this file, §1 | prose |
| Banned vocabulary | `banned-vocabulary.txt` | machine-readable list |
| Language registers | §3 below + `reference/editorial-language-registers.md` | prose |
| Failure modes | `failure-mode-registry.md` | prose + examples |

Corpus structure and frontmatter schema is a separate document: `CORPUS-SCHEMA.md`.

## 1. Gate-0 rules

The five rules reconciled and operator-ratified on 2026-05-21 (editorial plan
§2). They govern every TOPIC, GUIDE, and draft in the corpus. Where any wiki
style article conflicts with a rule here, the rule here governs.

1. **Sentence length is budgeted by sentence role.** An expansion sentence —
   one developing a mechanism or argument inside a body section — runs to about
   45 words at most. A disclosure sentence — a lede, a compliance claim, a
   regulatory statement — runs to 25 words at most. Every paragraph carries at
   least one short declarative sentence (accordion rhythm).
2. **Active verbs describe present-fact mechanism.** The active voice describes
   how something works now. It is not used to assert a forward-looking claim as
   accomplished fact — capability, timeline, or outcome not yet true keeps
   `planned` / `intended` / `may` / `target`. No personification. No absolute
   ban on `is` / `are` / `was`.
3. **Analogy is a ceiling, not a quota.** Optional; at most one per 300 words.
4. **The lede is the nut graf; the Franklin arc orders the body.** The Bloomberg
   four-paragraph lede carries the news in roughly the first 10%. The Franklin
   arc (Crisis → Quest → Breakthrough) orders body sections only.
5. **The SaaS-marketing register is rejected.** No software-product landing-page
   voice in public content. Internal codenames stay internal.

## 2. Banned vocabulary

The canonical machine-readable list is `banned-vocabulary.txt` in this
directory — one term per line, consumed directly by `editorial-lint.py`. Edit
the list there; do not maintain a second copy.

The wiki articles `reference/style-guide-topic.md` ("Voice") and
`reference/editorial-language-registers.md` ("Vocabulary retirement") echo the
list for human readers. Those echoes are **derived, not authoritative** — if an
echo and `banned-vocabulary.txt` disagree, the `.txt` file wins and the echo is
corrected.

A banned term inside a code span (backticks) is a *mention*, not a *use* — the
linter exempts it, so a style article may quote the list freely.

## 3. Language registers

Three registers, one per audience (full prose:
`reference/editorial-language-registers.md`):

| Register | Wikis | Voice |
|---|---|---|
| 1 — Bloomberg / FT / Economist | `content-wiki-corporate`, `content-wiki-projects` | Institutional decision-maker; consequence-first; 14–18 words, 25 hard |
| 2 — Stripe / Cloudflare + accessibility layer | `content-wiki-documentation` | Engineer primary, institutional reader secondary; Concept → Why it matters → How → Code |
| 3 — RIBA / IFC specification | specialist sites only (`bim.`, `gis.`, `design.`) | Prescriptive specification; never in the three main wikis |

The per-register sentence rules specialize Gate-0 rule 1; they never relax it.

## 4. Failure modes

`failure-mode-registry.md` in this directory is the versioned house list of
AI-writing tells (FM-01…FM-08), each with an example and a fix. It is the
checklist for the Track A critic pass (editorial plan §4 step 4) and the
advisory half of `editorial-lint.py`.

## 5. The single-source rule

The four files in `.agent/editorial-qa/` —
`editorial-standard.md` (this file), `banned-vocabulary.txt`,
`failure-mode-registry.md`, `CORPUS-SCHEMA.md` — are the canonical editorial
ruleset. Any other statement of these rules anywhere in the corpus (a wiki
style article, an engine constant, a draft pipeline doc) is a derived echo.
On any disagreement, the canonical file governs and the echo is reconciled to
it. New rules are added here first, then propagated to the echoes.

## Change log

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-05-21 | Initial consolidation — Gate-0 rules, banned-vocabulary pointer, register summary, failure-mode pointer. Track D / D4. |
