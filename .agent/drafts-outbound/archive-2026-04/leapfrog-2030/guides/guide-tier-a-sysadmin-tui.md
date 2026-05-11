---
schema: foundry-guide-v1
state: refined-pending-master-commit
target_repo: customer/woodfine-fleet-deployment
target_path: vault-privategit-source/guide-tier-a-sysadmin-tui.md
audience: operators
language_protocol: PROSE-GUIDE
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
research_done_count: 8
research_suggested_count: 2
open_questions_count: 0
research_provenance: master-empirical-tier-a-swap + slm-cli-design-from-cluster-research
research_inline: true
notes_for_editor: |
  English-only per CLAUDE.md §14. Body is within the 800-1,200 word target.
  All command examples preserved.
---

# Operating the Tier A Sysadmin TUI

This guide describes how an operator uses the `slm-cli` System Administrator
terminal interface (TUI) to run the Foundry substrate. It assumes the Totebox
is provisioned and the `local-doorman` service is running.

This guide reflects the intended slm-cli design. It becomes fully operational
when slm-cli ships in Phase 4 of the leapfrog roadmap. Until then, the
curl-based interface in §6 is the proof-of-life equivalent.

## Prerequisites

- A provisioned Totebox or workspace VM
- `local-doorman.service` active: `systemctl is-active local-doorman.service`
- `local-slm.service` active (Tier A llama-server with OLMo 2 1B Instruct Q4
  model loaded)
- Operator authenticated to the Totebox (SSH or local console)

## What you can do with the TUI

The Tier A specialist answers narrow-domain questions about your Totebox
operations:

- **Sysadmin queries**: "What does this systemd error mean?", "How do I check
  disk space?", "Show me the last hour of logs for service-fs"
- **Audit ledger lookups**: "What was the last marketplace transaction?",
  "Which audit entries were created today?"
- **Knowledge graph navigation**: "Who reports to ARTHUR_PENDELTON?", "What
  documents reference Q3 Capital Procurement?"
- **Routine operations**: "Generate a commit message for these changes",
  "Validate this systemd unit syntax"

The Tier A specialist responds in under ten seconds and is narrowly scoped to
your Totebox. For broader work (editorial passes, bilingual generation, complex
reasoning), enable Tier B on-demand.

## Starting a session

```
$ slm-cli
```

The TUI opens with a welcome banner showing tenant, current tier, adapter
version, and connection status:

```
service-slm // Totebox Archive System Administrator
tenant: woodfine | tier: A (local) | adapter: it-support-v0.0.1
connection: local-doorman:9080 (healthy)
```

Type a question at the bottom of the screen and press Enter; the response
streams in.

## Common commands

| Command | What it does |
|---|---|
| `/help` | List all available commands |
| `/status` | Health check: service-fs, service-content, service-doorman |
| `/audit [date]` | Query the audit ledger |
| `/audit today` | Today's audit entries |
| `/audit 2026-04-30` | Specific-date entries |
| `/graph [entity]` | Query the knowledge graph |
| `/graph ARTHUR_PENDELTON` | Show entity and relations |
| `/search [query]` | Keyword search across content |
| `/feedback good` | Mark the previous response as a good training tuple |
| `/feedback bad` | Mark the previous response as a bad training tuple |
| `/feedback refine [correction]` | Provide a corrected response |
| `/tier [a\|b\|auto]` | Force a tier for the next message (debug only) |
| `/adapters` | Show loaded adapter versions |
| `/export` | Prepare a transfer bundle of the tenant's data |
| `/marketplace browse` | Browse marketplace listings |
| `/marketplace enable` | Walk through marketplace opt-in for this tenant |

## Why your feedback matters

Every `/feedback` you provide is a training tuple for your tenant's specialist
adapter. The Tier A specialist learns specifically from your verdicts — over
weeks the responses are intended to become more useful for your operations, your
vocabulary, and your workflows.

Published research suggests 200 to 500 explicit-verdict interactions are
sufficient to produce a usable IT-support adapter for a narrow domain. At a
daily verdict rate of 5 to 10, your specialist reaches that threshold in
approximately one month.

## When the response is wrong

The Tier A specialist is a 1-billion-parameter model. It will sometimes be
wrong, particularly:

- When asked about uncommon software your Totebox does not have installed
- When asked about events outside the audit ledger window
- When asked complex multi-step reasoning questions

For wrong responses:

1. Press `R` (or type `/feedback refine`) and provide the correct answer
2. The TUI captures both the wrong response and your correction; both contribute
   to the next adapter training cycle
3. If the question is structurally outside Tier A scope, escalate with `/tier b`
   or mark `/feedback bad` and rephrase

## When Tier A is unavailable

If `systemctl is-active local-slm.service` returns inactive (model not loaded,
hardware insufficient, or operator-stopped), the TUI enters deterministic-only
mode:

- Slash commands continue to work (`/status`, `/audit`, `/graph`, `/search`,
  `/export`, `/help`)
- Natural-language chat is disabled with a clear status message
- The status bar shows "AI-disabled — deterministic operations only"

This is the substrate-without-inference base case. Per
`~/Foundry/conventions/substrate-without-inference-base-case.md`, your Totebox
remains operational and your data remains accessible even with all AI tiers
offline.

## Restarting Tier A

If the Tier A specialist stops responding or returns errors:

```
$ sudo systemctl restart local-slm.service
$ sudo systemctl restart local-doorman.service
$ slm-cli  # restart the TUI
```

The first response after restart may take 30–60 seconds (the model loads from
disk and warms up). Subsequent responses are fast.

## Switching to a fine-tuned adapter

When your tenant's IT-support adapter is ready (after 200+ verdict-signed
tuples), an operator with the appropriate identity key deploys it:

```
$ slm-cli /adapters install it-support-woodfine-v0.0.1
```

The TUI confirms the adapter is loaded; the next response uses the fine-tuned
weights. To compare against the base model:

```
$ slm-cli /adapters disable it-support-woodfine-v0.0.1
```

Adapters can be enabled or disabled per-session without restarting the service.

## When you need editorial or bilingual work

Tier A is purpose-routed for sysadmin and IT-support questions. For editorial
passes (drafting a customer newsletter, refining a long document), bilingual
generation, or complex reasoning, enable Tier B:

```
$ slm-cli /tier b
$ <your editorial request>
```

If Tier B (Yo-Yo) is not currently running, the TUI starts it on-demand.
Wake-up takes 60–90 seconds; subsequent responses are fast (50–100 tokens per
second). Tier B idle-shuts-down after 30 minutes of inactivity to control cost.

## Configuring marketplace and ad-exchange flows

When you are ready to monetize your tenant's data assets:

```
$ slm-cli /marketplace enable
```

The TUI guides you through:

1. Inventory category selection
2. Per-category consent term review
3. Pricing configuration
4. Settlement rail selection (Stripe Connect or crypto wallet)

The configuration is signed by your operator identity key and recorded in the
audit ledger. From this point, marketplace listings become available to external
buyers; settlement events flow to your configured payment rail directly.

To disable: `slm-cli /marketplace disable`. All listings are withdrawn;
existing transactions complete normally.

## When something does not match this guide

If the TUI behavior differs from this guide, check two possibilities:

1. **Older slm-cli version.** Check `slm-cli --version`; update via
   `sudo systemctl restart local-slm-cli.service` after the next Foundry
   release.
2. **Regression.** File via the operator outbox
   (`~/Foundry/.claude/outbox.md` if you operate the workspace dogfood) or
   contact the Foundry vendor channel.

Per CLAUDE.md §6 surface-drift discipline, regressions are surfaced as cleanup
items, not silently absorbed.

---

## Companion guides

- `guide-doorman.md` — operating the local-doorman.service
- `guide-knowledge-runtime-operations.md` — service-content lifecycle
- `guide-marketplace-operations.md` — marketplace seller and buyer operations
- `guide-yoyo-auto-wake-discipline.md` — Tier B GPU lifecycle

## References

- Doctrine: claims #45 (TUI-as-Corpus-Producer), #49 (Tier 0 Sovereign
  Specialist), #54 (Substrate-Without-Inference Base Case)
- Conventions: `tui-corpus-producer.md`,
  `tier-zero-customer-side-sovereign-specialist.md`,
  `substrate-without-inference-base-case.md`
- Engineering reference: `vendor/pointsav-monorepo/service-slm/crates/slm-cli/`
