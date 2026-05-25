---
schema: foundry-draft-v1
state: refined-pending-master-commit
originating_cluster: master-workspace
target_repo: ~/Foundry
target_path: conventions/single-boundary-compute-discipline.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
authored_with: claude-opus-4-7-1m
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 43
research_done_count: 8
research_suggested_count: 2
open_questions_count: 0
research_provenance: cluster-research-iter-24-citing-litellm-portkey-helicone-snyk + master-validation
research_inline: true
references:
  - DOCTRINE.md claim #43
  - conventions/three-ring-architecture.md
  - conventions/api-key-boundary-discipline.md
  - infrastructure/local-doorman/
  - vendor/pointsav-monorepo/service-slm/docs/yoyo-training-substrate-and-service-content-integration.md §6
---

# Single-Boundary Compute Discipline

The Doorman (`service-slm`) is the single boundary point for all AI inference
compute in every Foundry deployment. No process, session, or service accesses
an inference tier — local, Yo-Yo, or external API — except through the Doorman.

This convention codifies Doctrine claim #43 (ratified v0.1.0). Read at session
start by every cluster Task and at every operator review of Foundry deployments.

## §1 — The discipline

All AI inference traffic — from a Master session, a Root session, a cluster
Task, the customer-facing TUI (`slm-cli`), customer extensions, or any future
automation — routes through the Doorman's HTTP boundary at
`http://127.0.0.1:9080` (workspace VM) or the equivalent localhost binding on
customer Toteboxes.

The four enforcement mechanisms are structural, not policy:

1. **Bearer-only-in-Doorman.** The Yo-Yo bearer token, all Tier C provider API
   keys (Anthropic, Gemini, OpenAI), and all Tier A endpoint URLs live
   exclusively in the Doorman's environment file
   (`/etc/local-doorman/local-doorman.env` on workspace VM; equivalent on
   customer Toteboxes). No cluster Task session has read access. No other
   service holds these secrets. The secret surface is one process.

2. **Firewall.** The Yo-Yo VM accepts inbound connections only from the
   workspace VM's internal IP. Customer Tier B GPU boxes (when present) accept
   inbound only from the customer's Doorman-host IP. External API provider rate
   limits and audit are visible only to the Doorman.

3. **UID-owner iptables on Tier A.** The local llama-server bound to
   `127.0.0.1:8080` accepts connections only from the Doorman process UID
   (`local-doorman` system user). Implemented via iptables `--uid-owner` on the
   OUTPUT chain. Any other process on the same VM attempting to reach Tier A
   directly is dropped at the kernel boundary.

4. **Doorman startup verification.** On boot the Doorman verifies that
   `SLM_YOYO_BEARER` is present and non-empty (when `SLM_YOYO_ENDPOINT` is
   configured) and refuses to start with an unset bearer. This prevents
   misconfiguration-driven bypass: a Doorman that boots without the bearer
   cannot accidentally allow Tier B traffic to flow without audit.

## §2 — Bypass attempts are audit violations

A cluster session that calls `http://127.0.0.1:8080/v1/chat/completions`
(Tier A llama-server) directly, bypassing the Doorman, commits an audit
violation. Three reasons:

- **No audit ledger entry.** The audit ledger
  (`data/audit-ledger/<tenant>/<YYYY-MM>.jsonl`) is the legally admissible
  record under BCSC continuous-disclosure posture [ni-51-102]. Direct calls
  produce no entry — the ledger has gaps.
- **No corpus contribution.** The Apprenticeship Substrate (claim #32) captures
  every Doorman-mediated call as a training tuple. Direct calls produce no
  shadow brief — the corpus has gaps.
- **No cost control.** The budget cap (`monthly_cap_usd` in
  `infrastructure/slm-yoyo/tofu/`) and the kill-switch operate at Doorman
  level. Direct Tier B or Tier C calls bypass the cap.

When a session needs a capability the Doorman does not yet support, the
structural remedy is to add that capability to the Doorman (via project-slm
cluster work), not to route around it.

## §3 — What this is, what it is not

This is **exclusive-path with structural enforcement**.

This is **not** preferred-path routing (LiteLLM, Portkey, Helicone) — those
products are gateways customers may choose to use; bypass is permitted by
configuration. Foundry's claim is stronger: bypass is structurally prevented at
the kernel boundary (firewall and UID-owner iptables) and the secret boundary
(bearer-only-in-Doorman).

This is **not** a network tap or proxy in the traditional sense. The Doorman is
the only process that holds inference credentials; without credentials, Tier B
and Tier C calls from any other process fail at the authentication layer. Tier A
is locally bound and UID-restricted, so bypass attempts fail at the kernel layer.

## §4 — How callers interact with the Doorman

The Doorman exposes a standard OpenAI-compatible HTTP interface:

```
POST /v1/chat/completions
Headers (optional):
  X-Foundry-Module-ID: <tenant-or-module-scope>
  X-Foundry-Tier-Hint: local|yoyo|external|auto
  X-Foundry-Complexity: low|medium|high
  X-Foundry-Tier-C-Label: <allowlist-purpose>
Body: { "model": "auto", "messages": [...], "max_tokens": N }
```

Response shape:
```
{
  "request_id": "<ULID>",
  "tier_used": "local|yoyo|external",
  "model": "auto",
  "content": "...",
  "inference_ms": N,
  "cost_usd": 0.0
}
```

This is also the MCP gateway interface (per claim #46, MCP-as-Substrate-
Protocol). Foundry MCP clients (TUI, web UI, customer agents, IDE integrations)
use the same boundary.

## §5 — Operational form on workspace VM and customer Totebox

| Component | Workspace VM | Customer Totebox |
|---|---|---|
| Doorman binary | `/usr/local/bin/slm-doorman-server` | same |
| systemd unit | `local-doorman.service` | same |
| env file | `/etc/local-doorman/local-doorman.env` | same |
| HTTP bind | `127.0.0.1:9080` | same |
| Tier A binding | `127.0.0.1:8080` (UID-owner restricted) | same |
| Tier B binding | external IP (firewalled to workspace VM) | external IP (firewalled to customer Doorman) or off |
| Tier C | Doorman-only env vars | optional per-tenant; off by default |

The shape is identical from workspace dogfood to first customer deployment. Per
`conventions/customer-first-ordering.md`, this matters: PointSav operates the
same boundary the customer operates.

## §6 — Why this convention is doctrine-level

Compute boundary discipline composes with five other claims:

- Claim #16 (Three-Ring Architecture): Ring 3 (AI) is structurally optional;
  the Doorman is the entry to Ring 3 from Rings 1–2. Multiple Doorman entries
  break the three-ring model.
- Claim #32 (Apprenticeship Substrate): training tuples are produced at the
  Doorman boundary. Multiple boundaries produce divergent tuple shapes, which
  makes training data unusable.
- Claim #34 (Two-Bottoms Sovereign Substrate): customer sovereignty is enforced
  at the Doorman boundary. Bypass = sovereignty leak.
- Claim #44 (Knowledge-Graph-Grounded Apprenticeship): graph context is
  assembled at the Doorman before tier dispatch. Bypass = ungrounded inference.
- Claim #46 (MCP-as-Substrate-Protocol): the Doorman is the MCP gateway.
  Bypass = MCP graph break.

Encoding this as doctrine prevents future conventions or implementation
milestones from quietly reintroducing a second compute boundary.

## §7 — Bypass-prevention checklist

When implementing or reviewing any new feature that touches inference, the
structural question is: *does this change create a second path to inference
compute outside the Doorman?* Apply the checklist:

- [ ] Are bearer tokens and API keys for inference tiers loaded only into the
      Doorman's environment? *(yes / no)*
- [ ] Does the new component talk to inference tiers only via the Doorman's
      `/v1/chat/completions` endpoint? *(yes / no)*
- [ ] Is the Tier A llama-server still UID-owner-restricted? *(yes / no)*
- [ ] Does the Yo-Yo firewall rule still restrict inbound to the Doorman's
      host IP? *(yes / no)*
- [ ] Is the audit ledger still receiving entries for every inference call from
      this component? *(yes / no)*

A "no" answer requires either (a) the change is rejected, or (b) a doctrine
amendment relaxes the discipline (which has not occurred through v0.1.0).

## §8 — Composition with `conventions/api-key-boundary-discipline.md`

The api-key-boundary-discipline convention (ratified v0.0.14) is the narrower
form: API keys for external providers (Tier C) live exclusively in Doorman
config. Single-Boundary Compute Discipline (v0.1.0) generalizes that rule to
all inference tiers — including local Tier A and customer-deployment Tier B.
The api-key-boundary convention remains in force as the Tier C-specific
operational form.

## §9 — When this convention will be amended

Doctrine MAJOR or MINOR bump only. PATCH-level changes may document operational
mechanics (firewall rule syntax updates, env var renames) but do not change the
underlying single-boundary principle. Any change that weakens structural
enforcement is a doctrine MAJOR signal — it means Foundry is changing its
substrate posture.

## Provenance

Research reviewed: LiteLLM proxy server pattern (preferred-path; bypass
permitted); Portkey and Helicone gateway patterns; AWS API Gateway and Cognito
OAuth (preferred-path with policy); Snyk Cloud API security review (gateway as
audit boundary); ServiceNow CMDB single-source-of-truth pattern; Splunk
Universal Forwarder log aggregation; Kubernetes Istio/Linkerd mTLS sidecar
pattern; Foundry workspace v0.1.85 firewall rule for Yo-Yo.

Suggested next research: (1) empirical measurement of audit ledger completeness
(percentage of inference calls captured) over a one-month window
post-claim-#43-ratification; (2) per-customer-Totebox firewall configuration
template for first customer deployment.

## References

- `DOCTRINE.md` claim #43 (text reproduced in §1 above)
- Companion: `conventions/api-key-boundary-discipline.md`
- Companion: `conventions/three-ring-architecture.md`
- Operational: `infrastructure/local-doorman/local-doorman.service`
- Implementation reference: `vendor/pointsav-monorepo/service-slm/`
