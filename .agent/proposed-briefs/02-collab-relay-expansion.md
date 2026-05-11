# Brief: Expand topic-collab-via-passthrough-relay skeleton to full bulk draft

**target**: Replace all 7 `(draft-pending — substance follows in milestone N+1)` placeholder sections in the English and Spanish draft files with substantive prose drawn from the Step 7 implementation specifics.
**target_files**:
- `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-collab-via-passthrough-relay.draft.md`
- `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-collab-via-passthrough-relay.es.draft.md`
**expected_output**: Both files edited in-place; all `(draft-pending…)` placeholders replaced with substantive prose; `draft_shape` frontmatter field updated from `skeleton-only` to `bulk-draft` in both files; `authored_with` updated to `sonnet-4-6`.
**max_response_lines**: 350
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes
**layer_scope**: task
**anti_slop_check**: `state: draft-pending-language-pass` items in drafts-outbound are what `bin/draft-sweep.sh` picks up at project-language session start (cluster-wiki-draft-pipeline.md §1.2); a substantive bulk draft here is the direct prerequisite for that pickup to be non-trivial.
**dependencies**: none — source material is fully committed; no operator gate required.

## Specification

Expand §1 "The pattern in one paragraph" to 2–3 sentences stating the core inversion: the passthrough relay holds no document state; authority stays with the canonical git tree; the CRDT overlay is session-ephemeral. Cite the `tokio::sync::broadcast` per-slug room model as the mechanism and the `POST /edit` atomic-write path as the sole persistence boundary.

Expand §2 "Why a passthrough relay (vs a CRDT server)" to 3–4 paragraphs covering: (a) the contrast with Etherpad/HackMD-style server-authoritative document stores where a second durable state record exists in parallel with git; (b) how the passthrough design eliminates that second record — the server is a message conduit, not a store; (c) why this matters specifically for the disclosure-substrate posture per Doctrine claim #29: the canonical disclosure record is git, so collab sessions leave no shadow state that could drift from the committed record. The 256-message lag buffer (from STEP-7-COLLAB-SMOKE.md §1) resolves the late-joiner race without server-side persistence.

Expand §3 "The implementation in `app-mediakit-knowledge`" to cover the specific implementation from STEP-7-COLLAB-SMOKE.md and PHASE-2-PLAN.md Step 7: `tokio::sync::broadcast` channel per slug (256-message buffer); WebSocket route `GET /ws/collab/{slug}` via axum WS upgrade; the `window.WIKI_COLLAB_ENABLED` template flag that gates lazy-load of `cm-collab.bundle.js` (~302 KB; yjs + y-codemirror.next + y-websocket); the CLI `--enable-collab` flag pattern (default-off); existing `POST /edit/{slug}` atomic-write path as unchanged sole persistence boundary. State the test coverage (unit: WS route accepts, broadcast rooms multiplex, 256-message buffer drains without panic, client bundle loads on flag; integration: 4 additional tests per cleanup-log 2026-04-27).

Expand §4 "What the server doesn't carry" as a concise enumerated list: no Yjs Rust port (`yrs` crate not used — the server relays raw Yjs protocol messages, not document state); no document-state persistence at the relay layer; no auth-bearing identity longer than the WebSocket lifetime; no rate-limit on the relay path beyond axum defaults; no admin-accessible edit-history on the server side (because no server-side history exists to inspect).

Expand §5 "What this means for disclosure posture" in 2–3 paragraphs: in-flight CRDT state is not persisted and therefore not part of the disclosure record; saved edits enter the disclosure record at `POST /edit` time as ordinary git commits — the same path as non-collab edits; the relay restart on `--enable-collab` flag removal or service restart is non-destructive because the CRDT overlay was ephemeral by construction (per STEP-7-COLLAB-SMOKE.md §5 rollback).

Expand §6 "Generalising beyond the wiki" with the three candidate cases from the skeleton notes: `service-extraction` multi-author review pipeline; `app-workplace-presentation` deck-collab; `app-workplace-proforma` table-collab. For each, state the relevant canonical storage type and ask the framing question: would a stateful CRDT server compete with that canonical storage for authority? The answer governs whether the passthrough pattern applies.

For the Spanish file: expand the same seven sections as a strategic-adaptation overview per DOCTRINE.md §XII — preserves the structural points; Spanish prose may compress §3 + §4 into one implementation section if that serves a tighter treatment. Frontmatter `notes_for_editor` permits this compression explicitly; the agent may exercise it. Bulk discipline applies: technical depth in, register discipline out (project-language pass applies that).

Both files: update `draft_shape: skeleton-only` → `draft_shape: bulk-draft` and `authored_with: opus-4-7` → `authored_with: sonnet-4-6` in the YAML frontmatter. Leave all other frontmatter fields unchanged.

## Acceptance criteria

- All `(draft-pending — substance follows in milestone N+1)` and `(borrador-pendiente…)` placeholders replaced in both files.
- `draft_shape` field reads `bulk-draft` in both files.
- `authored_with` field reads `sonnet-4-6` in both files.
- §3 names `tokio::sync::broadcast`, the 256-message buffer, `cm-collab.bundle.js`, and `--enable-collab` specifically.
- §5 establishes that `POST /edit` is the sole persistence boundary and that in-flight CRDT state is non-disclosable by construction.
- §7 References section contains inline URLs for: Yjs (https://github.com/yjs/yjs), CodeMirror collab integration (https://codemirror.net/), tokio::sync::broadcast (https://docs.rs/tokio/latest/tokio/sync/broadcast/), and internal relative paths to PHASE-2-PLAN.md §1 Step 7, STEP-7-COLLAB-SMOKE.md, and ARCHITECTURE.md §11.
- No register-discipline self-application (bulk discipline: repetition OK, inline URLs OK, Bloomberg-standard phrasing not required).
- No new files created; both edits are in-place on the two draft files.

## Risks / unknowns

- `ARCHITECTURE.md §11` and `UX-DESIGN.md §4.7` are referenced in the skeleton frontmatter but not read by this brief; the sub-agent may cite them by path in §7 without reading them. If §3 expansion needs those sections, the agent should read them first via the Read tool.
- The Spanish compression decision (§3 + §4 merged) is at the agent's discretion per the frontmatter `notes_for_editor`. If it produces two distinct sections instead, that is also acceptable.
- The CRDT implementation in the shipped code uses `tokio::sync::broadcast` as a passthrough (no `yrs`); the original PHASE-2-PLAN.md Step 7 spec called for `yrs`. The cleanup-log 2026-04-27 entry confirms the passthrough design was what actually shipped. The brief asks the agent to describe the as-shipped design, not the plan's original spec.
