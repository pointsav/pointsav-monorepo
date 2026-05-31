# Architecture Layer Catalog

Last updated: 2026-05-14. Maintained by MASTER. project-editorial holds a read copy.

Three layers define all PointSav / Woodfine software, deployment guides, and running instances.
Layer 3 is STRICTLY PRIVATE — those folders never appear on any GitHub org or wiki.

---

## Layer 1 — Software (pointsav-monorepo) — PUBLIC

Source: `/srv/foundry/clones/project-intelligence/` (Totebox cluster clone)
GitHub: `pointsav/pointsav-monorepo`

### OS family (`os-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| os-console | Yes | Scaffold-coded |
| os-infrastructure | Yes | Scaffold-coded; generates `pointsav-os-infrastructure.iso`; bare-metal ISO delivery |
| os-interface | Yes | Scaffold-coded; legacy name — canonical is `os-orchestration` (rename in flight) |
| os-mediakit | Yes | Scaffold-coded |
| os-network-admin | Yes | Scaffold-coded; PPN routing authority; publishes `public/mesh-state.json` |
| os-privategit | Yes | Scaffold-coded |
| os-totebox | Yes | Scaffold-coded |
| os-workplace | Yes | Scaffold-coded |

### App — Console surface (`app-console-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-console-bim | No | Reserved-folder |
| app-console-bookkeeper | No | Reserved-folder |
| app-console-content | Yes | Scaffold-coded |
| app-console-email | Yes | Scaffold-coded |
| app-console-exchange | No | Reserved-folder |
| app-console-help | No | Reserved-folder |
| app-console-input | Yes | Scaffold-coded |
| app-console-keys | No | Reserved-folder |
| app-console-market | No | Reserved-folder |
| app-console-mesh | No | Reserved-folder |
| app-console-minutebook | No | Reserved-folder |
| app-console-people | Yes | Scaffold-coded |
| app-console-vault | No | Reserved-folder |

### App — Infrastructure surface (`app-infrastructure-*`) — NEW 2026-05-14

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-infrastructure-cloud | No | Reserved-folder; GCP node cartridge (fleet-infrastructure-cloud-1); scaffolded 2026-05-14 |
| app-infrastructure-leased | No | Reserved-folder; Laptop B cartridge (fleet-infrastructure-leased-1); scaffolded 2026-05-14 |
| app-infrastructure-onprem | No | Reserved-folder; Laptop A cartridge (fleet-infrastructure-onprem-1); scaffolded 2026-05-14 |

### App — MediaKit surface (`app-mediakit-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-mediakit-distributions | Yes | Scaffold-coded |
| app-mediakit-knowledge | Yes | Active; documentation wiki engine |
| app-mediakit-marketing | Yes | Scaffold-coded |
| app-mediakit-telemetry | Yes | Scaffold-coded |

### App — Network surface (`app-network-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-network-admin | No | Reserved-folder; control-plane for os-network-admin; scaffolded 2026-05-14 |
| app-network-cluster | No | Reserved-folder |
| app-network-gateway | No | Reserved-folder |
| app-network-help | No | Reserved-folder |
| app-network-infrastructure | No | Reserved-folder |
| app-network-keys | No | Reserved-folder |
| app-network-media | No | Reserved-folder |
| app-network-radar | No | Reserved-folder |
| app-network-vault | No | Reserved-folder |

### App — Orchestration surface (`app-orchestration-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-orchestration-bim | No | Reserved-folder |
| app-orchestration-exchange | No | Reserved-folder |
| app-orchestration-gis | No | Reserved-folder |
| app-orchestration-market | No | Reserved-folder |

### App — PrivateGit surface (`app-privategit-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-privategit-design-system | Yes | Scaffold-coded |
| app-privategit-source-control | Yes | Scaffold-coded |

### App — Totebox surface (`app-totebox-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-totebox-corporate | Yes | Scaffold-coded |
| app-totebox-real-property | Yes | Scaffold-coded; canonical rename to `cluster-totebox-property` in flight |

### App — Workplace surface (`app-workplace-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-workplace-bim | No | Reserved-folder |
| app-workplace-memo | No | Reserved-folder |
| app-workplace-presentation | No | Reserved-folder |
| app-workplace-proforma | No | Reserved-folder |

### Service (`service-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| service-bim | No | Reserved-folder |
| service-content | Yes | Scaffold-coded |
| service-egress | Yes | Scaffold-coded |
| service-email | Yes | Scaffold-coded |
| service-email-egress | No | Reserved-folder |
| service-email-template | Yes | Scaffold-coded |
| service-exchange | No | Reserved-folder |
| service-extraction | Yes | Active |
| service-fs | Yes | Scaffold-coded |
| service-http | Yes | Scaffold-coded |
| service-market | No | Reserved-folder |
| service-message-courier | No | Reserved-folder |
| service-people | Yes | Scaffold-coded |
| service-pty-bridge | Yes | Scaffold-coded |
| service-search | No | Reserved-folder |
| service-slm | Yes | Active; Doorman live |
| service-totebox-egress | No | Reserved-folder |
| service-vpn | No | Reserved-folder |

### System (`system-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| system-audit | Yes | Scaffold-coded |
| system-core | Yes | Scaffold-coded |
| system-gateway-mba | Yes | Scaffold-coded |
| system-interface | Yes | Scaffold-coded |
| system-ledger | Yes | Scaffold-coded |
| system-network-interface | Yes | Scaffold-coded |
| system-resolution | Yes | Scaffold-coded |
| system-security | Yes | Scaffold-coded |
| system-slm | Yes | Scaffold-coded |
| system-substrate | Yes | Scaffold-coded |
| system-substrate-broadcom | Yes | Scaffold-coded |
| system-substrate-freebsd | Yes | Scaffold-coded |
| system-substrate-wifi | Yes | Scaffold-coded |
| system-udp | Yes | Scaffold-coded |
| system-verification | Yes | Scaffold-coded |

### Tool (`tool-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| tool-acs-miner | Yes | Scaffold-coded |
| tool-archive-rescue | Yes | Scaffold-coded |
| tool-edgar-extractor | No | Reserved-folder |
| tool-egress-pull | Yes | Scaffold-coded |
| tool-template-rescue | Yes | Scaffold-coded |

### Vendor (`vendor-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| vendor-azure-auth | No | Reserved-folder |
| vendor-gpu-drivers | No | Reserved-folder |
| vendor-linux-systemd | No | Reserved-folder |
| vendor-microsoft-graph | No | Reserved-folder |
| vendor-phi3-mini | No | Reserved-folder |
| vendor-sel4-kernel | No | Reserved-folder; vendored seL4 kernel source |
| vendor-slm-engine | No | Reserved-folder |
| vendor-virtio | No | Reserved-folder |
| vendor-wireguard | No | Reserved-folder |

### Moonshot (`moonshot-*`)

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| moonshot-database | Yes | Scaffold-coded |
| moonshot-gpu | Yes | Scaffold-coded |
| moonshot-hypervisor | Yes | Scaffold-coded |
| moonshot-index | Yes | Scaffold-coded |
| moonshot-kernel | Yes | Scaffold-coded |
| moonshot-network | Yes | Scaffold-coded |
| moonshot-protocol | Yes | Scaffold-coded |
| moonshot-sel4-vmm | Yes | Scaffold-coded |
| moonshot-toolkit | Yes | Scaffold-coded |

**Layer 1 total: ~110 directories** (8 os + ~45 app + 18 service + 15 system + 5 tool + 9 vendor + 9 moonshot)

---

## Layer 2 — Showcase (woodfine-fleet-deployment) — PUBLIC

Source: `/srv/foundry/customer/woodfine-fleet-deployment/`
GitHub: `woodfine/woodfine-fleet-deployment`

| Name | Prefix family | Layer-1 counterpart | GUIDE-* count | Notes |
|---|---|---|---|---|
| cluster-totebox-corporate | cluster | app-totebox-corporate | multiple | Active |
| cluster-totebox-personnel | cluster | service-people / service-email | multiple | Active |
| cluster-totebox-property | cluster | app-totebox-real-property | multiple | Active |
| fleet-infrastructure-cloud | fleet | app-infrastructure-cloud | 0 | Scaffold; no runbooks yet |
| fleet-infrastructure-leased | fleet | app-infrastructure-leased | multiple | Has spoke-configs/ — see compliance report |
| fleet-infrastructure-onprem | fleet | app-infrastructure-onprem | multiple | Has LXC deploy guide |
| gateway-interface-command | gateway | app-orchestration-command | 0 | Reserved |
| gateway-orchestration-bim | gateway | app-orchestration-bim | 0 | Reserved |
| gateway-orchestration-gis | gateway | app-orchestration-gis | multiple | Active; GIS deployment |
| media-knowledge-corporate | media | app-mediakit-knowledge | 0 | Reserved |
| media-knowledge-projects | media | app-mediakit-knowledge | 0 | Reserved |
| media-marketing-landing | media | app-mediakit-marketing | 0 | Reserved |
| route-network-admin | route | app-network-admin | 0 | Scaffold |
| vault-privategit-source | vault | app-privategit-source-control | multiple | Active |
| node-console-operator | non-standard | (no L1 counterpart) | 0 | **Naming defect** — not a standard prefix |

---

## Layer 3 — Instances (`~/Foundry/deployments/`) — STRICTLY PRIVATE

**PRIVACY RULE:** Layer 3 folders NEVER appear on any GitHub org (`pointsav/*` or `woodfine/*`)
and NEVER in any wiki (content-wiki-documentation, content-wiki-corporate, content-wiki-projects).
Contents include running config, keys, secrets, and instance-specific operational state.

Source: `/srv/foundry/deployments/` (local-only, gitignored)

| Name | Layer-2 counterpart | Has MANIFEST.md? | Notes |
|---|---|---|---|
| cluster-totebox-corporate-1 | cluster-totebox-corporate | Yes | Active |
| cluster-totebox-corporate-2 | cluster-totebox-corporate | Yes | Active |
| cluster-totebox-corporate-3 | cluster-totebox-corporate | Yes | Active |
| cluster-totebox-jennifer | (personal workspace) | No | Missing MANIFEST.md |
| cluster-totebox-personnel-1 | cluster-totebox-personnel | Yes | Active; data on sdb mount |
| cluster-totebox-property-1 | cluster-totebox-property | Yes | Active |
| gateway-orchestration-bim-1 | gateway-orchestration-bim | Yes | Active |
| gateway-orchestration-gis-1 | gateway-orchestration-gis | Yes | Active |
| gateway-orchestration-proofreader-1 | (missing L2 counterpart) | Yes | **No Layer-2 showcase folder exists** |
| media-knowledge-documentation-1 | media-knowledge-documentation | Yes | Active |
| media-knowledge-projects-1 | media-knowledge-projects | Yes | Active |
| media-marketing-landing-1 | media-marketing-landing | Yes | Active |
| media-marketing-landing-2 | media-marketing-landing | Yes | Active |
| vault-privategit-design-1 | vault-privategit-source | Yes | Active |

---

## Needed — not yet created

| Name | Layer | Blocks | Notes |
|---|---|---|---|
| app-infrastructure-onprem | 1 | PPN three-node topology | Scaffolded 2026-05-14 (Reserved-folder) |
| app-infrastructure-leased | 1 | PPN three-node topology | Scaffolded 2026-05-14 (Reserved-folder) |
| app-infrastructure-cloud | 1 | PPN three-node topology | Scaffolded 2026-05-14 (Reserved-folder) |
| app-network-admin | 1 | PPN routing authority | Scaffolded 2026-05-14 (Reserved-folder) |
| gateway-orchestration-proofreader | 2 | Layer-2 coverage for L3 instance | Instance exists; no showcase folder |
| media-knowledge-documentation | 2 | Layer-2 coverage for L3 instance | Instance exists; no showcase folder |

---

## Defects surfaced by this catalog

| Defect | Location | Action |
|---|---|---|
| `node-console-operator` non-standard prefix | Layer-2 woodfine-fleet-deployment | Rename or reclassify |
| `cluster-totebox-jennifer` missing MANIFEST.md | Layer-3 deployments/ | Add MANIFEST.md |
| `gateway-orchestration-proofreader-1` has no Layer-2 counterpart | Layer-3 deployments/ | Create `gateway-orchestration-proofreader/` in woodfine-fleet-deployment |
| WireGuard private keys in woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/ | Layer-2 GitHub | URGENT — see compliance report |
| `gateway-orchestration-gis-1/` directory in woodfine-fleet-deployment | Layer-2 GitHub | Layer-3 instance name in Layer-2 repo — move GUIDE, remove dir |
