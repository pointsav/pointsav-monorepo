# Brief: Frontmatter compliance audit — drafts-outbound/ (project-knowledge)

**target**: Read-only validation pass on the 6 staged drafts in `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/` to verify frontmatter compliance with the foundry-draft-v1 schema specified in `cluster-wiki-draft-pipeline.md §2`.
**target_files** (read-only): 6 draft files at `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/`
**expected_output**: One markdown report (per-file table or consolidated table) showing PASS/FAIL/WARN per required field per draft; one-paragraph summary at end with total compliant + remediation guidance for each violation. No files written.
**max_response_lines**: 120
**model_tier**: sonnet
**parallelisable**: yes (read-only)
**confidence_gate_passes**: ≥95% — schema fully specified, files small, mechanical comparison
**layer_scope**: task (read-only scan within own cluster's drafts-outbound)
**anti_slop_check**: catches schema drift before project-language refusal (cluster-wiki-draft-pipeline.md §8 rule 2: "project-language refuses drafts without frontmatter")
**dependencies**: none

## Specification

The sub-agent reads each of 6 draft files, parses YAML frontmatter, validates against the schema:

**Required fields** (all must be present):
`schema`, `state`, `originating_cluster`, `target_repo`, `target_path`, `target_filename`, `audience`, `bcsc_class`, `language_protocol`, `authored`, `authored_by`, `authored_with`, `references`, `notes_for_editor`

**Enum constraints**:
- `state` ∈ {draft-pending-language-pass, draft-in-refinement, draft-refined, draft-archived}
- `audience` ∈ {vendor-public, vendor-internal, customer-public, customer-internal, governance, identity-passport}
- `bcsc_class` ∈ {forward-looking, current-fact, no-disclosure-implication}
- `language_protocol` ∈ {PROSE-TOPIC, PROSE-GUIDE, PROSE-README, COMMS-INBOX, LEGAL-LICENSE, LEGAL-CLA, TRANSLATE-ES}
- `authored_with` ∈ {opus-4-7, sonnet-4-6, haiku-4-5}

**Type constraints**:
- `references:` must be a YAML list (not a scalar or absent)
- `notes_for_editor:` must be a multi-line block string (`|` literal block)

**Files to audit (absolute paths)**:
1. `GUIDE-operate-knowledge-wiki.draft.md`
2. `topic-app-mediakit-knowledge.draft.md`
3. `topic-collab-via-passthrough-relay.draft.md`
4. `topic-collab-via-passthrough-relay.es.draft.md`
5. `topic-documentation-pointsav-com-launch-2026-04-27.draft.md`
6. `topic-substrate-native-compatibility.draft.md`

**Pre-audit observation** (parent context — confirm independently): file 4 (`*.es.draft.md`) may lack the `references:` field; flag if so.

**Output shape**: one markdown table (or per-file tables) with columns: File | Field | Status | Detail. Status ∈ {PASS, FAIL, WARN}. FAIL = required field absent or enum value invalid. WARN = field present but value unusual or ambiguous (inline YAML comment appended to value, etc.). End with one-paragraph summary: total drafts compliant, total with violations, minimum-change remediation per violation.

## Acceptance criteria

- All 6 files audited
- 14 required fields checked per file
- Enum values validated
- Type constraints (`references:` list, `notes_for_editor:` block) checked
- Output table shows PASS/FAIL/WARN per (file, field) pair
- Summary paragraph names total compliant count + remediation per violation
- Cap 120 lines
- No writes

## Risks / unknowns

- File 4 (`-es.draft.md`) may carry additional fields (`language: es`, `companion: …`) beyond the canonical schema; sub-agent should pass these as "extra fields" rather than flag (not in spec but not prohibited)
- Some drafts may carry inline YAML comments (`# project-language decides`); sub-agent should treat these as WARN not FAIL because the parser typically strips them
- File ordering in the report may matter for parent's review pass; alphabetical-by-filename is fine
