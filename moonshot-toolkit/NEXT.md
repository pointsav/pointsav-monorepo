# NEXT.md — moonshot-toolkit

> Last updated: 2026-05-27
> Read at session start. Update before session end.

---

## Right now

- Nothing in progress. Phase 1B (spec + plan + CLI) is complete at v0.1.3.
  Phase 1C (seL4 cross-compile hello-world, task #14) is blocked on 3A
  architecture decisions — see Blocked section below.

## Queue

- **CHANGELOG.md** — not yet created. Add entries for v0.1.3 / v0.1.x per workspace §7 versioning convention when next touching this crate.
- **Sigstore Cosign + customer-apex cosignature** — `plan_hash` field is in place; cosignature emission deferred until `moonshot-toolkit build` produces real binary outputs (post-#14).

## Blocked

- **seL4 hello-world build (#14)** — Blocked on three Group 3A decisions
  not yet made (as of 2026-05-27):
  1. Cross-compile toolchain: `aarch64-linux-gnu-gcc` (system dep), Nix,
     Bazel, or Docker?
  2. seL4 source vendoring: `vendor-sel4-kernel/` snapshot (already
     present), git submodule, or Cargo `build.rs` fetch?
  3. Toolchain installation ownership: operator-manual, Master Session,
     or Task-tier automated?
  Once these are decided → Group 4 seL4 cross-compile work can begin.

## Deferred

- `build-totebox.sh` legacy shell sketch — kept as migration reference
  until `moonshot-toolkit build` produces a bootable image end-to-end
  (post-#14). Remove then.
- `no_std` eligibility — not a constraint for the build orchestrator
  (runs on the workspace VM, not in the kernel). No action needed.

## Recently done

- 2026-05-27: NEXT.md updated to v0.1.3 delivered state — tasks #35/#36/#37
  closed; stale "Right now" cleared; Group 3A blockers for #14 made explicit.
- 2026-04-27 (Phase 1B): `src/spec.rs` (445 lines, 12 tests) + `src/plan.rs`
  (310 lines, 10 tests) + `src/main.rs` CLI rewrite (241 lines, 8 tests).
  30 tests total (`cargo test --all-targets`). `validate` / `plan` / `build`
  (stub) subcommands working. v0.1.3.
- 2026-04-27: framework §9 activation — CLAUDE.md / AGENTS.md / NEXT.md /
  ARCHITECTURE.md / DEVELOPMENT.md created; bilingual READMEs; workspace
  member; registry row Scaffold-coded → Active.
