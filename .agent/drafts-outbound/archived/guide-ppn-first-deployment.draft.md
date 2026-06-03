---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: woodfine-fleet-deployment
target_path: fleet-infrastructure/
target_filename: guide-ppn-first-deployment.md
audience: operators deploying a PointSav PPN for the first time on the Part A-lite topology
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - briefs/BRIEF-PPN-DEV-BOOTSTRAP.md
  - systems/os-network-admin.md
  - infrastructure/virt/vm-prove.sh
notes_for_editor: >
  New guide — targets woodfine-fleet-deployment/fleet-infrastructure/. This is the
  operator-facing runbook for the 5-step first-deployment sequence described in
  BRIEF-PPN-DEV-BOOTSTRAP.md §7. All five steps are unblocked as of 2026-05-28 —
  they require only cargo, WireGuard mesh up, and SSH access to the GCP VM and Laptop A.
  Steps that depend on pending operator decisions (Q2–Q6: subnet ratification, static IP,
  ratatui TUI) use planned/intended language per BCSC posture.
  Article frontmatter to add on commit: title "PPN First Deployment",
  category "operations", status "active", quality "review".
research_done_count: 3
research_suggested_count: 0
open_questions_count: 2
open_question_1: >
  Q2 (canonical PPN subnet): guide uses 10.8.0.x addresses as current de-facto values.
  When operator ratifies 10.42.0.0/16, update all IP addresses in this guide.
open_question_2: >
  Q5 (service-slm Doorman): app-network-admin F8 Gateway currently calls a subprocess;
  Step 4 notes this limitation. When Q5 is resolved (HTTP to localhost:9080), update
  the "Current state" note in Step 4.
---

# PPN First Deployment

This guide walks through the five-step sequence to bring up a PointSav Private Network
for the first time on the Part A-lite topology: one GCP cloud relay node
(`fleet-infrastructure-cloud-1`, 10.8.0.9), one on-premises operator node (Laptop A,
`station-workplace-mathew-1`, 10.8.0.6), and the WireGuard hub on Laptop B
(`route-network-admin-1`, 10.8.0.1).

All five steps are unblocked and can run today. No additional infrastructure or
decision gates are required.

## Prerequisites

- WireGuard mesh is up: Laptop A (10.8.0.6) and GCP VM (10.8.0.9) can reach each other
  over `ppn0`. Verify: `ping -c1 10.8.0.9` from Laptop A.
- Rust toolchain installed on the build machine (GCP VM or Laptop A): `cargo --version`.
- SSH access from the build machine to both Laptop A and the GCP VM.
- The `project-infrastructure` monorepo is checked out on the build machine.

---

## Step 1 — Deploy `service-ppn-pairing` on the GCP VM

`service-ppn-pairing` is the ceremony backend. It listens on `0.0.0.0:9202`, stores
pending node-join requests, and issues approval results. It runs on the GCP VM so it is
reachable by all PPN nodes over the WireGuard mesh.

**Build the release binary:**

```bash
cargo build --release -p service-ppn-pairing
```

**Copy binary and systemd unit to the GCP VM:**

```bash
scp target/release/service-ppn-pairing mathew@10.8.0.9:/usr/local/bin/
scp infrastructure/systemd/local-ppn-pairing.service mathew@10.8.0.9:/etc/systemd/system/
```

**Enable and start the service on the GCP VM:**

```bash
ssh mathew@10.8.0.9 "sudo systemctl daemon-reload && sudo systemctl enable --now local-ppn-pairing"
```

**Verify the service is running:**

```bash
ssh mathew@10.8.0.9 "systemctl is-active local-ppn-pairing"
# Expected: active
```

---

## Step 2 — Verify pairing service is reachable from Laptop A

Before proceeding, confirm that Laptop A can reach `service-ppn-pairing` over the
WireGuard mesh. This proves the mesh is up and the ceremony backend is reachable.

**From Laptop A:**

```bash
curl http://10.8.0.9:9202/v1/node-join/pending
```

**Expected response:**

```json
{"pending":[]}
```

An empty pending list is correct — no nodes have requested to join yet. A connection
refused or timeout means either the service is not running (check Step 1) or the
WireGuard tunnel is down (check `wg show ppn0`).

---

## Step 3 — Build and copy `os-network-admin` to Laptop A

`os-network-admin` is the PPN control plane. It runs on Laptop A (the operator's
primary site machine) and polls `service-ppn-pairing` every five seconds for pending
node-join requests.

**Build the release binary:**

```bash
cargo build --release -p os-network-admin
```

**Copy binary to Laptop A:**

```bash
scp target/release/os-network-admin mathew@10.8.0.6:~/bin/os-network-admin
```

Ensure `~/bin/` is on the `PATH` on Laptop A, or use the full path when running.

---

## Step 4 — Run `os-network-admin` as the control plane

**On Laptop A:**

```bash
PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin
```

`os-network-admin` will start polling `service-ppn-pairing` every five seconds. When a
new node submits a join request, the Crockford base32 short code will appear on stdout:

```
[2026-05-28T10:00:00Z] pending join: ABCD-EFGH (expires in 600s)
```

To approve a join request from a second terminal:

```bash
curl -s -X POST http://10.8.0.9:9202/v1/node-join/approve \
     -H 'Content-Type: application/json' \
     -d '{"code":"ABCD-EFGH"}'
```

**Current state note:** The `app-network-admin` F8 Terminal layer — which provides the
full HTTP :8085 plain-language command surface and UDP :8090 mesh broadcast — is the
production operator surface planned for deployment after service-slm Doorman is confirmed
at `localhost:9080`. Until that step is complete, `os-network-admin` itself is the
operator's approval surface for node-join requests.

---

## Step 5 — Optional: Run `vm-prove.sh` to demonstrate the hypervisor layer

This step is optional but recommended on Laptop A, which has hardware VT-x (Intel Sandy
Bridge i5-2400S) and can run VMs with full KVM acceleration.

See the companion guide **Guide: VM Prove and Balloon Demo** for the full procedure,
including how to demonstrate `virtio_balloon` resource pool management from the QEMU monitor.

**Quick start (on Laptop A):**

```bash
cd /path/to/project-infrastructure
./infrastructure/virt/vm-prove.sh
# Auto-detects KVM; boots Alpine Linux VM with virtio-balloon device
```

---

## What this deployment proves

After completing Steps 1–4, the following is demonstrated:

- The PPN mesh is operational (WireGuard tunnels are up between all nodes)
- `service-ppn-pairing` can receive and store node-join requests (WORM `nodes.jsonl` registry)
- `os-network-admin` can poll the ceremony backend and surface pending requests to the operator
- The operator has an approval surface for admitting new nodes to the mesh

What this does NOT yet demonstrate:

- Totebox Archive data access via PSP (gated on PSP protocol implementation)
- Multi-archive orchestration via `os-orchestration` (gated on Phase 4 gateway)
- The full Genesis Protocol boot sequence (gated on `os-infrastructure` code rewrite)
- Automated balloon controller in `os-infrastructure` (planned future milestone)

---

## VM capacity planning

When sizing how many VMs to run, the right unit of analysis is the **running deployment
instance** — not the source project. The monorepo contains ~116 source-code directories,
but most are dormant scaffold-coded crates that do not need a running VM.

| Scope | Count (today) | Min RAM needed | Notes |
|---|---|---|---|
| Per source project | 116 | ~464 GB | Impractical; most projects are dormant |
| Per deployment instance | 18 | ~72 GB | Right unit for capacity planning |
| Per logical cluster | 9 | ~36–72 GB | Natural grouping per PROJECT-CLONES.md |
| Single-node proof-of-concept | 1 | 4–8 GB | Fits the current GCP workspace VM |

The current workspace VM (32 GB RAM) fits a single-node proof-of-concept with headroom
for a few concurrent VMs. Scaling to per-cluster isolation (9 clusters) is the intended
next tier; upgrading workspace RAM is the primary prerequisite — not additional
provisioning work.

## Troubleshooting

| Symptom | Likely cause | Resolution |
|---|---|---|
| `curl` to 10.8.0.9:9202 times out | WireGuard tunnel down or service not started | `wg show ppn0`; check `systemctl status local-ppn-pairing` on GCP VM |
| `cargo build` fails for `os-network-admin` | Missing system libraries or wrong Rust edition | `rustup update stable`; check `Cargo.toml` edition field |
| `os-network-admin` shows no pending requests | No node has submitted a join request yet | Expected — the queue is empty until a node runs `service-ppn-pairing` |
| `os-network-admin` exits immediately | `PAIRING_SERVER` env var not set or service unreachable | Verify Step 2 before running Step 4 |
