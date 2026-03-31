# POINTSAV DIGITAL SYSTEMS: MASTER MONOREPO

## ENGINEERING MANDATE
PointSav Digital Systems engineers cryptographic, hardware-agnostic infrastructure for institutional asset managers. The primary deliverable is the Federated Panopticon—a zero-latency Command Ledger that mathematically aligns semantic intelligence with absolute physical data custody.

## ARCHITECTURAL BASELINE: V1.8
The core infrastructure relies on the Asymmetric Storage protocol:
1. **The Semantic Substrate:** Lightweight indices (`.jsonl`) hosted on federated Tier-2 cloud nodes.
2. **The Base Assets:** Heavy physical mass (`.pdf`, `.eml`) retained strictly in Tier-1 offline cold storage.

This bifurcation ensures complete data sovereignty, providing institutional clients with absolute infrastructure optionality and seamless portability across all commercial hyperscaler environments.



---
### `service-email-template`
* **Language:** Rust
* **Architecture:** Silent Sync Distribution Hub (M365 Graph API)
* **Customer:** Woodfine Management Corp.
* **Function:** Compiles operational email templates (text bodies + PDF/image assets) from the local monorepo and silently synchronizes them directly into target Microsoft 365 folders. Bypasses standard SMTP to enforce absolute version control without inbox flooding. Includes a self-distributing, neuro-inclusive `.html` offline catalog.
