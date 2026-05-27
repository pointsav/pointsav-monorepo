---
mailbox: outbox
owner: totebox@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-editorial Totebox

---
from: totebox@project-editorial
to: command@claude-code
re: convention-layer updates required — JOURNAL artifact type introduction
created: 2026-05-27T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260527-journal-convention-request
---

project-editorial has introduced the JOURNAL artifact type this session.
Six JOURNAL files now exist in `.agent/drafts-outbound/`. The local rules
file is at `.agent/rules/journal-artifact-discipline.md` (schema, forbidden
vocabulary, structural requirements, promotion criteria, author rules).

Four convention-layer changes are needed at Command Session scope:

**1. `conventions/artifact-classification.yaml` — add JOURNAL entry**

```yaml
- id: JOURNAL
  description: "Peer-reviewed academic paper. Named natural-person authors only. No internal Foundry branding or vocabulary."
  gateway: project-editorial
  destinations:
    - target_journal (external submission)
    - drafts-outbound (staging)
  schema: foundry-journal-v1
  frontmatter_required: true
  bilingual_pair: false
  note: "Distinct from PROSE-RESEARCH (scaffolding). JOURNAL is the promotion target when falsification programme is stable and literature gap is established."
```

**2. `conventions/journal-artifact-discipline.md` — new convention file**

Copy or symlink from project-editorial's `.agent/rules/journal-artifact-discipline.md`.
This file contains: mandatory 22-section structure, frontmatter schema, forbidden vocabulary
list, author rules, BCSC posture, AI disclosure (COPE 2024), CRediT roles, promotion
criteria, and submission workflow. It is the canonical workspace-level specification for
all JOURNAL artifacts across all clusters.

**3. `conventions/artifact-registry.md` — add JOURNAL section**

Add a `JOURNAL` row to the artifact type listing. Point to
`project-editorial` as gateway. Note: schema `foundry-journal-v1`.

**4. `NEXT.md` — add JOURNAL programme tracking item**

Suggested checkbox:
```
- [ ] JOURNAL programme — 6 papers (J1–J6) at project-editorial; J1/J2 scaffolded; J3 scaffolded; J4–J6 stub. Pre-submission blockers: language pass (all), ORCID IDs (all), bench #9 re-run (J2). [project-editorial 2026-05-27]
```

The local rules file at project-editorial is the source of truth for the
convention content until Command Session copies/adapts it to `conventions/`.

---
