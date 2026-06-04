---
artifact: brief
status: archived
archived_date: 2026-06-04
contamination_note: >-
  Correct home archive is project-console. Landed in project-intelligence via
  Stage-6 rebase contamination 2026-05-22. Previously absorbed by project-console
  BRIEF. Moved to archive; content intact for project-console to verify.
absorbed_by: BRIEF-project-console-master.md
absorbed: 2026-05-31
author: gemini (external audit)
notes: >
  3 items extracted: --plain mode, keyboard ergonomics contract, semantic color states.
  Remainder was stale (Stage 6, Phase 1 already complete). Port note (8011) was wrong;
  inherited from pre-fix BRIEF-os-console-platform.md §10.
---

# COMPREHENSIVE IMPROVEMENT PROPOSAL: PROJECT-CONSOLE (2026-05-31)

## Overview
This document consolidates findings from a holistic audit of `project-console` codebase and documentation, now grounded in external architectural research.

## Strategic Context & Industry Alignment
- **Chassis-First Platform (Alignment: Modular Robotics/NOS):** Our adoption of a chassis-first architecture (`os-console`) mirrors proven strategies in robotics and networking. By enforcing modularity at the chassis level, we prioritize fault isolation, standardized service inheritance for "cartridges" (F1–F12), and adaptive interoperability.
- **Knowledge-Graph-Grounded Editorial Platform (Alignment: Semantic RAG):** Our approach constitutes an advanced implementation of Semantic Retrieval-Augmented Generation (RAG). By grounding LLM authoring in a structured knowledge graph, we ensure semantic consistency across the multi-wiki ecosystem, exceeding standard generative AI capabilities in reliability.

## Current State Assessment
- **Architecture:** Transitioning to a unified `os-console` platform chassis with compiled-in cartridges.
- **Foundations:** PPN isolation, substrate protocols, and a centralized knowledge-graph-grounded editorial platform.
- **Key Challenges:** Stage 6 codebase rebase, modular cartridge integration, and aligning legacy binary development with the new library-based model.

## Proposed Improvements

### 1. Platform & Chassis
- **Unified Build Pipeline:** Transition from separate Cargo projects (like `os-console` and `app-console-content`) to a tightly integrated workspace build to simplify dependency management and cross-cartridge testing.
- **Substrate Refinement:** Standardize inter-cartridge communication protocols (`app-console-keys` and `app-console-mesh`) to reduce boilerplate code in individual cartridges.

### 2. Products & Services
- **Cartridge Lifecycle:** Establish a formal API for cartridge lifecycle management (load, unload, resource isolation) within the `os-console` chassis to improve reliability and performance.
- **Editorial Knowledge Integration:** Enhance `app-console-content` (F4) by directly embedding the editorial knowledge-graph client as a native component, enabling real-time, context-aware proofreading and drafting.

### 3. Codebase & Developer Experience
- **Automated Audit:** Implement automated linting and compliance checkers within the build pipeline to ensure adherence to substrate security protocols early in the development lifecycle.
- **Documentation Parity:** Ensure that all new features in `app-console-*` cartridges include automatically generated documentation updates in the `docs/` folder, preventing drift between implementation and intent.

### 4. UI/UX Strategy (Alignment: Modern TUI/CLI Design)
- **Spatial Consistency:** Adopt a responsive grid layout for the `os-console` chassis (the "IDE three-panel" pattern: Navigation, Main Content, Status) to ensure consistent user experience across varied terminal dimensions.
- **Keyboard-First Ergonomics:** Standardize keyboard interactions across all F-key cartridges (e.g., consistent `Esc` to cancel, `Enter` to select, `Tab` for focus navigation) and implement persistent command legends in the footer.
- **Visual & Semantic Clarity:** Utilize semantic color palettes to convey state (error, success, warning) independent of color-only signaling, and integrate iconography (Nerd Fonts) for faster visual parsing.
- **Accessibility:** Implement a `--plain` mode for all cartridges to ensure compatibility with screen readers and degraded SSH connections, ensuring no functionality is restricted by aesthetic choices.

## Implementation Roadmap
1. Complete Stage 6 rebase.
2. Refactor `app-console-content` from binary to cartridge library.
3. Integrate standardized inter-cartridge communication substrate.
4. Implement automated build/audit pipelines.

## Goal
This brief serves as a definitive North Star for the development of `project-console`, ensuring future AI/human contributions are aligned with the established strategic vision, substantiated by industry-leading architectural patterns.
