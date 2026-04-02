# WOODFINE MANAGEMENT CORP.: FLEET EXTRACTION SOP
## NODE: <NODE_NAME> (IMAP TEMPLATE)

### THE MANDATE
This node operates a **Sovereign Read-Only Loop**. In accordance with legal guidelines, you are extracting Apple iCloud data but you are strictly forbidden from executing destructive commands. 100% of data will remain in Staging (`/new`).

### STANDARD OPERATING PROCEDURE
Execute these steps strictly in order. **Do not skip steps.** Copy and paste the absolute paths below.

---

### STEP 1: The Crawler (Map the Cloud)
Scans the Apple iCloud server and builds the mathematical map for the next payload batch.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-roster
    ./run_roster_imap.sh

### STEP 2: The Physical Ingress (Download)
Downloads the payloads onto the physical USB drive, applying MD5 Hex-Armor and the 0-Byte Cryptographic Size Validator.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-archive-imap
    ./ingress_engine.sh

### STEP 3: The CRM & Ledger Vault (Update Intelligence)
Updates the relational database and autonomously drops an immutable, timestamped backup of your ledgers into the Snapshot Vault.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-roster
    ./update_crm_eml.sh

### STEP 4: The Aggressive Sentinel (Parity Check)
Mathematically proves the data transfer was flawless.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>
    ./audit_vault_integrity.sh

**🛑 NOTE:** Because this is a Read-Only node, there is no Step 5 (Destructor). Once the Sentinel reports `[ SYSTEM CLEAR ]`, your cycle is complete.
---
