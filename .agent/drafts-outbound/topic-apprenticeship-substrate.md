---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-slm
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-apprenticeship-substrate.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-28
authored_by: task-project-slm (session 3620a18e52bc5329)
authored_with: opus-4-7
references:
  - DOCTRINE.md §III claim #32 (Apprenticeship Substrate)
  - conventions/apprenticeship-substrate.md
  - service-slm/ARCHITECTURE.md §11
  - service-slm/crates/slm-doorman/src/apprenticeship.rs
  - service-slm/crates/slm-doorman/src/verdict.rs
  - service-slm/crates/slm-doorman/src/promotion_ledger.rs
notes_for_editor: |
  Skeleton only — substance lands as the apprenticeship corpus
  accumulates verdicts from real production routing (PS.5 plan).
  This cluster ORIGINATED the pattern (cited as workspace-wide
  precedent by Master at v0.1.30 codification of sub-agent-as-
  tier-discipline). The TOPIC describes WHAT the substrate is and
  WHY it works, not HOW to operate it (operational procedure
  belongs in a future GUIDE).

  Cross-reference with topic-doorman-protocol.md: the Doorman is
  the routing surface; the Apprenticeship Substrate is the
  production-routing application. Coordinate at refinement.

  BCSC posture: the substrate is current-fact for the substrate
  itself (live in code, three endpoints implemented, mock-tested);
  forward-looking for production-routing claims (PS.5 graduate-
  task-type-on-corpus is planned, not shipped).
---

# TOPIC — Apprenticeship Substrate

(draft-pending — substance follows as PS.5 graduates the first task type)

## Polarity flip

(draft-pending — substance follows in milestone N+1)

The default Claude / human collaboration: human asks, Claude
attempts, human reviews. The Apprenticeship Substrate flips it:
service-slm (the Doorman's small local model) attempts first;
Master/Root/Task Claude reviews. Disagreement between attempt
and verdict, captured as signed append-only training tuples, is
the highest-quality continued-pretraining signal Foundry produces.

## Three endpoints

(draft-pending — substance follows in milestone N+1)

`POST /v1/brief` — apprentice attempts a task. Response includes
self-confidence; below threshold or escalate=true returns empty
diff for senior to handle. `POST /v1/verdict` — senior signs and
submits the outcome (Accept / Refine / Reject / DeferTierC).
`POST /v1/shadow` — synthesize a brief on every code-shaped commit;
apprentice attempts in shadow mode; the (synthesized brief, actual
commit diff) tuple captures Stage-2 craft preference.

## Promotion ledger

(draft-pending — substance follows in milestone N+1)

Stages: review → spot-check → autonomous. Per task-type. Thresholds:
review→spot-check at n≥50 verdicts and accept-rate ≥0.85;
spot-check→autonomous at n≥100 and ≥0.95. Append-only ledger under
flock(2) for cross-process safety.

## Stage-1 vs Stage-2 corpus

(draft-pending — substance follows in milestone N+1)

Stage-1: the verdict itself (Accept / Refine / Reject / DeferTierC).
Stage-2: the (apprentice attempt, senior-corrected version) DPO
pair from Refine outcomes; the (synthesized brief, real commit
diff) DPO pair from shadow tuples; the (refined draft, creative-
edited published version) DPO pair from the Reverse-Funnel
Editorial Pattern (claim #35).

## SSH-keygen verdict signing

(draft-pending — substance follows in milestone N+1)

Senior signs verdicts via `ssh-keygen -Y sign -n
apprenticeship-verdict-v1`. Doorman verifies via `ssh-keygen -Y
verify` against `~/Foundry/identity/allowed_signers`. Same SSH
keypair the senior already uses for Git commit signing — no
new key infrastructure.

## Healing-effect economics

(draft-pending — substance follows in milestone N+1)

Once service-slm contributes alongside Claude, errors heal via
verdict signing → corpus → continued LoRA training loop. Sonnet
output today is acceptable because the loop heals it tomorrow
(framing per workspace v0.1.42 SLM Operationalization Plan).

## References

(draft-pending — citation IDs resolve at project-language refinement)
