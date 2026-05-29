---
artifact: brief
status: active
---

# PointSav Products and Services — Research Archive Framework
**Source:** 245 development research emails Dec 2025–Apr 2026 (Mathew → Jennifer for the "PointSav Documentation Wiki GIT")
**Generated:** 2026-05-14 (command@claude-code)
**Status:** Internal synthesis. Internal use only — full language + BCSC review required before any public use.

---

## 0. Reading guide

The email archive is a chronological design journal. Concepts evolved rapidly. Naming was unstable for the first three months and stabilised in mid-March 2026. Two distinct naming generations are present:

| Generation | Date band | Naming style | Status |
|---|---|---|---|
| **G1 — TitleCase / mixed** | 2025-12 to 2026-02-06 | `Totebox OS`, `Console OS`, `Interface OS`, `MediaKit OS`, `Workplace OS`, `PrivateGit OS`, `Infrastructure OS`, `Network Admin OS` | [SUPERSEDED BY: G2 kebab-case] |
| **G2 — strict kebab-case** | 2026-02-07 onward (locked email #083) | `totebox-os`, `console-os`, `interface-os`, etc.; `package-*`, `app-*`, `service-*`, `asset-*` prefixes | Canonical for the archive |
| **G3 — `os-` / `app-[os]-` prefixed** | 2026-02-15 onward (email #140) | `os-totebox`, `os-console`, `os-interface`, `os-infrastructure`, `os-network-admin`, `os-mediakit`, `os-privategit`, `os-workplace`; `app-console-bim`, `app-totebox-corporate`, `app-console-input`, `app-console-people`, `app-console-email`, `app-console-content` | Aligns to current canonical |

Where this document references a product, it uses the **current canonical name** (per CLAUDE.md and operator confirmation in this task) and flags older variants with `[SUPERSEDED BY: <canonical>]`.

**Current canonical names confirmed by operator:**

| Family | Canonical | Older variants (all superseded) |
|---|---|---|
| OS family | `os-console`, `os-infrastructure`, `os-interface`, `os-mediakit`, `os-network-admin`, `os-privategit`, `os-totebox`, `os-workplace` | `Totebox OS`, `Console OS`, `Interface OS`, `MediaKit OS`, `Workplace OS`, `PrivateGit OS`, `Infrastructure OS`, `Network Admin OS`, `mediakit-os`, `totebox-os`, `console-os`, `interface-os`, `workplace-os`, `privategit-os`, `infrastructure-os`, `network-admin-os` |
| Interface family rename | `os-orchestration` (per task brief) | `os-interface`, `interface-os`, `Interface OS` |
| Apps | `app-console-*`, `app-mediakit-*`, `app-network-*`, `app-orchestration-*`, `app-totebox-*`, `app-workplace-*` | `app-marketing`, `app-knowledge`, `app-distribution`, `app-people`, `app-email`, `app-pdfs`, `app-spreadsheet`, `app-wordprocessor`, `app-browser`, `app-chat`, `app-communications`, `app-file-manager`, `app-bim`, `app-gis`, `app-wiki`, `console-people`, `console-email`, `node-console-*`, `pointsav-cli` |
| PPN infrastructure family (current) | `app-infrastructure-onprem`, `app-infrastructure-leased`, `app-infrastructure-cloud`, `app-network-admin` | `fleet-infrastructure-onprem`, `fleet-infrastructure-leased`, `fleet-infrastructure-gcp`, `route-network-admin`, `gateway-interface-onprem`, `gateway-interface-leased`, `gateway-interface-gcp`, `gateway-interface-command` |
| Services | `service-slm`, `service-content`, `service-extraction` | `service-llm`, `system-slm`, `moonshot-semantic-router`, `vendor-google-functiongemma-270m`, `service-LLM` |
| Three orgs | PointSav Digital Systems (vendor) · Woodfine Management Corp. (customer) · Woodfine Capital Projects Inc. (parent) | `PointSav Digital Systems AG`, `MCorp`, `Woodfine` |
| Three-layer architecture | Layer 1 SOFTWARE = `pointsav-monorepo`; Layer 2 SHOWCASE = `woodfine-fleet-deployment`; Layer 3 INSTANCES = `deployments/` (PRIVATE) | `~/Developer`, `~/Employee`, `~/Production`, "Forge / Audit / Customer / Truth" horseshoe, `pwoodfine/Console-OS`, contributor forks, `woodfine-codex`, `pointsav-codex` |

---

## 1. Product catalog

| Name (canonical) | Purpose | Target user | Tech stack | Layer | Status | Stale variants in emails |
|---|---|---|---|---|---|---|
| `os-totebox` | Sovereign data vault + service host; per-user/per-entity archive that holds People / Email / Content / Minutebook / Bookkeeping ledgers; isolated, freely transferable, WORM-disciplined | Customer SMB, family office, public-company issuer, regulated SMB | Rust + seL4 microkernel (microkit / Microvisor); aspirational Unikernel form (NanoVMs/Unikraft); FreeBSD jail interim form for v1 | Layer 1 (vendor source) → Layer 2 (customer GUIDE) → Layer 3 (deployments) | Active research/prototype | `Totebox OS`, `totebox-os`, `Totebox Unikernel`, `Totebox Archive`, `SB - Totebox OS` |
| `os-console` | Universal human-facing terminal / TUI / "Command Ledger"; Asset Rendering Terminal; runs as VMM on Windows/Mac/Linux host (WHPX / Hypervisor.framework / KVM); single-archive view | Operator (employee, owner, fiduciary) | Rust + WGPU + custom SDF text engine (`pointsav-window`/`pointsav-gpu`/`pointsav-text`/`pointsav-layout`); F-key driven (F1 HELP / F2 PEOPLE / F3 EMAIL / F4 CONTENT / F5 MINUTEBOOK / F6 BOOKKEEPER / F12 INPUT) | Layer 1 | Active design | `Console OS`, `console-os`, `Console-OS`, `node-console-*` |
| `os-orchestration` | Multi-archive aggregator; fleet/portfolio view; commercial paid tier; hub-and-spoke aggregator that pulls metadata from many `os-totebox` instances | Enterprise customer with >1 Totebox; corporate command centre | Rust; commercial proprietary; runs commercial-only `app-orchestration-*` surfaces (multi-tenant aggregation, complex viewports) | Layer 1 | Active design | `Interface OS`, `interface-os`, `os-interface`, `Interface-OS`, `Aggregator`, "Federation Hub" |
| `os-infrastructure` | Bare-metal / VM provisioning substrate; the hardware-agnostic engine that hosts other OSs; runs on-prem, leased, cloud, with Genesis Protocol (N=1 self-bootstrap) | PointSav vendor + customer fleet admin | Rust + seL4; WireGuard mesh built-in | Layer 1 | Active research | `Infrastructure OS`, `infrastructure-os` |
| `os-network-admin` | The control plane / routing authority for the PPN (PointSav Private Network); manages MBA (Machine-Based Authorization) registry, mesh health, Diode rules | Network architect (Woodfine HQ / customer admin) | Rust + WireGuard 10.50.0.x mesh; F8 Terminal semantic interface (driven by `service-slm`) | Layer 1 | Active research | `Network Admin OS`, `network-admin-os`, `route-network-admin` |
| `os-mediakit` | Public/internal web presence appliance; "Sovereign Compliance Appliance" for Reporting Issuers; bundles three commercial-style web engines as MicroVMs | Public companies (Reporting Issuers), SMB hosting marketing+wiki+newsroom, customer multi-deployment | seL4 Microvisor host + FreeBSD jails / Linux MicroVMs for guests; WordPress engine (Marketing), MediaWiki engine (Knowledge), FreshRSS engine (Distribution); Rust `pointsav-protocol` Diode for federation | Layer 1 | Active research | `MediaKit OS`, `MediaKit-OS`, `mediakit-os`, `sys-marketing`, `sys-knowledge`, `sys-distro`, `app-marketing`, `app-knowledge`, `app-distribution` |
| `os-privategit` | Sovereign source control + design system hosting; replaces third-party Git for IP + brand assets | PointSav internal + customer | Gitea-based source control; Storybook/custom design system | Layer 1 | Active research | `PrivateGit OS`, `privategit-os`, `privategit-source-control`, `privategit-design-system` |
| `os-workplace` | Sovereign desktop / FOSS bare-metal interface; "Gateway Drug" desktop that pairs with `os-totebox` | Community + SMB customer | FreeBSD-hardened / seL4 (phase 2); native Rust apps (Servo / Himalaya / IronCalc / Typst / Whitebox-tools / Ifc-rs / Iced or Slint UI) | Layer 1 | Active research | `Workplace OS`, `workplace-os`, `workplace-desktop`, `workplace-mobile` |
| `app-totebox-*` (suite of cartridges) | Specialised configurations of `os-totebox` for different use cases (Personnel / Corporate / Real Property / Personal) | Customer | Rust | Layer 1 | Conceptual | `app-totebox-corporate`, `app-totebox-personal`, `cluster-totebox-personnel`, `cluster-totebox-corporate`, `cluster-totebox-real-property`, "Personnel Archive", "Corporate Archive", "Real Property Archive" |
| `app-console-input` (a.k.a. F12 INPUT MACHINE) | Human-in-the-loop ingestion gateway; drag-and-drop UI that strips execution permissions and routes files into `service-minutebook` or `service-bookkeeper`; the "Fiduciary Anchor" | Operator | Rust TUI | Layer 1 | Active design (F12 is a hard rule per SYS-ADR-10) | `app-input`, "Input Machine", "Verification Surveyor" |
| `app-console-people` | TUI for managing the F2 People ledger (contacts, identity claims, relationship graph) | Operator | Rust TUI | Layer 1 | Conceptual | `Console OS — People`, `app-people` |
| `app-console-email` | F3 Email viewer; "Comm Diode"; sanitised text-only inbox view | Operator | Rust TUI | Layer 1 | Conceptual | `Console OS [Email Server]`, `app-email` |
| `app-console-content` | F4 Content drafting / synthesis surface; the Verification UI for L5 truth ledger | Operator | Rust TUI | Layer 1 | Conceptual | `app-content`, "Content Console" |
| `app-console-bim` | F-key surface for BIM (Building Information Modelling) review and authoring | Operator (real estate / construction) | Rust TUI + truck B-rep kernel + Ifc-rs | Layer 1 | Future / aspirational | `app-bim` |
| `app-console-bookkeeper` | F6 Bookkeeper surface; spreadsheet widgets + ledger logic | Operator (finance) | Rust TUI + IronCalc | Layer 1 | Conceptual | `Console OS — Ledger`, `app-spreadsheet` |
| `app-mediakit-marketing` | WordPress-derived public website engine (landing pages, IR pages) | Public company / SMB | WordPress + Rust static cache | Layer 1 | Active research | `app-marketing`, `mediakit-marketing`, `sys-marketing`, `app-wordpress-mutation` |
| `app-mediakit-knowledge` | MediaWiki-derived structured-knowledge engine (corporate wiki, project wiki, documentation wiki) | Public company / SMB / customer | MediaWiki + Rust vector cache | Layer 1 | Active research | `app-knowledge`, `mediakit-knowledge`, `sys-knowledge`, `app-mediawiki-mutation` |
| `app-mediakit-distribution` | FreshRSS-derived push/aggregator engine ("Compliance Ledger / Newsroom") | Reporting Issuer | FreshRSS + Rust crawler + ActivityPub | Layer 1 | Active research | `app-distribution`, `mediakit-distribution`, `sys-distro`, `app-freshrss-mutation` |
| `app-orchestration-command` (a.k.a. command-centre) | The user-facing aggregator that pulls fleet metadata; primary `os-orchestration` surface | Enterprise admin | Rust | Layer 1 | Active design | `gateway-interface-command`, `gateway-interface-command-centre`, "Command Centre" |
| `app-network-admin` | The os-network-admin user surface; routing tables, Diode policy, MBA registry | Fleet admin | Rust + F8 Terminal | Layer 1 | Active research | `infra-network-admin`, `route-network-admin` |
| `app-infrastructure-onprem` | App surface for an on-premise `os-infrastructure` node | Fleet admin / customer | Rust | Layer 1 | Active research | `gateway-interface-onprem`, `fleet-infrastructure-onprem`, "on-prem node" |
| `app-infrastructure-leased` | App surface for a leased / colocated bare-metal node (Equinix-style) | Fleet admin | Rust | Layer 1 | Active research | `gateway-interface-leased`, `fleet-infrastructure-leased`, "leased server" |
| `app-infrastructure-cloud` | App surface for a hyperscaler `os-infrastructure` node (GCP, AWS, Azure) | Fleet admin | Rust | Layer 1 | Active research | `gateway-interface-gcp`, `fleet-infrastructure-gcp`, "cloud edge node" |
| `app-workplace-*` (suite) | Native desktop applications on `os-workplace` (PDFs / word processing / spreadsheets / email / browser / chat / file manager / wiki) | Community + SMB | Rust forks: pdf-rs, Typst, IronCalc, Himalaya, Servo, Broot/Xplr | Layer 1 | Conceptual | `app-pdfs`, `app-wordprocessor`, `app-spreadsheet`, `app-email`, `app-browser`, `app-chat`, `app-communications`, `app-file-manager`, `app-wiki` |

---

## 2. Service catalog

| Name (canonical) | Purpose | Inputs | Outputs | Status | Notes / stale variants |
|---|---|---|---|---|---|
| `service-slm` | The institutional Small Language Model service; lives behind the Doorman audit boundary; provides three compute tiers (Local / Yo-Yo / External API); acts as semantic command parser, taxonomy gardener, gravity engine, and translator; NOT a chat UI ("invisible AI") | Local prompts from `os-console`, F8 Terminal, gravity-engine vectors from `service-content`, entity lists from `service-people` | Routed responses, VALID/REJECT tokens, semantic UDP commands, Chart-of-Accounts socket assignments, theme suggestions | Active build | OLMo-2-0425-1B-Instruct or Olmo-3-1125-7B-Think-Q4_K_M [per memory: never Qwen]; older email names: `service-LLM` [SUPERSEDED], `service-llm` [SUPERSEDED], `system-slm` [SUPERSEDED], `moonshot-semantic-router` [SUPERSEDED], `vendor-google-functiongemma-270m` [SUPERSEDED] |
| `service-content` | The Gravity Engine; reads raw payloads from `service-email` (or other ingest), runs Aho-Corasick keyword extraction against the four-pillar Seed Vault (Archetypes / Chart of Accounts / Domains / Themes), produces a "50-Word Gravity Vector", routes verified output to `service-slm` for VALID/REJECT decision; also generates HTML wikis, PDFs, news releases via the D1/D2/D3 derivative engines (Thematic Quant / Linguistic Protocols / Content Creation) | WORM payloads, Seed Vault JSONs, Domain glossaries, Themes, Chart of Accounts | Gravity vectors, generated content (wikis, news, briefs), thematic-density signals | Active build | Lives in `pointsav-monorepo`; runs in a GCP VM with Gemini API in early prototypes; self-healing — output added back to input |
| `service-extraction` | Entity-mass extraction from raw payloads; "Infinite Net" — Aho-Corasick regex finds every Name / Email / Phone / Company; assigns deterministic UUIDv5; places entities in `Discovery` status until they accrue gravity | Raw email bodies, PDFs, DOCX from WORM vault | Sovereign-ID entities; bundles for `service-content` and `service-slm` | Active build | Implements ACS (Anchor / Claim / Socket) model; replaces older `node_discovery.py`, `node_maturated.py` from the Sovereign-Talent-Engine prototype |
| `service-people` | Sovereign identity / relationship substrate; SQLite deterministic ledger; maps raw contacts → Sovereign IDs; the F2 People ledger; ingests LinkedIn `Connections.csv`, MSFT 365, spreadsheets | CSV imports, email-extracted entities, manual `app-console-people` input | `people.db` (or successor ledger format); cross-archive entity graph; Chart-of-Accounts socket assignments | Active prototype | `Sovereign-Talent-Engine v1.1`, `people.db`, `service-people-db` are older names. Uses 20-point Digital Twin scoring, 11 Archetypes, Chart of Accounts |
| `service-email` | Sovereign email server + WORM ingest; SMTP/IMAP listener; sanitises HTML / tracking pixels; writes raw payload to immutable Maildir; the "Diode" boundary for inbound mail | M365 OAuth2 (Microsoft Graph), Exchange Online, SMTP/IMAP | Maildir-on-prem, JSONL audit log, `app-console-email` view | Active prototype | OAuth2 client-credentials flow via Microsoft Entra; older names: `SB - Email Server`, `service-email-server`, `app-email-server` |
| `service-minutebook` | Deep record archive; immutable `.docx`, `.pdf`, `.xlsx` storage with cryptographic checksums; holds corporate resolutions, MRI results, signed court filings; F5 surface in `os-console` | Files via `app-console-input` (F12) | Tamper-evident filed record with `.csv`/`.yaml` ledger pointer; isolated `/service-minutebook` partition | Conceptual | New service. Partitioned per-service on disk for audit isolation. |
| `service-bookkeeper` | Financial Ledger; F6 surface; deterministic accounting; the Chart of Accounts data plane | Files via `app-console-input` (F12); spreadsheet imports | `.csv` / `.yaml` ledger entries; deterministic state | Conceptual | Older name `service-bookkeeping` is variant; operator Jennifer is the bookkeeping practice operator per project memory |
| `service-fs` (unikernel file-system service) | Sovereign block-storage unikernel; the only service that touches raw disk; isolates filesystem from all other services via seL4 IPC | seL4 capabilities, IPC messages from other services | Append-only block writes; mathematical filesystem isolation | Future / aspirational | Email #241 architecture: "Multi-Server Microkernel" / "Sovereign Unikernel Matrix" / "PointSav Cell Architecture" |
| `service-pairing` (a.k.a. MBA Handshake) | Hardware-bound trust engine; manages cryptographic pairings between machines (Console ↔ Totebox ↔ Orchestration); replaces all usernames/passwords with public-key pairs | Hardware attestation, pre-shared keys | Pair tokens, MBA registry entries | Active research | Older names: `service-auth` [SUPERSEDED — Kanidm was rejected], `mba_handshake.py`. Uses Noise Protocol / WireGuard-style keys |
| `service-printing` | Hardened print-gate daemon; security-wrapper around CUPS preventing upstream network exploits | Print jobs from any app surface | One-way print payload | Conceptual | "pointsav-print-gate" wrapper |
| `service-udp` | Pure-Rust zero-broker UDP mesh; binary-intent routing across the PPN (port 8090); replaces NATS/Kafka | F8 Terminal commands, telemetry | Binary UDP packets across 10.50.0.x WireGuard mesh | Active research | The "Sovereign Mesh" / "Brokerless UDP Gossip Protocol" |
| `service-audit` | Immutable append-only SOC-3 ledger living at the system-* tier; physically un-disable-able audit trail; embedded in Capability-Based Manager | Capability routing events | seL4 microkernel log entries | Conceptual | Email #162; complements `service-resolution` for DARP |
| `service-resolution` | Cryptographic packager for DARP (Digital Asset Resolution Package); the "self-executing parachute" that cleanly severs a Totebox and transfers it to its owner on vendor failure | Totebox boundary events | Signed export packages | Conceptual | Email #162; satisfies Swiss FINMA-style DARP standard |
| `service-totebox-parser` | Outbound marketing/extraction engine; ingests `.jsonl` ledgers from `service-totebox-egress`; runs slow-drip campaigns | WORM `.jsonl` ledgers, M365 contacts | Tactical campaign payloads | Built (March 2026) | Email #193; lives in `pointsav-monorepo/service-totebox-parser/` |
| `service-pointsav-link` (a.k.a. `pointsav-protocol`) | Detachable Diode adapter; the "Smart Valve" service-VM that enforces unidirectional command flow between `os-orchestration` and other OSes | Console downstream commands; upstream telemetry | Sanitised downstream / blocked upstream | Conceptual | Older variants: `service-pointsav-link`, `pkg-pointsav-protocol`, `pointsav-link`, "the Diode" |

---

## 3. OS family

| OS name (canonical) | Role | Host type | Delivers | Status | Old variants |
|---|---|---|---|---|---|
| `os-console` | Human terminal / "Asset Rendering Terminal" / "Command Ledger" | Runs as VMM on Windows / macOS / Linux host (or bare) | F-key driven single-archive view of a Totebox; 70/25/5 CLI/GUI/visual; ~15 MB binary, <50ms cold start | Active design | Console OS, console-os |
| `os-totebox` | Sovereign vault + service host | Bare metal, leased server, cloud VM (long-term: unikernel) | Per-entity data archive running the F1–F6 services internally | Active prototype | Totebox OS, totebox-os, Totebox Archive, Totebox Node |
| `os-orchestration` | Fleet aggregator | Cloud VM / on-prem server | Multi-archive metadata aggregation; runs `app-orchestration-*` commercial apps | Active design | Interface OS, interface-os, os-interface |
| `os-infrastructure` | Compute substrate | Any bare metal / VM | Hosts other OSs via seL4 + Genesis Protocol; Three deployments: on-prem, leased, cloud | Active research | Infrastructure OS, infrastructure-os |
| `os-network-admin` | Network control plane | Bare metal / VM at customer HQ | MBA registry, Diode policy, WireGuard mesh routing, F8 Terminal | Active research | Network Admin OS, network-admin-os |
| `os-mediakit` | Public web appliance | Atomic 1:1 VM per workload | Hosts `app-mediakit-marketing` / `-knowledge` / `-distribution` | Active research | MediaKit OS, mediakit-os |
| `os-privategit` | Sovereign source / design hosting | On-prem or cloud VM | `pointsav-monorepo`, design system, brand assets | Active research | PrivateGit OS, privategit-os |
| `os-workplace` | Sovereign desktop | Dell XPS 13/14 (Developer Edition), HP ProBook 400 series, iMac 12.1 (legacy reference) | FOSS desktop + native Rust apps; community gateway-drug into the ecosystem | Active research | Workplace OS, workplace-os |

---

## 4. Architecture decisions timeline

| Decision | Date range from emails | Current canonical state | Superseded? |
|---|---|---|---|
| Kernel choice: build vs. adopt | 2026-01-28 to 2026-02-03 | seL4 microkernel ADOPTED; PointSav owns the Rust layer (sDDF / drivers) above; "We Own It" doctrine | Locked |
| Language: monoglot vs. polyglot | 2026-02-03 | Rust mandatory for all Core Totebox Services; C/C++ banned; Python/JS banned at core; WebAssembly permitted for guest user extensions | Locked |
| Container vs. unikernel | 2026-01-28 to 2026-03-24 | Long-term: Unikernel `os-totebox` (NanoVMs / Unikraft / Genode). Interim: LXC, FreeBSD jails. Production must be immutable single-binary | Direction set; not yet executed |
| Naming generation 1 (TitleCase) | 2025-12 to 2026-02-06 | [SUPERSEDED BY: kebab-case (G2) at email #083, then `os-`/`app-`/`service-` prefixed (G3) at email #140] | Yes |
| Naming generation 2 (`totebox-os` kebab) | 2026-02-07 (#083) to 2026-02-14 | [SUPERSEDED BY: `os-totebox` / `os-*` prefix style (G3) at email #140] | Yes |
| Naming generation 3 (`os-*`, `app-[os]-*`, `service-*`) | 2026-02-15 (#140) onward | Aligns with current canonical | Current |
| Six-tier sovereignty matrix | 2026-02-15 (#140) | `app-*` / `asset-*` / `moonshot-*` / `os-*` / `service-*` / `system-*` | Locked |
| Console OS rendering: terminal vs. game-engine | 2026-02-04 to 2026-02-05 | Standalone WGPU-based "Game Engine that looks like a TUI"; ratatui logic kept, ratatui rendering rejected; PointSav owns `pointsav-window` / `pointsav-gpu` / `pointsav-text` (SDF) / `pointsav-layout` | Locked |
| MediaKit kernel choice (Linux/GPL vs. FreeBSD/BSD) | 2026-02-05 | FreeBSD chosen (BSD licence allows proprietary closure); "Ship of Theseus" 4-phase metamorphosis: Hardened FreeBSD → NanoBSD → Custom Kernel → Unikernel | Locked |
| Hypervisor pattern for MediaKit | 2026-02-05 | seL4 Microvisor (Type-1) hosting MicroVMs for WP / Wiki / RSS; atomic 1:1 VM-per-workload deployment | Locked |
| Diode standard (unidirectional command flow) | 2026-02-05 (#067) | Universal: `os-console` (or `os-orchestration`) can command Totebox / MediaKit / etc.; reverse path blocked at protocol level | Locked |
| Word "Sovereign" — keep or drop? | 2026-02-07 (#087) | DROP from external marketing; "saturated by Sovereign AI marketing"; "Totebox Orchestration" and "Private Network" are the operative terms | Locked but partially honoured in emails (the word persists in later messages) |
| `service-auth` (Kanidm) vs. `service-pairing` | 2026-02-07 (#089) | `service-pairing` chosen; Kanidm rejected; no passwords, no user databases | Locked |
| Pull-based vs. push-based git supply chain | 2026-02-03 to 2026-02-06 | 5-stage circular: Contributor → Vendor (squash-merge IP transfer) → Customer mirror → Production deploy → Trunk-based contributor reset. "Double-blind air-gap": contractor never pushes to Woodfine; customer never sees contractor | Locked |
| Three-org legal topology | 2026-01-28 (#017) onward | Woodfine Capital Projects Inc. (100% parent) → owns 100% of PointSav Digital Systems (vendor) and 100% of Woodfine Management Corp. (customer); flow is vendor → customer → deployments | Locked |
| Sovereign Data Foundation role | 2026-01-28 (#017) | [BCSC-REVIEW-REQUIRED] Email #017 states "Sovereign Data Foundation (Denmark). The Foundation holds a 10% equity stake in PointSav Digital Systems and oversees the integrity of the open-source components." Per current BCSC posture (CLAUDE.md), Sovereign Data Foundation must be referenced in planned/intended terms only. The 10% equity-stake language in the email cannot be reproduced publicly without flagging it as a planning-stage construct. Surface to operator. | [BCSC-REVIEW-REQUIRED] |
| License strategy | 2026-01-28 (#010) | Apache 2.0 + "Sovereign Addendum" (planned); core packages FOSS, Orchestration / Interface OS proprietary; "Functional Access License" mentioned as planned variant | Direction set |
| GitHub topology (orgs + repos) | 2026-02-07 (#085) to 2026-03-03 (#160) | `pointsav-monorepo`, `pointsav-design-system`, `content-wiki-documentation` under `github.com/pointsav` (vendor); `woodfine-fleet-deployment`, `woodfine-fleet-manifest`, `content-wiki-corporate`, `content-wiki-projects` under `github.com/woodfine` (customer) | Live (per email #197) |
| Pair-based permissions (MBA) | 2026-01-28 onward | Replaces RBAC entirely; hardware-bound cryptographic pairings; "Connection is the Permission" | Locked |
| F-key UI doctrine | 2026-04-26 (#223) | F1 HELP · F2 PEOPLE · F3 EMAIL · F4 CONTENT · F5 MINUTEBOOK · F6 BOOKKEEPER · F12 INPUT MACHINE | Locked (SYS-ADR-10: F12 mandatory) |
| Compute Matrix (three tiers) | 2026-04-11 (#221) | Tier 1 — Zero-Compute Vault ($7 GCP node, WORM only); Tier 2 — Yo-Yo Relay (user-provisioned elastic GCP node for batch SLM); Tier 3 — Sovereign Iron Vault (in-house 16GB+ RAM, local SLM) | Locked; matches current `service-slm` three-tier routing |
| Genesis Protocol (autonomous N=1) | 2026-03-26 (#203) | `os-infrastructure` self-bootstraps a PPN of size 1 if no beacon found; `os-network-admin` later "claims" the cell via MBA socket | Active design |
| Brokerless mesh (no NATS/Kafka) | 2026-03-26 (#204) | Pure Rust UDP gossip protocol on port 8090, WireGuard-encrypted; binary 16-byte command protocol; no central control plane | Active research |
| Invisible AI in F8 Terminal | 2026-03-26 (#204) | `service-slm` as semantic command parser, never a chat window; translates English intent → binary UDP commands | Locked |
| Skeuomorphism rejection | 2026-03-30 (#212) | Word/Pages/Docs are "8.5×11 simulations, not digital"; PointSav generates content from structured data instead | Position lock |
| Tokenisation: pointer vs. asset | 2026-03-30 (#213) | "Wall Street tokenises the pointer; PointSav tokenises the asset itself"; cryptographic asset binding to Totebox | Position lock |

---

## 5. Competitive references

| Competitor / reference | Context in emails | PointSav differentiator mentioned |
|---|---|---|
| Q4 Inc, Cision | "IR Websites" rental; data lives in Q4 cloud; archive disappears if subscription stops; 5–15 min lag | `os-mediakit` "Sovereign Compliance Appliance" — push-based, direct from `os-totebox`, instant, cryptographically signed XML; "mathematical proof of disclosure" |
| Bloomberg Terminal | Reference standard for terminal-as-product, F-keys, Type-II hypervisor delivery (Martin Shkreli video #003); "writing for 60+ year-old family office founders" voice standard | F-key driven `os-console`; "Bloomberg article standard" prose |
| AWS / Azure / Google Cloud (hyperscalers) | "Hyperscaler Gap" — they couple Compute to Storage, charge egress, prevent custodial ownership; AWS Outposts and Kubernetes lock-in | PointSav uses cloud as Stateless Diode; storage runs on customer's own hardware; sovereignty + freely transferable |
| DE Shaw & Co. | Quoted as "investment + technology development firm" model — combine algorithmic and discretionary; first email (#000) | PointSav vendor structure mirrors this: builds the technology that powers the parent's operations |
| Groq / Discourse | Cited as CMS/community model for what `os-mediakit` should resemble; multiple separate sub-domains (Marketing / Community / Docs) | `os-mediakit` separates `app-mediakit-marketing` / `-knowledge` / `-distribution` into atomic 1:1 deployments |
| Wikipedia | "Bottom-up ontology generation" — referenced as the model for L4 Sovereign Taxonomy emergence; topics emerge from articles, not pre-defined | Foundry wiki strategy: organic article-driven structure, not central taxonomy |
| Solid (Tim Berners-Lee) | Struggles with adoption because it forces users to become IT administrators | PointSav UI is for Fiduciaries, not engineers; "Falcon 9 / Astronaut" Human-in-the-Loop UI |
| Kanidm | Considered for `service-auth`; rejected as RBAC-shaped | `service-pairing` replaces it; MBA pairings, not user accounts |
| Microsoft Word / Google Docs / Apple Pages | "Skeuomorphic simulations of 8.5×11 paper"; not digital | `service-content` generates structured-data → format outputs; native machine-readable |
| Electron / Chromium | "100MB+ bloat"; web-app trap | PointSav `os-console` ~15MB Rust + WGPU; native everywhere |
| Adobe (PDF) | Bloat | `app-pdfs` = pdf-rs / lopdf, strict ISO PDF/A fidelity |
| LibreOffice | Legacy C++ | `app-workplace-wordprocessor` = Typst + Rust |
| QGIS / ESRI | 30 years of legacy C++ (GDAL/OGR) | `app-console-bim`/`-gis` = Whitebox-tools + Ifc-rs + truck (pure Rust) |
| NATS / Kafka | "Crutches"; centralised brokers | `service-udp` = pure Rust UDP gossip on WireGuard mesh; brokerless |
| Bloomberg Terminal | Treated as muscle-memory + visual reference; "Bloomberg Bureau Chief" voice editor in `service-content` v2.8 | `os-console` F-keys + Command Ledger replicate the terminal feel |
| OpenAI / ChatGPT / Gemini chat UIs | "Sparkle Icons", "Chat boxes" — exhausting; force users to interact with AI | "Invisible AI" doctrine — `service-slm` is a silent semantic translator; no chat surface |
| SafeGraph / Snowflake Data Exchange | Referenced as "Data Refinery" precedents for the Totebox Archive | PointSav Totebox = decentralized, sovereign equivalent; data stays on customer hardware |
| Ondo Chain / permissioned L1 blockchains | Forwarded by Peter (email #147) as precedent for compliance-grade infrastructure | (No direct PointSav comparison drawn) |

**Public-content rule:** per "Structural positioning only" doctrine (CLAUDE.md §6), competitor names above must NOT appear in public-facing wiki content. Use structural framing instead ("legacy SaaS IR vendors", "hyperscaler clouds", "legacy office suites", etc.).

---

## 6. Design system notes

| Element | Description | Status in emails |
|---|---|---|
| `package-fonts` (asset vault) | Pure-asset storage for all font files; categorised into 5 functional families + 2 brand | Locked (#088, #092) |
| `package-typography` (engine) | Rust-native fallback controller / text shaping / variable-font interpolation; uses cosmic-text or harfbuzz-rs | Locked (#088) |
| `font-ui-variable` | Single Variable Sans for UI / Workplace / mobile; replaces App+Mobile families; Roboto Flex or Inter Variable | Locked (#092) |
| `font-mono-code` | Nerd-Font patched monospace for Console-OS + The Forge; coding ligatures; slashed zero; distinguishable `1 l I` | Locked (#079, #092) |
| `font-document-serif` | Transitional Serif for screens + print; Wiki + Record Keeping; Noto Serif or Literata | Locked (#079, #092) |
| `font-tabular-nums` | Specialised numeric subset for IronCalc + financial reports; vertical alignment in tables | Locked (#092) |
| `font-iconography` | SVG-in-OTF vector symbol set replacing PNG icons | Locked (#092) |
| `asset-brand-pointsav` | Marketing / logo / public keynote brand; never used for UI text | Locked (#079) |
| `asset-brand-wcp` | Distinct sub-brand identity for Woodfine Capital Projects; legally distinct from PointSav | Locked (#079, #080 — github.com/woodfine/media-assets-wcp) |
| BIM design system | NOT to be conflated with `pointsav-design-system`; lives in separate `woodfine-design-bim` per project memory `project_two_design_systems.md` | [SUPERSEDED IN EMAILS: dtcg-vault was being staged in `pointsav-design-system` — per current memory this is a misplacement; future BIM tokens belong in `woodfine-design-bim → bim.woodfinegroup.com`] |
| Print = Screen fidelity (PDF/A) | "Typography as Law"; printed audit reports must be byte-identical to digital | Doctrine (#082, #097) |
| Language-switch button "illusion" | Dual `<span>` with CSS show/hide; no JavaScript needed for static-page bilingual swap | Locked (#167); DARP-compliant Vanilla Web API approach for dynamic state (navigator.language) |
| Six-tier sovereignty matrix | `app-*` / `asset-*` / `moonshot-*` / `os-*` / `service-*` / `system-*` directory taxonomy | Locked (#140) |
| Strict kebab-case | All repos, dirs, packages, binaries; capital letters BANNED | Locked (#083) |
| Voice / tone | Bloomberg article standard; reading-age 60+ family-office founder; Warren Buffett "plainspoken financial professional" tone; no AI-product marketing vocabulary; no "tech-bro" jargon | Locked (#137, #170, #176) |
| "Structural Integrity Synthesis" (SIS-V2) | Matrix-based Markdown schema; each paragraph an Agent tied to a Heading + Theme; YAML frontmatter | Active doctrine (#157–#159) |

---

## 7. Legal / IP structure notes

The three-org legal topology is the cornerstone of the entire architecture and recurs in dozens of emails:

```
Woodfine Capital Projects Inc.  (100% parent — real-estate firm)
 ├─ PointSav Digital Systems     (vendor; IP holder; technology development arm)
 └─ Woodfine Management Corp.    (customer; operates real-property assets)
```

Email #017 explicitly states `PointSav Digital Systems AG` (Swiss/Austrian/German GmbH-style suffix) and references Sovereign Data Foundation (Denmark) as a 10%-equity governance holder. **[BCSC-REVIEW-REQUIRED]** — current Foundry BCSC posture (CLAUDE.md, project memory `feedback_bcsc_disclosure.md`) constrains Sovereign Data Foundation references to planned/intended language only. The email's equity-stake claim cannot be reproduced publicly without explicit operator review.

**IP transfer mechanism — "Squash and Merge as IP Assignment"** (email #045):

| Step | Actor | Action |
|---|---|---|
| 1. The Bid | Contractor (jwoodfine / pwoodfine) | Push & PR — submit deliverable; not yet approved or paid |
| 2. Acquisition | Vendor (PointSav) | Squash-and-merge: strips contractor commit history; mints single corporate commit; ownership transfers to PointSav |
| 3. Distribution | Vendor (PointSav) | Release tag v1.X |
| 4. Procurement | Customer (Woodfine) | Pulls verified product from vendor; never touches contractor |
| 5. Execution | Host (iMac / Production) | Deploys client's licensed copy |
| 6. Reset | Contractor | Trunk-based rebase — wipes local environment, matches vendor standard for next cycle |

**Double-blind air-gap rule** (email #045): Contractor never pushes to Woodfine; Customer never sees contractor repos; PointSav is the only entity that sees both.

**Authority of Assignment — "Handwritten Note" protocol** (email #082): For paid contributors, a handwritten note is the binding contract for an assignment ("Personnel Risk Management"). Strict governance over who is allowed to touch the code and the data.

**Executive vs. Manager regulatory split** (email #233): Per BCSC definitions, Peter Woodfine = Executive (fiduciary duty, sets strategic mandate); Jennifer Woodfine = Manager (oversees daily execution). This split aligns with GitHub permission tiers.

**Patent strategy** (email #173): "For PointSav, we are patenting the software structural mechanics (the seL4 microkernel bridging, the Capability-Based Manager). For Woodfine, we are patenting the financial-operational mechanism — specifically, how the Direct-Hold Solution systematically [creates and tokenises real-property as a real-asset]."

**License strategy** (email #010, #034): Apache 2.0 with a "Sovereign Addendum"; the addendum guarantees the data instance (the running VM) remains freely transferable by the user. Inspired by Google's Android playbook: open core, proprietary services. `os-totebox` and `os-console` = FOSS (the free phone); `os-orchestration` = proprietary (the Play Store).

---

## 8. Hardware research

| Hardware | Purpose | Status / Notes from emails |
|---|---|---|
| **iMac 12.1** (Sandy Bridge, 2011) | The "Foundry" build machine; reference development device; canonical first deployment | Reference device through #109; superseded by MacPro post-March (#194, #195) for service-content development. Email #007 calls VM-login "the Master Key" |
| **Dell XPS 13 / 14 Developer Edition** | The "Executive" / High-End Flagship workplace-os reference device | Locked (#091); Dell officially supports Linux ⇒ open WiFi+GPU drivers ⇒ mature into FreeBSD |
| **HP ProBook 400 series (445/450)** | The "Fleet" / Mid-Tier Workhorse | Locked (#091); enterprise-grade, BSD-compatible WiFi unlike Acer |
| **MacBook Air** (Laptop A) | Simulated on-prem `os-infrastructure` node | Active (#106, #109, #111, #123, #124) |
| **MacBook Pro** (Laptop B) | Simulated leased `os-infrastructure` node (Equinix-style) | Active (#104, #109, #111, #123, #124) |
| **MacPro** | Production service-content development workstation; macOS 10.13.6; later host for SLM/RAG | March 2026 (#178, #190, #194) |
| **Google Cloud VM ($7/mo e2-micro)** | Tier-1 Zero-Compute Vault baseline; cloud-side `os-infrastructure` node | Active (#107, #221, #222); hardware virtualization disabled (no nested VMs) |
| **Atal Networks (Vancouver bare metal)** | Top recommendation for local performance bare-metal dedicated server | Researched (#081); <1-2ms latency to Vancouver; unmanaged IPMI/KVM |
| **GTHost (Vancouver)** | Instant-activation dedicated servers | Researched (#081); automation-first |
| **Vultr (Seattle)** | Cloud-API bare metal for Vancouver-region (~3-10ms) | Researched (#081) |
| **OVHcloud (Montreal)** | Cheapest big-vendor for Canadian bare metal | Researched (#081) |
| **Dell PowerEdge T160 (tower)** | Considered for on-prem server | Researched (#120) |
| **Dell PowerEdge R760 (rack)** | Considered for on-prem rack | Researched (#120) |
| **CPU architecture target** | Haswell+ x86_64 (Nehalem too old; `fsgsbase` instruction required); 1 GB Huge Pages (`+pdpe1gb` flag) | Locked (#078, #112) |

---

## 9. Compliance / regulatory framework references

| Framework | Role in PointSav design |
|---|---|
| **SOC 3** | Operational trust gold standard. Embedded as `system-audit` crate at microkernel level — physically un-disable-able. Email #098, #162. |
| **DARP (Digital Asset Resolution Package)** | Structural sovereignty / Swiss FINMA standard. Embedded as `system-resolution` crate — self-executing parachute on vendor failure. Email #098, #162, #167. |
| **OSCAL (NIST Open Security Controls Assessment Language)** | Machine-readable compliance manifests at `oscal-manifests/` in every repo | Email #094, #096, #098. |
| **ISO 19650** | Document state codes for the BIM / record-keeping suffix system (`_JW`=S0, `_FIN`=S4, `_PUB`=A0, `_EXE`=CR) | Email #166. |
| **ISO/IEC 42001 (AI Management System / AIMS)** | Algorithmic transparency — immutable log of every AI decision in the supply chain | Email #098. |
| **DORA (EU Digital Operational Resilience Act)** | Targeted EU compliance | Email #082, #094, #098. |
| **FedRAMP** | Targeted US Government compliance | Email #098. |
| **GDPR** | Targeted EU privacy compliance | Email #098. |
| **NI 51-102 / OSC SN 51-721 (BCSC continuous-disclosure)** | Per Foundry doctrine (CLAUDE.md) — all forward-looking claims carry planned/intended/may/target language. Sovereign Data Foundation must be referenced in planned terms only | Doctrine; not explicit in emails |
| **SLSA (Supply-chain Levels for Software Artifacts)** | Vendor-side supply-chain compliance | Email #122. |
| **ISO 15489 (Records Management)** | Naming-convention basis for repository documentation | Email #128. |
| **ISO 24495-1 (Plain Language)** | Voice / tone standard for `service-content` outputs | Email #157. |
| **Minto Pyramid Principle** | Executive-communication structure for `service-content` outputs | Email #157. |

---

## 10. Open questions / unresolved contradictions

| Question | Where it appears | Recommended action |
|---|---|---|
| Sovereign Data Foundation equity stake | Email #017 ("10% equity stake in PointSav Digital Systems") | [BCSC-REVIEW-REQUIRED] — surface to operator; do not publish; reframe as planned governance partner per BCSC posture |
| `PointSav Digital Systems AG` vs. `PointSav Digital Systems` | Email #017 uses "AG"; later emails drop it | Confirm legal entity name with operator |
| BIM design system placement | Emails place BIM content inside `pointsav-design-system`; project memory `project_two_design_systems.md` says BIM belongs in separate `woodfine-design-bim` | Cross-reference project memory at editorial review; route BIM content to `woodfine-design-bim` not `pointsav-design-system` |
| Console OS singular vs. tiered | Email #028 says "no tiered version of the Console — there is only one"; later emails (#089, #150) talk about commercial-only features in `os-orchestration` aggregator | Consistent: Console is single; commercial complexity moves to `os-orchestration` |
| `service-llm` vs. `service-slm` | Some later emails (#206, #243) use `service-LLM`; per project memory the canonical is `service-slm` (OLMo, not Qwen, not Gemma) | Always normalise to `service-slm`; flag `service-llm` mentions [SUPERSEDED BY: service-slm] |
| Workplace-OS jurisdictional data locking | Email #093 — was originally proposed in `workplace-os`, corrected to belong in `os-totebox` (the data layer) | Correction is consistent with current architecture |
| Unikernel timeline | Email #200 — "ultimate undisputed endgame, but a production artifact, not a development environment"; iterative development needs LXC / FreeBSD jails / Alpine first | Resolution is "both": iterate now in LXC, ship unikernel later |
| GitHub repo naming generation drift | Three generations of repo names appear in emails (`pwoodfine/Console-OS` → `pointsav/console-os` → `pointsav-monorepo`); per email #160, #197 current state is `pointsav/pointsav-monorepo` + `woodfine/woodfine-fleet-deployment` | Use current canonical; flag old names |
| Token Architecture purge | Email #186 — "purge the architecture protocol; redundancy is systemic friction"; older docs reference separate Zero-Drift and Data Mesh files | Confirm token architecture is now single unified file per email #186 |
| "Master / Slave" terminology | Used in some emails (#040, #150, #211) to describe Console / Totebox relationship | Replace with "Command / Subject" or "Console / Archive" per current language standards; [SUPERSEDED] |
| GitHub credentials leaked in emails | Email #202 contains a username/password pair in plaintext; email #196 reports an Azure AD Application Secret committed to a public repo | [SECRET-EXPOSURE] surface to operator; rotate the password and confirm Azure secret was revoked |
| L4 Sovereign Taxonomy build authority | Email #220 — "Built initially by the Gemini API, but heavily influenced and adjusted by the system operators over time" | Per SYS-ADR-07 (no structured data through AI) and current Foundry doctrine, AI cannot publish structured data autonomously; flag this as misaligned-with-doctrine |
| service-content + Gemini API in production | Email #163 — "service-content … running in Google Cloud … Gemini API running on the VM"; per SYS-ADR-19, no automated AI publishing to verified ledgers | Surface as drift item; confirm `service-content` outputs are routed through human-in-the-loop verification, not autonomous |

---

## 11. Email-archive provenance map

| Topic cluster | Email IDs (representative) | Approximate count |
|---|---|---|
| Three-org legal topology / Sovereign-Talent-Engine prototype | 005, 008–010, 014, 017, 019, 022, 025–026 | 12 |
| Chart of Accounts + 11 Archetypes substrate | 021–025, 028, 224 | 8 |
| os-totebox architectural blueprint | 005, 010–019, 027–028, 034–035, 041, 211, 217, 219, 238 | 24 |
| os-console (Asset Rendering Terminal / WGPU / SDF) | 015, 028, 047, 049–054, 070, 089, 149–151 | 14 |
| os-orchestration / Interface aggregator | 028, 034, 040, 045–046, 089, 150–151 | 10 |
| os-mediakit (FreeBSD + WordPress/MediaWiki/FreshRSS + Diode) | 058–074 | 17 |
| os-workplace + apps | 089, 091, 140 | 5 |
| os-privategit (source + design system) | 086, 091, 099 | 4 |
| os-infrastructure + os-network-admin + PPN | 102, 104, 106–112, 122–124, 139, 202–207, 219, 222 | 25 |
| seL4 + Rust + memory-mapping bugs | 027, 042, 047–054, 076, 078, 101, 112, 118, 152, 200, 205 | 18 |
| Naming conventions / six-tier matrix | 050, 054, 061, 072, 083, 087, 094, 140, 206, 214, 231 | 12 |
| GitHub supply-chain pipeline ("Horseshoe") | 036, 042, 045, 055, 077, 084–085, 122, 131, 154 | 11 |
| Compliance (SOC 3 / DARP / OSCAL / DORA / FedRAMP / ISO) | 094, 096, 098, 144, 157, 162, 167 | 8 |
| service-slm + service-content (Doorman + Gravity Engine) | 175, 178–179, 200, 203–206, 220–227, 229–243 | 30 |
| service-email + M365 + Maildir | 030, 032, 181–184, 188–192, 232–233 | 14 |
| Design system / typography / fonts | 079–080, 082, 084, 088, 092, 105, 187, 198 | 10 |
| Voice / tone / language protocols | 121, 130, 137, 158–159, 170, 174, 176, 198 | 9 |
| Hardware research | 081, 091, 109, 120, 178, 190 | 6 |
| GitHub-noise (PRs, SSH-keys, org-membership notifications) | 057, 100, 108, 115–117, 141–146, 148, 168, 194–197 | 17 |
| Forwards from Peter / Peter's notes | 000–003, 020, 056, 113–114, 147, 155–156, 182 | 12 |
| Other / one-offs (DNS, secrets, misc) | 029, 044, 103, 161, 164–166, 169, 171–173, 177, 199, 201, 207–215 | 20 |
| **Total** (with overlap) | | **~245** |

---

## 12. Notes for downstream editorial use

1. **Canonical-name normalisation pass required.** Any draft moving from `.agent/drafts-outbound/` to `content-wiki-documentation/` must replace older variants (G1, G2) with current canonical (`os-*`, `app-*`, `service-*`).
2. **No "Sovereign Data Foundation" public claims** until BCSC-review-required items above are cleared by operator.
3. **No "Sovereign" as a noun marketing term** — email #087 already retired it. Use "Totebox Orchestration" or "Private Network".
4. **No competitor names in public wikis** — replace with structural framing.
5. **No automated AI publishing** to `content-wiki-*` — per SYS-ADR-19; AI drafts must transit `project-editorial` and Master ratification.
6. **GUIDE vs. TOPIC split**: anything operational ("how to run", "how to deploy") goes to `woodfine-fleet-deployment` as GUIDE-*; anything conceptual ("what it is", "why it exists") goes to `content-wiki-documentation` as TOPIC-*.
7. **BIM content** must route to `woodfine-design-bim` (planned customer-tier design system), NOT to `pointsav-design-system`.
8. **Two emails contain secrets** — surface to operator separately (see §10).
