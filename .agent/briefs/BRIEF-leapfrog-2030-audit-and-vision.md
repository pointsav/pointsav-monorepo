---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Leapfrog 2030 Audit and Vision (Source of Truth)
status: archived
superseded_by: BRIEF-workplace-architecture.md
created: 2026-05-31
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
purpose: Source of truth for checking architectural drift in project-workplace.
---

# BRIEF — Leapfrog 2030: Sovereign Native Workbench

## 1. Mission
To deliver a 2030-class native workbench for office and technical workflows that hyperscalers cannot compete with. This is achieved through:
- **Sovereignty by construction:** Proof-based security (seL4), air-gapped capability sets, and local-first data.
- **Leapfrog UI/UX:** A hybrid TUI/GUI paradigm (the "Desk" surface) that achieves clipboard/device parity with macOS/Windows while maintaining terminal-tier performance and auditable security.
- **AI as an Accelerant:** Secure, audited AI integration (Doorman) that remains local-first (Tier A local OLMo, Tier B/C user-initiated only) to ensure user data never leaks and AI remains a controllable tool.

## 2. Competitive Advantage (Why hyperscalers cannot compete)
Hyperscalers are structurally compelled to build telemetry, cloud-subscription dependencies, and browser-engine reliance into their products. PointSav is structured to invert this:
- **Zero-Trust Surface:** No Chromium, no Electron, no vendor-account requirements, no telemetry-channel-outside-policy.
- **Provable Auditability:** Every application’s capability set is visible, live-revokable, and auditable from a single keystroke (capability-mode launcher).
- **Architecture as Moat:** One OS process per app per CNode (seL4), with a formal-verification path, which no vendor-OS can retrofit.

## 3. Deployment Shapes (The Triple-Shape Topology)
The platform delivers three shapes from one monorepo, sharing the same substrate (Kitty, ratatui, BFS attribute store, audit ledger):
1. **`os-workplace` (Graphical):** For office workers (bookkeeping, property). Uses Tauri v2 + WebKitGTK.
2. **`os-tui` (Sovereign/TUI):** For technical operators, remote SSH access, low-bandwidth contexts. Uses Kitty + pointsav-tui-shell + ratatui.
3. **`os-developer` (Developer):** For PointSav engineers. Stripped WM, keyboard-first, with built-in auditability tools (`Super+M`, `Super+Shift+Space`).

## 4. Drift Check Guidelines (For AI Agents)
Any proposed change must be tested against these four non-negotiables:
1. **Network Surface:** Does this change increase the network surface? (Goal: Zero network syscalls by default; all egress must be audited via system-mba).
2. **Capability Visibility:** Is the app's capability set still visible via `pointsav-launch --capability-mode`?
3. **Canonical Format:** Is the new file format open, flat, and standard (e.g., .html, .json, .ifc)? Does it avoid proprietary binaries?
4. **AI-Safety:** Does this feature transit Doorman inference? If so, is it local-first (Tier A) or strictly user-initiated (Tier B/C)?

## 5. Visionary UI/UX Principles (2030 Trends & Integration)
- **Agentic UX:** Integration of local AI agents that orchestrate tasks *locally* without data leakage. The `os-workplace` Doorman architecture is the mechanism to achieve this.
- **Keyboard-First & Ambient Computing:** Moving beyond static dashboards to context-aware interfaces. The TUI shell multiplexer (Zellij-fork) and the F12 "quake" surface are foundations for this.
- **Hardware-Verified Sovereignty:** Designed for eventual RISC-V and TEE (Confidential Computing) integration, ensuring the "trust" isn't just software-based, but silicon-verified.
- **Frictionless Clipboard:** Seamless, multi-MIME clipboard parity (the `pointsav-clipboard-daemon`) is a critical component for users migrating from proprietary ecosystems (macOS/Windows).
- **The "Desk" Surface:** A cognitive staging area for in-progress work (BFS `DESK:staged` attribute), replacing the hierarchical file browser — a key differentiator for sovereign workflows.
