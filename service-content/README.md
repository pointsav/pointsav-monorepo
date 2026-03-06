# Service Content: The Stateless Linguistic Compiler

## Core Mandate
This engine acts as the deterministic Linguistic Compiler for the PointSav and Woodfine fleets. It is a strictly stateless engine operating in its own isolated microkernel partition. It holds no structural templates and writes no data directly back to operational vaults.

## Execution Mechanics (The Donor-to-Outbox Lifecycle)
1. **Ingestion (State):** Reads inert donor assets from a target vault's `/assets/` directory based on verified `/ledger/` pointers.
2. **The Law (Source Control):** Reads universal Linguistic Tokens (e.g., `EXTRACT`, `DESIGN`) mapped from the Tier-6 `pointsav-design-system` anchor in Source Control.
3. **The Structure (Templates):** Retrieves the domain-specific layout from the target vault's local `/templates/` directory.
4. **Synthesis & Egress:** Synthesizes the final artifact and ejects it securely across the air-gap to the local `/outbox/` for human verification.

## The Glossary Anchor
To ensure 100% compliance during translation and extraction, `service-content` holds the localized Institutional Glossaries (`corporate`, `projects`, `documentation`) within its own isolated partition to enforce universal nomenclature lock.

## Execution Syntax
All payload synthesis must follow the standard triple-argument execution:

    cargo run -- <PROTOCOL_YAML_PATH> "<ENGINEERING_PROMPT>" <OUTPUT_DIRECTORY>

---
*© 2026 PointSav Digital Systems™*
