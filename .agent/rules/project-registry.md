# Project Registry — pointsav-monorepo

Living inventory of every top-level project directory with its current
state. Read at session start. Update when activating, retiring, or
reclassifying a project. Registry drift (a directory not in the
table, or a table row without a matching directory) is visible and
must be closed.

State vocabulary — see `~/Foundry/CLAUDE.md` §8 for definitions.

Last updated: 2026-05-31.

---

## App — Console surface (`app-console-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-console-bim | Reserved-folder | app-console | 1 file (RESEARCH.md); research phase |
| app-console-bookkeeper | Active | app-console | Activated 2026-04-22 via framework §8 (pilot); HTML-plugin pattern (view + cartridge); registry row was originally mis-classified; `README.*` and data-binding pending |
| app-console-content | Scaffold-coded | app-console | 8 files; in workspace members |
| app-console-email | Active | app-console | Phase C 2026-05-31; lib crate; EmailCartridge (F3); inbox list + read + compose/send; workspace member; plain mode supported |
| app-console-help | Reserved-folder | app-console | READMEs only |
| app-console-input | Scaffold-coded | app-console | 6 files |
| app-console-keys | Active | app-console | Phase 3+4 active; chassis + Kitty/Sixel QR + Cartridge trait; ratatui-image v9 |
| app-console-mesh | Reserved-folder | app-console | Placeholder |
| app-console-minutebook | Reserved-folder | app-console | READMEs only |
| app-console-people | Scaffold-coded | app-console | 5 files |
| app-console-slm | Active | app-console | Phase D 2026-05-31; lib crate; SlmCartridge (F9); Doorman health dashboard + entity count; 10s poll + R refresh; workspace member |
| app-console-system | Active | app-console | Phase 4 (pairing) 2026-05-23; F11 operator panel; pending-pair approvals; workspace member |
| app-console-vault | Reserved-folder | app-console | Placeholder |

## App — MediaKit surface (`app-mediakit-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-mediakit-distributions | Scaffold-coded | app-mediakit | 4 files |
| app-mediakit-knowledge | Scaffold-coded | app-mediakit | 4 files |
| app-mediakit-marketing | Scaffold-coded | app-mediakit | 4 files |
| app-mediakit-telemetry | Scaffold-coded | app-mediakit | 14 files; MaxMind `.mmdb` pending move to build-time fetch |

## App — Network surface (`app-network-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-network-cluster | Reserved-folder | app-network | Placeholder |
| app-network-gateway | Reserved-folder | app-network | Placeholder |
| app-network-help | Reserved-folder | app-network | Placeholder |
| app-network-infrastructure | Reserved-folder | app-network | Placeholder |
| app-network-keys | Reserved-folder | app-network | Placeholder |
| app-network-media | Reserved-folder | app-network | Placeholder |
| app-network-radar | Reserved-folder | app-network | Placeholder |
| app-network-vault | Reserved-folder | app-network | Placeholder |

## App — Orchestration surface (`app-orchestration-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-orchestration-bim | Reserved-folder | app-orchestration | 2 files (CLAUDE.md + RESEARCH.md byte-identical to app-console-bim copy, Task Claude — BIM to rationalise); triggered taxonomy expansion to seventh in-force domain on 2026-04-22; directory created 2026-04-23 (closes registry drift) |
| app-orchestration-exchange | Reserved-folder | app-orchestration | Browser ad campaign UI; deployed as gateway-orchestration-exchange-N; Doctrine claim #52 |
| app-orchestration-gis | Reserved-folder | app-orchestration | GIS/mapping orchestration surface; deployed as gateway-orchestration-gis-1; registry drift closed 2026-05-07 |
| app-orchestration-market | Reserved-folder | app-orchestration | Browser marketplace storefront; deployed as gateway-orchestration-market-N; Doctrine claim #52 |
| app-orchestration-slm | Scaffold-coded | app-orchestration | Commercial Yo-Yo broker chassis (DOCTRINE #23); 3-crate workspace (orchestration-slm-core, orchestration-slm, orchestration-slm-server); port :9180; MVP scaffold 2026-05-27; deploys as gateway-orchestration-slm-N on os-orchestration |

## App — PrivateGit surface (`app-privategit-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-privategit-design-system | Scaffold-coded | app-privategit | 4 files |
| app-privategit-marketplace | Active | app-privategit | software.pointsav.com storefront; product catalog, license issuance, payment verification; v0.0.3 deployed on vault-privategit-source-1 port 9202; activated 2026-05-21 |
| app-privategit-source | Active | app-privategit | Binary release server + Ed25519 license token verification; port 9201; v0.1.0 deployed; supersedes `app-privategit-source-control`; activated 2026-05-21 |
| app-privategit-source-control | Scaffold-coded | app-privategit | Original scaffold (lib.rs stub, 4 files); superseded by `app-privategit-source`; pending cleanup |
| app-privategit-workbench | Scaffold-coded | app-privategit | Browser-based Totebox developer workbench; 1,191-line main.rs; v0.0.1; not in manifest; registry drift closed 2026-05-31 |

## App — Totebox surface (`app-totebox-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-totebox-corporate | Scaffold-coded | app-totebox | 4 files |
| app-totebox-real-property | Scaffold-coded | app-totebox | 4 files; canonical is `cluster-totebox-property` per rename table |

## App — Workplace surface (`app-workplace-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| app-workplace-bim | Reserved-folder | app-workplace | RESEARCH.md + NEXT.md; research phase; Wave 3 |
| app-workplace-gis | Scaffold-coded | app-workplace | Wave 2; MapLibre GL WebView shell; CLAUDE.md + NEXT.md; src-tauri stub; no Tauri crate yet |
| app-workplace-memo | Active | app-workplace | Wave 1; Tauri v1.7 document editor; CLAUDE.md + NEXT.md present |
| app-workplace-pdf | Scaffold-coded | app-workplace | Wave 2; pdfium-render crate (Apache 2.0); CLAUDE.md + NEXT.md; src-tauri stub; no Tauri crate yet |
| app-workplace-presentation | Active | app-workplace | Wave 1; Tauri v1.7 slides; CLAUDE.md + NEXT.md + src-tauri skeleton added 2026-05-27 |
| app-workplace-proforma | Active | app-workplace | Wave 2; Tauri v1.7; CLAUDE.md + NEXT.md; minimumSystemVersion 10.13 confirmed |
| app-workplace-workbench | Active | app-workplace | Wave 1; Tauri v1.7 WebView shell; configurable port; CLAUDE.md + NEXT.md + src-tauri skeleton added 2026-05-27 |

## OS (`os-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| os-console | Scaffold-coded | os | 9 files |
| os-infrastructure | Scaffold-coded | os | 20 files; ISO artefact in directory — tracking status TBD |
| os-interface | Scaffold-coded | os | 4 files; legacy name — canonical is `os-orchestration` (rename in flight) |
| os-mediakit | Scaffold-coded | os | 4 files |
| os-network-admin | Scaffold-coded | os | 12 files; ISO artefact — tracking status TBD |
| os-privategit | Scaffold-coded | os | 4 files |
| os-totebox | Scaffold-coded | os | 6 files; IMG artefact — tracking status TBD |
| os-workplace | Scaffold-coded | os | 4 files |

## System (`system-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| system-audit | Reserved-folder | system | 2 files |
| system-core | Scaffold-coded | system | 5 files |
| system-gateway-mba | Scaffold-coded | system | 8 files; in workspace members |
| system-interface | Scaffold-coded | system | 4 files |
| system-network-interface | Scaffold-coded | system | 6 files |
| system-resolution | Reserved-folder | system | 2 files |
| system-security | Scaffold-coded | system | 22 files; in workspace members |
| system-slm | Scaffold-coded | system | 4 files |
| system-substrate | Scaffold-coded | system | 4 files |
| system-substrate-broadcom | Scaffold-coded | system | 4 files; hardware bridge |
| system-substrate-freebsd | Scaffold-coded | system | 4 files; hardware bridge |
| system-substrate-wifi | Scaffold-coded | system | 4 files; hardware bridge |
| system-udp | Reserved-folder | system | 3 files |
| system-verification | Reserved-folder | system | 2 files |
| system-vm-fleet-types | Scaffold-coded | system | PPN VM resource pool wire types; `no_std`-compatible; workspace member; 4 tests passing (2026-05-29) |

## Service (`service-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| service-bim | Reserved-folder | service | 1 file (RESEARCH.md); research phase |
| service-content | Scaffold-coded | service | 37 files; in workspace members |
| service-egress | Scaffold-coded | service | 4 files |
| service-email | Scaffold-coded | service | 18 files |
| service-email-egress-ews | Scaffold-coded | service | EWS protocol adapter; doubly-nested wrapper flattened 2026-04-23 (prior "consolidation" plan reversed — kept separate from `-imap` because they are two protocol-specific implementations, not duplicates); 6 sub-crates including EWS-only `egress-prune` and `egress-balancer`; Cargo.toml name mismatches (13 total across both) remain as separate audit finding |
| service-email-egress-imap | Scaffold-coded | service | IMAP protocol adapter; doubly-nested wrapper flattened 2026-04-23; 4 sub-crates; parallel structure to `-ews` but without prune/balancer |
| service-email-template | Scaffold-coded | service | 5 files |
| service-extraction | Active | service | 21 files; CLAUDE.md present but stale (see NEXT.md Item 9) |
| service-fs | Scaffold-coded | service | 3 files; in workspace members |
| service-http | Scaffold-coded | service | 9 files |
| service-message-courier | Reserved-folder | service | 1 file |
| service-people | Scaffold-coded | service | 17 files; in workspace members |
| service-pty-bridge | Scaffold-coded | service | Renamed 2026-04-23 from `pointsav-pty-bridge` (brand-prefix violation resolved); 1 source file (`src/main.rs`); not a workspace member |
| service-search | Reserved-folder | service | 1 file |
| service-slm | Scaffold-coded | service | Contains `router/` (Rust runtime, renamed 2026-04-23 from `cognitive-forge/`) and `router-trainer/` (Python distillation workflow, moved in 2026-04-23 from former top-level `tool-cognitive-forge/`); both names replace the retired "cognitive-forge" term per Do-Not-Use list |
| service-totebox-egress | Scaffold-coded | service | 18 files |
| service-vm-fleet | Active | service | PPN VM fleet controller; axum :9203; heartbeat ingestion + advisory placement; GET /v1/vms?tenant_id= added; 14 tests passing; workspace member (2026-06-12) |
| service-vm-host | Scaffold-coded | service | PPN VM per-node heartbeat agent; /proc/meminfo reader; QEMU monitor stub; current_thread tokio; workspace member (2026-05-29) |
| service-vm-tenant | Active | service | PPN customer-facing VM proxy; axum :9221; Bearer auth + tenant namespace + quota enforcement; WORM audit trail; TOCTOU create Mutex; workspace member (2026-06-12) |
| service-vpn | Scaffold-coded | service | 11 files |

## Moonshot (`moonshot-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| moonshot-bim-engine | Scaffold-coded | moonshot | 4 files; sovereign IFC/BIM engine — replaces web-ifc/xeokit (app-workplace-bim licensing gate) |
| moonshot-crdt | Scaffold-coded | moonshot | 4 files; collaborative state + version lineage — replaces Loro/Yjs/Automerge |
| moonshot-database | Scaffold-coded | moonshot | 4 files |
| moonshot-docengine | Scaffold-coded | moonshot | 4 files; document model + AST bidirectional mapping — replaces ProseMirror/Lexical/TipTap |
| moonshot-editor | Scaffold-coded | moonshot | 4 files; editor/viewer/file-tree widget surface — replaces CodeMirror/Monaco/react-arborist |
| moonshot-gpu | Scaffold-coded | moonshot | 4 files |
| moonshot-hypervisor | Scaffold-coded | moonshot | 4 files |
| moonshot-index | Scaffold-coded | moonshot | 4 files |
| moonshot-kernel | Scaffold-coded | moonshot | 4 files |
| moonshot-network | Scaffold-coded | moonshot | 4 files |
| moonshot-parser | Scaffold-coded | moonshot | 4 files; incremental syntax parser — replaces tree-sitter |
| moonshot-protocol | Scaffold-coded | moonshot | 4 files |
| moonshot-sel4-vmm | Scaffold-coded | moonshot | 4 files |
| moonshot-toolkit | Scaffold-coded | moonshot | 5 files; Rust-only build orchestrator per repo CLAUDE.md |

## Tool (`tool-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| tool-acs-miner | Scaffold-coded | tool | 3 files; in workspace members |
| tool-archive-rescue | Reserved-folder | tool | 3 files |
| tool-edgar-extractor | Reserved-folder | tool | 2 files |
| tool-egress-pull | Scaffold-coded | tool | 4 files |
| tool-template-rescue | Reserved-folder | tool | 3 files |
| tool-wallet | Active | tool | Polygon USDC payment watcher + receipt writer; BIP-39/BIP-32 HD address derivation; Ed25519 keygen; v0.0.3 deployed on vault-privategit-source-1; activated 2026-05-21 |

## Vendor (`vendor-*`)

| Project | State | Type | Notes |
|---|---|---|---|
| vendor-azure-auth | Reserved-folder | vendor | 1 file |
| vendor-gpu-drivers | Reserved-folder | vendor | 1 file |
| vendor-linux-systemd | Reserved-folder | vendor | 1 file |
| vendor-microsoft-graph | Reserved-folder | vendor | 1 file |
| vendor-phi3-mini | Reserved-folder | vendor | 2 files |
| vendor-sel4-kernel | Scaffold-coded | vendor | 1074 files; vendored external seL4 kernel source |
| vendor-slm-engine | Reserved-folder | vendor | 3 files |
| vendor-virtio | Reserved-folder | vendor | 1 file |
| vendor-wireguard | Reserved-folder | vendor | 1 file |

## Other / special

| Project | State | Type | Notes |
|---|---|---|---|
| discovery-queue | Not-a-project | runtime data | 22 `TX-*_identity.json` files; gitignore + move to `service-fs/data/` |
| target | Not-a-project | build output | Rust cargo output; in .gitignore |
| xtask | Scaffold-coded | xtask | 2 files; in workspace members; Rust xtask convention |

---

## Summary (2026-06-12)

- **Active:** 12 (adds `service-vm-fleet`, `service-vm-tenant` 2026-06-12)
- **Scaffold-coded:** 59 (`service-vm-fleet` promoted to Active 2026-06-12)
- **Reserved-folder:** 39
- **Defect:** 0
- **Not-a-project:** 2 (`discovery-queue`, `target`)
- **Dormant:** 0
- **Archived:** 0

**Total rows:** 113 (added `service-vm-tenant` 2026-06-12). Last updated: 2026-06-12.
