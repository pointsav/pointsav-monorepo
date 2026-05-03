# 🛡️ FOUNDRY DOCTRINE: The Sovereign Airlock
**Status**: LOCKED FEB 23, 2026 | **Authority**: iMac 12.1

## 1. THE FOUR-SILO ARCHITECTURE
Every folder in `~/Foundry` represents a specific GitHub Identity or Organization.
* **factory-pointsav/**: The Vendor/Source (PointSav AG).
* **fleet-woodfine/**: The Customer/Operations (Woodfine Management Corp.).
* **stage-pwoodfine/**: Engineering Airlock (Identity: pwoodfine).
* **stage-jwoodfine/**: Operations Airlock (Identity: jwoodfine).

## 2. THE AIRLOCK PROTOCOL (Logic Flow)
Direct editing in `stage-*` folders is FORBIDDEN.
1. **Develop**: Edits are made only in `factory-pointsav/` or `fleet-woodfine/`.
2. **Sync**: `rsync` copies payloads to the Airlock (skipping `.git` metadata).
3. **Verify**: Check files in `stage-*` for correctness.
4. **Push**: Transmit from Airlock to staging identities using specific SSH keys.
5. **Merge**: Final push from staging to Organization via Administrator keys.

## 3. IDENTITY SHIELD (SSH Keys)
| Silo | Git Identity | SSH Key |
| :--- | :--- | :--- |
| **PointSav Factory** | `ps-administrator` | `id_ps-administrator` |
| **Woodfine Fleet** | `mcorp-administrator` | `id_mcorp-administrator` |
| **pwoodfine Staging** | `pwoodfine` | `id_pwoodfine` |
| **jwoodfine Staging** | `jwoodfine` | `id_jwoodfine` |

## 4. DEPLOYMENT REGISTRY (Feb 16 Memo Alignment)
* **Infrastructure**: `fleet-infrastructure-{onprem,leased,gcp}`.
* **Network**: `route-network-admin` (iMac 12.1 Host).
* **Platform**: `cluster-totebox-{corporate,personnel,real-property}`.
* **Terminals**: `node-console-{content,email,people}`.

---
*Verified Execution Environment. All nomenclature is locked per Version 3 of the Feb 16 Memo.*
