---
artifact: convention
schema: foundry-convention-v1
title: Software Distribution Substrate
status: active
ratified: 2026-05-24
author: totebox@project-editorial
destination: ~/Foundry/conventions/software-distribution-substrate.md
cites:
  - artifact-classification
  - doctrine-claim-55-proposed
related_briefs:
  - BRIEF-software-distribution-substrate.md
brief_patches:
  # Apply these changes to ~/Foundry/.agent/briefs/BRIEF-software-distribution-substrate.md
  # before archiving this draft.
  - section: "§2 table — tier row"
    old: '"Apache 2.0" | Apache License, Version 2.0 | $1 USDC | Convenience binary. May fork, redistribute, compete. Fully open source.'
    new: '"PointSav Commercial (Apache-compatible)" | PointSav Commercial binary grant (Apache-2.0-equivalent rights) | $1 USDC | Convenience binary. May fork, redistribute, compete. No copyleft obligations.'
  - section: "§5 sidecar schema — license_tier field"
    old: "license_tier: apache     # apache → $1 USDC  |  fsl → $19 USDC"
    new: "license_tier: commercial # commercial → $1 USDC  |  fsl → $19 USDC"
  - section: "§5 sidecar schema — add notice_url after price_usdc"
    add: 'notice_url: ""          # required for commercial tier (Apache-2.0 §4(d)); see conventions/software-distribution-substrate.md §6'
  - section: "§10 checklist — license tier label display"
    old: '[ ] License tier label: display "Apache 2.0 — open source" or "FSL — source-available"'
    new: '[ ] License tier label: display "PointSav Commercial — Apache-compatible, no copyleft" or "FSL-1.1-ALv2 — source-available, non-compete"'
  - section: "§10 checklist — add AGPL §13 item"
    add: '[ ] AGPL §13 source link: footer links to github.com/pointsav/pointsav-monorepo (satisfies §13 network service obligation for AGPL server components)'
  - section: "§11 Doctrine note — convention cross-reference"
    old: "Specification at `conventions/software-distribution-substrate.md` (to be drafted; Command or project-software)."
    new: "Specification: `conventions/software-distribution-substrate.md` (ratified 2026-05-24)."
---

# Software Distribution Substrate

*Authoritative governance for binary distribution via software.pointsav.com.*
*Ratifies and supersedes the governance sections of `BRIEF-software-distribution-substrate.md` (§1–§3, §5, §11).*
*The brief remains the business-model and implementation reference; this document is the licensing and trademark authority.*

---

## §1. Three-path model

Three distinct access paths serve different customer segments:

| Path | Access | What | Cost |
|---|---|---|---|
| Open source | `github.com/pointsav/pointsav-monorepo` — clone + compile | Source code | Free |
| **software.pointsav.com** | Public storefront | **os-* images only** | $1 or $19 USDC (one-time) |
| **app-privategit-source** | License Key gated (os-* purchase unlocks) | **app-* and service-* packages** | $1 or $19 USDC (per package) |

**tool-*:** Internal operator tooling only. `soft_enabled: false`. Never distributed.

---

## §2. License tiers (two only)

| Tier label | Sidecar value | Price | Source license | Binary terms |
|---|---|---|---|---|
| **PointSav Commercial (Apache-compatible)** | `license_tier: commercial` | **$1 USDC** | AGPL-3.0-or-later | Apache-2.0-equivalent: no copyleft; may fork, redistribute, compete |
| **FSL-1.1-ALv2** | `license_tier: fsl` | **$19 USDC** | FSL-1.1-ALv2 | Non-compete restriction; source-readable; converts to Apache-2.0 after 2 years |

**Storefront display copy (canonical):**
- Commercial tier: `PointSav Commercial — Apache-compatible, no copyleft`
- FSL tier: `FSL-1.1-ALv2 — source-available, non-compete`

**Sidecar `license:` field:**
- Commercial tier: `Apache-2.0` — identifies the terms the customer receives
- FSL tier: `FSL-1.1-ALv2` — identifies the terms the customer receives

**No subscriptions. No automatic updates. No kill-switch.** Every download is a one-time transaction. An expired license key never blocks a running binary (see §4 expiry semantics). [Doctrine claim #54, #48, #28]

---

## §3. Copyright relicensing authority (commercial tier)

Woodfine Capital Projects Inc. (WCP Inc.) is the first owner of all AGPL-3.0-or-later
source code in `pointsav-monorepo` by operation of Canadian Copyright Act § 13(3)
(employer is first owner of in-scope employee work under a contract of service).
As copyright owner, WCP Inc. may distribute compiled binaries of that code under any
license terms it chooses, including terms that convey Apache-2.0-equivalent rights
to the purchaser.

**This is not a source-level relicensing.** The GitHub source code remains AGPL-3.0-or-later,
and downstream forks of the source must comply with AGPL terms. The commercial grant is
a separate, narrowly scoped license for the compiled binary artifact issued to a specific
purchaser in exchange for payment. It does not affect the AGPL obligations of any third
party who independently forks or distributes the source.

The commercial tier creates two simultaneous license chains:

1. **Source** (GitHub) → AGPL-3.0-or-later → community contributors and forks
2. **Binary** (software.pointsav.com) → PointSav Commercial (Apache-compatible) → paying customer

**Governance cross-reference:** `LICENSE-MATRIX.md §3.4` records this authority. The
`foundry-soft-v1` sidecar (§4) encodes it per binary artifact.

---

## §4. foundry-soft-v1 sidecar schema

Every distributed binary ships alongside a YAML sidecar. This is the canonical schema:

```yaml
schema: foundry-soft-v1
product: os-console
version: 0.3.1
class: os-image          # os-image | app-bundle | service-package
image_format: raw        # raw | ova | tarball
target_platform: x86_64-linux
layer: base              # base | extension
requires: []             # for extension: list of base image product IDs required
release_channel: stable  # stable | beta
sha256: <hex>
signature: <ed25519-hex>
source_commit: <sha>     # must be promoted canonical commit on pointsav-monorepo/main
license: Apache-2.0      # commercial tier: Apache-2.0  |  FSL tier: FSL-1.1-ALv2
license_tier: commercial # commercial → $1 USDC  |  fsl → $19 USDC
price_usdc: "1.00"       # string — avoids float representation
notice_url: "https://software.pointsav.com/notices/os-console/0.3.1/"
                         # required for commercial tier (Apache-2.0 §4(d) NOTICE)
                         # omit or empty string for FSL tier
```

**`license_tier` values:**
- `commercial` — PointSav Commercial (Apache-compatible) grant; $1 USDC
- `fsl` — FSL-1.1-ALv2 terms; $19 USDC

**Expiry semantics (non-negotiable):**
- `channel_expiry` in the License Key controls update-channel access only
- An expired key never blocks running an already-installed binary
- A production system on an expired key continues operating indefinitely
- No arbitrary kill-switch

---

## §5. AGPL §13 network service disclosure

The server components running software.pointsav.com — `app-privategit-marketplace`,
`app-privategit-source`, and `tool-wallet` — are licensed AGPL-3.0-or-later (they are
app-privategit-* and tool-* directories in `pointsav-monorepo`). AGPL §13 requires that
operators of AGPL network services offer the corresponding source code to users who
interact with the service over a network.

**Compliance requirement:** The storefront must include a visible link to the source
repository. This is a **project-software implementation item** (see
`BRIEF-software-distribution-substrate.md §10` checklist). Acceptable implementation:

```html
<!-- storefront footer -->
<a href="https://github.com/pointsav/pointsav-monorepo">Storefront source (AGPL-3.0)</a>
```

The source is already publicly available on GitHub. No additional hosting is required —
only the link in the storefront UI.

---

## §6. Apache-2.0 NOTICE obligation

Apache License, Version 2.0 §4(d) requires that distributions include or reference a
NOTICE file attributing copyright and any required third-party notices.

**Commercial tier** (`license_tier: commercial`, `license: Apache-2.0`):
- A NOTICE page must be served at `https://software.pointsav.com/notices/<product>/<version>/`
- The sidecar `notice_url` field must point to this URL (non-empty)
- Minimum NOTICE content: WCP Inc. copyright statement, SPDX identifier `Apache-2.0`,
  and any third-party library attributions required by vendored dependencies in the binary

**FSL tier** (`license_tier: fsl`, `license: FSL-1.1-ALv2`):
- NOTICE is optional — FSL-1.1-ALv2 does not impose a NOTICE requirement
- Set `notice_url` to empty string or omit the field

---

## §7. FSL version precision

All references to the FSL license in storefront copy, sidecar fields, and governance
documents must use the full SPDX form: **`FSL-1.1-ALv2`**

This form encodes:
- Version 1.1 of the Functional Source License
- Apache-2.0 as the "future license" (the license to which the FSL converts after 2 years)

Shorthand forms (`FSL`, `Functional Source License`) are ambiguous and must not appear
in any artifact that carries license-identification significance.

---

## §8. Cross-references

- `LICENSE-MATRIX.md §3.4` — human-readable authority for commercial tier relicensing
- `mapping/repo-license-map.yaml` — machine-readable per-repo and per-directory license assignments
- `BRIEF-software-distribution-substrate.md` — business model, payment rail, implementation checklist
- `conventions/artifact-classification.yaml` — SOFT- artifact type entry
- `licenses/PointSav-Commercial.txt` — full PointSav-Commercial license text

---

*Ratified: 2026-05-24 | Author: totebox@project-editorial → committed by Command Session*
*Specification of proposed Doctrine claim #55 — Software Distribution Substrate.*
