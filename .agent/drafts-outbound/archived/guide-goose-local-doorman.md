# GUIDE: Running Goose Against the Local AI Gateway

Goose (block/goose, v1.36.0+) is an open-source AI agent that uses the Anthropic
Messages API wire format. By setting two environment variables, Goose routes through
the local gateway rather than Anthropic's servers — allowing local model inference
and keeping session data inside the cluster.

## Prerequisites

- Local gateway running and responding to `GET /readyz` with `has_local: true`
- Goose binary installed at `/usr/local/bin/goose` (or on PATH)
- Local model loaded in llama-server with at least one free slot

Check:

```bash
curl -s http://127.0.0.1:9080/readyz
# Expected: {"ready":true,"has_local":true,...}

goose --version
# Expected: 1.36.0 or later

curl -s http://127.0.0.1:8080/health
# Expected: {"status":"ok",...}
```

## Installing Goose

Download the pre-built binary from the project's GitHub releases:

```bash
# Find the latest release at https://github.com/block/goose/releases
# Download the linux-x86_64 binary and install

curl -L -o /tmp/goose \
    https://github.com/block/goose/releases/download/v1.36.0/goose-x86_64-unknown-linux-musl
chmod +x /tmp/goose
sudo mv /tmp/goose /usr/local/bin/goose
goose --version
```

## Running a Session

Set the three environment variables and run:

```bash
export ANTHROPIC_HOST=http://127.0.0.1:9080
export ANTHROPIC_API_KEY=foundry-local
export GOOSE_MODEL=claude-haiku-4-5-20251001

goose session
```

The gateway maps any model name in the Haiku family (`claude-haiku-*`) to the
local Tier A model (OLMo-2-7B Q4_K_M). The session behaves exactly as a standard
Goose session; responses come from the local model rather than a remote API.

For a non-interactive one-shot invocation:

```bash
ANTHROPIC_HOST=http://127.0.0.1:9080 \
ANTHROPIC_API_KEY=foundry-local \
GOOSE_MODEL=claude-haiku-4-5-20251001 \
goose run --text "Say hello and tell me the date"
```

## Verifying Gateway Routing

In a second terminal, watch gateway logs while Goose runs:

```bash
journalctl -u local-doorman.service -f | grep -iE "POST|tier|model"
```

Successful routing shows a `POST /v1/messages` log line with `tier: local`.

## Known Limitation: Tool Invocation

The Tier A local model (OLMo-2-7B Q4_K_M) **does not reliably invoke tools**. When
Goose asks it to read or write a file, the model responds with explanatory text
(describing how to use `cat`, for example) rather than emitting a `tool_use` content
block. This is a model capability limit — the gateway correctly reformats tool
definitions into the format the inference server expects, but the model itself does
not use them.

**Effect:** Goose chat and reasoning tasks work. Goose file-tool tasks (read, write,
edit) require a tool-use-fine-tuned model on Tier B.

**To enable reliable tool invocation:** provision the Tier B Yo-Yo VM running a
tool-capable model (OLMo-3-32B-Think or equivalent). Once Tier B is live and the
gateway circuit breaker closes, Goose will route through it and tool invocations
will work.

## Checking Session Capture

Goose sessions are written to `~/.config/goose/sessions/` as JSONL files. The
local CORPUS bridge (`local-claude-bridge.service`) watches `~/.claude/projects/`
for Claude Code session ledgers — not Goose sessions. Goose sessions are therefore
not currently captured for entity extraction. This is by design: the pipeline targets
Claude Code's structured ledger format, which includes richer metadata about tool
use and file edits.

## References

- `guide-activate-anthropic-shim.md` — activating the local gateway shim
- `topic-tos-training-constraints.md` — constraints on what model outputs may be used for training
