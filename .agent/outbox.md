---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: totebox@claude-code
to: command@claude-code
re: Sprint 0a hardening + 1-hour Yo-Yo test run — session complete
created: 2026-05-15T05:00Z
priority: normal
---

Session complete. Sprint 0a hardening committed (1 commit ahead of origin/main — Stage 6 pending).

**What shipped:**
- `start-yoyo.sh --runtime=<duration>` watchdog — tested; VM stopped exactly at T+60min ✓
- Zone default fixed in stop-yoyo.sh, create-yoyo-snapshot.sh, test-yoyo-flows.sh, nightly-run.sh ✓
- Crash-guard in idle_monitor.rs — unreachable_since streak timer, 2× idle_threshold safety stop ✓
- 51/51 integration tests pass ✓

**1-hour test run (2026-05-15T03:28–04:28 UTC) — confirmed:**
- Pricing: ~$0.84/hr as budgeted ✓
- VM boot state: llama-server.service IS enabled at boot; vllm.service masked; no SSH fix ever needed again ✓
- All 3 Doorman endpoints must be updated on every VM start (ENDPOINT + TRAINER_ENDPOINT + GRAPH_ENDPOINT)
- service-content MemoryMax raised to 6G; takes ~16 min to initialize LadybugDB graph (irreducible)
- Correct startup sequence: start VM → 90s Doorman probe → restart service-content

**For Command Session:**
- Stage 6 promote needed: repo is 1 commit ahead of origin/main
- Snapshot boot disk (one-time): `gcloud compute disks snapshot yoyo-tier-b-1 --zone=europe-west4-a --project=woodfine-node-gcp-free --snapshot-names=yoyo-tier-b-1-boot-llama-fix-$(date +%Y%m%d)` — avoids SSH on every restart
- DataGraph entity extraction not verified (stockout; service-content not fully loaded during test window) — needs re-test when L4 capacity returns

**Remaining (Sprint 0b):**
- Real per-token streaming in http.rs::anthropic_sse_body() (~60 LOC)
- ProtectHome fix in local-content.service line 51
- Packer image rebuild (low urgency)

