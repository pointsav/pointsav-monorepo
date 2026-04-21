# ADR-0003: AGPL-3.0 for PointSav-authored code

- **Status:** accepted
- **Date:** 2026-04-20
- **Deciders:** Peter M. Woodfine, Jennifer Woodfine
- **Supersedes:** [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §8
  recommendation of Apache-2.0 for PointSav's own code.

## Context

[ADR-0001](./0001-rust-end-to-end.md) settled on Rust end-to-end with
a permissive-only dependency graph. It did not settle which licence
PointSav's own code should carry.

[`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §8 recommended
Apache-2.0 for PointSav's own code, on the grounds that the explicit
patent grant is valuable in institutional markets and that the Rust
community norm is MIT-OR-Apache-2.0 dual-licensing. This was the
default in the absence of an org-wide policy.

The policy now exists. `factory-release-engineering/` establishes a
canonical per-repo licence mapping for the PointSav platform; its §3
License inventory maps every `service-*` repository — including
service-slm — to AGPL-3.0-only. Introducing Apache-2.0 as a one-off for
service-slm would create an organisational-consistency drag without a
corresponding benefit large enough to justify it.

AGPL-3.0 is OSI-approved, SPDX-identified (`AGPL-3.0-only`), and its
copyleft reach extends to network use per §13. The permissive-only
dependency discipline this repository enforces via
[`deny.toml`](../../deny.toml) is unaffected: inbound dependencies remain
restricted to MIT, Apache-2.0, BSD, ISC, and other compatible permissive
licences.

## Decision

PointSav-authored code in this repository is licensed under **AGPL-3.0-only**.

- Every file carries the SPDX header `// SPDX-License-Identifier: AGPL-3.0-only`
  and the copyright line `// Copyright (c) 2026 Woodfine Capital Projects Inc.`
- The [`LICENSE`](../../LICENSE) file at the repository root is the
  canonical AGPL-3.0 text, sourced verbatim from
  `factory-release-engineering/licenses/AGPL-3.0.txt`.
- The [`deny.toml`](../../deny.toml) dependency allow-list is unchanged:
  we consume permissive (MIT, Apache-2.0, BSD, ISC, Unicode-DFS, MPL-2.0
  file-level, Zlib) and publish AGPL-3.0-only.
- AGPL-3.0-only applies to our own code only and does not appear in the
  dependency allow-list. A hypothetical future AGPL-3.0-only dependency
  would require its own ADR and a deliberate exception in `deny.toml`.

This decision supersedes SLM-STACK §8's Apache-2.0 recommendation.

## Rationale

- **Organisational consistency.** The canonical licence mapping in
  `factory-release-engineering/README.md §3` assigns AGPL-3.0-only to all
  `service-*` repositories. Following that mapping reduces licensing
  review overhead for internal customers and for institutional partners
  auditing the portfolio as a whole.

- **Copyleft alignment with commercial strategy.** AGPL-3.0 §13 extends
  the copyleft reach to users who interact with the software over a
  network — closing the SaaS loophole that a plain GPL-3.0 or a
  permissive licence would leave open. For a project where the commercial
  moat is the doorman, the ledger, and the adapter library, this
  reciprocity is consistent with how Woodfine prefers to see its code
  commercialised by third parties: contributions back, or a separate
  commercial licence through the PointSav Commercial channel documented
  in `factory-release-engineering/README.md §3`.

- **OSI approval with copyleft reach.** AGPL-3.0 is OSI-approved, which
  is non-negotiable for institutional buyers who audit licence
  conformance, while providing copyleft protections that pure Apache-2.0
  would not.

### Trade-off accepted

Two costs of this decision are worth naming explicitly so they are not
surprises later:

- **Narrower downstream adoption.** Some commercial downstream adopters
  will not embed AGPL-3.0 code into closed products. This reduces the
  pool of potential embedders. In practice it aligns with Woodfine's
  commercial preferences (see the second rationale bullet above), since
  commercial embedders should be routed to the PointSav Commercial
  licence rather than consuming AGPLv3 directly.

- **Patent grant language differs from Apache-2.0.** AGPL-3.0 §11 grants
  patent rights with different language and scope than Apache-2.0 §3. For
  institutional customers with patent-heavy portfolios, this is a
  talking point that may need addressing case-by-case.

### Alternatives considered

- **Apache-2.0** (the SLM-STACK §8 recommendation). Strongest
  institutional-market story, best downstream adoption, explicit patent
  grant. Rejected on organisational-consistency grounds per
  `factory-release-engineering/README.md §3`.
- **MIT OR Apache-2.0 dual-licence** (the Rust community norm). Maximum
  downstream adoption. Rejected for the same reason as Apache-2.0 plus
  the additional overhead of maintaining two licence texts.
- **Split-licence at crate granularity** (permissive infrastructure,
  copyleft product-differentiation). Attractive on the merits but high
  maintenance overhead (two LICENSE files, per-crate SPDX discipline,
  reviewer confusion). Deferred to a future ADR if a concrete need
  arises.

## Consequences

- **Positive.**
  - Consistent with the canonical licence mapping in
    `factory-release-engineering/`.
  - Copyleft on own code without copyleft-contamination fear on
    dependencies (`deny.toml` ensures the dependency graph stays
    permissive).
  - OSI-approved; institutional-audit compliant.
  - Network-use copyleft (AGPL §13) closes the SaaS loophole for
    unauthorised commercial hosting.
- **Negative.**
  - Contradicts the SLM-STACK §8 recommendation. This ADR makes the
    override explicit; future readers of SLM-STACK should follow this
    ADR.
  - Some downstream adopters will pass. Accepted; routed to PointSav
    Commercial where appropriate.
- **Follow-up.**
  - The [`LICENSE`](../../LICENSE) file is the canonical AGPL-3.0 text,
    sourced from `factory-release-engineering/licenses/AGPL-3.0.txt`.
  - All SPDX headers in the repository reference AGPL-3.0-only.
  - [`CONTRIBUTING.md`](../../CONTRIBUTING.md) documents the CLA
    requirement enforced via CLA Assistant, consistent with
    `factory-release-engineering/` policy for AGPLv3 repos.
  - The [`NOTICE`](../../NOTICE) file names Woodfine Capital
    Projects Inc. as copyright holder and points to Peter's
    contact details for licence questions.

## References

- [OSI AGPL-3.0 page](https://opensource.org/license/agpl-v3)
- [GNU AGPL-3.0 canonical text](https://www.gnu.org/licenses/agpl-3.0.html)
- `factory-release-engineering/licenses/AGPL-3.0.txt` — canonical text
  used for `LICENSE` and `LICENSES/AGPL-3.0-only.txt`.
- `factory-release-engineering/README.md §3` — licence inventory for
  all PointSav platform repositories.
- [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §8 (superseded by
  this ADR).
- [`deny.toml`](../../deny.toml) (dependency allow-list, unchanged
  by this ADR).
