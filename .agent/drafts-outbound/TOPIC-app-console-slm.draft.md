---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "app-console-slm — inference infrastructure monitoring console"
slug: app-console-slm
target_repo: media-knowledge-documentation
target_path: media-knowledge-documentation/applications/app-console-slm.md
paired_with: app-console-slm.es.md
category: applications
quality: complete
bcsc_class: public-disclosure-safe
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — app-console-slm Phase D code read, F9 cartridge design"
research_inline: true
---

# app-console-slm — inference infrastructure monitoring console

app-console-slm is a terminal user interface (TUI) cartridge for the operator console
that displays the live state of the AI inference infrastructure. It shows the health
of the local inference model, the status of remote GPU nodes, the depth of the
priority queue, the organizational graph entity count, and the current day's spending.
It provides keyboard controls for adjusting the routing policy and toggling per-tier
kill switches.

The console runs in a terminal window on the same node as the inference gateway. It
requires no browser, no network connection to an external service, and no
authentication beyond local shell access. It is the operator's primary dashboard for
understanding and controlling the inference layer.

## Display panels

The console organizes information into five panels that refresh automatically every
ten seconds. The operator can trigger an immediate refresh at any time with the R key.

### Gateway panel

The gateway panel shows the current state of the inference router: whether it is
running, the active routing policy (balanced, drain-batch, drain-express, or
local-only), and the availability of each tier. A green indicator marks a tier as
available. A yellow indicator marks a tier as degraded — available but with recent
failures. A grey indicator marks a tier as offline.

The gateway panel also shows the active routing policy and, if a tier's kill switch
is closed, an explicit "kill: CLOSED" label.

### GPU node fleet panel

The fleet panel shows each configured remote GPU node with its current state. States
are: stopped (VM is off, no billing), starting (VM is booting, billing has begun),
available (VM is ready and healthy), failed (VM failed to start or become healthy),
and zombie (VM is running but unresponsive). For available nodes, the panel shows the
most recent probe latency in milliseconds.

Each node has an independent kill switch. The K key opens a dialog to toggle the
kill switch for any node, or to close all switches globally.

### Organizational graph panel

The graph panel shows the total entity count in the organizational knowledge graph,
the number of distinct edge types present, and the timestamp of the most recent
successful extraction. It shows the circuit breaker state for the graph service:
if the inference router's graph query path has experienced repeated failures and
opened its circuit, the panel displays the time elapsed since the circuit opened.

### Queue panel

The queue panel shows the current depth of each priority queue level. P0 holds
background classification tasks. P1 holds extraction tasks awaiting a GPU node.
P2 holds training corpus generation and apprenticeship work. The panel also shows
the total completed and the current poison count — tasks that have failed the
maximum number of retry attempts and require operator review.

### Cost panel

The cost panel shows the current day's spending across all tiers in the deployment's
configured currency. The panel breaks down spending by node label: the batch node,
the express node, and the external API (if configured). This gives the operator
immediate visibility into whether a scheduled nightly drain has concluded and at
what cost.

## Keyboard controls

| Key | Action |
|---|---|
| R | Immediate refresh — re-queries all status endpoints |
| K | Kill switch dialog — toggle per-tier or global kill switch |
| P | Policy dialog — select routing policy (balanced / drain-batch / drain-express / local-only) |
| G | Graph detail — show entity type breakdown and recent extraction activity |
| ? | Help overlay — show all keybindings |
| Q | Quit |

## Technical characteristics

The console is a library crate that implements the Cartridge trait for the operator
console chassis. It loads at slot F9. Communication with the inference gateway uses
standard HTTP against the gateway's monitoring endpoints; no special protocol is
required. The console performs only read operations by default; write operations
(kill switch toggles, policy changes) require explicit keyboard confirmation.

The console uses a background polling task that fetches status data every ten seconds
and sends it to the rendering task via a channel. The rendering task does not block
on network requests; it displays whatever data arrived most recently. This design
ensures the console remains responsive even when the gateway is slow to respond.

The display degrades gracefully when individual status endpoints are unavailable.
Missing panels show a "— unavailable —" indicator rather than preventing the console
from rendering.

Plain-text mode is available via the `--plain` flag for terminal environments without
unicode support. Unicode status symbols are replaced with ASCII equivalents.

## Relationship to the inference gateway

The console is a read-mostly observer of the inference gateway. It does not participate
in routing decisions. Kill switch and policy commands sent through the console take
effect immediately in the gateway, but the console does not verify the effect beyond
showing the updated state on the next refresh cycle.

The console is deployed alongside the inference gateway on the same node. It does not
require network connectivity to external services to function. If the inference gateway
is unreachable, the console continues running and shows all panels as unavailable.
