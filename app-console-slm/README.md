<div align="center">

# app-console-slm | Sovereign Infrastructure Console

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** App Console — AI Infrastructure

</div>

<br/>

## I. PURPOSE

`app-console-slm` is the operator console for the Foundry AI infrastructure. It consolidates visibility and control over `service-slm` (Doorman), the Yo-Yo inference VM, the training corpus, and the DataGraph entity store into a single binary. It replaces manual curl checks, `start-yoyo.sh`, direct SSH to the Yo-Yo VM, and the `slm-chat.sh` shell script.

The binary connects to `service-slm` Doorman at the endpoint configured by `SLM_DOORMAN_ENDPOINT` (default: `http://127.0.0.1:9080`).

## II. BINARY SURFACE

### Sprint 4a — Planned

**`console-slm status`**

One-command health check. Reports Doorman availability, active tier (Tier A local / Tier B Yo-Yo / Tier C Anthropic), DataGraph reachability and entity count, training corpus row count, and apprenticeship ledger state.

```
Doorman    ● running  (http://127.0.0.1:9080)
Tier A     ● running  OLMo 2 1B   http://127.0.0.1:8080
Tier B     ● running  OLMo 3 32B  https://x.x.x.x:9443  14.7 tok/s
Tier C     ○ standby  Anthropic API  key: configured
DataGraph  ● healthy  entity_count=10414  last_run=02:52 UTC
Training   ⊙ pending  7 briefs queued  corpus: 0 tuples
```

**`console-slm admin`**

Operator TUI (ratatui). Panels:

| Panel | Replaces |
| :--- | :--- |
| Doorman tier health + circuit state | `curl http://127.0.0.1:9080/readyz` |
| Yo-Yo VM start / stop / snapshot | `start-yoyo.sh` / `stop-yoyo.sh` |
| Nightly run live log stream | `tail /tmp/nightly-run-*.log` |
| Corpus statistics | `corpus-stats.sh` |
| DataGraph entity count + delta | manual `jq datagraph-health.json` |
| Apprenticeship ledger summary | manual `ledger.md` reads |

### Sprint 4b — Deferred

**`console-slm code`** — Agentic coding loop with tool-use (Bash, Read, Write, Edit, Search). Deferred until: (a) team size reaches three or more developers, (b) the local sovereign model is consistently competitive with the current external model tier on Foundry-specific tasks, or (c) a product distribution motive emerges.

**`console-slm chat`** — Interactive REPL against the Doorman `/v1/messages` endpoint. Currently served by `slm-chat.sh`. Deferred to Sprint 4b.

**Note on Sprint 4b:** Claude Code with `ANTHROPIC_BASE_URL` pointing at Doorman is the correct coding agent surface for the current team scale. Sprint 4b becomes justified only when per-seat subscription cost at team scale exceeds fixed infrastructure cost, or when the local model quality warrants it.

## III. CURRENT STATE

Sprint 4a: planned. The crate is a structural placeholder. Implementation begins after Sprint 3 (MCP server) ships, which is the prerequisite for the Doorman client library this console depends on.

## IV. SPRINT SEQUENCE CONTEXT

This crate is part of the sovereign routing sprint sequence:

```
0a  Anthropic shim activation (DONE — http.rs:1214)
0b  Real streaming + on-demand Yo-Yo boot
1   Canonical IR (unlocks full Claude Code tool-use)
2   Native Tier C + OpenAI Responses API inbound
3   slm-mcp-server (MCP tools for DataGraph, corpus, Doorman)
4a  app-console-slm status + admin TUI        ← THIS CRATE
4b  app-console-slm code + chat (deferred)
5   A2A agent card
```

---
*© 2026 PointSav Digital Systems™.*
