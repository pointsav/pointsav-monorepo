---
name: project-vm-hardening-state
description: Current state of workspace VM hardening — 7B model to be removed, Yo-Yo is the AI path, build session todo sent to Command
metadata:
  type: project
---

Workspace VM (`foundry-workspace`, e2-standard-8, us-west1-a) hardening state as of 2026-05-23:

- **7B model (local-slm.service)**: still running as of session end; should be stopped tonight by Command build session. `SLM_FORCE_BROKER_MODE=true` is the immediate fix (no build needed).
- **service-content (local-content.service)**: was stopped since 2026-05-21; restart attempted this session; may still be loading (LadybugDB init takes 15–25 min).
- **Yo-Yo VM**: offline (1495+ consecutive health failures); Packer image rebuild needed to activate Phase 0 hardening (G3/G17).
- **Correct AI path**: Yo-Yo (Tier B) or Tier C (Claude API) — no local model on workspace VM per BRIEF-flow-restructure §8.F (NUC on-device AI gated behind named customer).

**Why:** e2-standard-8 is Hardware-class; 7B model is wrong for both Tier A (=1B specialist) and Tier B (=GPU-speed). Swap pressure and inference hangs traced to 7B model + 15+ idle Claude shells.

**How to apply:** At session start, check if local-slm.service is still running. If yes, the Command build session hasn't run yet — remind operator.

Full todo: BRIEF-vm-hardening-and-consolidation.md §1–§4. Outbox message `project-intelligence-20260523-build-session-todo` sent to Command.
