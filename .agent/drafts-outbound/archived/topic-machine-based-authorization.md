---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: pointsav/content-wiki-documentation
target_path: ./
target_filename: topic-machine-based-authorization.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: true
bilingual_pair: topic-machine-based-authorization.es.md
references:
  - woodfine-fleet-deployment/README.md (lines 68-102)
  - woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
  - woodfine-fleet-deployment/vault-privategit-source/guide-command-session.md
  - conventions/architecture-layer-catalog.md (system-gateway-mba entry)
  - pointsav-monorepo/app-console-content/src/auth.rs
  - pointsav-monorepo/app-console-content/src/db.rs
  - pairings.yaml (workspace root)
research_trail:
  source_commits:
    - "app-console-content/src/auth.rs — russh SSH fingerprint implementation"
    - "woodfine-fleet-deployment README.md — Geometric Security definition (lines 68-102)"
    - "guide-command-session.md — pairings.yaml schema and operator ceremony"
  prior_drafts: []
  citations: []
  operator_inputs:
    - "MBA is peer-to-peer between os-* services; NOT granted by PointSav Private Network (2026-05-20)"
    - "Pairing as Permission — the act of pairing IS the permission grant (2026-05-20)"
    - "No permissions database is legally accurate: SSH fingerprints are not credentials (2026-05-20)"
    - "system-gateway-mba is the server-side crate; app-console-keys is the MBA client presenting MBA LINK ACTIVE (2026-05-20)"
    - "PPN is infrastructure; os-* authorization is entirely separate and above it (2026-05-20)"
  related_files:
    - .agent/plans/os-console-platform.md
    - .agent/drafts-outbound/topic-pointsav-private-network.md
notes_for_editor: |
  Comprehensive first draft establishing the canonical MBA architecture description.
  This TOPIC is a foundational reference — other TOPICs and GUIDEs cite it.

  Refinement pass priorities:
  - Verify Doctrine claim numbers for MBA references against DOCTRINE.md
  - Register citation IDs in ~/Foundry/citations.yaml
  - Generate bilingual .es.md pair
  - Apply Bloomberg-article register; remove any AI-product-marketing vocabulary
  - Confirm system-gateway-mba Scaffold-coded state (check architecture-layer-catalog.md)
  - "Pairing as Permission" is a candidate for trademark consideration — flag to operator
  - Target length: ~1200-1500 words English
---

# Machine-Based Authorization

## The principle

Access to Woodfine's archives is not granted through credentials. It is granted through
a direct cryptographic pairing between two machines. If two systems are not explicitly
paired, they cannot communicate — regardless of what network they share. Access control
is defined by the topology of active pairings, an architecture Peter Woodfine describes
as Geometric Security.

There is no central permissions database, because there are no permissions in the
traditional sense. There are pairings. A pairing is a permission.

## What a pairing is

A pairing is a bilateral cryptographic relationship between two `os-*` services. When
`os-console` is paired with `os-totebox`, each machine holds the other's SSH public key
fingerprint. The act of establishing that relationship — registering keys and verifying
the connection — is the authorization event. Nothing else is required.

The pairing is recorded in the operator's `pairings.yaml` topology file and in the
local pairing registry on each machine. Neither record is a credentials database. A
public key fingerprint is not a credential — it cannot be stolen and replayed to
impersonate the key's owner. The pairing record says "these two machines trust each
other." It does not store a password, a token, or a shared secret.

## Peer-to-peer, not network-layer

MBA operates at the `os-*` application layer, above and independent of any network
infrastructure. This separation is central to the architecture.

The PointSav Private Network (PPN) — the WireGuard mesh, GCP relay, and fleet node
hardware — provides the transport that `os-*` services run on. The PPN carries packets
between virtual machines. It does not grant access to those machines' application data.
A node on the PPN cannot read the contents of `os-totebox` simply because it shares
the network.

This is a deliberate property. Even PointSav Digital Systems, as the vendor that built
and operates the PPN infrastructure, has no application-layer access to a customer's
Totebox Archives through the network. The vendor owns the pipes. The customer owns the
doors. The pipes cannot open the doors.

MBA is the mechanism that opens the doors: a direct peer-to-peer cryptographic
relationship established by the machine's operator, held locally on each machine,
verified at connection time without consulting any central authority.

## The two verification layers

A complete `os-console` → `os-totebox` connection crosses two independent security layers:

**Layer 1 — Network membership (PPN):** The connecting machine must be a registered
WireGuard peer. WireGuard uses Curve25519 public keys per node. If a machine's public
key is not in the WireGuard peer configuration, the network layer drops the traffic.
This layer is managed through PPN provisioning (see TOPIC: PointSav Private Network).

**Layer 2 — Application pairing (MBA):** The connecting `os-*` service must present a
registered public key fingerprint. The target service's `system-gateway-mba` component
checks the fingerprint against its local registry. If no pairing record exists for that
fingerprint, the connection is refused at the application layer — even if Layer 1 allowed
the traffic through.

A machine can be on the PPN without any MBA pairings. It can reach the network; it
cannot open any doors. MBA pairings exist above the network, not because of it.

## The system-gateway-mba crate

Each `os-*` service runs a `system-gateway-mba` component (a Rust crate in
`pointsav-monorepo`). This component:

- Holds the local pairing registry for that machine
- Verifies incoming connection fingerprints against the registry
- Returns `PAIRING_ACCEPTED` or `PAIRING_REJECTED` to the calling service
- Maintains an immutable audit log of all connection attempts
- Exposes a management interface for `proofctl` (the admin CLI)

`system-gateway-mba` stores SSH public key fingerprints. It does not store passwords,
tokens, or shared secrets. Public key fingerprints are mathematically derived from
private keys; possession of a fingerprint alone does not allow impersonation.

## The MBA client in os-console

`app-console-keys` — the base chassis of `os-console` — manages the outbound side of
MBA connections. It reads the pairing configuration, initiates connections to paired
`os-*` services, and presents the connection state in the console status bar.

Status bar indicators:
- `MBA LINK ACTIVE` — the paired os-* service verified the connection; full access
- `MBA LINK INACTIVE` — no pairing or connection refused; reason shown
- `MBA LINK PENDING` — connection attempt in progress

When `MBA LINK INACTIVE`, `os-console` operates in local-only mode: locally-cached
content is accessible, but backend service requests fail gracefully.

**Naming note:** The name `app-console-keys` refers to F-keys (keyboard function keys),
not cryptographic keys. MBA is implemented by `system-gateway-mba`, not by
`app-console-keys`. The naming distinction matters for architecture clarity.

## The pairing ceremony

Establishing a new MBA pairing is a P1 operator action:

1. Identify the SSH public key for the connecting machine
2. Register the key with the target service: `proofctl user add <username> --tenant <tenant> --key-file <path>`
3. Add a pairing entry to `pairings.yaml` at the workspace root
4. Verify: `os-console` status bar shows `MBA LINK ACTIVE`

The ceremony is performed once per machine pair. Subsequent connections are automatic
— the machines recognize each other without further operator action.

Key rotation: `proofctl user rotate-key <username> --key-file <new-path>` replaces the
registered fingerprint. No service restart required.

## No credentials database

The claim is precise: there is no credentials database to steal, because there are no
credentials.

What exists is a pairing registry — a topology record of which machines trust which
other machines, expressed as public key fingerprints. Fingerprints are not secrets.
Stealing a fingerprint does not enable connection as the key's owner, because connection
requires the corresponding private key, which never leaves the operator's hardware.

The pairing registry is not a username/password store, not a token vault, not a session
database. It is a topology record expressed in public mathematics.

This is Geometric Security: access is defined by the geometry of active pairings, not
by the transmission or memorization of shared secrets.

## os-* services connected via MBA

`os-console` maintains MBA pairings to:

| Service | Role |
|---|---|
| `os-totebox` | Totebox Archive — content, people, email, files |
| `os-orchestration` | Command hub — multi-archive aggregation |
| `os-privategit` | Air-gapped source control vault |
| `os-mediakit` | Media production and documentation wiki |
| `os-network-admin` | PPN mesh management authority |

Each pairing is independent. `os-console` can be active with some peers and inactive
with others simultaneously. The status bar shows the active-pairing summary at a glance.

## See also

- TOPIC: PointSav Private Network — the infrastructure layer MBA operates above
- TOPIC: os-console and app-console-keys — the platform presenting the MBA client
- GUIDE: MBA Pairing Ceremony — step-by-step operator procedure
- `conventions/architecture-layer-catalog.md` — `system-gateway-mba` crate entry
- `woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md` — operational reference
