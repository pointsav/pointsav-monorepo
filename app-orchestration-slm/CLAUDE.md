# app-orchestration-slm

Yo-Yo broker chassis for multi-Totebox Archive commercial tier. Stateless — holds
no persistent data of its own. Connects multiple `service-slm` Doorman instances
to a single shared Yo-Yo fleet and provides per-tenant metering above base-node
capacity.

**Licence: Proprietary — paid.** This is the commercial boundary in the SLM
product line. Every Totebox Archive runs `service-slm` free. This chassis is
what connects them.

---

## What this crate does

DOCTRINE claim #23 in practice:

| Tier | Description | Cost |
|---|---|---|
| Solo Totebox (Tier A) | Local OLMo 1B/7B on `hardware` or `accelerated` node | Free |
| Multi-Totebox via chassis (Tier B) | Shared Yo-Yo fleet brokered through this chassis | Paid |

Each connected Totebox points its Doorman at the chassis instead of directly at
a Yo-Yo VM:

```
SLM_YOYO_DEFAULT_ENDPOINT=http://<chassis>:9180/v1/yoyo/proxy
SLM_YOYO_TRAINER_ENDPOINT=http://<chassis>:9180/v1/yoyo/trainer
SLM_YOYO_GRAPH_ENDPOINT=http://<chassis>:9180/v1/yoyo/graph
```

The chassis validates identity, injects `X-Foundry-Module-ID` for per-tenant
isolation, meters cost, and proxies to the actual Yo-Yo VM.

---

## Stateless rule

`app-orchestration-slm` holds no persistent data. The fleet registry is rebuilt
from Doorman `POST /v1/discovery/register` calls on startup. Per-tenant metering
is in-process; the source of truth stays in each Totebox's own Doorman audit
ledger.

If you find a code path where this chassis writes to a Totebox archive directly,
it is a bug.

---

## Yo-Yo fleet — existing nodes only

Do NOT provision new Yo-Yo VMs. Use the existing fleet:

| Label | Node | Model | Env var |
|---|---|---|---|
| `"trainer"` | Yo-Yo #1, L4 24GB | OLMo 3 32B-Think | `ORCHESTRATION_YOYO_TRAINER_ENDPOINT` |
| `"graph"` | Yo-Yo #2, H100 80GB | Llama 3.3 70B grammar | `ORCHESTRATION_YOYO_GRAPH_ENDPOINT` |
| `"proxy"` | Either (default) | General inference | `ORCHESTRATION_YOYO_DEFAULT_ENDPOINT` |

Capacity planning deferred until the chassis is tested end-to-end.

---

## Build and test

```bash
cargo check --workspace
cargo test  --workspace
```

Run against a local Doorman (no Yo-Yo configured):

```bash
ORCHESTRATION_BIND_ADDR=127.0.0.1:9180 \
    cargo run -p orchestration-slm-server
```

---

## MVP endpoints

| Endpoint | Method | Phase |
|---|---|---|
| `/healthz` | GET | MVP |
| `/readyz` | GET | MVP |
| `/v1/fleet` | GET | MVP |
| `/v1/discovery/register` | POST | MVP |
| `/v1/yoyo/proxy` | POST | MVP |
| `/v1/yoyo/trainer` | POST | MVP |
| `/v1/yoyo/graph` | POST | MVP |
| `/v1/graph/federated` | POST | Phase 2 |
| `/v1/training/schedule` | POST | Phase 2 |
| `/v1/adapters` | GET | Phase 2 |
| `/v1/audit/rollup` | GET | Phase 2 |

---

## Hard constraints

- **Do not write to Totebox archives.** Chassis is read-proxy + scheduling only.
- **No new Yo-Yo VMs.** Use existing fleet until tested.
- **Stateless rule.** No persistent state. Rebuild from heartbeats on restart.
- **Per `~/Foundry/AGENT.md` hard rules:** SYS-ADR-07, SYS-ADR-10, SYS-ADR-19.
- **Licence boundary:** everything inside `app-orchestration-slm/` is proprietary.
  Do not make this Apache-2.0 or EUPL.
