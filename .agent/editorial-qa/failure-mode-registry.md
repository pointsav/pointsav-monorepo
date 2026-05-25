---
artifact: editorial-qa-registry
title: Failure-mode registry
slug: failure-mode-registry
version: 1.0
status: active
created: 2026-05-21
owner: project-editorial
consumes: the Gate-0 editorial standard (editorial plan §2)
used_by:
  - Track A critic pass — editorial plan §4 step 4
  - editorial-lint.py advisory checks (Track D — D1)
---

# Failure-mode registry

A versioned house list of the recurring tells of weak AI-assisted prose. Each
entry names the failure, shows it, and shows the fix. The registry is the
checklist for the **critic pass** — editorial plan §4 step 4 — and the advisory
half of the editorial linter (D1).

It is an operational editorial tool, not public wiki content. English-only.
Revise it by bumping `version:` and dating the change; never delete an entry —
supersede it.

## How to run the critic pass

Read the draft once as a copy editor, in a role separate from the one that wrote
it. For every entry below, scan the draft for the tell. Emit a structured defect
list — entry id, location, the offending text — then revise against it. Repeat
at most twice. A draft that survives two clean critic passes is ready for the
deterministic lint and staging.

The `Lintable` field marks how much of each failure a machine can catch:
`deterministic` — the linter flags it outright; `heuristic` — the linter raises
an advisory the editor confirms; `critic-only` — judgement, no machine check.

## Failure modes

### FM-01 — Sameness

**Tell.** Every sentence runs to the same length and the same shape — clause,
comma, clause. No short sentence breaks the rhythm. The paragraph reads as a
monotone and the reader's eye slides off it.

**Before.** *The system records each transaction in an append-only log, and the
log is replicated across three nodes, and each node verifies the hash chain
before accepting a new entry, and the result is a record that cannot be altered
after the fact.*

**After.** *The system records each transaction in an append-only log,
replicated across three nodes. Each node verifies the hash chain before it
accepts a new entry. The record cannot be altered afterward.*

**Fix.** Vary sentence length deliberately. Put at least one short declarative
sentence in every paragraph (Gate-0 rule 1). Read the paragraph aloud — if it
has no rhythm, it fails.

**Lintable:** heuristic (sentence-length variance metric — advisory only).

### FM-02 — Hedging and false symmetry

**Tell.** Reflexive qualifiers that protect the writer rather than inform the
reader: *it is important to note*, *it is worth considering*, *arguably*,
*in many ways*. False symmetry presents two sides as balanced when the content
does not support the balance.

**Before.** *It is worth noting that, while the local tier is generally faster,
the cloud tier may in some cases also be appropriate.*

**After.** *The local tier resolves the request in about five seconds. The cloud
tier is the fallback when the local tier misses its deadline.*

**Fix.** Assert the fact. If a claim is genuinely uncertain, say so precisely and
once — not with a reflexive hedge. Forward-looking claims keep `planned` /
`intended` / `may` / `target`; that is disclosure discipline, not hedging.

**Lintable:** heuristic (hedge-phrase wordlist — advisory).

### FM-03 — Negative parallelism

**Tell.** The *not just X, but Y* / *it isn't about X — it's about Y* /
*X is not Y; it is Z* construction, used for rhythm rather than meaning. One
instance is rhetoric; three in a page is a tell.

**Before.** *This is not merely a storage format — it is a compliance
guarantee. The ledger is not just durable; it is structurally immutable.*

**After.** *The storage format is a compliance guarantee: a record cannot be
altered after it is written.*

**Fix.** State the positive claim directly. Delete the negated half unless the
contrast carries real information the reader needs.

**Lintable:** heuristic (construction pattern — advisory).

### FM-04 — Elevated filler

**Tell.** Inflated diction that sounds substantial and carries nothing:
`leverage`, `utilize`, `facilitate`, `robust`, `seamless`, `empower`,
`cutting-edge`, `next-generation`, `world-class`, `industry-leading`. Abstract
nouns standing where a concrete verb belongs.

**Before.** *The platform leverages a robust architecture to empower seamless,
next-generation compliance workflows.*

**After.** *The platform routes every compliance check through one audited
gateway, so each check is logged the same way.*

**Fix.** Replace the inflated word with the plain one — `use`, not `leverage`.
Replace an abstract noun with the verb it hides. If removing the word loses no
meaning, the word was filler.

**Lintable:** deterministic (banned-vocabulary list — the linter flags it).

### FM-05 — Padding

**Tell.** Sentences that restate the previous sentence; throat-clearing openers
(*In today's fast-moving landscape*, *It is well known that*); closing sentences
that summarize a paragraph short enough to need no summary.

**Before.** *In today's complex regulatory environment, compliance matters more
than ever. Organizations must take compliance seriously. This is why compliance
is important.*

**After.** *SEC Rule 17a-4(f) requires that broker-dealer records be stored so
they cannot be altered or deleted.*

**Fix.** Cut the sentence that adds no fact. Open on the first real claim. Trust
the reader to hold a four-sentence paragraph without a recap.

**Lintable:** critic-only.

### FM-06 — Rule-of-three reflex

**Tell.** Every list has exactly three items; every noun takes three adjectives;
every argument has three parts — regardless of what the content needs.

**Before.** *The system is fast, secure, and reliable, delivering speed,
safety, and trust to every user.*

**After.** *A request that resolves on the local tier never leaves the
customer's infrastructure.*

**Fix.** Use the number of items the content actually has. Two is fine. One,
stated well, is often better than three padded.

**Lintable:** critic-only.

### FM-07 — Vague attribution

**Tell.** *Studies show*, *experts agree*, *it is widely understood*,
*research suggests* — authority claimed without a source the reader can follow.

**Before.** *Studies show that immutable ledgers reduce audit cost.*

**After.** *Per [citation-id], audit preparation on an append-only ledger took
roughly half the analyst hours of a mutable store.* — or, if no source exists,
delete the claim.

**Fix.** Cite the specific source (style-guide-topic: cite every non-obvious
claim) or remove the claim. A non-obvious claim with no citation does not ship.

**Lintable:** heuristic (attribution-phrase wordlist — advisory).

### FM-08 — Mechanism without consequence

**Tell.** A paragraph describes how something works and stops, never reaching
*so what for a regulated buyer or a risk-aware engineer*. The CFO sentence test
(style-guide-topic) fails.

**Before.** *The storage engine appends each record to an immutable tile
format.*

**After.** *A record cannot be deleted or modified after it is written — the
storage engine makes modification structurally impossible rather than
policy-prohibited, which is what an auditor needs to see.*

**Fix.** Add the consequence for compliance, custody, cost, or risk — or fold
the mechanism into the paragraph that already states one.

**Lintable:** critic-only.

## Change log

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-05-21 | Initial registry — 8 failure modes (FM-01…FM-08). Track D / D2. |
