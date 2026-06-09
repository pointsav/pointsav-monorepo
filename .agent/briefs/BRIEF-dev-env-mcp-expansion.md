---
artifact: brief
status: active
created: 2026-06-09
session: command@claude-code
related_briefs:
  - BRIEF-slm-substrate-master.md
  - BRIEF-slm-learning-loop.md
doctrine_claims: [22, 32, 40, 43, 44, 46, 48]
---

# BRIEF: Foundry Development Environment as First Totebox Deployment
## MCP Expansion — Session Efficiency + LoRA Training + DataGraph Enrichment

---

## Context — Why This Is the Right Frame

The Foundry workspace IS `vault-privategit-source-1` — the first customer deployment of the
Totebox Orchestration substrate (MANIFEST.md, Doctrine §IV.a). Every tool, protocol, and
discipline the workspace uses IS the product delivered to customers and community members.

Using `slm-mcp-server` for this work is constitutionally correct. Doctrine claim #46:
"Every Ring 1 and Ring 2 service exposes an MCP server interface as its primary external
contract." The workspace running the same MCP server customers receive is the whole point —
dogfooding is doctrine.

**Three goals, one sprint plan:**
1. **Session efficiency** — eliminate 3,000–8,000 tokens of per-session boilerplate
2. **LoRA training connection** — make Claude Code commits feed the apprenticeship pipeline
3. **DataGraph enrichment** — ground inference in live entity context

---

## Architectural Decision: Option C (extend slm-mcp-server)

Option C confirmed. Option B (separate binary) documented as future sprint at Phase 4 Doorman
MCP migration. `slm-mcp-server` already IS the product; adding mailbox tools makes the workspace
a more complete reference deployment.

---

## Session 1 — Status (2026-06-09) COMPLETE

### Track 1 P0 tools — SHIPPED

Binary: `slm-mcp-server v0.2.0` at `/usr/local/bin/slm-mcp-server`
SHA256: `eb24a38932c73b92d024ab5fab5fb0e412382765793a2f8960d08dee2f003138`
Source: `service-slm/crates/slm-mcp-server/src/main.rs`

**New tools added:**

| Tool | Purpose | Token saving |
|---|---|---|
| `get_session_brief` | Reads inbox, outbox, NOTAM, session-context, workspace-state in one call | 3,000–8,000/session |
| `send_mailbox_message` | Routes through bin/mailbox-send.sh — restores M-2/M-10 audit | Eliminates YAML hand-edit |
| `get_doorman_status` | Concurrent /healthz + /readyz + /v1/status/flow + /v1/status/cost | Replaces 2 deprecated calls |
| `query_mailbox` | Multi-archive sweep; scope="all" scans all 23+ archives | Replaces 23+ Read calls |

**Structural changes:**
- `FoundryServer` struct now has `foundry_root: PathBuf`
- `FOUNDRY_ROOT=/srv/foundry` added to `~/.claude.json` env block
- `doorman_health()` and `get_corpus_stats()` marked `[deprecated: use get_doorman_status]`
- Version bumped to 0.2.0

### Track 2 Gap A — COMPLETE

Git post-commit hook installed on 4 priority archives:
- project-intelligence: ✓
- project-editorial: ✓
- project-knowledge: ✓
- project-design: ✓

Source: `service-slm/scripts/git-post-commit-hook.sh`
Effect: every `commit-as-next.sh` commit now fires `POST /v1/shadow` with diff → apprenticeship queue

---

## Session 2 — Status (2026-06-09) COMPLETE

### Track 1 P1 tools — SHIPPED

Binary: `slm-mcp-server v0.3.0` at `/usr/local/bin/slm-mcp-server`
SHA256: `1eab9b954c866db917340b302d35fcaf582571cbb06307cb4780bf2e4619d6a1`

**New tools added:**

| Tool | Purpose |
|---|---|
| `cast_apprenticeship_verdict` | Builds + SSH-signs verdict body; POSTs `VerdictWireBody` to `POST /v1/verdict` |
| `get_service_status` | `GET /v1/status/queue` + optional fs counts + audit-ledger line count |

### Track 2 Gap B — CONFIRMED WORKING (not broken)

Re-investigated `router.rs` code + live Doorman logs. The graph context injection IS
correct and operational. Log confirmed: "graph context injected module_id=woodfine
entity_count=5". The BRIEF fault note from 2026-06-05 was stale — `service-content`
at port 9081 with 9,688 entities was already wired and `SERVICE_CONTENT_ENDPOINT`
IS set in local-doorman.env. No code change needed. `ask_local` description updated
to reflect the confirmed-working state.

### Track 2 Gap C — SHIPPED

`cast_apprenticeship_verdict` implemented in Sprint 5:
- Reads `identity/.toggle` → determines signing identity (jwoodfine/pwoodfine)
- Builds YAML body matching `parse_verdict_body` in `slm-doorman/src/verdict.rs`
- Signs via `ssh-keygen -Y sign -f {key_path} -n apprenticeship-verdict-v1 {body_file}`
- Base64-encodes PEM `.sig` output
- POSTs `VerdictWireBody { body, signature, senior_identity }` to `POST /v1/verdict`
- Returns dispatch outcome (`VerdictDispatchOutcome` JSON)

---

## Session 3 — Pending

### Track 3 — DataGraph enrichment

1. **Graph mutation audit** (~10 LOC) — `http.rs` POST /v1/graph/mutate handler; add
   `event_type: "graph-mutation"` audit write after successful proxy
2. **NEXT.md** — add Option B migration sprint item

---

## Pre-operator action needed

Install ML libraries on yoyo-batch VM for LoRA training:
```bash
pip install trl>=0.8 peft>=0.10 transformers>=4.40 datasets bitsandbytes
```
Blocks all LoRA training runs regardless of pipeline completeness.

---

## Verified facts

| Fact | Verified |
|---|---|
| Binary v0.3.0 at `/usr/local/bin/slm-mcp-server` | ✓ 2026-06-09 |
| sha256: `1eab9b95...` in binary-ledger | ✓ 2026-06-09 |
| Binary v0.2.0 sha256: `eb24a38...` (Session 1 record) | ✓ 2026-06-09 |
| FOUNDRY_ROOT in ~/.claude.json | ✓ 2026-06-09 |
| Post-commit hook: 4 archives | ✓ 2026-06-09 |
| Apprenticeship enabled (SLM_APPRENTICESHIP_ENABLED=true) | ✓ verified |
| Graph injection IS WORKING (log: "entity_count=5") | ✓ 2026-06-09 |
| identity/.toggle = 0 → jwoodfine at session start | ✓ 2026-06-09 |
| identity key at identity/jwoodfine/id_jwoodfine readable by mathew | ✓ 2026-06-09 |
| apprenticeship queue dirs: queue, queue-done, queue-poison, queue-in-flight, queue-paused | ✓ 2026-06-09 |

---

## Connection to app-orchestration-command

Once `app-orchestration-command` Phase 3 ships:
- `query_mailbox(scope="all")` → call `http://127.0.0.1:8020/archives` instead of reading 23 files
- `send_mailbox_message` → route through `POST /v1/message` instead of shelling to script

Same MCP tool names, same parameters, new backend. Add to NEXT.md as migration item.
