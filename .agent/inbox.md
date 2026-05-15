---
mailbox: inbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — task-project-intelligence

---
from: command@claude-code
to: totebox@project-intelligence
re: AGENTS.md retro-add — 10 monorepo crates (batch)
created: 2026-05-14T22:34:22Z
priority: low
---

Add `AGENTS.md` (vendor-neutral pointer file, `root-files-discipline.md` Tier 2) to the
following 10 crates in `vendor/pointsav-monorepo/`. Follow the pattern at
`system-ledger/AGENTS.md` or `moonshot-toolkit/AGENTS.md` — brief header, quick-reference
block pointing to `CLAUDE.md` at that directory, and workspace navigation links.

Crates missing AGENTS.md (confirmed 2026-05-14):
- `app-console-bookkeeper/`
- `app-console-bim/`
- `app-mediakit-knowledge/`
- `app-orchestration-bim/`
- `app-workplace-bim/`
- `app-workplace-memo/`
- `app-workplace-proforma/`
- `service-bim/`
- `service-extraction/`
- `service-slm/`

Commit staging-tier; push to staging mirrors. Stage 6 can batch with other commits.

---
from: command@claude-code
to: task@project-intelligence
re: OPERATOR ACTION — mask vllm.service on yoyo-tier-b-1 (europe-west4-a); gates nightly-run
created: 2026-05-14T16:15:00Z
priority: high
---

This is an operator-required action that blocks the nightly-run pipeline. Track it in your NEXT.md and surface it at every session start until confirmed complete.

**Operator must SSH to yoyo-tier-b-1 and run:**

```bash
sudo systemctl mask vllm.service
sudo systemctl enable llama-server.service
sudo systemctl start llama-server.service
```

Then snapshot the boot disk to lock in the fix.

**Zone:** `europe-west4-a` (not us-west1-b — the VM was reprovisioned there 2026-05-13).

**Why this is blocking:** `vllm.service` is still enabled on the boot image and crash-loops with CUDA OOM on every VM restart (BF16 32B needs ~21.9 GiB + KV cache — just over the 22 GiB L4 limit). `llama-server.service` was started ad-hoc and does not survive a reboot. Until this is fixed, any VM restart will leave Tier B down.

**Note:** item 7 in the prior inbox message (`re: comprehensive handoff`) references `us-west1-b` — that is stale. The correct zone is `europe-west4-a`. Archive item 7 and use this message as the authoritative record.

— command@claude-code

---
from: command@claude-code
to: task@project-intelligence
re: comprehensive handoff — all outstanding project-intelligence work (2026-05-14)
created: 2026-05-14T00:00:00Z
priority: high
---

This message consolidates all outstanding Totebox-scope work for project-intelligence.
Command Session is handing this off cleanly — nothing here requires Command action.

**Prior inbox messages — status:**
- `re: URGENT — rebuild + deploy service-content` (2026-05-13T17:58Z) — **COMPLETED.**
  Watcher fix (b8a70ee / 3e8c8a4) is deployed and confirmed working. Service has been
  stable since 2026-05-13T20:05Z. Archive this message.
- `re: investigate Doorman routing returning invalid JSON` (2026-05-13T23:30Z) — **OPEN.**
  Still needs investigation. See item 1 below.

---

## 1. Doorman extraction interface — investigation + fix (carry-forward from open inbox)

During the 2026-05-13 startup scan, all 114 CORPUS_ files returned
`[SYS_HALT] Doorman response was not a valid entity JSON array`. Watcher fix is working
(each file attempted exactly once, no hang). But every extraction failed because Tier A
(local OLMo 7B) was the only backend and cannot produce a structured JSON array via
`/v1/chat/completions`. DataGraph has zero extractions from corpus since redeployment.

**Investigate:**
1. What does `slm-doorman` actually return from Tier A for an extraction prompt — error
   object `{"error":"..."}`, chat-style text, or something else? Trace through
   `slm-doorman/src/tier/local.rs`.
2. Should service-content distinguish `"Doorman returned an error"` (retry at next boot)
   vs `"Doorman returned malformed data"` (permanent skip)?
3. Should Doorman have a dedicated `POST /v1/extract` endpoint that always returns a
   valid array (possibly `[]` on Tier A failure) rather than routing through
   `/v1/chat/completions`?

Pick the approach that fits cleanest with the existing tier routing architecture and
implement. This is the highest-priority code item — DataGraph is stale until it's fixed.

---

## 2. start-yoyo.sh line 340 — update_doorman_env on every Mode 1 success

`update_doorman_env` is currently only called when `known_zone != PRIMARY_ZONE`.
Spot VMs get a new IP on every restart regardless of zone, so the Doorman env goes
stale after every restart even within the same zone.

**Fix:** call `update_doorman_env` unconditionally on every Mode 1 success, not only
on zone change. File: `service-slm/scripts/start-yoyo.sh` line ~340.
Commit in cluster branch via `commit-as-next.sh`.

---

## 3. Universal AI Gateway — Sprint 0a (Anthropic Messages shim)

Full plan at `.agent/plans/universal-ai-gateway.md`. Sprint 0a is the immediate next
feature for the cluster. Implement `POST /v1/messages` Anthropic shim in
`crates/slm-doorman-server/src/http.rs` (~305 LOC):

- New structs: `AnthropicMessagesBody`, `AnthropicMessage`, `AnthropicContent`,
  `AnthropicContentBlock`
- Adapter: `anthropic_to_compute_request()` — flatten content blocks, map model → Complexity
- Response: `compute_to_anthropic_response()` — emit Anthropic Messages API shape
- Fake SSE streaming: buffer full response, emit SSE events in one burst
- Model routing: haiku → Tier A, sonnet → Tier B ("trainer"), opus → Tier C
- 3 unit tests (simple message, system prompt, SSE format)

This enables Claude Code itself to route through Doorman via `ANTHROPIC_BASE_URL`.

---

## 4. Drafts outbound — notify project-editorial

11 drafts are staged at `.agent/drafts-outbound/` with status `draft-pending-language-pass`.
Send an outbox message to `project-editorial` flagging them for pickup.

| Draft | Type | Language |
|---|---|---|
| `guide-yo-yo-nightly-pipeline.md` | GUIDE | EN |
| `topic-apprenticeship-substrate.md` | TOPIC | EN |
| `topic-apprenticeship-substrate.es.md` | TOPIC | ES |
| `topic-doorman-protocol.md` | TOPIC | EN |
| `topic-doorman-protocol.es.md` | TOPIC | ES |
| `topic-jennifer-datagraph-rebuild.md` | TOPIC | EN |
| `topic-jennifer-datagraph-rebuild.es.md` | TOPIC | ES |
| `topic-yo-yo-lora-training-pipeline.md` | TOPIC | EN |
| `topic-yo-yo-lora-training-pipeline.es.md` | TOPIC | ES |
| `topic-zero-container-inference.md` | TOPIC | EN |
| `topic-zero-container-inference.es.md` | TOPIC | ES |

---

## 5. Outbox — archive stale messages

The project-intelligence outbox has 5 messages accumulated from 2026-05-12 and 2026-05-13.
All have been read and actioned by Command Session. Archive them to `outbox-archive.md`.

---

## 6. Stage 6 — promote cluster branch to canonical main

5 commits on the cluster branch are ahead of `origin/main` and need promotion:

```bash
cd /srv/foundry/clones/project-intelligence
~/Foundry/bin/promote.sh
```

Verify the push completes cleanly and confirm `origin/main` is up to date.

---

---

## 7. Yo-Yo — mask vllm.service before next boot (BLOCKING)

`vllm.service` is still enabled and crash-loops with CUDA OOM on restart (BF16 32B needs
~21.9 GiB + KV cache headroom — just over the 22 GiB L4 limit). When us-west1-b L4
capacity returns and the VM can start:

```bash
gcloud compute ssh yoyo-tier-b-1 --zone=us-west1-b --project=woodfine-node-gcp-free
sudo systemctl mask vllm.service
sudo systemctl enable llama-server.service
sudo systemctl start llama-server.service
```

Then snapshot the boot disk to lock in the fix.

---

## 8. Set SLM_YOYO_WEIGHTS_GCS_BUCKET in local-doorman.env

Training markers are currently local-only. Set the GCS bucket so they dispatch correctly:

```bash
sudo tee -a /etc/local-doorman/local-doorman.env <<'EOF'
SLM_YOYO_WEIGHTS_GCS_BUCKET=woodfine-node-gcp-free-foundry-substrate
EOF
sudo systemctl restart local-doorman.service
```

Also add to `docs/deploy/local-doorman.env.example` in the cluster for documentation.

---

## 9. Packer image rebuild + OLMo 3 32B weights upload (after item 7 complete)

Once vllm.service is masked and llama-server is confirmed working:

```bash
cd /srv/foundry/clones/project-intelligence/service-slm/compute/packer
packer build yoyo-image.pkr.hcl
```

Then upload OLMo 3 32B-Think Q4 weights to the Yo-Yo VM (~20 GB). Reference the
weights location in `SLM_YOYO_WEIGHTS_GCS_BUCKET` for the pull.

— command@claude-code

---
from: command@claude-code
to: task@project-intelligence
re: investigate Doorman routing returning invalid JSON during service-content startup scan
created: 2026-05-13T23:30:00Z
priority: normal
---

During the service-content startup scan on 2026-05-13 (~18:49–20:05 UTC), all 114 CORPUS_
files received `[SYS_HALT] Doorman response was not a valid entity JSON array` from
Doorman at `http://127.0.0.1:9080/v1/chat/completions`. The watcher retry storm fix
(b8a70ee / 3e8c8a4) is confirmed working — each file was attempted exactly once. However,
Doorman should either return a valid entity JSON array or return a structured error that
service-content can handle gracefully.

**Observed:** Doorman returned a non-JSON-array response for every extraction request
during a ~76-minute window. Yo-Yo (Tier B) was stopped. Tier A (local OLMo 7B) was the
only available backend.

**Questions to investigate:**
1. What does Doorman actually return when Tier A cannot process a DataGraph extraction
   request? Is it returning an error object `{"error": "..."}` or an empty response or
   something else — all of which would fail the `is_array()` check in service-content?
2. Should service-content distinguish between "Doorman returned an error" (retry later)
   vs "Doorman returned malformed data" (skip permanently)?
3. Should Doorman have a dedicated extraction endpoint that always returns a valid array
   (possibly empty) rather than routing through `/v1/chat/completions` which is designed
   for chat completions, not structured entity extraction?

**Context:** service-content calls Doorman at `/v1/chat/completions` and expects the
response to be a JSON array of entity objects. This is mixing a chat endpoint with a
structured-data contract — the mismatch may be the root cause.

**Not blocking** — service-content is stable and the watcher fix prevents VM hangs.
But the 114-file scan produced zero successful extractions, which means the DataGraph
was not updated from any of the corpus files during this boot.

— command@claude-code

---
from: command@claude-code
to: task@project-intelligence
re: URGENT — rebuild + deploy service-content with watcher fix; VM hang root cause
created: 2026-05-13T17:58:00Z
priority: high
---

**The VM hung and required a manual operator reset. Root cause: watcher retry storm.**

The deployed binary (LBUG_SHARED=1 variant) does NOT include the b8a70ee watcher fix.
The journal shows it hammering Doorman every ~37 seconds right now:

```
[WATCHER] Routing payload to Doorman...
[SYS_HALT] Doorman response was not a valid entity JSON array.
[WATCHER] Routing payload to Doorman...   ← repeating indefinitely
```

This will hang the VM again. **Please rebuild and redeploy immediately.**

**Build approach (LBUG_SHARED=1 is the only working path right now):**

```bash
cd /srv/foundry/clones/project-intelligence
export LBUG_SHARED=1
cargo build --release -p service-content
sudo cp target/release/service-content /usr/local/bin/service-content
sudo systemctl restart local-content.service
journalctl -u local-content.service -f
```

**Verify success:** After restart, confirm:
1. Service reaches `[HTTP] Graph API listening on 127.0.0.1:9081`
2. Memory stabilises below 3G (RSS ~2.8G at start, should hold)
3. No `[SYS_HALT] Doorman response...` loop in journal after first attempt

The watcher fix (b8a70ee Fix 2: filename pushed to `processed_ledgers` unconditionally)
is now on main (cherry-picked by Command Session). Rebuild will include it.

— command@claude-code

