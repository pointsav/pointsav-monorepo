# WOODFINE MANAGEMENT CORP.: FLEET EXTRACTION SOP
## NODE: <NODE_NAME> (EWS TEMPLATE)

### THE MANDATE
This node operates a **Destructive Extraction Loop**. You are authorized to extract corporate assets from the Microsoft cloud, secure them physically, and permanently incinerate the server copies. 

### STANDARD OPERATING PROCEDURE
Execute these steps strictly in order. **Do not skip steps.** Copy and paste the absolute paths below.

---

### STEP 1: The Crawler (Map the Cloud)
Scans the Microsoft Exchange server and builds the mathematical Kill List for the next payload batch.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-roster
    ./run_roster_ews.sh

### STEP 2: The Physical Ingress (Download)
Downloads the payloads onto the physical USB drive, applying MD5 Hex-Armor and the 0-Byte Cryptographic Size Validator.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-archive-ews
    ./ingress_engine.sh

### STEP 3: The CRM & Ledger Vault (Update Intelligence)
Updates the relational database and autonomously drops an immutable, timestamped backup of your ledgers into the Snapshot Vault.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-roster
    ./update_crm_eml.sh

### STEP 4: The Aggressive Sentinel (Parity Check) - CRITICAL
Mathematically proves the data transfer was flawless. It searches for Legacy Traps, Vault Overlaps, and Ledger Fractures.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>
    ./audit_vault_integrity.sh

**🛑 STOP:** If the Sentinel prints a red `[ CRITICAL HALT ]` banner, you are stripped of operational clearance. **DO NOT PROCEED TO STEP 5.** Contact engineering immediately. If the Sentinel prints `[ SYSTEM CLEAR ]`, proceed to Step 5.

### STEP 5: The Destructor (Vaporize Cloud Copies)
Uses Atomic Rsync to move the physical files to Cold Storage (`/cur`) and issues the `HardDelete` command to Microsoft.

    cd /Users/Office/woodfine-fleet-deployments/cluster-totebox-personnel-<NODE_NAME>/egress-prune
    ./run_prune_ews.sh

---
