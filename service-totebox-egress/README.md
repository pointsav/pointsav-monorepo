# PointSav Digital Systems: Totebox Egress Engine

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

© 2026 PointSav Digital Systems

## The Infinite Egress Loop (Archive Sweeper)
This subsystem is designed to natively bridge the Microsoft Exchange database chasm, bypassing standard Graph API quotas to extract and destroy assets directly from the In-Place Archive using legacy EWS SOAP protocols.

## Azure Entra ID (App Registration) Requirements
To authorize the Tier 1 EWS Parity Gate, the corporate Azure Application MUST have the following configurations:

1. **Authentication:** - `Allow public client flows` MUST be enabled.
2. **API Permissions (Microsoft Graph):**
   - `Mail.ReadWrite` (For Active Mailbox targeting)
   - `Directory.Read.All` (For Roster cross-referencing)
3. **API Permissions (Office 365 Exchange Online):**
   - *Crucial for Archive Egress:* You must select `APIs my organization uses` -> search for `Office 365 Exchange Online`.
   - Add Application Permission: `full_access_as_app`.
   - **Admin Consent MUST be granted by Woodfine Management Corp.**

## Execution Environment
Target Compilation: macOS 10.13.6 / Linux Cloud Native.
Fault Tolerance: Includes Level 4 exponential backoff for large multi-gigabyte extractions.
