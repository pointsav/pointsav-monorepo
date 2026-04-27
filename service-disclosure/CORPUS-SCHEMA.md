# CORPUS-SCHEMA.md — service-disclosure

JSONL tuple shape for the editorial-apprenticeship corpus. Extends
`~/Foundry/conventions/apprenticeship-substrate.md` §8 to the
editorial task-types declared by `project-language` (see the
cluster manifest's `apprenticeship_task_types`).

The substrate convention defines a code-shaped tuple. This
document specialises it to editorial work without forking the
record schema — the same `tuple_type: apprenticeship`, the same
brief / attempt / verdict triple, the same redaction discipline.

---

## 1. Where the corpus lives

```
~/Foundry/data/training-corpus/apprenticeship/<task-type>/<tenant>/<ulid>.jsonl
```

One file per (brief, attempt, verdict) triple. Filename is a ULID
so chronological scan is filesystem-ordered.

Per-tenant partitioning is mandatory at the directory level.
Tenant-private records never leave the tenant's substrate per
DOCTRINE.md §IV.b. Cross-tenant aggregation, where it happens,
runs against records explicitly marked `redaction_class:
federable` or `public`.

The eight editorial task-types declared by the cluster manifest
(`prose-edit`, `comms-edit`, `frontmatter-normalize`,
`citation-insert`, `register-tighten`, `cross-link-verify`,
`schema-validate`, `template-author`) each get their own
directory tree under `apprenticeship/`. Promotion / demotion
state is per-task-type per the substrate convention §6.

## 2. Record shape

Every line is one JSON object. Required top-level fields match
`apprenticeship-substrate.md` §8 verbatim; editorial task-types
constrain the inner shapes.

```json
{
  "tuple_type": "apprenticeship",
  "doctrine_version": "0.0.8",
  "task_type": "prose-edit",
  "stage_at_capture": "review",
  "brief": { ... },
  "attempt": { ... },
  "verdict": { ... },
  "final_diff": "<unified diff or null>",
  "redaction_class": "internal",
  "evidence_class": "primary",
  "tenant": "pointsav",
  "cluster": "project-language",
  "session_id": "<id>",
  "created": "2026-04-27T14:32:18Z"
}
```

`tuple_type` is always `apprenticeship` for records produced by
this pipeline. Foundry-wide trajectory captures continue to use
`tuple_type: trajectory` per `trajectory-substrate.md`; the two
streams are distinct.

`stage_at_capture` follows the substrate's stage model: `review`,
`spot-check`, `autonomous`, or `shadow`.

## 3. The `brief` payload — editorial specialisation

Briefs are senior-authored requests for editorial work on a
specific document or fragment. Editorial task-types share the
substrate's `foundry-apprentice-brief-v1` schema with three
additional fields specific to language-protocol routing:

```yaml
---
schema: foundry-apprentice-brief-v1
brief_id: <ulid>
created: <ISO 8601>
senior_role: master | root-<repo> | task-<cluster>
senior_identity: ps-administrator | jwoodfine | pwoodfine | mcorp-administrator
task_type: prose-edit | comms-edit | frontmatter-normalize | citation-insert | register-tighten | cross-link-verify | schema-validate | template-author
scope:
  cluster: <cluster-name | null>
  files: [<path>, ...]
  fragment_anchor: <heading-or-line-range | null>   # narrow scope to a section
acceptance_test: |
  <plain-text test the apprentice should make pass>
doctrine_citations: [<citation-id>, ...]
shadow: true | false
# Editorial-specific fields ↓
language_protocol:
  family: prose | comms | legal | translate
  template: <GenreTemplate kebab-form>
  register: bloomberg | operational | technical | casual | legal | null
target_audience: <free-form descriptor or null>
target_language: <BCP 47 tag or null>
---

<brief body — plain English. State what is being edited and why.
Cite doctrine clauses by ID. Reference the genre template by
kebab-case name (e.g., `topic`, `readme-project`).>
```

The three editorial fields (`language_protocol`, `target_audience`,
`target_language`) mirror the `ProtocolRequest` struct in
`service-disclosure`. The Doorman composes the apprentice's
request prompt by reading `language_protocol.template`, calling
`get_template`, and concatenating the result with the brief body.

## 4. The `attempt` payload — editorial specialisation

Attempts carry the substrate's `foundry-apprentice-attempt-v1`
schema with one additional field:

```yaml
---
schema: foundry-apprentice-attempt-v1
brief_id: <ulid>
attempt_id: <ulid>
created: <ISO 8601>
model: olmo-3-1125-7b-q4 | olmo-3-1125-32b-think-q4
adapter_composition: [<adapter-id>, ...]
self_confidence: 0.0 - 1.0
escalate: true | false
inference_ms: <int>
tier: local | yoyo
cost_usd: <float>
# Editorial-specific field ↓
banned_vocabulary_hits: [<string>, ...]
---

## Reasoning

<apprentice's chain-of-thought citing brief invariants and
doctrine clauses.>

## Diff

```diff
<unified diff against the source file, or empty if escalate=true>
```
```

`banned_vocabulary_hits` lists any banned-vocabulary terms the
apprentice's draft would have introduced (per
`BANNED_VOCABULARY` in `service-disclosure`). When Phase 1B's
decode-time CFG ships, this field becomes structurally
unreachable — the constraint blocks the tokens before generation.
Until then, the field surfaces what the prompt-only constraint
missed and feeds DPO refinement.

## 5. The `verdict` payload — editorial specialisation

Verdicts use the substrate's `foundry-apprentice-verdict-v1`
schema unchanged:

```yaml
---
schema: foundry-apprentice-verdict-v1
brief_id: <ulid>
attempt_id: <ulid>
verdict: accept | refine | reject | defer-tier-c
created: <ISO 8601>
senior_identity: ps-administrator | jwoodfine | pwoodfine | mcorp-administrator
final_diff_sha: <commit-sha or null>
notes: |
  <required for refine / reject; one sentence on why>
---

<verdict body, free-form prose>
```

Editorial task-types add no fields. The `notes` field carries
the editorial reason on `refine` or `reject` — for example
"`leverage` slipped past the prompt-only constraint" or
"register drifted to marketing". These notes are the highest-
signal training data the corpus produces.

`defer-tier-c` is used when the apprentice's draft is competent
but the task should run on Tier C anyway — for instance, a
LEGAL-family brief where the senior wants Claude or GPT to
author from scratch despite the apprentice's reasonable attempt.

## 6. Redaction discipline

Every brief, attempt, verdict, and `final_diff` passes the
substrate's redaction filter at capture time per
`apprenticeship-substrate.md` §9: PEM keys, AWS / OpenAI /
Anthropic / GitHub / Slack tokens, and generic ≥32-char
bearer / API-key patterns are stripped before write.

Editorial task-types carry one additional concern: customer
text. A `prose-edit` brief on a Customer's draft email contains
that draft email. The redaction filter does not strip prose
content — that is the training signal. Tenant isolation
(directory-level partitioning per §1) is the boundary that
keeps customer text inside the customer's substrate.

## 7. DPO triple on `refine` / `reject`

Per `apprenticeship-substrate.md` §8 final paragraph, a `refine`
or `reject` verdict produces a Direct Preference Optimisation
triple:

```
(rejected_attempt, corrected_diff, doctrine_violation_tag)
```

Written to:

```
~/Foundry/data/training-corpus/feedback/apprenticeship-<task-type>-<ulid>.jsonl
```

For editorial task-types, the `doctrine_violation_tag` is one
of:

- `banned-vocabulary-hit` — apprentice emitted a banned term
- `register-drift` — output register diverged from brief
- `frontmatter-invalid` — frontmatter validator rejected the diff
- `citation-missing` — claim-shaped sentence without `[citation-id]`
- `bilingual-pair-broken` — added section to one language only
- `template-non-conformance` — required-section omitted

The tag set is closed; new tags require a `task-type-add`-shaped
ledger event so promotion threshold computations remain stable.

## 8. Cross-cluster contract

`project-proofreader` is the operational consumer that produces
production briefs at scale. Its `service-proofreader` (HTTP
write-assistant) emits one apprenticeship tuple per inbound
request when the operator is in P1 or P2 routing per the
substrate convention §7.

`project-language` is the substrate curator: it ships
`service-disclosure` (this crate), authors the genre templates,
and reviews the verdict ledger for promotion / demotion signals.
It does not produce production briefs at scale; it produces
seed briefs during template authoring (each genre template
needs one or more worked-example briefs to bootstrap the
corresponding adapter).

`project-slm` is the inference substrate: `service-slm`'s
Doorman serves the apprentice attempts and writes the JSONL
records to disk per §1.

The three clusters compose. None of them holds the full
pipeline alone.

## 9. Migration from the v0.1 stub

`service-proofreader` runs on hardcoded protocol templates
until the schema-stable signal is emitted (Phase 1B + Master
ratification). During the stub period, briefs may carry
`schema: foundry-apprentice-brief-v1-stub` and verdict records
may omit the editorial-specific fields. These records remain
valid corpus tuples — the substrate's redaction discipline and
record shape apply unchanged — but they do not feed adapter
training until the schema-stable signal lands and a corpus
sweep retrofits the editorial fields.

The migration is one-way: once a record carries
`schema: foundry-apprentice-brief-v1`, it never reverts to the
stub schema. New deployments after schema-stable issuance ship
without the stub path.
