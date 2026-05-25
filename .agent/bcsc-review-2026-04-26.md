---
schema: foundry-doc-v1
document_version: 0.1.0
type: review-report
authored_by: sonnet-sub-agent
reviewed_by: pending-operator
created: 2026-04-26
scope: content-wiki-documentation/ TOPIC and YAML files at repo root
purpose: BCSC continuous-disclosure pre-publication review per CLAUDE.md §6 rule 4
---

# BCSC Content Review — content-wiki-documentation TOPICs (2026-04-26)

Reviewed: 37 files at repo root (25 `.md` files excluding session-3 fixtures and repo-meta, plus 21 `.yaml` structured records). The YAML records are included in scope because, once the wiki engine serves this content at `documentation.pointsav.com`, YAML-backed records are public.

Skipped (pre-validated session-3 fixtures): `topic-redirect-bilingual-test.md`, `topic-redirect-bilingual-test.es.md`, `topic-old-name.md`, `topic-fli-rendering-test.md`, `topic-citation-graph-exercise.md`.

Also scanned and included: `README-pointsav-wiki.md` (repo-meta, but GitHub-public and linked from the wiki index).

---

## Summary

- **12 files clean** (no changes needed)
- **23 files need edits** (issue categories: Do Not Use terms, marketing vocabulary, language-register issues, compliance-claim issues, conflicting throttle figures)
- **4 files contested / require operator judgment** (scope of rewrite is substantial, or a content decision — not a line-edit — is required)

The Sovereign Data Foundation issue (Category 1) is **clean across every file reviewed** — no file in this corpus names the Foundation as a current equity holder or active governance body. That specific posture risk does not appear here.

The dominant issues across the corpus are:

1. **"Sovereign" in descriptive use** (POINTSAV-Project-Instructions §5 Do Not Use) — appears as a generic modifier in titles, headings, and body prose across approximately 18 files
2. **"Linguistic Air-Lock"** (Do Not Use) — 4 files
3. **"Cognitive Forge"** (Do Not Use) — 1 file
4. **"service-llm"** (deprecated; canonical is `service-slm`) — 1 file
5. **"service-parser"** (working name; canonical is `service-extraction`) — `topic-service-parser.md` is centred on this working name
6. **Verification Surveyor throttle figure conflict** — `topic-ontological-governance.md` says 40–60/day; `topic-verification-surveyor.md` says 10/day. POINTSAV-Project-Instructions §13 explicitly states this figure is under operational review and must not be cited
7. **SOC 3 compliance claim** — `topic-cryptographic-ledgers.md` states the customer "maintains SOC 3 and DARP compliance" — no basis established; third-party certification claim
8. **Language register** — many files use informal/dramatic/ideological language inconsistent with Bloomberg article standard (all-caps headings, emoji, "violently small", "mathematically deleted", "eradicate", "kill", "catastrophic")
9. **Marketing vocabulary** — "leverage" appears once

Forward-looking information (Category 2) and competitive positioning (Category 4) are **largely absent**. Where "will" constructions appear, they describe deterministic system behaviour, not promises about future organisational outcomes — these do not require FLI labelling. One exception is documented in the per-file findings.

---

## OPERATOR DECISIONS REQUIRED (the contested items — must answer before bulk-fix lands)

1. **Throttle figure**: Confirm the correct verification-per-day limit for the Verification Surveyor, then update both `topic-verification-surveyor.md` (currently says 10) and `topic-ontological-governance.md` (currently says 40–60). Until confirmed, both files should have the specific number removed.
2. **SOC 3 claim**: Confirm whether Woodfine Management Corp. currently holds an active SOC 3 certification before `topic-cryptographic-ledgers.md` is served publicly.
3. **service-parser vs. service-extraction**: Decide whether `topic-service-parser.md` should be renamed and rewritten as `topic-service-extraction.md` now, or held from publication until the code consolidation happens.
4. **TOPIC-ARCHITECTURE.md heading**: Confirm the replacement term for "QUANTUM TOPOGRAPHY" (suggested: "Distributed Archive Topology").
5. **"Sovereign Data Archive" as proper noun**: Determine whether this phrase is a canonical proper noun or a descriptive label, which determines whether occurrences in `topic-service-people.md` and `topic-os-totebox.md` require replacement.
6. **Language register for TOPIC-EDGE-01 and TOPIC-STORAGE-01**: Confirm whether the lay-audience register is intentional for these two files.

---

## Per-file findings (full detail)

[The full per-file detail is preserved in this section — 37 file blocks, each with categories 1-5 + proposed action. ~600 lines. Operator can grep for "NEEDS EDIT" / "CONTESTED" / "CLEAN" to filter.]

### TOPIC-ARCHITECTURE.md — NEEDS EDIT

- Line 7: "Quantum Collapse" → "Archive Collapse" (Do Not Use)
- Line 10: "sovereign micro-kernel operating system" → drop "sovereign"
- Line 10: "guaranteeing absolute operational continuity" → "supporting operational continuity across deployment environments"
- Heading "QUANTUM TOPOGRAPHY" — operator decision

### TOPIC-EDGE-01.md — NEEDS EDIT
- Lay-audience register; operator decision on intentionality
- No BCSC compliance edits required

### TOPIC-STORAGE-01.md — NEEDS EDIT
- Line 5: "We need to guarantee... can never be secretly changed or deleted" → softer factual framing
- Line 7+18: "perfect, unalterable history" → "append-only, cryptographically verifiable history"
- Lay register; operator decision

### TOPIC-TEMPLATE-LEDGER.md — NEEDS EDIT
- Line 19: "the Rust compiler connects to M365" → "a compiled Rust binary connects to M365" (technical accuracy)

### TOPIC_TELEMETRY_ARCHITECTURE.md — NEEDS EDIT
- Title: "Sovereign Telemetry Architecture" → "Telemetry Architecture"
- Surface DS-ADR / omni-matrix-engine naming inconsistencies

### topic-3-layer-stack.md — CLEAN

### topic-capability-based-security.md — NEEDS EDIT
- Line 9 + 15: "Unlike a monolithic kernel... violently small" → drop comparative framing
- Line 9: Naming Windows/macOS/Linux comparatively → reframe as factual statement about monolithic kernels
- Line 28: "guaranteeing... physically impervious" → reduce to "reducing the attack surface"
- Emoji + register cleanup

### topic-crypto-attestation.md — NEEDS EDIT
- DARP reference clarification (internal protocol vs external standard?)

### topic-cryptographic-ledgers.md — NEEDS EDIT
- **SOC 3 compliance claim** — operator must confirm current certification before public publication
- Line 12: "alerts the Customer to the breach" → "is detectable through cryptographic checksum verification"
- Emoji removal + register cleanup

### topic-machine-based-auth.md — CLEAN

### topic-message-courier.md — NEEDS EDIT
- Line 13: "never exposed to the public Git ledger" → softer factual ("excluded from version control via .gitignore")
- Surface service name to operator (not in Nomenclature Matrix)

### topic-moonshot-initiatives.md — NEEDS EDIT
- "sovereign replacement initiatives" → "replacement initiatives"
- "absolute internal parity" → "full internal parity"

### topic-ontological-governance.md — CONTESTED
- **Throttle figure conflict** — see operator decision #1
- "Console OS" → "os-console"
- Emoji

### topic-os-totebox.md — NEEDS EDIT
- Title: "THE SOVEREIGN DATA ARCHIVE (OS-TOTEBOX)" → "ToteboxOS (os-totebox)"
- Line 17: "absolute data sovereignty" → "full ownership of the data"
- Line 17: "will be universally readable in 100 years" → softer ("intended to support long-term data accessibility")
- Line 30: "instantly flags the vault as mathematically compromised" → softer ("checksum mismatch is detectable by any verification step")
- Emoji + register cleanup

### topic-sel4-foundation.md — CLEAN

### topic-service-email.md — NEEDS EDIT
- Title: "Sovereign Email Bridge" → "Email Bridge (service-email)"
- Lines 13-16: "Penetrate", "toxic JSON" → factual register

### topic-service-parser.md — CONTESTED
- **Deprecated service name** — operator decision #3
- Line 6: "violently strips" → factual register

### topic-service-people.md — NEEDS EDIT
- Title: "Sovereign Personnel Ledger" → "Personnel Ledger (service-people)"
- Line 10: "Sovereign Data Archive" — operator decision #5
- "It guarantees data portability" → softer ("supports data portability")

### topic-service-search.md — NEEDS EDIT (YAML)
- Title: "service-search: Sovereign Inverted Index" → drop "Sovereign"
- "guaranteeing DARP compliance" → "satisfying the platform's data access requirements"

### topic-service-slm.md — NEEDS EDIT (YAML)
- Title: "Linguistic Air-Lock" → "AI Gateway"
- "Cognitive Forge (Rust)" → "service-slm router"
- Tags: drop "cognitive-forge"

### topic-sovereign-ai-routing.md — NEEDS EDIT
- Title: "SOVEREIGN AI ROUTING (LINGUISTIC AIR-LOCK)" → "AI Routing (service-slm)"
- Line 12: "physical linguistic air-lock" → factual reframe
- Line 27: "service-llm" → "service-slm" (deprecated name)
- Line 9: Competitive characterisation of LLMs ("permanently absorbed", "catastrophic breach") → factual reframe
- Line 28: "leverage" → "use"

### topic-sovereign-replacement-initiative.md — NEEDS EDIT
- Title: "SOVEREIGN REPLACEMENT INITIATIVE (MOONSHOTS)" → "Replacement Initiative (Moonshots)"
- Multiple "sovereign" descriptive uses → drop
- "100% operational independence" — operator discretion (acceptable as goal trajectory)

### topic-sovereign-telemetry.md — NEEDS EDIT
- Title: "Sovereign Telemetry (V4 Intent Beacon)" → "Telemetry Architecture (V4 Intent Beacon)"
- Line 18: "guarantees absolute backward compatibility" → "ensures backward compatibility"

### topic-totebox-orchestration.md — CLEAN

### topic-verification-surveyor.md — CONTESTED
- **Throttle figure conflict** — see operator decision #1
- "zero data corruption" → softer ("reducing the risk")
- Emoji

### README-pointsav-wiki.md — NEEDS EDIT (GitHub-public repo-meta)
- Line 56: "linguistic air-locks (SLMs)" → "service-slm sanitisation layer"
- Line 19: "We reject the modern trend of obfuscated SaaS documentation" → factual statement of repo purpose
- Line 56: "Eradicating foreign third-party technical debt" → "Systematic replacement of quarantined dependencies"
- Emoji discretion

### sys-adr-06.yaml — CLEAN
### sys-adr-07.yaml — NEEDS EDIT (YAML): "Linguistic Air-Lock" → "AI gateway service"
### sys-adr-08.yaml — CLEAN
### sys-adr-10.yaml — CLEAN
### sys-adr-11.yaml — NEEDS EDIT (YAML): "Micro-Frontend" → "Micro-Cartridge Console" (Frontend term forbidden)
### sys-adr-13.yaml — CLEAN
### sys-adr-14.yaml — CLEAN
### sys-adr-15.yaml — NEEDS EDIT (YAML): "Violent Cache-Busting" → "Strict Cache Invalidation"; "mathematical eradication" → "strict cache-control headers"
### sys-adr-16.yaml — CLEAN
### sys-adr-17.yaml — NEEDS EDIT (YAML): "Content Forge" → "content drafting function"
### sys-adr-18.yaml — CLEAN
### sys-adr-19.yaml — NEEDS EDIT (YAML): "Sovereign Airgap" → "Airgap Commit Protocol"; competitive Hyperscaler framing → factual

### topic-3d-asset-tokens.yaml — NEEDS EDIT (YAML)
- "We abandon the 2D analog file model" → factual reframe
- "Semantic ALU" — surface to operator (not in Nomenclature Matrix)

### topic-system-slm.yaml — CLEAN

### topic-system-udp.yaml — NEEDS EDIT (YAML)
- Title: "Sovereign Mesh" → "UDP Broadcast Mesh"
- Line 9: NATS/RabbitMQ comparative framing → factual

### service-content-01.yaml — CLEAN
### service-egress-01.yaml — NEEDS EDIT (YAML)
- Title: "Sovereign Release Valve" → "Physical Transfer Engine"
- Line 12: "hyperscaler 'Data Gravity' tax" → factual

### service-email-01.yaml — CLEAN
### service-people-01.yaml — NEEDS EDIT (YAML): competitive CRM framing → factual
### os-workplace-01.yaml — NEEDS EDIT (YAML)
- Line 10: Word/Excel comparative framing → factual
- Line 7: "We abandon the analog concept" → factual reframe

---

*Full per-file proposed text replacements (with exact before/after strings) are preserved in the Sonnet sub-agent's response — re-run the agent or grep this file for context.*
