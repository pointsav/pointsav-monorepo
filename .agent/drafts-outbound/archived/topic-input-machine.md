---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: pointsav/content-wiki-documentation
target_path: ./
target_filename: topic-input-machine.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: true
bilingual_pair: topic-input-machine.es.md
references:
  - woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
  - woodfine-fleet-deployment/node-console-operator/guide-console-operations.md
  - conventions/architecture-layer-catalog.md (app-console-input, service-input entries)
  - conventions/three-ring-architecture.md (service-input as Ring 1)
  - .agent/plans/leapfrog-2030-coding.md (Phase 4)
research_trail:
  source_commits:
    - "guide-command-ledger.md — F12 as The Anchor, Input Machine definition"
    - "guide-console-operations.md — Input Machine as Anchor in Derivative HUD"
    - "three-ring-architecture.md — service-input as Ring 1 boundary ingest"
  prior_drafts: []
  citations:
    - SYS-ADR-10
    - SYS-ADR-07
  operator_inputs:
    - "Input Machine routes to service-input on the Totebox Archive; service-input classifies+routes+audits (2026-05-20)"
    - "F12 is immovable — The Anchor; cannot be reassigned (2026-05-20)"
    - "All file/text ingest routes through F12; cannot be bypassed from other panes (2026-05-20)"
  related_files:
    - .agent/drafts-outbound/topic-os-console-platform.md
    - .agent/plans/leapfrog-2030-coding.md (Phase 4)
notes_for_editor: |
  Comprehensive first draft. The Input Machine is a foundational architectural element
  — every other TOPIC about os-console refers back to it.

  Refinement priorities:
  - Verify SYS-ADR-10 exact wording in DOCTRINE.md; use citation ID
  - Verify SYS-ADR-07 exact wording; use citation ID
  - Confirm service-input is in the architecture-layer-catalog (it IS in three-ring-architecture.md as Ring 1)
  - Generate bilingual .es.md pair
  - Apply Bloomberg-article register
  - Target length: ~900-1200 words English
---

# The Input Machine

## What it is

The Input Machine is the mandatory ingest gate through which all documents and text
enter the os-console workflows. It is bound to F12 permanently. The F12 assignment is
architectural — it does not move regardless of how other F-key slots are reorganized.

The operational name for this slot is The Anchor. Every cartridge in os-console depends
on the Input Machine for its source material. No workflow bypasses it.

## Why it is immovable

F12 occupies the boundary position on the keyboard's function-key row: physically
separated from F1–F11 by a wider gap on most keyboards. This positioning is intentional.
The Input Machine is not a workflow feature; it is a boundary control. It must be
immediately and unambiguously findable in any moment, by any user, regardless of which
cartridge is currently active.

SYS-ADR-10 establishes F12 as the mandatory human checkpoint for all ingest operations.
It cannot be bypassed by another pane. It cannot be reassigned to a different F-key.
These constraints are enforced in the `app-console-keys` event dispatcher, not
by convention.

## What happens when F12 is pressed

Pressing F12 at any point in os-console — regardless of which cartridge is active —
triggers the Input Machine modal. The currently-active cartridge is suspended. The
operator completes the ingest workflow. The cartridge resumes.

The ingest workflow:
1. **Input gate opens:** A modal presents the file path input field
2. **Operator provides path:** The path is entered and confirmed (explicit confirm
   required — passive submission is not accepted)
3. **Validation:** The Input Machine validates that the file exists and is readable
4. **POST to service-input:** The validated file is sent to `service-input` on the
   Totebox Archive
5. **Classification:** `service-input` classifies the document and determines the
   routing target (which backend service will process it)
6. **Routing:** `service-input` routes the document to the appropriate service
   (e.g., `service-proofreader` for editorial content, `service-fs` for archiving)
7. **Audit entry:** An immutable audit log entry is written: timestamp, file path,
   classification, routing target, operator identity
8. **Return to cartridge:** The active cartridge resumes with the ingested document
   available in its context

## service-input: the Ring 1 boundary service

`service-input` is a Ring 1 service in the Three-Ring Architecture — a per-tenant
boundary ingest service that handles generic document ingestion. It is the server-side
counterpart of the Input Machine cartridge.

Ring 1 placement means `service-input` is:
- Per-tenant (each Totebox Archive runs its own instance)
- A data-ingest boundary (data flows in through it; it depends on nothing downstream)
- An MCP server (implements the Model Context Protocol boundary per the Ring 1 pattern)

`service-input` performs three functions:
1. **Classify:** Determine the document type and appropriate processing pipeline
2. **Route:** Forward the document to the correct Ring 2 or Ring 1 service
3. **Audit:** Write an immutable record of the ingest event to the ledger

`service-input` does not perform AI inference. Classification uses deterministic rules
(SYS-ADR-07: no structured data through AI). This ensures the audit trail is
reproducible and not dependent on model availability.

## The audit trail

Every document that passes through the Input Machine generates an audit trail entry.
The entry is written in two places:

- **Local SQLite log** on the `os-console` machine: timestamp, file path, classification,
  routing target. This local record persists even if the Totebox Archive is unreachable.
- **`service-input` ledger** on the Totebox Archive: the canonical record. Immutable
  once written.

The audit trail serves compliance: every document that entered a workflow is logged
with when it arrived, what it was classified as, where it was sent, and who sent it.

## The app-console-input cartridge

`app-console-input` is the F12 cartridge crate in `pointsav-monorepo` (Scaffold-coded).
It implements the Input Machine workflow on the os-console client side:
- Renders the file path input modal
- Sends the POST to `service-input` on the Totebox Archive (30s timeout)
- Writes the local SQLite audit entry
- Returns control to the previously-active cartridge after ingest completes

`app-console-input` is always installed. The Input Machine modal is always reachable
via F12. This is not configurable.

## SYS-ADR-07 compliance

SYS-ADR-07 states: no structured data passes through AI inference. The Input Machine
enforces this at the ingest boundary. `service-input`'s classification is deterministic
— it uses file extension, MIME type, and structural signatures to classify documents.
It does not call Doorman or any AI service during ingest.

This means the audit trail's classification field is deterministic and reproducible:
given the same file, `service-input` will always produce the same classification. The
audit record is not dependent on model versions or inference availability.

## The Zero-Form architecture

The Input Machine is the foundation of what `guide-console-operations.md` describes
as the Zero-Form architecture. Traditional document workflows require forms: the
operator fills in fields to contextualize the document before it enters a system.

The Input Machine inverts this. The operator provides a document. The system classifies,
routes, and contextualizes it automatically. The only input required from the operator
is the document itself and an explicit confirmation that they intend to submit it.

This is not convenience — it is a compliance design. Explicit confirmation means the
audit trail reflects a deliberate human decision, not an accidental submission. No
document enters a Totebox Archive workflow without the operator's explicit intent.

## F12 in the context of other cartridges

Each cartridge uses the Input Machine for its source material:

- **F4 (Content):** Documents submitted via F12 become the source text for proofreading
  or the context for draft generation
- **F5 (Minutebook):** Meeting notes and resolution documents submitted via F12 are
  processed by the governance workflow
- **F6 (Bookkeeper):** Financial documents submitted via F12 enter the ledger pipeline
- **F7 (BIM):** IFC model files submitted via F12 are routed to `service-bim`

The routing decision — which cartridge receives the document after F12 ingest — is made
by `service-input` based on classification, not by which cartridge happened to be active
when F12 was pressed. The operator presses F12; the system decides where it goes.

## See also

- TOPIC: os-console and app-console-keys — the platform context for the Input Machine
- GUIDE: os-console Operator Reference — F12 usage in practice
- `conventions/three-ring-architecture.md` — service-input Ring 1 placement
- `woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md` — operational reference
