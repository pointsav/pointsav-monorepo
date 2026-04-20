# VERIFICATION.md
# Distributed Verification Layer
**Version:** 1 · April 19, 2026
**Status:** Optional feature — switched on separately. Archive runs without it.
**Commercial value:** Higher verification = higher confidence scores = premium marketplace pricing.

---

## Overview

The verification layer is an optional paid feature, not a mandatory architectural component.
An archive runs fully without verification — WORM compliance, DARP compliance, search, and
all derivative layer computation work independently of verification status.

Enabling verification unlocks premium tiers in the Data Marketplace and Ad Exchange. The commercial
logic is direct: independently verified institutional data commands a price premium that unverified
data cannot.

No single source of truth. Duplicates self-heal over time. Verification accelerates disambiguation
and stamps nodes with human-confirmed confidence scores.

---

## What Gets Verified

| Node type | What the verifier confirms | Commercial priority |
|---|---|---|
| Entity (person) | Identity, current role, organization, LinkedIn URL | High — premium for verified person data |
| Entity (organization) | Legal name, registration, jurisdiction, legal form | High |
| Relationship | Are two entities correctly linked in this context? | Medium |
| CoA classification | Is this document correctly categorized? | Medium |
| Content accuracy | Does /assets accurately represent /source? | Low initially |
| Archetype assignment | Does this community match the assigned standard class? | Medium |

---

## How It Works — Step by Step

### Step 1: Questionnaire Generation (Automated)

During each GCP batch job, Gemma 4 generates questionnaires from graph nodes that have
`confidence < threshold` or `verified = false`.

**Question types:**

```
CONFIRM:    "Does Peter Woodfine currently serve as Managing Director at
             Woodfine Management Corp.? [Yes / No / Unknown]"

CORRECT:    "What is the correct job title for this person? [free text]"

CLASSIFY:   "Which CoA macro-domain best describes this document?
             [pick from list: Entity & Governance / Risk & Controls / Financial / ...]"

VERIFY_URL: "Paste a URL confirming this claim: [URL input]"

DISPUTE:    "Is any information in this node incorrect? [Yes / No + free text]"
```

Context shown to verifier is **anonymised** — PII stripped, coordinates removed, full archive
context not shared. Verifier sees enough to verify, not the full institutional record.

### Step 2: Verifier Access via os-console

Verifiers log in via os-console from any location (laptop, mobile, tablet).
Interface presents one questionnaire at a time.

### Step 3: External Cross-Reference (Air-Gapped Protocol)

Verifier opens their own personal browser to cross-reference. The system never touches LinkedIn,
Google, government directories, or any external service during verification.

**LinkedIn URL workflow (the standard verification path for person entities):**

1. System displays: "Entity: Victoria Johnson · Extracted company: Woodfine Management Corp."
2. Verifier opens LinkedIn in their personal browser (their own session, their own network)
3. Searches for the person
4. Finds correct profile
5. Copies LinkedIn profile URL
6. Pastes URL into os-console response field
7. Submits
8. Graph node updates: `verified = true · confidence = 0.85 · source_url = [URL]`

**Air-gap principle preserved:** The verifier's browser session is entirely separate from the
system. The machine never initiates any external network connection during verification.

### Step 4: Confidence Scoring and Consensus

```
Single confirmation:    confidence = 0.65  (partially verified)
Two agreeing:          confidence = 0.85  (verified)
Two disagreeing:       status = "disputed" → queued for senior review
Three+ with 2/3 agree: confidence = 0.90+ (high confidence)
Any + URL evidence:    +0.05 additional confidence bonus
```

Consensus rule: 2/3 majority agreement promotes a node from "partially verified" to "verified".
A single contradiction flags as "disputed" and pauses its use in derivative computation until
senior review resolves it.

### Step 5: Graph Node Updated

- `verified` property updated on the LadybugDB node
- `confidence` score updated
- `source_url` recorded for auditing
- Verification event logged in `service-verification/ledger/`
- YAML snapshot generated to capture the update in SOC3 audit trail

Nodes are never auto-merged. Disputed nodes remain as separate nodes with `status = "disputed"`.
No single source of truth — verification increases confidence but doesn't assert absolute truth.

---

## Workforce and Payment Model

### Payment Infrastructure

**Primary:** Polygon USDC on Polygon PoS — ~$0.002 per transaction (0.2% of a $1 task payout).
Sub-5-second finality globally 24/7. Stablecoin — no FX volatility.

**Backup:** Lightning Network (LND MIT / Core Lightning BSD) with LNbits (self-hosted Python
accounts layer issuing per-user sub-wallets). Censorship-resistant.

**Why not Stripe/PayPal:** Stripe Connect Express ($0.25 + 0.25% per payout + $2/account/month)
and PayPal Payouts (~$0.25 domestic minimum) consume 25–50% of payout value for sub-$2 tasks.
Structurally non-viable.

**GNU Taler:** Reached v1.0 May 2025 with Swiss-franc pilot. Geographically limited. Not yet.

### Regulatory Requirements

FinCEN MSB registration required regardless of payment amount (CTR at $10K, SAR at $2K).
FINTRAC mirrors this at CAD $10K LVCTR and STR with no threshold. DKA must register as an MSB
and maintain KYC proportional to payment volume before enabling paid verification workforce.

### Workforce Paths

**Internal staff:** Assigned queue in os-console. Tracked internally. No payment processing.

**Distributed external workforce:** PointSav provisions os-console access to external verifiers,
manages payment pool. Verifiers earn per accepted questionnaire. Consensus acceptance triggers
payment — not submission.

**Client self-service:** Archive owner pays PointSav for verification throughput. PointSav manages
the verifier pool. Archive owner receives verified graph nodes as the deliverable.

---

## Questionnaire Throttle

**Single-operator mode:** 10 verifications per day. Intentional design — makes verification a
high-value ritual rather than a checkbox exercise. 10 perfect records per day = 3,650 verified
relationships per year with zero data corruption.

**Distributed workforce mode:** Total daily throughput scales with workforce size. Per-verifier
throttle remains in place. System throttle is the sum of all per-verifier limits.

**Phase 1 trial:** All entities start `verified = false`. Verification pass runs after graph build
as a separate operation (not during Phase 1). Questionnaires are queued but not distributed during
Phase 1 — the feature is prepared but not switched on.

---

## SOC3 Audit Trail

Every verification transaction logged to `service-verification/ledger/verification.csv`:

```csv
questionnaire_id, verifier_id, timestamp_utc, question_type, response,
url_evidence, confidence_before, confidence_after, node_id, node_type
```

- Append-only CSV — never modified after write
- Included in YAML snapshot exported after each batch
- SOC3 Processing Integrity: every confidence change is fully traceable
- Independent audit: any SOC3 auditor can re-derive confidence scores from the ledger
