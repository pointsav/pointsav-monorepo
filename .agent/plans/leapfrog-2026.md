# Leapfrog 2026 — Strategic Deep-Think

> Authored: 2026-05-14 task@project-intelligence (Opus 4.7 1M deep-think)
> Status: Strategic roadmap — pending operator ratification
> Companions:
>   - `.agent/plans/universal-ai-gateway.md` (sprint plan)
>   - `.agent/plans/tier-architecture-2026.md` (tier policy)
>   - `~/.claude/plans/sovereign-coding-agent-leapfrog-2030.md` (product strategy)
>   - `~/.claude/plans/wire-format-leapfrog-2030.md` (wire format strategy)
>   - `service-slm/ARCHITECTURE.md` §13 (2030 headroom)

---

## 0. Executive verdict

**The leapfrog thesis is correct, but with one significant amendment.**

The thesis as written is: *capability matters less than data substrate; by 2028–2030 small models will catch up; the moat is the organisational data + the fine-tune trained on it.*

This is right about the model-capability trajectory and right about the data flywheel, but it is **wrong about which artifact is the moat**. The moat is not the fine-tuned model. The moat is the **WORM-anchored audit substrate plus the regulated jurisdiction posture plus the wire-format-neutral gateway**. The model is the marketing surface; the substrate is the durable advantage. Hyperscalers can match the model. They cannot match a Canadian PBC-style sovereign-data appliance under BC Securities Commission disclosure discipline that they don't operate.

This reframes priorities: ship the gateway, lock the BCSC posture, close the apprenticeship loop. The model improvements compound *into* a substrate that hyperscalers cannot replicate even when their model is better.

**Three things to internalise before reading on:**

1. **Sovereignty is not a model property — it is a jurisdictional property.** No hyperscaler can claim NI 51-102 alignment from a BC-headquartered operation. That is the durable wedge.
2. **The gateway is a load-bearing product, not infrastructure.** Wire-format-neutral routing through a Canadian-jurisdiction Doorman is the licensable surface. The own-model is a *feature* of that surface.
3. **The compounding loop is not optional.** If the apprenticeship corpus does not close by Q3 2026, the leapfrog thesis collapses into "we built a competent gateway." That's a real but commodity outcome.

---

## 1. Is the leapfrog thesis correct?

### Where it is correct

**Model capability is catching up faster than data accumulates.** OLMo 2 7B-Instruct in 2026 is roughly at GPT-3.5-mid-2023 quality for instruction-following. By 2028, OLMo 4/5 7B will likely match Claude Sonnet 3.5 (mid-2024) for structured tasks. That's a credible 18–24 month lag, narrowing. Meta, Mistral, and AI2 are all on a release cadence that delivers ~one major generation per year. Compute efficiency (FP8, MoE, sparse attention, test-time compute) is improving the inference economics 2–3× per generation.

**The data flywheel cannot be back-filled by hyperscalers.** Anthropic does not have Woodfine's internal correspondence, vendor invoices, ledger entries, BCSC filing drafts, or the human-accepted code diffs from this specific codebase. They never will. This is the structural advantage of customer-owned data: it is non-fungible and cannot be acquired post-hoc.

**The substrate is correctly designed.** Three-ring architecture, WORM ledgers, Doorman boundary, OLMo-only-for-base, signed adapters — every piece is correctly oriented for the long game. The architecture is not the problem.

### Where it is wrong (or at risk)

**Risk 1: The "small models will be capable enough" claim has a tail.** Capability is not uniform — small models lag specifically on *long-horizon multi-step tool use* and *novel reasoning under uncertainty*. These are exactly the tasks Claude Code is best at. OLMo 3 32B Think will match Sonnet on focused single-step tasks long before it matches Sonnet on 5+-step agentic chains. The leapfrog could arrive for read-heavy tasks (already true) and never arrive for code-edit-in-a-mature-codebase (the actual differentiating use case). Mitigation: route ruthlessly. Tier C is not a fallback, it is a *correct tier* for opus-tier tasks. Plan for permanent Tier C use, not Tier C deprecation.

**Risk 2: "Open-data only" is a slow lane.** OLMo's release cadence is good but it is not the frontier. By the time AI2 ships OLMo 4 with full-Dolma transparency, Meta/Mistral/Anthropic will have shipped 2–3 generations on opaque data. The capability gap between "open-data permissible" and "frontier" is not closing — it is widening. Mitigation: the BCSC posture *requires* this slow lane for the own-model base. That's not a bug; it's the regulatory cost. Accept it and use Tier C aggressively for the gap.

**Risk 3: The flywheel only flies if data quality is high.** Empty `actual_diff` fields produce garbage tuples. SFT on noisy briefs produces a model that confidently does the wrong thing. The "453 tuples above threshold" figure is *count*, not *quality*. Without an eval harness, you cannot tell whether the corpus is improving or polluting the model. Mitigation: §3 below.

**Risk 4: The compounding rate is slower than founder optimism suggests.** Plausible math: 5 Claude Code sessions per day, ~30% become useful training tuples (others are exploration, throwaway, multi-turn that doesn't close), so ~1.5 tuples/day. First viable LoRA needs ~1000 high-quality DPO pairs in the engineering domain. That's ~22 months at current rate without intervention. The flywheel needs *acceleration mechanics* (multi-developer team, broader task surface, or synthetic pair augmentation) or the compounding moat is real but slow. Mitigation: app-console-slm scaling to multi-developer teams is not optional — it's how the flywheel reaches velocity.

**Risk 5: Hyperscalers will offer "sovereign" deployments.** AWS Bedrock, Azure OpenAI Service, and Google Cloud Vertex AI already offer "in your VPC" deployments with data-residency commitments. By 2028, "sovereign Claude" running in a Canadian region with logging guarantees is likely a real product. Mitigation: their sovereignty is contractual ("we promise we won't read your data"); yours is *structural* (the data never leaves the WORM ledger; the model runs on hardware you control; the audit substrate is cryptographically verifiable). This is a real differentiation but it is *narrower* than the marketing implies. The customer must value structural sovereignty over contractual sovereignty — that's a B2B sales motion, not a product fact.

### Preconditions for the thesis to work

1. **Apprenticeship loop closes within 6 months.** Git post-commit hook + corpus quality gate + eval harness. If any one is missing 12 months from now, the flywheel is not flying.
2. **At least 3 developers using app-console-slm by 2027.** The single-operator data rate is insufficient. Need a small team or a paying customer's team feeding the corpus.
3. **BCSC posture survives first audit.** The "Canadian sovereign Foundation" claim has to be defensible under continuous disclosure scrutiny. The `permissible-model-substrate.md` convention is the artefact that makes this defensible. Ratify it.
4. **Tier C remains affordable.** If Anthropic pricing doubles or terms change adversely, the routing economics shift. Have a contingency: Google Vertex Claude or AWS Bedrock Claude as a fallback Tier C provider, both routed through the Doorman shim.
5. **No structural pivot away from local-first.** If the broader market consolidates on cloud-only inference (e.g., Apple silicon SLMs flop, on-device inference stagnates), the sovereign-appliance product loses its strategic context. Watch this — it's an environmental dependency.

---

## 2. The capability gap problem

OLMo 3 32B Think on L4 is the Tier B workhorse. Honest assessment per task:

### Rust code edits in `service-slm` (a mature codebase)

**Honest verdict: capability gap is real and won't fully close by 2028.**

- Claude Sonnet 4.6 / Opus 4.7 quality on this task: applies idiomatic edits, respects existing patterns, catches `Result` propagation, understands async lifetimes in `tokio`, navigates the `Arc<RwLock>` discipline correctly.
- OLMo 3 32B Think on the same task: produces syntactically correct Rust ~80% of the time on isolated functions; struggles with multi-file refactors; over-eagerly adds `unwrap()`; doesn't reliably maintain `tracing` instrumentation patterns; misses subtle ownership errors that compile but mean the wrong thing.
- 2027 OLMo 4 7B-13B Think prediction: probably matches mid-2024 Claude (Sonnet 3.5) — adequate for single-file edits, weak on cross-module refactors.
- 2028–2029 prediction: open-data 13B models may match Sonnet 4.5 for *single-file* edits. Multi-step refactors and architectural changes will still want Tier C.

**Correct routing today:** Single-file edits to leaf modules → Tier B. Anything touching the router, trait boundaries, or `slm-core` types → Tier C. Hard-code this in app-console-slm's complexity classifier; don't trust the model to self-route.

### Entity extraction from business correspondence

**Honest verdict: capability gap is small and closing fast. This is where the leapfrog already happened.**

- Sonnet quality: excellent.
- OLMo 3 32B Think with grammar constraints: 95%+ valid JSON, comparable extraction precision on Woodfine corpus shapes. Proven (74 entities).
- The H100/Llama 3.3 70B graph Yo-Yo handles the strict-ontology tail.
- **The leapfrog has arrived for this task class.** Lean into it — this is a real customer-visible win.

### BCSC-compliant financial disclosure drafting

**Honest verdict: capability gap is small but the *risk* of getting it wrong is large.**

- Sonnet drafts plausible disclosure language but hallucinates statutory references ~5–10% of the time. *Any* hallucination is unacceptable for filings.
- OLMo 3 32B Think will be worse on this — open-data training has thinner exposure to securities-law prose.
- **Correct disposition: this task should not be model-fronted at all.** Use the model for *retrieval and structuring* against a citation-grounded corpus (the citations.yaml substrate + relevant precedents). The actual prose is human-written. The model is a research assistant, not a drafter.
- This is also a SYS-ADR-19 boundary: no automated AI publishing to verified ledgers. Filings are verified ledgers in the regulatory sense. Don't cross it.

### Knowledge graph query formulation

**Honest verdict: capability gap is small. Tier B works.**

- Mapping NL question → LadybugDB query syntax is a constrained transformation. Grammar-guided generation on Tier B handles it.
- This is the right place to put Tier B work that has clean evaluation: query roundtrip is testable (does it return results, do the results match a gold set).
- Build the eval harness *here* first — easiest task class, fastest signal, generalises.

### Routing decision summary

| Task class | Tier today | Tier 2028 (predicted) | Defensible position |
|---|---|---|---|
| File summarisation | A | A | Already won |
| Grep/search interpretation | A | A | Already won |
| Entity extraction | B (graph YYO) | B (own LoRA) | Already won — lean in |
| Knowledge graph query | B | A | Easy win — own this fast |
| Single-file Rust edit | C → B over time | B | Hot zone of competition |
| Multi-file refactor | C | C | Permanent Tier C |
| BCSC disclosure drafting | Human + B for retrieval | Same | Don't ever model-front |
| Architecture decision | C | C | Permanent Tier C |
| Long agent chain (5+) | C | C | Permanent Tier C |

The strategic point: Tier C is not a stepping stone. It is part of the product permanently. The product is *correct routing*, not *avoiding Tier C*.

---

## 3. The LoRA compound loop — does it actually compound?

### Trace the loop honestly

```
Claude Code session
  → user message (brief)
  → tool-use chain (Read, Edit, Bash, ...)
  → file diffs
  → user reviews + commits OR discards
  → if commit: git post-commit hook (NOT YET WIRED)
  → POST /v1/shadow {brief, actual_diff}
  → corpus grows
  → nightly LoRA training (NOT YET RUNNING)
  → adapter → GCS (NOT YET SIGNED)
  → adapter deployed to Tier A or B (NOT YET A REAL PATH)
  → next session: more tasks route locally
```

Of the five steps after `/v1/shadow`, **only the corpus growth is live**. Everything downstream is code-complete or not yet built. The loop is not compounding today.

### What signal quality is required?

**SFT on `{brief, accepted_diff}` is the floor.** This teaches the model "given task X, produce diff Y." It is useful but weak — there's no negative signal.

**DPO on `{brief, accepted_diff, rejected_diff}` is the goal.** This teaches the model "given task X, *prefer* Y *over* Z." DPO has 3–10× the data efficiency of SFT for code tasks. The apprenticeship substrate is designed for this — the *senior verdict* (refine/reject) creates the rejected pair.

**Critical observation:** the shadow path (`/v1/shadow`) produces SFT pairs only, not DPO pairs. The apprenticeship full path (`/v1/brief` → `/v1/verdict`) produces DPO pairs. The shadow path is easier to deploy but weaker signal. **Plan for both. Run shadow as the bulk-capture layer; reserve apprenticeship for the high-signal pairs.**

### Minimum viable corpus

For a first LoRA on OLMo 3 7B Think that is *worth deploying* (improves the eval set without regressing other tasks):

- **SFT-only path:** ~2,000–3,000 high-quality `{brief, accepted_diff}` pairs. At current Foundry rate (~1–2/day usable), this is 3–6 years.
- **DPO path:** ~500–1,000 verdict pairs. At current rate, 2–4 years.
- **Synthetic augmentation:** generate variants of existing accepted diffs (rename, refactor, comment) to expand the SFT pool ~5–10×. Reduces SFT timeline to ~6 months. Risk: synthetic-induced mode collapse on stylistic patterns.

**Recommendation: dual-track corpus.**
- Bulk SFT pool from shadow capture + synthetic augmentation → first LoRA in 6–9 months
- DPO refinement pool from apprenticeship verdicts → second-generation LoRA at 12–18 months
- The DPO model is what becomes the "Woodfine Model" worth talking about publicly

### Does it actually compound?

**Yes — but only if three things happen:**

1. **Routing shifts.** A new LoRA must produce measurable shift of traffic from Tier C to Tier B/A. If routing is static, the new model just sits there. The complexity classifier in the shim must re-evaluate routing thresholds when a new adapter ships.
2. **Eval harness gates deployment.** Each new adapter must beat the previous one on a held-out eval set before it ships. Otherwise you ship regressions and the loop becomes a random walk.
3. **Multi-user data rate.** Single-operator rate is too slow. The compounding becomes real at ~10+ committed diffs per day, which is roughly 3–5 active developers.

**The honest answer:** the loop compounds in theory and will compound in practice only after app-console-slm has 3+ users. Today it accumulates a useful corpus but does not flywheel.

---

## 4. Model selection over time (2026–2030)

### 2026 (now)

- **Tier A:** OLMo 2 1124 7B Instruct Q4 (per tier-architecture-2026 recommendation; current drift between 1B and 7B Think to be reconciled)
- **Tier B trainer:** OLMo 3 32B Think on L4 (proven for extraction; adequate for moderate edits)
- **Tier B graph:** Llama 3.3 70B on H100 (process-only, audit-flagged)
- **Tier C:** Claude (Anthropic API passthrough)

### 2027 (likely)

- **AI2 release cadence:** OLMo 4 7B and 32B variants likely by Q2–Q3 2027. OLMo 3 has been a notable jump; OLMo 4 will probably narrow the Sonnet gap to ~12 months.
- **Tier A:** OLMo 4 7B-Think on CPU (if quantisation continues to improve) or stay on OLMo 2/3 7B Instruct if Think-on-CPU latency is unworkable. Likely transition: mistralrs-server with LoRA hot-swap, native Anthropic API on the engine side.
- **Tier B trainer:** OLMo 4 32B-Think on L4 — base for the first fine-tuned `engineering-pointsav` adapter. This is the year the LoRA starts paying off.
- **Tier B graph:** Could move to Llama 4 70B if it ships with comparable extraction quality and equivalent licensing. Reassess annually per `permissible-model-substrate.md`.
- **Tier C:** Claude (whatever's current). Maybe also Mistral Le Chat or Gemini for redundancy, all routed through the Doorman shim.

The Tier A/B split likely stays the same in 2027. The Tier A model becomes more capable; Tier B picks up code-edit-quality wins from fine-tuning.

### 2028 (the inflection year, if it comes)

This is the year the leapfrog thesis is tested.

**Optimistic scenario:** Fine-tuned OLMo 5 7B + apprenticeship LoRA handles 80% of "sonnet-tier" tasks from the Woodfine corpus. Cost structure shifts dramatically:

```
Today (2026):
  Tier A: ~$5/month VM amortised
  Tier B: ~$60/month average (1–3hr/day × $0.40)
  Tier C: ~$200/month (heavy Sonnet use)
  Total: ~$265/month per active developer

2028 optimistic:
  Tier A: ~$10/month (slightly heavier 7B-Think workload)
  Tier B: ~$20/month (much less Tier B as Tier A absorbs work)
  Tier C: ~$30/month (only opus-tier residual)
  Total: ~$60/month per active developer
```

**The business shape:** at $60/month/developer marginal cost, app-console-slm at $100–150/seat/month is a credible product with healthy unit economics. The customer comparison is Claude Code at $100/seat/month with no sovereignty — same price, sovereign substrate, BCSC posture, audit trail, fine-tuned-on-your-code model. That's a real product.

**Pessimistic scenario:** Open-data models stall. OLMo 5 7B handles only 50% of sonnet-tier tasks; Tier C remains 50% of token volume. Total marginal cost stays near $150/developer/month. App-console-slm has thinner margins but still works at $200–250/seat for sovereignty-motivated customers (financial services, healthcare, government).

### 2030 — defensibility

**Can a hyperscaler replicate the moat by 2030? Partial yes, partial no.**

What they can replicate:
- A frontier model running in a Canadian data centre with data-residency guarantees
- A logging substrate that satisfies most enterprise audit requirements
- A "no training on your data" contractual commitment
- A vector-database + RAG layer customised to the customer's documents

What they cannot replicate:
- A WORM ledger they don't operate (sovereignty requires customer-controlled infrastructure)
- An audit trail cryptographically signed by keys they don't hold
- A fine-tune trained *exclusively* on the customer's accepted patterns (their fine-tunes are contaminated with their other customers' data and base-model distributions)
- BC Securities Commission jurisdictional alignment (they're regulated under US/EU regimes, not Canadian)
- The cumulative apprenticeship corpus built over 4–5 years inside the customer's appliance
- The economic model: fixed infra cost beats per-token billing at any team size beyond 2–3 developers

**Defensibility verdict:** the moat is real but narrow. It defends against hyperscalers; it does NOT defend against another sovereign appliance vendor. If a competitor builds the same substrate with the same posture, the moat shrinks to first-mover advantage in customer relationships. That's why locking the Woodfine reference deployment and getting 2–3 paying customers by 2028 matters more than perfecting the technology.

---

## 5. Wire format as competitive infrastructure

### What the gateway unlocks beyond pure model improvements

The neutral canonical IR (A2A + MCP + Anthropic Messages + OpenAI Responses) is the **most important strategic move** in the plan, and it's underweighted in the current planning. Here's why:

**Without the gateway:** service-slm is a sovereign inference appliance. Niche product, narrow market. Sells to ~100 BC-jurisdiction SMBs that care intensely about sovereignty.

**With the gateway:** service-slm is the *control plane* between any AI client and any AI provider. The provider can be Anthropic, OpenAI, Google, Mistral, or your own LoRA on Yo-Yo. The client can be Claude Code, Cursor, OpenAI SDK, MCP-compatible agents, future agentic frameworks. Suddenly the product is "the Canadian-jurisdiction AI router" — that's a 10× larger market.

**Concrete unlocks:**

1. **Cost arbitrage across providers.** When Anthropic raises prices or rate-limits, route to Vertex Claude or even OpenAI o-series. Customer doesn't have to change a line of code.
2. **Provider redundancy as a feature.** Claude API outage? Auto-failover to Bedrock Claude. This is a real enterprise concern hyperscalers cannot offer (they ARE the single point of failure).
3. **Audit consolidation.** All AI use across multiple providers logged in one WORM ledger with consistent schema. That's a compliance officer's dream and is a real procurement criterion in regulated sectors.
4. **Model-mix routing.** Sensitive tasks → local OLMo, mainstream tasks → Claude, frontier tasks → Opus, with policy declared once at the gateway. No client-side complexity.
5. **A2A node identity.** When the agent-mesh ecosystem matures (it will, the standards are real), having a sovereign A2A node is the only way SMBs participate without renting capability from a hyperscaler.

### Is the gateway a product or infrastructure?

**Both — and which one matters at each customer size.**

- **Solo developer / small team:** the gateway is infrastructure inside app-console-slm. They don't think about it; they use the console.
- **Mid-market customer (10–50 users):** the gateway is a product. They want to standardise on it as the AI control plane, plug in their existing Claude/OpenAI keys, get unified billing and audit.
- **Enterprise (50+ users):** the gateway is a *licensable platform*. They embed it in their existing developer tooling, route their existing AI clients through it, use it as the policy enforcement point.

This means the productisation path is: app-console-slm first (single integrated experience), then unbundle the gateway as a standalone product (slm-doorman-as-a-service / on-prem appliance), then license the gateway technology (the Rust crates + the audit ledger schema + the operational tooling).

**Pricing model implications:**

- App-console-slm: per-seat ($50–150/month, like Claude Code)
- Gateway-as-product: per-route ($X per 1M tokens routed, plus annual base) or flat appliance license
- Gateway licensing: enterprise deal, six-figure annual

### How does the gateway interact with the own-model roadmap?

**Gateway value *increases* with own-model maturity, not the other way around.**

Here's why: the gateway's marginal value to a customer is the difference between (a) what they get from a single provider and (b) what they get from intelligent routing. If your only "alternative" to Anthropic is OpenAI, the routing value is modest (mostly redundancy and cost arbitrage). If your alternatives include a fine-tuned-on-your-data local model that handles 60% of tokens for free, the routing value is enormous — that's where 80% of the dollar savings come from.

The own-model is the *engine* that makes the gateway economically interesting. The gateway is the *interface* that makes the own-model usable. Neither works without the other.

**Implication:** don't position these as separate product tracks. They are one product. The gateway is the surface; the own-model is what powers the cheap tier; the routing intelligence is the glue.

---

## 6. Build sequence — strategic priority ordering

Given constraints (one operator, BCSC-regulated, open-data-only base, 16GB workspace, L4 trainer, H100 graph, Anthropic Tier C backstop), here is the priority ordering for the listed capabilities:

### Tier 1 — High leverage, low effort, unlocks everything else

**a. Universal gateway Sprint 0a (Anthropic shim)** — ~305 LOC, 2–3 days
- **Why first:** every other capability assumes the shim exists. Apprenticeship capture from Claude Code sessions, app-console-slm routing, multi-provider redundancy — all gated on the shim. Token cost reduction begins immediately.
- **Leverage: maximum.** This is the foundation of the leapfrog product.

**h. Ratify `conventions/permissible-model-substrate.md`** — 1 week of writing
- **Why second:** policy artefact that locks BCSC posture. Cheap to write, expensive to retrofit if you ship products without it and later find a model choice is unsupportable. Required for credibility in any customer conversation about sovereignty.
- **Leverage: high.** Defends the moat narrative against future drift.

**e. Git post-commit hook (diff capture closure)** — ~50 LOC
- **Why third:** the apprenticeship corpus is *the* moat asset. Capturing diffs is the difference between "we have task descriptions" and "we have DPO pairs." Tiny LOC, immediate compounding effect.
- **Leverage: maximum-future.** Every day this is unwired is a day of lost training signal.

### Tier 2 — Required for production loop, moderate effort

**c. Eval harness for Tier A and Tier B** — ~200 LOC + held-out set construction
- **Why:** without this, every LoRA you train is shipped on hope. The eval harness is the gating function for the entire compound loop. Build the held-out set from existing corpus (split 90/10) and run eval before each adapter deployment.
- **Leverage: high.** Inverts to maximum once first LoRA is trained — that day you must have the harness running.

**d. Corpus quality gate + PII scrub** — ~150 LOC
- **Why:** prevents poisoning before it happens. The current `redact.rs` exists; this builds on it. Acceptance criteria: min brief length 50 chars, min diff size 1 LOC, dedup by `(brief_hash, diff_hash)`, PII scrub via existing `sanitize`.
- **Leverage: high.** Cheap insurance against catastrophic corpus pollution.

**b. Persistent extraction queue** — ~50 LOC (per tier-architecture-2026)
- **Why:** eliminates the per-boot retry storm bug class. Lower strategic leverage than items above but high operational leverage.
- **Leverage: moderate.** A bug fix dressed as a feature.

### Tier 3 — Required for compounding, high effort

**f. First LoRA training run on Yo-Yo #1** — Phase 3 work; nightly-run.timer + corpus threshold + training scripts
- **Why:** the first concrete demonstration that the loop closes. Until this happens, the moat is theoretical.
- **Leverage: maximum-once-ready.** Cannot do this without c, d, e in place. Gate carefully.

**g. mistralrs-server migration** — Tier A engine swap with LoRA hot-swap
- **Why:** required for runtime adapter deployment. Defer to Sprint 1.5 per tier-architecture-2026 recommendation. Don't migrate just to migrate — wait until the first LoRA needs to be hot-loaded.
- **Leverage: high but late.** Wrong order if done before f.

### Tier 4 — Product-shaping, very high effort, high payoff

**i. app-console-slm** — Sprint 4, ~1800 LOC over 6–8 weeks
- **Why:** this is the *product*. Everything above is substrate. Without the console, service-slm is infrastructure that has no commercial surface.
- **Leverage: maximum-strategic.** This is the leapfrog product. But: defer until 0a + e + c are live. Building the console without the gateway + capture loop produces a "pretty CLI for Claude Code" — no compounding.

### Tier 5 — Speciality, low priority for leapfrog

**j. Tier B "graph" Yo-Yo (H100 + Llama 70B)** — already code-complete; deployment-only
- **Why deferred for leapfrog:** the graph Yo-Yo is operationally useful (entity extraction quality) but it's a process-only backend, not a compounding asset. It doesn't train the moat model. Run it when you need a fresh DataGraph build, not as a continuous priority.
- **Leverage: moderate operational, low strategic.**

### What can be deferred without losing compounding value?

- **j (graph Yo-Yo continuous use):** run quarterly batches, not continuous
- **g (mistralrs migration):** defer until f is imminent
- **i (app-console-slm)** can be deferred up to 6 months if the operator alone is the only user, but cannot be deferred past 12 months without losing the multi-user data rate that makes the flywheel real

### What CANNOT be deferred without breaking the thesis

- **a (Sprint 0a):** if not shipped within 30 days, every metric in this analysis is delayed by that amount
- **e (post-commit hook):** every day unwired is a day of permanent corpus loss
- **c (eval harness):** without this, you can never confidently deploy a LoRA
- **h (permissible-model convention):** without this, BCSC posture is undefended

### Recommended sequence (12-month rolling horizon)

| Month | Focus | Outcome |
|---|---|---|
| 1 | Sprint 0a + post-commit hook + corpus quality gate | Capture loop live; Claude Code routes through Doorman |
| 2 | Eval harness build + held-out set | Can measure model quality regression |
| 3 | permissible-model-substrate.md ratification + Tier A 7B upgrade + persistent queue | Substrate hardened |
| 4–5 | Sprint 1 (canonical IR) + Sprint 2 (native Anthropic Tier C) | Wire-format-neutral gateway |
| 6 | First LoRA training run (corpus permitting) + adapter signing | Loop closes; flywheel begins |
| 7–8 | mistralrs migration + first hot-swap deployment | Tier A becomes adapter-aware |
| 9–14 | Sprint 4 app-console-slm | Product surface live |
| 12+ | First external customer pilot (beyond Woodfine reference) | Validation of the leapfrog thesis |

---

## 7. The sovereign coding agent vision

### Is it the right end-state, or a stepping stone?

**It is a stepping stone, but a strategically important one.** The end-state is broader than a coding agent: it's a *sovereign AI control plane for SMBs*, of which the coding agent is one console application. App-console-slm code mode is the first; bookkeeper, content, email, legal-draft, BCSC-filing-assistant consoles are the broader product family.

The coding agent matters because:

1. **It's the highest-touch use case.** Developers use AI tools every day; bookkeepers use them weekly. Daily use means rich corpus accumulation.
2. **It validates the technical stack.** If app-console-slm code mode works as a daily driver, the broader product family is engineering-derisked.
3. **It's the dogfooding loop.** Foundry itself is built using app-console-slm; every improvement to the coding agent improves the rate of building everything else.

But it is *not* the killer customer use case for most SMBs. Most SMBs are not building software. Their killer use case is **regulated-data assistance that hyperscalers cannot provide because of jurisdiction or sovereignty constraints.** That's bookkeeping, that's correspondence drafting, that's compliance pre-flight, that's contract review.

### The killer use case that makes a customer switch

For a coding customer specifically: **price + sovereignty + capture.** A team of 5 developers spending $500/month on Claude Code subscriptions could move to app-console-slm at $400/month total ($80/seat) and get a fine-tuned-on-their-code model as a bonus after 6–12 months. The economics are real. The sovereignty story is the differentiator over alternatives like Cursor or Continue.

For a non-coding customer: **regulated-data work that can't go to a US-hosted API.** Examples:
- A Canadian accounting firm processing client tax data — cannot send to Anthropic per provincial privacy law interpretations
- A BC-regulated financial advisor handling client portfolio data — KYC/AML constraints favour sovereign data handling
- A municipal government processing constituent correspondence — data-residency policy
- A healthcare provider's administrative back-office (not the EHR itself) — PHIPA-style provincial regulation

For these customers, "Claude with sovereign substrate" is a category-of-one product. They are buying jurisdiction, not capability.

### What does "sovereignty" mean to a Claude Code customer?

Be honest: for a pure Claude Code customer today (using it for software dev, no regulated data involved), sovereignty means very little. They are renting capability. They care about price, latency, quality, and not having their code train someone else's product. Anthropic offers reasonable assurances on the latter and the price is acceptable.

**The leapfrog play for this customer is NOT "buy sovereignty" — it is "you can have a coding agent that is *trained on your code*, owned by you, and costs less."** That's the value proposition. Sovereignty is a side effect of the architecture, not the headline. Pitching sovereignty to a code-only customer is the wrong selling motion.

Save the sovereignty pitch for the regulated-data customer. Use the cost + customisation pitch for the developer-team customer. Same product, different framing.

---

## 8. The leapfrog product in one paragraph (2029)

**Sovereign AI Foundation (SAIF) appliance** — a Canadian-jurisdiction sovereign AI substrate running on a customer's own infrastructure (cloud, on-prem, or hybrid), comprising a WORM-anchored data ledger (os-totebox), a knowledge graph (service-content + LadybugDB), and an inference control plane (service-slm Doorman) that routes between a customer-fine-tuned local model trained on their own accepted patterns (Ring 3b LoRA on OLMo open-data base), elastic burst compute on a Canadian-region GPU node, and external frontier models (Anthropic, Mistral, Google) as a regulated passthrough. The customer-facing surface is the **app-console family** — app-console-slm for software development, app-console-bookkeeper for financial work, app-console-content for correspondence, all routing through the same sovereign substrate. Customers are Canadian SMBs (10–500 employees) in regulated sectors: financial services, healthcare administration, professional services, municipal government, and Crown-corporation subsidiaries. They get what no hyperscaler can provide: a cryptographically-verifiable audit trail of every AI interaction, structural (not contractual) data sovereignty, BC Securities Commission continuous-disclosure alignment, and a fine-tuned local model that gets better at *their* work specifically over time — at a cost structure that beats per-seat hyperscaler subscriptions at any team size. The BCSC-regulated BC company narrative is the trust frame: a small Canadian PBC-style operation accountable under NI 51-102 disclosure, with policy artefacts (DOCTRINE, conventions, ADRs) reviewable as continuous-disclosure documents. Hyperscalers cannot match the jurisdictional alignment, and the WORM ledger structure makes customer data exfiltration architecturally impossible rather than contractually prohibited. The product is not "cheaper AI" — it is "AI you can show to your auditor and your regulator without breaking a sweat."

---

## 9. Summary — strategic posture

**The leapfrog thesis is correct with one amendment:** the moat is the substrate + jurisdiction + audit trail, not the fine-tuned model. The model is what the customer notices; the substrate is what makes the model defensible.

**Three immediate priorities (next 30 days):**

1. Ship Sprint 0a (Anthropic shim) — opens the capture loop and the wire-format gateway story simultaneously
2. Wire git post-commit hook (50 LOC) — every day delayed is corpus loss
3. Ratify permissible-model-substrate.md — locks BCSC posture

**Three operational priorities (next 90 days):**

4. Eval harness on a held-out set (200 LOC) — gates the LoRA path
5. Corpus quality gate + PII scrub (150 LOC) — prevents pollution
6. Reconcile Tier A model drift; upgrade to OLMo 2 7B Instruct — Tier A quality matters more than expected

**Three product priorities (next 12 months):**

7. First LoRA training run + adapter signing — the moment the flywheel becomes real
8. mistralrs-server migration with hot-swap — required for live adapter deployment
9. App-console-slm code mode — the product surface customers actually interact with

**Three strategic risks to monitor:**

- **Data rate insufficient** if app-console-slm stays single-user past 12 months → mitigation: aggressive multi-user onboarding via paying pilot customer
- **Open-data model lag widens** if AI2 release cadence slows → mitigation: process-only Llama or Mistral usage stays permissible; accept Tier C dependency as permanent
- **Hyperscaler sovereign deployments** mature faster than expected → mitigation: structural sovereignty (WORM + jurisdiction + customer-controlled hardware) is the durable wedge, not contractual sovereignty

**Defensibility horizon:** the moat is real and growing through 2028. By 2030, hyperscalers will have matched the contractual sovereignty story. The structural sovereignty (WORM, jurisdiction, customer-fine-tuned model trained on customer data inside customer infrastructure) remains uncopyable. First-mover advantage in customer relationships within Canadian regulated SMBs is the durable position. Lock 3 paying customers by 2028 and the leapfrog has succeeded.
