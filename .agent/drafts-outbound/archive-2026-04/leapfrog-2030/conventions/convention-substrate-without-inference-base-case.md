---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/substrate-without-inference-base-case.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 54
research_done_count: 6
research_suggested_count: 3
open_questions_count: 1
research_provenance: master-direct-from-three-ring-architecture-rule + operator-2026-04-30T05-58Z-addition
research_inline: true
---

# Substrate-Without-Inference Base Case

The Totebox Archive must remain operationally functional and freely transferable
even when service-slm cannot run any inference. AI inference is value-add, not
load-bearing.

This convention codifies Doctrine claim #54 (ratified v0.1.0). It is the
operational form for the deterministic-only mode of every customer-deployed
Totebox.

## §1 — The base case

The base case is "AI tier completely disabled." All of the following must hold:

- service-fs (WORM ledger): operational; ingest, query, and checkpoint all work
- service-content (knowledge runtime): operational; graph queries, vector search,
  and temporal queries work; mutations from non-AI paths only
- service-input: operational; ingests files, email, voice (deterministic parsing
  only), and forms
- service-extraction: operational at the deterministic-extraction layer (regex,
  mailparse, structured-format parsing); AI-enriched extraction paths return
  "AI-tier unavailable" gracefully
- service-egress: operational; outbound formatting and delivery
- service-people, service-email, service-fs: operational
- service-slm Doorman: bound and listening; returns 503 to inference endpoints
  when no tier is available; deterministic endpoints (`/healthz`, `/readyz`,
  `/v1/contract`) remain operational
- slm-cli TUI: operational in deterministic-only mode (see §3)
- service-marketplace, service-ad-exchange, service-settlement (when enabled per
  claims #52, #53): operational; transactions may proceed without AI grounding
  (audit and consent records still enforced)

## §2 — What "no inference tier" means

The trigger conditions for base-case mode:

- Tier A llama-server not running (service stopped, model file missing, or
  hardware insufficient)
- Tier B Yo-Yo unreachable (network, instance terminated, or customer-side GPU
  box offline)
- Tier C disabled at tenant config (no API keys; sovereignty mode)
- Doorman starting up and not yet ready

Or any combination of the above leading to all three tiers being simultaneously
unavailable.

The base case is the union: *zero* inference tiers available. If even one tier
is up, normal AI-enabled operations resume; the substrate falls to deterministic
mode only when all tiers fail simultaneously.

## §3 — slm-cli TUI deterministic-only mode

When the TUI detects no inference tier available (via Doorman `/readyz`
returning all tier flags false), it enters deterministic-only mode.

**UI changes**:
- Status bar shows "AI-disabled — deterministic operations only"
- Chat input shows: "AI tier unavailable; use /commands"
- Pressing Enter on a non-slash-command input returns: "Cannot answer
  natural-language queries without AI tier. Run `/help` for available
  deterministic commands."

**Slash commands that remain operational**:
- `/status` — query service-fs and service-content health
- `/audit [tenant] [date]` — query audit ledger
- `/graph [entity]` — query knowledge graph (deterministic Cypher)
- `/search [query]` — keyword search across content (no AI re-ranking)
- `/export [format]` — export tenant graph, audit ledger, and content
- `/transfer prepare` — prepare an ownership-transfer bundle
  (audit-signed export package)
- `/help` — list deterministic commands

**Slash commands that are disabled** (require AI tier):
- `/feedback` — verdict capture; gracefully no-ops
- `/brief` — apprenticeship brief submission; gracefully no-ops
- `/adapters` — adapter list; shows "no adapters loaded"

## §4 — The transfer-of-ownership flow

The "freely transferable" property requires a structural transfer flow.
`slm-cli /transfer prepare` is intended to produce a self-contained bundle:

1. Per-tenant graph snapshot (Cypher dump and vector index dump)
2. Per-tenant audit ledger (last N months; full history available via `--full`)
3. Per-tenant adapter weights (LoRA files, signed)
4. Seed taxonomy (current state)
5. Pack manifest (which Vertical Seed Pack was installed; per claim #50)
6. Tenant configuration (settlement-rail config, consent records, marketplace
   listings)
7. Cryptographic signature over the bundle (operator's identity key)
8. Sigstore Rekor anchor receipt (proof of bundle integrity)
   [sigstore-rekor-v2]

The bundle is the customer's complete sovereign artefact. They may:
- Move it to a new Totebox at a new operator
- Hand it to an acquiring party in a business sale
- Transfer it at estate-planning level (in jurisdictions that recognize digital
  asset inheritance)
- Operate from it indefinitely without Foundry's involvement

The receiving party imports the bundle into a fresh Totebox; the substrate boots
with the imported state; deterministic operations work immediately. AI tier
becomes available when the new operator configures it.

## §5 — Why this matters commercially

The "freely transferable" property distinguishes a sovereign asset from a
service. Examples:

- **A restaurant sells the business**: the new owner imports the Totebox bundle.
  Customer history, vendor relationships, recipe knowledge graph, and audit
  ledger are all present immediately. The new owner does not re-subscribe to
  SaaS, does not migrate data, and does not engage migration consultants.
- **A law firm dissolves**: the partners split the matter graph by practice
  area. Each partner receives a Totebox bundle with their share. No vendor
  coordination, no data export project.
- **A regional hospital is acquired**: the acquiring system imports the Totebox
  bundle. Patient history, audit ledger, and anonymized clinical patterns
  transfer in one cryptographically-signed artefact.
- **Foundry ceases operations**: the customer continues operating their Totebox
  indefinitely. The deterministic substrate works without Foundry. They lose
  the ability to receive new vertical packs and to sell on Foundry's
  marketplace, but their existing operations do not pause.

Hyperscaler vendors (Salesforce, Epic, iManage) are structurally unable to
match this property. Foundry's claim is that operating without the vendor is a
first-class supported mode.

## §6 — Implementation requirements (per service)

The base case constrains every service implementation:

- **service-fs**: deterministic; no AI dependency; already meets the bar
- **service-content**: must support deterministic Cypher queries, vector search,
  and temporal queries without LLM augmentation. The graph is structured data;
  queries are deterministic by default.
- **service-extraction**: must produce a deterministic-extraction baseline
  (regex, mailparse, structured parsing) that operates without AI. AI-enriched
  extraction is a separate code path that gracefully no-ops when tiers are
  unavailable.
- **service-input**: deterministic format parsers; no AI requirement for routine
  ingest
- **service-egress**: deterministic format generators (PDF, email, web);
  AI-enhanced summaries are optional
- **service-slm Doorman**: returns 503 from `/v1/chat/completions` when no tier
  is available; `/healthz`, `/readyz`, and `/v1/contract` always respond
- **slm-cli**: deterministic-only mode (this convention §3)
- **service-marketplace, service-ad-exchange, service-settlement**: audit and
  consent enforcement is deterministic; AI-grounded listing recommendations
  are optional value-add

The structural rule: every service has a deterministic baseline that operates
without AI. AI-enhanced operations are documented as "requires AI tier —
gracefully degrades."

## §7 — Composition with other claims

- Claim #16 (Three-Ring Architecture): Ring 3 (AI) is structurally optional —
  this convention is the operational form of that optionality
- Claim #34 (Two-Bottoms Sovereign Substrate): the substrate boots on customer
  hardware; this convention extends "boots without Foundry" to "operates without
  inference"
- Claim #48 (Customer-Owned Graph IP): IP only matters if the customer can use
  it independently; deterministic-only mode is the use-it mechanism
- Claim #49 (Tier 0 Customer-Side Sovereign Specialist): Tier 0 with AI is the
  value-added mode; Tier 0 without AI is the freely transferable mode

## §8 — Verification protocol

Foundry's intended CI/CD pipeline includes a "no-tier" test suite:

1. Spin up a test deployment with all tiers disabled
2. Run the full deterministic-operations test suite (audit query, graph query,
   vector search, export bundle generation, transfer import)
3. Confirm all deterministic flows pass
4. Confirm AI flows return graceful "AI tier unavailable" errors

This test suite is intended to run on every workspace PATCH. Regression in the
deterministic baseline is a doctrine-level signal — Foundry has broken its core
commitment.

## Provenance

Research reviewed: Three-Ring Architecture v0.0.4 (Ring 3 structurally
optional); Two-Bottoms Sovereign Substrate v0.0.10 (sovereignty without vendor
dependency); hyperscaler vendor lock-in patterns (Salesforce, Epic, iManage —
what Foundry diverges from); Sigstore Rekor anchored receipts (cryptographic-
signed transfer bundle integrity) [sigstore-rekor-v2]; estate-planning for
digital assets — legal precedents (US, EU, Canada); operator chat-surface
addition 2026-04-30T05:58Z framing.

Suggested next research: (1) transfer-bundle format specification (separate
document; project-data Task scope during service-content rebuild); (2) CI/CD
no-tier test suite implementation (project-slm Task scope); (3) operator UX for
`/transfer prepare` flow (slm-cli implementation; project-slm Task scope).

**OQ #1 — Vertical Seed Pack updates without AI.** When the customer operates
without inference, do they still receive marketplace updates (new Vertical Seed
Packs)? Pack distribution itself is a deterministic flow; AI is needed only to
recommend packs. Operational decision: pack fetch endpoint should be always
deterministic.

## References

- `DOCTRINE.md` claim #54
- `DOCTRINE.md` claim #16 (Three-Ring Architecture; Ring 3 optional)
- `DOCTRINE.md` claim #34 (Two-Bottoms Sovereign Substrate)
- `DOCTRINE.md` claim #48 (Customer-Owned Graph IP)
- `DOCTRINE.md` claim #49 (Tier 0 Customer-Side Sovereign Specialist)
- [sigstore-rekor-v2]: https://docs.sigstore.dev/logging/overview/
- Companion: `conventions/three-ring-architecture.md`
- Companion: `conventions/customer-owned-graph-ip.md`
