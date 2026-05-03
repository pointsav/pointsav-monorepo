# 🏛️ The Sovereign Foundry | Command Authority

This is the primary local engineering and orchestration substrate for **PointSav Digital Systems™** and **Woodfine Management Corp.** It strictly enforces the **Diode Standard** and operates under a 5-Silo architecture to maintain absolute domain isolation.

---

## 📂 The 5-Silo Taxonomy
The Foundry is physically divided into distinct source, identity, and egress domains:

1. **`factory-pointsav/`** (Source): Engineering logic, source code, system design, and Tier-6 documentation for PointSav.
2. **`fleet-woodfine/`** (Source): Operational deployment manifests and corporate governance for Woodfine.
3. **`sovereign-profiles/`** (Source): Public GitHub identity repositories (`.github`) for all administrators and contributors.
4. **`stage-pwoodfine/`** (Egress): Ephemeral outbound staging area for the Engineering Lead identity.
5. **`stage-jwoodfine/`** (Egress): Ephemeral outbound staging area for the Operations Lead identity.

---

## 🔄 Protocol 1: The Sovereign Sync Cycle
**Script:** `foundry_sync_v1.3.sh`

The Foundry syncs to public repositories using a **Weighted Activity Randomizer**. This protocol ensures 100% of the active source files reach GitHub daily while mimicking organic human collaboration.

* **Human Mimicry**: During execution, the script generates a randomized daily threshold (between 25% and 75%).
* **Assignment Dispatch**: Every repository is evaluated against this threshold. A repository is staged and pushed by either `pwoodfine` OR `jwoodfine` for that day, never both.
* **Result**: Over time, commit graphs reflect varied, natural human activity across the ecosystem.

---

## 📡 Protocol 2: Context Aggregation
**Scripts:** `generate_foundry_master.sh` & `stream_foundry_context.sh`

These tools aggregate the physical state of the 5-Silo Taxonomy into a single High-Fidelity Signal for architectural review and AI ingestion.

* **Mechanics**: They scan up to 4 levels deep for `README.md` and `.yaml` files.
* **Security**: They explicitly ignore binary `.git` data but intentionally parse public `.github` profile directories to ensure an accurate structural read.
* **Output**: `generate` writes to a static `FOUNDRY_MASTER_CONTEXT.md` file; `stream` pipes the data directly to standard output (STDOUT) for a zero-footprint read.

---
*Verified by the Sovereign Data Foundation. Adheres to Leapfrog 2030 standards.*
