---
schema: foundry-sketch-v1
sketch_name: netbsd-phase2-prototype-tofu-module
created: 2026-04-27
author: Master Claude (forward-prep IaC design)
status: design-sketch — not implementation-ready; Task Claude implements against this
doctrine_version: 0.0.8
references:
  - system-substrate-doctrine.md claim #34
  - infrastructure/slm-yoyo/tofu/ (pattern source)
---

# IaC Design Sketch — NetBSD Phase 2 Prototype OpenTofu Module

Forward-prep sketch for the Task Claude session that will implement
Phase 2 (NetBSD compat-bottom prototype, Doctrine claim #34). This
document is a design specification, not HCL. No `.tf` files are
produced here.

---

## 1. Pattern source identified

The existing pattern source is the `slm-yoyo` OpenTofu module at
`/srv/foundry/infrastructure/slm-yoyo/tofu/`. It provisions a GCP
Compute Engine GPU VM (g2-standard-4 / A100) in a dedicated GCP
project for service-slm Yo-Yo inference burst. The module is
structured across nine files: `main.tf` (GCP project + API enablement),
`compute.tf` (static IP, persistent disk, `google_compute_instance`,
firewall rule), `iam.tf` (runtime SA + kill-switch SA + operator IAM
bindings), `variables.tf` (prefix, billing account, region, zone,
GPU class, image family/project/version, port, CIDRs), `outputs.tf`
(project ID, VM name, endpoint URL, SA email, Secret Manager IDs),
`secrets.tf` (Secret Manager resources for bearer token + Gemini
key), `quota.tf` (null\_resource that files a GPU quota preference
via gcloud), `budget.tf` (billing budget + Pub/Sub kill-switch trigger),
and `versions.tf` (OpenTofu ≥ 1.8, google ~> 6.10). The key pattern:
one GCP project per module instance (strong tenant isolation), a
custom image consumed by name/family via a `data.google_compute_image`
lookup, and a dedicated VM service account scoped to minimum-viable
roles. The proposed NetBSD module mirrors this shape but replaces
GPU-specific resources with AArch64-specific constraints and replaces
the image-lookup pattern with an import-based custom image, described
in §5 below.

---

## 2. Module purpose and scope

The proposed module provisions a NetBSD AArch64 VM on GCP Compute
Engine as a single-instance Phase 2 prototype for Doctrine claim #34
(The Two-Bottoms Sovereign Substrate). The VM boots a custom NetBSD
AArch64 disk image with Veriexec configured and a `signatures.veriexec`
file applied at first boot, proving that the compat-bottom verified-
image-boot posture is operative on commodity cloud hardware.

This is a one-off prototype instance — not a fleet pattern, not
multi-tenant, not production-ready. Its sole purpose is to demonstrate
that the same `os-*` binary can run on the NetBSD bottom with
Veriexec-enforced integrity, analogous to what seL4's verified-image
invariant provides on the native bottom.

---

## 3. Proposed module structure

```
infrastructure/netbsd-phase2-prototype/
├── main.tf
├── variables.tf
├── outputs.tf
├── versions.tf
├── README.md
├── README.es.md
└── netbsd-bootstrap/
    ├── build-image.sh
    ├── import-image.sh
    ├── generate-veriexec.sh
    └── apex-signing-stub.md
```

**`main.tf`** — GCP project resource (following the `slm-yoyo`
pattern: one project per instance for tenant isolation), API
enablement (compute.googleapis.com + iam.googleapis.com at minimum),
`google_compute_image` resource or `google_compute_disk` snapshot
import block, `google_compute_address` (static external IP),
`google_compute_instance` for the NetBSD AArch64 VM, `google_compute_firewall`
for SSH ingress, and `google_service_account` for the VM runtime SA
(minimum roles: `logging.logWriter`, `monitoring.metricWriter`).

**`variables.tf`** — configurable knobs: prefix, billing account,
operator email, region, zone (must be AArch64-capable), machine type
(operator/Master to confirm), disk size GB, custom image URI or GCS
object path, operator SSH public key, SSH CIDR allowlist. Full table
in §6.

**`outputs.tf`** — VM instance name, zone, external IP address, SSH
connection string, VM service account email, GCP project ID. The SSH
string is the primary operational output because this VM is accessed
by the Task session, not via an API endpoint.

**`versions.tf`** — OpenTofu ≥ 1.8; google provider ~> 6.10 (same
as `slm-yoyo`); no additional providers required unless image import
uses a GCS bucket that needs the `google-beta` provider.

**`README.md`** — apply prerequisites (GCP project billing, AArch64
zone confirmation, NetBSD image build and GCS upload, operator SSH key),
step-by-step apply procedure, Veriexec verification steps post-boot,
cleanup (destroy) procedure.

**`README.es.md`** — bilingual companion per CLAUDE.md §6. Spanish
overview of what the module provisions and its purpose; not a
1:1 translation (strategic-adaptation pattern per DOCTRINE.md §XII).

**`netbsd-bootstrap/build-image.sh`** — cross-compile NetBSD AArch64
from source using `build.sh release`; see §7.

**`netbsd-bootstrap/import-image.sh`** — upload the produced disk
image to a GCS bucket and register it as a GCP custom image in the
target project; required before `tofu apply` when using the custom-
image-import path (Option A in §5).

**`netbsd-bootstrap/generate-veriexec.sh`** — walk the produced
image file tree and generate `signatures.veriexec`; see §7.

**`netbsd-bootstrap/apex-signing-stub.md`** — placeholder describing
the customer-apex co-signing step; see §7.

---

## 4. Key tofu resource declarations

Pseudo-HCL sketch. Not valid HCL; syntactic shortcuts used for clarity.

```hcl
# main.tf — project + API enablement (mirrors slm-yoyo pattern)

resource "google_project" "netbsd_phase2" {
  name            = "netbsd-phase2-prototype (${var.prefix})"
  project_id      = "${var.prefix}-netbsd-phase2"
  billing_account = var.billing_account
  labels          = { component = "netbsd-phase2", managed_by = "opentofu" }
  deletion_policy = "DELETE"
}

resource "google_project_service" "enabled" {
  for_each = toset(["compute.googleapis.com", "iam.googleapis.com",
                    "logging.googleapis.com", "monitoring.googleapis.com",
                    "secretmanager.googleapis.com", "storage.googleapis.com"])
  project  = google_project.netbsd_phase2.project_id
  service  = each.value
}

# compute.tf — image data source, static IP, boot disk, VM instance

# Option A (recommended): operator pre-registers a custom image via
# import-image.sh before apply. Tofu looks up the image by name.
data "google_compute_image" "netbsd_aarch64" {
  project = google_project.netbsd_phase2.project_id
  name    = var.netbsd_image_name   # e.g. "netbsd-aarch64-10-1-20260427"
}

resource "google_compute_address" "netbsd_phase2" {
  project = google_project.netbsd_phase2.project_id
  region  = var.region
  name    = "${var.prefix}-netbsd-ip"
}

resource "google_compute_instance" "netbsd_phase2" {
  project      = google_project.netbsd_phase2.project_id
  zone         = var.zone
  name         = "${var.prefix}-netbsd-phase2"

  # AArch64 machine type on GCP Compute Engine.
  # Tau T2A series uses Ampere Altra (AArch64). t2a-standard-1 is the
  # smallest SKU (1 vCPU, 4 GB RAM) — adequate for NetBSD + Veriexec
  # prototype; no GPU required.
  # IMPORTANT: T2A machines are only available in specific zones.
  # As of 2026 the T2A series is available in us-central1-a,
  # us-central1-b, us-central1-f, europe-west4-a, asia-southeast1-b,
  # and a small number of others. Operator must confirm zone availability
  # before apply. This is NOT the same zone set as the workspace VM.
  machine_type = var.machine_type   # default "t2a-standard-1"

  # T2A does not require on_host_maintenance = TERMINATE (no GPU).
  # MIGRATE is the default and is acceptable here.
  scheduling {
    on_host_maintenance = "MIGRATE"
    automatic_restart   = true
    preemptible         = false
  }

  # No guest_accelerator block — no GPU on this instance.

  boot_disk {
    initialize_params {
      # Custom NetBSD AArch64 image imported via import-image.sh.
      # This is the hardest part — see §5. The image must be pre-
      # registered before tofu apply; Tofu does not build it.
      image = data.google_compute_image.netbsd_aarch64.self_link

      # 20 GB is adequate for NetBSD base + pkgsrc essentials + Veriexec db.
      # Expand to 40 GB if a full pkgsrc bulk build is needed in Phase 2.
      size  = var.disk_gb    # default 20
      type  = "pd-balanced"
    }
  }

  network_interface {
    network = "default"
    access_config {
      nat_ip = google_compute_address.netbsd_phase2.address
    }
  }

  # NetBSD does not support GCE OS Login (enable-oslogin = TRUE),
  # which requires a Google-maintained SSH agent. Use the ssh-keys
  # metadata key instead with an operator-supplied public key.
  # This is a documented difference from the slm-yoyo pattern.
  metadata = {
    ssh-keys = "foundry:${var.operator_ssh_pubkey}"
    # Veriexec signatures file path — informational; actual file
    # placement is part of build-image.sh + generate-veriexec.sh.
    veriexec-db-path = "/etc/signatures.veriexec"
  }

  service_account {
    email  = google_service_account.netbsd_phase2.email
    scopes = ["cloud-platform"]
    # cloud-platform scope follows the workspace VM pattern (CLAUDE.md §3).
    # Minimum-privilege binding in iam.tf narrows what the SA can actually do.
  }

  labels = { component = "netbsd-phase2", prefix = var.prefix }
  tags   = ["${var.prefix}-netbsd-ssh"]

  lifecycle {
    # Image self_link is normalized by GCP; ignore drift.
    ignore_changes = [boot_disk[0].initialize_params[0].image]
  }

  depends_on = [
    google_project_service.enabled,
    google_project_iam_member.netbsd_log_writer,
  ]
}

resource "google_compute_firewall" "netbsd_ssh_inbound" {
  project       = google_project.netbsd_phase2.project_id
  network       = "default"
  name          = "${var.prefix}-netbsd-ssh"
  source_ranges = var.ssh_ip_cidrs   # default ["0.0.0.0/0"]; tighten for production
  target_tags   = ["${var.prefix}-netbsd-ssh"]
  allow { protocol = "tcp"; ports = ["22"] }
  depends_on    = [google_project_service.enabled]
}
```

Block-by-block notes:

- **`google_project`** — dedicated project per module instance,
  identical to slm-yoyo. Isolates billing, IAM, and audit logs to
  this prototype.
- **`machine_type = "t2a-standard-1"`** — Tau T2A is the only GCP
  first-party AArch64 machine series (Ampere Altra). Zone availability
  is limited; operator must verify before apply. Operator/Master to
  confirm; do not assume availability in the workspace VM zone.
- **`scheduling.on_host_maintenance = "MIGRATE"`** — T2A supports
  live migration, unlike GPU instances. No need for TERMINATE.
- **`boot_disk.image`** — consumes a pre-imported custom image.
  NetBSD is not in GCP's public image catalog; the custom image must
  be built and imported before `tofu apply`. See §5 for options.
- **`metadata.ssh-keys`** — uses the raw SSH metadata key, not OS
  Login, because NetBSD has no GCE OS Login agent. The operator must
  supply a public key in `var.operator_ssh_pubkey`. This diverges
  from slm-yoyo's `enable-oslogin = TRUE` pattern.
- **`service_account.scopes = ["cloud-platform"]`** — mirrors the
  workspace VM pattern (CLAUDE.md §3). IAM bindings in `iam.tf`
  restrict actual API surface to log writer + metric writer.

---

## 5. NetBSD AArch64 image — the hardest part

NetBSD AArch64 is NOT in GCP's public image catalog. The module
cannot reference a first-party image. Three resolution paths:

### Option A — Build from source + import as GCP custom image

**Mechanism:** Run `build.sh release` (NetBSD's hermetic cross-build
system) on a POSIX host (the workspace VM, x86\_64) to produce a
raw AArch64 disk image. Convert to a format GCP accepts (raw image
in `.tar.gz`). Upload to a GCS bucket. Register as a `google_compute_image`
resource in the target project via `import-image.sh`. Tofu's
`data.google_compute_image` lookup then finds it by name.

**Cost/friction:** Build time on workspace VM CPU: approximately
2-4 hours for a full NetBSD AArch64 release build. GCS storage for
the raw image: ~500 MB compressed, negligible cost. Image import: 15-30
minutes. One-time setup; subsequent prototype restarts reuse the
registered image. Cross-compiling NetBSD on x86\_64 Linux is well-
documented (NetBSD build.sh supports Linux hosts); the workspace VM
runs Ubuntu which satisfies the host requirements.

**Reproducibility:** Strongest option. `build.sh` is the NetBSD
Foundation's canonical reproducible-build interface. A vendored source
snapshot (tarball at a fixed release tag) produces a byte-identical
disk image on the same host architecture. Offline-reproducibility
property is preserved — the same snapshot can rebuild the image from
a USB-archived copy without network access (Mechanism C, §7 of
system-substrate-doctrine.md).

**Customer-demo viability:** High. The customer can run the same
`build.sh` command on their own hardware (any POSIX x86\_64 or
AArch64 host). They can import the image into their own GCP project
or flash it onto their own AArch64 appliance. The build is not tied
to PointSav infrastructure. This satisfies the customer-first ordering
rule: we build it the same way the customer will build it.

### Option B — iPXE chain-load from a generic AArch64 Linux base

**Mechanism:** Start with a GCP-supported AArch64 image (Debian
arm64 or Ubuntu arm64 are in the GCP public catalog). Use a startup
script to PXE-boot or kexec into a NetBSD kernel fetched from a GCS
bucket. Requires a working `kexec` path or UEFI override — neither
is straightforward on GCP VMs where the virtual firmware (UEFI/EDK2)
is Google-controlled.

**Cost/friction:** High friction. GCP's UEFI implementation for T2A
is EDK2-based; customer network booting via iPXE requires either a
UEFI shell that iPXE can launch or a prebuilt UEFI image with iPXE
embedded. GCP's Secure Boot feature (shielded VMs) would reject an
unsigned bootloader chain. Effectively requires disabling Secure Boot
and relying on a multi-step startup script, which means the boot is
not clean from the UEFI boundary — the Veriexec posture is weakened
because Linux's startup script is the trust root, not the NetBSD
kernel itself.

**Reproducibility:** Weak. The Linux intermediary is a moving target;
build reproduction requires pinning the Linux base image. Offline-
reproducibility is broken because the iPXE/kexec stage requires GCS
access at boot time.

**Customer-demo viability:** Low. The customer cannot reproduce this
on bare metal (no GCP UEFI intermediary on physical hardware). Defeats
the "boot anywhere" property of Mechanism C. Not recommended.

### Option C — Leased AArch64 appliance (physical or hosted)

**Mechanism:** Use a dedicated AArch64 server rather than GCP Compute
Engine. Options include a Raspberry Pi 5 (AArch64, available today,
~$80), an Ampere Altra developer board (~$500-2,000), a Hetzner
AX52 arm64 dedicated server (~$60/month), or Scaleway ARM64 instances.
NetBSD 10.x AArch64 supports Raspberry Pi 4/5 via the `earmv7hf` /
`evbarm-aarch64` ports.

**Cost/friction:** For physical hardware: one-time purchase, no
recurring cloud cost, but requires physical access or remote-hands for
OS install. For leased servers: Hetzner / Scaleway are simpler than
GCP for NetBSD installation (they support custom OS via their rescue
mode + `dd`). Tofu IaC modules exist for Hetzner (hetznercloud provider)
and Scaleway (scaleway provider). However, this forks the IaC pattern
away from the GCP-provider shape Foundry has ratified in slm-yoyo.

**Reproducibility:** Strong on physical hardware. Moderate on leased
servers (depends on whether the host exposes raw disk writes). The
`build.sh` hermetic-build property is preserved regardless of where
the image boots.

**Customer-demo viability:** Highest on physical hardware. The customer
can replicate exactly using the same `build.sh` output flashed to
their own appliance. This is the intended long-horizon target: a
customer-owned Ampere or Raspberry Pi-class device running a Foundry
compat-bottom deployment, fully self-contained.

### Recommendation

**Option A** for the Phase 2 prototype. Build from source on the
workspace VM; import as a GCP custom image; keep the IaC provider
shape identical to `slm-yoyo`. This produces the cleanest Veriexec
boot (NetBSD kernel owns the boot chain from UEFI forward with no
Linux intermediary), the strongest reproducibility story, and the
highest customer-demo portability. Option C on physical hardware is
the right long-horizon target once the compat-bottom is validated; the
GCP prototype is the fastest path to exercising Veriexec on real AArch64
hardware. Option B is not recommended.

The operator must decide whether to use the workspace VM T2A zone
(requires confirming zone + quota availability in the workspace GCP
project) or a fresh GCP project in a T2A-capable zone. This is a
Master/operator decision per the open questions in §9.

---

## 6. Variables specification

| Name | Type | Default | Description |
|---|---|---|---|
| `prefix` | `string` | — | Short identifier forming GCP project ID `<prefix>-netbsd-phase2`. Lowercase letters, digits, dashes; 4-20 chars. |
| `billing_account` | `string` | — | GCP billing account ID. Required for project creation. |
| `operator_email` | `string` | — | Email for IAM binding (compute.instanceAdmin.v1). |
| `region` | `string` | `"us-central1"` | GCP region. Must contain a T2A-capable zone. |
| `zone` | `string` | `"us-central1-a"` | GCP zone. Must support T2A (Tau AArch64). Operator to verify before apply. |
| `machine_type` | `string` | `"t2a-standard-1"` | GCE machine type. t2a-standard-1 = 1 vCPU / 4 GB RAM (Ampere Altra AArch64). Operator/Master to confirm availability in chosen zone. |
| `disk_gb` | `number` | `20` | Boot disk size in GB. 20 GB fits NetBSD base + pkgsrc essentials + Veriexec db. |
| `netbsd_image_name` | `string` | — | Name of the pre-imported custom GCP image. Must be registered in the target project before `tofu apply`. Produced by `import-image.sh`. |
| `operator_ssh_pubkey` | `string` | — | Operator's SSH public key in `authorized_keys` format. Injected via GCE metadata `ssh-keys`. OS Login is not used (NetBSD has no GCE OS Login agent). |
| `ssh_ip_cidrs` | `list(string)` | `["0.0.0.0/0"]` | CIDR blocks permitted to reach port 22. Tighten to workspace VM IP before any customer demo. |
| `veriexec_db_path` | `string` | `"/etc/signatures.veriexec"` | Path on the VM where the Veriexec signatures file is located. Must match the NetBSD loader configuration. |

---

## 7. Bootstrap scripts — what they do

### `build-image.sh`

Invokes `build.sh release` from a vendored NetBSD source tree at a
pinned release tag (e.g., NetBSD 10.1). The cross-compile target is
`evbarm-aarch64` (the NetBSD AArch64 embedded-systems port). Produces
a raw AArch64 disk image under `obj/destdir.evbarm-aarch64/`. After
the build, the script assembles a bootable disk image (partition table,
bootloader, root filesystem) using NetBSD's `makefs` and `fdisk`
utilities. Output artifact: `netbsd-aarch64-<version>.img` in the
script's working directory. Build host requirements: Linux x86\_64
or AArch64, POSIX toolchain (`make`, `binutils`, `cvs`-optional;
NetBSD provides its own cross-toolchain entirely via `build.sh`). No
GCP access required; this runs entirely offline.

### `import-image.sh`

Takes the disk image produced by `build-image.sh` and registers it
as a GCP custom image. Steps: (1) compress the raw image to a `.tar.gz`
with the mandatory GCP naming convention (`disk.raw` inside the archive);
(2) upload to a GCS bucket in the target project using `gsutil cp`;
(3) call `gcloud compute images create` with `--source-uri` pointing
to the GCS object. The resulting image name is printed to stdout and
should be passed to `var.netbsd_image_name` in the `tofu apply`
invocation. Requires GCP credentials with `compute.images.create` and
`storage.objects.create` permissions on the target project.

### `generate-veriexec.sh`

Walks the file tree of the assembled disk image (mounted via loop
device on Linux, or via `vnconfig` on a NetBSD host). For each
executable file (`-perm /111`), computes a SHA-256 fingerprint and
emits a line in NetBSD Veriexec format:
`/path/to/binary SHA256 <hex-hash>`. Writes the complete
`signatures.veriexec` file to the script's working directory. This
file must be present on the disk image before import (the `build-image.sh`
script should call `generate-veriexec.sh` as a final step and inject
the signatures file into the image). The Veriexec database format is
documented in the NetBSD `veriexec(5)` manual page.

### `apex-signing-stub.md`

Placeholder document describing the customer-apex co-signing step
that Phase 2 does not yet implement. In the target architecture
(system-substrate-doctrine.md §4), the `signatures.veriexec` file
is cosigned by the customer's apex key before being embedded in the
disk image, producing a verifiable chain: `build.sh` source → disk
image → Veriexec db → apex signature → ledger entry. The stub names
the manual steps an operator must perform until this ceremony is
automated: (1) locate the apex key (currently `identity/pointsav-administrator/id_pointsav-administrator`
or a dedicated Phase 2 apex key — see §9); (2) sign the SHA-256 hash
of the `signatures.veriexec` file with the apex key; (3) embed the
signature in the disk image alongside the Veriexec db, in a location
the Phase 2 verification harness can check at boot. The stub is the
acceptance criterion for Phase 2 completion: if the Task session
replaces this stub with a working implementation, claim #34 is
operationally demonstrated.

---

## 8. Operator-action expectations

Actions the tofu module cannot automate:

- **GCP project billing enablement** — a billing account must be
  linked before `google_project` creation succeeds. The module accepts
  `var.billing_account` but does not provision the account itself.
- **T2A zone confirmation** — the T2A machine series (AArch64) is not
  available in all GCP zones. The operator must verify that the chosen
  `var.zone` has T2A quota available. The command is:
  `gcloud compute machine-types list --filter="name:t2a" --zones=<zone>`.
  If T2A is not available in the workspace VM's current zone
  (us-central1 or us-west1 depending on the workspace VM zone), a
  different zone must be selected.
- **NetBSD image build** — `build-image.sh` is a multi-hour CPU task.
  The operator runs it on the workspace VM or a suitable build host
  before `tofu apply`. Tofu cannot build an OS from source.
- **NetBSD image import** — `import-image.sh` uploads the image to
  GCS and registers it in GCP. This must complete before `tofu apply`
  because `data.google_compute_image` will fail if the image does not
  exist. The `var.netbsd_image_name` value must match the name
  registered by `import-image.sh` exactly.
- **Customer-apex key procurement** — the Veriexec signing ceremony
  described in `apex-signing-stub.md` requires a key the operator
  holds. For Phase 2, this may be the existing `ps-administrator`
  key or a new dedicated Phase 2 key; see §9.
- **SSH key provisioning** — the operator supplies `var.operator_ssh_pubkey`
  with their AArch64 SSH access key. This is distinct from the workspace
  VM SSH key; a new keypair may be appropriate for the NetBSD prototype.
- **Veriexec mode configuration** — after first boot, Veriexec must
  be switched from `learning` mode to `strict` mode to enforce the
  policy. This is a manual step (`sysctl -w veriexec.strict=2`) after
  verifying that all required executables are fingerprinted. The tofu
  module cannot enforce this from the outside.

---

## 9. Open questions for Master

1. **GCP vs leased appliance — which does the operator prefer for the
   Phase 2 prototype?** The sketch assumes GCP + Option A (custom image
   import), following the slm-yoyo precedent. A leased AArch64 appliance
   (Hetzner, physical Raspberry Pi 5, Ampere developer board) may be
   operationally simpler for NetBSD installation and avoids the GCP
   custom-image import complexity. Customer-first ordering slightly
   favors physical appliance for long-term demos; GCP is faster to stand
   up without hardware procurement. Which is the operator preference?

2. **NetBSD AArch64 image source — vendor-built or Foundry-built-from-
   source?** Option A assumes Foundry builds from the NetBSD Foundation's
   source tarball via `build.sh`. An alternative is to start from the
   NetBSD Foundation's pre-built AArch64 installation image (available
   as a `.img.gz` from cdn.netbsd.org) and skip the cross-compile step
   for the prototype. Pre-built is faster (minutes vs hours); built-from-
   source is the reproducibility story claim #34 actually requires. Does
   the operator want to demonstrate the full `build.sh` reproducibility
   for Phase 2, or defer that to a later milestone?

3. **Workspace VM-SA scope sufficiency** — the workspace VM authenticates
   to GCP via its Compute Engine SA with `cloud-platform` scope
   (CLAUDE.md §3). This scope should cover `compute.images.create`,
   `storage.objects.create`, and `google_project` creation in a
   new subproject. However, `google_project` creation requires
   `resourcemanager.projects.create` on the organization or folder,
   which is not guaranteed by `cloud-platform` scope alone. Does the
   workspace VM's SA have this permission? Should be verified with
   `gcloud projects create --dry-run` before committing to the GCP
   path.

4. **Apex-signing key for Phase 2** — the `apex-signing-stub.md` names
   a signing key that will be used to co-sign `signatures.veriexec`.
   Should this be the existing `ps-administrator` identity key (already
   in `identity/pointsav-administrator/`) or a new dedicated Phase 2
   key registered in `~/Foundry/identity/` with its own folder and
   `allowed_signers` entry? A dedicated key is cleaner separation
   (the workspace repo signing key should not double as a boot-time
   capability root); a new key requires operator action to generate
   and register.

5. **Where does `infrastructure/netbsd-phase2-prototype/` live in the
   commit flow?** The `slm-yoyo` tofu module lives in the workspace repo
   at `infrastructure/` (Master-layer, not tracked in any engineering
   repo). Should the NetBSD Phase 2 module follow the same pattern
   (Master commits to workspace `infrastructure/`), or should it live
   in the `project-system` cluster as a Task-layer deliverable under
   `pointsav-fleet-deployment/fleet-infrastructure-cloud/` (the cluster
   manifest names that catalog subfolder)? The slm-yoyo pattern suggests
   workspace `infrastructure/` for infrastructure that provisions a VM
   to run `service-slm`; the project-triad discipline suggests the
   catalog subfolder for anything that belongs to the fleet-deployment
   customer leg.

---

## 10. References

- Pattern source: `/srv/foundry/infrastructure/slm-yoyo/tofu/` —
  `main.tf`, `compute.tf`, `variables.tf`, `outputs.tf`, `iam.tf`,
  `secrets.tf`, `quota.tf`, `versions.tf`
- `system-substrate-doctrine.md` §2 (two-bottoms shape), §2.1 (why
  NetBSD), §2.2 (AArch64 hardware target priority), §6 (Mechanism B —
  Reproducible-Verification-On-Customer-Metal), §11 (Phase 2 scope)
- Cluster manifest: `/srv/foundry/clones/project-system/.claude/manifest.md`
- CLAUDE.md §3 (GCP / Google identity model — VM-SA cloud-platform scope)
- CLAUDE.md §10 (deployment lifecycle — catalog vs instance pattern)
- CLAUDE.md §11 (action matrix — IaC module authorship is workspace /
  Master layer; Task provisions instances)
- GCP Compute Engine AArch64 (T2A): `https://cloud.google.com/compute/docs/general-purpose-machines#t2a_machines`
  — machine series, zone availability, and Tau T2A standard shapes.
  Operator/Master to confirm `t2a-standard-1` availability in chosen zone.
- NetBSD `build.sh`: `https://www.netbsd.org/docs/guide/en/chap-build.html` —
  canonical cross-build documentation; `build.sh release` target for
  AArch64 (evbarm-aarch64) is the hermetic-build path.
- NetBSD Veriexec: `veriexec(5)`, `veriexecgen(8)` manual pages in
  NetBSD 10.x distribution; `https://www.netbsd.org/docs/guide/en/chap-veriexec.html`
- C2SP `signed-note`: `https://github.com/C2SP/C2SP/blob/main/signed-note.md`
  (apex-cosigning primitive for the `signatures.veriexec` ceremony)

---

*Sketch authored 2026-04-27. Pattern source: `slm-yoyo` tofu module.
Not implementation-ready; forward-prep only. Task Claude implements
against this in the Phase 2 session.*
