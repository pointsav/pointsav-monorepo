#!/usr/bin/env python3
"""
guide_cleanup.py — GUIDE-* content cleanup sweep.

Handles:
1. ~/Foundry/ path fixes (6 woodfine + 2 pointsav files)
2. Stub deployment guide rewrites (cluster-specific content)
3. Other stub rewrites (provision-relay, provision-standalone, etc.)
4. guide-telemetry-governance.md rewrite (was GUIDE_TELEMETRY.md)
5. Footer addition for pointsav-fleet-deployment guides (previously missed by footer sweep)

Modes:
  --dry-run   Show what would change without writing.
  --apply     Write all changes.
"""

import argparse
import os
import re
from pathlib import Path

FOUNDRY = Path("/srv/foundry")
WFD = FOUNDRY / "customer/woodfine-fleet-deployment"
PFD = FOUNDRY / "vendor/pointsav-fleet-deployment"

# ---------------------------------------------------------------------------
# IP footer blocks
# ---------------------------------------------------------------------------

FOOTER_GUIDE = """\
---

*Copyright © 2026 Woodfine Management Corp. All rights reserved.*

*Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. All other trademarks are the property of their respective owners.*"""

# ---------------------------------------------------------------------------
# STUB DEPLOYMENT GUIDE REPLACEMENTS
# The old content block (same in all stub deployment guides in WFD):
# ---------------------------------------------------------------------------

WFD_STUB_OLD = """\
# 🚀 guide-01: Deployment & Synchronization
**Status:** Provisioning Placeholder
**Taxonomy:** Tier-3 Fleet Operations

## 1. Operational Mandate
This file serves as the architectural placeholder for the node-specific deployment sequence.
Detailed physical execution steps will be mathematically locked here during the next engineering cycle."""

# Pointsav stubs are 7-line version (no footer yet):
PFD_STUB_OLD = """\
# 🚀 guide-01: Deployment & Synchronization
**Status:** Provisioning Placeholder
**Taxonomy:** Tier-3 Fleet Operations

## 1. Operational Mandate
This file serves as the architectural placeholder for the node-specific deployment sequence.
Detailed physical execution steps will be mathematically locked here during the next engineering cycle."""


def stub_guide(cluster: str, description: str) -> str:
    return f"""\
# Deployment Guide — {cluster}

{description}

This cluster is in the scaffold phase. Full deployment procedures will be documented when the cluster moves to Active state. For current node configuration and service inventory, refer to `README.md` in this directory.

When this cluster is deployed, this guide will cover: binary installation, systemd unit configuration, environment file setup, and post-deploy smoke verification."""


# Cluster-specific deployment guide content
CLUSTER_DESCRIPTIONS: dict[str, str] = {
    "cluster-totebox-corporate/guide-deployment.md": stub_guide(
        "cluster-totebox-corporate",
        "Covers initial deployment of the Totebox Orchestration cluster for Woodfine Management Corp. corporate operations, including the corporate communications pipeline and ledger services.",
    ),
    "cluster-totebox-personnel/guide-deployment.md": stub_guide(
        "cluster-totebox-personnel",
        "Covers initial deployment of the Totebox Orchestration cluster for personnel operations: HR data ingestion, identity ledger management, and SLM-backed communications processing.",
    ),
    "cluster-totebox-property/guide-deployment.md": stub_guide(
        "cluster-totebox-property",
        "Covers initial deployment of the Totebox Orchestration cluster for real property operations, including the property portfolio ledger and asset data pipeline.",
    ),
    "fleet-infrastructure-cloud/guide-deployment.md": stub_guide(
        "fleet-infrastructure-cloud",
        "Covers initial deployment of the GCP cloud infrastructure nodes that serve as the static, public-facing WireGuard hub for the Woodfine private network.",
    ),
    "fleet-infrastructure-onprem/guide-deployment.md": stub_guide(
        "fleet-infrastructure-onprem",
        "Covers initial deployment of on-premises infrastructure nodes at Woodfine corporate locations, forming the Totebox network fabric on physical hardware.",
    ),
    "gateway-interface-command/guide-deployment.md": stub_guide(
        "gateway-interface-command",
        "Covers initial deployment of the command interface gateway, which serves as the management plane entry point for the Woodfine fleet.",
    ),
    "media-knowledge-corporate/guide-deployment.md": stub_guide(
        "media-knowledge-corporate",
        "Covers initial deployment of the corporate knowledge wiki instance (`app-mediakit-knowledge`), serving internal Woodfine Management Corp. documentation.",
    ),
    "media-knowledge-projects/guide-deployment.md": stub_guide(
        "media-knowledge-projects",
        "Covers initial deployment of the project knowledge wiki instance (`app-mediakit-knowledge`), serving project-specific documentation for active Woodfine operations.",
    ),
    "route-network-admin/guide-deployment.md": stub_guide(
        "route-network-admin",
        "Covers initial deployment of the network routing administration node, which manages the WireGuard mesh topology, cryptographic keys, and subnet routing for the Woodfine private network.",
    ),
    "vault-privategit-source/guide-deployment.md": stub_guide(
        "vault-privategit-source",
        "Covers initial deployment of the source code vault node, providing private Git hosting for Woodfine-specific source code and configuration.",
    ),
}

# media-knowledge-documentation gets a real deployment guide since it's live
MEDIA_DOCS_DEPLOYMENT = """\
# Deployment Guide — media-knowledge-documentation

Covers the initial bring-up of `app-mediakit-knowledge` as the `local-knowledge.service` systemd unit, serving `documentation.pointsav.com`. Initial deployment completed at workspace v0.1.29.

For day-2 operations (content updates, search, service restarts), see `guide-operate-knowledge-wiki.md`.

## Prerequisites

- Node provisioned per `guide-provision-node.md`
- Binary built from `pointsav-monorepo/app-mediakit-knowledge/` and installed to `/usr/local/bin/app-mediakit-knowledge`
- Content tree cloned from `content-wiki-documentation` and available locally
- DNS A record for `documentation.pointsav.com` pointing to the VM public IP
- GCP and OS (ufw) firewalls open on TCP 80 and TCP 443

## 1. System user and state directory

```bash
sudo useradd --system --no-create-home --shell /usr/sbin/nologin local-knowledge
sudo mkdir -p /var/lib/local-knowledge/state
sudo chown -R local-knowledge:local-knowledge /var/lib/local-knowledge
```

## 2. Install and start the systemd unit

The canonical unit is at `vault-privategit-source-1/infrastructure/local-knowledge/local-knowledge.service` on the Foundry workspace VM. Copy it to `/etc/systemd/system/local-knowledge.service`, then:

```bash
sudo systemctl daemon-reload
sudo systemctl enable local-knowledge.service
sudo systemctl start local-knowledge.service
sudo systemctl status local-knowledge.service
```

## 3. TLS certificate

```bash
sudo certbot --nginx -d documentation.pointsav.com
```

Certbot modifies the nginx vhost to add TLS. Renewal is automatic via the certbot systemd timer.

## 4. Smoke test

```bash
curl -s https://documentation.pointsav.com/healthz
```

Expect HTTP 200. The wiki is live. See `guide-operate-knowledge-wiki.md` for ongoing operations."""

CLUSTER_DESCRIPTIONS["media-knowledge-documentation/guide-deployment.md"] = MEDIA_DOCS_DEPLOYMENT

# ---------------------------------------------------------------------------
# SPECIFIC STUB REWRITES (non-deployment stub files)
# ---------------------------------------------------------------------------

# guide-provision-relay.md — the "PPN Cloud Anchor Ignition" stub
PROVISION_RELAY_OLD = """\
# GUIDE: PPN Cloud Anchor Ignition
**Customer:** Woodfine Management Corp.
**Target Environment:** fleet-infrastructure-cloud (GCP Node)
**Operation:** Hub-and-Spoke Network Binding

## 1. Operational Overview
This protocol dictates the application of the `vendor-wireguard` payload to the Tier-2 execution body. The cloud node acts as the static, public-facing anchor for the PointSav Private Network.

*Awaiting exact execution parameters.*"""

PROVISION_RELAY_NEW = """\
# Cloud Relay Provisioning Guide

Covers provisioning the GCP cloud node as the static WireGuard hub for the Woodfine private network. This node has a fixed public IP address and acts as the central relay that all fleet nodes dial into.

This guide is in development. The steps below reflect the provisioning approach and will be filled in as the fleet-infrastructure-cloud cluster moves from Scaffold-coded to Active state.

## Overview

The cloud relay runs WireGuard on the GCP compute instance. Other fleet nodes — on-premises hardware and leased endpoints — dial out to this relay. The relay does not dial in to nodes; all traffic initiates from the nodes.

For WireGuard mesh key generation and subnet assignment, see `route-network-admin/guide-mesh-orchestration.md`.

## Prerequisites

- GCP compute instance provisioned with a static external IP
- GCP firewall rule allowing UDP on the WireGuard port (default: 51820) from all sources
- WireGuard installed: `sudo apt-get install wireguard`

## Steps

Steps to be documented when exact network parameters are ratified (IP range, port, peer list)."""

# guide-provision-standalone.md stub
PROVISION_STANDALONE_OLD = """\
# GUIDE: Standalone Node Provisioning
**Customer:** Woodfine Management Corp.
**Target Environment:** fleet-infrastructure-leased

## 1. Operational Overview
This protocol outlines the steps to bring up a leased standalone node into the Woodfine fleet network.

*Awaiting exact execution parameters.*"""

# Read the actual file to check
PROVISION_STANDALONE_NEW = """\
# Standalone Node Provisioning Guide

Covers provisioning a leased standalone node (MacBook or other portable hardware) into the Woodfine fleet network via WireGuard. For endpoint-specific configuration (macOS), see `spoke-configs/guide-macos-endpoints.md`.

This guide is in development. Steps below outline the provisioning sequence; exact parameters will be documented when the fleet-infrastructure-leased cluster moves to Active state.

## Prerequisites

- Node hardware available with network connectivity
- WireGuard peer configuration generated by `route-network-admin/guide-mesh-orchestration.md`
- SSH access to the node

## Steps

Steps to be documented when network parameters and node inventory are ratified."""

# guide-provision-onprem.md stub
PROVISION_ONPREM_OLD_1 = "Awaiting exact execution parameters."

# guide-mesh-orchestration.md — "PPN Orchestration" stub
MESH_ORCH_OLD = """\
# GUIDE: PointSav Private Network (PPN) Orchestration
**Customer:** Woodfine Management Corp.
**Target Environment:** route-network-admin (iMac Command Authority)
**Operation:** Cryptographic Key Generation & Mesh Subnetting

## 1. Operational Overview
This protocol dictates the generation of the `10.x.x.x` subnet routing tables and the physical Ed25519 cryptographic keys required to fuse `fleet-infrastructure-onprem` (Laptop A) and `fleet-infrastructure-cloud` (GCP) into a sovereign mesh.

*Awaiting exact execution parameters.*"""

MESH_ORCH_NEW = """\
# Mesh Network Orchestration Guide

Covers generating WireGuard key pairs and subnet routing tables for the Woodfine private network. The network admin node (`route-network-admin`) holds the master cryptographic keys and authoritative subnet assignments.

This guide is in development. The steps below reflect the design intent; exact IP ranges and peer list will be documented when the route-network-admin cluster moves to Active state.

## Network topology

The Woodfine private network uses a hub-and-spoke topology with a GCP cloud relay as the hub:

| Node | Role | WireGuard endpoint |
|---|---|---|
| `fleet-infrastructure-cloud` | Hub (static public IP) | `<cloud-ip>:51820` |
| `fleet-infrastructure-onprem` | Spoke (on-premises iMac) | dials cloud relay |
| `fleet-infrastructure-leased` | Spoke (laptop endpoints) | dials cloud relay |

## Key generation

WireGuard uses Curve25519 key pairs. Generate a pair per node:

```bash
wg genkey | tee privatekey | wg pubkey > publickey
```

Store private keys securely on each respective node. Collect public keys centrally on the network admin node to build the peer configuration.

## Subnet assignment

The mesh uses the `10.x.x.x/24` range. Assign one IP per node. Document the assignment in `INVENTORY.yaml` at the repository root.

## Steps

Full configuration files (`wg0.conf` per node) to be documented when exact IP ranges and all peer public keys are ratified."""

# guide-01-deployment.md stub (was GUIDE-01-Deployment.md in service-slm)
SLM_DEPLOY_OLD = """\
# 🚀 guide-01: Deployment & Synchronization
**Status:** Provisioning Placeholder
**Taxonomy:** Tier-3 Fleet Operations

## 1. Operational Mandate
This file serves as the architectural placeholder for the node-specific deployment sequence.
Detailed physical execution steps will be mathematically locked here during the next engineering cycle."""

SLM_DEPLOY_NEW = """\
# SLM Deployment Guide — cluster-totebox-personnel

Covers deploying the `service-slm` inference stack within the personnel cluster, including the Doorman routing service and the local Tier A backend.

This guide is in development. For the full Doorman operational runbook, see `vault-privategit-source/guide-doorman.md`. For the Tier A TUI, see `vault-privategit-source/guide-tier-a-sysadmin-tui.md`.

## Prerequisites

- Personnel cluster node provisioned
- `pointsav-monorepo` source available (`pointsav-monorepo/service-slm/`)
- Rust toolchain installed

## Steps

Detailed deployment steps to be documented when the personnel cluster SLM configuration is ratified."""

# guide-telemetry-governance.md (was GUIDE_TELEMETRY.md)
TELEMETRY_GOV_CONTENT = """\
# Telemetry Governance Guide — media-marketing-landing

Covers the data governance posture for the Woodfine Management Corp. marketing landing surface. All telemetry collected through this surface is processed locally under the Totebox Orchestration architecture — no data is routed to third-party analytics platforms.

## Architecture

Visitor interactions with the marketing landing are routed to the `os-totebox` local processing stack. The `os-mediakit` component handles ingestion and outputs structured records to the deployment's outbox directories for internal audit.

## Accessing telemetry

Telemetry extraction runs on the network admin node. The extraction procedure will be documented in this guide when the fleet reaches Active state. For current telemetry operations (service status, log inspection), see `guide-telemetry-operations.md`.

## Data sovereignty note

No visitor data is transmitted to external cloud services. The processing pipeline is physically isolated on Woodfine-owned hardware. This posture supports Woodfine Management Corp.'s customer-data privacy commitments."""

# ---------------------------------------------------------------------------
# ~/Foundry/ PATH FIXES — search/replace tuples per file
# ---------------------------------------------------------------------------

PATH_FIXES: dict[str, list[tuple[str, str]]] = {
    # woodfine
    "vault-privategit-source/guide-doorman.md": [
        # Source path in cluster clone
        (
            "Source lives on the project-slm cluster clone at:\n```\n~/Foundry/clones/project-slm/pointsav-monorepo/service-slm/crates/slm-doorman-server/\n```",
            "Source code: `pointsav-monorepo/service-slm/crates/slm-doorman-server/`\n(public: `github.com/pointsav/pointsav-monorepo`; on the Foundry workspace VM at `clones/project-slm/pointsav-monorepo/`)",
        ),
        # Upgrade procedure — promote.sh
        (
            "~/Foundry/bin/promote.sh --cluster project-slm --target pointsav-monorepo",
            "bin/promote.sh --cluster project-slm --target pointsav-monorepo  # on the Foundry workspace VM",
        ),
        # Upgrade procedure — cd clone
        (
            "cd ~/Foundry/clones/<canonical-clone>/pointsav-monorepo/service-slm",
            "cd <pointsav-monorepo-clone>/service-slm",
        ),
        # References section — conventions
        (
            "- `~/Foundry/conventions/four-tier-slm-substrate.md` — Tier 0/1/2/3 ladder",
            "- `[[four-tier-slm-substrate]]` — Tier 0/1/2/3 ladder",
        ),
        (
            "- `~/Foundry/conventions/api-key-boundary-discipline.md` — keys at the gateway only",
            "- `[[api-key-boundary-discipline]]` — keys at the gateway only",
        ),
        (
            "- `~/Foundry/conventions/apprenticeship-substrate.md` — brief / verdict / shadow",
            "- `[[apprenticeship-substrate]]` — brief / verdict / shadow",
        ),
        (
            "- `~/Foundry/conventions/three-ring-architecture.md` — Ring 1/2/3 boundaries",
            "- `[[three-ring-architecture]]` — Ring 1/2/3 boundaries",
        ),
        (
            "- `~/Foundry/conventions/adapter-composition.md` — request-time adapter algebra",
            "- `[[adapter-composition]]` — request-time adapter algebra",
        ),
        # References section — infrastructure (Foundry workspace VM)
        (
            "- `~/Foundry/infrastructure/local-doorman/README.md` — endpoint surface, on-disk layout, status",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/local-doorman/README.md` — endpoint surface, on-disk layout, status",
        ),
        (
            "- `~/Foundry/infrastructure/local-doorman/local-doorman.service` — canonical systemd unit",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/local-doorman/local-doorman.service` — canonical systemd unit",
        ),
        (
            "- `~/Foundry/infrastructure/local-doorman/bootstrap.sh` — install procedure",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/local-doorman/bootstrap.sh` — install procedure",
        ),
        (
            "- `~/Foundry/infrastructure/local-slm/` — Tier A backend (OLMo 7B Q4)",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/local-slm/` — Tier A backend (OLMo 7B Q4)",
        ),
        (
            "- `~/Foundry/infrastructure/yoyo-manual/README.md` — Tier B operator-presence runbook",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/yoyo-manual/README.md` — Tier B operator-presence runbook",
        ),
        (
            "- `~/Foundry/infrastructure/slm-yoyo/CONTRACT.md` — Tier B wire contract",
            "- on the Foundry workspace VM: `vault-privategit-source-1/infrastructure/slm-yoyo/CONTRACT.md` — Tier B wire contract",
        ),
        # References section — tools (bin/)
        (
            "- `~/Foundry/bin/post-impl-brief-queue.sh` — §7C ship sequence (Stage-6 + build + install + restart + smoke + diff-preview)",
            "- on the Foundry workspace VM: `bin/post-impl-brief-queue.sh` — §7C ship sequence (Stage-6 + build + install + restart + smoke + diff-preview)",
        ),
        (
            "- `~/Foundry/bin/promote.sh` — Stage-6 staging-tier → canonical promotion",
            "- on the Foundry workspace VM: `bin/promote.sh` — Stage-6 staging-tier → canonical promotion",
        ),
        (
            "- `~/Foundry/bin/capture-edit.py` — post-commit shadow brief emitter (workspace v0.1.83+ writes to queue, not HTTP)",
            "- on the Foundry workspace VM: `bin/capture-edit.py` — post-commit shadow brief emitter",
        ),
        # References section — source (cluster clone → canonical repo)
        (
            "- `~/Foundry/clones/project-slm/pointsav-monorepo/service-slm/crates/slm-doorman-server/` — Doorman crate source",
            "- `pointsav-monorepo/service-slm/crates/slm-doorman-server/` — Doorman crate source",
        ),
        (
            "- `~/Foundry/clones/project-slm/pointsav-monorepo/service-slm/ARCHITECTURE.md` — per-project architecture",
            "- `pointsav-monorepo/service-slm/ARCHITECTURE.md` — per-project architecture",
        ),
        # References / citations
        (
            "Per `~/Foundry/citations.yaml`:",
            "Per the Foundry workspace citations registry (`vault-privategit-source-1/citations.yaml` on the workspace VM):",
        ),
    ],
    "vault-privategit-source/guide-tier-a-sysadmin-tui.md": [
        (
            "`~/Foundry/conventions/substrate-without-inference-base-case.md`, your Totebox",
            "`[[substrate-without-inference-base-case]]`, your Totebox",
        ),
        (
            "(`~/Foundry/.claude/outbox.md` if you operate the workspace dogfood) or",
            "(the Master outbox on the Foundry workspace VM, if you operate the workspace) or",
        ),
    ],
    "vault-privategit-source/guide-operating-yoyo.md": [
        (
            "surface via `~/Foundry/.claude/inbox.md` (Master inbox) so the next workspace session can address it.",
            "surface via the Master inbox (on the Foundry workspace VM) so the next workspace session can address it.",
        ),
    ],
    "media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md": [
        (
            "The deployment instance at `~/Foundry/deployments/media-knowledge-documentation-1/` runs",
            "The deployment instance (on the Foundry workspace VM at `deployments/media-knowledge-documentation-1/`) runs",
        ),
        (
            "   ~/Foundry/bin/commit-as-next.sh \"rotate featured pin: <slug> — <one-line rationale>\"",
            "   bin/commit-as-next.sh \"rotate featured pin: <slug> — <one-line rationale>\"  # on the Foundry workspace VM",
        ),
    ],
    "media-knowledge-documentation/guide-operate-knowledge-wiki.md": [
        (
            "| systemd unit (IaC) | `~/Foundry/infrastructure/local-knowledge/local-knowledge.service` | version-controlled |",
            "| systemd unit (IaC) | `vault-privategit-source-1/infrastructure/local-knowledge/local-knowledge.service` (on the Foundry workspace VM) | version-controlled |",
        ),
        (
            "The unit, binary, citations registry, and IaC are version-controlled in `~/Foundry`. The content tree is version-controlled in `~/Foundry/clones/project-knowledge/content-wiki-documentation/` (or whichever subdirectory is named in `--content-dir`). State directory contents are non-canonical — the Tantivy search index rebuilds on startup from the content tree, so wiping state is non-destructive.",
            "The unit, binary, citations registry, and IaC are version-controlled in the Foundry workspace VM (`vault-privategit-source-1/`). The content tree is version-controlled in `content-wiki-documentation` (the path configured in `--content-dir`). State directory contents are non-canonical — the Tantivy search index rebuilds on startup from the content tree, so wiping state is non-destructive.",
        ),
        (
            "To edit the unit, edit the IaC copy at `~/Foundry/infrastructure/local-knowledge/local-knowledge.service`, then propagate to `/etc/systemd/system/local-knowledge.service`, then reload and restart:",
            "To edit the unit, edit the IaC copy at `vault-privategit-source-1/infrastructure/local-knowledge/local-knowledge.service` (on the Foundry workspace VM), then propagate to `/etc/systemd/system/local-knowledge.service`, then reload and restart:",
        ),
    ],
}

# pointsav-fleet-deployment path fixes (same guide text, same fixes)
PATH_FIXES_PFD: dict[str, list[tuple[str, str]]] = {
    "media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md": PATH_FIXES[
        "media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md"
    ],
    "media-knowledge-documentation/guide-operate-knowledge-wiki.md": PATH_FIXES[
        "media-knowledge-documentation/guide-operate-knowledge-wiki.md"
    ],
}

# ---------------------------------------------------------------------------
# Pointsav fleet deployment guide footer additions
# ---------------------------------------------------------------------------

POINTSAV_GUIDE_FILES = [
    "media-knowledge-distribution/guide-deployment.md",
    "media-knowledge-distribution/guide-provision-node.md",
    "media-knowledge-documentation/guide-deployment.md",
    "media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md",
    "media-knowledge-documentation/guide-operate-knowledge-wiki.md",
    "media-knowledge-documentation/guide-provision-node.md",
    "media-marketing-landing/guide-telemetry-integration.md",
    "media-marketing-landing/guide-telemetry-operations.md",
    "vault-privategit-design-system/guide-deployment.md",
    "vault-privategit-design-system/guide-provision-node.md",
    "vault-privategit-source/guide-deployment.md",
    "vault-privategit-source/guide-provision-node.md",
]

COPYRIGHT_RE = re.compile(r"^\*Copyright © 2026 Woodfine", re.MULTILINE)


def strip_old_footer(text: str) -> str:
    m = COPYRIGHT_RE.search(text)
    if not m:
        return text
    before = text[: m.start()].rstrip()
    if before.endswith("---"):
        before = before[:-3].rstrip()
    return before


def apply_path_fixes(text: str, fixes: list[tuple[str, str]]) -> str:
    for old, new in fixes:
        text = text.replace(old, new)
    return text


def process_wfd_stub(rel_path: str, dry: bool, verbose: bool) -> bool:
    """Rewrite a stub deployment/provision guide. Returns True if changed."""
    abs_path = WFD / rel_path
    if not abs_path.exists():
        return False

    original = abs_path.read_text("utf-8")

    # Determine new body
    new_body = None

    if rel_path in CLUSTER_DESCRIPTIONS:
        if "mathematically locked here during the next engineering cycle" in original:
            new_body = CLUSTER_DESCRIPTIONS[rel_path]

    elif rel_path == "fleet-infrastructure-cloud/guide-provision-relay.md":
        if PROVISION_RELAY_OLD in original:
            new_body = PROVISION_RELAY_NEW

    elif rel_path == "fleet-infrastructure-leased/guide-provision-standalone.md":
        # Read to check current content
        if "Awaiting exact execution parameters" in original:
            new_body = PROVISION_STANDALONE_NEW

    elif rel_path == "route-network-admin/guide-mesh-orchestration.md":
        if MESH_ORCH_OLD in original:
            new_body = MESH_ORCH_NEW

    elif rel_path == "cluster-totebox-personnel/service-slm/guide-01-deployment.md":
        if "Point-in-Time Execution" in original or "mathematically locked" in original or "Provisioning Placeholder" in original:
            new_body = SLM_DEPLOY_NEW

    elif rel_path == "media-marketing-landing/guide-telemetry-governance.md":
        # Full rewrite
        new_body = TELEMETRY_GOV_CONTENT

    if new_body is None:
        return False

    # Strip existing footer, attach canonical footer
    new_text = new_body.rstrip() + "\n\n" + FOOTER_GUIDE + "\n"
    changed = original != new_text

    if verbose and changed:
        print(f"  STUB  {rel_path}")
    if changed and not dry:
        abs_path.write_text(new_text, "utf-8")
    return changed


def process_path_fixes_wfd(rel_path: str, dry: bool, verbose: bool) -> bool:
    abs_path = WFD / rel_path
    if not abs_path.exists():
        return False
    original = abs_path.read_text("utf-8")
    fixes = PATH_FIXES.get(rel_path, [])
    new_text = apply_path_fixes(original, fixes)
    changed = original != new_text
    if verbose and changed:
        count = sum(1 for old, _ in fixes if old in original)
        print(f"  PATHS ({count} fixes)  {rel_path}")
    if changed and not dry:
        abs_path.write_text(new_text, "utf-8")
    return changed


def process_path_fixes_pfd(rel_path: str, dry: bool, verbose: bool) -> bool:
    abs_path = PFD / rel_path
    if not abs_path.exists():
        return False
    original = abs_path.read_text("utf-8")
    fixes = PATH_FIXES_PFD.get(rel_path, [])
    new_text = apply_path_fixes(original, fixes)
    changed = original != new_text
    if verbose and changed:
        count = sum(1 for old, _ in fixes if old in original)
        print(f"  PATHS ({count} fixes)  {rel_path}")
    if changed and not dry:
        abs_path.write_text(new_text, "utf-8")
    return changed


def process_pfd_footer(rel_path: str, dry: bool, verbose: bool) -> bool:
    """Add footer to pointsav-fleet-deployment guide if missing."""
    abs_path = PFD / rel_path
    if not abs_path.exists():
        return False
    original = abs_path.read_text("utf-8")
    if COPYRIGHT_RE.search(original):
        return False  # already has footer
    body = original.rstrip()
    new_text = body + "\n\n" + FOOTER_GUIDE + "\n"
    changed = original != new_text
    if verbose and changed:
        print(f"  FOOTER  {rel_path}")
    if changed and not dry:
        abs_path.write_text(new_text, "utf-8")
    return changed


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--apply", action="store_true")
    args = parser.parse_args()

    dry = args.dry_run
    verbose = True
    changed_total = 0

    print("=== WFD stub rewrites ===")
    wfd_stubs = list(CLUSTER_DESCRIPTIONS.keys()) + [
        "fleet-infrastructure-cloud/guide-provision-relay.md",
        "fleet-infrastructure-leased/guide-provision-standalone.md",
        "route-network-admin/guide-mesh-orchestration.md",
        "cluster-totebox-personnel/service-slm/guide-01-deployment.md",
        "media-marketing-landing/guide-telemetry-governance.md",
    ]
    for rel in sorted(wfd_stubs):
        if process_wfd_stub(rel, dry, verbose):
            changed_total += 1

    print()
    print("=== WFD ~/Foundry/ path fixes ===")
    for rel in sorted(PATH_FIXES.keys()):
        if process_path_fixes_wfd(rel, dry, verbose):
            changed_total += 1

    print()
    print("=== PFD ~/Foundry/ path fixes ===")
    for rel in sorted(PATH_FIXES_PFD.keys()):
        if process_path_fixes_pfd(rel, dry, verbose):
            changed_total += 1

    print()
    print("=== PFD footer additions ===")
    for rel in POINTSAV_GUIDE_FILES:
        if process_pfd_footer(rel, dry, verbose):
            changed_total += 1

    print()
    print(f"Total files {'that would change' if dry else 'changed'}: {changed_total}")
    if dry:
        print("Run with --apply to write.")


if __name__ == "__main__":
    main()
