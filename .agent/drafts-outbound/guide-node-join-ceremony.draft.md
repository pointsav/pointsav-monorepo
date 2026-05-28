---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: woodfine-fleet-deployment
target_path: fleet-infrastructure/
target_filename: guide-node-join-ceremony.md
audience: operators admitting new physical nodes to the PPN mesh
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - briefs/BRIEF-PPN-DEV-BOOTSTRAP.md
  - systems/os-network-admin.md
  - architecture/genesis-protocol.md
notes_for_editor: >
  New guide — targets woodfine-fleet-deployment/fleet-infrastructure/. Covers the
  node-join ceremony from both perspectives: the node side (generating the short code
  via service-ppn-pairing) and the operator side (seeing the code in os-network-admin
  and approving via curl). The current minimal implementation uses stdout + curl;
  the planned ratatui TUI is described in planned/intended language per BCSC posture.
  Article frontmatter to add on commit: title "Node Join Ceremony",
  category "operations", status "active", quality "review".
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
---

# Node Join Ceremony

The node-join ceremony is the process by which a new physical node earns membership in
the PPN WireGuard mesh. It is a mutual authentication ritual: the node proves it controls
a known short code; the operator confirms the code out-of-band and approves the join.
Neither side can succeed without the other. The ceremony produces a `nodes.jsonl` entry
that persists the node's identity across restarts.

The ceremony uses a **CPace PAKE** (Password-Authenticated Key Exchange) over the short
code channel, with a **Short Authenticated String (SAS) comparison** to close the
man-in-the-middle gap. The short code itself is a Crockford base32 string, eight
characters, approximately 40 bits of entropy. It expires after **600 seconds**.

---

## Node side

The joining node runs `service-ppn-pairing`. On startup or on receiving a join trigger,
the service:

1. Generates a Crockford base32 short code
2. Submits a join request to `service-ppn-pairing` on the GCP relay node (10.8.0.9:9202)
   via `POST /v1/node-join/request`
3. Waits for operator approval (or expiry at 600s)

The short code is displayed to the physical operator at the node's console. In the current
minimal implementation, the code is printed to stdout by the node-side process. In
production, the code would be displayed on a hardware display or QR code attached to the
node chassis.

**Example `POST /v1/node-join/request` payload:**

```json
{
  "code": "ABCD-EFGH",
  "public_key": "<WireGuard public key base64>",
  "requested_ip": "10.8.0.x"
}
```

The node's WireGuard public key is included in the request so that, upon approval, the
key is registered in the peer map without a second round-trip.

---

## Operator side

The operator runs `os-network-admin` on Laptop A. It polls `service-ppn-pairing` every
five seconds and prints any pending join requests to stdout.

**Start the control plane:**

```bash
PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin
```

**When a node submits a join request, output appears:**

```
[2026-05-28T10:15:00Z] pending join: ABCD-EFGH (expires in 600s)
  public_key: <base64>
  requested_ip: 10.8.0.4
```

The operator verifies the short code matches what is displayed at the node (in person,
by phone, or by any out-of-band channel). This verification step is the SAS comparison
that closes the man-in-the-middle gap — the operator is confirming that the code they see
in `os-network-admin` matches the code the node generated locally.

**To approve the join:**

```bash
curl -s -X POST http://10.8.0.9:9202/v1/node-join/approve \
     -H 'Content-Type: application/json' \
     -d '{"code":"ABCD-EFGH"}'
```

**Expected response:**

```json
{"status":"approved","node_id":"fleet-infrastructure-onprem-2"}
```

**To deny the join** (or simply let the code expire after 600 seconds):

```bash
curl -s -X POST http://10.8.0.9:9202/v1/node-join/deny \
     -H 'Content-Type: application/json' \
     -d '{"code":"ABCD-EFGH"}'
```

---

## What happens on approval

When the operator approves a join request:

1. `service-ppn-pairing` writes the node's identity to `nodes.jsonl` — an append-only
   registry. This entry persists across restarts and records the approval event with
   a UTC timestamp.
2. The node's WireGuard public key is added to the peer map, which distributes to all
   existing mesh members automatically.
3. The node receives a cluster CA-signed certificate confirming its mesh membership.
4. The node's WireGuard tunnel comes up on `ppn0` and the node becomes reachable at its
   assigned mesh address.

The `nodes.jsonl` entry format:

```json
{
  "node_id": "fleet-infrastructure-onprem-2",
  "code": "ABCD-EFGH",
  "public_key": "<base64>",
  "assigned_ip": "10.8.0.4",
  "approved_at": "2026-05-28T10:15:42Z",
  "approved_by": "os-network-admin@station-workplace-mathew-1"
}
```

The WORM property of `nodes.jsonl` means this entry cannot be modified or deleted.
Revoking a node's membership requires adding a revocation entry, not editing the original.

---

## What happens on expiry or denial

- **Expiry:** The short code becomes invalid after 600 seconds. The node must generate
  a new code and submit a new join request. The expired request is not written to
  `nodes.jsonl`.
- **Denial:** `service-ppn-pairing` records the denial event in `nodes.jsonl` (as a
  denial entry, not an approval entry) and drops the request. The node must generate a
  new code to retry.

---

## Planned production operator surface

The current minimal implementation (stdout polling + curl approval) is an interim surface.
The planned production operator surface is a keyboard-driven **ratatui TUI** that
`os-network-admin` will present at the terminal:

- Pending requests listed with node identity, short code, and expiry countdown
- `a` key: approve the selected request
- `d` key: deny the selected request
- QR code display via `system-pairing-codes::qr_unicode` for scanning the short code
  from a mobile device

The TUI is a planned milestone. Until it is implemented, the stdout + curl workflow
described above is the operator surface.

---

## Security notes

- **Never approve a code you did not verify out-of-band.** The SAS comparison step is
  the entire security guarantee of the ceremony. An attacker who intercepts the request
  channel can substitute their own public key; the only defence is the operator physically
  confirming the code matches at the node.
- **Codes are single-use.** Once approved or denied, a code cannot be reused.
- **The approval is not reversible in `nodes.jsonl`.** The WORM property of the registry
  means that an approval event is a permanent record. To remove a node from the mesh,
  add a revocation entry — do not attempt to delete or modify the approval entry.
- **`os-network-admin` holds zero cryptographic authority.** Approving a node-join grants
  the node PPN mesh membership. It grants no access to Totebox Archive data, no MBA
  pairing, and no `pairings.yaml` entry. Data access is a separate ceremony governed by
  Machine-Based Authorization.

---

## Troubleshooting

| Symptom | Likely cause | Resolution |
|---|---|---|
| Code does not appear in `os-network-admin` | Node did not submit a request, or mesh is down | Check `systemctl status local-ppn-pairing` on GCP VM; verify WireGuard tunnel |
| `curl` approve returns 404 | Code expired (>600s elapsed) | Node must generate a new code and resubmit |
| `curl` approve returns 409 | Code already used | Node must generate a new code |
| `nodes.jsonl` not growing | `service-ppn-pairing` has no write permission to its data dir | Check systemd unit `WorkingDirectory` and file permissions |
| Node does not come up on mesh after approval | WireGuard peer config not distributed | Check `wg show ppn0` on both nodes; peer may need manual peer-map refresh |
