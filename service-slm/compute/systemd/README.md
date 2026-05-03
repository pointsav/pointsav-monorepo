# service-slm Doorman — systemd unit

This directory contains the systemd unit and installation script for the Doorman, the secure gateway between Totebox and external LLMs.

## Purpose

The Doorman (`slm-doorman-server` binary) is a three-tier router that:

- **Tier A (Local):** Routes to OLMo 3 7B Q4 running on the same VM via `local-slm.service`
- **Tier B (Yo-Yo):** Bursts to GCP Cloud Run / RunPod / Modal for compute-intensive tasks
- **Tier C (External API):** Routes to Anthropic Claude, Google Gemini, or OpenAI via narrow allowlist

The Doorman:
- Sanitises outbound payloads before routing
- Logs every call (per-call audit ledger at `/var/lib/slm-doorman/audit/`)
- Enforces cost guardrails (Tier C disabled unless explicitly configured)
- Serves OpenAI-compatible `/v1/chat/completions` endpoint on `127.0.0.1:9080`
- (v0.1.x+) Implements Apprenticeship Substrate for model training feedback

## Installation (Master scope)

**Requirements:**
- `local-slm.service` running on the same VM (Tier A, listening on port 8080)
- Rust toolchain on the VM (or pre-built release binary)
- systemd

**Quick start:**

```bash
cd /srv/foundry/infrastructure/slm-doorman
sudo bash bootstrap.sh
```

The bootstrap script:
1. Builds the release binary from `cluster/project-slm` (if not already built)
2. Creates the `slm-doorman` system user and group
3. Installs the binary to `/usr/local/bin/slm-doorman-server`
4. Installs the systemd unit to `/etc/systemd/system/`
5. Enables the unit to start on boot

## Configuration

All configuration is via environment variables in `slm-doorman.service`. Edit the file and run:

```bash
sudo systemctl daemon-reload
sudo systemctl restart slm-doorman
```

### Tier A (Local — always enabled)

```
Environment="SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080"
Environment="SLM_LOCAL_MODEL=Olmo-3-1125-7B-Think-Q4_K_M.gguf"
```

Points to `local-slm.service`. No setup needed — Doorman boots in community-tier mode if Tier B/C are not configured.

### Tier B (Yo-Yo — optional)

To enable bursting to external GPU compute:

```
Environment="SLM_YOYO_ENDPOINT=<GCP Cloud Run URL or RunPod endpoint>"
Environment="SLM_YOYO_BEARER=<static bearer token from provider>"
Environment="SLM_YOYO_HOURLY_USD=0.84"  # GCP L4 example rate
Environment="SLM_YOYO_MODEL=Olmo-3-1125-32B-Think"
```

The Doorman computes cost per call as `(hourly_usd / 3,600,000) × inference_time_ms`.

**Cost guardrail:** Tier B disabled if `SLM_YOYO_ENDPOINT` is not set. No accidental bursts.

### Tier C (External API — optional, requires allowlist)

To enable calls to Anthropic Claude / Google Gemini / OpenAI:

```
Environment="SLM_TIER_C_ANTHROPIC_ENDPOINT=https://api.anthropic.com"
Environment="SLM_TIER_C_ANTHROPIC_API_KEY=sk-..."
Environment="SLM_TIER_C_ANTHROPIC_INPUT_PER_MTOK_USD=0.0003"
Environment="SLM_TIER_C_ANTHROPIC_OUTPUT_PER_MTOK_USD=0.0015"
```

**Allowlist enforcement:** Requests must include `X-Foundry-Tier-C-Label` header with one of these labels:
- `citation-grounding` — resolve citations against external knowledge base
- `initial-graph-build` — bootstrap semantic graph from corpus
- `entity-disambiguation` — clarify entity references

Requests without the header, or with an unlisted label, are denied before any network call is made.

**Cost guardrail:** Tier C disabled if no provider endpoint is configured. No accidental API calls.

### Apprenticeship Substrate (v0.1.x+)

Once `bin/apprentice.sh` + `bin/capture-edit.py` land (AS-5):

```
Environment="SLM_APPRENTICESHIP_ENABLED=true"
Environment="FOUNDRY_ROOT=/srv/foundry"
Environment="FOUNDRY_ALLOWED_SIGNERS=/srv/foundry/identity/allowed_signers"
Environment="FOUNDRY_DOCTRINE_VERSION=0.0.7"
Environment="FOUNDRY_TENANT=pointsav"
```

Enables three new endpoints:
- `POST /v1/brief` — seek apprenticeship attempt
- `POST /v1/verdict` — submit signed senior verdict
- `POST /v1/shadow` — record shadow brief for corpus

See `service-slm/ARCHITECTURE.md` §11 for the full apprenticeship flow.

## Status and logs

**Unit status:**

```bash
systemctl status slm-doorman
```

**View logs:**

```bash
journalctl -u slm-doorman -f    # tail the logs
journalctl -u slm-doorman -p err  # errors only
```

**Health check:**

```bash
curl http://127.0.0.1:9080/healthz
# Expected: HTTP 200
```

**Ready check:**

```bash
curl http://127.0.0.1:9080/readyz
# Expected: HTTP 200 if local-slm.service is responding
```

**View contract:**

```bash
curl http://127.0.0.1:9080/v1/contract | jq .
```

**Audit ledger:**

```bash
tail -f /var/lib/slm-doorman/audit/$(date +%Y-%m-%d).jsonl
```

Each line is a JSON event with `tier`, `model`, `cost_usd`, `completion_status`, etc.

## Operations

**Restart after config change:**

```bash
sudo systemctl restart slm-doorman
```

**Stop the service:**

```bash
sudo systemctl stop slm-doorman
```

**Uninstall (remove from systemd but leave binary/state intact):**

```bash
sudo systemctl disable slm-doorman
sudo systemctl daemon-reload
sudo rm /etc/systemd/system/slm-doorman.service
```

## Dependency on Tier A

The Doorman depends on `local-slm.service`. If Tier A is not running, the Doorman starts but returns HTTP 502 (Bad Gateway) on `/v1/chat/completions` calls that route to Tier A.

The unit declares:

```
After=local-slm.service
Wants=local-slm.service
```

This means:
- The Doorman waits for `local-slm.service` to start if both are enabled
- If `local-slm.service` stops, the Doorman continues running but cannot serve requests

## Cross-cluster consumption

All Task Claude sessions on the workspace VM can reach the Doorman at `http://127.0.0.1:9080`. Each call is logged with the originating `module_id` header.

Example from a cluster:

```bash
curl -X POST http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "X-Foundry-Module-ID: project-slm" \
  -d '{
    "model": "olmo-3-7b-instruct",
    "messages": [{"role": "user", "content": "hello"}]
  }'
```

The Doorman routes the request to the appropriate tier and logs the call with `module_id: project-slm`.

## Resource limits

The unit specifies conservative resource limits suitable for an e2-standard-4 VM:

```
MemoryMax=512M
CPUQuota=100%
```

The Doorman itself is lightweight; Tier A does the heavy lifting. Adjust if needed for your hardware.

## References

- `service-slm/ARCHITECTURE.md` — Full architecture and design
- `service-slm/DEVELOPMENT.md` — Build and development guide
- `infrastructure/local-slm/README.md` — Tier A (local SLM) setup
- `conventions/customer-first-ordering.md` — Installation order for customers
