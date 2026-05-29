---
schema: foundry-plan-v1
archive: project-console
topic: zero-jargon-pairing-protocol
created: 2026-05-21
updated: 2026-05-23
status: archived
cites: []
---

> **ARCHIVED 2026-05-23.** Pre-implementation research. Pairing Phases 1–2 shipped
> (d6267e39, 30874995). §7 (Operator UX / F11 panel layout) referenced by
> `BRIEF-pairing-phase3-4.md` Phase 4 section.

# Pairing System Design — Zero-Jargon Pairing Protocol

> Research brief produced by Opus system-design agent, 2026-05-21.
> Scope: replace `proofctl user add --key-file` with a consumer-grade pairing flow.

---

## 1. Current state

**Server side (`system-gateway-mba`):**
- `db.rs` — `users` table keyed on `fingerprint UNIQUE` (a trust ledger, not a credentials store)
- Operations: `add_user`, `find_user`, `disable_user`, `rotate_key`
- `auth.rs` — SHA256 fingerprint via `russh::keys::compute_fingerprint`
- `bin/proofctl.rs` — CLI ceremony today: `user add --tenant --key-file --role`

**Client side (`os-console`):**
- `mba_client.rs` — loads private key from config path, authenticates outbound via russh CLIENT
- `ssh_server.rs` — `auth_publickey` calls `find_user(&conn, &fingerprint)` — steady-state auth path; needs no changes

**Key architectural insight:** Pairing is purely a front door to `add_user()`. The steady-state `auth_publickey` path is already correct — only the *first registration experience* needs redesigning. Blast radius is minimal.

---

## 2. Design rationale

Three separable concerns — only (2) gets redesigned:

1. **Key generation** — user generates `~/.ssh/id_ed25519` once (SSH tooling handles this; out of scope)
2. **Fingerprint transport** — getting the public key fingerprint from user's machine to the operator — **this is the friction point**
3. **Authorization decision** — operator approves; `add_user()` executes — already handled by system-gateway-mba

**The fingerprint is public data.** The design goal is making transport frictionless and the authorization decision visible without requiring the user to understand any of this.

---

## 3. Consumer analogues

| Analogue | User sees | Operator sees | Friction |
|---|---|---|---|
| Bluetooth | 6-digit PIN on screen | Same PIN on pairing device | Low — match visual code |
| AirDrop | Name + avatar | Accept/Decline | Very low — one tap |
| WhatsApp Web | QR on screen | Scan with phone | Very low — scan |
| Nintendo Switch nearby play | 4-digit code | Enter same code | Low |
| NFC card tap | Nothing — just tap | Optional confirmation | Zero |
| **Current proofctl** | Raw instructions: copy key file, run CLI with flags | Copy-paste fingerprint, run proofctl manually | Very high |
| **Target: Pairing Code** | 6-char code on screen | Code appears in TUI review panel; one-key approval | Low |

---

## 4. The pairing artifact

A signed `PairingRequest` blob rendered three ways:

**Primary:** 6-character Crockford Base32 code with checksum (e.g., `A7K-R3M`)
- Typeable over phone/in-person
- 6 EFF-wordlist safety words for MITM detection (users read back words to operator)
- Short-lived: 10-minute TTL

**Secondary:** QR code rendered via Kitty graphics protocol (or Sixel fallback)
- Encodes the same signed blob
- Operator scans with any QR reader → browser tab shows approval UI

**Tertiary (offline/air-gapped):** `.pairing` file (signed blob)
- User transfers via USB, email attachment, etc.
- Operator imports via `proofctl pair import <file>` or F11 TUI file picker

The short code is a *claim ticket* (rendezvous pointer to a pending request on the server), **not** the key. The fingerprint travels server-side after the claim is redeemed.

---

## 5. State machines

### Client state machine

```
FIRST_RUN
    │ os-console launched; no user entry in proof.db
    ▼
UNPAIRED
    │ user presses [P] or chassis detects MBA INACTIVE with no prior pairing
    ▼
CODE_DISPLAYED ── shows QR + 6-char code + safety words
    │ user communicates code to operator (phone, chat, in-person)
    ▼
WAITING ── polling /v1/pairing/status/<code> every 5s
    │ operator approves
    ▼
VERIFYING ── MBA connect_mba() called; fingerprint checked
    │ auth succeeds
    ▼
PAIRED ── MBA LINK ACTIVE; chassis enters normal operation
```

On timeout (10min) → return to UNPAIRED. On rejection → show friendly message, regenerate code.

### Server state machine (F11 operator panel)

```
IDLE
    │ POST /v1/pairing/request arrives (from client claiming code)
    ▼
PENDING_REVIEW ── entry appears in F11 panel
    │ operator presses [A] (approve) or [R] (reject)
    ▼
OPERATOR_DECIDING
    │ approve path
    ▼
APPROVED ── add_user() called; response sent to polling client
    │ client verifies MBA link
    ▼
LINK_ESTABLISHED ── entry moves to "active" in F11 panel
```

---

## 6. New API endpoints for system-gateway-mba

| Method | Path | Description |
|---|---|---|
| `POST` | `/v1/pairing/request` | Client submits fingerprint + username; server returns 6-char code + safety words |
| `GET` | `/v1/pairing/status/<code>` | Client polls; returns `pending` / `approved` / `rejected` / `expired` |
| `GET` | `/v1/pairing/pending` | Operator: list all pending requests (for F11 TUI panel) |
| `POST` | `/v1/pairing/approve/<code>` | Operator: approve → triggers `add_user()` |
| `POST` | `/v1/pairing/reject/<code>` | Operator: reject with optional message to client |
| `POST` | `/v1/pairing/import` | Operator: import offline `.pairing` file |

New SQLite table in system-gateway-mba (`proof.db`):

```sql
CREATE TABLE pairing_requests (
    code        TEXT PRIMARY KEY,
    fingerprint TEXT NOT NULL,
    username    TEXT NOT NULL,
    tenant      TEXT NOT NULL,
    safety_words TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    expires_at  INTEGER NOT NULL,
    status      TEXT NOT NULL DEFAULT 'pending'
);
```

---

## 7. Operator UX

**F11 cartridge (app-console-system) — Pairing Review panel:**

```
┌─ Pairing Requests ───────────────────────────────────────────────┐
│  Code     User       Tenant     Requested    Safety Words        │
│  ─────────────────────────────────────────────────────────────── │
│  A7K-R3M  jennifer   woodfine   2 min ago    apple fence harbor  │
│  B9P-X2Q  peter      woodfine   just now     cloud marble river  │
│                                                                  │
│  [A] Approve   [R] Reject   [?] Show fingerprint                 │
└──────────────────────────────────────────────────────────────────┘
```

**Updated proofctl commands (jargon-free wording):**

```
proofctl pair list               # show pending requests
proofctl pair approve A7K-R3M   # approve by code
proofctl pair reject  A7K-R3M   # reject
proofctl pair import  user.pairing  # offline import
```

The old `user add --key-file` is preserved for operator bootstrap (the operator who owns the server must register themselves once the old way, or via a secure first-run wizard).

---

## 8. Security analysis

| Threat | Mitigation |
|---|---|
| Interception of code in transit | Code is a claim ticket only; attacker cannot redeem it without knowing the fingerprint (not transmitted with code) |
| MITM key-swap | 6 safety words derived from fingerprint; user reads back to operator — swap would change words |
| Replay attack | 10-minute TTL; code single-use (deleted on approval/rejection) |
| Brute-force code | ~60 billion combinations (Base32^6 with checksum); rate-limited to 3 attempts per 5 minutes |
| Request flooding | Rate limit: 1 pending request per username per hour |

**4 preserved cryptographic invariants:**
1. The fingerprint is derived directly from the Ed25519 public key — not transmitted as plaintext until the operator redeems the claim
2. `add_user()` is only called server-side after explicit operator approval
3. The steady-state `auth_publickey` path is unchanged
4. All `add_user()` calls are WORM-logged (SYS-ADR-19 compliant)

---

## 9. Implementation phases

**Phase MVP (2-3 days):** CLI pairing code flow
- New API endpoints in system-gateway-mba (actix-web routes)
- `pairing_requests` SQLite table + migration
- Client: `UNPAIRED → CODE_DISPLAYED → WAITING → PAIRED` state machine in chassis
- Operator: updated `proofctl pair` subcommands
- No QR, no TUI panel — just the 6-char code displayed in the pairing screen

**Phase 2 (3-4 days):** QR + F11 TUI panel
- QR code rendering (Kitty graphics via `viuer` or `ratatui-image` crate)
- F11 `app-console-system` cartridge with pending-requests panel
- Safety-word readback displayed alongside QR

**Phase 3 (1-2 days):** Offline import
- `.pairing` file format (signed JSON blob)
- `proofctl pair import` + F11 file picker modal

**Phase 4 (future):** NFC / browser bridge
- Out of scope for current milestone

---

## 10. Open questions for engineering

1. **HTTP control plane placement** — should the new pairing endpoints run on the same port as system-gateway-mba's SSH server (2222) or a separate HTTP port? Separate port (e.g., 8022) avoids SSH/HTTP muxing complexity.
2. **Wordlist language-neutrality** — bilingual doctrine requires Spanish support; EFF wordlist is English-only. Options: use numbers only, or translate the wordlist subset.
3. **Operator bootstrap problem** — the first operator cannot pair themselves via the new flow; they must use `proofctl user add` (the old CLI). Document as intentional operator-only ceremony.
4. **Identity ownership** — pairing requests identify the user by `username` from their config.toml; this must match the identity the operator expects. No way to verify username at pairing time (only fingerprint is cryptographic).
5. **WORM ledger audit** — SYS-ADR-19: does `pairing_requests` table need WORM properties, or is the existing `users` table sufficient as the authoritative ledger?
6. **Polling interval** — 5s polling for WAITING state; should switch to Server-Sent Events (SSE) to reduce load on system-gateway-mba.
7. **Rejection UX** — when operator rejects, what message does the user see? Must be non-technical and not reveal any system information.
8. **Key rotation** — when a user's SSH key changes (new laptop), the pairing flow should be the same; `rotate_key()` should be the server action, not `add_user()` + `disable_user()`.

---

## 11. File layout (new files)

```
system-gateway-mba/
├── src/
│   ├── pairing.rs          # new: PairingRequest, PairingCode, safety words
│   ├── pairing_db.rs       # new: pairing_requests table CRUD
│   └── pairing_routes.rs   # new: actix-web route handlers
app-console-keys/
└── src/
    └── pairing_state.rs    # new: client-side PairingState enum + polling logic
app-console-system/         # new crate: F11 cartridge
└── src/
    └── cartridge.rs        # new: SystemCartridge with pairing review panel
os-console/
└── src/
    └── main.rs             # updated: PairingState integration
```

---

*Next: read UX, UI, and Engineering briefs when agents complete. Cross-reference with this document to confirm state machine and API shape before implementation begins.*
