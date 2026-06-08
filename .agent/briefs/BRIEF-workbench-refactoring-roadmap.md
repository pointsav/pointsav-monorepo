---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workbench Institutional-Grade Refactoring Roadmap
status: archived
superseded_by: BRIEF-workplace-roadmap.md
created: 2026-05-31
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
purpose: Roadmap for refactoring the workbench from a monolithic SPA to a component-based, institutionally resilient architecture.
---

# BRIEF — Workbench Institutional-Grade Refactoring Roadmap

## 1. Mission
To evolve `app-privategit-workbench` from a monolithic, imperative SPA into an institutional-grade, component-based application capable of hosting PointSav's 8+ tool surfaces while maintaining the "Workbench" user paradigm.

## 2. Institutional-Grade Pillars (Resilience & Hardening)
1. **Resilience:** Per-app seL4 CNode isolation. Renderer crashes must not take down the Workbench Shell.
2. **Consistency:** Unified IPC and JSON-Manifest-driven dynamic toolbar injection.
3. **Observability:** Structured telemetry and audit logs integrated into the system-wide audit ledger (via `system-mba`).
4. **Distribution:** Atomic, self-verifying, build-host-attested artifacts anchored in a public Merkle log.

## 3. Implementation Roadmap
### Phase 1: Frontend Componentization
- [ ] Transition `index.html` monolith to a structured component framework (SolidJS/WebComponents).
- [ ] Implement a reactive state store to replace manual DOM manipulations.
- [ ] Decouple File Tree, Viewer, and Editor panes into independent, state-reactive components.

### Phase 2: Shell/Renderer Decoupling
- [ ] Implement `app-workplace-launcher` as the unified orchestrator.
- [ ] Define JSON-Schema-based Toolbar Manifests for each tool surface.
- [ ] Wire the IPC handshake (`OpenDocument`, `ToolbarManifestInjection`) between Shell and Renderer.

### Phase 3: Institutional Hardening
- [ ] Formalize `system-mba` UDS contract for renderer process authentication and audit logging.
- [ ] Implement structured error handling and localized logging surfaced to the `Workbench` Shell.
- [ ] Integrate per-app capability set visibility.

## 4. Drift Check Guidelines (Refactoring)
- **State management:** Does the new component architecture use the reactive store, or is there any residual imperative synchronization? (Goal: Zero imperative DOM manipulation).
- **Isolation:** Does the new component-shell still enforce the IPC contract, or is it leaking state between panes?
