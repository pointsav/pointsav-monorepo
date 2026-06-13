# editorial-audit — Vocabulary and Structure Audit Skill

> **Invoke:** `/editorial-audit`
> **Purpose:** Run a deterministic vocabulary and structure check on a completed draft before
> committing. This is Pass 2 in the two-pass generation/enforcement architecture (see
> BRIEF-artifact-style-guide.md §15). It does not rewrite — it flags. The operator or a
> generation pass resolves each flag.

---

## How to use this skill

After generating a draft, invoke `/editorial-audit` and provide the draft text (paste
inline or reference the file path). The skill emits a severity-tagged list of violations.
Fix Tier 1 issues before committing; Tier 2 and 3 are judgment calls.

---

## Tier 1 — Always flag (block commit until resolved)

These terms are in the workspace Do-Not-Use list or signal AI generation drift. No
exceptions in article body text. Find-and-replace or rephrase.

| Banned term | Replacement guidance |
|---|---|
| leverage (verb) | use, apply, draw on |
| robust | reliable, stable, fault-tolerant, resilient (pick the precise meaning) |
| seamless | smooth, direct, without handoff, frictionless (be specific) |
| utilize | use |
| cutting-edge | name the specific capability instead |
| next-generation | name the version or capability instead |
| groundbreaking | state the specific claim it is meant to support |
| revolutionary | state the specific change instead |
| transformative | state what specifically changed |
| game-changing | state the quantified impact instead |
| state-of-the-art | cite a specific benchmark or standard |
| unprecedented | verify: if true, cite evidence; if not, delete |
| innovative | describe the specific mechanism |
| deploy (metaphor) | use "apply", "use", "introduce" when not referring to software deployment |
| harness (metaphor) | use "use", "apply", "direct" |
| leverage (noun phrase "our leverage") | restate with the concrete mechanism |
| In today's rapidly evolving | delete; begin with the substantive claim |
| It's worth noting that | delete; if worth noting, note it directly |
| When it comes to | delete; rewrite the sentence |
| Let's explore | delete; state what is being explained |
| Consider this | delete; state the claim |
| at the end of the day | delete; make the actual point |
| in the realm of | delete; name the domain directly |
| going forward | delete or use a specific date/version |
| moving forward | delete or use a specific date/version |

---

## Tier 2 — Flag for review (judgment call)

These patterns are common failure modes but not always wrong. Flag and review in context.

**Structural tells:**

- **Bold-label paragraph opener** — `**Term.** Explanation continues.` in body prose.
  Reserve bold for hypothesis labels (H₁, H₀) and CRediT roles in JOURNAL. Use run-in
  italic instead: `*Term.* Explanation continues.` Or convert to a proper H3 subsection.

- **Summary-closer that restates the opener** — if the last sentence of a section says
  what the first sentence said, delete it.

- **Uniform sentence length** — scan a paragraph: if every sentence is 20–25 words, the
  prose is rhythmically flat. Vary: short declarative → medium evidential → longer inferential.

- **Passive-voice clustering** — one passive per section is fine; three in a row signals
  a generation artifact. Rewrite the verb as active.

- **Abstract noun stacks** — "the implementation of a mechanism for the management of"
  → "managing". Each nominalization adds 3–5 words and reduces force.

- **Hedge clustering** — "may potentially suggest that it could be considered" → pick one
  hedge that is epistemically correct and delete the rest.

**Opener patterns (flag if sentence 1 contains any of these):**

- "Consider how…" / "Imagine a…" / "In a world where…"
- "Over the past decade…" / "In recent years…"
- "As [subject] continues to grow…"
- Any sentence beginning with "This [article/guide/paper] will…"

---

## Tier 3 — JOURNAL-specific checks

Run these only on files with `language_protocol: JOURNAL` or `artifact_type: JOURNAL`.

**Abstract density check:**
- Sentence 1 must contain the headline finding AND its magnitude (a number, percentage,
  factor, or comparison). If sentence 1 is a motivation statement, the abstract fails.
- Word count ≤ 250.

**Results completeness:**
- Every section marked `## Results` must contain actual data, not placeholder text
  ("pending Phase X", "to be measured", "data forthcoming"). Empty Results = desk-rejection.

**Citation verification reminder:**
- Emit a warning for every entry in `references:` or every `[^N]` footnote:
  "VERIFY independently — Claude fabricates plausible-but-false DOIs."
- Do not verify DOIs programmatically — flag for human verification.

**Forbidden vocabulary scope:**
- All Tier 1 terms above apply in JOURNAL body text.
- Additionally flag: PointSav, Foundry, Totebox, Doorman, any `service-*` / `app-*`
  internal name, any F-key reference (F12, F11), internal process terms (Stage 6, BRIEF,
  commit-as-next, Totebox Archive, spawn, project-editorial).
- After a full forbidden-vocabulary pass, set `forbidden_terms_cleared: true` in frontmatter.

---

## Per-artifact structural checks

### TOPIC check

- [ ] Sentence 1 of the lede carries a concrete claim (not a definition, not a question)
- [ ] No environment-specific hostnames, port numbers, or one-time commands in body
- [ ] Architecture TOPIC: ≥1 diagram (Mermaid or ASCII) present
- [ ] `cites:` field is non-empty if the article makes load-bearing factual claims
- [ ] `See also` section present with 3–5 wikilinks, each with a one-line rationale
- [ ] `language_protocol:` is `PROSE-TOPIC`, not bare `TOPIC`
- [ ] ES pair exists if article is in a wiki-published category
- [ ] `l10n.sourceCommit: <SHA>` present in ES pair frontmatter

### GUIDE check

- [ ] Scope paragraph (first paragraph, no heading) names: what the guide does, where
      commands run (OS / service / deployment name), end state, time to complete
- [ ] `**BLAST RADIUS:**` one-sentence statement present at top of Procedure section
- [ ] `last_verified:` frontmatter field present (distinct from `last_edited:`)
- [ ] Verification section exists with copy-paste commands and expected output
- [ ] Rollback section exists (or explicit "no rollback — see X" statement)
- [ ] Guide contains no concept explanations — those belong in the linked TOPIC
- [ ] `language_protocol:` is `PROSE-GUIDE`, not bare `GUIDE`

### JOURNAL check

All Tier 3 checks above, plus:
- [ ] All 22 mandatory sections present (per `journal-artifact-discipline.md` §22)
- [ ] ≥3 figures with formal captions (`**Figure N.** Caption.`) before `under-review` promotion
- [ ] `forbidden_terms_cleared: false` until a dedicated language-pass session verifies
- [ ] `orcid:` field non-empty for all authors before `under-review`
- [ ] `submission_status:` reflects actual state (not stale)

### COMMS check

- [ ] Sub-type declared (`COMMS-ANNOUNCEMENT`, `COMMS-PRESS`, `COMMS-CORPORATE`,
      `COMMS-EMAIL`, `COMMS-NOTES`) — bare `COMMS` is deprecated
- [ ] Sentence 1 carries the ask, decision, or announcement (not context)
- [ ] FLS paragraph present if article contains forward-looking claims (COMMS-ANNOUNCEMENT,
      COMMS-PRESS)

---

## Core prose rules (reference)

These are the generation-time rules that this audit enforces at review time:

1. **Hard fact opener.** Never "Consider / Imagine / In a world where."
2. **State the claim.** Do not hedge by default — hedge only where epistemic precision requires.
3. **Concrete nouns, vivid verbs.** No abstract noun stacks.
4. **One idea per sentence. Active voice. Cut every unnecessary word.**
5. **Narrative, not textbook.** Write as if the reader will keep reading, not scan for bullets.
6. **First sentence of each paragraph = standalone claim.** Body = evidence. Closer = implication.

---

## Output format

Emit findings as a numbered list with severity prefix:

```
[T1] BLOCKED — "leverage" in §3, line 47 → replace with "use" or name the mechanism
[T2] REVIEW  — bold-label opener "**DataGraph.**" in §2 → convert to run-in italic or H3
[T3] JOURNAL — Abstract sentence 1 is motivational, not a quantified claim → rewrite
[OK] TOPIC structural checks: all pass
```

Finish with a one-line summary: `N T1 blockers, M T2 reviews, K T3 journal notes.`

If zero T1 blockers: `CLEAR — no Tier 1 violations. Review Tier 2 and 3 items before committing.`
