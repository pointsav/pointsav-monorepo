---
artifact: brief
name: BRIEF-PPN-DEV-BOOTSTRAP
status: active
created: 2026-05-28
engine: claude-code
session: totebox@project-infrastructure
synthesises:
  - BRIEF-PPN-ARCHITECTURE.md (project-infrastructure)
  - BRIEF-sovereign-os-family-master-plan.md (workspace)
  - BRIEF-totebox-ppn-infrastructure-master-plan.md (workspace; superseded)
  - BRIEF-dev-environment-hardening-2026-05-18.md (workspace)
description: >
  Operational BRIEF for activating the first live PPN ceremony on the existing
  WireGuard mesh. Documents the dogfood principle, current topology, role mapping,
  service deployments, virtualization proof-of-concept, and migration path.
---

# PPN Dev-Environment Bootstrap

**The first PointSav Private Network is the development environment itself.**

---

## 1. The Dogfood Principle

The three machines on the WireGuard mesh Part A-lite are not just connected — they
*are* the first PPN deployment. Building PPN on PPN is intentional: the development
environment is the proving ground. Every piece of software shipped to future customers
runs here first. The iMac runs `os-network-admin`. The GCP VM hosts `service-ppn-pairing`.
Laptop B is the WireGuard routing hub. The first node-join ceremony that runs in this
environment is the proof that the architecture works.

This mirrors the Totebox Orchestration principle: COMMAND runs the same session tooling
that customers will use. The operator using the PPN is the same person building it.

---

## 2. Current Topology — Part A-lite (live as of 2026-05-23)

Ground truth is `infrastructure/wireguard/topology.yaml`. Authoritative peer list:

| Machine | PPN role | WireGuard IP | Target IP (10.42) | Status |
|---|---|---|---|---|
| **Laptop B** | route-network-admin-1 (WireGuard hub) | 10.8.0.1 | 10.42.0.1 | live; 24.86.192.209:51820 |
| **GCP VM** (foundry-workspace) | fleet-infrastructure-cloud-1 | 10.8.0.9 | 10.42.10.1 | live |
| **Laptop A** (iMac / Linux Mint) | station-workplace-mathew-1 | 10.8.0.6 | 10.42.20.2 | live |
| **Jennifer** (macPro) | station-workplace-jennifer-1 | 10.8.0.5 | 10.42.20.1 | pending (key rotated 2026-05-23) |

The three live machines can already reach each other. SSH from Laptop A:
```
ssh mathew@10.8.0.9    # GCP VM
ssh mathew@10.8.0.1    # Laptop B
```

---

## 3. Role Mapping — Dev Environment as PPN Deployment

Two-layer architecture per BRIEF-sovereign-os-family-master-plan.md:

```
TOTEBOX ORCHESTRATION LAYER (MBA pairings)
──────────────────────────────────────────
  gateway-orchestration-command-1 [GCP VM — co-tenant with fleet-cloud-1]
    └── cluster-totebox-* / media-* / vault-privategit-* / node-console-*

POINTSAV PRIVATE NETWORK LAYER (WireGuard)
──────────────────────────────────────────
  route-network-admin-1 [Laptop B — WireGuard hub]
    ├── fleet-infrastructure-cloud-1 [GCP VM] (10.8.0.9 → target 10.42.10.1)
    └── station-workplace-mathew-1  [Laptop A] (10.8.0.6 → target 10.42.20.2)
```

**`os-network-admin` runs on Laptop A.** It is the operator's admin surface for the PPN
layer. It polls `service-ppn-pairing` on the GCP VM and shows pending node-join requests.
The operator approves new nodes from Laptop A; approved WireGuard public keys are written
to `nodes.jsonl` on the GCP VM.

**Governance gap (from sovereign-os-family master plan §4):** `route-network-admin-1` is
the only sovereign component currently absent from `pairings.yaml`. The WireGuard hub has
no MBA pairing to the orchestration gateway — it is governance-orphaned. This must be
closed before Part A proper.

---

## 4. What service-ppn-pairing Provides

Crate: `service-ppn-pairing/` (committed 2026-05-27, deployed as `local-ppn-pairing.service`)
Default listen: `0.0.0.0:9202` (accessible from all WireGuard peers)
Database: `~/.local/share/ppn/ppn-pairing.db` (SQLite; auto-migrates)
Registry: `~/.local/share/ppn/nodes.jsonl` (append-only; one JSON line per approved node)

API:

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/v1/node-join/request` | Node submits node_id + wireguard_pubkey + bottom + arch; gets back code + expires_at |
| `GET` | `/v1/node-join/pending` | Operator polls for pending approvals |
| `POST` | `/v1/node-join/approve` | Operator approves a code; writes to nodes.jsonl |
| `POST` | `/v1/node-join/deny` | Operator denies a code |
| `GET` | `/v1/node-join/status/{id}` | Node polls its own request state |

Code format: Crockford base32 XXXX-XXXX (40-bit entropy, 600s TTL). Confusable characters
normalised: I→1, L→1, O→0.

When a node is approved, `register_approved_node()` appends to `nodes.jsonl`:
```json
{"node_id":"...","wireguard_pubkey":"...","bottom":"netbsd-compat","arch":"x86_64","request_id":"...","approved_at":"..."}
```
This is the Phase 1 registry. Phase 2 (Genesis Protocol) will read `nodes.jsonl` and
generate WireGuard peer entries automatically.

---

## 5. os-network-admin on Laptop A

Crate: `os-network-admin/` — minimal poll mode (deferred: ratatui TUI with QR display)

Current implementation: polls `$PAIRING_SERVER/v1/node-join/pending` every 5 seconds;
prints pending requests to stdout. Operator approves via curl.

```bash
# On Laptop A, after copying the binary:
PAIRING_SERVER=http://10.8.0.9:9202 ./os-network-admin
```

To approve a pending node-join (from any WireGuard peer):
```bash
curl -s -X POST http://10.8.0.9:9202/v1/node-join/approve \
     -H 'Content-Type: application/json' \
     -d '{"code":"XXXX-XXXX"}'
```

**Deferred (NEXT.md):** ratatui TUI with keyboard approve/deny (`a`/`d`), QR rendering
via `system-pairing-codes::qr_unicode`, expiry countdown. This is the full §9.2 Step 4
of the BRIEF-PPN-ARCHITECTURE.md build order.

---

## 6. Virtualization Proof-of-Concept

The thesis claim (BRIEF-PPN-ARCHITECTURE.md §1.1 Contribution #3): "sub-five-minute SMB
deployment" and "formally-isolated VMs." The development environment proves the concept
using KVM (where available) as the compat-bottom stand-in.

**TCB delta — what each hypervisor proves:**

| Hypervisor | Isolation claim | Formal proof | Status |
|---|---|---|---|
| QEMU/KVM (Linux) | Process isolation + EPT | None | **Used for proof today on Laptop A** |
| NetBSD/bhyve | EPT isolation; VeriExec load-time integrity | Argued (not proved) | compat bottom target |
| seL4 (AArch64) | Machine-checked IFC; intransitive non-interference | Murray et al. 2013 | native bottom target (moonshot-kernel) |

**KVM availability:**
- **Laptop A (iMac / Intel Sandy Bridge)**: VT-x present; KVM available on Linux Mint
  (`/dev/kvm` expected). Use `vm-prove.sh` without `--tcg`.
- **GCP VM (foundry-workspace)**: runs AS a KVM guest; nested virtualisation NOT enabled
  by default on e2 instances. Use `vm-prove.sh --tcg` (QEMU TCG, slower but proves the
  concept). To enable KVM on the GCP VM: stop instance → `gcloud compute instances update
  foundry-workspace --enable-nested-virtualization` → restart.

**Proof script:** `infrastructure/virt/vm-prove.sh`

What it does:
1. Detects KVM availability (`/dev/kvm`)
2. Downloads Alpine Linux virt ISO (~50 MB, single download cached in `work/`)
3. Boots a 256 MB VM with virtio NIC
4. Forwards host port 10202 → VM port 9202
5. Operator installs `service-ppn-pairing` binary in the VM, starts it
6. From Laptop A: `curl http://10.8.0.9:10202/v1/node-join/pending` — proves a
   Totebox service is reachable inside a VM through the PPN mesh

This is "Totebox Orchestration using a VM": the VM IS the cluster-totebox host.
When `os-infrastructure` compiles, KVM is replaced by the bhyve/seL4 hypervisor.
The ceremony and mesh provisioning are unchanged.

---

## 7. Deployment Sequence (unblocked right now)

### Step 1 — Deploy service-ppn-pairing on GCP VM

```bash
# On GCP VM (10.8.0.9):
cd /srv/foundry/clones/project-infrastructure
cargo build --release -p service-ppn-pairing
sudo cp target/release/service-ppn-pairing /usr/local/bin/
sudo cp infrastructure/systemd/local-ppn-pairing.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-ppn-pairing
systemctl status local-ppn-pairing
```

### Step 2 — Verify from Laptop A

```bash
# From Laptop A (10.8.0.6) over WireGuard:
curl http://10.8.0.9:9202/v1/node-join/pending
# Expected: {"pending":[]}
```

### Step 3 — Build and copy os-network-admin to Laptop A

```bash
# On GCP VM:
cargo build --release -p os-network-admin
scp target/release/os-network-admin mathew@10.8.0.6:~/bin/
```

### Step 4 — Run os-network-admin on Laptop A

```bash
# On Laptop A:
PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin
```

### Step 5 (optional proof) — Run vm-prove.sh on Laptop A

```bash
# On Laptop A (real hardware, KVM available):
cd /path/to/project-infrastructure
./infrastructure/virt/vm-prove.sh
# Boot Alpine, install service-ppn-pairing in VM, verify port 10202
```

---

## 8. Address Plan Migration Path

The 10.8.0.0/24 range is Part A-lite (manual WireGuard setup, 2026-05-23). The canonical
address plan from BRIEF-sovereign-os-family-master-plan.md §B is **10.42.0.0/16**.

Migration is gated on operator subnet ratification (NEXT.md Q2). When confirmed:

| Step | Change |
|---|---|
| Update topology.yaml | `ppn_ip` → `target_ppn_ip` values; already populated |
| Update Laptop B wg0.conf | New AllowedIPs ranges; regenerate peer configs |
| Update app-network-admin | Replace hardcoded `10.50.0.x` with `10.42.x.x` |
| Update os-network-admin | Replace default `10.8.0.9` with `10.42.10.1` |
| Update INVENTORY.yaml | Fill all node assignments |

The `target_ppn_ip` fields in `topology.yaml` are already correct:
- Laptop B: 10.42.0.1 (routing hub)
- GCP VM: 10.42.10.1 (fleet-cloud)
- Laptop A: 10.42.20.2 (station-workplace-mathew)

---

## 9. Connection to COMMAND's Totebox Transformation

COMMAND is migrating from master/root session roles to Command/Totebox vocabulary (Sessions
8–24, 2026-05-17 to 2026-05-27). The Totebox Orchestration layer (MBA pairings) and the
PPN layer (WireGuard) are the two-layer architecture that governs all production deployments.

**Three open items for COMMAND to close before Part A proper:**

1. **MBA governance gap:** `route-network-admin-1` (Laptop B) is absent from `pairings.yaml`.
   Every other sovereign component has an MBA entry. This must be added as an admin-tier
   commit to `pointsav-monorepo`.

2. **Subnet ratification (Q2):** Confirm 10.42.0.0/16. The 10.8.0.0/24 Part A-lite range
   is a transitional stepping stone — the 10.42.0.0/16 plan is ratified in principle by the
   sovereign-os-family BRIEF; it needs an operator acknowledgement commit.

3. **DNS resolution (Q4):** `network.woodfinegroup.com` DNS record was deleted 2026-05-26.
   The Guide references it. Either restore the record or update the Guide to use a direct IP.

---

## 10. What Remains for Genesis Protocol

This BRIEF describes the "activate what we have" phase. The full Genesis Protocol
(BRIEF-PPN-ARCHITECTURE.md §5) requires:

| Step | Crate | Notes |
|---|---|---|
| 1 | `os-infrastructure` rewrite | blind boot → mDNS → short-code ceremony; **does not compile** |
| 2 | `system-substrate-broadcom` | silicon_ping(); Broadcom 14e4:16b4 PCI detection |
| 3 | `system-network-interface` | WireGuard/mDNS substrate |
| 5 | `app-network-admin` | Replace system-slm subprocess with Doorman HTTP |
| 6 | `app-network-admin` | Replace JSON mesh payloads with 16-byte binary |

Steps 1–3 are gated on Q2 (subnet ratification). Step 5 is gated on Q5 (Doorman deployed).

---

*End of BRIEF — project-infrastructure / 2026-05-28*
*Activating the first ceremony: service-ppn-pairing on GCP VM + os-network-admin on Laptop A*
