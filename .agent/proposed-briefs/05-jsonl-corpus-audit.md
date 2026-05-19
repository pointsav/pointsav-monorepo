# Brief: Apprenticeship corpus integrity audit — prose-edit/pointsav

**target**: Read-only integrity audit of the 6 JSONL files in `~/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/` produced by the project-knowledge cluster. For each file: parse as JSON, check required fields, verify event type, verify `raw` content matches the staged draft, and verify `metadata` block against the foundry-draft-v1 schema.
**target_files** (read-only):
- 6 JSONL files at `/home/mathew/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/draft-2026-04-2{7,8}-*.jsonl`
- 6 corresponding draft files at `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/{topic-*,GUIDE-*}.draft.md` and the `.es.draft.md` sibling
**expected_output**: One markdown discrepancy report (or "all 6 clean") returned as text only; no files written
**max_response_lines**: 200
**model_tier**: sonnet
**parallelisable**: yes (read-only — per v0.1.30 §1A rule 2 the 6 file pairs may be read in parallel)
**confidence_gate_passes**: yes — mechanical schema validation with deterministic pass/fail
**layer_scope**: task — corpus path readable from Task scope per CLAUDE.md §11 v0.1.31 amendment (Tasks have explicit write permission to apprenticeship corpus path; read is implied)
**anti_slop_check**: catches schema drift before project-language's `bin/draft-sweep.sh` picks up (cluster-wiki-draft-pipeline.md §1.2) — sweep ratification depends on JSONL integrity for Stage-1 DPO pair construction
**dependencies**: none — all files exist on disk

## Specification

The sub-agent walks the 6 JSONL files in parallel, parsing each as JSON (single object, not stream — per cluster-wiki-draft-pipeline.md §7 the `draft-created` event is one object per file). For each file, runs the 12-check audit:

| # | Check |
|---|---|
| 1 | File parseable as JSON (single object) |
| 2 | Required top-level fields present: `event`, `task_type`, `tenant`, `cluster`, `draft_id`, `raw`, `metadata`, `created` |
| 3 | `event == "draft-created"` |
| 4 | `task_type == "prose-edit"` |
| 5 | `tenant == "pointsav"` |
| 6 | `raw` byte-for-byte matches the corresponding draft file content (read draft, compare; report length mismatch or content diff) |
| 7 | `metadata` has all 8 keys: `target_repo`, `target_path`, `target_filename`, `language_protocol`, `bcsc_class`, `audience`, `authored`, `authored_with` |
| 8 | `authored_with` ∈ {opus-4-7, sonnet-4-6, haiku-4-5} |
| 9 | `language_protocol` ∈ {PROSE-TOPIC, PROSE-GUIDE, PROSE-README, COMMS-INBOX, LEGAL-LICENSE, LEGAL-CLA, TRANSLATE-ES} |
| 10 | `bcsc_class` ∈ {forward-looking, current-fact, no-disclosure-implication} |
| 11 | `audience` ∈ {vendor-public, vendor-internal, customer-public, customer-internal, governance, identity-passport} |
| 12 | `created` parseable as ISO 8601 datetime |

The corresponding draft mapping:

| JSONL | Draft file |
|---|---|
| `draft-2026-04-27-guide-operate-knowledge-wiki.jsonl` | `GUIDE-operate-knowledge-wiki.draft.md` |
| `draft-2026-04-27-topic-app-mediakit-knowledge.jsonl` | `topic-app-mediakit-knowledge.draft.md` |
| `draft-2026-04-27-topic-documentation-pointsav-com-launch.jsonl` | `topic-documentation-pointsav-com-launch-2026-04-27.draft.md` |
| `draft-2026-04-27-topic-substrate-native-compatibility.jsonl` | `topic-substrate-native-compatibility.draft.md` |
| `draft-2026-04-28-topic-collab-via-passthrough-relay.jsonl` | `topic-collab-via-passthrough-relay.draft.md` |
| `draft-2026-04-28-topic-collab-via-passthrough-relay-es.jsonl` | `topic-collab-via-passthrough-relay.es.draft.md` |

Output structure:

```
## Audit: apprenticeship/prose-edit/pointsav — 6 JSONL files
Audited: <ISO 8601 datetime>

### Summary
<"All 6 clean." OR "N discrepancies found across M files.">

### Per-file results
#### 1. <jsonl-name>
- Parse: OK | FAIL
- Required fields: OK | MISSING: [list]
- event=draft-created: OK | FAIL (found: "...")
- raw match: OK | MISMATCH (details)
- metadata block: OK | MISSING: [list] | INVALID: [field: value]

[repeat for each file 1–6]

### Discrepancy index (if any)
[Each discrepancy as a numbered line: file, check, finding]
```

## Acceptance criteria

- All 6 files audited
- Per-file results enumerated for all 12 checks
- Any discrepancies listed in the index section with file + check + finding
- "All 6 clean" stated explicitly if no findings
- Cap 200 lines

## Risks / unknowns

- The `raw` byte-for-byte match check is the most likely to fail because trailing whitespace or newline differences between Python's `pathlib.read_text()` (which produced the JSONL) and a manual file read may surface; sub-agent should report exact byte-count if mismatch found
- File 6 (`-es.draft.md`) was authored 2026-04-28; some metadata fields (e.g. `companion`, `language: es`) are present that may not be in the canonical schema enumeration — sub-agent should pass these as "extra fields" rather than fail the audit (the 8-key minimum is required; additional fields are allowed)
