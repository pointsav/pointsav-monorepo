---
schema: foundry-draft-v1
artifact: editorial-ratification
title: "Vocabulary ratification — E2 delivery for project-intelligence"
slug: vocabulary-ratification-e2
version: 1.0
status: approved
state: approved
created: 2026-05-24
owner: project-editorial
target_cluster: project-intelligence
target_endpoints:
  - "POST /v1/editorial/grammar"
  - "POST /v1/editorial/seed"
  - "RelatedTo edges (editorial vocabulary graph)"
canonical_source: clones/project-editorial/.agent/editorial-qa/editorial-standard.md
banned_vocabulary_source: clones/project-editorial/.agent/editorial-qa/banned-vocabulary.txt
bcsc_class: public-disclosure-safe
originating_cluster: project-editorial
research_trail:
  - editorial-standard.md (ratified 2026-05-21, Track D / D4)
  - banned-vocabulary.txt (canonical machine-readable list)
  - failure-mode-registry.md (FM-01..FM-08)
---

# Vocabulary ratification — E2

This document formally ratifies the editorial vocabulary standard for
consumption by `project-intelligence`'s three editorial API endpoints:
`POST /v1/editorial/grammar`, `POST /v1/editorial/seed`, and the
`RelatedTo` editorial vocabulary graph edges.

The canonical editorial ruleset lives in full at:
`clones/project-editorial/.agent/editorial-qa/`

This ratification document packages the machine-consumable subset. On
any disagreement between this document and the canonical files, the
canonical files govern.

---

## 1. Banned vocabulary — Do Not Use

The following terms are banned from all public-facing content. The list
is the authoritative subset for `POST /v1/editorial/grammar` vocabulary
checking. Matching is case-insensitive at word boundaries; inflected
forms (e.g., "leveraging", "leverages") are also banned.

| Term | Class |
|---|---|
| `leverage` | marketing-register |
| `empower` | marketing-register |
| `next-generation` | marketing-register |
| `industry-leading` | marketing-register |
| `seamless` | marketing-register |
| `robust` | marketing-register |
| `cutting-edge` | marketing-register |
| `world-class` | marketing-register |
| `utilize` | elevated-filler |
| `facilitate` | elevated-filler |

**Additional Do-Not-Use terms** (not in `banned-vocabulary.txt` but
enforced by project-editorial per `POINTSAV-Project-Instructions` §5):

| Term | Reason |
|---|---|
| Yo-Yo | Internal product codename — never in public content |
| Liquid Glass | Internal design-system internal codename — never in public content |
| Doctrine | Workspace-internal governance vocabulary |
| Convention (as governance vocabulary) | Workspace-internal governance vocabulary |

The Do-Not-Use set expands `banned-vocabulary.txt` for governance-
vocabulary leaks; the banned-vocabulary file covers the marketing-
register and elevated-filler cases. Both sets apply to all public
content in `media-knowledge-documentation`, `media-knowledge-projects`,
and `media-knowledge-corporate`.

---

## 2. Gate-0 rules — machine-consumable summary

The five ratified rules from `editorial-standard.md §1` (ratified
2026-05-21). Each rule is stated in the form used by the editorial
endpoints.

### Rule 1 — Sentence-length budget (by role)

| Sentence role | Limit |
|---|---|
| Expansion sentence (body mechanism/argument) | ≤ 45 words |
| Disclosure sentence (lede, compliance claim, regulatory statement) | ≤ 25 words |

Every paragraph must contain at least one short declarative sentence
(accordion rhythm). No absolute minimum enforced by linter; advisory
only.

### Rule 2 — Active-voice / forward-looking discipline

Active voice describes present-fact mechanism. Forward-looking claims
(capability, timeline, or outcome not yet true) must carry one of:
`planned`, `intended`, `may`, `target`.

Prohibited: personification ("the system believes"), absolute bans on
`is`/`are`/`was` (only prefer active; not a hard ban).

For `POST /v1/editorial/grammar`: flag passive constructions that assert
forward-looking facts as accomplished. Flag personification. Do not flag
all uses of `is`/`are`.

### Rule 3 — Analogy ceiling

Maximum one analogy per 300 words. Analogies are optional. The endpoint
should flag content that exceeds this density.

### Rule 4 — Lede structure (Bloomberg nut-graf)

The Bloomberg four-paragraph lede carries the news in the first ~10% of
the article. Body sections follow the Franklin arc: Crisis → Quest →
Breakthrough.

For `POST /v1/editorial/seed`: seed content must open with a nut-graf
lede, not a definitional sentence. The nut-graf names the consequence
or decision before the mechanism.

### Rule 5 — Register rejection (SaaS-marketing)

The SaaS-marketing register is rejected for all public content. Markers:
- Software-product landing-page voice ("Now you can…", "Unlock…",
  "Say goodbye to…")
- Internal codenames in public text
- Aspirational claims stated as fact ("X delivers Y" when Y is not yet
  delivered)

---

## 3. Language registers — endpoint routing

Three registers are in use. Endpoints should use these to calibrate
feedback:

| Register | Wikis | Voice target | Sentence mean |
|---|---|---|---|
| 1 — Bloomberg/FT/Economist | `media-knowledge-corporate`, `media-knowledge-projects` | Institutional decision-maker; consequence-first | 14–18 words; 25 hard max |
| 2 — Stripe/Cloudflare + accessibility layer | `media-knowledge-documentation` | Engineer primary, institutional secondary; Concept → Why → How → Code | Per Gate-0 rule 1 |
| 3 — RIBA/IFC specification | Specialist sites only | Prescriptive specification | Not used in the three main wikis |

For `POST /v1/editorial/grammar` and `POST /v1/editorial/seed`:
determine which wiki the content targets (from the `category` field in
frontmatter or the request context) and apply the appropriate register.

---

## 4. BCSC posture — mandatory flag

The BCSC continuous-disclosure posture applies to all three wikis.
This is a hard rule, not advisory.

**Sovereign Data Foundation** must be referred to in planned/intended
terms only. Never as a current equity holder or active governance body.

Forward-looking claims about PointSav products, timelines, or revenue
must carry one of: `planned`, `intended`, `may`, `target`.

For `POST /v1/editorial/grammar`: flag any sentence that presents a
future state as accomplished fact where the subject is a PointSav
product, feature, or market position.

---

## 5. Approved vocabulary — RelatedTo edge types

For the `RelatedTo` editorial vocabulary graph, the following
relationship types are ratified:

| Edge type | Meaning |
|---|---|
| `implements` | Article A implements a rule or standard stated in article B |
| `supersedes` | Article A replaces article B (e.g., updated ADR) |
| `extends` | Article A adds scope to article B without replacing it |
| `contrasts_with` | Article A describes an approach that differs from article B |
| `see_also` | General cross-reference without a semantic relationship |
| `pairs_with` | Article A is the bilingual partner of article B (EN↔ES) |

These match the `relates_to` frontmatter vocabulary proposed in
`naming-convention.md §6`. Do not invent additional edge types without
routing the proposal back to project-editorial.

---

## 6. Single-source rule

The canonical editorial ruleset is:

```
clones/project-editorial/.agent/editorial-qa/
  editorial-standard.md     — Gate-0 five rules + register summary
  banned-vocabulary.txt     — machine-readable banned-term list
  failure-mode-registry.md  — FM-01..FM-08 failure modes
  CORPUS-SCHEMA.md          — frontmatter + corpus-structure schema
```

This ratification document is derived from those files. Any update to
the canonical files supersedes this document; project-intelligence
should re-request a ratification update when the canonical files change.

---

*Issued by project-editorial, 2026-05-24. Routed to project-intelligence
outbox per editorial plan §6 E2.*
