---
description: Compare a proposed change against the authoritative specs and report any conflicts.
---

Before a significant change lands, verify it is consistent with the
normative specifications in `specs/`:

1. Read `specs/SLM-STACK.md` and `specs/YOYO-COMPUTE.md`.
2. Identify any section that governs the change under review.
3. If the change is consistent with the spec: report "consistent" and
   the section numbers that govern.
4. If the change contradicts the spec: stop and report the conflict.
   Do not proceed without either:
   - Revising the change to fit the spec, or
   - An ADR that explicitly supersedes the relevant spec section.

A change that silently overrides a spec is a governance violation.
This command exists to catch that class of error before it reaches PR.
