---
schema: foundry-convention-v1
convention_name: claim-authoring-convention
ratified: PENDING — Command Session
doctrine_version: 0.1.1
doctrine_claims: []          # Command to assign at ratification (claim-native data model)
canonical_language: en
cites:
  - citation-substrate
  - bcsc-disclosure-posture
  - draft-research-trail-discipline
---

# Convention — Claim-Authoring (PROPOSED)

> **PROPOSED DRAFT** — authored 2026-05-21 by `totebox@project-knowledge`
> (claude-code) as **Phase 2** of `KNOWLEDGE-PLATFORM-PLAN.md`. It is **not yet
> ratified.** Per `conventions/artifact-classification.yaml`, a `CONVENTION-`
> artifact ratifies through the **Command Session** and commits to
> `~/Foundry/conventions/` — suggested filename `claim-authoring-convention.md`
> (Command may rename). Routed in parallel to `project-editorial` (consumer —
> the Track-A2 TOPIC rewrites annotate against it) and `project-design`
> (informational — the citation/freshness components visualise these claims).
>
> **Upstream:** `KNOWLEDGE-PLATFORM-VISION.md` §2, §6, §9 (the claim-native data
> model). **Downstream implementation:** `KNOWLEDGE-PLATFORM-PLAN.md` Phase 3
> (the engine `Claim` layer). This document specifies the **authoring surface
> only** — the markdown an author writes — not the engine data model.

---

## 1. Purpose

A *claim* is the atomic unit of the knowledge platform — a first-class,
machine-extractable knowledge object, per Vision §2 ("the claim is the atomic
unit, not the article"). An article becomes an ordered composition of claims
plus connective prose.

This convention defines the **inline markdown syntax** by which an author marks
a span of TOPIC prose as a claim and attaches its metadata. It is
*convention-first* by deliberate sequencing (Decision 1 of the execution plan):
the authoring surface is frozen **before** the engine implements extraction, so
`project-editorial`'s twelve flagship TOPIC rewrites annotate claims **once**,
against a stable target, with no later double-touch.

## 2. Design constraints

Four hard requirements, from Vision §9.1 and Plan §2.1:

1. **Inline — no sidecar.** Claim metadata lives in the TOPIC markdown file
   itself. No companion `claims.yaml`, no separate database in the content
   repo. Markdown-in-Git stays the single source of truth.
2. **Light.** Authoring friction kills adoption. An author types the minimum;
   everything mechanical is computed by the engine (§5).
3. **Graceful degradation.** Claim-annotated markdown renders **correctly and
   unchanged** on *today's* engine — the annotations are invisible and inert.
   No engine change is required for annotated TOPICs to be *tolerated*; engine
   change is required only to *exploit* the annotations.
4. **Future-extractable.** A render-time pass on the future engine recovers the
   structured `Claim` objects deterministically from the markup.

## 3. The carrier — HTML comments

The annotation carrier is the **HTML comment**.

Rationale, verified against the engine: `render.rs:181` sets the comrak option
`render.r#unsafe = true`. With that flag, comrak passes raw HTML — including
comments — through to output verbatim. An HTML comment authored in TOPIC
markdown therefore (a) appears in the served HTML source, (b) renders **nothing**
in any browser, (c) is ignored by every other comrak extension in use
(`render.rs:167–180`), and (d) needs **zero** engine change to be tolerated
today. The future claim-extraction pass scans for these comment markers.

No other markdown construct degrades this cleanly: link syntax, footnote syntax
(`[^n]`), and attribute-span syntax all emit visible chrome. The HTML comment is
the only carrier that is simultaneously inline, inert, and invisible.

### Engine Verification Gate

The graceful-degradation guarantee rests on one fact that **must be checked
before ratification**: that comrak — with the exact option set at
`render.rs:167–182`, `unsafe = true` — emits an authored `<!--claim …-->` /
`<!--/claim-->` pair into the output HTML *unchanged* and *adjacent to the claim
text*, with no effect on surrounding rendering. `render.rs:181` is strong
evidence (raw-HTML pass-through is explicitly enabled), but the spec author has
not executed a render test. **Gate owner:** Phase 3.1, `project-knowledge`. A
one-test confirmation in `render.rs`'s test module discharges this gate.

## 4. Syntax

### 4.1 Claim span — paired markers

A claim wraps a contiguous span of body text between an opening and a closing
marker:

```markdown
<!--claim id=git-canonical cites=[git-scm] confidence=structural-->
PointSav stores every TOPIC as Markdown in a Git repository; every database and
search index is derived state, rebuilt by re-reading the Git tree.
<!--/claim-->
```

- **Opening marker** — `<!--claim KEY=VALUE …-->` — carries the authored fields
  (§4.3).
- **Closing marker** — `<!--/claim-->` — bare; closes the open claim.
- The claim **content** is the rendered text between the markers.
- Markers MAY sit on their own lines (a **block claim** — wraps one or more
  whole paragraphs / list items) or inline within a paragraph (an **inline
  claim** — wraps a clause or sentence).
- Claims **MUST NOT overlap** and **MUST NOT nest** in this version. One claim
  closes before the next opens. Cross-claim relationships are expressed by the
  `depends_on` field (§4.3), not by nesting — this keeps both authoring and
  extraction unambiguous.

### 4.2 Marker grammar

```
opening   := "<!--claim" SP field (SP field)* SP? "-->"
closing   := "<!--/claim-->"
field     := key "=" value
key       := "id" | "cites" | "valid_at" | "confidence" | "depends_on"
value     := bareword | list
list      := "[" [ bareword ("," bareword)* ] "]"
bareword  := [A-Za-z0-9._:-]+      ; no spaces, no quotes
```

- A value or list contains **no spaces** — the marker stays a single,
  regex-tractable token.
- Field order is not significant.
- An **unknown key** is ignored with a linter warning (forward-compatible).
- A **malformed marker** (unparseable) is treated as a plain HTML comment: it
  still degrades gracefully, and the extractor logs it for the editorial linter.

### 4.3 Authored fields

These are the **only** things an author writes. The friction budget is four
fields, two of them usually defaulted.

| Field | Required | Form | Meaning |
|---|---|---|---|
| `id` | **yes** | kebab-case, unique within the file | Local claim identifier. The engine namespaces it globally as `<topic-slug>:<id>`. |
| `cites` | **yes** | `[id1,id2]` of `citations.yaml` registry IDs; `[]` permitted only when `confidence=structural` | The claim's citation set; each ID resolves against the registry (§6). |
| `confidence` | **yes** | closed enum (§4.4) | The claim's epistemic grade. |
| `valid_at` | no | ISO date, `YYYY-MM`, or `YYYY` | When the asserted fact began applying. Omitted ⇒ timeless (valid since topic inception). |
| `depends_on` | no | `[ref,…]` — `<id>` (same file) or `<slug>:<id>` (cross-file) | Other claims this claim's truth rests on; builds the claim graph (Plan §3.3). |

### 4.4 The `confidence` enum (closed)

| Value | Meaning |
|---|---|
| `established` | Multiple independent sources, or a primary source; not in dispute. |
| `reported` | A single source or secondary reporting; true *as reported*, not independently corroborated. |
| `projected` | Forward-looking / planned / intended. The claim text **MUST** use planned/intended/may/target language per `bcsc-disclosure-posture.md`, and the TOPIC frontmatter should carry `forward_looking: true`. |
| `contested` | Sources materially disagree; the claim records the disagreement rather than resolving it. |
| `structural` | A definitional or architectural statement about the platform itself (e.g. "Markdown-in-Git is canonical"). Self-evident from the system; `cites` may be empty. |

## 5. Derived fields — never authored

The engine computes these at extraction time. An author **MUST NOT** write them
into a marker. This split is the friction guarantee of §2.2.

| Field | Source |
|---|---|
| `content_hash` | blake3 of the normalised claim-span text. The engine already computes blake3 over content; a changed hash means the claim text changed. |
| `published_at` | The committer timestamp of the Git commit that last modified the claim span — `git log` is the provenance table (Vision §4a). This is the second clock: `published_at` (when written, from Git) vs `valid_at` (when the fact applies, authored). |
| `revision_sha` | The commit SHA the claim was extracted at. |
| `topic_slug` | The containing TOPIC's slug; used to namespace `id` into the global address `<topic-slug>:<id>`. |

## 6. Relationship to existing mechanisms

- **Frontmatter `cites:`** (article-level, `render.rs:103`) — retained. Once
  claims are pervasive it becomes derivable as the union of every per-claim
  `cites`; during the transition both coexist. The Citation Authority Ribbon
  aggregates *per-claim* verification status.
- **`[^n]` footnotes / `references:`** — retained, unaffected. They remain a
  human-reading convenience. A claim's `cites` points at the **registry**
  (`citations.yaml`) — the content-addressed, continuously re-verified path
  (Vision §2.2) — which is the auditable source of a claim's support.
- **`research_trail` frontmatter** (`draft-research-trail-discipline.md`) —
  unaffected; it documents how the *draft* was researched, orthogonal to claims.

## 7. Identifiers and addressing

- `id` is unique **within a file**. The global address is `<topic-slug>:<id>`.
- Cross-file `depends_on` references use the global form; same-file references
  may use the bare `id`.
- An `id` is a **stable handle.** Renaming an `id` is a breaking change to every
  `depends_on` that references it — treat it like renaming a wikilink target.

## 8. Worked examples

**Structural claim** (no citation needed):

```markdown
<!--claim id=derived-state confidence=structural cites=[]-->
The search index and the link graph are derived state: deleting them and
re-reading the Git tree reproduces them exactly.
<!--/claim-->
```

**Established claim with a registry citation and a validity date:**

```markdown
<!--claim id=tile-format-c2sp cites=[c2sp-tlog-tiles] valid_at=2024 confidence=established-->
The on-disk tile format follows the C2SP tlog-tiles specification verbatim.
<!--/claim-->
```

**Projected claim** (forward-looking — note the "planned" language and the
`projected` grade working together):

```markdown
<!--claim id=monthly-anchor confidence=projected valid_at=2026 cites=[c2sp-tlog-tiles]-->
The platform plans to publish tile checkpoints monthly to a public transparency
log, giving any third party an independent integrity oracle.
<!--/claim-->
```

**Inline claim** with a dependency on the claim above:

```markdown
Because checkpoints are externally anchored, <!--claim id=audit-without-operator
confidence=established cites=[rfc-9162] depends_on=[monthly-anchor]-->an auditor
can confirm a record's integrity without involving the platform operator<!--/claim-->.
```

## 9. Linter hooks

The editorial linter (Plan Phase 8 / `project-editorial` Track D — one ruleset,
two consumers) gains a claim-validation pass:

- every claim has an `id` and a `confidence`;
- `id` is unique within the file;
- `cites` is non-empty unless `confidence=structural`;
- every `cites` ID resolves against `citations.yaml`;
- every `depends_on` reference resolves to an existing claim;
- a `projected` claim's text uses planned/intended/may/target language
  (`bcsc-disclosure-posture.md`);
- markers are balanced and non-overlapping.

## 10. Scope boundary — what this convention does NOT do

- It does **not** define the engine `Claim` struct, the redb claim graph, the
  `query_claims` MCP API, or continuous citation verification — those are
  Phase 3 engine work (`KNOWLEDGE-PLATFORM-PLAN.md` §5).
- It does **not** require any engine change for annotated TOPICs to *render
  correctly* — only for the annotations to be *used*.
- It introduces **no** new files in the content repo and **no** new frontmatter
  schema. Claims live in the body, in comments.

## 11. Adoption sequence

1. **Command Session ratifies** this convention → commits it to
   `~/Foundry/conventions/`.
2. **`project-editorial`** annotates claims during the twelve Track-A2 flagship
   TOPIC rewrites — one pass, against this frozen surface.
3. **Phase 3.1** (`project-knowledge`) implements render-time extraction; the
   §3 Engine Verification Gate is discharged here.
4. New TOPICs are claim-annotated from first authoring; the back-catalogue is
   annotated opportunistically as TOPICs are next touched.

---

*Authored 2026-05-21 by `totebox@project-knowledge` (claude-code) — Phase 2 of
`KNOWLEDGE-PLATFORM-PLAN.md`. Pending Command Session ratification.*
