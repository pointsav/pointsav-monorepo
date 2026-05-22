# Layer 3 Privacy Compliance Report
Date: 2026-05-14
Checked by: command@claude-code (B2 parent-session execution)

## Summary

**3 violations found — 1 URGENT (private keys on GitHub), 2 structural**

---

## URGENT — WireGuard private keys on GitHub canonical

**Repo:** `woodfine/woodfine-fleet-deployment`
**Path:** `fleet-infrastructure-leased/spoke-configs/`
**Files confirmed live on GitHub:**
- `jennifer-macpro.conf` — contains `PrivateKey = WP+czadXEgkKu2BSIyfYxlGQlNVrZMrU6fTZ1zKlVWE=`
- `jennifer-phone.conf` — contains `PrivateKey = sBYaiHwpPD3Ib4zSsUXIegao5D+xcR2duYhS0Lw/5kU=`
- `peter-mexico.conf` — contains `PrivateKey = yKQSdUTJl6MSrrXHqBS2T/nwOWYqIvVKw5tsbONUNnk=`

Also present in same folder: `jennifer-phone-qr.png` (likely contains WireGuard QR code — also compromised if so).

**Assessment:** These are real WireGuard private keys committed to a public GitHub repository.
Any party with these private keys can impersonate these VPN peers on the PPN mesh.

**OPERATOR ACTION REQUIRED IMMEDIATELY:**
1. Rotate all three WireGuard keys — generate new key pairs for jennifer-macpro, jennifer-phone, peter-mexico
2. Update the WireGuard hub config to replace the public keys derived from these private keys
3. Remove `spoke-configs/` directory from `woodfine-fleet-deployment` entirely — private keys have no place in any public repo
4. Consider whether git history needs to be scrubbed (git-filter-repo or BFG) since keys were pushed to a public repo

**Triage:** Do NOT route anywhere. Operator must rotate keys before any other action.

---

## Structural violation — Layer 3 instance folder in woodfine-fleet-deployment

**Repo:** `woodfine/woodfine-fleet-deployment`
**Path:** `gateway-orchestration-gis-1/` (with `guide-gis-adding-a-chain.md` inside)

**Assessment:** `gateway-orchestration-gis-1` is a Layer 3 instance name (numbered instance).
Layer 2 showcase should be `gateway-orchestration-gis/` (no `-1` suffix). The GUIDE inside is legitimate
Layer 2 content but is in the wrong folder.

**Triage:** Move the GUIDE file:
- `gateway-orchestration-gis-1/guide-gis-adding-a-chain.md` → `gateway-orchestration-gis/guide-gis-adding-a-chain.md`
- Remove `gateway-orchestration-gis-1/` directory
- This is a GUIDE file — route to project-editorial for re-integration into the correct showcase folder.

**Local copy:** `/srv/foundry/customer/woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-gis-adding-a-chain.md`

---

## Structural violation — .key files in pointsav-monorepo

**Repo:** `pointsav/pointsav-monorepo`
**Files:**
- `system-gateway-mba/operator_private.key`
- `system-gateway-mba/vault_identity.key`

**Assessment:** File contents are a SHA-256 hex hash (`63f0da7d...`), not a PEM/base64 private key.
These appear to be challenge tokens or hash references used by `system-gateway-mba` for identity verification,
not operational private keys. **However,** the `.key` filename extension is misleading and the files should not
be committed to a public repo regardless of content. The Foundry identity store rule (§3) requires all keys
to be in `identity/` at 0600 permissions, never in source repos.

**Triage:**
- If these are hash references (not actual keys): move to `system-gateway-mba/assets/` and rename to `.sha256` or `.hash`
- If these are actual key material: rotate immediately and git-filter-repo out of history
- Route to project-intelligence for re-integration under correct path

---

## GitHub check — clean repos

| Repo | Result |
|---|---|
| pointsav/pointsav-design-system | CLEAN |
| pointsav/content-wiki-documentation | CLEAN |
| pointsav/pointsav-fleet-deployment | CLEAN |
| pointsav/pointsav-media-assets | CLEAN |
| pointsav/factory-release-engineering | CLEAN |
| woodfine/woodfine-media-assets | CLEAN |
| woodfine/content-wiki-corporate | CLEAN |
| woodfine/content-wiki-projects | CLEAN |
| woodfine/woodfine-design-bim | CLEAN |

---

## Local vendor/customer content — findings

### 10.50.x.x IP references in public wiki content

**File:** `vendor/content-wiki-documentation/infrastructure/telemetry-architecture.md` (+ `.es.md`)
**Finding:** `10.50.0.2:8081` and `10.50.0.2:8082` cited as telemetry relay endpoints.
**Assessment:** This is in a technical architecture article in the public documentation wiki. PPN IPs are
internal addressing but the article describes the architecture at a conceptual level. Not a Layer 3 secret
per se — the IP addresses are the subnet design, not credentials. However, this is borderline: the article
describes live operational infrastructure. Flag for project-editorial to assess whether to use placeholder
language (`<PPN-node-ip>:8081`) rather than actual subnet addresses.

**File:** `vendor/content-wiki-documentation/reference/style-guide-inventory.md`
**Finding:** `route-network-admin-1` appears as an instance name (with "pending" status note).
**Assessment:** A style-guide-inventory article in the public wiki mentioning a Layer 3 instance name
by its numbered form. Should use the Layer 2 showcase name `route-network-admin` instead.
Route to project-editorial for correction.

### cluster-totebox-personnel-1 in source code

Multiple occurrences of `cluster-totebox-personnel-1` in Rust source files (service-email, service-content,
service-extraction, service-http, tool-egress-pull, service-egress, xtask, system-gateway-mba/service-unpacker.py).

**Assessment:** These are runtime path constants in software that deploys TO cluster-totebox-personnel-1.
This is Layer 1 software with hardcoded deployment paths. This is a software architecture decision, not
a Layer 3 leakage — the code is the product, and it must know where to deploy. These references do not
expose secrets or operational state. **Not a violation** per the Layer 3 privacy rule (the rule covers
instance config, keys, and state — not software that targets an instance by name).

However, hardcoded paths are a software quality concern — they should be runtime configuration, not
compiled-in constants. Flag for project-intelligence as a follow-up software task.

### PrivateKey template strings in service-vpn scripts

**Files:** `vendor/pointsav-monorepo/service-vpn/scripts/` — several shell scripts contain `PrivateKey = ${PETER_PRIV}` etc.
**Assessment:** These are shell variable substitutions in provisioning scripts, not literal key values.
The key values are never written to the repo — they are injected at runtime via environment variables.
**Not a violation.**

**Files:** `vendor/pointsav-monorepo/service-vpn/spokes/peter-mexico.conf` and `jennifer-macpro.conf`
**Assessment:** These files have `PrivateKey = ` with an empty value (no key material). The actual keys
are NOT in these template files. **Not a violation for these vendor/ copies** — the problem is the
customer/ copies in `woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/` which DO
have real key values (see URGENT section above).

---

## Files flagged for operator action

| File | Action | Priority |
|---|---|---|
| `woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/jennifer-macpro.conf` | Rotate key + git rm + consider history scrub | URGENT |
| `woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/jennifer-phone.conf` | Rotate key + git rm + consider history scrub | URGENT |
| `woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/peter-mexico.conf` | Rotate key + git rm + consider history scrub | URGENT |
| `woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/jennifer-phone-qr.png` | Remove + consider history scrub | URGENT |
| `woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-gis-adding-a-chain.md` | Move to `gateway-orchestration-gis/` | High |
| `pointsav-monorepo/system-gateway-mba/operator_private.key` | Assess + rename or remove | Medium |
| `pointsav-monorepo/system-gateway-mba/vault_identity.key` | Assess + rename or remove | Medium |
| `content-wiki-documentation/infrastructure/telemetry-architecture.md` | Review PPN IP exposure | Low |
| `content-wiki-documentation/reference/style-guide-inventory.md` | Replace `route-network-admin-1` with `route-network-admin` | Low |
