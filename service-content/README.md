# Service Content: The Linguistic Compiler

## Core Mandate
This engine acts as the deterministic Linguistic Compiler for the PointSav and Woodfine fleets. It ingests raw textual artifacts, applies the active Data Mesh (Corporate, Project, and Documentation Glossaries), and outputs high-fidelity documents utilizing mathematically strict language constraints.

## Supported Protocols
The engine requires a specific protocol manifest to execute a synthesis run.

* **MEMO:** For internal corporate overviews, structural summaries, and operational logic. Enforces high-density, Minto Pyramid structuring and strict paragraph mapping.
* **COMM:** For external transactional messaging (emails, media releases, social posts). Enforces BCSC continuous disclosure compliance, bans technological puffery, and secures institutional tone.
* **LEGAL:** For binding corporate agreements and disclosures. Enforces strict liability boundaries, Flow-Through Taxation definitions, and statutory phrasing.
* **TRANSLATE:** For strictly mapped 1-to-1 bilingual parity (English/Spanish) across all corporate artifacts.
* **TEXT:** For repository documentation, README files, system architecture definitions, and machine-facing text. Enforces the imperative mood, flat hierarchies, and the ISO 24495-1 Plain Language standard.

## Execution Syntax
All payload synthesis must follow the standard triple-argument execution:

```bash
cargo run -- <PROTOCOL_YAML_PATH> "<ENGINEERING_PROMPT>" <OUTPUT_DIRECTORY>
```

## System Architecture
* **Ingestion:** Automatically mounts all `.txt`, `.md`, and `.csv` files located in the target `artifacts/` directory to build the active context window.
* **Execution:** Transmits the payload to the Gemini API using strict structural mandates.
* **Output:** Writes the synthesized document to the isolated `outbox/` directory to prevent recursive context looping.

---
*© 2026 PointSav Digital Systems™*
