# Sovereign Workflow & Governance Manual
### *Protocol for the PointSav Factory & Woodfine Fleet*

**Version:** 1.0  
**Status:** Active  
**Scope:** Engineering (P. Woodfine) & Operations (J. Woodfine)

---

## 1. The Sovereign Architecture
The system is divided into three distinct zones to ensure security and governance.

| Zone | Location | Purpose |
| :--- | :--- | :--- |
| **The Factory** | `~/Foundry/factory-pointsav` | **Source of Truth (Engineering).** The Golden Master for code. |
| **The Fleet** | `~/Foundry/fleet-woodfine` | **Source of Truth (Operations).** The Golden Master for deployment. |
| **The Foundry** | `~/Foundry/identities` | **The Wardrobe.** Stores personal profiles (PWoodfine/JWoodfine). |
| **The Stage** | `~/Foundry/stage-*` | **The Air Gap.** Temporary areas for pushing to GitHub. |

---

## 2. The Matrix Protocol (Co-Authoring)
We utilize a **Matrix Strategy** where both users maintain copies of all repositories.

* **P. Woodfine (Engineering Lead):** Primary on Code, Secondary on Fleet.
* **J. Woodfine (Operations Lead):** Primary on Fleet, Secondary on Code.

**The Golden Rule:**
> **NEVER** edit files in `stage-*`.  
> **ALWAYS** edit in `factory` or `fleet`, then Sync.

---

## 3. The 3-Cycle Workflow

### Cycle A: The Engineering Loop (Code)
1.  **Author:** Edit code in `factory-pointsav/pointsav-monorepo`.
2.  **Sync:** Run `~/Foundry/sync_to_staging.sh`.
3.  **Push:** Go to `stage-pwoodfine/pointsav-monorepo` and push.
4.  **Governance:** Open PR from `pwoodfine` -> `pointsav` (Corp).

### Cycle B: The Operations Loop (Nodes)
1.  **Author:** Edit manifests in `fleet-woodfine/fleet-deployment-manifest`.
2.  **Sync:** Run `~/Foundry/sync_to_staging.sh`.
3.  **Push:** Go to `stage-jwoodfine/fleet-deployment-manifest` and push.
4.  **Governance:** Open PR from `jwoodfine` -> `woodfine` (Corp).

### Cycle C: The Recovery Loop (Restoration)
1.  **Event:** Local data loss or drift.
2.  **Action:** Go to `factory` or `fleet`.
3.  **Command:** `git pull upstream main`.
4.  **Result:** Local Foundry matches Corporate Golden Master.

---

## 4. Scripts & Tools
* **`sync_to_staging.sh`**: Wipes staging and mirrors Factory/Fleet to both users.
* **`generate_context_factory.sh`**: Scans this repo and monorepo for AI Context.
* **`generate_context_fleet.sh`**: Scans fleet manifests for AI Context.

---
*Verified by the Sovereign Foundry.*
