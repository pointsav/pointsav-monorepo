# os-mediakit
### *Autonomous Edge Substrate*

**Status: Active Engineering** | **Taxonomy: Tier-3 (Platform Layer)**

This component provides the baseline substrate for public-facing edge delivery. In the PointSav architecture, edge nodes are not "dumb" web servers that query central databases. `os-mediakit` enforces the **Autonomous Edge Node Pattern**.

## Architectural Mandate
An `os-mediakit` deployment bundles the presentation layer (`app-mediakit-knowledge`), the intelligence engine (`app-mediakit-telemetry`), and a local micro-vault into a single, highly portable execution environment. 

This design guarantees that if an edge node is compromised, the blast radius is strictly limited to that specific userspace. The attacker gains zero access to the broader `os-totebox` network.

---
*© 2026 PointSav Digital Systems™*
