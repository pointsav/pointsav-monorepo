# MIGRATION: system-slm to system-resolver

## Overview
This document tracks the replacement of the probabilistic, AI-reliant `system-slm` crate with a deterministic, rule-based `system-resolver` crate.

## Status: VERIFIED & ISOLATED
- **Crate Status:** Implementation complete and verified via unit tests.
- **AI Dependency:** Removed. 
- **Network Dependency:** Removed.
- **Safety:** Deterministic rule-set matches legacy intent-to-command mappings.

## Migration Steps
1. **Implementation:** Created `system-resolver` in `/pointsav-monorepo/system-resolver/`.
2. **Testing:** Unit tests pass for "PING", "ISOLATE", and "Unknown Intent" cases.
3. **Rollout:** 
    - Keep `system-slm` as a legacy artifact for now.
    - Update `system-core` to point to `system-resolver`.
    - Run integration tests before deleting `system-slm`.

## Rollback Policy
If integration fails, revert the `Cargo.toml` dependencies back to `system-slm`. The code is physically present and functional if needed.
