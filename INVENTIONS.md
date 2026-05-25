
# INVENTIONS.md — pointsav-monorepo

PointSav Digital Systems' contributions to the open AI-native substrate
landscape, organized as ratified Foundry Doctrine claims. Each claim below is a
structural pattern that is novel relative to the 2026 hyperscaler stack.

This document is an addition to the engineering monorepo, complementary to
`~/Foundry/DOCTRINE.md` (the constitutional charter) and the `conventions/`
library (operational forms). Read after CLAUDE.md and DOCTRINE.md to understand
what makes Foundry structurally different from the alternatives.

---

## The leapfrog inventions (Doctrine claims #43–#54)

### #43 — Single-Boundary Compute Discipline

Exclusive-path inference gateway with structural enforcement (firewall,
UID-owner iptables, bearer-only-in-Doorman). The audit guarantee is
structural rather than procedural: bypass is prevented at the kernel boundary,
not by policy alone. Existing LLM gateway products implement preferred-path
routing, not exclusive-path routing.

### #44 — Knowledge-Graph-Grounded Apprenticeship

Graph and adapter co-evolve. Training tuples carry graph-context; verdict
signing rewards graph-coherent generation; accepted output mutates the graph.
Distinguishes Foundry from stateless GraphRAG patterns where the graph is
static input and the model's output does not feed back into it.

### #45 — TUI-as-Corpus-Producer

Sysadmin and IT-support TUI interactions are first-class training corpus.
Narrow domain, verifiable ground truth, and domain-expert feedback combine to
produce an order-of-magnitude improvement in data efficiency. An IT-support
adapter is expected to reach production quality with 200–500 verdict-signed
interactions, not the tens-of-thousands required for code-generation tasks.

### #46 — MCP-as-Substrate-Protocol

Model Context Protocol is the substrate-level wire contract for service
composition. Every Ring 1 and Ring 2 service is an MCP server; the Doorman is
the MCP gateway; customer extensions plug in as additional servers without
modifying the core. Distinguishes Foundry from bespoke-wire-format vendor
stacks where inter-service contracts are proprietary.

### #47 — Seed Taxonomy as SMB Bootstrap

Compact, hand-tunable, gravity-keyword-classified ontology (5–10 entities per
category across four categories: Archetypes, Chart of Accounts, Domains,
Themes). Customers review and customize their entire taxonomy in approximately
30 minutes. Enterprise ontologies require weeks to months with specialized
staff.

### #48 — Customer-Owned Graph IP

The per-tenant graph and adapter weights are customer property, not a SaaS
side-effect. Export is a single-command operation. Foundry has no aggregate-data
rights without per-tenant opt-in. Inverts the lock-in pattern where customer
data is shaped to the vendor's ontology and exit is a multi-year project.

### #49 — Tier 0 Customer-Side Sovereign Specialist

A 1B sysadmin specialist plus Ring 1 and Ring 2 services on customer hardware
(approximately $300–$500 mini-PC for a 5-employee SMB). No GPU required. No
cloud dependency. AI-native SMB operations without hyperscaler complexity or
recurring subscription fees.

### #50 — Vertical Seed Packs Marketplace

Industry-specific starter taxonomies (pack-restaurant-smb, pack-law-firm-mid,
pack-hospital-regional, pack-real-estate-mid, pack-default). Compact,
permissively licensed, and intended to be community-extensible. Each pack is
small enough for a customer to understand in full before adopting it.

### #51 — Code-for-Machines First

Every contract, audit record, configuration, and ontology is machine-readable
as the primary surface. Human UIs are skins on machine-first APIs. Customer
extensions in any language via MCP. AI-native composition without a retrofit
step.

### #52 — Reverse-Flow Substrate

The same Doorman and audit ledger that enforce inbound discipline are intended
to also enforce outbound commercial flows: Data Marketplace and Ad Exchange
(IAB OpenRTB 2.6+). Per-tenant opt-in; cryptographic provenance required;
per-tenant moduleId isolation prevents cross-tenant leakage. SMB customers gain
a potential first-class revenue channel from data assets they already hold.

### #53 — Direct-Payment Settlement

Buyer-to-tenant payment is planned to flow directly (Stripe Connect for fiat;
crypto rail optional). Foundry is planned to take a transaction fee at
settlement, not a subscription fee on access. Customer is the intended recipient,
not the payer. The Brave BAT model — direct-payment-to-rights-holder at consumer
scale — is the validating precedent.

### #54 — Substrate-Without-Inference Base Case

The Totebox Archive operates fully without any AI tier (Tier A, B, and C all
unavailable). Deterministic-only mode in slm-cli provides the full sysadmin
surface. The "freely transferable" property — the customer may sell, transfer,
or inherit the Totebox without Foundry involvement, without re-training, and
without SaaS migration — is the strongest form of substrate sovereignty.

### #40 amendment — Purpose-Routed Tier Discipline

Tier A, B, and C differ by **purpose** (sysadmin specialist / editorial
generalist / external precision), not merely by size. Each tier has its own
training trajectory. This re-charters the Four-Tier SLM Substrate Ladder
originally ratified at v0.0.14, making Tier A first-class rather than a slower
version of Tier B.

---

## The structural bet

Hyperscalers (Salesforce, Microsoft, Oracle, Google, Amazon) are committed to
*concentrated compute + integrated stack + recurring revenue*. Foundry is
committed to *sovereign substrate + composable services + transactional revenue*.
The 2026 SMB market research validates the latter direction: "fastest-growing
deployment type within SMB software market: on-premises" (IDC); "the 2026
battleground is the Corporate Brain — a private, sovereign, persistent data
fabric" (Techaisle). The structural gap is real. Foundry is positioned at the
intersection where hyperscaler vendors are structurally unable to follow.

---

## Reference implementation

This monorepo is the engineering source for all twelve claims:

| Service | Claims realized | Cluster scope |
|---|---|---|
| service-slm | #43 #44 #46 #51 | project-slm |
| service-content | #44 #46 #47 #48 #54 | project-data |
| service-extraction | #44 #46 #54 | project-data |
| service-input | #46 #54 | project-data |
| service-fs | #46 #54 | project-data |
| service-people | #46 #54 | project-data |
| service-egress | #46 #54 | project-data |
| service-marketplace | #46 #48 #52 #53 | project-marketplace (new) |
| service-ad-exchange | #46 #52 #53 | project-marketplace |
| service-settlement | #46 #53 | project-marketplace |
| slm-cli (TUI) | #45 #51 #54 | project-slm |
| Tier 0 Totebox | #49 #54 + composition | (deployment instance) |

---

## When this document is amended

This INVENTIONS.md is amended at PATCH-level updates as new claims are ratified
at workspace doctrine tier. The structure (claim per section) is stable; the
count grows as the substrate matures.

---

## References

- `~/Foundry/DOCTRINE.md` v0.1.0 (constitutional source for all claims)
- `~/Foundry/conventions/*.md` (operational forms; one file per claim)
- `vendor/pointsav-monorepo/service-slm/docs/yoyo-training-substrate-and-service-content-integration.md`
  (origin research for claims #43, #44, #45)
- Workspace `CHANGELOG.md` v0.1.96 (the leapfrog-2030 staging entry)
