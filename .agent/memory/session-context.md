# Session Context — project-system cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

---

## 2026-05-28 — Totebox Session — claude-code (claude-sonnet-4-6)

**Done this session:**
- moonshot-toolkit v0.2.1 `6b59fd0`: corrected AssembleImage error message —
  `microkit` PyPI package is an unrelated Flask helper; real SDK is a tarball
  from github.com/seL4/microkit/releases
- Phase 1C.c DONE `d550217` (Peter Woodfine): seL4 qemu-arm-virt AArch64 QEMU boot confirmed
  - Full boot: elfloader → seL4 kernel → hello-rootserver → "hello from seL4 rootserver"
  - Root cause 1: KernelVerificationBuild=ON silently disabled CONFIG_PRINTING (the CMake
    cache shows `_DISABLED:INTERNAL=TRUE` with no warning); rebuilt kernel with
    KernelVerificationBuild=OFF, KernelDebugBuild=ON, KernelPrinting=ON
  - Root cause 2: GNU cpio --create --format=newc adds ~11 extra bytes per entry beyond
    4-byte alignment; replaced with gen_cpio.py using exact ALIGN4 formula
  - Root cause 3: QEMU -m 512M insufficient; kernel DTB describes [40000000..80000000) (1GB);
    boot with -m 1G
  - Elfloader built manually from vendor-sel4-tools/ source (45 C/ASM sources + libcpio)
  - Committed: hello-rootserver source + build-support/ (gen_cpio.py, build-elfloader.sh,
    libcpio, gen_config headers)
  - NEXT.md updated: Phase 1C.c marked complete

**Pending / carry-forward:**
- Phase 1C.d (AssembleImage): blocked on Microkit SDK tarball or Rust image assembler
- Stage-6 for system-core+system-ledger v1.0.0: outbox `project-system-20260527-stage6-v100`
- Image-signing key for Veriexec: outbox `project-system-20260527-image-signing-key`
- Bench #9 quiet-VM re-run: verify_inclusion_proof composed 1024-leaf (load avg < 1.0)
- PhD thesis pre-publication checklist
- fleet-deployment file-mode drift: Command Session review needed

**Operator preferences:**
- Auto Mode active; all decisions proceed without stopping for clarifications

---

## 2026-05-27 (continued) — Totebox Session — claude-code (claude-sonnet-4-6)

**Done this session (continuation of prior context):**
- Group 3A + 3D architecture decisions: all 7 resolved
- Phase 1C.a DONE `34a1111`: moonshot-toolkit v0.2.0, real aarch64-linux-gnu-gcc
- Phase 1C.b DONE: seL4 kernel.elf built, KernelPlatform=qemu-arm-virt, entry 0xffffff8040000000

**Pending / carry-forward:**
- (see 2026-05-28 entry above for updated status)

**Operator preferences:**
- Auto Mode active

---

## 2026-05-27 — Totebox Session — claude-code (claude-sonnet-4-6)

**Done this session:**
- Full startup sequence (12 steps per AGENT.md)
- Direction-finding: 4 Q&A questions answered; session agenda agreed (v1.0.0 bumps + Stage-6)
- PhD thesis BRIEF created: `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md` (719 lines)
  — Yale PhD thesis-quality research on system-* layer, service-fs, seL4/NetBSD, Totebox transferability
  — 12 Opus research agents used for source material; schema foundry-draft-v1 / PROSE-RESEARCH
- Committed BRIEF: `edd4928` (Jennifer Woodfine)
- Fixed pointsav-monorepo branch: switched from `main` to `cluster/project-system`
- v1.0.0 version bumps: system-core 0.2.0→1.0.0, system-ledger 0.2.1→1.0.0
  — Cargo.toml updated, CLAUDE.md headers updated, CHANGELOG.md created for both
  — 62+47 tests passing; `c2ae1e9` (Jennifer Woodfine)
- Outbox: `project-system-20260527-stage6-v100` prepended (Stage-6 ready signal to Command)
- Old gate-decisions outbox msg marked actioned
- cleanup-log.md session entry added
- project-system-todo.md: v1.0.0 items marked complete

**Pending / carry-forward:**
- Stage-6 not yet run (awaiting Command Session to process outbox)
- `pointsav-fleet-deployment` file-mode drift (32 files 644→755 + `.claude/rules/project-registry.md` structural deletion) — surfaced in outbox via cleanup-log; needs Command review
- Bench #9 quiet-VM re-run: verify_inclusion_proof composed 1024-leaf; needs load avg < 1.0
- PhD thesis pre-publication: bench #9, Group 3A decisions, citation promotion, language pass, ES panorama
- WFD registry drift: `gateway-knowledge-documentation-1` absent from HEAD; reconcile against WFD main

**Operator preferences surfaced:**
- Auto Mode active: proceed without stopping for clarifications; make reasonable calls
- Approved all 4 v1.0.0 gate decisions using recommendations

---
