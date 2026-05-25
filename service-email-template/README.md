# service-email-template
**Vendor:** PointSav Digital Systems
**Customer:** Woodfine Management Corp.
**Architecture:** Silent Sync Distribution Hub (Rust / M365 Graph API)

## Operational Overview
`service-email-template` is an autonomous distribution engine. It compiles operational templates (text bodies + PDF/image assets) from the local PointSav monorepo and utilizes the Microsoft Graph API to silently synchronize them directly into Woodfine Management Corp. operator mailboxes.

It enforces absolute version control. By bypassing the traditional SMTP send protocol and manipulating the M365 database directly, it prevents inbox flooding and ensures operators are always executing with the most current linguistic and legal assets.

## The "Silent Sync" Protocol
1. **Compilation:** The Rust engine reads `manifest.json`, pairing local `/templates` with local `/assets`, and dynamically generates the interactive `catalog_base.html` dashboard.
2. **Telemetry Injection:** A globally unique search key (e.g., `[TMPL-042]`) is injected into the header of every compiled email.
3. **The Purge:** The engine connects to M365, scans the target `Template Ledger` sub-folders, and executes a hard deletion of any email containing the old `[TMPL-]` keys.
4. **The Payload Injection:** The newly compiled template emails are injected directly into their respective categorical sub-folders.
5. **The Master Dashboard (`[TMPL-000]`):** The engine injects a single email into the root of the `Template Ledger` folder. This email contains the freshly compiled `catalog_base.html` as an attachment, ensuring operators always have immediate access to the most current offline search dashboard.

## Directory Structure

service-email-template/
├── Cargo.toml            # Rust dependencies (reqwest, serde, msft-graph)
├── Identity.env          # [IGNORED BY GIT] M365 API Keys & Broadcast Targets
├── manifest.json         # The State Ledger mapping templates to assets
├── README.md             # System Documentation
├── src/                  # Rust compiler engine and Graph API connection logic
├── assets/               # Physical storage for riders, NDAs, schedules
└── templates/            # Raw .txt/.html email bodies and catalog_base.html


## Operator Execution Loop (M365)
1. Open Microsoft 365.
2. Navigate to the root `Template Ledger` folder and open the `[TMPL-000]` email.
3. Download/Open the attached `.html` file to view the interactive Template Dashboard.
4. Locate the desired template and click "Copy Key" (e.g., `[TMPL-088]`).
5. Paste the Key into the M365 Search Bar to instantly isolate the template.
6. Click **Forward**, adjust the Subject Line, clear the Telemetry Header, and deploy.

## Security & Egress
* **Strict Decoupling:** Personal Information (PII) and internal assets are never pushed to external web servers.
* **Identity Protection:** `Identity.env` strictly isolates all Microsoft Graph connection strings from the repository.
