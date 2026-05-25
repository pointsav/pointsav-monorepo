# Genre templates

Skeletons for the genres the editorial gateway refines. Copy a template,
rename it to the target filename, fill every `<PLACEHOLDER>`, and lint with
`editorial-lint.py` before staging. Each template carries a frontmatter stub,
the required-section headings, and inline register reminders.

These are the Track D / D3 deliverable. Their eventual home is the
`service-disclosure/` crate's template registry (editorial plan §5 D3); until
that crate is scaffolded they live here.

## The 16 templates, by family

| Family | Templates |
|---|---|
| PROSE | `template-topic`, `template-guide`, `template-readme`, `template-memo`, `template-architecture`, `template-inventory`, `template-changelog`, `template-license-explainer` |
| COMMS | `template-email`, `template-chat`, `template-ticket-comment`, `template-meeting-notes` |
| LEGAL | `template-contract`, `template-cla`, `template-policy`, `template-terms` |

## TRANSLATE

TRANSLATE is a pipeline pattern, not a single skeleton: every TOPIC, README,
and bilingual policy ships an `.es.md` strategic adaptation alongside its
English file. The adaptation is not a 1:1 translation — it translates the
orientation a Spanish-reading audience needs and drops the deeper
implementation detail (`reference/style-guide-topic.md`, "Bilingual pair
required"). Each PROSE and LEGAL template above that targets the main wikis is
shipped as an EN + ES pair.

## Standard applied

All templates conform to the Gate-0 editorial standard
(`../editorial-standard.md` §1) and the corpus schema (`../CORPUS-SCHEMA.md`).
