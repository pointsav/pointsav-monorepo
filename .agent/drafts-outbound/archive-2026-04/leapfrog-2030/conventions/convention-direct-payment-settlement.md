---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/direct-payment-settlement.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 53
research_done_count: 8
research_suggested_count: 5
open_questions_count: 3
research_provenance: master-web-research-brave-bat + stripe-connect + ocean-protocol
research_inline: true
---

# Direct-Payment Settlement

Payment for data marketplace transactions and ad exchange revenue is intended to
flow directly from buyer to tenant (the customer), not from buyer to platform
to tenant. Foundry's share is planned as a transaction fee at settlement time,
not a recurring subscription.

This convention codifies Doctrine claim #53 (ratified v0.1.0). It is the
operational form for the intended Phase 5 implementation of `service-settlement`.

## §1 — Why direct-payment matters for SMB

The hyperscaler marketplace pattern (Snowflake Marketplace, AWS Data Exchange)
intermediates payment: the buyer pays the platform; the platform deducts its
fee; the platform remits to the seller after a 30–60-day cycle. This pattern
creates three problems for SMB customers:

1. **Treasury complexity**: a 5-employee business has no treasury operations.
   Receiving 30–60-day payouts from a marketplace platform is operational
   overhead they cannot absorb.
2. **Float risk**: between buyer payment and seller payout, the platform holds
   the funds. Platform insolvency means seller loss. SMBs cannot absorb
   counterparty risk at platform scale.
3. **Audit complexity**: platform-intermediated payment requires the seller to
   reconcile platform statements against their own records. For a regional
   hospital with compliance requirements, direct payment simplifies the audit
   chain.

Direct-payment removes all three. The buyer pays the seller; Foundry takes its
fee at the moment of transaction; no funds flow through Foundry's accounts; no
payout cycles.

## §2 — Two settlement rails

`service-settlement` (Ring 2) is intended to support two rails simultaneously:

### Rail A — Traditional banking (Stripe Connect or equivalent)

Planned pattern:
1. Tenant (seller) connects their business bank account at provisioning
2. Buyer initiates transaction via service-marketplace or service-ad-exchange
3. Buyer's payment routes through service-settlement (Stripe Connect's
   "destination charge" pattern)
4. service-settlement calls Stripe API: charge buyer, deduct Foundry fee,
   transfer remainder to tenant's connected account
5. Stripe issues confirmation; service-settlement records an audit-ledger entry;
   tenant receives funds typically within 2–3 business days
6. Foundry receives its fee in its operating account

Rationale for Stripe Connect (or equivalent — Square Connect, Adyen for
Platforms, Mollie): it is the proven pattern for marketplace and platform models;
KYC, AML, and compliance are handled by Stripe; 1099-K reporting is handled by
Stripe for US tenants; transaction fees are at industry-standard rates (~2.9% +
$0.30 per transaction), which Foundry passes through with its own fee added
transparently.

### Rail B — Direct cryptocurrency

Planned pattern:
1. Tenant connects a crypto wallet at provisioning (operator-chosen network)
2. Buyer initiates transaction; buyer chooses the crypto rail
3. service-settlement constructs the transaction: buyer-to-tenant transfer, plus
   a fee output to Foundry's operating wallet
4. Buyer signs and broadcasts; the transaction confirms on-chain
5. service-settlement records the on-chain receipt plus a Sigstore Rekor anchored
   audit-ledger entry [sigstore-rekor-v2]
6. Tenant holds funds the moment the chain confirms (typically minutes)

Rationale for crypto rail: it aligns with the sovereign substrate posture (no
banking intermediary required); settlement is faster (minutes vs days); fees are
lower for cross-border buyers; cryptographic provenance composes with the audit
ledger; and it is opt-in — tenants who do not want crypto do not see it.

The Brave BAT model is the validating precedent at consumer scale: approximately
60 million monthly active users earn rewards directly from advertisers, with
Brave taking 30% as a platform fee. Direct-payment-to-rights-holder is
operationally viable at scale; Foundry's planned claim is the same pattern
applied at SMB-business scale.

## §3 — Foundry's transaction fee model

Foundry's intended fee is a percentage of transaction value, deducted at
settlement. The fee is planned to be:

- Transparent (visible to both buyer and tenant before transaction)
- Consistent (per-transaction percentage, not per-tenant negotiation)
- Configurable per-vertical (e.g., 5% for marketplace listings, 3% for
  ad-exchange impressions — pending operator decision; see OQ #1)
- Non-recurring (no subscription, no minimum, no annual fee)

Tenant onboarding is not planned to require any upfront payment to Foundry.
Foundry is intended to earn only when the tenant earns. This composes with
claim #48 (Customer-Owned Graph IP): the customer's data is their IP; Foundry
is planned to take a service fee on monetization, not a license fee on access.

## §4 — Settlement event audit shape

Every intended settlement records:

```json
{
  "schema": "foundry-settlement-event-v1",
  "settlement_id": "ULID",
  "transaction_id": "<from service-marketplace or service-ad-exchange>",
  "tenant": "module_id",
  "rail": "stripe-connect" | "crypto-bitcoin" | "crypto-ethereum" | "...",
  "buyer_identity": "<rail-specific identifier>",
  "gross_amount": 100.00,
  "currency": "USD" | "BTC" | "ETH" | "...",
  "stripe_fee": 3.20,
  "foundry_fee": 5.00,
  "tenant_amount": 91.80,
  "settled_at": "2026-04-30T...",
  "confirmation": "<stripe-charge-id or on-chain-tx-hash>",
  "rekor_anchor": "rekor:..."
}
```

The audit ledger is anchored monthly to Sigstore Rekor [sigstore-rekor-v2]
(Doctrine Invention #7). Settlement events are part of the anchored corpus;
any past settlement may be verified against the cryptographic record.

## §5 — Composition with claims #43, #48, #52

- Claim #43 (Single-Boundary): settlement traffic is intended to route through
  the Doorman; the same audit boundary that captures inference is planned to
  also capture monetization
- Claim #48 (Customer-Owned Graph IP): direct-payment is the economic
  realization of customer-owned IP; payment goes to the IP holder
- Claim #52 (Reverse-Flow Substrate): every marketplace and ad-exchange
  transaction is intended to trigger a settlement event in this convention's flow

## §6 — When direct-payment is not the answer

Two narrow exceptions:

1. **Foundry-managed Tier B or Tier C compute**: when a tenant uses
   PointSav-arranged Yo-Yo capacity or PointSav-paid Tier C API access, Foundry
   is the seller (compute is Foundry's product). Settlement is Foundry-as-seller,
   not Foundry-as-platform. The billing surface for compute is separate from the
   marketplace settlement surface.

2. **Customer-to-customer settlement within Foundry tenants**: if two Foundry
   tenants transact, the default is direct-payment via service-settlement. An
   operator-override may route through Foundry as escrow agent for high-value
   transactions. This is opt-in per transaction.

## §7 — Tax and regulatory composition

- **US tax**: Stripe Connect handles 1099-K issuance for tenants who cross the
  threshold. Crypto rail tenants are responsible for their own tax reporting;
  service-settlement is intended to provide annual transaction reports.
- **EU VAT**: service-settlement is intended to support VAT-compliant invoicing
  for EU tenants and buyers (deferred; B2B reverse-charge pattern).
- **Cross-border**: Stripe Connect handles currency conversion; the crypto rail
  is currency-agnostic at the protocol layer.
- **Anti-money-laundering**: Stripe Connect's KYC handles AML for the fiat
  rail. Crypto rail tenants self-attest at provisioning; high-value transactions
  are intended to trigger additional verification above a configurable threshold.

These are operational forms; the convention's structural commitment is direct
payment from buyer to tenant. Compliance details adapt per jurisdiction.

## §8 — Intended rollout sequence

Per Phase 5 of the leapfrog roadmap:

1. **Weeks 1–2**: Stripe Connect integration scaffold; service-settlement crate
   scaffolded in pointsav-monorepo
2. **Week 3**: First end-to-end test (Foundry-as-tenant, Foundry-as-buyer on
   test-mode Stripe)
3. **Week 4**: Marketplace transaction → settlement integration
4. **Weeks 5–6**: Ad-exchange transaction → settlement integration
5. **Weeks 7–8**: Crypto rail (one network first; network pending operator
   decision; see OQ #2)
6. **Week 9+**: First customer (Woodfine) marketplace and settlement live
7. **Phase 6 (Year 2)**: Federated marketplace launch (claim #14 Federated LoRA
   Marketplace; settlement composes with adapter sales)

## Provenance

Research reviewed: Brave BAT direct-payment-to-user model (~60M MAU; 70/30
split; 2026 validating precedent); Stripe Connect destination-charge pattern
(marketplace standard); Square Connect equivalent capabilities; Adyen for
Platforms (European alternative); Snowflake Marketplace settlement model
(intermediated; what Foundry diverges from); AWS Data Exchange settlement model
(similar intermediated pattern); IAB OpenRTB win-notification → settlement flow
(ad-exchange standard); HIPAA-compliant settlement audit chain requirements;
Sigstore Rekor anchoring composition (Doctrine Invention #7) [sigstore-rekor-v2].

Suggested next research: (1) crypto rail implementation — which network first?
(operator decision gating Phase 5 week 7); (2) Stripe Connect pricing
negotiation (Foundry-side; volume-based fees); (3) per-vertical recommended
Foundry fee percentages (operator decision); (4) EU VAT compliance reference
implementation; (5) tax-reporting UX in slm-cli (`/settlement reports` command).

**OQ #1 — Foundry fee percentage.** What share does Foundry retain on
marketplace transactions vs ad-exchange transactions? The Brave 30% precedent
suggests an upper bound; SMB market sensitivity may favor a lower fee.
Operator decision gating ratification.

**OQ #2 — Crypto rail network choice.** Bitcoin (sovereignty and ubiquity),
Ethereum (smart-contract programmability and USDC stablecoin), Solana (low
fees and speed), or multi-chain from launch? Operator decision pending.

**OQ #3 — Foundry-as-tenant on its own marketplace.** Does Foundry list its
own data assets (e.g., aggregated workspace operational patterns, anonymized)
on the marketplace? Sets a transparency norm but introduces a conflict-of-interest
consideration. Pending operator decision.

## References

- `DOCTRINE.md` claim #53
- Companion: `conventions/reverse-flow-substrate.md` (claim #52)
- Companion: `conventions/customer-owned-graph-ip.md` (claim #48)
- Companion: `conventions/single-boundary-compute-discipline.md` (claim #43)
- [sigstore-rekor-v2]: https://docs.sigstore.dev/logging/overview/
- External: Stripe Connect documentation
- External: IAB OpenRTB 2.6 specification (win-notification flow)
- Cross-industry: Brave BAT 70/30 model
