---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: woodfine/woodfine-fleet-deployment
target_path: node-console-operator/
target_filename: guide-mba-pairing-ceremony.md
target_cluster: node-console-operator
audience: operators
bcsc_class: current-fact
language_protocol: PROSE-GUIDE
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: false
references:
  - woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
  - woodfine-fleet-deployment/vault-privategit-source/guide-command-session.md
  - pointsav-monorepo/app-console-content/src/bin/proofctl.rs
  - pairings.yaml (workspace root)
  - .agent/plans/os-console-platform.md (§5 MBA peer-to-peer)
research_trail:
  source_commits:
    - "app-console-content/src/bin/proofctl.rs — proofctl user add/list/disable/rotate-key commands"
    - "guide-command-session.md lines 60-96 — pairings.yaml schema and provisioning steps"
    - "guide-command-ledger.md — MBA LINK ACTIVE, machine-based authorization definition"
  prior_drafts: []
  citations: []
  operator_inputs:
    - "MBA pairing ceremony: register SSH key via proofctl + add pairings.yaml entry (2026-05-20)"
    - "Adding a pairing is a P1 operator action; pairing history is immutable ledger (2026-05-20)"
  related_files:
    - .agent/drafts-outbound/topic-machine-based-authorization.md
    - .agent/drafts-outbound/guide-os-console-operator.md
notes_for_editor: |
  Comprehensive first draft. Operational step-by-step for the MBA pairing ceremony.
  Routes to node-console-operator/ in woodfine-fleet-deployment.

  Refinement priorities:
  - Apply foundry-doc-v1 schema header when promoting to woodfine-fleet-deployment
  - Verify proofctl command syntax against current proofctl.rs source
  - Confirm pairings.yaml schema fields are current
  - Add copyright footer per woodfine-fleet-deployment convention
  - English only (GUIDE files are not bilingual)
---

# GUIDE: MBA Pairing Ceremony

**Audience:** Operators setting up a new `os-console` ↔ `os-*` connection
**Prerequisite:** `os-console` is installed; target `os-*` service is running
**Authority:** P1 operator action — only the workspace operator may add pairings

---

## I. Overview

Machine-Based Authorization (MBA) connects `os-console` to `os-*` services via direct
peer-to-peer cryptographic pairing. This guide covers:

- Establishing a new pairing (`os-console` ↔ `os-totebox`, `os-orchestration`, etc.)
- Verifying the connection
- Rotating keys
- Revoking access

A pairing is a permanent relationship unless explicitly revoked. Adding a pairing is an
immutable ledger event — the history of pairings is preserved for audit even after revocation.

---

## II. Prerequisites

Before performing the pairing ceremony:

1. `os-console` is installed on the operator machine (Linux Mint or macOS)
2. The target `os-*` service is running and reachable on the network
3. The target service has `system-gateway-mba` running
4. The operator has the SSH public key for the connecting machine
5. The operator has write access to `pairings.yaml` at the workspace root

---

## III. Step 1 — Identify the connecting machine's SSH public key

The SSH public key is the identity credential for the MBA pairing. It is the public
half of an Ed25519 SSH key pair on the operator's machine.

```bash
# View the public key (replace with your key file path)
cat ~/.ssh/id_ed25519.pub

# If you need to generate a key:
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -C "jennifer@woodfine-console"
```

The key looks like:
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... jennifer@woodfine-console
```

Keep the public key (`.pub`) file accessible. The private key never leaves this machine.

---

## IV. Step 2 — Register the key with the target service

Use `proofctl` on the target `os-*` service to register the public key. `proofctl` is
the admin CLI for `system-gateway-mba`.

```bash
# On the target os-* service machine:
proofctl user add jennifer \
  --tenant woodfine \
  --key-file /path/to/jennifer_console.pub \
  --role editor
```

**Parameters:**
- `jennifer` — username for this operator on this service
- `--tenant woodfine` — tenant scope (`woodfine` or `pointsav`)
- `--key-file` — path to the SSH public key file
- `--role editor` — access role (`editor` is the standard role; admin roles are granted separately)

**Expected output:**
```
Added jennifer@woodfine  SHA256:abc123...xyz
```

The fingerprint shown confirms the key was registered. Note it for verification in Step 4.

---

## V. Step 3 — Add the pairing entry to pairings.yaml

Add a new entry to `pairings.yaml` at the workspace root. This is the topology record
that `os-console` reads to know which services it should attempt to connect to.

```yaml
# pairings.yaml — add this entry

- cluster_name: project-totebox          # or the relevant project name
  module_id: totebox                     # the service module identifier
  slm_endpoint: http://localhost:9080    # Doorman endpoint (always 9080)
  paired_on: 2026-05-20                  # ISO 8601 date of pairing
  type: active
  branch: cluster/project-totebox        # feature branch in pointsav-monorepo
```

Commit `pairings.yaml` after adding the entry. Pairing history is an immutable ledger
record — do not remove or modify existing entries; add new entries and set
`type: revoked` when access is withdrawn.

---

## VI. Step 4 — Verify: MBA LINK ACTIVE

Start or restart `os-console` on the operator machine. Observe the status bar.

**Expected status bar:**
```
jennifer@woodfine | MBA LINK ACTIVE | F4: Content | Tier A | 00:00:01
```

If `MBA LINK ACTIVE` does not appear:

| Status shown | Diagnosis | Action |
|---|---|---|
| `MBA LINK INACTIVE: key not registered` | The key was not registered on the target service | Repeat Step 2; confirm key file path |
| `MBA LINK INACTIVE: service unreachable` | Network issue or service not running | Check that the target service is running; check network |
| `MBA LINK INACTIVE: fingerprint mismatch` | The registered key does not match the connecting key | `proofctl user rotate-key` (see §VIII) |
| `MBA LINK PENDING` | Connection attempt in progress | Wait 10 seconds; if unchanged, check logs |

---

## VII. Listing active registrations

To see all operators registered with a target service:

```bash
proofctl user list
```

**Expected output:**
```
ID  Username    Tenant    Role    Fingerprint           Active  Created
1   jennifer    woodfine  editor  SHA256:abc123...xyz   yes     2026-05-20
2   peter       woodfine  editor  SHA256:def456...uvw   yes     2026-05-20
```

---

## VIII. Key rotation

When an operator's hardware changes or a key is rotated for security:

```bash
# On the target os-* service machine:
proofctl user rotate-key jennifer \
  --key-file /path/to/jennifer_new.pub
```

No service restart required. No sshd reload needed. The new fingerprint takes effect
immediately for the next connection attempt.

Update `pairings.yaml` if the `paired_on` date should reflect the rotation — this is
optional but recommended for audit clarity.

---

## IX. Revoking access

To permanently revoke an operator's access:

```bash
proofctl user disable jennifer
```

**Expected output:**
```
Disabled: jennifer@woodfine
```

The user record is preserved in the audit log. Set `type: revoked` in the corresponding
`pairings.yaml` entry. Do not delete the entry — the pairing history is an immutable
ledger record.

---

## X. Pairing multiple os-* services

`os-console` can maintain simultaneous MBA connections to multiple `os-*` services.
Repeat Steps 2–4 for each target service:

| Target service | Module ID | Notes |
|---|---|---|
| `os-totebox` | totebox | Totebox Archive — content, people, email, files |
| `os-orchestration` | orchestration | Command hub |
| `os-privategit` | privategit | Air-gapped source control vault |
| `os-mediakit` | mediakit | Media production and documentation wiki |
| `os-network-admin` | network-admin | PPN mesh management |

The `os-console` status bar shows aggregate MBA state. Individual service states are
visible in the F11 System cartridge (when installed).

---

## See also

- TOPIC: Machine-Based Authorization — full architectural description
- GUIDE: os-console Operator Reference — console startup and F-key reference
- `node-console-operator/guide-command-ledger.md` — MBA definition and Zero-Form context
