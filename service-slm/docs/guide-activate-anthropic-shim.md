---
schema: foundry-doc-v1
document_version: 1.0.0
research_provenance: direct-consultation
research_inline: false
authored: 2026-05-16
authored_by: project-intelligence Totebox
authored_with: claude-sonnet-4-6
cites: []
---

# GUIDE: Activating the Anthropic Messages API Shim (Sprint 0a)

Sprint 0a is implemented in `crates/slm-doorman-server/src/http.rs:1214`. This guide
covers activation only — no code changes are required.

## Prerequisites

- [ ] Doorman running: `systemctl status local-doorman.service` → active
- [ ] Tier A healthy: `curl -s http://127.0.0.1:8080/health` → `{"status":"ok"}`
- [ ] Doorman readyz: `curl -s http://127.0.0.1:9080/readyz` → 200
- [ ] Commercial API key obtained (separate from Max subscription — not a Pro/Max OAuth
      token; see `topic-tos-training-constraints.md`)

## Step 1 — Configure Doorman for Tier C

Add to `/etc/local-doorman/local-doorman.env`:

```bash
ANTHROPIC_API_KEY=<commercial-api-key>
```

Restart to pick up:

```bash
sudo systemctl restart local-doorman.service
```

Verify Tier C is reachable (Doorman logs should show Tier C configured, not mock-only):

```bash
journalctl -u local-doorman.service | grep -i "tier.c\|external"
```

## Step 2 — Set Gateway-Internal Auth Token

Choose an arbitrary token string. This is validated by the shim only and is never
forwarded upstream to any backend.

Add to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_GATEWAY_TOKEN=<arbitrary-secret-string>
```

Restart Doorman again to pick up.

## Step 3 — Configure a Test Project

Create or update `.claude/settings.local.json` in the project root (gitignored — do not
commit this until the team agrees to route all sessions through the gateway):

```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "http://127.0.0.1:9080",
    "ANTHROPIC_AUTH_TOKEN": "<same-value-as-SLM_GATEWAY_TOKEN>",
    "ANTHROPIC_SMALL_FAST_MODEL": "claude-haiku-4-5-20251001"
  }
}
```

Use `.claude/settings.local.json` (gitignored) for initial testing. Promote to
`.claude/settings.json` (commit-tracked) only after validating quality on your project.

## Step 4 — Smoke Test

```bash
curl -s http://127.0.0.1:9080/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: <gateway-token>" \
  -d '{
    "model": "claude-haiku-4-5-20251001",
    "max_tokens": 16,
    "messages": [{"role": "user", "content": "say ok"}]
  }' | jq '{tier: .x_foundry_tier_used, text: .content[0].text}'
```

Expected output:
```json
{"tier": "local", "text": "ok"}
```

For a sonnet-tier test (routes to Tier B if Yo-Yo is running; falls back to Tier A if not):

```bash
curl -s http://127.0.0.1:9080/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: <gateway-token>" \
  -d '{
    "model": "claude-sonnet-4-6",
    "max_tokens": 16,
    "messages": [{"role": "user", "content": "say ok"}]
  }' | jq '{tier: .x_foundry_tier_used, text: .content[0].text}'
```

## Step 5 — Test with Claude Code (Chat Mode Only)

```bash
ANTHROPIC_BASE_URL=http://127.0.0.1:9080 \
ANTHROPIC_AUTH_TOKEN=<gateway-token> \
claude --print "say ok"
```

Check Doorman logs to confirm the request was received and routed:

```bash
journalctl -u local-doorman.service -f | grep -E "tier_used|route|messages"
```

## CRITICAL CONSTRAINTS — Read Before Proceeding

**Do not route Claude Code agentic sessions through Sprint 0a.**

Sprint 0a flattens `tool_use` and `tool_result` content blocks to plain strings. Claude
Code's agentic coding loop (where it invokes Bash, Read, Write, Edit, Search tools
iteratively) requires these blocks to be preserved structurally. Until Sprint 1 (Canonical
IR) is merged, the shim is safe only for:

- Chat and simple Q&A
- `claude --print` single-turn invocations
- Sessions where Claude Code does not invoke tools in a loop

Setting `ANTHROPIC_BASE_URL` globally (in shell profile or `~/.claude/settings.json`)
during the Sprint 0a period will break your normal Claude Code coding sessions.

**Max subscription does not apply to gateway sessions.**

Tier C calls bill to the Commercial API key configured in the Doorman. The Pro/Max
OAuth credential is not forwarded through the gateway and does not cover Tier C here.

## Model Routing Reference

| Model name prefix | Tier | VM state required |
|---|---|---|
| `claude-haiku-*` | Tier A (OLMo 2 1B, local) | None — always-on |
| `claude-sonnet-*` | Tier B (OLMo 3 32B Think, Yo-Yo) | Yo-Yo must be running; falls back to Tier A if not |
| `claude-opus-*` | Tier C (Anthropic API) | Commercial API key configured |

## Rollback

Remove or rename `.claude/settings.local.json` in the project root. Claude Code reverts
to direct Anthropic connections using the normal OAuth/API key path.

## Next Steps

After Sprint 0a is validated on a test project:

- Sprint 0b: real SSE streaming + on-demand Yo-Yo lazy-start (eliminates Tier A fallback
  on first request of the day)
- Sprint 1: Canonical IR — enables full Claude Code agentic loop through the gateway
- See `.agent/plans/universal-ai-gateway.md` for the full sprint breakdown
