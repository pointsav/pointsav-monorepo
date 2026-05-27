# app-orchestration-slm

Yo-Yo broker chassis — connects multiple Totebox Archives to a shared Yo-Yo GPU fleet.

**Licence:** Proprietary. See `CLAUDE.md` for the commercial-tier boundary.

## Ports

| Port | Role |
|---|---|
| `:9180` | Chassis HTTP — Totebox Doorman connections |

## Quick start

```bash
# No Yo-Yo configured (fleet registration only)
ORCHESTRATION_BIND_ADDR=127.0.0.1:9180 \
    cargo run -p orchestration-slm-server

# With Yo-Yo fleet
ORCHESTRATION_BIND_ADDR=0.0.0.0:9180 \
ORCHESTRATION_YOYO_TRAINER_ENDPOINT=http://10.10.0.5:8080 \
ORCHESTRATION_YOYO_GRAPH_ENDPOINT=http://10.10.0.6:8080 \
ORCHESTRATION_YOYO_BEARER=<token> \
    cargo run -p orchestration-slm-server
```

## Totebox connection

Each Totebox Doorman points its Yo-Yo config at the chassis:

```bash
SLM_YOYO_DEFAULT_ENDPOINT=http://<chassis>:9180/v1/yoyo/proxy
SLM_YOYO_TRAINER_ENDPOINT=http://<chassis>:9180/v1/yoyo/trainer
SLM_YOYO_GRAPH_ENDPOINT=http://<chassis>:9180/v1/yoyo/graph
```

## Architecture

See `CLAUDE.md` for full architecture notes.
