---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/tui-corpus-producer.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 45
research_done_count: 5
research_suggested_count: 2
open_questions_count: 1
research_provenance: cluster-research-iter-24-tui-design + master-rlhf-dpo-literature-validation
research_inline: true
---

# TUI-as-Corpus-Producer

Every terminal interaction with service-slm through the System Administrator
TUI (`slm-cli`) is a curated training corpus contribution.

This convention codifies Doctrine claim #45 (ratified v0.1.0). It is the
operational form for the slm-cli implementation phase (per leapfrog roadmap
Phase 4).

## §1 — Why TUI interactions are high-quality training data

Three properties distinguish sysadmin and IT-support interactions from general
training data:

- **Verifiable ground truth.** When an operator follows AI advice (for example,
  "run `systemctl restart local-fs.service`"), the system either recovers or it
  does not. Other domains — creative writing, strategic reasoning — lack this
  immediate-feedback property. The IT-support domain has it by default.
- **Narrow domain.** Totebox Archive operations, Foundry conventions, and
  customer archive specifics form a bounded vocabulary, command set, and failure
  mode space. Models train more efficiently on bounded domains.
- **Domain-expert feedback.** The operator providing `/feedback [good|bad]` is
  the person who knows whether the response was correct — not a proxy labeler.

The published RLHF and DPO literature consistently reports that high-quality
interaction tuples train an order of magnitude more efficiently than observation
tuples. The TUI is engineered to produce the highest interaction-tuple density
Foundry can capture.

## §2 — The /feedback mechanism

After every assistant response in the TUI, a status bar shows:

```
[ESC] dismiss   [G] good   [R] refine   [B] bad
```

Three explicit verdicts:

- **G (good)**: the response was correct and useful. The tuple is flagged as a
  positive DPO example.
- **R (refine)**: the response was close but needs adjustment. The operator
  provides a correction inline; the tuple captures the (response, refinement)
  pair as training signal.
- **B (bad)**: the response was wrong. The tuple is flagged as a negative DPO
  example.

Implicit signal: if the operator dismisses without a verdict, the tuple is
captured as "no-verdict" and contributes to SFT but not DPO.

## §3 — Adapter quality budget

The published OLMo 2 fine-tuning literature suggests 200–500 high-quality
verdict-signed interactions are sufficient for first-cycle adapter production
in a narrow domain. Foundry's intended targets:

- **Weeks 1–4 (post-TUI launch)**: 50 interactions/week from operator dogfood;
  200 cumulative
- **Week 4, first adapter cycle**: train `it-support-pointsav-v0.0.1` LoRA on
  200+ verdict-signed tuples
- **Quality gate**: validation acceptance-rate ≥ 0.6 → promote to workspace
  deployment
- **Customer launch**: when the first customer Totebox ships, their TUI starts
  producing `it-support-<tenant>-v0.0.1` corpus immediately

## §4 — Per-tenant adapter ownership (composition with claim #48)

The TUI corpus produced by a customer's operators trains the customer's tenant
adapter, not Foundry's general adapter. Per claim #48 (Customer-Owned Graph
IP), the adapter weights are the customer's property. Foundry distributes the
model architecture and the training pipeline; the customer keeps the trained
adapter.

## §5 — slm-cli implementation requirements

Per the leapfrog roadmap Phase 4 implementation:

- Rust + ratatui v0.30+
- Doorman client via `reqwest` (the TUI never calls Tier A, B, or C directly;
  per claim #43)
- SSE streaming response rendering with auto-follow
- Slash commands: `/status`, `/audit`, `/graph`, `/feedback`, `/help`,
  `/tier`, `/adapters`
- Verdict capture writes to `data/training-corpus/it-support/<tenant>/` via
  the Doorman `POST /v1/verdict` endpoint
- F-key bindings for help, stats, clear, and quit, per the operator-friendly
  pattern of htop, glances, and lazygit

## §6 — When TUI verdicts are NOT corpus contributions

- **Test sessions** (TUI started with `--no-corpus` flag): verdicts are
  audit-logged but not written to the training corpus.
- **Error-before-completion** (Tier unavailable, network timeout): captured
  for operational diagnostics, not training.
- **`/tier` debug mode** (operator forcing a specific tier): captured but
  flagged as tier-forced and excluded from normal-distribution training data.

## Provenance

Research reviewed: ratatui v0.30 modular workspace (https://ratatui.rs/);
tuichat reference implementation (approximately 1,800 lines Rust); Apprenticeship
Substrate §7C (Brief Queue) operational since v0.1.85; sysadmin AI products that
implement a passive-beneficiary model; DPO literature supporting "quality over
volume" for narrow-domain adapters.

Suggested next research: (1) `/feedback` prompt design — minimize friction;
the operator should verdict in one keypress for the common case; (2) per-cluster
adapter cross-pollination policy — when multiple clusters' TUI verdicts compose
into a workspace-tier adapter.

**OQ #1 — Verdict signing identity.** When an operator at a customer Totebox
issues `/feedback good`, who signs the verdict? Options: the operator's
per-tenant SSH key (claim #48 alignment); a Totebox-resident key issued at
provisioning; an OAuth token from Foundry. Pending operational form during
Phase 4 implementation.

## References

- `DOCTRINE.md` claim #45
- Companion: `conventions/apprenticeship-substrate.md` §7D extension
- Companion: `conventions/customer-owned-graph-ip.md` (claim #48)
