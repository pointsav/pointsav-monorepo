---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: woodfine-fleet-deployment
target_path: fleet-infrastructure/
target_filename: guide-vm-prove-balloon-demo.md
audience: operators verifying the PPN hypervisor layer and demonstrating resource pool management
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - infrastructure/virt/vm-prove.sh
  - architecture/ppn-hypervisor-resource-pool.md
  - briefs/BRIEF-PPN-DEV-BOOTSTRAP.md
notes_for_editor: >
  New guide — targets woodfine-fleet-deployment/fleet-infrastructure/. Covers running
  vm-prove.sh and demonstrating virtio_balloon resource pool management from the QEMU
  monitor. The balloon controller (automated inflation/deflation based on demand signals)
  is a planned milestone — described in planned/intended language per BCSC posture.
  The guide explicitly states what this demo does and does not prove.
  Article frontmatter to add on commit: title "VM Prove and Balloon Demo",
  category "operations", status "active", quality "review".
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
open_question_1: >
  KVM nested virtualisation on GCP VM: `--enable-nested-virtualization` requires stopping
  the instance. The guide includes this step. Operator should confirm instance name and
  zone before running. Current dev topology uses Laptop A (has hardware VT-x) as the
  preferred KVM host for this demo.
---

# VM Prove and Balloon Demo

This guide demonstrates two things:

1. **The PPN hypervisor layer works:** the `vm-prove.sh` script boots an Alpine Linux
   virtual machine under QEMU, with port forwarding that would allow Totebox services to
   run inside the VM and be reachable over the PPN mesh.
2. **Resource pool management works:** the `virtio_balloon` device installed in the VM
   can be inflated and deflated from the QEMU monitor, proving that the hypervisor can
   reclaim and return RAM from a running guest on demand.

This is a **proof-of-concept demonstration**. It uses a test VM (Alpine Linux), not a
production Totebox Archive image. The full hypervisor integration — the automated balloon
controller that responds to fleet demand signals — is a planned milestone in
`os-infrastructure`.

---

## Prerequisites

- `qemu-system-x86_64` and `qemu-img` installed.
  - On Laptop A (macOS/Debian): `apt install -y qemu-system-x86 qemu-utils`
  - On GCP VM: same; or enable nested virtualisation first (see below)
- Internet access to download the Alpine Linux ISO on first run (~50 MB, cached after that)
- The `project-infrastructure` monorepo checked out with `infrastructure/virt/vm-prove.sh`

**Checking KVM availability:**

```bash
ls -la /dev/kvm
```

If `/dev/kvm` exists, KVM hardware acceleration is available (Laptop A with Intel VT-x).
If absent, the script falls back to QEMU TCG software emulation — slower but functionally
equivalent for this demonstration.

---

## Optional: Enable nested virtualisation on GCP VM

The GCP VM (`foundry-workspace`) does not have nested virtualisation enabled by default.
To run VMs on the GCP VM with KVM acceleration:

```bash
# Stop the instance first:
gcloud compute instances stop foundry-workspace --zone=<your-zone>

# Enable nested virtualisation:
gcloud compute instances update foundry-workspace \
    --zone=<your-zone> \
    --enable-nested-virtualization

# Start the instance:
gcloud compute instances start foundry-workspace --zone=<your-zone>
```

If you prefer not to stop the instance, use the `--tcg` flag to run without KVM (see below).

---

## Running the VM proof

Navigate to the project root and run the script:

```bash
# Auto-detect KVM (preferred on Laptop A):
./infrastructure/virt/vm-prove.sh

# Force TCG (no KVM, for GCP VM without nested virt):
./infrastructure/virt/vm-prove.sh --tcg
```

The script will:

1. Download the Alpine Linux virt ISO if not cached (`work/alpine-virt-3.20.0-x86_64.iso`, ~50 MB)
2. Create a 512 MB QCOW2 disk image (`work/ppn-prove.qcow2`) if it does not exist
3. Boot the VM with:
   - 256 MB RAM
   - 1 vCPU
   - `virtio-net-pci` network adapter with port forwarding:
     - `localhost:10022` → VM port 22 (SSH)
     - `localhost:10202` → VM port 9202 (service-ppn-pairing)
   - **`virtio-balloon` device** (this is the resource pool management hook)
   - Shared host filesystem (`virtfs`)

When the boot prompt appears, log in as `root` (no password in the Alpine virt image).

---

## Demonstrating balloon resource pool management

The `virtio_balloon` device is installed in the VM. From the **QEMU monitor**, the
hypervisor can reclaim RAM from the guest or return it on demand.

**Enter the QEMU monitor** (while the VM is running):

Press `Ctrl-A` then `C`. The prompt changes from the VM console to:

```
(qemu)
```

**Check current guest-visible RAM:**

```
(qemu) info balloon
balloon: actual=256
```

`actual=256` means the guest currently sees 256 MB — its full allocation.

**Inflate the balloon (reclaim memory):**

```
(qemu) balloon 128
```

This instructs the balloon driver inside the guest to allocate 128 MB of pages inside the
guest's address space, removing them from the guest's usable memory and making them
available to the host node pool.

**Confirm the reclaim:**

```
(qemu) info balloon
balloon: actual=128
```

The guest now sees 128 MB. The hypervisor has reclaimed 128 MB into the node pool. If
other VMs were running on this node, the reclaimed pages would be available to them.

**Deflate the balloon (return memory):**

```
(qemu) balloon 256
(qemu) info balloon
balloon: actual=256
```

The guest is back to 256 MB. The pool has shrunk by the returned pages.

**Exit the QEMU monitor and return to the VM console:**

Press `Ctrl-A` then `C` again (toggles back to console).

---

## Pool formula

What you just demonstrated is the live version of:

```
pool_available = physical_ram − Σ(balloon_minimum across all VMs)
```

By inflating the balloon, you moved pages from the guest's allocation into the pool.
By deflating, you returned them. In the planned production configuration, the
`os-infrastructure` balloon controller will do this automatically based on demand signals
from running workloads — a VM running an active inference workload receives more RAM; an
idle archive VM gives RAM back to the pool.

---

## Shutting down the VM

From the VM console (not the QEMU monitor):

```sh
shutdown -h now
```

Or, from the QEMU monitor (`Ctrl-A C`):

```
(qemu) quit
```

The disk image (`work/ppn-prove.qcow2`) is preserved. The next run of `vm-prove.sh` will
reuse the same disk image and skip the download step.

---

## What this proves

| Proved | Description |
|---|---|
| ✅ QEMU/KVM hypervisor layer | The PPN can boot and host a VM |
| ✅ `virtio_balloon` device | Balloon driver installs correctly in guest |
| ✅ Memory reclaim | Hypervisor can inflate balloon and recover guest RAM |
| ✅ Memory return | Hypervisor can deflate balloon and give RAM back to guest |
| ✅ Port forwarding | Service inside VM is reachable via PPN-forwarded ports |

## What this does not yet prove

| Not yet proved | Planned milestone |
|---|---|
| Automated balloon controller | `os-infrastructure` balloon controller (future milestone) |
| PSP protocol queries | Totebox Archive data access via PointSav Protocol |
| MBA pairing | Machine-Based Authorization for archive access |
| os-infrastructure Genesis Protocol | `os-infrastructure` code rewrite (gated on Q2–Q6) |
| Production Totebox Archive image | Real `os-totebox` disk image (not Alpine Linux) |

---

## Troubleshooting

| Symptom | Likely cause | Resolution |
|---|---|---|
| Script exits: `Could not access KVM kernel module` | KVM not enabled | Use `--tcg` flag or enable nested virt on GCP |
| Script hangs at ISO download | No internet access on build machine | Download manually and place at `work/alpine-virt-3.20.0-x86_64.iso` |
| `balloon 128` has no effect | `virtio_balloon` module not loaded in guest | Run `modprobe virtio_balloon` inside the Alpine VM first |
| `info balloon` returns `balloon: actual=256` after inflation | Guest balloon driver not responding | Wait a few seconds; inflation is not instantaneous |
| Port 10022 / 10202 already in use | Another QEMU instance running | Kill previous QEMU process: `pkill qemu-system-x86` |
