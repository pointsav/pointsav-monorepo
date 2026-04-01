# PointSav Digital Systems: Sovereign Ingestion Gateway (IMAP Edition)
© 2026 PointSav Digital Systems

## I. Architectural Mandate: READ-ONLY
This node is configured strictly for IMAP traversal (e.g., Apple iCloud). The Destructor and Balancer modules are physically excluded from this architecture. This engine will ONLY copy assets to the local drive and update the Master Ledger. It will NEVER issue a delete command to the remote server.

## II. Execution Environment
**Target Compilation:** POSIX Compliant (macOS / Linux Cloud Native).
Duplicate `template.env` to `totebox-index.env` and populate your IMAP server details and App-Specific Password before executing.

---

## III. INITIALIZATION: The Database Primer (Run Once)
```bash
cd egress-roster
./prime_master_crm.sh
```

---

## IV. THE MASTER OPERATING LOOP
Execute these three steps sequentially from the root directory of your deployment.

### Step 1: The IMAP Crawler
```bash
cd egress-roster
./run_roster_imap.sh
```

### Step 2: The Physical Ingress
```bash
cd ../egress-archive-imap
./ingress_engine_imap.sh
```

### Step 3: The Incremental CRM Updater
```bash
cd ../egress-roster
./update_crm_eml.sh
```
