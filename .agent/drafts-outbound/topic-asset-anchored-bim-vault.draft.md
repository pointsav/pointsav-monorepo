---
schema: foundry-draft-v1
state: draft-pending-editorial-pass
originating_cluster: project-bim
target_repo: vendor/content-wiki-documentation
target_path: topic-asset-anchored-bim-vault.md
audience: vendor-public
bcsc_class: vendor-public
language_protocol: PROSE-TOPIC
authored: 2026-05-17T00:00:00Z
authored_by: totebox@project-bim
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - workspace-tier sub-agent C — `~/Foundry/.claude/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md`
  - strategic source — `~/Foundry/BIM_Buildable Architecture.md`
  - manifest.md §deployment — cluster-totebox-property vault layout
research_inline: true
references:
  - cites: [iso-16739-1-2024, iso-19650-1, bcf-3-0, ids-1-0, ni-51-102, osc-sn-51-721]
notes_for_editor: |
  Technical reference article. Audience spans property owners,
  project managers, and BIM operators. The "travels with the land
  title" framing is strategic — keep it, but ensure it is grounded
  in the technical description first so it reads as conclusion, not
  marketing.
  BCSC vendor-public: no forward-looking revenue claims; "intended"
  and "planned" for future capabilities.
---

# The asset-anchored BIM vault

A building's authoritative digital record is a directory of plain-text
and standardised-binary files that live on the owner's storage,
travel with the property deed when ownership changes hands, and remain
readable without a proprietary software licence for as long as the
underlying open standards are maintained. This document describes the
vault layout, the versioning layer that gives the archive git-grade
traceability, and the ISO 19650 qualification that makes a flat-file
Git repository a conforming Common Data Environment.

## The vault layout

The canonical vault for a Woodfine property archive is structured as
follows:

```
vault/
├── ifc/             # Authoritative IFC-SPF files (ISO 16739-1:2024)
├── elements/        # Per-element YAML sidecars (Pset_* + sensor + work-order)
├── bcf/             # BCF 3.0 per-topic directories (XML + PNG; unzipped)
├── ids/             # IDS 1.0 validation contracts (per-jurisdiction overlay)
├── materials/       # Material database (flat files; service-materials input)
├── codes/           # Building-codes-as-composable-geometry overlays
│                    #   (bsdd-*.json + *.ids + *.ifc fragments per jurisdiction)
├── geometry/        # glTF 2.0 + CityJSONSeq (regenerable caches; not canonical)
├── drawings/        # SVG 2D drawings (regenerable; IFC GUIDs in SVG element IDs)
├── objects/<hash>.json  # Hash-addressed object store
└── refs/            # Git-style ref pointers (branches, tags, HEADs)
```

The `.ifc` files are the sole authoritative spatial and semantic state
of the building. Every other directory either validates the IFC state,
enriches it with non-geometric data, or caches a derivative
representation that can be regenerated from the canonical source.

## The IFC-SPF file as canonical archive

IFC-SPF is the STEP Physical File encoding of IFC, specified in ISO
10303-21. It is a line-oriented clear-text format: a person with a
text editor can read an IFC-SPF file. A Unix diff of two IFC-SPF
files shows exactly which entities changed between model versions.
A grep finds all instances of a specific element type.

The format has been in production since IFC 1.0 in 1996. IFC 4.3,
published as ISO 16739-1:2024, is the current revision. The standard's
governance model — maintained by buildingSMART International, ratified
by ISO — provides the longest credible shelf life of any building data
format in use today.

## Per-element YAML sidecars

Each IFC element that carries non-geometric operational data has a
corresponding YAML sidecar in `vault/elements/`. The sidecar
filename is the IFC GUID of the element, which is stable across model
revisions: `{1a2b3c4d-...}.yaml`.

The sidecar can carry:

- **Pset overrides** — non-geometric property values that were not
  captured in the IFC authoring session
- **Sensor readings** — timestamped JSON records from an MQTT broker
  (temperature, CO₂, occupancy) written as append-only log entries
- **Work orders** — references to maintenance tasks, inspection records,
  and repair history by work-order ID
- **Lease references** — tenant identifier and lease term, linking the
  spatial element to the lease register

Because the sidecar is a plain YAML file in the same git repository as
the IFC file, every change to sensor data or work-order history is a
git commit. The building's operational history is version-controlled
alongside its geometry.

## The hash-addressed object store

The `vault/objects/` directory implements a Speckle-inspired hash-
addressed object store. Each object is a JSON file whose filename is
the SHA-256 hash of its content. `vault/refs/` holds named pointers
— branches, tags, and HEAD — that resolve to specific object hashes.

This architecture gives the vault git-like content-addressable
semantics independently of the git repository that wraps it. An
element state at any point in time can be referenced by its object
hash. A "commit" in the object store is an object that lists the
hashes of all elements at a given point in time. The result is a
Merkle DAG: the root hash of a model state is cryptographically
bound to every element it contains.

The Merkle structure is architecturally significant for two reasons:

1. **Audit trail integrity.** A claimed historical state of the model
   can be verified against the root hash without trusting the server
   that stored it. This satisfies the audit-trail requirements of BCSC
   continuous-disclosure posture and, in jurisdictions that recognise
   it, functions as a tamper-evident record.

2. **Efficient delta transfer.** When two parties synchronise vaults,
   only the objects whose hashes differ need to transfer — the same
   efficiency principle that makes git fast over slow connections.

## ISO 19650 qualification

ISO 19650 defines a Common Data Environment (CDE) as a system for
collecting, managing, and disseminating information in structured
containers. The standard is technology-neutral: it specifies what a
CDE must provide, not how.

A Git repository hosting a vault directory qualifies as a CDE under
ISO 19650 with the following mapping:

| ISO 19650 concept | Git + vault implementation |
|---|---|
| Information container | IFC file or YAML sidecar (one per element) |
| Container UID | Git object hash or IFC GUID |
| Status | Branch name (`work-in-progress`, `shared`, `published`) |
| Revision | Git commit hash |
| Classification | Directory path (`vault/ifc/`, `vault/bcf/`) + YAML header |
| Change history | `git log --follow <filename>` |
| CDE workflow states | Git branch merge / pull-request workflow |

The CDE does not need to be a cloud service. A local Git repository
on an air-gapped workstation satisfies ISO 19650 as fully as a hosted
platform. This is what makes the vault architecture appropriate for
ITAR defence projects, EU Data Act jurisdictions, and HIPAA-governed
healthcare facilities.

## Vendor-obsolescence survivability

Buildings are typically designed to stand for 50 to 100 years. The
software tools used to author BIM models typically change format with
every major release and become unreadable by competing tools within
a decade.

The vault architecture addresses this asymmetry in two ways:

**First**, the canonical formats — IFC-SPF, BCF 3.0, IDS 1.0, YAML —
are ISO-governed open standards or widely adopted plain-text formats.
Any competent engineer can write a reader for IFC-SPF from the ISO
specification without access to proprietary SDKs. The file does not
need a specific vendor's software to remain legible.

**Second**, the regenerable derivatives — glTF visualisation caches,
SVG 2D drawings — are explicitly marked as non-canonical. If the
tool that generated them disappears, the canonical IFC file remains
and any IFC-to-glTF or IFC-to-SVG converter can regenerate them.
The viewer is replaceable; the archive is not.

## The archive travels with the land

A physical property asset transfers with a title deed. The Woodfine
vault is intended to be bundled with that transfer: the same git
repository that holds the IFC archive, the lease register references,
and the operational history accompanies the property when ownership
changes hands.

No cloud platform can make this guarantee. A multi-tenant SaaS
platform holds the digital twin on behalf of the current tenant; when
that tenant's subscription lapses or the vendor discontinues the
product, the data requires explicit export — and export formats are
invariably lossy relative to the native platform representation.

A flat-file vault is the owner's property in the same sense that the
physical building is the owner's property: unconditionally,
transferrably, and without ongoing vendor permission.

## Provenance

Authored by totebox@project-bim, 2026-05-17. Sources: ISO 19650-1:2018;
ISO 16739-1:2024; BCF 3.0 buildingSMART specification; workspace
sub-agents A and C (2026-04-28); strategic source `BIM_Buildable
Architecture.md`; manifest.md §deployment cluster-totebox-property
vault layout.

Refinement target: `vendor/content-wiki-documentation/topic-asset-anchored-bim-vault.md`
plus Spanish adaptation at `topic-asset-anchored-bim-vault.es.md`.

## Research trail

### Done

- Validated ISO 19650 CDE technology-neutral scope and the Git-as-CDE
  mapping. Sub-agent C.
- Confirmed Merkle DAG / hash-addressed object store architecture from
  Speckle open-source implementation. Sub-agent A.
- Validated IFC-SPF shelf life and ISO governance chain. Sub-agent A +
  strategic source.
- Confirmed ITAR, EU Data Act, and HIPAA applicability of offline-first
  flat-file architecture. Sub-agent C.

### Suggested

- Confirm current ISO 19650 series revision status (parts 1, 2, 3, 5);
  the 2018 editions are cited here but part 3 has subsequent amendments.
- Verify bcf-3-0 ZIP structure against current buildingSMART bcf-XML
  schemas at publication time.

### Open questions

- Whether the Merkle DAG root-hash audit-trail satisfies evidence
  requirements under BCSC NI 51-102 disclosure obligations. The
  structural argument is sound; legal opinion is recommended before
  making any public claim about regulatory qualification.
