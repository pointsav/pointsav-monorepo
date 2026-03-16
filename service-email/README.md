# ⚙️ SERVICE-EMAIL
**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-5 Core Component
**Status:** Active Engineering / Operational

---

## I. ARCHITECTURAL MANDATE
n* **Pipeline Position:** Step 1 (Ingestion Gateway). Feeds the universal Spool for `service-people` and `service-content`.
This component operates as an autonomous Transport Interceptor. It penetrates legacy email infrastructures (Microsoft 365), extracts inbound assets, mutates the server state, and deposits raw OData JSON into temporary local queuing directories.

## II. EXECUTION MECHANICS
* **Inputs:** Microsoft Graph API (OData JSON).
* **Outputs:** Unparsed `.json` payloads written to `/assets/tmp-maildir/`.
* **Dependencies:** Secure OAuth2 App Registration (Entra ID) with `Mail.ReadWrite` application-level authority.
