# TASK SUMMARY: Radical UI Redesign & Rust Distillation

## Objective
Transform the `app-console-proofreader` UI into an award-winning web experience, build a pure-Rust distillation tool, and set up cross-cluster routing for design tokens and documentation.

## Task Checklist
- [x] Draft `topic-radical-proofreader-ui.md` and `guide-proofreader-distillation.md` for `/content-wiki-documentation` via `.agent/drafts-outbound/`.
- [x] Notify TASK agent via `.agent/inbox.md`.
- [ ] **Phase 1: Radical UI/UX Rewrite**
  - [ ] Redesign HTML/CSS payload in `app-console-proofreader/src/ui.rs`.
  - [ ] Identify extracted generic tokens (`/pointsav-design-system`) and tenant branding (`/woodfine-media-assets`).
- [ ] **Phase 2: Rust-Based Training Distillation**
  - [ ] Implement `tool-proofreader-trainer` in Rust to parse `apprenticeship/prose-edit` JSONL.
  - [ ] Generate `training_dataset.jsonl` formatted for `service-slm`.
- [ ] **Phase 3: Cross-Cluster Token Handoff**
  - [ ] Document generic and tenant tokens in `.agent/outbox.md` for `project-design`.
- [ ] **Phase 4: Verification**
  - [ ] Ensure local UI rendering aligns with redesign goals.
  - [ ] Ensure the distillation binary compiles (`cargo check`) and produces correct output shapes.
