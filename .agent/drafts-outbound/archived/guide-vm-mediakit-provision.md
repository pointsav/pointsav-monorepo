# Provision vm-mediakit

This guide covers provisioning the `vm-mediakit` Ubuntu 24.04 guest VM on a GCP workspace
host. The VM runs the MediaKit service surface — wikis, marketing sites, proofreader, and
BIM orchestration — in isolation from the host source vault.

Run this guide once to create the VM. To migrate services into the running VM, see
`guide-vm-mediakit-service-migration.md`.

---

## Prerequisites

All of the following must be true before running the provisioning script.

**On the GCP workspace host:**

```bash
# Install QEMU, genisoimage, and socat
sudo apt-get install -y qemu-system-x86 genisoimage socat

# Confirm QEMU is available
qemu-system-x86_64 --version   # should print version string
```

**In the project-infrastructure archive:**

```bash
ls infrastructure/virt/work/foundry-vm-key   # SSH key must exist
```

If the key is missing, generate it:

```bash
ssh-keygen -t ed25519 -f infrastructure/virt/work/foundry-vm-key -N "" -C "foundry@vm-mediakit"
```

The public key `infrastructure/virt/work/foundry-vm-key.pub` is embedded in
`infrastructure/virt/cloud-init-mediakit/user-data`. If you regenerate the key, update
that file before provisioning.

**Disk space:** the script downloads a ~630 MB Ubuntu 24.04 cloud image and creates a
20 GB QCOW2 disk. Ensure at least 25 GB free in the `infrastructure/virt/work/` directory.

---

## Why Ubuntu 24.04 (not Debian 12)

All service binaries on the GCP host are compiled against glibc 2.39 (Ubuntu 24.04).
Debian 12 provides only glibc 2.36. Loading a glibc 2.39 binary on Debian 12 produces
a load-time failure:

```
/lib/x86_64-linux-gnu/libc.so.6: version 'GLIBC_2.39' not found
```

Ubuntu 24.04 is the required guest OS for Phase 1.

---

## Step 1 — Run provision-vm-mediakit.sh

```bash
cd /srv/foundry/clones/project-infrastructure
./infrastructure/virt/provision-vm-mediakit.sh
```

The script:
1. Downloads `ubuntu-24.04-server-cloudimg-amd64.img` (~630 MB) if not already in `work/`
2. Resizes the image to 20 GB
3. Builds a cloud-init seed ISO from `infrastructure/virt/cloud-init-mediakit/`
4. Launches QEMU with TCG acceleration, 6 GiB RAM, virtio-balloon, and 9 port-forwards
5. Daemonizes and writes the QEMU PID to `work/vm-mediakit.pid`

Expected output:

```
provision-vm-mediakit: downloading Ubuntu 24.04 cloud image (~630MB)...
provision-vm-mediakit: resizing disk to 20G...
provision-vm-mediakit: building cloud-init seed ISO...
provision-vm-mediakit: launching vm-mediakit (TCG, 6144M RAM)...
provision-vm-mediakit: VM started — PID XXXXXXX
provision-vm-mediakit: waiting for SSH (may take 5-10 min on TCG)...
provision-vm-mediakit: SSH ready.
provision-vm-mediakit: vm-mediakit is running.
  SSH:  ssh -p 10022 -i infrastructure/virt/work/foundry-vm-key foundry@localhost
  Monitor: echo 'info status' | socat - UNIX-CONNECT:infrastructure/virt/work/vm-mediakit.monitor
```

If the download stalls, check disk space. If the SSH wait times out, see the troubleshooting
section below.

---

## Step 2 — Wait for cloud-init

cloud-init runs on first boot to configure the hostname, user, and directory structure.
On TCG (software emulation, ~10x slower than KVM), cloud-init takes approximately 5–10
minutes to complete.

Confirm it has finished:

```bash
# Read the serial log for the final cloud-init message
tail -30 infrastructure/virt/work/vm-mediakit-serial.log | grep -E "cloud-init|login:"
```

You should see:

```
[  OK  ] Finished cloud-final.service - Execute cloud user/final scripts.
vm-mediakit login:
```

If the serial log is empty or not yet at the login prompt, wait 2–5 more minutes and
check again.

---

## Step 3 — Install packages

The cloud-init configuration does not install packages during first boot (this would take
60–90 minutes over SLIRP NAT on TCG). Install them manually once SSH is up:

```bash
KEY="infrastructure/virt/work/foundry-vm-key"
SSH="ssh -p 10022 -i ${KEY} -o StrictHostKeyChecking=no foundry@localhost"

$SSH "sudo apt-get update -qq && sudo DEBIAN_FRONTEND=noninteractive apt-get install -y nginx build-essential"
```

Verify:

```bash
$SSH "nginx -v 2>&1 && gcc --version | head -1"
```

Expected: `nginx version: nginx/1.24.0 (Ubuntu)` and a GCC version line.

---

## Step 4 — Verify the VM

```bash
KEY="infrastructure/virt/work/foundry-vm-key"
SSH="ssh -p 10022 -i ${KEY} -o StrictHostKeyChecking=no foundry@localhost"

# Hostname
$SSH "hostname"                         # should print: vm-mediakit

# glibc version — must be 2.39
$SSH "ldd --version | head -1"          # should print: ldd (Ubuntu GLIBC 2.39) 2.39

# Directories created by cloud-init
$SSH "ls /opt/mediakit/"                # should show: bin  data

# Uptime and memory
$SSH "uptime && free -h"
```

All four checks must pass before running `migrate-service-to-vm.sh`.

---

## Port-forward reference

The QEMU launch includes 9 SLIRP port-forwards. Each service inside the VM
binds on `0.0.0.0:<port>`; the host maps `localhost:1<port>` to it.

| Host port | VM port | Service |
|---|---|---|
| 10022 | 22 | SSH |
| 19090 | 9090 | knowledge-documentation |
| 19092 | 9092 | proofreader |
| 19093 | 9093 | knowledge-projects |
| 19095 | 9095 | knowledge-corporate |
| 19096 | 9096 | bim-orchestration |
| 19100 | 9100 | service-fs |
| 19101 | 9101 | marketing-pointsav |
| 19102 | 9102 | marketing/woodfine |

---

## QEMU monitor commands

The monitor socket allows control of the running VM without SSH:

```bash
MONITOR="infrastructure/virt/work/vm-mediakit.monitor"

# Check VM status
echo "info status" | socat - UNIX-CONNECT:${MONITOR}

# Adjust balloon RAM (e.g. set to 4096 MB)
echo "balloon 4096" | socat - UNIX-CONNECT:${MONITOR}

# Graceful shutdown (preferred over kill)
echo "system_powerdown" | socat - UNIX-CONNECT:${MONITOR}
```

---

## Performance notes — TCG emulation

The GCP workspace VM does not have hardware KVM (nested virtualisation is not enabled by
default on GCP). QEMU runs in TCG (software emulation) mode at approximately one-tenth
the speed of a native KVM guest.

Expected behaviour under TCG:
- First boot to SSH-ready: 5–10 minutes
- First HTTP request to a wiki service: 30–60 seconds (service does internal initialisation)
- Subsequent requests: faster (caches warmed)
- CPU usage: the QEMU process uses 100–110% of one host CPU core continuously — normal

TCG is adequate for Phase 1 testing. For production use with hardware KVM, provision the
VM on a host with nested virtualisation enabled (e.g. Laptop A with VT-x, or a GCP VM
with `--enable-nested-virtualization`).

---

## Troubleshooting

**SSH not available after 10 minutes:**

```bash
# Check the serial log for errors
tail -50 infrastructure/virt/work/vm-mediakit-serial.log

# Check the QEMU process is still running
ps aux | grep qemu | grep -v grep
cat infrastructure/virt/work/vm-mediakit.pid
```

If the QEMU process has exited, check for error messages in the serial log. The most
common cause is insufficient disk space for the QCOW2 image.

**SSH host key conflict (VM was rebuilt):**

```bash
ssh-keygen -f ~/.ssh/known_hosts -R '[localhost]:10022'
```

**cloud-init did not create `/opt/mediakit/`:**

```bash
ssh -p 10022 -i infrastructure/virt/work/foundry-vm-key foundry@localhost \
  "sudo mkdir -p /opt/mediakit/bin /opt/mediakit/data && sudo chown -R foundry:foundry /opt/mediakit"
```

---

## Stopping and restarting

**Graceful stop (preferred):**

```bash
echo "system_powerdown" | socat - UNIX-CONNECT:infrastructure/virt/work/vm-mediakit.monitor
```

**Force kill (last resort — risks filesystem corruption):**

```bash
kill $(cat infrastructure/virt/work/vm-mediakit.pid)
```

**Restart:** run `provision-vm-mediakit.sh` again. The script detects an existing PID file
and the running QEMU process; it will exit with an error rather than launch a second VM.
Stop the VM first, then rerun.

To completely rebuild (new QCOW2 from scratch):

```bash
# Stop the VM
echo "system_powerdown" | socat - UNIX-CONNECT:infrastructure/virt/work/vm-mediakit.monitor
sleep 10
# Remove the disk and seed ISO — the script will regenerate both
rm -f infrastructure/virt/work/vm-mediakit.qcow2 infrastructure/virt/work/vm-mediakit-seed.iso
./infrastructure/virt/provision-vm-mediakit.sh
```

---

## See also

- `guide-vm-mediakit-service-migration.md` — migrate services into the running VM
- `guide-vm-prove-balloon-demo.md` — verify the virtio-balloon resource pool mechanism
