# PointSav Digital Systems: Sovereign Ingestion Gateway (EWS Edition)
© 2026 PointSav Digital Systems

## I. The Infinite Egress Loop (Archive Sweeper)
This subsystem natively bridges the Microsoft Exchange database chasm. It bypasses standard Graph API quotas to extract and destroy assets directly from the In-Place Archive using legacy EWS SOAP protocols.

## II. Azure Entra ID (App Registration) Requirements
To authorize the Tier-1 EWS Parity Gate, the corporate Azure Application MUST maintain the following configurations:
1. **Authentication:** `Allow public client flows` MUST be enabled.
2. **API Permissions (Microsoft Graph):** `Mail.ReadWrite`, `Directory.Read.All`
3. **API Permissions (Office 365 Exchange Online):** `full_access_as_app` (Requires Admin Consent).

## III. Execution Environment
**Target Compilation:** POSIX Compliant (macOS / Linux Cloud Native).
Duplicate `template.env` to `totebox-index.env` and populate your Microsoft credentials before executing.

---

## IV. INITIALIZATION: The Database Primer (Run Once)
```bash
cd egress-roster
./prime_master_crm.sh
```

---

## V. THE MASTER OPERATING LOOP
Execute these five steps sequentially from the root directory of your deployment.

### Step 1: The Crawler (Phase 1)
```bash
cd egress-roster
./run_roster_ews.sh
```

### Step 2: The Physical Ingress (Phase 2)
```bash
cd ../egress-archive-ews
./ingress_engine.sh
```

### Step 3: The Incremental CRM Updater
```bash
cd ../egress-roster
./update_crm_eml.sh
```

### Step 4: The Destructor & Commit (Phase 3 & 4)
```bash
cd ../egress-prune
./run_prune_ews.sh
```

### Step 5: The Thermodynamic Balancer (Phase 5)
```bash
cd ../egress-balancer
./run_balancer_ews.sh
```
