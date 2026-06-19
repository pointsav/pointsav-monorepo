---
artifact: guide
schema: foundry-draft-v1
title: "Machine Pairing: Authorizing a Computer for Totebox Access (F11)"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-49, claim-34, SYS-ADR-10]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, BRIEF-project-console-master.md, system-core-v1.0.0, app-console-system-source]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: F11 SystemCartridge source + pairing-server Cargo.toml reviewed
---

# Machine Pairing: Authorizing a Computer for Totebox Access (F11)

Machine pairing is how your Totebox recognizes and authorizes a specific computer. Unlike
a password or an account — which authorize a person — machine pairing authorizes the
computer itself. The distinction matters: if someone else logs into your computer, they
do not automatically have Totebox access. The machine holds the authorization, not the
account.

---

## What Pairing Authorizes

When you pair a computer with a Totebox, the Totebox records:

- **Machine fingerprint** — a unique identifier derived from the computer's hardware and
  operating environment. Not a password; not a username.
- **Authorized cartridges** — which F-key cartridges this machine may open (F2 People,
  F4 Content, etc.). The administrator controls this list.
- **Access period** — how long the pairing is valid. The administrator may set a time
  limit (e.g., 90 days) or leave it open-ended.

When os-console launches on a paired machine, the pairing token is verified automatically.
No login prompt appears. No password is entered. The cartridges activate within seconds.

---

## The Pairing Process

**For the operator (computer side):**

1. Launch os-console on the computer you want to pair.
2. Press **F11** to open the SystemCartridge.
3. Select **Pair this machine** from the menu.
4. A QR code appears. This encodes your machine fingerprint and a pairing request token.
5. Tell your Totebox administrator that a pairing request is waiting.
6. Wait for approval. When approved, the QR code screen closes and the cartridges activate.

**For the Totebox administrator (Totebox side):**

1. Open the Totebox operator panel (on the Totebox itself, or via the admin interface).
2. Navigate to **Paired Machines**.
3. A pending request appears with the machine fingerprint and the operator's name (if provided
   in the pairing request).
4. Review the request. Select which cartridges to authorize for this machine.
5. Approve or reject the request.
6. If approved, the operator's os-console activates the granted cartridges within 30 seconds.

---

## What Pairing Looks Like in Practice

A practical example: Jennifer in the accounts team needs access to the Bookkeeper (F6)
and People (F2) cartridges. She does not need Content (F4) or SLM (F9).

1. Jennifer launches os-console on her workstation and presses F11.
2. She selects Pair this machine. A QR code appears.
3. She tells the Totebox administrator (Peter) that she needs pairing.
4. Peter opens the admin panel, sees Jennifer's machine fingerprint listed as pending.
5. Peter approves, granting access to F2 and F6 only.
6. Jennifer's os-console activates F2 (People) and F6 (Bookkeeper). F4 and F9 remain
   greyed out. She cannot access them regardless of what keys she presses.

The cartridge restriction is enforced by the Totebox, not by os-console's display. Even
if os-console were modified to show all F-keys, the Totebox would reject any request
from an unauthorized cartridge.

---

## Viewing Active Pairings

Press **F11 → Active Pairings** to see:
- Which machines are currently paired with this Totebox
- What cartridges each machine is authorized for
- When each pairing was established and when it expires
- Last connection time for each machine

This view is available to all paired machines. The administrator panel additionally shows
the machine fingerprint and can filter by cartridge access level.

---

## Revoking a Pairing

Revocation is immediate. When a Totebox administrator removes a pairing:

1. The pairing record is deleted from the Totebox.
2. Any in-progress os-console session on that machine loses connection within seconds.
3. The revoked machine cannot reconnect. Re-pairing requires a new approval.

Revocation is the right response to:
- A lost or stolen computer
- An employee leaving the organization
- A security incident where a machine may be compromised
- Periodic access review (revoke all; re-pair only active machines)

**No action is required on the revoked machine.** The Totebox holds the authorization.
When the Totebox says no, os-console cannot proceed regardless of what is on the machine.

---

## Multiple Toteboxes

A single computer can be paired with multiple Toteboxes. Each pairing is independent:

- Press **F11 → Switch Totebox** to select which Totebox to connect to.
- Each Totebox has its own pairing approval process.
- Cartridges from different Toteboxes are not mixed. When connected to Totebox A, you
  see Totebox A's data. Switch to Totebox B and you see Totebox B's data.
- Revoking a pairing on one Totebox does not affect pairings on other Toteboxes.

If your organization uses os-orchestration to federate multiple Toteboxes, the connection
is managed through a single gateway. Your machine is paired with the os-orchestration
hub, which holds capability grants from each connected Totebox. The F-key cartridges
aggregate data from all authorized Toteboxes in a single session view.

---

## Security Properties

Machine pairing provides authorization that is:

**Machine-scoped, not user-scoped.** The authorization follows the hardware, not the
account. A user who switches computers must re-pair on the new machine.

**Cartridge-scoped.** A machine authorized for F2 (People) has no access to F4 (Content)
unless explicitly granted. Access is not binary (full / none) but per-cartridge.

**Revocable with immediate effect.** Revocation propagates to the Totebox's authorization
state and takes effect on the next IPC attempt from os-console. There is no cached
credential that remains valid after revocation.

**Audit-logged.** Every pairing approval, revocation, and cartridge access attempt is
recorded in the Totebox's WORM audit log (F12 audit trail, SYS-ADR-10). The log is
append-only and cannot be modified after the fact.
