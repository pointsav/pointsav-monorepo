---
description: Run cargo-deny's licence check and summarise the result.
---

Run `cargo deny check licenses` from the workspace root. Summarise the
result as follows:

- If all dependencies pass the allow-list in `deny.toml`: report "clean"
  with the count of crates checked.
- If any dependency fails: identify which crate, which licence it
  carries, and what the allowed alternatives are. Suggest the
  minimal-impact fix (upgrade, swap, or — only as a last resort —
  propose an ADR for an exception).

Do not modify `deny.toml`. The policy is intentional; the fix is in the
dependency tree.
