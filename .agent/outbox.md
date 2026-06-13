---
from: totebox@project-system
to: command@claude-code
re: Stage 6 pending — project-system — 2 unpromoted commits (2026-06-12)
created: 2026-06-12T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260612-stage6-brief-usedns
---

Two commits need Stage 6 promotion from the project-system Totebox archive:

**project-system archive (archive `.git/`):**
- `12b6eae4` — `ops(project-system): BRIEF housekeeping — archive phase1-deployment, thesis schema fix + §7.5.6 + relocation marker`
  - Files: `.agent/briefs/BRIEF-ostotebox-phase1-deployment.md`, `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md`

**pointsav-monorepo sub-clone (`pointsav-monorepo/.git/`):**
- `6bfa1c75` — `fix(os-totebox): add UseDNS no to sshd_config to resolve SSH banner timeout under TCG`
  - File: `os-totebox/scripts/build-image.sh`

Both are on `main` branch of their respective `.git/` instances. Prior Stage 6 pending commits
from this archive (Phase 1 boot milestone) were on monorepo commit `92692800` — confirm that
was promoted before queuing these.

---
from: totebox@project-system
to: command@claude-code
re: Stage 6 pending — project-system — Phase 1 boot milestone (2026-06-12)
created: 2026-06-12T00:00:00Z
priority: high
status: pending
msg-id: project-system-20260612-stage6-phase1-boot
---

Phase 1 os-totebox boot milestone commits require Stage 6 promotion:

**pointsav-monorepo sub-clone (`pointsav-monorepo/.git/`):**
- `92692800` — `feat(os-totebox): Phase 1 boot milestone — UEFI QCOW2 boots to multiuser on GCP TCG`
  - Files: `.cargo/config.toml`, `os-totebox/scripts/build-image.sh`, `os-totebox/scripts/rc.d/doorman`, `os-totebox/scripts/rc.d/system_ledger`
- `f5a12440` — likely the second Phase 1 commit (verify with git log)

Smoke test PASSED: NetBSD 10.1 multiuser, system-ledger-server PID 931, slm-doorman-server PID 1142,
sshd PID 937; `/healthz` 200 + `/readyz` 503 COLD confirmed 2026-06-12.
