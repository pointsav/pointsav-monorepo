---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Deploy and operate app-console-slm — the inference infrastructure TUI"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-console-slm-deploy.md
target_filename: guide-console-slm-deploy.md
bcsc_class: no-disclosure-implication
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — app-console-slm Phase D code, F9 cartridge design"
research_inline: true
---

# Deploy and operate app-console-slm — the inference infrastructure TUI

app-console-slm is the terminal dashboard for monitoring the inference infrastructure.
It loads as cartridge F9 in the operator console (os-console chassis). This guide
covers enabling the cartridge and using the dashboard controls.

## Pre-flight

```bash
# Confirm the os-console chassis is running
systemctl status local-console.service

# Confirm service-slm Doorman is running and healthy
curl -s http://127.0.0.1:9080/healthz
```

## Step 1 — Build the console cartridge

```bash
cd /srv/foundry/clones/project-intelligence
cargo build -p app-console-slm --release
```

The built cartridge is loaded automatically by the console chassis when it is
installed to the correct location:

```bash
sudo cp app-console-slm/target/release/libapp_console_slm.so \
  /usr/local/lib/console-cartridges/f9_slm.so
sudo systemctl restart local-console.service
```

## Step 2 — Configure cartridge endpoints

The cartridge reads endpoints from environment variables. Add to the console
service environment or set in the shell before launching the console:

```bash
SLM_DOORMAN_ENDPOINT=http://127.0.0.1:9080
SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180  # optional; omit if no chassis
```

## Step 3 — Open the SLM panel

Launch the operator console:
```bash
console
```

Press **F9** to switch to the SLM infrastructure panel.

The panel loads with the last cached state and begins polling immediately.

## Reading the dashboard

```
╭─ F9 — SLM + DataGraph ─────────────────────────────────────────╮
│  Gateway ● running  Policy: balanced                           │
│  Tier A: ✓  Tier C: ○                                         │
├─ YoYo Fleet ───────────────────────────────────────────────────┤
│  batch   (L4)    ● available  145ms  kill: OPEN               │
│  express (A100)  ○ stopped    —      kill: OPEN               │
├─ DataGraph ─────────────────────────────────────────────────────┤
│  Entities: 7,445  Circuit: closed  Last: 4 min ago             │
├─ Queue ─────────────────────────────────────────────────────────┤
│  P0: 0   P1: 12   P2: 391   done: 799   poison: 11            │
├─ Cost Today ────────────────────────────────────────────────────┤
│  $2.47  batch: $0.71  express: $0.00  Tier C: $1.76           │
╰─ [K]ill  [P]olicy  [G]raph  R=refresh  ?=help ──────────────────╯
```

**Gateway row:** ● = running, ✗ = unreachable. Policy shows the active routing mode.

**Tier A:** ✓ = model healthy. Tier C: ○ = disabled (no API key).

**Fleet rows:** State is one of: ● available, ○ stopped, ⟳ starting, ✗ failed/zombie.
Latency shown in ms for available nodes. `kill: OPEN/CLOSED` shows kill switch state.

**DataGraph:** Entity count from `/healthz`. Circuit shows if the graph query path
is open (degraded) or closed (healthy). "Last" is time since the most recent
successful extraction.

**Queue:** Depth per priority level. "poison" = tasks requiring operator review.

**Cost Today:** Cumulative spend since midnight UTC across all tiers.

## Keyboard controls

| Key | Action |
|---|---|
| **R** | Immediate refresh — re-queries all endpoints |
| **K** | Kill switch dialog — toggle per-label or global |
| **P** | Policy dialog — change routing policy |
| **G** | Graph detail — entity type breakdown |
| **?** | Help overlay |
| **Q** | Quit |

### Using the kill switch dialog (K)

Press K to open the kill switch dialog. Navigate with arrow keys:

```
Kill Switch Control
──────────────────
  batch    [OPEN  ] → toggle
  express  [OPEN  ] → toggle
  tier-c   [OPEN  ] → toggle
  GLOBAL   [OPEN  ] → toggle all
──────────────────
  Enter: toggle  Esc: cancel
```

Closing a switch stops all new dispatching to that tier immediately. In-flight
requests complete; queued work accumulates until the switch is reopened.

### Using the policy dialog (P)

Press P to open the routing policy selector:

```
Routing Policy
─────────────────────────────
  ● balanced        (default)
  ○ drain-batch     (all work to L4)
  ○ drain-express   (all work to A100)
  ○ local-only      (Tier A only)
─────────────────────────────
  Enter: apply  Esc: cancel
```

The policy change takes effect immediately in the Doorman without restart.

## Connecting to the chassis instead of the Doorman

If `SLM_ORCHESTRATION_ENDPOINT` is set, the console also shows the fleet panel
with all registered Doorman instances. This is useful when multiple archives share
the same GPU nodes through app-orchestration-slm.

## Troubleshooting

| Symptom | Check |
|---|---|
| Gateway shows ✗ | `curl http://127.0.0.1:9080/healthz` — is Doorman running? |
| Fleet rows missing | `SLM_ORCHESTRATION_ENDPOINT` set? Chassis running? |
| Cost Today: $0.00 | Normal when no GPU work has run today |
| Queue poison > 0 | Run `ls /srv/foundry/data/apprenticeship/queue-poison/` to inspect failed briefs |
| Panel not updating | Press R; check network to Doorman |
