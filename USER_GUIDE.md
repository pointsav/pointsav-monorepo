# Woodfine / PointSav Platform — User Guide

> Generated: 2026-03-28
> To regenerate this document, run `/generate-user-guide` in Claude Code.

---

## Table of Contents

- [Part I: The Foundation — Why This Exists](#part-i-the-foundation--why-this-exists)
- [Part II: The Vision — Where This Is Going](#part-ii-the-vision--where-this-is-going)
- [Part III: The Architecture — How It Is Organized](#part-iii-the-architecture--how-it-is-organized)
- [Part IV: The Security Model](#part-iv-the-security-model)
- [Part V: Data Architecture — Files Over Databases](#part-v-data-architecture--files-over-databases)
- [Part VI: The Data Pipeline — How Information Flows](#part-vi-the-data-pipeline--how-information-flows)
- [Part VII: The Operator Interface](#part-vii-the-operator-interface)
- [Part VIII: Fleet Deployment](#part-viii-fleet-deployment)
- [Part IX: Operational Guides](#part-ix-operational-guides)
- [Part X: Architecture Decisions](#part-x-architecture-decisions)
- [Part XI: The Road Ahead — Moonshots](#part-xi-the-road-ahead--moonshots)
- [Appendix: Glossary](#appendix-glossary)

---

# Part I: The Foundation — Why This Exists

## The Companies

There are four organizations involved in this platform. Understanding who they are and how they relate to each other is the best starting point.

**Woodfine Capital Projects Inc.** is a real estate development company. It is developing more than 75 LEED-certified buildings over an 8 to 10 year period across Canada, the United States, Mexico, and Spain. This is a traditional real estate firm — not a technology company. The domain language throughout this platform (tenants, minute books, land titles, reporting issuers) comes from real estate and corporate law, not from software.

**Woodfine Management Corp.** (called "MCorp" internally) is a subsidiary of Woodfine Capital Projects. It is the primary customer for all software built by PointSav. When source materials refer to "the customer," they almost always mean MCorp. MCorp uses the platform to run the day-to-day operations of the broader organization.

**PointSav Digital Systems** is the technology arm. It is wholly owned by Woodfine Capital Projects, and it builds the software. PointSav operates on a cost-plus model — it builds for its own operations first, and only monetizes after the core platform is mature. This is deliberately different from a startup model: there is no hypothetical customer to guess at. The customer is Woodfine, which is in the same building.

**Sovereign Data Foundation** is a Danish organization that holds a 10% equity stake in PointSav. Its role is to oversee the integrity of the open-source components and to ensure that the "Freely Transferable" guarantee (explained in Part V) is enforceable.

## The Three Pillars

Every piece of software PointSav builds serves one or more of three business purposes. These are not technical categories — they are the business reasons the platform exists.

**Business Administration.** Managing properties, tenants, service providers, communications, budgets, and reporting. This is the operational work that happens every day inside a real estate company.

**Record Keeping.** Personnel records, corporate records (financial books, minute books, statutory calendars), and real property records. These are records that must exist for legal and regulatory reasons, must be auditable, and must be preserved for the full lifetime of the asset they describe.

**Cyber-physical Connectivity.** IoT sensors, Building Information Modelling (BIM) drawings, materials databases, digital twins, and remote building management. This is where the physical buildings connect to software.

If a component cannot be traced back to one of these three pillars, it needs a strong reason to exist.

---

# Part II: The Vision — Where This Is Going

## The End State

The long-term goal is a platform where every major component runs as its own self-contained operating system — a purpose-built machine that does one thing and does it well. Think of a physical office network where the printer has its own firmware, the network-attached storage has its own firmware, and the firewall has its own firmware. Each device is independent. If the printer is compromised, the attacker is physically trapped inside the printer — they cannot reach the filing cabinet.

That is the model PointSav is building toward, but virtualized and applied to corporate data. Each component will run as a tiny, isolated virtual machine called a unikernel (explained below). No two components share a kernel. If one is compromised, the damage is mathematically contained to that single machine.

The reference implementations for this architecture are AWS Firecracker (a micro-VM technology) and the seL4 kernel (a mathematically proven microkernel). These are reference points, not locked-in choices.

## The Current Reality

The end state is years away. Right now, the platform runs as ordinary Linux applications, mostly on AWS cloud servers. This is the MVP — Minimum Viable Product — phase.

What matters is that the **component names, boundaries, and data ownership rules are the same in both phases**. The code you write today should treat each component as if it were already running on its own machine. That means:

- Components communicate over the network, not through shared files or databases.
- Each archive's data is self-contained and can be packaged up independently.
- Authentication is designed so it can later be replaced by hardware-level cryptography.

Low-cost choices made today smooth the transition to the end state. Hardcoded connection strings and shared databases make the transition expensive.

## The 3-Layer Stack

The full system breaks into three layers.

**Infrastructure.** Raw computing power. Physical servers, cloud instances, or dedicated hardware that provides CPU, memory, storage, and networking. In the end state, this layer runs a PointSav-built virtualization substrate called InfrastructureOS. In the MVP, it is AWS EC2.

**Platform.** The operating systems themselves — the ToteboxOS archives (data vaults), the InterfaceOS logic hubs, and the NetworkAdminOS that manages the network. These components run as kernel-isolated virtual machines on top of the infrastructure layer.

**Delivery.** The ConsoleOS — the software each operator uses at their desk. In the end state, each variant runs as its own virtual machine on the user's workstation. In the MVP, it is Linux applications with a terminal interface.

```
+--------------------------------------------------+
|  DELIVERY LAYER                                  |
|  ConsoleOS variants on operator workstations     |
|  (FKeysConsole, InputMachine, BIMConsole, etc.)  |
+--------------------------------------------------+
                        |
                        | network
                        |
+--------------------------------------------------+
|  PLATFORM LAYER                                  |
|  ToteboxOS archives  (data)                      |
|  InterfaceOS hubs    (logic)                     |
|  NetworkAdminOS      (management)                |
+--------------------------------------------------+
                        |
                        | runs on
                        |
+--------------------------------------------------+
|  INFRASTRUCTURE LAYER                            |
|  InfrastructureOS / AWS EC2 / on-prem servers    |
+--------------------------------------------------+
```

## The Licensing Boundary

Single-archive use — one person, one Totebox Archive, one ConsoleOS — is completely free and open source. This is the Community Member path.

The moment you need to aggregate across multiple archives (for example, pulling together personnel records and property records for a project team), you need the InterfaceOS, which is proprietary software. This aggregation layer is how PointSav monetizes: it does not charge for sovereign data storage, but it charges for the logic that connects multiple vaults together.

This boundary is not just commercial — it is a design constraint. FOSS (free and open source) components must never contain proprietary logic. An independent developer should be able to run a Totebox Archive and a ConsoleOS variant for their own use without touching any proprietary code.

---

# Part III: The Architecture — How It Is Organized

## The Five Core Components

The platform is organized around five named components. These names are canonical — they appear in code, documentation, and conversation. Do not substitute synonyms.

### ToteboxOS (The Data Vault)

A ToteboxOS instance is a self-contained store for one specific asset. Think of it as a sealed filing cabinet that belongs entirely to one entity — a building, a company, or a person. Everything relating to that entity lives inside it: documents, records, databases, and logs.

In the current phase, a ToteboxOS instance is a structured directory on a cloud server. In the end state, it is a minimal operating system — a unikernel — that runs as a virtual machine, boots from a disk image, and can be physically moved to another computer.

There are three preset types of Totebox Archives:

| Archive Type | Anchored To | Contains |
|---|---|---|
| PersonnelArchive | SIN or Passport ID | Professional network, identity records, contact history |
| CorporateArchive | Business Incorporation Number or Tax ID | Financial records, minute books, statutory ledgers, calendars |
| RealPropertyArchive | Land Title PIN or legal address | Property data, permits, lifecycle logs, BIM drawings, IoT data |

The anchor identifier is the universal key. A PersonnelArchive anchored to a SIN number ties together every record about that individual regardless of which system generated it.

### InterfaceOS (The Logic Hub)

The InterfaceOS is a stateless layer — it holds no data of its own. Its job is to connect a ConsoleOS to one or more ToteboxOS archives and apply business logic to the combined result.

If a user only needs to view one archive, they can connect to it directly from their ConsoleOS. The InterfaceOS becomes necessary when they need to see across multiple archives simultaneously — for example, a project manager who needs to view both the building's records and the personnel records for the project team at the same time.

This aggregation function is the monetization boundary. The main variant used by Woodfine is the CommandCentre, which aggregates PersonnelArchives and CorporateArchives.

Other InterfaceOS variants include:

| Variant | Purpose |
|---|---|
| CommandCentre | Primary administrative hub. Aggregates Personnel and Corporate Archives |
| BIMServer | Serves structured Building Information Modelling data to field engineers |
| GISEngine | Geographic analysis — origin-destination studies, mapping |
| IoTConnect | Bridges IoT device logs: water sensors, access doors, temperature |
| DataWarehouse | Aggregates data from multiple archives for analytics |
| BuildingIDServer | Links digital archives to physical building identifiers |
| JobShackServer | Construction project coordination |

### ConsoleOS (The User Interface)

The ConsoleOS is what operators use at their desk. Each variant is purpose-built for a specific workflow and connects to a specific InterfaceOS variant. In the current phase, these are Linux applications. In the end state, each variant runs as its own virtual machine on the user's workstation.

The primary variant is the FKeysConsole — a terminal interface where each function key activates a different context within the corporate record system. It is designed to be fast, keyboard-driven, and stable across backend upgrades.

Other ConsoleOS variants:

| Variant | Purpose |
|---|---|
| FKeysConsole | Primary administrative terminal. F-key driven interface to archives |
| InputMachine | Data entry and record creation |
| BIMConsole | Building Information Modelling for field engineers |
| MapsConsole | Geographic and location intelligence visualization |
| DataConsole | Analytics and data access (also called DataMarketplace) |
| BuildingPreferences | IoT device management and building state control |
| SiteSuper | Construction site coordination |

### InfrastructureOS (The Virtualization Substrate)

InfrastructureOS is the layer that physical hardware runs. It provides the virtualization environment for all other components — the CPU, memory, storage, and networking that ToteboxOS instances and InterfaceOS hubs run on top of.

In the MVP phase, this is AWS. In the end state, it is a purpose-built PointSav operating system that can run on on-premises servers, leased dedicated servers, and public cloud simultaneously.

### NetworkAdminOS (The Network Manager)

NetworkAdminOS gives operators visibility and control over everything running in the Private Network. It maintains a registry of which components are paired to which — the MBA registry (see Part IV). It handles health monitoring, deployment status, and network isolation commands.

The distinction from InfrastructureOS is worth emphasizing: InfrastructureOS is the substrate (like a server rack), while NetworkAdminOS is the management console (like the dashboard that shows you what is running in the rack).

## The Canonical Naming Table

The table below lists canonical names and what NOT to substitute them with. Using alternate names in code or conversation creates confusion because the domain has a precise vocabulary.

| Canonical Name | Do NOT Call It |
|---|---|
| ToteboxOS | ArchiveOS, VaultOS, DataNode |
| InterfaceOS | LogicLayer, AggregationService, MiddleTier |
| ConsoleOS | ClientApp, Terminal, Frontend |
| InfrastructureOS | HypervisorLayer, VMSubstrate |
| NetworkAdminOS | MeshController, NodeManager |
| ToteboxArchive | (generic name for any ToteboxOS instance with its data) |
| PersonnelArchive | HRArchive, PeopleNode |
| CorporateArchive | FinanceArchive, BusinessNode |
| RealPropertyArchive | BuildingArchive, AssetNode |
| CommandCentre | OrchestratorHub, AccessGateway |
| FKeysConsole | MainTerminal, AdminConsole |
| PairingAsPermission | RBAC, ACL, PermissionSystem |
| FreelyTransferable | portable, migratable |
| TheSnap | (cross-archive verification mechanism) |
| Contributor | employee, developer, team member |
| GeneralStaff | end users, employees |
| Assignment | ticket, story, task |

---

# Part IV: The Security Model

## Why This Matters

Most software systems grant access through usernames, passwords, and a database of permissions. An attacker who steals credentials — or who tricks an administrator into granting them a role — can move through the system as if they were an authorized user.

PointSav's security model rejects this entirely. Access is not granted through identity credentials. It is granted through the physical topology of the network itself. If two machines are not cryptographically paired, they cannot communicate. There is no credentials database to steal because there are no credentials.

This model is called **Pairing as Permission** and it is implemented through **Machine-Based Authorization (MBA)**.

## How It Works

Every node in the network has a machine identity — a cryptographic certificate tied to that specific hardware instance. When two nodes are paired, they exchange cryptographic keys and register the pairing in the MBA registry maintained by NetworkAdminOS.

From that point on, the question the system asks is not "Does this user have permission?" but rather "Is this ConsoleOS instance paired with this InterfaceOS instance?" The answer is visible directly from the network topology.

**An example:** A project team needs access to two buildings and their associated staff records.

1. Deploy one CommandCentre (InterfaceOS) instance.
2. Pair it to the two RealPropertyArchive instances for those buildings.
3. Pair it to the PersonnelArchive instances for the team members.
4. Pair each team member's FKeysConsole to the CommandCentre.

The access boundary is now defined by the wiring diagram. No permissions database was updated. No user roles were assigned. The architecture IS the access control. This is what Peter Woodfine calls **Geometric Security**.

## What This Means for the MVP

The full cryptographic hardware pairing is a future-state capability. In the MVP, the platform uses conventional authentication. However, this conventional auth is implemented behind a clean interface so it can later be swapped for MBA without rewriting business logic.

When designing any component that involves access control, the question to ask is: "Could this access pattern be expressed through component topology rather than a permissions table?" If the answer is yes, prefer the topology-based pattern even if the underlying mechanism is still conventional.

## Capability-Based Security

Underneath the MBA model is a deeper principle: Capability-Based Security. In a conventional operating system, an application can often escalate its privileges if it finds a vulnerability — a compromised application can reach out and touch other parts of the system.

In the PointSav architecture, each component holds a cryptographic capability token that specifies exactly which other components it is allowed to communicate with. An application cannot communicate with something it doesn't hold a capability for. If an edge node (such as the MediaKitOS web layer) is compromised, the attacker is trapped inside that node's memory sandbox. They hold no capabilities to reach the ToteboxOS vaults. The blast radius is physically contained.

The seL4 kernel is the reference point for this approach because it is the only operating system kernel whose security properties have been mathematically proven — not just tested, but formally verified using machine-checked mathematical proofs.

## The Snap: Cross-Archive Integrity Verification

The Snap (formally the IssuerSnap) is a verification mechanism that cross-checks records across multiple archives. It is used primarily for producing authenticated quarterly reports as required of Reporting Issuers (publicly-traded limited partnerships).

The MediaKitOS pulls data from Corporate Archives (financial records) and Real Property Archives (property data), cross-references them against Personnel Archives (who is responsible for what), and generates a report whose data chain can be verified end-to-end. Each cross-reference uses the anchor identifiers described in Part III — a Corporate Archive references a person by their SIN, not by a local database ID.

---

# Part V: Data Architecture — Files Over Databases

## The Core Principle

A database is a running software engine. If the database software is shut down, deprecated, or becomes unavailable, your data is inaccessible. A flat file — a text file, a YAML file, a CSV spreadsheet — is just bytes on a disk. It can be read with any text editor on any computer, now or in one hundred years.

This is the philosophical foundation of PointSav's data architecture: store corporate knowledge as inert flat files, not in database engines. Software engines read and process those files, but the files exist independently of the engines.

The practical mandate is captured in the Totebox directory structure:

```
cluster-totebox-corporate/
├── app-console-input/      <-- execution software (processes the files)
├── assets/                 <-- physical vault (PDFs, images, contracts)
└── ledger/                 <-- state machine (YAML metadata, CSV ledgers)
```

The `assets/` directory holds the physical files. The `ledger/` directory holds the metadata and cryptographic checksums. The execution software in `app-console-input/` reads and writes to both — but the data exists without it.

## The Freely Transferable Standard

A ToteboxArchive must be exportable as a complete, self-contained package that a recipient can deploy on their own infrastructure without any proprietary runtime, vendor relationship, or platform subscription.

In the end state, this export artifact is a Bootable Disk Image — a virtual machine image file that boots on any standard hypervisor (bare metal, AWS, Azure, Google Cloud, Oracle Cloud).

**Important distinction:** A Docker container is NOT Freely Transferable. It requires Docker to run. A bootable disk image requires only a standard hypervisor, which is a universal commodity.

A practical example: When Woodfine sells a building, the entire history of that building — permits, contracts, IoT logs, maintenance records — lives in a RealPropertyArchive. The buyer receives a Bootable Disk Image of that archive. They boot it on their own infrastructure. The complete history is now under their control, with zero ongoing dependency on Woodfine's systems.

In the MVP phase, "Freely Transferable" means: no proprietary data formats, no vendor lock-in in the data path, and "export this archive" as a tested operation from day one.

## Cryptographic Ledgers

Because flat files cannot defend themselves — a rogue system administrator could silently edit a YAML file and backdate the change — PointSav enforces integrity at the filesystem level using SHA-256 checksums.

When a physical asset (a PDF contract, an invoice, a compliance document) is stored in a ToteboxArchive, the system performs two simultaneous operations:

1. The file is placed in the `/assets/` vault with execution permissions stripped. It becomes an inert binary — software cannot run it, only read it.

2. A `.yaml` file is created in the `/ledger/` directory containing the file's metadata (author, date, category) and a SHA-256 cryptographic checksum of the file's exact contents.

A SHA-256 checksum is a mathematical fingerprint. If anyone changes even a single character in the original file — a number in a financial record, a date on a contract — the fingerprint no longer matches. Any auditor can independently re-run the checksum against the stored file and compare it to the one in the ledger. If they match, the document is verified. If they don't, the archive has been tampered with.

This is the WORM principle — Write Once, Read Many. Once a record enters cold storage, it is mathematically sealed.

## The Sovereign Replacement Initiative

Any digital transformation begins with a dependency on third-party software. Microsoft 365 for email, AWS for hosting, proprietary authentication systems. These dependencies represent a risk: if a vendor changes their terms of service, deprecates an API, or goes out of business, the corporate infrastructure that depends on them breaks.

PointSav tracks every third-party dependency as a formal technical debt entry. Foreign dependencies are quarantined in isolated directories (named `vendor-*`) where their behavior is tightly controlled. For each quarantined dependency, a corresponding Moonshot engineering initiative (in a `moonshot-*` directory) works on a native replacement.

When the native replacement reaches parity, it physically replaces the quarantined vendor code. This continuously shrinks the platform's dependency surface and eliminates vendor lock-in over time.

---

# Part VI: The Data Pipeline — How Information Flows

## Overview

The data pipeline describes how information from the outside world — primarily email — enters the ToteboxArchive system, gets processed, and becomes queryable corporate knowledge. There are seven named components in this pipeline.

```
 ╔════════════════════════════════╗
 ║   EXTERNAL WORLD               ║
 ║   (Microsoft 365, Email)       ║
 ╚═══════════════╤════════════════╝
                 │  OAuth2
                 ▼
 ┌───────────────────────────────┐
 │  service-email                │
 │  (Transport Interceptor)      │
 │  Pulls raw email from M365.   │
 │  Writes to tmp-maildir queue. │
 │  Zero intelligence applied.   │
 └──────────────┬────────────────┘
                │
                ▼
 ┌───────────────────────────────┐
 │  service-parser               │
 │  (Traffic Controller)         │
 │  Strips MIME/JSON formatting. │
 │  Creates Entity Bundles.      │
 │  Routes by payload type.      │
 └─────┬──────────────┬──────────┘
       │              │
       ▼              ▼
 Structured      Unstructured
 data            human text
 (.csv, .xlsx)   (email body, PDF)
       │              │
       ▼              ▼
 Deterministic   ┌────────────────┐
 services        │  service-slm   │
 (service-people,│  (AI Air-Lock) │
  telemetry)     │  Sub-billion   │
                 │  param model.  │
                 │  Extracts facts│
                 │  then shuts    │
                 │  down.         │
                 └───────┬────────┘
                         │
                         ▼
                 ┌────────────────┐
                 │ service-content│
                 │ (Knowledge     │
                 │  Graph)        │
                 │ Self-healing   │
                 │ First          │
                 │ Derivative.    │
                 └───────┬────────┘
                         │
                         ▼
                 ┌────────────────┐
                 │ Verification   │
                 │ Surveyor       │
                 │ Human confirms │
                 │ identity facts.│
                 │ 10/day limit.  │
                 └───────┬────────┘
                         │
                         ▼
                 ┌────────────────┐
                 │ service-search │
                 │ (Inverted      │
                 │  Index)        │
                 │ Microsecond    │
                 │ search across  │
                 │ all files.     │
                 └───────┬────────┘
                         │
                         ▼
                 ┌────────────────┐
                 │ FKeysConsole   │
                 │ (Operator HUD) │
                 │ F2/F3/F4/F12   │
                 └───────┬────────┘
                         │
                         ▼
                 ┌────────────────┐
                 │ service-message│
                 │ -courier       │
                 │ Outbound       │
                 │ messaging via  │
                 │ adapters.      │
                 └────────────────┘
```

## service-email — The Transport Interceptor

The email service's only job is to pull email out of the external cloud (Microsoft 365) and write it to a local disk. It uses an OAuth2 cryptographic handshake against the Microsoft Graph API to authenticate. It polls for unread messages, extracts the raw data, writes it to a temporary queue directory (`/assets/tmp-maildir/`), and marks the message as read on the external server to prevent it from being pulled again.

This service deliberately has no intelligence. It does not read the email content, categorize it, or make any decisions. It is a transport mechanism only.

## service-parser — The Traffic Controller

The parser receives raw email from the queue and strips the proprietary formatting (MIME multipart, Base64 encoded attachments, JSON wrappers) to produce clean, readable content.

It then routes the cleaned content based on what type of data it is:

- **Structured data** (spreadsheets, CSV files, telemetry logs) goes directly to the appropriate deterministic service. These have predictable formats that can be parsed with 100% accuracy using standard algorithms.
- **Unstructured human text** (email body text, PDFs, Word documents) is routed to service-slm for AI extraction.
- **Consumable media** (newsletters, marketing emails) is routed directly to the AI synthesis engine and then deleted — it is not kept in long-term storage.

The raw original file is always vaulted before any processing begins. This maintains chain of custody.

## service-slm — The AI Air-Lock

The SLM service (Small Language Model) is an AI component that processes unstructured human text to extract structured facts. "Small Language Model" refers to a compact AI model (sub-one-billion parameters) that runs entirely on the organization's own hardware — no external API calls, no data leaving the network.

The service's operating principle is: it reads, extracts, and then **shuts down**. It does not maintain a running connection to external AI services. It receives raw text via standard input, produces structured Markdown output, and terminates. This is the "point-in-time execution" model.

This prevents the two failure modes that plague commercial AI integrations:
1. Corporate data being absorbed into a third-party AI provider's training data.
2. The AI having ongoing influence over stored records (which could introduce invisible errors over time).

## service-people — The Personnel Ledger

The personnel ledger maintains the master contact database for the organization. It stores unique identifiers (SIN or Passport ID as the anchor), contact states, and communication history for every person in the corporate network.

It operates as a deterministic flat-file engine — a JSON-based state machine rather than a conventional database. It processes queries and updates from authorized execution adapters (the ConsoleOS F2 key interface) and executes read/write operations against the stored files.

## service-content — The Knowledge Graph

The content service maintains the "First Derivative" — the self-healing knowledge graph derived from all the raw source material in the archive. Where service-email holds the immutable originals, service-content holds the extracted, processed intelligence.

The knowledge graph is organized around four control valves that update at different rates to preserve stability:

| Control Valve | Update Rate | Contents |
|---|---|---|
| Archetypes | Every 24+ months | The psychological and functional identity of the firm |
| Chart of Accounts | Every 18–24 months | The structural and financial categories. Requires Executive override to change |
| Domains | Every 12+ months | The major theaters of operation: Corporate, Projects, Documentation |
| Themes | Every 3–8 months | Active operational narratives (e.g., "Co-Location Expansion") |

These slow update rates are deliberate. Stability in the taxonomy means that data recorded five years ago and data recorded today are comparable and searchable in the same terms.

## Verification Surveyor — The Human Checkpoint

The Verification Surveyor is an intentional bottleneck in the identity resolution pipeline. Before an extracted person record can be committed to the verified ledger, a human operator must confirm it.

The process is deliberately air-gapped: the system does not call the LinkedIn API or any external directory service to verify records. Instead, it displays the extracted information at the operator's terminal, and the operator uses their own personal browser to verify the information independently. The machine never touches the external network during verification.

A hard throttle of 10 verifications per day is enforced. This is not a technical limitation — it is an intentional design choice. The goal is to make verification a high-value ritual rather than a checkbox exercise. Ten perfect records per day produces 3,650 verified relationships per year with zero data corruption. A higher limit would produce more records but would invite fatigue-driven errors.

## service-search — The Inverted Index

The search service provides rapid retrieval across all files in the archive. It is built using Tantivy, a Rust-based search library, and implements an inverted index — the same underlying structure used by the index at the back of a textbook.

The index is static and binary, meaning it does not require a running database engine. The entire index can be copied to a USB drive and searched instantly on any machine. This is what the DARP (Data Access and Retention Protocol) compliance standard requires: data must be searchable without proprietary software.

## service-message-courier — Outbound Messaging

The message courier handles outbound communications — sending messages to external platforms on behalf of the organization. It uses a headless browser automation approach and is built on the Adapter Pattern: the core engine contains no hard-coded targets or credentials. All platform-specific logic is injected at runtime through scripts in a private adapters directory that is excluded from version control.

This design keeps client-specific operational data (credentials, target URLs) out of the public codebase while maintaining a single, generic execution engine.

---

# Part VII: The Operator Interface

## The Command Ledger

The operator's primary interface is the Sovereign Command Ledger — accessible at `console.woodfinegroup.com`. It is built as a Heads-Up Display (HUD): a window that routes data between the operator and the underlying archives, not a destination in itself.

The HUD exposes three layers of the Derivative Architecture:

- **Base Assets** (the raw, immutable originals — emails, PDFs)
- **First Derivative** (the processed, searchable knowledge graph)
- **Third Derivative** (generated outputs — drafts, reports, CSV exports)

The interface is designed around function keys. Each F-key activates a specific context within the system. This keyboard-first design is intentional: it is fast, does not require a mouse, and remains stable across backend upgrades.

## Zero-Form Architecture

Traditional web applications show you forms — boxes to fill in, dropdowns to select, buttons to click. The Command Ledger is designed to minimize this. The operator speaks to the system in natural English ("Find John from the plumbing company"), and the SLM silently translates that into a structured query against the knowledge graph. The result is not a form submission — it is a physical file (a CSV index card or a Markdown document) dropped to the operator's desktop.

This Zero-Form design eliminates the overhead of navigating through nested menus and filling in structured forms for every query. The operator works at the speed of language, not at the speed of a form wizard.

## F-Key Taxonomy

| F-Key | Name | What It Does | Output |
|---|---|---|---|
| F2 | People | Search and view personnel entities and contact records | CSV "Index Card" to desktop |
| F3 | Email | View cold-stored base email assets (the originals) | Display / export |
| F4 | Content Forge | Draft communications with SLM assistance. The SLM passively highlights alignment with active Themes and Archetypes | Markdown file to desktop |
| F8 | Network Command | PPN mesh management. Issue fleet commands. Broadcast network health checks. Isolate nodes | Real-time network action |
| F12 | Input Machine | The Anchor. Human-in-the-loop gateway for committing base assets to cold storage. Must manually select Destination Totebox, Service, and Chart of Accounts | Immutable ledger entry |

**Critical note on F12:** The Input Machine is the ground truth of the entire system. Nothing enters long-term storage without passing through F12. The SLM may suggest a Chart of Accounts classification, but the human operator must physically authorize the commit. This is the Fiduciary Anchor — the point where a human's judgment becomes the irreversible fact of record.

## Micro-Frontend Cartridge Architecture

The ConsoleOS is built as a strict two-part system (SYS-ADR-11):

**The Chassis** (os-console): An empty picture frame. It holds the global stylesheet, the F-key routing logic, and the Machine-Based Authorization parameters. It contains zero business data and zero application logic. It does not know what a PersonnelArchive is.

**The Cartridges** (app-console-*): Isolated HTML/JavaScript fragments, one per F-key function. Each cartridge is completely unaware of the broader system. When an operator presses an F-key, the Chassis dynamically fetches and mounts the corresponding cartridge into the viewport using the browser's native fetch() API.

This separation means that a change to the F4 Content Forge cartridge cannot break the F12 Input Machine cartridge. They do not share code. Each cartridge can be updated, replaced, or extended independently.

---

# Part VIII: Fleet Deployment

## Physical Infrastructure

Woodfine's fleet consists of three infrastructure tiers. On-premises hardware handles the most sensitive, control-plane functions. Cloud relays handle the operational workloads. Operator terminals are the user-facing endpoints.

| Fleet Directory | Role | Current Hardware |
|---|---|---|
| fleet-infrastructure-onprem | On-premises operator node. Local operator terminal | MacBook Pro (NODE-LAPTOP-A) — Provisioning |
| fleet-command-authority | Master routing node. Holds cryptographic keys. Command Authority | iMac 12.1 (NODE-IMAC-12) — Active |
| fleet-infrastructure-cloud | Cloud relay nodes. Operational workloads | GCP e2-micro (NODE-GCP-CORPORATE) — Active Testing |
| fleet-infrastructure-leased | Dedicated leased servers (future) | — |

## The PointSav Private Network (PPN) Topology

The Private Network is the encrypted mesh that connects all fleet nodes. It runs over WireGuard — an open-source VPN protocol. The key architectural decision (SYS-ADR-13) is that the master routing node and its cryptographic keys physically reside on the Executive's desk (the iMac Command Authority), not on a cloud server.

This means: if AWS disappears tomorrow, the organization retains physical custody of its network keys. The Command Authority dials OUT to the cloud relay — the public internet cannot dial IN to it.

```
  ┌────────────────────────────────────────────────────┐
  │  EXECUTIVE DESK (On-premises)                       │
  │  ┌─────────────────────────────┐                   │
  │  │  Node 3: iMac 12.1          │                   │
  │  │  fleet-command-authority    │                   │
  │  │  Type-II VM running         │                   │
  │  │  os-network-admin           │                   │
  │  │  Holds: cryptographic keys  │                   │
  │  │  Dials OUT to Node 2        │ ◄── No inbound    │
  │  └──────────────┬──────────────┘    from internet  │
  └─────────────────│──────────────────────────────────┘
                    │ WireGuard tunnel (outbound only)
                    │
  ┌─────────────────▼──────────────────────────────────┐
  │  CLOUD (GCP / AWS)                                  │
  │  ┌─────────────────────────────┐                   │
  │  │  Node 2: Cloud Relay        │                   │
  │  │  fleet-infrastructure-cloud │                   │
  │  │  NGINX Cloud Shield         │                   │
  │  │  cluster-totebox-corporate  │                   │
  │  └─────────────────────────────┘                   │
  └────────────────────────────────────────────────────┘
```

## Fleet Node Directory

The full fleet deployment directory maps to the following roles:

| Directory | Type | Purpose |
|---|---|---|
| cluster-totebox-corporate | ToteboxOS | CorporateArchive: financial records, minute books, ledgers |
| cluster-totebox-personnel-1 | ToteboxOS | PersonnelArchive: contacts, identity records |
| cluster-totebox-real-property | ToteboxOS | RealPropertyArchive: properties, permits, BIM |
| fleet-infrastructure-cloud | InfrastructureOS | Cloud compute nodes |
| fleet-infrastructure-leased | InfrastructureOS | Dedicated leased server nodes |
| fleet-infrastructure-onprem | InfrastructureOS | On-premises hardware nodes |
| gateway-interface-command | InterfaceOS | CommandCentre: aggregates archives for administration |
| media-knowledge-corporate | MediaKitOS | Corporate knowledge wiki |
| media-knowledge-distribution | MediaKitOS | Distribution / newsroom |
| media-knowledge-projects | MediaKitOS | Projects wiki |
| media-marketing-landing | MediaKitOS | Public-facing marketing site |
| node-console-operator | ConsoleOS | FKeysConsole operator terminal |
| route-network-admin | NetworkAdminOS | PPN management and MBA registry |
| vault-privategit-source | PrivateGitOS | Sovereign version control |

---

# Part IX: Operational Guides

## Cold Storage Sync — Pulling Telemetry

Website visitor data and fleet telemetry are collected by a zero-knowledge pipeline (no cookies, no session IDs, no third-party trackers) and stored in the cloud node. To pull this data to your local machine:

```bash
~/Foundry/pull_sovereign_telemetry.sh
```

This script executes a strict one-way transfer (a "pull diode") — data flows from the cloud to your local machine, never in the other direction. Reports are placed in the `outbox/` directory of the respective fleet node. A 9-day local retention cycle is enforced automatically.

A synthesis step must run first if you want human-readable Markdown reports rather than raw CSV data:

```bash
# Step 1: Trigger synthesis on the cloud node
tool-telemetry-synthesizer.sh

# Step 2: Pull synthesized reports to local machine
tool-telemetry-pull.sh
```

Reports are generated for the following time windows: 1 Day, 1 Week, 30 Days, 60 Days, 90 Days, Year-to-Date, and Inception-to-Date.

## SLM Point-in-Time Execution

The AI extraction service (`service-slm`) is never run as a persistent daemon. It is triggered once, processes its input, and terminates. This is the "point-in-time execution" standard:

```bash
# Correct: pipe input through the service and route output to the knowledge graph
service-slm < input_payload.txt >> service-content/knowledge-graph/

# Verify the process has terminated before proceeding
```

The service must be invoked via standard input (STDIN) and must route its output to the knowledge graph directory. Verify that the process has terminated before any downstream operations read from the knowledge graph.

## Personnel Ledger — Verification Surveyor

When service-people surfaces an unverified identity fragment (extracted from email), the Verification Surveyor workflow begins:

1. The system displays the extracted text at your terminal — name, company, contact details as the AI understood them.
2. Open your **personal browser** (not a company browser, not via any API) and locate the individual on LinkedIn or a corporate directory.
3. Verify their current employment status matches what was extracted.
4. Paste the verified profile URL back into the terminal.
5. The record is committed to the verified ledger.

The daily throttle is enforced by the system at 10 verifications per day. When the limit is reached, the system will not accept further verification inputs until the following day. This is intentional — do not attempt to bypass it.

## Sovereign Search

To search across all archived files:

1. Use the F3 (Email) or F2 (People) interface in the FKeysConsole.
2. Type your query in plain English: "Find the lease renewal letter from Apex Plumbing last spring."
3. The SLM translates your query into a search against the inverted index.
4. Results are displayed as a list of matching records. Select any record to export as a CSV Index Card to your desktop.

The search index operates without a running database engine. If you need to perform a search on an air-gapped machine (no network connection), copy the entire `service-search/` index directory to the target machine and run the search binary locally.

## Telemetry Operations — Daily Governance Extraction

To access the latest fleet telemetry from the primary Management Terminal:

```bash
~/Foundry/pull_sovereign_telemetry.sh
```

Reports manifest in the `outbox/` directories of the respective fleet nodes. The Rust telemetry daemon (`telemetry-daemon.rs`) handles backward compatibility automatically — if an older payload format arrives (from a cached client), it writes `unknown` or `0` to missing fields rather than failing.

## VPN Setup — F8 Network Command

The F8 interface in the FKeysConsole is the terminal for the PPN (PointSav Private Network). It supports natural-language commands:

**Example: Health Check**
- Type: `Check the health of the network.`
- The system proposes: `[PROPOSED] ACTION: PING, TARGET: ALL`
- Review the proposed action and click EXECUTE.
- All active nodes reply with CPU, RAM, and routing status.

**Example: Node Isolation**
- Type: `Lock down the laptop node immediately.`
- The system proposes: `[PROPOSED] ACTION: ISOLATE, TARGET: NODE-LAPTOP-A`
- Review and confirm.
- The target node drops all routing tables except the master link to the Command Authority.

Every F8 command follows the Two-Step Protocol: (1) submit your intent in plain English, (2) review the machine-translated action, (3) explicitly authorize execution. The system will never execute a network command without your visible confirmation.

**Technical note (SYS-ADR-16):** The F8 interface uses application-level unicast over WireGuard rather than UDP broadcast. This is because WireGuard operates as a point-to-point tunnel and cannot route broadcast addresses. The system maintains a list of known peer IP addresses and sends each command individually to each node.

## Physical Egress — Regulatory Printing

When printing official documents from the web interface for regulatory submission, enforce the following browser print configuration to achieve 1:1 parity with the official PDF format:

| Setting | Value |
|---|---|
| Orientation | Portrait |
| Headers/Footers | OFF |
| Margins | Default or exactly 0.5 in |
| Background Graphics | As needed per document |

The CSS print stylesheet handles the rest automatically — it hides the digital infrastructure block and ensures the contact block renders directly before the legal disclosures.

---

# Part X: Architecture Decisions

Architecture Decision Records (ADRs) are formal commitments to specific design choices. They are not proposals — they represent decisions already made and implemented. Each ADR includes the problem it solves and the rule it establishes.

## SYS-ADR-06: Immutable Ledgers vs. Self-Healing Intelligence

**The problem:** A compliance archive and an active intelligence system have incompatible requirements. A compliance archive must never change — its value as evidence depends on it being untouched. An active intelligence system must constantly update — its value depends on reflecting current reality.

**The decision:** These are two physically separate systems that serve completely different purposes.

- `service-email` is the **Compliance Layer**: an immutable WORM (Write Once, Read Many) archive of every raw email received. It is a legal record. It is mathematically forbidden from modification.
- `service-content` is the **Intelligence Layer**: a self-healing knowledge graph that continuously updates as new information arrives. Old facts are replaced with current truths to give the AI a clean, accurate picture of the present.
- `service-slm` is the **Bridge**: it reads the compliance layer, extracts the intelligence, and writes exclusively to the intelligence layer. The bridge only flows in one direction.

**The implication:** Never query the compliance archive for intelligence purposes. It will return noise. Never expect the intelligence layer to serve as a legal record. It will not be immutable.

## SYS-ADR-07: Bifurcated Ingestion (Two Roads for Data)

**The problem:** Applying an AI model to every piece of incoming data is wasteful and introduces errors. A spreadsheet with financial figures should be parsed with a deterministic algorithm that guarantees 100% accuracy. Applying an AI to it introduces a small but unacceptable probability of misreading a number.

**The decision:** Every inbound payload is classified before processing. The classification determines which engine handles it.

- **Path A (Deterministic):** Structured data (CSV, JSON, XLSX, telemetry logs) goes to dedicated Tier-5 services using standard parsing algorithms. The AI is forbidden from this path.
- **Path B (Probabilistic):** Unstructured human text (email body, PDF, Word documents) goes to `service-slm` for semantic extraction.

**The implication:** Before processing any inbound data, determine its type. Do not route structured data through the AI. Do not apply rigid parsing rules to human text.

## SYS-ADR-08: Systemd Quarantine

**The problem:** The cloud relay servers run Debian Linux, which is permanently entangled with `systemd` — a Linux process supervisor that has absorbed network management, logging, and other responsibilities into a 1.5-million-line monolith. This conflicts with the microkernel isolation principle.

**The decision:** `systemd` is formally classified as a Quarantined Foreign Component — a piece of technical debt that is accepted temporarily. It is used for the 5-second automated ingestion loops because removing it from Debian would cause a system failure. It is not used for anything beyond basic process supervision.

**The implication:** Do not add new `systemd` integrations. Do not rely on `systemd` for any function beyond basic process management. When the cloud relays are migrated to a FreeBSD substrate or native seL4, `systemd` will be replaced with minimalist text-based supervision (rc.d or runit).

## SYS-ADR-10: The Fiduciary Anchor (F12 Input Machine)

**The problem:** Fully automated AI categorization of documents is a fiduciary liability. If an AI autonomously assigns a source document to the wrong account in the Chart of Accounts, every downstream calculation, report, and compliance record built on that document is wrong. By the time the error is discovered, it may have propagated through years of records.

**The decision:** The F12 Input Machine is the mandatory human checkpoint for all base asset ingestion. The human operator must manually select three things before any document enters cold storage: (1) the destination ToteboxArchive, (2) the Totebox Service, and (3) the Chart of Accounts category.

The SLM may silently suggest a classification while the operator reviews the document. It is strictly forbidden from executing the commit. The human must physically authorize the ledger entry.

**The implication:** There is no batch import path that bypasses F12. Every base asset enters the system through a human decision point.

## SYS-ADR-11: Micro-Frontend Cartridge Architecture

**The problem:** A monolithic user interface becomes increasingly fragile as it grows. A bug in one feature can break an unrelated feature. Deploying a small update requires re-deploying and re-testing the entire interface.

**The decision:** The ConsoleOS is split into two physically separate components:

- **The Chassis** (`os-console`): A static shell with no business logic. It holds the layout, routing rules, and authentication parameters. When an operator presses an F-key, the Chassis fetches the corresponding Cartridge.
- **The Cartridges** (`app-console-*`): Isolated fragments of HTML and JavaScript. Each Cartridge is self-contained and knows nothing about the broader system. The Chassis mounts them on demand using the native browser `fetch()` API.

**The implication:** Each F-key function is a separate deployable unit. Updating F4 does not affect F12. Adding a new F-key function means creating a new Cartridge, not modifying the Chassis. Each Cartridge must be completely self-contained — no shared state with other Cartridges.

---

# Part XI: The Road Ahead — Moonshots

## The Philosophy

Every third-party dependency is a liability. Not a theoretical liability — a practical one. If Microsoft changes its Graph API, `service-email` breaks. If AWS deprecates a service, the infrastructure must be rebuilt. If a hardware vendor's driver stops working, the system stops working.

The Sovereign Replacement Initiative is the ongoing program to eliminate these dependencies one by one. Each dependency is quarantined, tracked, and replaced with a native alternative when that alternative reaches functional parity.

## Technical Debt Ledger

| Quarantined Dependency | Directory | Replacement Target | Moonshot Directory |
|---|---|---|---|
| Microsoft Graph API (email) | vendor-microsoft-graph | Native SMTP/IMAP layer in Rust | moonshot-message-transport |
| Azure Auth (authentication) | vendor-azure-auth | Machine-Based Authorization (MBA) native layer | moonshot-auth |
| systemd (process supervision) | (system-wide, Debian) | rc.d or runit on FreeBSD / seL4 | moonshot-kernel |
| Cloud SQL / PostgreSQL | vendor-database | Flat-file state machine (partial parity achieved) | moonshot-database |
| Docker container runtime | vendor-containers | Unikernel bootable disk images (ToteboxOS end state) | moonshot-unikernel |

## The Moonshot Pipeline

For each quarantined dependency, an engineering initiative is underway in the corresponding `moonshot-*` directory. These are real engineering projects — not aspirational notes. When a Moonshot component achieves structural parity with the component it is replacing, it physically replaces the quarantined vendor code.

The Sovereign Data Foundation in Denmark audits this pipeline to verify that the organization is on a credible trajectory toward operational independence.

## Where the Architecture Is Going

The end state is a platform where:

1. Every component runs as its own kernel-isolated virtual machine.
2. Every archive is exportable as a Bootable Disk Image — portable, proprietary-free, and bootable on any hypervisor.
3. Authentication is entirely machine-based — no passwords, no credential databases, no social engineering vectors.
4. Every software dependency is either owned outright or replaceable without rewriting business logic.
5. The data pipeline operates entirely within the organization's hardware perimeter — AI assistance is local, external models receive only anonymized structural skeletons.

None of this requires over-engineering today. It requires awareness: choosing an interface boundary over a direct function call, a configuration file over a hardcoded connection string, an open file format over a proprietary one. Each of these is a low-cost choice now that keeps the path to the end state clear.

---

# Appendix: Glossary

| Term | Plain-Language Definition |
|---|---|
| Archive Preset | One of three standard ToteboxArchive configurations: PersonnelArchive, CorporateArchive, or RealPropertyArchive. Each is pre-loaded with the right databases and services for its asset type. |
| Assignment | What PointSav calls a task or work ticket. Assignments must compile and function before they are considered complete. |
| Base Asset | The raw, immutable original file — an email (.eml), a contract (.pdf), a spreadsheet (.xlsx). Stored in cold storage via F12. Cannot be altered after entry. |
| BIM | Building Information Modelling. A digital 3D representation of a physical building that includes structural, mechanical, and spatial data. |
| Bootable Disk Image | A VM image file that contains a complete operating system and can be started on any compatible hypervisor without additional software. The end-state artifact for Freely Transferable archives. |
| Capability-Based Security | A security model where each software component must hold a cryptographic token (a "capability") to communicate with any other component. No capability means no communication, regardless of network access. |
| Chart of Accounts | A structured list of financial and operational categories used to classify every document and transaction. Requires Executive override to change (update rate: 18–24 months). |
| CommandCentre | The primary InterfaceOS variant. Aggregates PersonnelArchives and CorporateArchives for administrative use. |
| ConsoleOS | The user-facing delivery layer. Each variant is a purpose-built interface for a specific workflow. |
| Contributor | A PointSav developer or designer. Individual contributors, not agency teams. |
| CorporateArchive | A ToteboxArchive for a legal entity. Anchored to a Business Incorporation Number or Tax ID. Contains financial records, minute books, and statutory ledgers. |
| CostingEmail | What PointSav calls a statement of work or work order. The commercial document that defines an assignment. |
| Customer | An organization that pays for industrial-scale Totebox Orchestration using proprietary InterfaceOS components. |
| DARP | Data Access and Retention Protocol. The compliance standard governing how data must be stored, accessed, and retained. Requires data to be searchable without proprietary software. |
| Derivative Architecture | The three-tier data model: Base Assets (ground truth) → First Derivative (indexed knowledge) → Third Derivative (generated outputs). |
| DiodeStandard | The one-way command flow principle. Data moves in one direction only: from source to destination. No reverse channel. Applies to both the F8 telemetry pull and the overall security architecture. |
| DARP | Data Access and Retention Protocol. The compliance standard requiring that data be readable and searchable without proprietary software. |
| F-Key | A hardware function key on the keyboard that activates a specific context in the FKeysConsole. F2 = People, F3 = Email, F4 = Content, F8 = Network, F12 = Input Machine. |
| FKeysConsole | The primary ConsoleOS variant for administrative work. A keyboard-driven terminal where each function key activates a different layer of the Derivative Architecture. |
| First Derivative | The processed, self-healing knowledge graph derived from Base Assets by service-slm. Contains the organization's current operational reality in a clean, queryable form. |
| Freely Transferable | The non-negotiable standard that every ToteboxArchive must be exportable as a complete, self-contained package deployable without proprietary runtimes or vendor relationships. |
| GeneralStaff | The Woodfine employees who use the platform for daily operations. |
| Geometric Security | Peter Woodfine's term for the Machine-Based Authorization model. The topology of the network defines the access control. |
| GIS | Geographic Information System. Software for capturing, storing, and analyzing spatial and geographic data. |
| InfrastructureOS | The virtualization substrate. The operating system that physical hardware runs. Provides the environment for all other components. |
| InterfaceOS | The stateless logic layer. Holds no data. Aggregates data from multiple ToteboxArchives for ConsoleOS users. Required for multi-archive use cases (the monetization boundary). |
| Inverted Index | A search structure where each word in a corpus maps to a list of files containing that word — like the index at the back of a textbook. Enables microsecond search without a running database. |
| IoTConnect | An InterfaceOS variant that bridges IoT sensor data into the ToteboxArchive system. |
| IssuerSnap | See: TheSnap. |
| Machine-Based Authorization (MBA) | A security model where access is granted through cryptographic hardware pairing between machines, not through usernames and passwords. |
| MediaKitOS | The web framework and CMS layer for public-facing web properties and Reporting Issuers' disclosure obligations. |
| Minute Book | The corporate governance record required by law for any incorporated entity. Records resolutions, meetings, and structural changes of the corporation. |
| Moonshot | An engineering initiative to build a native replacement for a quarantined third-party dependency. Named `moonshot-*` in the codebase. Prohibited from production until parity is achieved. |
| NetworkAdminOS | The management layer for the Private Network. Maintains the MBA registry, monitors node health, and provides fleet control interfaces. |
| Operator | Someone who manages a ToteboxOrchestration at the infrastructure level. Distinct from an Owner (who owns the archives). |
| Owner | The entity that owns the ToteboxArchives. Distinct from the Operator who administers them. |
| PairingAsPermission | The core security principle: access is determined by which machines are cryptographically paired to which, not by user credentials or role assignments. |
| PersonnelArchive | A ToteboxArchive for an individual person. Anchored to a SIN (Social Insurance Number) or Passport ID. Contains professional network records, identity data, and communication history. |
| PointSav Private Network (PPN) | The encrypted mesh network connecting all fleet nodes. Runs over WireGuard. The master routing node resides on the Executive's physical desk. |
| PrivateGitOS | The sovereign version control system. Preferred on-premises for physical IP possession. |
| RAG | Retrieval-Augmented Generation. An AI technique where a model is given relevant context documents before generating a response, rather than relying solely on its training data. |
| RealPropertyArchive | A ToteboxArchive for a physical property. Anchored to a Land Title PIN or legal address. Contains permits, lifecycle records, BIM drawings, IoT data, and maintenance history. |
| Reporting Issuer | A publicly-traded limited partnership that is required by securities law to disclose financial and operational information to investors on a regular schedule. Woodfine LPs are Reporting Issuers. |
| seL4 | A mathematically proven microkernel. The seL4 security properties have been formally verified using machine-checked mathematical proofs — not just tested but proven. Reference point for the ToteboxOS kernel. |
| ServiceProvider | External professionals (lawyers, accountants, architects, trades) who interact with the organization. |
| SLM | Small Language Model. A compact AI model (sub-one-billion parameters) that runs entirely on the organization's own hardware. Used for text extraction and AI routing. Operates as a point-in-time process that terminates after execution. |
| TheSnap | The cross-archive integrity verification mechanism. Pulls verified data from Corporate and Real Property Archives, cross-references against Personnel Archives, and generates authenticated quarterly reports. Used for Reporting Issuer compliance. |
| Third Derivative | Generated outputs synthesized from the First Derivative: drafts, memos, reports, CSV exports for data markets. |
| ToteboxArchive | A self-contained data store for one specific asset. Contains both the data and the services that process it. The unit of ownership, backup, and export. |
| ToteboxOrchestration | A complete, interconnected deployment of all platform components — ToteboxOS archives, InterfaceOS hubs, ConsoleOS terminals, and NetworkAdminOS. |
| ToteboxOS | The operating system of a ToteboxArchive. In the end state, a unikernel running as a kernel-isolated virtual machine. |
| Unikernel | A minimal operating system containing only the kernel, the libraries, and the application code needed for one specific purpose. No shell, no package manager, no multi-user support. Fast to boot, tiny attack surface. |
| Vendor-* | Quarantined third-party dependencies. Isolated in their own directories. Tracked as technical debt. |
| Verification Surveyor | The human-in-the-loop checkpoint for identity records. Operators verify extracted person records using their own browser. Throttled to 10 verifications per day to ensure quality. |
| WorkplaceOS | A Linux-based desktop environment for General Staff. Separate from Totebox Orchestration. The alternative delivery path for users who prefer a full desktop OS over running ConsoleOS as a virtual machine. |
| WORM | Write Once, Read Many. A data storage principle where records cannot be modified after they are written. The compliance archive (`service-email`) is WORM. |
| WireGuard | An open-source VPN protocol used to create the encrypted tunnels of the PointSav Private Network. Operates as a Layer-3 point-to-point tunnel. |
| WoodfineLP | A limited partnership within the Woodfine Capital Projects network. These are the Reporting Issuers subject to securities disclosure requirements. |

---

*End of User Guide*
